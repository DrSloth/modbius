#![no_std]
//! A low level modbus library working with raw data.
//!
//! This library provides allocation free functions to work with raw modbus streams.
//! Directly using this library is discouraged as it is mainly meant to be abstracted over. <br/>
//! The main idea is that a slice of bytes containing raw modbus data can be passed to various functions
//! to build and handle requests/responses. If a functions returns something it will return a tupel where the last
//! field is the slice you should pass to the next function, if it doesn't only the slice is returned. If no slice
//! is returned there shouldn't be more data to consume

/// Mapping of modbus function codes and routines to read them
pub mod function;
/// Handling and building of requests
pub mod request;
/// Modbus related utility functions
pub mod util;
