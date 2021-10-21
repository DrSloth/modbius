use core::convert::TryFrom;

use crate::{BitState, ModbusSerializationError, PublicModbusFunction, util};

/// Request structure to write single coil
#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriteSingleCoil {
    pub addr: u16,
    pub state: BitState,
}

impl WriteSingleCoil {
    pub const MODBUS_FUNCTION_CODE: PublicModbusFunction = PublicModbusFunction::WriteSingleCoil;

    /// Create a new request to write a single coil at addr to state
    pub fn new(addr: u16, state: BitState) -> Self {
        Self { addr, state }
    }

    /// Parse this request from the given modbus data
    ///
    /// The data should only consist out of the address and quantity as the slave id function
    /// will be already read through other means.
    pub fn from_data(data: &[u8]) -> Result<(Self, &[u8]), ModbusSerializationError> {
        if data.len() < 4 {
            Err(ModbusSerializationError::UnexpectedEOF {
                expected: 4,
                got: data.len(),
            })
        } else {
            unsafe { Self::from_data_unchecked(data) }
        }
    }

    /// Parse this request from the given modbus data without bounds checks.
    ///
    /// The data should only consist out of the address and quantity as the slave id function
    /// will be already read through other means.
    ///
    /// # Safety
    /// This function causes undefined behavior if the len of data is smaller than 4
    pub unsafe fn from_data_unchecked(data: &[u8]) -> Result<(Self, &[u8]), ModbusSerializationError> {
        let (addr, data) = util::read_u16_unchecked(data);
        let (state, data) = util::read_u16_unchecked(data);
        let state = BitState::try_from(state)?;

        Ok((Self::new(addr, state), data))
    }

    pub fn into_data(self) -> [u8;5] {
        let addr_bytes = self.addr.to_be_bytes();
        let state_bytes = u16::to_be_bytes(self.state.into());

        [Self::MODBUS_FUNCTION_CODE as u8, addr_bytes[0], addr_bytes[1], state_bytes[0], state_bytes[1]]
    }

    /// Write this request to the slice as modbus data
    pub fn write_to_slice(
        self,
        out: &mut [u8],
    ) -> Result<(), ModbusSerializationError> {
        if out.len() < 5 {
            return Err(ModbusSerializationError::InsufficientBuffer {
                expected: 5,
                got: out.len(),
            });
        }

        unsafe { self.write_to_slice_unchecked(out) };
        Ok(())
    }

    /// Write this request to the slice as modbus data without bounds checking.
    ///
    /// # Safety
    /// This function invokes undefined behavior if the len of data is less than 5
    pub unsafe fn write_to_slice_unchecked(self, out: &mut [u8]) {
        out.get_unchecked_mut(0..5).copy_from_slice(&self.into_data());
    }
}
