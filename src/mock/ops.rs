use serde::{Serialize, Deserialize};

/// Mock operation
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Op {
    #[serde(flatten)]
    pub kind: Kind,
    pub res: i32,
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PinState {
    High,
    Low,
}

impl From<PinState> for embedded_hal::digital::PinState {
    fn from(s: PinState) -> Self {
        use embedded_hal::digital::PinState::*;

        match s {
            PinState::High => High,
            PinState::Low => Low,
        }
    }
}


impl From<embedded_hal::digital::PinState> for PinState {
    fn from(s: embedded_hal::digital::PinState) -> Self {
        use embedded_hal::digital::PinState::*;

        match s {
            High => PinState::High,
            Low => PinState::Low,
        }
    }
}

/// Mock operation kind enumeration
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag="kind", rename_all="snake_case")]
pub enum Kind {
    I2cInit{
        port: u32,
        baud: u32,
        sda: i32,
        scl: i32,
    },
    I2cDeinit{
        handle: i32,
    },
    I2cWrite{
        handle: i32,
        addr: u16,
        data_out: Vec<u8>,
    },
    I2cRead{
        handle: i32,
        addr: u16,
        data_in: Vec<u8>,
    },
    I2cWriteRead{
        handle: i32,
        addr: u16,
        data_out: Vec<u8>,
        data_in: Vec<u8>,
    },
    SpiInit{
        port: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    },
    SpiDeinit{
        handle: i32,
    },
    SpiRead{
        handle: i32,
        data_in: Vec<u8>,
    },
    SpiWrite{
        handle: i32,
        data_out: Vec<u8>,
    },
    SpiTransfer{
        handle: i32,
        data_out: Vec<u8>,
        data_in: Vec<u8>,
    },
    UartInit{
        port: u32,
        baud: u32,
        tx: i32,
        rx: i32,
    },
    UartDeinit{
        handle: i32,
    },
    UartWrite{
        handle: i32,
        flags: u32,
        data_out: Vec<u8>,
    },
    UartRead{
        handle: i32,
        flags: u32,
        data_in: Vec<u8>,
    },
    GpioInit{
        port: i32,
        pin: i32,
        output: bool,
    },
    GpioDeinit{
        handle: i32,
    },
    GpioSet{
        handle: i32,
        state: PinState,
    },
    GpioGet{
        handle: i32,
        state: PinState,
    },
}
