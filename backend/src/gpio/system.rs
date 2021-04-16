use tokio::sync::watch::{
    self,
    Sender,
    Receiver,
};
use std::time::Duration;
use log::*;

use super::{GpioInstruction, GpioEvent};

/// Current state of entire system
pub struct SystemState {
    damaged_shot: Option<Sender<GpioInstruction>>,
}

impl SystemState {
    pub fn new() -> Self { Self {
        damaged_shot: None,
    } }

    pub fn start(&mut self, init_instruction: GpioInstruction) {
        let (tx, rx) = watch::channel(init_instruction);
        self.damaged_shot = Some(tx);

        // Start async loop
        tokio::spawn(async move {
            Self::gpio_loop(rx).await;
        });
    }

    pub fn update_instruction(&mut self, instruction: GpioInstruction) {
        // instruction set damaged ==> notify evt loop
        if let Some(sender) = &self.damaged_shot {
            if sender.is_closed() {error!("No receiver found in async loop");}

            sender.send(instruction)
                .map_err(|_| "Could not write to async loop")
                .unwrap();
        }
    }

    /// Main loop, not for external use
    async fn gpio_loop(recv: Receiver<GpioInstruction>) {
        loop {
            // Fetch current instrucions
            let instruction = recv.borrow().to_owned();

            // Execute it
            tokio::join!(
                Self::exec_evt(&instruction.feed),
                Self::exec_evt(&instruction.delay),
                Self::exec_evt(&instruction.exhaust),
                Self::exec_evt(&instruction.end),
            );
        }
    }

    /// Execute one Gpio event {feed|delay|exhaust}
    async fn exec_evt(evt: &GpioEvent) {
        tokio::time::sleep(Duration::from_secs(evt.time)).await;

        // TODO: Execute feed
        debug!("Evt executed: Time {}", evt.time);
        for valve in evt.state.iter() {
            debug!("Pin: {}, status: {}", valve.valve_id.get_pin_nr(), valve.status);
        }
    }
}

