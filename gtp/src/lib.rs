#![feature(try_from)]

#[macro_use]
extern crate nom;

type Byte = u8;

mod types;

use messages::MessagePart;
use parse::Input;

mod parse {
	use super::Byte;

	const DISCARD: [Byte; 31] = [
		0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19,
		20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127,
	]; // “Control characters”: 0 – 8, 11 – 31, 127
	const SPACE: [Byte; 2] = [9, 32]; // " \t"
	const NEWLINE: Byte = 10; // "\n"
	const COMMENT: Byte = 35; // "#"

	pub struct Input<'a> {
		data: &'a [Byte],
	}
}

mod messages {
	pub struct MessagePart {
		data: Vec<Byte>,
	}

	use super::types::*;
	use super::Byte;

	pub struct CommandMessage {
		id: Option<Int>,
		command_name: String,
		arguments: Collection,
	}

	pub struct Line {
		data: Vec<Byte>,
	}

	impl SingleLine for Line {}

	// TODO: Support for standard error messages?

	pub enum Content {
		Collection(Collection),
		Response(MultilineList<Line>),
		ErrorMessage(MultilineList<List<String>>),
	}

	pub struct ResponseMessage {
		id: Option<Int>,
		content: Content,
	}
}

mod command {
	use super::types::*;

	// TODO: Macros

	struct Command {
		name: String,
	}
}
