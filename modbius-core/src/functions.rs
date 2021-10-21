//! Mapping of modbus function codes and routines to read and check them
//! 
//! The main way to interact with modbus function codes is to construct [ModbusFunction] instances
//! The [PublicModbusFunction] enum maps publicly documented function codes.
//! For reference see <https://www.modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf>

use core::{convert::From, mem};

/// Gets the function code of the given modbus data.
///
/// None is returned if data contains less than 1 byte
pub fn get_function(data: &[u8]) -> (Option<ModbusFunction>, Option<&[u8]>) {
    (
        data.get(0).map(|byte| ModbusFunction::new(*byte)),
        data.get(1..),
    )
}

/// Gets the function code of the given modbus data.
///
/// # Safety
/// Providing data with less than one byte is undefined behavior
pub unsafe fn get_function_unchecked(data: &[u8]) -> (ModbusFunction, &[u8]) {
    (
        ModbusFunction::new(*data.get_unchecked(0)),
        data.get_unchecked(1..),
    )
}

/// A modbus function. This struct may store public as well as custom functions. The only invalid function is 0.
///
/// This structure is a new type wrapper over u8 which adds function code identification methods.
#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModbusFunction(pub u8);

impl ModbusFunction {
    pub const fn new(function_code: u8) -> Self {
        Self(function_code)
    }

    /// Create a modbus function from a public Modbus function
    pub const fn new_public(public_function: PublicModbusFunction) -> Self {
        Self(public_function as u8)
    }

    /// Checks if this function matches the given public function
    pub const fn is(self, public_function: PublicModbusFunction) -> bool {
        self.0 == Self::new_public(public_function).0
    }

    /// Check if this function code is valid. Only 0 is invalid.
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }

    /// Checks if this function is a publicly documented modbus function.
    ///
    /// This function returns true if this is a public reserved AND publicly documented function specified in the
    /// [modbus spec](https://www.modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf)
    pub const fn is_public(self) -> bool {
        PublicModbusFunction::is_public_function(self.0)
    }

    /// Checks if this function is a public reserved function. Any valid and non custom function is reserved.
    pub const fn is_public_reserved(self) -> bool {
        self.is_valid() && !self.is_custom()
    }

    /// Checks if this function is a custom function code
    pub const fn is_custom(self) -> bool {
        self.0 >= 65 && self.0 <= 72 || self.0 >= 100 && self.0 <= 110
    }

    /// Checks if this function code is an expection code. Which means that self >= 128
    pub const fn is_exception(self) -> bool {
        self.0 >= 128
    }

    /// Gets the function code of the given modbus data.
    ///
    /// None is returned if data contains less than 1 byte
    pub fn from_data(data: &[u8]) -> (Option<Self>, Option<&[u8]>) {
        get_function(data)
    }

    /// Gets the function code of the given modbus data.
    ///
    /// # Safety
    /// Providing data with less than one byte is undefined behavior
    pub unsafe fn from_data_unchecked(data: &[u8]) -> (Self, &[u8]) {
        get_function_unchecked(data)
    }
}

impl PartialEq<PublicModbusFunction> for ModbusFunction {
    fn eq(&self, other: &PublicModbusFunction) -> bool {
        self.is(*other)
    }
}

impl From<u8> for ModbusFunction {
    fn from(b: u8) -> Self {
        Self::new(b)
    }
}

impl Into<u8> for ModbusFunction {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<PublicModbusFunction> for ModbusFunction {
    fn from(public_function: PublicModbusFunction) -> Self {
        Self::new_public(public_function)
    }
}

macro_rules! public_modbus_function {
    ($($name:ident = $fcode:expr,)*) => {
        /// An enum mapping all publicly documented modbus function codes
        ///
        /// [Invalid](PublicModbusFunction::Invalid) is defined as 0, any undocumented or any custom function code.
        /// All publicly documented function codes are specified here <https://www.modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf>
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum PublicModbusFunction {
            $($name = $fcode,)*
        }

        impl PublicModbusFunction {
            /// Check if the given code is a publicly documented and valid function code
            pub const fn is_public_function(code: u8) -> bool {
                if code == 0 {
                    return false;
                }
                match code {
                    $($fcode => true,)*
                    _ => false,
                }
            }
        }
    };
}

public_modbus_function! {
    Invalid = 0,
    ReadCoils = 1,
    ReadDiscreteInputs = 2,
    ReadHoldingRegisters = 3,
    ReadInputRegisters = 4,
    WriteSingleCoil = 5,
    WriteSingleRegister = 6,
    ReadExceptionStatus = 7,
    Diagnostics = 8,
    GetCommEventCounter = 11,
    GetCommEventLog = 12,
    WriteMultipleCoils = 15,
    WriteMultipleRegisters = 16,
    ReportServerID = 17,
    ReadFileRecord = 20,
    WriteFileRecord = 21,
    MaskWriteRegister = 22,
    ReadWriteMultipleRegisters = 23,
    ReadFIFOQueue = 24,
    EncapsulatedInterfaceTransport = 43,
}

impl PublicModbusFunction {
    /// Create a [PublicModbusFunction] from a single byte. Every no public function code returns [PublicModbusFunction::Invalid]
    pub fn new(code: u8) -> Self {
        if Self::is_public_function(code) {
            unsafe { Self::new_unchecked(code) }
        } else {
            Self::Invalid
        }
    }

    /// Transmutes a ModbusFunction from a byte
    ///
    /// # Safety
    /// Providing a code where [is_public_function](PublicModbusFunction::is_public_function) code invokes undefined behavior
    pub unsafe fn new_unchecked(code: u8) -> Self {
        mem::transmute(code)
    }
}

impl PartialEq<ModbusFunction> for PublicModbusFunction {
    fn eq(&self, other: &ModbusFunction) -> bool {
        other == self
    }
}

impl From<ModbusFunction> for PublicModbusFunction {
    fn from(mf: ModbusFunction) -> Self {
        Self::new(mf.0)
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

#[cfg(test)]
mod test {
    use super::{get_function, ModbusFunction, PublicModbusFunction};

    #[test]
    fn invalid() {
        assert!(!ModbusFunction::from(0).is_valid())
    }

    #[test]
    fn public() {
        assert!(ModbusFunction::from(1).is_public());
    }

    #[test]
    fn custom() {
        let mbf = ModbusFunction::from(69);
        assert!(mbf.is_custom());
    }

    #[test]
    fn custom_all() {
        for i in 65..=72 {
            let mbf = ModbusFunction::from(i);
            assert!(mbf.is_custom() && mbf.is_valid());
        }

        for i in 100..=110 {
            let mbf = ModbusFunction::from(i);
            assert!(mbf.is_custom() && mbf.is_valid());
        }
    }

    #[test]
    fn invalid_from_data_no_tail() {
        let data = [0];
        let (function, tail) = get_function(&data);
        assert_eq!(
            function,
            Some(ModbusFunction::new_public(PublicModbusFunction::Invalid))
        );
        assert_eq!(tail, Some(&[] as &[u8]));
    }

    #[test]
    fn no_data() {
        let data = [];
        let (function, tail) = get_function(&data);
        assert_eq!(function, None);
        assert_eq!(tail, None);
    }

    #[test]
    fn invalid_from_data() {
        let data = [0, 1];
        let (function, tail) = get_function(&data);
        assert_eq!(
            function,
            Some(ModbusFunction::new_public(PublicModbusFunction::Invalid))
        );
        assert_eq!(tail, Some(&[1u8] as &[u8]));
    }

    #[test]
    fn read_coils_from_data() {
        let data = [1, 1];
        let (function, tail) = get_function(&data);
        assert_eq!(
            function,
            Some(ModbusFunction::new_public(PublicModbusFunction::ReadCoils))
        );
        assert_eq!(tail, Some(&[1u8] as &[u8]));
    }
}
