mod registers;

pub use registers::*;

pub fn get_write_quantity(data: &[u8]) -> (Option<u16>, &[u8]) {
    if data.len() < 3 {
        (None, data)
    } else {
        let (quantity, data) = unsafe { get_write_quantity_unchecked(data) };
        (Some(quantity), data)
    }
}

pub unsafe fn get_write_quantity_unchecked(data: &[u8]) -> (u16, &[u8]) {
    (
        u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]),
        data.get_unchecked(2..),
    )
}

pub fn get_start_addr(data: &[u8]) -> (Option<u16>, &[u8]) {
    if data.len() < 3 {
        (None, data)
    } else {
        let (quantity, data) = unsafe { get_write_quantity_unchecked(data) };
        (Some(quantity), data)
    }
}

pub unsafe fn get_start_addr_unchecked(data: &[u8]) -> (u16, &[u8]) {
    (
        u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]),
        data.get_unchecked(2..),
    )
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddrQuantity {
    pub addr: u16,
    pub quantity: u16,
}

impl AddrQuantity {
    pub const SIZE: u16 = 4;

    pub fn from_modbus_data(data: &[u8]) -> (Option<AddrQuantity>, &[u8]) {
        if data.len() < 3 {
            (None, data)
        } else {
            let (me, data) = unsafe { Self::from_modbus_data_unchecked(data) };
            (Some(me), data)
        }
    }
    
    pub unsafe fn from_modbus_data_unchecked(data: &[u8]) -> (AddrQuantity, &[u8]) {
        (
            Self {
                addr: u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]),
                quantity: u16::from_be_bytes([*data.get_unchecked(2), *data.get_unchecked(3)]),
            },
            data.get_unchecked(4..),
        )
    }

    pub fn write_to_modbus_data(self, out_data: &mut [u8]) {
        if out_data.len() < 4 {
            panic!()
        }

        unsafe { self.write_to_modbus_data_unchecked(out_data) }
    }

    pub unsafe fn write_to_modbus_data_unchecked(self, out_data: &mut [u8]) {
        let addr_bytes = self.addr.to_be_bytes();
        *out_data.get_unchecked_mut(0) = addr_bytes[0];
        *out_data.get_unchecked_mut(1) = addr_bytes[1];
        let quantity_bytes = self.quantity.to_be_bytes();
        *out_data.get_unchecked_mut(2) = quantity_bytes[0];
        *out_data.get_unchecked_mut(3) = quantity_bytes[1];
    }
}
