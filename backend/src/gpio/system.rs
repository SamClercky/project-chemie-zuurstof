use mpsc::error::TrySendError;
use tokio::sync::{
    watch, mpsc,
};
use std::time::Duration;
use log::*;

use super::*;

/// Current state of entire system
pub struct SystemState {
    instructions: Option<watch::Sender<GpioInstruction>>,
}

impl SystemState {
    pub fn new() -> Self { Self {
            instructions: None,
        } }

    pub fn get_default_state() -> ValveState { [
                GpioState { valve_id: Valve::FEED, status: true},
                GpioState { valve_id: Valve::OO, status: true},
                GpioState { valve_id: Valve::NN, status: true},
    ]}

    pub fn start(&mut self, init_instruction: GpioInstruction,
                 update_tx: Option<watch::Sender<[GpioState; 3]>>) {
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
                       update_tx: Option<watch::Sender<ValveState>>) {
        let (tx, mut rx) = mpsc::channel::<ValveState>(4);

        loop {
            // Fetch current instrucions
            let instruction = recv.borrow().to_owned();

            // Execute it
            tokio::join!(
                Self::exec_evt(&instruction.feed, tx.clone()),
                Self::exec_evt(&instruction.delay, tx.clone()),
                Self::exec_evt(&instruction.exhaust, tx.clone()),
                Self::exec_evt(&instruction.end, tx.clone()),

                // Poll state and send it to other places
                Self::broadcast_state(&update_tx, &mut rx),
            );
        }
    }

    /// Execute one Gpio event {feed|delay|exhaust}
    async fn exec_evt(evt: &GpioEvent, update_tx: mpsc::Sender<ValveState>) {
        tokio::time::sleep(Duration::from_secs(evt.time)).await;

        // TODO: Execute feed
        debug!("Evt executed: Time {}", evt.time);
        for valve in evt.state.iter() {
            debug!("Pin: {}, status: {}", valve.valve_id.get_pin_nr(), valve.status);
        }

        // update state
        match update_tx.try_send(evt.state.to_owned()) {
            Ok(_) => (),
            Err(TrySendError::Closed(_)) => {error!("update status queue is closed");},
            Err(TrySendError::Full(_)) => {warn!("update status queue is full");}
        }
    }

    async fn broadcast_state(update_tx: &Option<watch::Sender<ValveState>>,
                            new_data_rx: &mut mpsc::Receiver<ValveState>) {
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

