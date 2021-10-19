use crate::ModbusSerializationError;

/// Reads an u16 from the given modbus data. The data is considered to be big endian with msB first
/// 
/// # Errors 
/// If the data slice contains less than 2 bytes a [ModbusSerializationError::UnexpectedEOF] will be returned 
/// with its expected field set to 2 and its got field set to data.len().
pub fn read_u16(data: &[u8]) -> Result<(u16, &[u8]), crate::ModbusSerializationError> {
    if let [Some(dhi), Some(dlo)] = [data.get(0), data.get(1)] {
        Ok((u16::from_be_bytes([*dhi, *dlo]), &data[2..]))
    } else {
        Err(ModbusSerializationError::UnexpectedEOF {
            expected: 2,
            got: data.len(),
        })
    }
}

/// Reads an u16 from the given modbus data without performing bounds checks. The data is considered to be big endian with msB first.
pub unsafe fn read_u16_unchecked(data: &[u8]) -> (u16, &[u8]) {
    let word = u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]);

    //This is safe if data.len() > 2 as get_unchecked only invokes undefined behavior
    //if get would return none (idx > len)
    (word, data.get_unchecked(2..))
}

#[cfg(test)]
mod read_u16_test {
    use super::read_u16;

    #[test]
    fn read8() {
        let (w8, tail) = read_u16(&[0, 8]).unwrap();
        assert_eq!(w8, 8);
        assert_eq!(tail, []);
    }

    #[test]
    fn read0() {
        let (w8, tail) = read_u16(&[0, 0, 100]).unwrap();
        assert_eq!(w8, 0);
        assert_eq!(tail, &[100]);
    }

    #[test]
    fn read256() {
        let (w8, tail) = read_u16(&[1, 0, 100, 200, 2, 3, 4, 5]).unwrap();
        assert_eq!(w8, 256);
        assert_eq!(tail, &[100, 200, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn read_error1() {
        let (_w8, _tail) = read_u16(&[1]).unwrap();
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn read_error0() {
        let (_w8, _tail) = read_u16(&[]).unwrap();
    }
}

/// Interprets the data as an address and a quantity. They are returned in this order.
/// The data left in the slice is returned too. 
/// 
/// # Errors 
/// If the data slice contains less than 4 bytes a [ModbusSerializationError::UnexpectedEOF] will be returned 
/// with its expected field set to 4 and its got field set to data.len().
pub fn read_addr_quantity(data: &[u8]) -> Result<(u16, u16, &[u8]), ModbusSerializationError>  {
    if data.len() < 4 {
        return Err(ModbusSerializationError::UnexpectedEOF {expected: 4, got: data.len()})
    }

    Ok(unsafe { read_addr_quantity_unchecked(data) })
}

pub unsafe fn read_addr_quantity_unchecked(data: &[u8]) -> (u16, u16, &[u8]) {
    let (addr, data) = read_u16_unchecked(data);
    let (quantity, data) = read_u16_unchecked(data);

    (addr, quantity, data)
}

pub fn write_addr_quantity(
    addr: u16,
    quantity: u16,
    out: &mut [u8],
) -> Result<(), ModbusSerializationError> {
    if out.len() < 4 {
        return Err(ModbusSerializationError::InsufficientBuffer {
            expected: 4,
            got: out.len(),
        });
    }

    unsafe {
        write_addr_quantity_unchecked(addr, quantity, out);
    }

    Ok(())
}

pub unsafe fn write_addr_quantity_unchecked(addr: u16, quantity: u16, out: &mut [u8]) {
    let addr_bytes = addr.to_be_bytes();
    *out.get_unchecked_mut(0) = addr_bytes[0];
    *out.get_unchecked_mut(1) = addr_bytes[1];

    let quantity_bytes = quantity.to_be_bytes();
    *out.get_unchecked_mut(2) = quantity_bytes[0];
    *out.get_unchecked_mut(3) = quantity_bytes[1];
}
