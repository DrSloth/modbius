use crate::ModbusSerializationError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RegisterSlice<'a> {
    bytes: &'a [u8],
}

impl<'a> RegisterSlice<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, ModbusSerializationError> {
        if bytes.len() % 2 == 0 {
            Ok(Self { bytes })
        } else {
            Err(ModbusSerializationError::Invalid)
        }
    }

    pub unsafe fn new_unchecked(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn get(self, idx: usize) -> Option<u16> {
        (self.bytes.len() >= (idx + 1) * 2).then(|| unsafe { self.get_unchecked(idx) })
    }

    pub unsafe fn get_unchecked(self, idx: usize) -> u16 {
        let bidx = idx * 2;
        let bytes = self.bytes.get_unchecked(bidx..=(bidx + 1));
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
    
    pub fn len(self) -> usize {
        self.bytes.len() / 2
    }

    pub fn bytes_len(self) -> usize {
        self.bytes.len()
    }

    pub fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> TryFrom<&'a [u8]> for RegisterSlice<'a> {
    //TODO this should be a different error type
    type Error = ModbusSerializationError;
    fn try_from(data: &'a [u8]) -> Result<Self, ModbusSerializationError> {
        Self::new(data)
    }
} 
