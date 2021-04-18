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
        time: 3,
    },
    exhaust: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: true },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 5,
    },
    end: GpioEvent {
        state: [
            GpioState {valve_id: Valve::FEED, status: false },
            GpioState {valve_id: Valve::OO, status: false },
            GpioState {valve_id: Valve::NN, status: false },
        ],
        time: 10,
    }
};

pub fn get_default_state() -> ValveState {SystemState::get_default_state()}

/// Bootstrap gpio module
pub async fn bootstap(mut rx: mpsc::Receiver<GpioInstruction>, 
                      tx: Option<watch::Sender<ValveState>>) {
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
