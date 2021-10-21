#![no_std]
//! A low level modbus library working with raw data.
//!
//! This library provides allocation free functions to work with raw modbus streams.
//! Directly using this library is discouraged as it is mainly meant to be abstracted over. <br/>
//! The main idea is that a slice of bytes containing raw modbus data can be passed to various functions
//! to build and handle requests/responses.
//! 
//! Functions MUST always return the "tail" data, the data that was passed in but not parsed. 
//! Functions MUST NOT to perform any assumptions based on the length of passed in modbus data.
//! Invalid or unneeded data MUST be returned as tail.

pub mod functions;
pub mod bitstate;
pub mod slaveid;
pub mod requests;
pub mod util;
pub mod registerslice;
mod error;

pub use functions::{ModbusFunction, PublicModbusFunction};
pub use bitstate::BitState; 
pub use slaveid::SlaveId;
pub use error::*;
