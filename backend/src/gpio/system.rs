use mpsc::error::TrySendError;
use tokio::sync::{
    watch, mpsc, Mutex,
};
use std::sync::Arc;
use std::time::Duration;
use log::*;
use std::io::Write;

use super::*;

/// Current state of entire system
pub struct SystemState {
    instructions: Option<watch::Sender<GpioInstruction>>,
}

impl SystemState {
    pub fn new() -> Self { 
        Self {
            instructions: None,
        }
    }

    pub fn get_default_state() -> ValveState { [
                GpioState { valve_id: Valve::FEED, status: false},
                GpioState { valve_id: Valve::OO, status: false},
                GpioState { valve_id: Valve::NN, status: false},
    ]}

    pub fn start(&mut self, init_instruction: GpioInstruction,
                 update_tx: Option<watch::Sender<GpioEvent>>) {
        let (tx, rx) = watch::channel(init_instruction);

        self.instructions = Some(tx);

        // Start async loop
        tokio::spawn(async move {
            Self::gpio_loop(rx, update_tx).await;
        });
    }

    pub fn update_instruction(&mut self, instruction: GpioInstruction) {
        // instruction set damaged ==> notify evt loop
        if let Some(sender) = &self.instructions {
            if sender.is_closed() {error!("No receiver found in async loop");}

            sender.send(instruction)
                .map_err(|_| "Could not write to async loop")
                .unwrap();
        }
    }

    /// Main loop, not for external use
    async fn gpio_loop(recv: watch::Receiver<GpioInstruction>,
                       update_tx: Option<watch::Sender<GpioEvent>>) {
        let (tx, mut rx) = mpsc::channel::<GpioEvent>(4);
        let serial_port = Arc::new(Mutex::new(
                serialport::TTYPort::open(
                    &serialport::new("/dev/ttyACM0", 9600)
                ).ok()
        ));
        // set non exclusive
        {
            let mut serial_port = serial_port.lock().await;
            if let Some(serial_port) = &mut *serial_port {
                serial_port.set_exclusive(false)
                    .expect("Could not set serial port to not exclusive");
            } else {
                error!("Could not open serial port");
            }
        }

        loop {
            // Fetch current instrucions
            let instruction = recv.borrow().to_owned();

            // Execute it
            tokio::join!(
                Self::exec_evt( &instruction.feed,
                                tx.clone(),
                                format!("Feed: {}", 
                                    ( instruction.delay.time-instruction.feed.time )/1000), // print in sec
                                serial_port.clone()),
                Self::exec_evt( &instruction.delay,
                                tx.clone(),
                                format!("Delay: {}", 
                                    (instruction.exhaust.time-instruction.delay.time)/1000), // print in sec
                                serial_port.clone()),
                Self::exec_evt( &instruction.exhaust,
                                tx.clone(),
                                format!("Exhaust: {}", 
                                    (instruction.end.time-instruction.exhaust.time)/1000), // print in sec
                                serial_port.clone()),
                Self::exec_evt( &instruction.end,
                                tx.clone(),
                                "End".to_string(),
                                serial_port.clone()),

                // Poll state and send it to other places
                Self::broadcast_state(&update_tx, &mut rx),
            );
        }
    }

    /// Execute one Gpio event {feed|delay|exhaust}
    async fn exec_evt(evt: &GpioEvent,
                      update_tx: mpsc::Sender<GpioEvent>,
                      status_msg: String,
                      serial_port: Arc<Mutex<Option<serialport::TTYPort>>>) {
        let duration = Duration::from_millis(evt.time);
        let duration = if duration > Duration::from_secs(60) {
            warn!("Duration: {}ms is too long, truncating to 60s", duration.as_millis());
            Duration::from_secs(60)
        } else {
            duration
        };
        tokio::time::sleep(duration).await;

        debug!("Evt executed: Time {}", evt.time);

        let mut payload = String::from("");
        for valve in evt.state.iter() {
            debug!("Pin: {}, status: {}", valve.valve_id.get_pin_nr(), valve.status);
            let status = if valve.status {"H"} else {"L"};
            payload += format!("{}{}", status, valve.valve_id.get_pin_nr()).as_str();
        }
        // send status message
        payload += format!("M{}\n", status_msg).as_str();

        // send to gpio
        async move {
            if let Some(port) = &mut*serial_port.lock().await {
                if let Err(e) = port.write(payload.as_bytes()) {
                    error!("Could not write to serial: {}", e);
                }
                if let Err(e) = port.flush() {
                    error!("Could not flush data serial: {}", e);
                }
            } else {
                warn!("Not sending payload because no port found");
            }
        }.await;

        // update state
        match update_tx.try_send(evt.to_owned()) {
            Ok(_) => (),
            Err(TrySendError::Closed(_)) => {error!("update status queue is closed");},
            Err(TrySendError::Full(_)) => {warn!("update status queue is full");}
        }
    }

    async fn broadcast_state(update_tx: &Option<watch::Sender<GpioEvent>>,
                            new_data_rx: &mut mpsc::Receiver<GpioEvent>) {
        if let Some(update_tx) = update_tx {
            for i in 0..4_u8 {
                debug!("{}th loop", i);
                if let Some(state) = new_data_rx.recv().await {
                    update_tx.send(state)
                        .map_err(|_| "Could not broadcast new state")
                        .unwrap();
                }
            }
        }
    }
}

