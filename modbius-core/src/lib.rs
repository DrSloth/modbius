#![no_std]
//! A low level modbus library working with raw data.
//!
//! This library provides allocation free functions to work with raw modbus streams.
//! Directly using this library is discouraged as it is mainly meant to be abstracted over. <br/>
//! The main idea is that a slice of bytes containing raw modbus data can be passed to various functions
//! to build and handle requests/responses. If a function returns something it will return a tupel where the last
//! field is the slice you should pass to the next function.

pub mod functions;
pub use functions::{ModbusFunction, PublicModbusFunction};

pub mod slaveid;
pub use slaveid::SlaveId;

pub mod requests;

pub mod util;
