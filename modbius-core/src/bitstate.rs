use core::{convert::TryFrom, ops::Not};

use crate::ModbusSerializationError;

/// Bit addressable modbus data item state helper type.
///
/// This enum works similiar to bool but adds some helper functionality
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum BitState {
    Off = 0,
    On = 1,
}

impl BitState {
    pub fn is_on(self) -> bool {
        self.into()
    }
}

impl Default for BitState {
    fn default() -> BitState {
        BitState::Off
    }
}

impl TryFrom<u16> for BitState {
    type Error = ModbusSerializationError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            0xFF00 => Ok(Self::On),
            _ => Err(ModbusSerializationError::InvalidValue)
        }
    }
}

impl Into<u16> for BitState {
    fn into(self) -> u16 {
        match self { 
            Self::Off => 0,
            Self::On => 0xFF00,
        }
    }
}

impl From<bool> for BitState {
    fn from(b: bool) -> Self {
        if b {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl Into<bool> for BitState {
    fn into(self) -> bool {
        match self {
            Self::Off => false,
            Self::On => true,
        }
    }
}

impl Not for BitState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BitState::On => BitState::Off,
            BitState::Off => BitState::On,
        }
    }
}

#[cfg(test)]
mod bitstate_test {
    use core::convert::TryFrom;

    use crate::{BitState, ModbusSerializationError};

    #[test]
    fn from_bool_true() {
        let state_on = BitState::from(true);
        assert_eq!(state_on, BitState::On);
        assert_eq!(!state_on, BitState::Off);
        assert_eq!(true, state_on.into());
        assert_eq!(0xFF00u16, state_on.into());
    }


    #[test]
    fn from_bool_false() {
        let state_on = BitState::from(false);
        assert_eq!(state_on, BitState::Off);
        assert_eq!(!state_on, BitState::On);
        assert_eq!(false, state_on.into());
        assert_eq!(0u16, state_on.into());
    }

    #[test]
    fn from_u16_on() {
        let state_on = BitState::try_from(0xFF00).unwrap();
        assert_eq!(state_on, BitState::On);
        assert_eq!(!state_on, BitState::Off);
        assert_eq!(true, state_on.into());
        assert_eq!(0xFF00u16, state_on.into());
    }

    #[test]
    fn from_u16_off() {
        let state_on = BitState::try_from(0).unwrap();
        assert_eq!(state_on, BitState::Off);
        assert_eq!(!state_on, BitState::On);
        assert_eq!(false, state_on.into());
        assert_eq!(0u16, state_on.into());
    }

    #[test]
    fn from_u16_invalid0() {
        let invalid = BitState::try_from(0xF0FF).unwrap_err();
        assert_eq!(invalid, ModbusSerializationError::InvalidValue);
    }

    #[test]
    fn from_u16_invalid1() {
        let invalid = BitState::try_from(0xFFFF).unwrap_err();
        assert_eq!(invalid, ModbusSerializationError::InvalidValue);
    }

    #[test]
    fn from_u16_invalid2() {
        let invalid = BitState::try_from(1).unwrap_err();
        assert_eq!(invalid, ModbusSerializationError::InvalidValue);
    }
}
