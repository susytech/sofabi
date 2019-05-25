#![allow(unknown_lints)]

use std::io;
use {sofabi, docopt, hex};

error_chain! {
	links {
		Sofabi(sofabi::Error, sofabi::ErrorKind);
	}

	foreign_links {
		Io(io::Error);
		Docopt(docopt::Error);
		Hex(hex::FromHexError);
	}
}
