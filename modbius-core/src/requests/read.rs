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
            pub fn from_data(data: &[u8]) -> Option<(Self, &[u8])> {
                let (addr, data) = crate::util::read_u16(data)?;
                let (quantity, data) = crate::util::read_u16(data)?;

                Some((Self { addr, quantity }, data))
            }

            /// Parse this request from the given modbus data without bounds checks
            /// 
            /// # Safety
            /// This function causes undefined behavior if the len of data is smaller than 4
            pub unsafe fn from_data_unchecked(data: &[u8]) -> (Self, &[u8]) {
                let (addr, data) = crate::util::read_u16_unchecked(data);
                let (quantity, data) = crate::util::read_u16_unchecked(data);

                (Self { addr, quantity }, data)
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
            pub fn write_to_slice(self, data: &mut [u8]) {
                //TODO return Result
                let addr = self.addr.to_be_bytes();
                if let Some(addr_bytes) = data.get_mut(0..2) {
                    addr_bytes[0] = addr[0];
                    addr_bytes[1] = addr[1];
                }

                let quantity = self.quantity.to_be_bytes();
                if let Some(quantity_bytes) = data.get_mut(2..4) {
                    quantity_bytes[0] = quantity[0];
                    quantity_bytes[1] = quantity[1];
                }
            }

            /// Write this request to the slice as modbus data without bounds checking. 
            /// 
            /// # Safety
            /// This function invokes undefined behavior if the len of data is less than 4 
            pub unsafe fn write_to_slice_unchecked(self, data: &mut [u8]) {
                let addr = self.addr.to_be_bytes();
                *data.get_unchecked_mut(0) = addr[0];
                *data.get_unchecked_mut(1) = addr[1];

                let quantity = self.quantity.to_be_bytes();
                *data.get_unchecked_mut(2) = quantity[0];
                *data.get_unchecked_mut(3) = quantity[1];
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
                assert!(res.is_none())
            }

            #[test]
            fn from_data_fail1() {
                let data = [];
                let res = $name::from_data(&data);
                assert!(res.is_none())
            }

            #[test]
            fn from_data_fail2() {
                let data = [255, 255, 0];
                let res = $name::from_data(&data);
                assert!(res.is_none())
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
            fn from_data_from_data_unecked() {
                let data = [255, 255, 255, 255];
                let (req_unchecked, tail_unchecked) = unsafe { $name::from_data_unchecked(&data) };
                let (req, tail) = unsafe { $name::from_data_unchecked(&data) };

                assert_eq!(req.addr, u16::MAX);
                assert_eq!(req.quantity, u16::MAX);

                assert_eq!(req_unchecked.addr, u16::MAX);
                assert_eq!(req_unchecked.quantity, u16::MAX);

                assert_eq!(req.addr, req_unchecked.addr);
                assert_eq!(req.quantity, req_unchecked.quantity);
                
                assert!(tail.is_empty());
                assert_eq!(tail, tail_unchecked);
            }
        }
    };
}

read_req!(ReadCoils, PublicModbusFunction::ReadCoils, "Coils", coils);
read_req!(ReadDiscreteInputs, PublicModbusFunction::ReadDiscreteInputs, "DiscreteInputs", discrete_inputs);
read_req!(ReadHoldingRegisters, PublicModbusFunction::ReadHoldingRegisters, "HoldingRegisters", holding_registers);
read_req!(ReadInputRegisters, PublicModbusFunction::ReadInputRegisters, "InputRegisters", input_registers);
