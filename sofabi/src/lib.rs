//! Sophon ABI encoding decoding library.

#![warn(missing_docs)]

extern crate rustc_hex as hex;
extern crate serde;
extern crate serde_json;
extern crate tiny_keccak;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

extern crate sophon_types;

pub mod param_type;
pub mod token;
mod constructor;
mod contract;
mod decoder;
mod encoder;
mod errors;
mod event;
mod event_param;
mod filter;
mod function;
mod log;
mod operation;
mod param;
mod signature;
mod util;

pub use param_type::ParamType;
pub use constructor::Constructor;
pub use contract::{Contract, Functions, Events};
pub use token::Token;
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use encoder::encode;
pub use decoder::decode;
pub use filter::{Topic, TopicFilter, RawTopicFilter};
pub use function::Function;
pub use param::Param;
pub use log::{Log, RawLog, LogParam, ParseLog, LogFilter};
pub use event::Event;
pub use event_param::EventParam;

/// ABI address.
pub type Address = sophon_types::Address;

/// ABI fixed bytes.
pub type FixedBytes = Vec<u8>;

/// ABI bytes.
pub type Bytes = Vec<u8>;

/// ABI signed integer.
pub type Int = sophon_types::U256;

/// ABI unsigned integer.
pub type Uint = sophon_types::U256;

/// Commonly used FixedBytes of size 32
pub type Hash = sophon_types::H256;

/// Contract functions generated by sofabi-derive
pub trait FunctionOutputDecoder {
	/// Output types of the contract function
	type Output;

	/// Decodes the given bytes output for the contract function
	fn decode(&self, &[u8]) -> Result<Self::Output>;
}
