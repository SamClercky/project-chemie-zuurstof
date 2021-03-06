use tokio::sync::mpsc;
use tokio::sync::watch;
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
        time: 500,
    },
    exhaust: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: true },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 6_000,
    },
    end: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: false },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 10_000,
    }
};

pub fn get_default_state() -> GpioEvent { GpioEvent {
    state: SystemState::get_default_state(),
    time: 0,
}}

/// Bootstrap gpio module
pub async fn bootstap(mut rx: mpsc::Receiver<GpioInstruction>, 
                      tx: Option<watch::Sender<GpioEvent>>) {
    let mut system = system::SystemState::new();
    system.start(INIT_INSTRUCTION, tx);

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
