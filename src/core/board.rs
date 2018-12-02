use self::indexer::Indexer;
use core::util::bool_mat::BoolMat;
use core::util::indexer;

pub trait Board: Clone + Eq {
	type I: Indexer;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I>;

	fn is_hoshi(&self, i: <Self::I as Indexer>::Index) -> bool;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Graph<'a> {
	adj: BoolMat<'a, usize, usize>,
}

impl<'a> Board for Graph<'a> {
	type I = usize;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I> {
		&self.adj
	}

	fn is_hoshi(&self, _i: <Self::I as Indexer>::Index) -> bool {
		false
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Rect<'a> {
	indexer: &'a indexer::Rect,
	graph: Graph<'a>,
}

impl<'a> Rect<'a> {
	pub fn new(height: usize, width: usize) -> Rect<'a> {
		let indexer = indexer::Rect::new(height, width);
		let mut adj = BoolMat::identity(indexer);
		let sym_set = |a, b| {
			adj[(a, b)] = true;
			adj[(b, a)] = true;
		};
		for j in 0..height {
			for k in 1..width {
				let a = indexer.to_num(j, k - 1);
				let b = indexer.to_num(j, k);
				sym_set(a, b);
			}
		}
		for j in 1..height {
			for k in 0..width {
				let a = indexer.to_num(j - 1, k);
				let b = indexer.to_num(j, k);
				sym_set(a, b);
			}
		}
		let graph = Graph { adj };
		Rect { indexer, graph }
	}
}

impl<'a> Board for Rect<'a> {
	type I = indexer::Rect;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I> {
		self.graph.adjacencies()
	}

	fn is_hoshi(&self, _i: <Self::I as Indexer>::Index) -> bool {
		false
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Square<'a> {
	rect: Rect<'a>,
}

impl<'a> Square<'a> {
	pub fn new(length: usize) -> Square<'a> {
		let rect = Rect::new(length, length);
		Square { rect }
	}
}

impl<'a> Board for Square<'a> {
	type I = indexer::Rect;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I> {
		&self.rect.adjacencies()
	}

	fn is_hoshi(&self, _i: <Self::I as Indexer>::Index) -> bool {
		false
	}
}

// TODO: Implement the three standard boards with their hoshi
// TODO: Boards need to provide more information about their hoshi
//       than whether a given moku is one.
