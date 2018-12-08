use super::Byte;
use super::Input;
use super::MessagePart;

// TODO: I'm unhappy with quite a few of the names.

use self::common::Common;
//use self::simple_entity::SimpleEntity;

mod common {
	use super::Input;
	use super::MessagePart;
	use nom::IResult;

	pub trait Common: Into<MessagePart> {
		// TODO: Which kind of errors do we need to throw?
		fn parse(i: Input) -> IResult<Input, Self>;
	}
}

mod simple_entity {
	use super::Common;
	use std::convert::TryFrom;

	pub enum Type {
		Int,
		Float,
		String,
		Vertex,
		Color,
		Move,
		Boolean,
	}

	pub enum Value {
		Int(super::Int),
		Float(super::Float),
		String(super::String),
		Vertex(super::Vertex),
		Color(super::Color),
		Move(super::Move),
		Boolean(super::Boolean),
	}

	pub trait SimpleEntity: Common + TryFrom<Value> + Into<Value> {
		fn concrete_type(&self) -> Type;
	}
}

pub trait GtpType {}
pub trait SingleLine {}
pub trait SimpleEntity {}

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
	data: Vec<Byte>,
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

enum SimpleEntityType {
	Int,
	Float,
	String,
	Vertex,
	Color,
	Move,
	Boolean,
}

enum SimpleEntityValue {
	Int(Int),
	Float(Float),
	String(String),
	Vertex(Vertex),
	Color(Color),
	Move(Move),
	Boolean(Boolean),
}

enum ListType {
	IntList,
	FloatList,
	StringList,
	VertexList,
	ColorList,
	MoveList,
	BooleanList,
}

enum ListValue {
	IntList(List<Int>),
	FloatList(List<Float>),
	StringList(List<String>),
	VertexList(List<Vertex>),
	ColorList(List<Color>),
	MoveList(List<Move>),
	BooleanList(List<Boolean>),
}

enum SingleLineType {
	SimpleEntity(SimpleEntityType),
	Collection,
	List(ListType),
	Alternatives(SimpleEntityType, SimpleEntityType),
}

enum SingleLineValue {
	SimpleEntity(SimpleEntityValue),
	Collection(Collection),
	List(ListValue),
}

enum GtpTypeType {
	SingleLine(SingleLineType),
	MultilineList(SingleLineType),
}
