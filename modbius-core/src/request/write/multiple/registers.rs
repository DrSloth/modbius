use crate::util::AddrQuantity;

pub struct WriteMultipleRegistersRequest<'a> {
    pub addr: u16,
    pub register_values: &'a [u16],
}

impl<'a> WriteMultipleRegistersRequest<'a> {
    pub fn new(addr: u16, register_values: &'a [u16]) -> Self {
        Self {
            register_values,
            addr,
        }
    }

    pub fn from_modbus_data(
        addr: u16,
        quantity: u16,
        modbus_data: &'a [u8],
        out_registers: &'a mut [u16],
    ) -> Self {
        crate::util::registers_from_modbus_data(quantity, modbus_data, out_registers);
        Self::new(addr, out_registers)
    }

    pub unsafe fn from_modbus_data_unchecked(
        addr: u16,
        quantity: u16,
        modbus_data: &'a [u8],
        out_registers: &'a mut [u16],
    ) -> Self {
        crate::util::registers_from_modbus_data_unchecked(quantity, modbus_data, out_registers);
        Self::new(addr, out_registers)
    }

    pub fn from_modbus_data_raw(addr: u16, quantity: u16, data: &'a [u8]) -> Self {
        Self::new(
            addr,
            crate::util::registers_from_modbus_data_raw(quantity, data),
        )
    }

    pub unsafe fn from_modbus_data_raw_unchecked(addr: u16, quantity: u16, data: &'a [u8]) -> Self {
        Self::new(
            addr,
            crate::util::registers_from_modbus_data_raw_unchecked(quantity, data),
        )
    }

    pub fn as_modbus_data(&self, out_data: &mut [u8]) {
        Self::new_as_modbus_data(self.addr, self.register_values, out_data)
    }

    pub unsafe fn as_modbus_data_unchecked(&self, out_data: &mut [u8]) {
        Self::new_as_modbus_data_unchecked(self.addr, self.register_values, out_data)
    }

    pub fn new_as_modbus_data(addr: u16, register_values: &[u16], out_data: &mut [u8]) {
        if out_data.len() < AddrQuantity::SIZE as usize + 1 + register_values.len() * 2 {
            panic!()
        }

        unsafe {Self::new_as_modbus_data_unchecked(addr, register_values, out_data)}
    }

    pub unsafe fn new_as_modbus_data_unchecked(
        addr: u16,
        register_values: &[u16],
        out_data: &mut [u8],
    ) {
        *out_data.get_unchecked_mut(0) = 16;
        AddrQuantity {
            addr,
            quantity: register_values.len() as u16,
        }
        .write_to_modbus_data_unchecked(out_data.get_unchecked_mut(1..));

        core::ptr::copy_nonoverlapping(
            register_values as *const _ as *const u8,
            out_data.get_unchecked_mut(AddrQuantity::SIZE as usize) as *mut _ as *mut u8,
            register_values.len() * 2,
        );
    }
}
