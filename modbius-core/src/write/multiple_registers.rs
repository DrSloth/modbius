use crate::{registerslice::RegisterSlice, util, ModbusSerializationError, PublicModbusFunction};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct WriteMultipleRegisters<'a> {
    addr: u16,
    registers: RegisterSlice<'a>,
}

impl<'a> WriteMultipleRegisters<'a> {
    pub const MODBUS_FUNCTION_CODE: PublicModbusFunction =
        PublicModbusFunction::WriteMultipleRegisters;
    /// The minimum size required for a request like Self
    /// 
    /// Normally the minimum size of a [WriteMultipleRegisters] request consists out of 8 bytes:
    /// [HEADER_SIZE](WriteMultipleRegisters::HEADER_SIZE) + at least one register (2 byte) but 
    /// for input data the function code (1 byte) will already be read.
    pub const MIN_INPUT_SIZE: usize = 7;
    /// The header size of a [WriteMultipleRegisters] request.
    /// 
    /// The header consists of: 
    /// function code (1 byte) + starting address (2 byte) + quantity (2 byte) + number of following bytes (1 byte)
    pub const HEADER_SIZE: usize = 6;
    /// The minimum size required to write Self to a slice
    /// 
    /// Same as [MIN_INPUT_SIZE](WriteMultipleRegisters::MIN_INPUT_SIZE) just that it also contains the function code
    pub const MIN_OUTPUT_SIZE: usize = 8;

    /// Create a new request to write multiple registers.
    ///
    /// # Errors
    /// It is not allowed to write 0 or more than 123 registers. If you try to write 0 [ModbusSerializationError::Invalid]
    /// will be returned.
    /// If the len of registers exceeds 123 [ModbusSerializationError::TooLarge] will be returned.
    /// [ModbusSerializationError::Overflow] will be returned if addr + registers.len() overflows the 0xFFFF boundary
    pub fn new(addr: u16, registers: RegisterSlice<'a>) -> Result<Self, ModbusSerializationError> {
        match registers.len() {
            0 => Err(ModbusSerializationError::Invalid),
            n if n > 123 => Err(ModbusSerializationError::TooLarge),
            n if addr.overflowing_add(n as u16).1 => Err(ModbusSerializationError::Overflow),
            _ => Ok(unsafe { Self::new_unchecked(addr, registers) }),
        }
    }

    /// Create a new request to write multiple registers without checking the quantity of registers  
    ///
    /// # Safety
    /// This function doesn't directly invoke undefined behavior if called with registers len being 0 or larger than 123,
    /// but all other code MAY make assumptions based on the length of registers being in this range. As violating this
    /// invariant could invoke undefined behavior later it is konservatively set as unsafe.
    pub unsafe fn new_unchecked(addr: u16, registers: RegisterSlice<'a>) -> Self {
        Self { addr, registers }
    }

    pub fn addr(self) -> u16 {
        self.addr
    }

    pub fn registers(self) -> RegisterSlice<'a> {
        self.registers
    }

    pub fn from_data(data: &'a [u8]) -> Result<Self, ModbusSerializationError> {
        if data.len() < Self::MIN_INPUT_SIZE {
            Err(ModbusSerializationError::UnexpectedEOF {
                expected: Self::MIN_INPUT_SIZE,
                got: data.len(),
            })
        } else {
            unsafe { Self::from_data_unchecked(data) }
        }
    }

    pub unsafe fn from_data_unchecked(data: &'a [u8]) -> Result<Self, ModbusSerializationError> {
        let (addr, data) = util::read_u16_unchecked(data);

        let (quantity, data) = util::read_u16_unchecked(data);
        let nbytes = *data.get_unchecked(0) as usize;

        if nbytes / 2 != quantity as usize {
            return Err(ModbusSerializationError::Ambivalent);
        }

        if let Some(data) = data.get(1..(nbytes + 1)) {
            let registers = RegisterSlice::new(data)?;
            Self::new(addr, registers)
        } else {
            Err(ModbusSerializationError::UnexpectedEOF {
                // We subtract 1 byte because of the missing function code
                expected: nbytes + Self::HEADER_SIZE - 1,
                // We subtract 2 bytes because of the nbytes byte that we did not advance past and the missing function code.
                got: Self::HEADER_SIZE - 1 + data.len() - 1,
            })
        }
    }

    /// Get how many bytes this request needs to be encoded
    pub fn data_size(self) -> usize {
        self.registers.bytes_len() + Self::HEADER_SIZE
    }

    pub fn write_to_slice(self, out: &mut [u8]) -> Result<(), ModbusSerializationError> {
        let data_size = self.data_size();

        if out.len() < data_size {
            Err(ModbusSerializationError::InsufficientBuffer {
                expected: data_size,
                got: out.len(),
            })
        } else {
            unsafe { self.write_to_slice_unchecked(out) };
            Ok(())
        }
    }

    pub unsafe fn write_to_slice_unchecked(self, out: &mut [u8]) {
        *out.get_unchecked_mut(0) = Self::MODBUS_FUNCTION_CODE as u8;

        let addr_bytes = self.addr.to_be_bytes();
        *out.get_unchecked_mut(1) = addr_bytes[0];
        *out.get_unchecked_mut(2) = addr_bytes[1];

        let quantity = self.registers.len() as u16;
        let quantity_bytes = quantity.to_be_bytes();
        *out.get_unchecked_mut(3) = quantity_bytes[0];
        *out.get_unchecked_mut(4) = quantity_bytes[1];

        let nbytes = self.registers.bytes_len() as u8;
        *out.get_unchecked_mut(5) = nbytes;

        out.get_unchecked_mut(Self::HEADER_SIZE..(nbytes as usize + 6))
            .copy_from_slice(self.registers.bytes());
    }
}

#[cfg(test)]
mod test_write_registers {
    use super::*;

    #[test]
    fn create_new() {
        let regs_data = &[0,1,255,255,1,0];
        let regs = RegisterSlice::new(regs_data).unwrap();
        let req = WriteMultipleRegisters::new(10, regs).unwrap();

        assert_eq!(req.addr(), 10);
        assert_eq!(req.registers().bytes(), regs_data);
        assert_eq!(req.registers().get(0), Some(1));
        assert_eq!(req.registers().get(1), Some(u16::MAX));
        assert_eq!(req.registers().get(2), Some(256));
        assert_eq!(req.data_size(), regs_data.len() + 6);
    }

    #[test]
    fn create_fail_invalid_empty() {
        let regs = RegisterSlice::new(&[]).unwrap();
        let err = WriteMultipleRegisters::new(10, regs).unwrap_err();

        assert_eq!(err, ModbusSerializationError::Invalid);
    }

    #[test]
    fn create_fail_too_large() {
        let regs = [0;123 * 2 + 2];
        let regs = RegisterSlice::new(&regs).unwrap();
        let err = WriteMultipleRegisters::new(10, regs).unwrap_err();

        assert_eq!(err, ModbusSerializationError::TooLarge);
    }


    #[test]
    fn create_fail_overflow() {
        let regs = RegisterSlice::new(&[0,1,2,3]).unwrap();
        let err = WriteMultipleRegisters::new(0xFFFE, regs).unwrap_err();

        assert_eq!(err, ModbusSerializationError::Overflow);
    }

    #[test]
    fn create_new_eq_create_new_unchecked() {
        let regs = RegisterSlice::new(&[0,1,2,3]).unwrap();
        let req = WriteMultipleRegisters::new(10, regs).unwrap();
        // For every valid data unchecked versions should deliver the same value as checked versions
        let req_unchecked = unsafe { WriteMultipleRegisters::new_unchecked(10, regs) };

        assert_eq!(req.addr(), 10);
        assert_eq!(req.registers().bytes(), &[0,1,2,3]);

        assert_eq!(req_unchecked.addr(), 10);
        assert_eq!(req_unchecked.registers().bytes(), &[0,1,2,3]);
        
        assert_eq!(req, req_unchecked);
    }

    #[test]
    fn from_data_spec() {
        //Example from modbus spec
        let data = [0, 1, 0, 2, 4, 0, 0xA, 1, 2];
        let req = WriteMultipleRegisters::from_data(&data).unwrap();

        assert_eq!(req.addr(), 1);
        //NBytes field + header
        assert_eq!(req.data_size(), 4 + 6); 
        assert_eq!(req.registers().bytes(), &[0, 0xA, 1, 2]);
        assert_eq!(req.registers().get(0), Some(0xA));
        assert_eq!(req.registers().get(1), Some(258));
    }

    #[test]
    fn from_data() {
        //Example from modbus spec
        let data = [0, 1, 0, 2, 4, 0, 2, 0, 3];
        let req = WriteMultipleRegisters::from_data(&data).unwrap();

        assert_eq!(req.addr(), 1);
        //NBytes field + header
        assert_eq!(req.data_size(), 4 + 6); 
        assert_eq!(req.registers().bytes(), &[0, 2, 0, 3]);
        assert_eq!(req.registers().get(0), Some(2));
        assert_eq!(req.registers().get(1), Some(3));
    }

    #[test]
    fn from_data_fail_invalid0() {
        let data = [0,0,0,0,0,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::Invalid);
    }

    #[test]
    fn from_data_fail_unexpected_eof0() {
        let data = [];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::UnexpectedEOF {got: 0, expected: WriteMultipleRegisters::MIN_INPUT_SIZE});
    }

    #[test]
    fn from_data_fail_unexpected_eof1() {
        let data = [0,0,0,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::UnexpectedEOF {got: 5, expected: WriteMultipleRegisters::MIN_INPUT_SIZE});
    }

    #[test]
    fn from_data_fail_unexpected_eof2() {
        let data = [0,0,0,20,20];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::UnexpectedEOF {got: 5, expected: WriteMultipleRegisters::MIN_INPUT_SIZE});
    }

    #[test]
    fn from_data_fail_ambivalent0() {
        let data = [0,0,0,200,3,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::Ambivalent);
    }

    #[test]
    fn from_data_fail_ambivalent1() {
        let data = [0,0,0,20,20,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::Ambivalent);
    }

    #[test]
    fn from_data_fail_unexpected_eof_parse_registers0() {
        let data = [0,0,0,4,8,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::UnexpectedEOF {got: data.len(), expected: 13});
    }

    #[test]
    fn from_data_fail_unexpected_eof_parse_registers1() {
        let data = [0,0,0,5,10,0,0,0,0,0,0,0,0];
        let err = WriteMultipleRegisters::from_data(&data).unwrap_err();

        assert_eq!(err, ModbusSerializationError::UnexpectedEOF {got: data.len(), expected: 15});
    }

    #[test]
    fn write_to_slice0() {
        let data = [0, 1, 0, 2, 4, 0, 0xA, 1, 2];
        let req = WriteMultipleRegisters::from_data(&data).unwrap();

        assert_eq!(req.data_size(), 10);
        assert_eq!(req.registers().bytes(), &[0, 0xA, 1, 2]);
        
        let mut out = [0u8;10];
        req.write_to_slice(&mut out).unwrap();

        assert_eq!(out[0], WriteMultipleRegisters::MODBUS_FUNCTION_CODE as u8);
        assert_eq!(&out[1..], &data);
    }

    #[test]
    fn write_to_slice1() {
        let data = [0, 1, 0, 3, 6, 5, 6, 1, 2, 22, 42];
        let req = WriteMultipleRegisters::from_data(&data).unwrap();

        assert_eq!(req.data_size(), 12);
        assert_eq!(req.registers().bytes(), &[5, 6, 1, 2, 22, 42]);
        
        let mut out = [0u8;12];
        req.write_to_slice(&mut out).unwrap();

        assert_eq!(out[0], WriteMultipleRegisters::MODBUS_FUNCTION_CODE as u8);
        assert_eq!(&out[1..], &data);
    }

    #[test]
    fn write_to_slice_fail_insufficient_buffer() {
        let data = [0, 1, 0, 2, 4, 0, 0xA, 1, 2];
        let req = WriteMultipleRegisters::from_data(&data).unwrap();

        assert_eq!(req.data_size(), 10);
        assert_eq!(req.registers().bytes(), &[0, 0xA, 1, 2]);
        
        let mut out = [0u8;9];
        let err = req.write_to_slice(&mut out).unwrap_err();

        assert_eq!(err, ModbusSerializationError::InsufficientBuffer {expected: req.data_size(), got: 9})
    }

    //TODO check correctness of unchecked versions with correct data
}
