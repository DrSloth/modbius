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
    /// An invalid value was encountered that can not be accepted.
    InvalidValue,
}
