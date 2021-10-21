use crate::functions::PublicModbusFunction;

macro_rules! read_req {
    ($name:ident, $fcode:expr, $entity:literal, $test:ident) => {
        #[doc=concat!("The request structure to read ", $entity)]
        #[doc=concat!("\n")]
        /// This structure may be used to parse and build modbus data to read quantity entities starting from addr.
        #[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            pub addr: u16,
            pub quantity: u16,
        }

        impl $name {
            /// The Modbus function this read requests corresponds to.
            pub const MODBUS_FUNCTION_CODE: PublicModbusFunction = $fcode;

            #[doc=concat!("Create a new request to read quantity ", $entity)]
            pub const fn new(addr: u16, quantity: u16) -> Self {
                Self { addr, quantity }
            }

            /// Parse this request from the given modbus data
            ///
            /// The data should only consist out of the address and quantity as the slave id function
            /// will be already read through other means.
            pub fn from_data(
                data: &[u8],
            ) -> Result<(Self, &[u8]), $crate::ModbusSerializationError> {
                if data.len() < 4 {
                    Err($crate::ModbusSerializationError::UnexpectedEOF {
                        expected: 4,
                        got: data.len(),
                    })
                } else {
                    Ok(unsafe { Self::from_data_unchecked(data) })
                }
            }

            /// Parse this request from the given modbus data without bounds checks.
            ///
            /// The data should only consist out of the address and quantity as the slave id function
            /// will be already read through other means.
            ///
            /// # Safety
            /// This function causes undefined behavior if the len of data is smaller than 4
            pub unsafe fn from_data_unchecked(data: &[u8]) -> (Self, &[u8]) {
                let (addr, quantity, data) = $crate::util::read_addr_quantity_unchecked(data);
                (Self::new(addr, quantity), data)
            }

            /// Create modbus data of the correct size from this request
            ///
            /// The format of the array will be [addrhi, addrlo, quantityhi, quantitylo] in big endian
            pub fn into_data(self) -> [u8; 4] {
                let addr = self.addr.to_be_bytes();
                let quantity = self.quantity.to_be_bytes();
                [addr[0], addr[1], quantity[0], quantity[1]]
            }

            /// Write this request to the slice as modbus data
            pub fn write_to_slice(
                self,
                out: &mut [u8],
            ) -> Result<(), $crate::ModbusSerializationError> {
                if out.len() < 5 {
                    return Err($crate::ModbusSerializationError::InsufficientBuffer {
                        expected: 5,
                        got: out.len(),
                    });
                }

                unsafe { self.write_to_slice_unchecked(out) };
                Ok(())
            }

            /// Write this request to the slice as modbus data without bounds checking.
            ///
            /// # Safety
            /// This function invokes undefined behavior if the len of data is less than 5
            pub unsafe fn write_to_slice_unchecked(self, out: &mut [u8]) {
                *out.get_unchecked_mut(0) = Self::MODBUS_FUNCTION_CODE as u8;
                $crate::util::write_addr_quantity_unchecked(
                    self.addr,
                    self.quantity,
                    out.get_unchecked_mut(1..),
                )
            }
        }

        #[cfg(test)]
        mod $test {
            use super::*;

            #[test]
            fn create() {
                let req = $name {
                    addr: 10,
                    quantity: 20,
                };

                let req_new = $name::new(10, 20);

                assert_eq!(req, req_new);
                assert_eq!(req.addr, req_new.addr);
                assert_eq!(req.quantity, req_new.quantity);
            }

            #[test]
            fn from_data0() {
                let data = [0, 10, 0, 20];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, 10);
                assert_eq!(req.quantity, 20);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data1() {
                let data = [1, 10, 2, 20];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, 266);
                assert_eq!(req.quantity, 532);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data2() {
                let data = [0, 255, 0, 255];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, 255);
                assert_eq!(req.quantity, 255);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data3() {
                let data = [255, 0, 255, 0];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, 65280);
                assert_eq!(req.quantity, 65280);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data4() {
                let data = [255, 255, 255, 255];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_unchecked0() {
                let data = [0, 10, 0, 20];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, 10);
                assert_eq!(req.quantity, 20);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_unchecked1() {
                let data = [1, 10, 2, 20];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, 266);
                assert_eq!(req.quantity, 532);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_unchecked2() {
                let data = [0, 255, 0, 255];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, 255);
                assert_eq!(req.quantity, 255);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_unchecked3() {
                let data = [255, 0, 255, 0];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, 65280);
                assert_eq!(req.quantity, 65280);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_unchecked4() {
                let data = [255, 255, 255, 255];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert!(tail.is_empty());
            }

            #[test]
            fn from_data_fail0() {
                let data = [255, 255];
                let res = $name::from_data(&data);
                assert_eq!(
                    res.unwrap_err(),
                    $crate::ModbusSerializationError::UnexpectedEOF {
                        expected: 4,
                        got: 2
                    }
                );
            }

            #[test]
            fn from_data_fail1() {
                let data = [];
                let res = $name::from_data(&data);
                assert_eq!(
                    res.unwrap_err(),
                    $crate::ModbusSerializationError::UnexpectedEOF {
                        expected: 4,
                        got: 0
                    }
                );
            }

            #[test]
            fn from_data_fail2() {
                let data = [255, 255, 0];
                let res = $name::from_data(&data);
                assert_eq!(
                    res.unwrap_err(),
                    $crate::ModbusSerializationError::UnexpectedEOF {
                        expected: 4,
                        got: 3
                    }
                );
            }

            #[test]
            fn from_data_tail0() {
                let data = [255, 255, 255, 255, 1, 2, 3, 4];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert_eq!(tail, &[1, 2, 3, 4]);
            }

            #[test]
            fn from_data_tail1() {
                let data = [255, 255, 255, 255, 1];
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert_eq!(tail, &[1]);
            }

            #[test]
            fn from_data_unchecked_tail0() {
                let data = [255, 255, 255, 255, 1, 2, 3, 4];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert_eq!(tail, &[1, 2, 3, 4]);
            }

            #[test]
            fn from_data_unchecked_tail1() {
                let data = [255, 255, 255, 255, 1];
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert_eq!(tail, &[1]);
            }

            #[test]
            fn from_data_eq_from_data_unecked() {
                let data = [255, 255, 255, 255];
                let (req_unchecked, tail_unchecked) = unsafe { $name::from_data_unchecked(&data) };
                let (req, tail) = $name::from_data(&data).unwrap();

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);

                assert_eq!(req_unchecked.addr, u16::MAX);
                assert_eq!(req_unchecked.quantity, u16::MAX);

                assert_eq!(req.addr, req_unchecked.addr);
                assert_eq!(req.quantity, req_unchecked.quantity);

                assert!(tail.is_empty());
                assert_eq!(tail, tail_unchecked);
            }

            #[test]
            fn into_data0() {
                let data = [255, 255, 255, 255];
                let (req, _tail) = $name::from_data(&data).unwrap();
                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);
                assert_eq!(data, req.into_data());
            }

            #[test]
            fn into_data1() {
                let data = [0, 255, 0, 255];
                let (req, _tail) = $name::from_data(&data).unwrap();
                assert_eq!(req.addr, 255);
                assert_eq!(req.quantity, 255);
                assert_eq!(data, req.into_data());
            }

            #[test]
            fn into_data2() {
                let req = $name::new(10, 20);
                assert_eq!(req.addr, 10);
                assert_eq!(req.quantity, 20);
                assert_eq!(req.into_data(), [0, 10, 0, 20]);
            }

            #[test]
            fn write_to_slice0() {
                let req = $name::new(10, 20);
                let mut slice = [0; 5];

                req.write_to_slice(&mut slice).unwrap();
                assert_eq!(slice, [$name::MODBUS_FUNCTION_CODE as u8, 0, 10, 0, 20]);
            }

            #[test]
            fn write_to_slice1() {
                let req = $name::new(256, 255);
                let mut slice = [0, 1, 2, 3, 4];

                req.write_to_slice(&mut slice).unwrap();
                assert_eq!(slice, [$name::MODBUS_FUNCTION_CODE as u8, 1, 0, 0, 255]);
            }

            #[test]
            fn write_to_slice2() {
                let req = $name::new(u16::MAX, u16::MAX);
                let mut slice = [1, 1, 1, 1, 1, 0, 0, 0, 0];

                req.write_to_slice(&mut slice).unwrap();
                assert_eq!(
                    slice,
                    [$name::MODBUS_FUNCTION_CODE as u8, 255, 255, 255, 255, 0, 0, 0, 0]
                );
            }

            #[test]
            fn write_to_slice_fail0() {
                let req = $name::new(u16::MAX, u16::MAX);
                let mut slice = [1, 1, 1, 1];

                let err = req.write_to_slice(&mut slice).unwrap_err();

                assert_eq!(err, $crate::ModbusSerializationError::InsufficientBuffer { got: 4, expected: 5 });
            }

            #[test]
            fn write_to_slice_fail1() {
                let req = $name::new(u16::MAX, u16::MAX);
                let mut slice = [];

                let err = req.write_to_slice(&mut slice).unwrap_err();

                assert_eq!(err, $crate::ModbusSerializationError::InsufficientBuffer { got: 0, expected: 5 });
            }

            #[test]
            fn write_to_slice_eq_write_to_slice_unchecked() {
                let req = $name::new(u16::MAX, u16::MAX);
                let mut slice = [1,2,3,4,5];
                let mut slice_unchecked = [1,2,3,4,5];

                req.write_to_slice(&mut slice).unwrap();
                unsafe { req.write_to_slice_unchecked(&mut slice_unchecked) };

                assert_eq!(slice, slice_unchecked);
                assert_eq!(slice, [$name::MODBUS_FUNCTION_CODE as u8, 255, 255, 255, 255]);
                assert_eq!(slice, [$name::MODBUS_FUNCTION_CODE as u8, 255, 255, 255, 255]);
            }
        }
    };
}

read_req!(ReadCoils, PublicModbusFunction::ReadCoils, "Coils", coils);
read_req!(
    ReadDiscreteInputs,
    PublicModbusFunction::ReadDiscreteInputs,
    "DiscreteInputs",
    discrete_inputs
);
read_req!(
    ReadHoldingRegisters,
    PublicModbusFunction::ReadHoldingRegisters,
    "HoldingRegisters",
    holding_registers
);
read_req!(
    ReadInputRegisters,
    PublicModbusFunction::ReadInputRegisters,
    "InputRegisters",
    input_registers
);
