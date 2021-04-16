use tokio::sync::oneshot::{
    self,
    Sender,
    Receiver,
    error::TryRecvError
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
        let (tx, rx) = oneshot::channel();
        self.damaged_shot = Some(tx);

        // Start async loop
        tokio::spawn(async move {
            Self::gpio_loop(rx).await;
        });
        // Send first instruction
        self.update_instruction(init_instruction);
    }

    pub fn update_instruction(&mut self, instruction: GpioInstruction) {
        // instruction set damaged ==> notify evt loop
        if let Some(sender) = self.damaged_shot.take() {
            sender.send(instruction)
                .map_err(|_| "Could not write to async loop")
                .unwrap();
        }
    }

    /// Main loop, not for external use
    async fn gpio_loop(mut recv: Receiver<GpioInstruction>) {
        let mut instruction = None;
        loop {
            // check for damage
            instruction = match recv.try_recv() {
                Ok(instr) => {
                    info!("Instructions damaged, stop loop");
                    Some(instr)
                }, // damaged, stop loop
                Err(TryRecvError::Closed) => {
                    error!("Loop channel unexpectedly closed");
                    return;
                }, // something went wrong, exit loop
                Err(TryRecvError::Empty) => instruction,
            };

            // If there is a correct instruction, execute it
            if let Some(instr) = &instruction {
                tokio::join!(
                    Self::exec_evt(&instr.feed),
                    Self::exec_evt(&instr.delay),
                    Self::exec_evt(&instr.exhaust),
                    Self::exec_evt(&instr.end),
                );
            }
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

