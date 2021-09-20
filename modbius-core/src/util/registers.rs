pub fn registers_from_modbus_data(quantity: u16, modbus_data: &[u8], out_registers: &mut [u16]) {
    //regs.len() is guaranteed to be quantity
    if out_registers.len() < quantity as usize {
        panic!()
    }

    let regs = registers_from_modbus_data_raw(quantity, modbus_data);

    for (reg, o) in regs.iter().zip(out_registers.iter_mut()) {
        *o = u16::from_be(*reg);
    }
}

pub unsafe fn registers_from_modbus_data_unchecked(quantity: u16, modbus_data: &[u8], out_registers: &mut [u16]) {
    let regs = registers_from_modbus_data_raw_unchecked(quantity, modbus_data);

    for (i, reg) in regs.iter().enumerate() {
        *out_registers.get_unchecked_mut(i) = u16::from_be(*reg);
    }
}

pub fn registers_from_modbus_data_raw<'a>(quantity: u16, data: &'a [u8]) -> &'a [u16] {
    if data.len() * 2 < quantity as usize {
        panic!()
    }

    unsafe {registers_from_modbus_data_raw_unchecked(quantity, data)}
}

pub unsafe fn registers_from_modbus_data_raw_unchecked<'a>(quantity: u16, data: &'a [u8]) -> &'a [u16] {
    core::slice::from_raw_parts(data.as_ptr() as *const u16, quantity as usize)
}

pub fn registers_from_modbus_data_raw_mut<'a>(quantity: u16, data: &'a mut [u8]) -> &'a mut [u16] {
    if data.len() * 2 < quantity as usize {
        panic!()
    }

    unsafe {registers_from_modbus_data_raw_unchecked_mut(quantity, data)}
}

pub unsafe fn registers_from_modbus_data_raw_unchecked_mut<'a>(quantity: u16, data: &'a mut [u8]) -> &'a mut [u16] {
    core::slice::from_raw_parts_mut(data.as_ptr() as *mut u16, quantity as usize)
}

pub fn register_to_native_endian(register: u16) -> u16 {
    u16::from_be(register)
}

pub fn registers_data_to_native_endian(registers: &mut [u8]) {
    for bytes in registers.chunks_exact_mut(2) {
        if cfg!(target_endian = "little") {
            unsafe {
                *bytes.get_unchecked_mut(0) = *bytes.get_unchecked(1);
                *bytes.get_unchecked_mut(1) = *bytes.get_unchecked(0);
            }
        }
    }
}
