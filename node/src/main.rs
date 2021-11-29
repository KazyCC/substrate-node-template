//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;
mod karaoke;

fn main() -> sc_cli::Result<()> {
	command::run()
}
