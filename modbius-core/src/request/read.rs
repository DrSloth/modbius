use crate::function::PublicModbusFunction;

macro_rules! read_request {
    ($name:ident, $fcode:expr) => {
        #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            /// Starting address of the read request
            pub addr: u16,
            /// Quantity of items to read
            pub quantity: u16,
        }

        impl $name {
            pub const FUNCTION_CODE: PublicModbusFunction = $fcode;

            pub const fn new(addr: u16, quantity: u16) -> Self {
                Self { addr, quantity }
            }

            /// Parses this request from modbus data, the parsed data should consist of
            /// [address hi, address lo, quantity hi, quantity lo].
            ///
            /// This function checks the length of the data and if insufficient data (data.len() < 4) is given
            /// none is returned. The function code is missing because it should be parsed by [function::get_function](crate::function::get_function)
            pub fn from_modbus_data(data: &[u8]) -> (Option<Self>, Option<&[u8]>) {
                let (me, _) = unsafe { Self::from_modbus_data_unchecked(data) };
                if data.len() >= 4 {
                    if me.quantity > 0 {
                        return (Some(me), data.get(4..));
                    }
                }

                (None, Some(data))
            }

            /// Parses this request from modbus data, the parsed data must consist of
            /// [address hi, address lo, quantity hi, quantity lo].
            ///
            /// The function code is missing because it was already parsed by [function::get_function](crate::function::get_function)
            pub unsafe fn from_modbus_data_unchecked(data: &[u8]) -> (Self, &[u8]) {
                let (aq, data) = crate::util::AddrQuantity::from_modbus_data_unchecked(data);

                (
                    Self {
                        addr: aq.addr,
                        quantity: aq.quantity,
                    },
                    data,
                )
            }

            /// Returns this requests as a complete modbus data item request.
            ///
            /// The returned data consists of [function code, address hi, address lo, quantity hi, quantity lo]
            pub fn as_modbus_data(self) -> [u8; 5] {
                Self::new_as_modbus_data(self.addr, self.quantity)
            }

            /// The data consists of [function code, addr hi, addr lo, quantity hi, quantity lo]
            pub fn new_as_modbus_data(addr: u16, quantity: u16) -> [u8; 5] {
                let addr_bytes = addr.to_be_bytes();
                let quantity_bytes = quantity.to_be_bytes();
                [
                    $fcode as u8,
                    addr_bytes[0],
                    addr_bytes[1],
                    quantity_bytes[0],
                    quantity_bytes[1],
                ]
            }
        }
    };
}

read_request!(ReadCoilsRequest, PublicModbusFunction::ReadCoils);
read_request!(
    ReadDiscreteInputsRequest,
    PublicModbusFunction::ReadDiscreteInputs
);
read_request!(
    ReadHoldingRegistersRequest,
    PublicModbusFunction::ReadHoldingRegisters
);
read_request!(
    ReadInputRegistersRequest,
    PublicModbusFunction::ReadInputRegisters
);

macro_rules! read_request_test {
    ($name:ident, $testname:ident) => {
        macro_rules! test_suite {
            ($suite_name:ident, $data:expr, $addr:expr, $quantity:expr) => {
                #[cfg(test)]
                mod $suite_name {
                    use super::super::*;
                    fn from_modbus_data(data: &[u8]) -> $name {
                        let (fcode, data) = crate::function::get_function(data);
                        assert_eq!(fcode.unwrap(), $name::FUNCTION_CODE.into());
                        let (me, _) = $name::from_modbus_data(data.unwrap());
                        me.unwrap()
                    }

                    #[test]
                    fn test_from_modbus_data() {
                        let data = $data;
                        let me = from_modbus_data(&data);
                        assert_eq!(me.quantity, $quantity);
                        assert_eq!(me.addr, $addr);
                    }

                    #[test]
                    fn same_eq_as() {
                        let data = $data;
                        let me = from_modbus_data(&data);
                        assert_eq!(me.as_modbus_data(), data);
                        assert_eq!(data, $name::new_as_modbus_data(me.addr, me.quantity))
                    }

                    #[test]
                    fn as_modbus_data() {
                        let data = $data;
                        let me_data = $name::new_as_modbus_data($addr, $quantity);
                        assert_eq!(me_data, data)
                    }
                }
            };
        }

        mod $testname {
            test_suite!(t1, [$name::FUNCTION_CODE as u8, 0, 33, 0, 33], 33, 33);
            test_suite!(t2, [$name::FUNCTION_CODE as u8, 0, 1, 0, 5], 1, 5);
            test_suite!(t3, [$name::FUNCTION_CODE as u8, 1, 0, 2, 0], 256, 512);
        }
    };
}

read_request_test!(ReadCoilsRequest, read_coils);
read_request_test!(ReadHoldingRegistersRequest, read_holdings);
read_request_test!(ReadDiscreteInputsRequest, read_discretes);
read_request_test!(ReadInputRegistersRequest, read_input_registers);
