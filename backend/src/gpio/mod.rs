use tokio::sync::mpsc::{self, Receiver, Sender};
use log::*;

mod types;
mod system;

pub use types::*;
pub use system::SystemState;

const INIT_INSTRUCTION: GpioInstruction = GpioInstruction {
    feed: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: true },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 0,
    },
    delay: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: false },
            GpioState {valve_id: Valve::OO, status: true },
            GpioState {valve_id: Valve::NN, status: true },
        ],
        time: 30,
    },
    exhaust: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: true },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 300,
    },
    end: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: false },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 580,
    }
};

/// Bootstrap gpio module
pub async fn bootstap(mut rx: Receiver<GpioInstruction>) {
    let mut system = system::SystemState::new();
    system.start(INIT_INSTRUCTION);

    loop {
        tokio::select! {
            instr = rx.recv() => {
                debug!("Instr received");
                if let Some(instr) = instr {
                    system.update_instruction(instr);
                }
            }
        }
    }
}
