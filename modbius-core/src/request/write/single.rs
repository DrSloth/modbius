use crate::function::PublicModbusFunction;

pub struct WriteSingleCoilRequest {
    pub addr: u16,
    pub state: bool,
}

impl WriteSingleCoilRequest {
    pub const fn new(addr: u16, state: bool) -> Self {
        Self { addr, state }
    }

    pub fn from_modbus_data(data: &[u8]) -> Option<Self> {
        (data.len() >= 4).then(|| unsafe { Self::from_modbus_data_unchecked(data) })
    }

    pub unsafe fn from_modbus_data_unchecked(data: &[u8]) -> Self {
        let addr = u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]);
        let state = u16::from_be_bytes([*data.get_unchecked(2), *data.get_unchecked(3)]);

        Self::from_addr_state_unchecked(addr, state)
    }

    pub fn from_addr_state(addr: u16, state: u16) -> Option<Self> {
        if state != 0 && state != 0xFF00 {
            None
        } else {
            Some(unsafe { Self::from_addr_state_unchecked(addr, state) })
        }
    }

    pub unsafe fn from_addr_state_unchecked(addr: u16, state: u16) -> Self {
        Self {
            addr,
            state: state != 0,
        }
    }

    /// Returns this requests as a complete modbus data item request.
    ///
    /// The returned data consists of [function code, address hi, address lo, quantity hi, quantity lo]
    pub fn as_modbus_data(self) -> [u8; 5] {
        Self::new_as_modbus_data(self.addr, self.state)
    }

    /// The data consists of [function code, addr hi, addr lo, quantity hi, quantity lo]
    pub fn new_as_modbus_data(addr: u16, state: bool) -> [u8; 5] {
        let addr_bytes = addr.to_be_bytes();
        [
            PublicModbusFunction::WriteSingleCoil as u8,
            addr_bytes[0],
            addr_bytes[1],
            if state { 0xFF } else { 0x00 },
            00,
        ]
    }
}

pub struct WriteSingleRegisterRequest {
    pub addr: u16,
    pub val: u16,
}

impl WriteSingleRegisterRequest {
    pub const fn new(addr: u16, val: u16) -> Self {
        Self { addr, val }
    }

    pub fn from_modbus_data(data: &[u8]) -> Option<Self> {
        (data.len() >= 4).then(|| unsafe { Self::from_modbus_data_unchecked(data) })
    }

    pub unsafe fn from_modbus_data_unchecked(data: &[u8]) -> Self {
        let addr = u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]);
        let val = u16::from_be_bytes([*data.get_unchecked(2), *data.get_unchecked(3)]);

        Self { addr, val }
    }

    /// Returns this requests as a complete modbus data item request.
    ///
    /// The returned data consists of [function code, address hi, address lo, quantity hi, quantity lo]
    pub fn as_modbus_data(self) -> [u8; 5] {
        Self::new_as_modbus_data(self.addr, self.val)
    }

    /// The data consists of [function code, addr hi, addr lo, quantity hi, quantity lo]
    pub fn new_as_modbus_data(addr: u16, val: u16) -> [u8; 5] {
        let addr_bytes = addr.to_be_bytes();
        let val_bytes = val.to_be_bytes();
        [
            PublicModbusFunction::WriteSingleCoil as u8,
            addr_bytes[0],
            addr_bytes[1],
            val_bytes[0],
            val_bytes[1],
        ]
    }
}
