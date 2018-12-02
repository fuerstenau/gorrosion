use self::indexer::Indexer;
use core::util::bool_mat::BoolMat;
use core::util::indexer;

// TODO: We might be able to get rid of a few of those lifetimes.

pub trait Board: Eq {
	type I: Indexer;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I>;

	fn is_hoshi(&self, i: <Self::I as Indexer>::Index) -> bool;
}

#[derive(PartialEq, Eq, Debug)]
pub struct Graph<'a, 'i: 'a, I: 'i + Indexer> {
	adj: BoolMat<'a, 'i, 'i, I, I>,
}

impl<'a, 'i, I> Board for Graph<'a, 'i, I>
where
	I: Indexer,
{
	type I = I;

	fn adjacencies(&self) -> &BoolMat<I, I> {
		&self.adj
	}

	fn is_hoshi(&self, _i: I::Index) -> bool {
		false
	}
}

#[derive(PartialEq, Eq, Debug)]
pub struct Rect<'a, 'i: 'a> {
	indexer: indexer::Rect,
	graph: Graph<'a, 'i, indexer::Rect>,
}

impl<'a, 'i> Rect<'a, 'i> {
	pub fn new(height: usize, width: usize) -> Rect<'a, 'i> {
		let indexer = indexer::Rect::new(height, width);
		let mut adj = BoolMat::id_matrix(&indexer);
		let sym_set = |a, b| {
			adj[(a, b)] = true;
			adj[(b, a)] = true;
		};
		for j in 0..height {
			for k in 1..width {
				let a = (j, k - 1);
				let b = (j, k);
				sym_set(a, b);
			}
		}
		for j in 1..height {
			for k in 0..width {
				let a = (j - 1, k);
				let b = (j, k);
				sym_set(a, b);
			}
		}
		let graph = Graph { adj };
		Rect { indexer, graph }
	}
}

impl<'a, 'i> Board for Rect<'a, 'i> {
	type I = indexer::Rect;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I> {
		self.graph.adjacencies()
	}

	fn is_hoshi(&self, _i: <Self::I as Indexer>::Index) -> bool {
		false
	}
}

#[derive(PartialEq, Eq, Debug)]
pub struct Square<'a, 'i: 'a> {
	rect: Rect<'a, 'i>,
}

impl<'a, 'i> Square<'a, 'i> {
	pub fn new(length: usize) -> Square<'a, 'i> {
		let rect = Rect::new(length, length);
		Square { rect }
	}
}

impl<'a, 'i> Board for Square<'a, 'i> {
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
