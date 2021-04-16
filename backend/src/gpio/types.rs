use serde_derive::{Deserialize, Serialize};

/// Instructionset of operation, descriptive
#[derive(Deserialize, Serialize, Clone)]
pub struct GpioInstruction {
    pub feed: GpioEvent,
    pub delay: GpioEvent,
    pub exhaust: GpioEvent,
    pub end: GpioEvent,
}

/// Event with all valve states, descriptive
#[derive(Deserialize, Serialize, Clone)]
pub struct GpioEvent {
    pub state: [GpioState; 3],
    pub time: u64,
}

/// State of Gpio pin, descriptive 
#[derive(Deserialize, Serialize, Clone)]
pub struct GpioState {
    pub valve_id: Valve,
    pub status: bool,
}

/// Representation of a real live valve
#[derive(Deserialize, Serialize, Clone)]
pub enum Valve {
    FEED, OO, NN
}

impl Valve {
    pub fn get_pin_nr(&self) -> usize {
        match self {
            Self::FEED => 1,
            Self::OO => 2,
            Self::NN => 3,
        }
    }
}
