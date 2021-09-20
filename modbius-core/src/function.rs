use core::{convert::From, mem::transmute};

//TODO define exception codes function codes here and exception codes in a different module

//TODO implement TryFrom with fitting error types

/// Gets the function code of the given modbus data.
///
/// None is returned if data contains less than 2 bytes
pub fn get_function(data: &[u8]) -> (Option<ModbusFunction>, Option<&[u8]>) {
    (
        data.get(0).map(|byte| ModbusFunction::from(*byte)),
        data.get(1..),
    )
}

/// Gets the function code of the given modbus data.
///
/// # Safety
/// The provided data has to contain at least 2 bytes, if only one byte is present use ModbusFunction::from.
/// Providing data with less than two bytes is undefined behavior
pub unsafe fn get_function_unchecked(data: &[u8]) -> (ModbusFunction, &[u8]) {
    (
        ModbusFunction::from(*data.get_unchecked(0)),
        data.get_unchecked(1..),
    )
}

/// Any normal ModbusFunction which is not an exception.
///
/// ModbusFunctions exist in the range 1..=127. <br/>
/// The general idea is that the first thing when receiving modbus data is to read the ModbusFunction and match over it  
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModbusFunction {
    /// Public defined function
    Public(PublicModbusFunction),
    /// Custom defined function
    Custom(CustomModbusFunction),
    /// Non custom definable, non publicly defined function
    Other(OtherModbusFunction),
}

impl From<u8> for ModbusFunction {
    fn from(fcode: u8) -> Self {
        if CustomModbusFunction::is_custom_function(fcode) {
            Self::Custom(unsafe { CustomModbusFunction::new_unchecked(fcode) })
        } else if PublicModbusFunction::is_public_function(fcode) {
            Self::Public(unsafe { PublicModbusFunction::new_unchecked(fcode) })
        } else {
            Self::Other(unsafe { OtherModbusFunction::new_unchecked(fcode) })
        }
    }
}

impl Into<u8> for ModbusFunction {
    fn into(self) -> u8 {
        match self {
            Self::Custom(c) => c.into(),
            Self::Public(p) => p.into(),
            Self::Other(o) => o.into(),
        }
    }
}

impl From<PublicModbusFunction> for ModbusFunction {
    fn from(fc: PublicModbusFunction) -> Self {
        Self::Public(fc)
    }
}

impl From<CustomModbusFunction> for ModbusFunction {
    fn from(fc: CustomModbusFunction) -> Self {
        Self::Custom(fc)
    }
}

impl From<OtherModbusFunction> for ModbusFunction {
    fn from(fc: OtherModbusFunction) -> Self {
        Self::Other(fc)
    }
}

/// An enum mapping all publicly defined modbus function codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PublicModbusFunction {
    /// Function code 0 is defined as truly invalid
    Invalid = 0,
    ReadCoils = 1,
    ReadDiscreteInputs = 2,
    ReadHoldingRegisters = 3,
    ReadInputRegisters = 4,
    WriteSingleCoil = 5,
    WriteSingleRegister = 6,
    ReadExceptionStatus = 7,
    Diagnostics = 8,
    Program484 = 9,
    Poll484 = 10,
    GetCommEventCounter = 11,
    GetCommEventLog = 12,
    ProgramController = 13,
    PollController = 14,
    WriteMultipleCoils = 15,
    WriteMultipleregisters = 16,
    ReportServerID = 17,
    Program884M84 = 18,
    ResetCommLink = 19,
    ReadFileRecord = 20,
    WriteFileRecord = 21,
    MaskWriteRegister = 22,
    ReadWriteMultipleRegisters = 23,
    ReadFIFOQueue = 24,
    EncapsulatedInterfaceTransport = 43,
}

impl PublicModbusFunction {
    pub fn new(code: u8) -> Self {
        if Self::is_public_function(code) {
            unsafe { Self::new_unchecked(code) }
        } else {
            Self::Invalid
        }
    }

    pub fn is_public_function(code: u8) -> bool {
        (0..=24).contains(&code) || code == 43
    }

    /// Parses a ModbusFunction from a byte
    ///
    /// # Safety
    /// Providing a code where [is_public_function](PublicModbusFunction::is_public_function) code invokes undefined behavior
    pub unsafe fn new_unchecked(code: u8) -> Self {
        transmute(code)
    }
}

impl From<u8> for PublicModbusFunction {
    fn from(code: u8) -> Self {
        Self::new(code)
    }
}

impl Into<u8> for PublicModbusFunction {
    fn into(self) -> u8 {
        self as u8
    }
}

/// A custom defined modbus function
///
/// This struct is a function code which is outside of the range of public or reserved function codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomModbusFunction(u8);

impl CustomModbusFunction {
    pub fn new(code: u8) -> Option<Self> {
        Self::is_custom_function(code).then(|| unsafe { Self::new_unchecked(code) })
    }

    pub fn is_custom_function(code: u8) -> bool {
        (65..=72).contains(&code) || (100..=110).contains(&code)
    }

    /// Parses a custom modbus function from the given byte
    ///
    /// # Safety
    /// Providing a code where [is_custom_function](Self::is_custom_function) yields false doesn't
    /// invoke direct undefined behavior but undefined behavior as defined by this crate
    pub unsafe fn new_unchecked(code: u8) -> Self {
        Self(code)
    }
}

impl Into<u8> for CustomModbusFunction {
    fn into(self) -> u8 {
        self.0
    }
}

/// Non custom definable, non publicly defined function code
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OtherModbusFunction(u8);

impl OtherModbusFunction {
    pub fn new(code: u8) -> Option<Self> {
        Self::is_other_function(code).then(|| unsafe { Self::new_unchecked(code) })
    }

    pub fn is_other_function(code: u8) -> bool {
        !PublicModbusFunction::is_public_function(code)
            && !CustomModbusFunction::is_custom_function(code)
    }

    /// Parses a custom modbus function from the given byte
    ///
    /// # Safety
    /// Providing a code where [is_other_function](Self::is_other_function) yields false doesn't
    /// invoke direct undefined behavior but undefined behavior as defined by this crate
    pub unsafe fn new_unchecked(code: u8) -> Self {
        Self(code)
    }
}

impl Into<u8> for OtherModbusFunction {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::{ModbusFunction, PublicModbusFunction, CustomModbusFunction, OtherModbusFunction, get_function};

    #[test]
    fn invalid() {
        assert_eq!(
            ModbusFunction::from(0),
            ModbusFunction::Public(PublicModbusFunction::Invalid)
        )
    }

    #[test]
    fn custom() {
        assert_eq!(
            ModbusFunction::from(66),
            ModbusFunction::Custom(CustomModbusFunction::new(66).unwrap())
        );
        assert_eq!(
            ModbusFunction::from(66),
            ModbusFunction::Custom(unsafe { CustomModbusFunction::new_unchecked(66) })
        );
    }

    #[test]
    fn other() {
        assert_eq!(
            ModbusFunction::from(42),
            ModbusFunction::Other(OtherModbusFunction::new(42).unwrap())
        );
        assert_eq!(
            ModbusFunction::from(42),
            ModbusFunction::Other(unsafe { OtherModbusFunction::new_unchecked(42) })
        );
    }

    #[test]
    fn invalid_from_data_no_tail() {
        let data = [0];
        let (function, tail) = get_function(&data);
        assert_eq!(function, Some(ModbusFunction::Public(PublicModbusFunction::Invalid)));
        assert_eq!(tail, Some(&[] as &[u8]));
    }

    #[test]
    fn invalid_from_data_no_data() {
        let data = [];
        let (function, tail) = get_function(&data);
        assert_eq!(function, None);
        assert_eq!(tail, None);
    }


    #[test]
    fn invalid_from_data() {
        let data = [0, 1];
        let (function, tail) = get_function(&data);
        assert_eq!(function, Some(ModbusFunction::Public(PublicModbusFunction::Invalid)));
        assert_eq!(tail, Some(&[1u8] as &[u8]));
    }

    #[test]
    fn read_coils_from_data() {
        let data = [1, 1];
        let (function, tail) = get_function(&data);
        assert_eq!(function, Some(ModbusFunction::Public(PublicModbusFunction::ReadCoils)));
        assert_eq!(tail, Some(&[1u8] as &[u8]));
    }
}
