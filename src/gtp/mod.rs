type byte = u8;

// const discard: [byte; 31] = [0, …, 8, 11, …, 31, 127];
const space: [byte; 2] = [9, 32]; // " \t"
const newline: byte = 10; // "\n"
const comment: byte = 35; // "#"

mod types {
	use super::byte;

	pub trait GtpType {}
	pub trait SimpleEntity {}
	pub trait SingleLine {}

	impl GtpType for SingleLine {}
	impl SingleLine for SimpleEntity {}

	pub struct Int {
		data: u32,
	}

	impl SimpleEntity for Int {}

	pub struct Float {
		data: f32,
	}

	impl SimpleEntity for Float {}

	pub struct String {
		data: Vec<byte>,
	}

	impl SimpleEntity for String {}

	pub enum Vertex {
		Pass,
		Coord(char, u8),
	}

	impl SimpleEntity for Vertex {}

	pub enum Color {
		Black,
		White,
	}

	impl SimpleEntity for Color {}

	pub struct Move {
		color: Color,
		vertex: Vertex,
	}

	impl SimpleEntity for Move {}

	pub enum Boolean {
		False,
		True,
	}

	impl SimpleEntity for Boolean {}

	enum CollectionItem {
		Int(Int),
		Float(Float),
		String(String),
		Vertex(Vertex),
		Color(Color),
		Move(Move),
		Boolean(Boolean),
	}

	pub enum Collection {
		None,
		Collection(Box<(CollectionItem, Collection)>),
		IntList(List<Int>),
		FloatList(List<Float>),
		StringList(List<String>),
		VertexList(List<Vertex>),
		ColorList(List<Color>),
		MoveList(List<Move>),
		BooleanList(List<Boolean>),
	}

	impl SingleLine for Collection {}

	pub struct List<T: SimpleEntity> {
		data: Vec<T>,
	}

	impl<T: SimpleEntity> SingleLine for List<T> {}

	// The so-called specification is unclear
	// on the matter of alternatives of compound types.
	// For now, we'll do the simpler thing.
	pub enum Alternatives<T: SimpleEntity, S: SimpleEntity> {
		Left(T),
		Right(S),
	}

	impl<T: SimpleEntity, S: SimpleEntity> SingleLine for Alternatives<T, S> {}

	pub struct MultilineList<T: SingleLine> {
		data: Vec<T>,
	}

	impl<T: SingleLine> GtpType for MultilineList<T> {}
}

mod command {
	use super::types;

	struct Command {
		id: Option<types::Int>,
		command_name: types::String,
		arguments: types::Collection,
	}
}
