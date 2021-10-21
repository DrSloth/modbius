/// An error type describing what can happen when parsing Modbus data from bytes 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModbusSerializationError {
    /// The input data didn't include enough bytes
    UnexpectedEOF {
        /// The number of expected bytes
        expected: usize,
        /// The number of bytes received
        got: usize
    },
    /// The given output buffer was too small to receive all data
    InsufficientBuffer {
        /// The expected minimum size of the output buffer
        expected: usize,
        /// The size of the given output buffer
        got: usize
    },
    /// An invalid value or invalid data was encountered that can not be accepted.
    /// 
    /// One example would be a value other than 0xFF00 or 0x0000 as coil value 
    /// or trying to write 0 coils/registers
    Invalid,
    /// The given value would exceed the maximum allowed size of a modbus request or request field
    TooLarge,
    /// The given data had values that logically opposed itself.
    /// 
    /// For instance if data states that `N` bytes will follow but less data follows
    Ambivalent,
    /// The given data would overflow a modbus field or is logically incoherent because of something similiar
    /// 
    /// In contrast to [ModbusSerializationError::TooLarge] this doesn't mean that a field contains too much data but rather 
    /// that multiple fields in combination would break some numeric invariant. 
    /// 
    /// For instance if a "write multiple" request
    /// would write over the 0xFFFF adddress boundary (e.g. giving addr=0xFFFE but 50 registers to write)
    Overflow,
}
