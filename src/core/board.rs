//! Go is a board game.
//! While it is usually played on a rectangular board
//! with a very specific structure,
//! it can theoretically be played on a wide variety of (undirected) graphs.
//! This module provides a suitable interface
//! which allows the rules to be evaluated
//! without knowledge of the exact board layout.

use self::indexer::Indexer;
use core::util::bool_mat::BoolMat;
use core::util::indexer;

// TODO: We might be able to get rid of a few of those lifetimes.

/// This trait captures the information needed to play Go on a board.
pub trait Board: Eq {
	/// The interface for addressing the intersection point.
	/// In classical Go, this will in some way implement a square.
	type I: Indexer;

	/// The adjacency matrix of the underlying graph.
	/// This should be symmetric.
	/// (Asymmetric versions of Go are thinkable
	/// but not currently supported by Gorrosion.)
	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I>;

	/// This will soon go away, presumably.
	/// However, the boards will need to provide some way
	/// to know where to place a fixed handicap.
	/// This will come in one of the refactors of the future.
	fn is_hoshi(&self, i: <Self::I as Indexer>::Index) -> bool;
}

/// The most generic board: A graph.
/// All other boards could theoretically be implemented
/// using this as a basis.
#[derive(PartialEq, Eq, Debug)]
pub struct Graph<I: Indexer> {
	adj: BoolMat<I, I>,
}

impl<I> Board for Graph<I>
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

/// Rectangular boards with the classical line pattern.
/// ```none
/// ┼─┼─┼─┼
/// ┼─┼─┼─┼
/// ┼─┼─┼─┼
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct Rect {
	graph: Graph<indexer::Rect>,
}

impl Rect {
	/// Create a new rectangular board with the given measurements.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::board::Rect;
	/// # use gorrosion::core::board::Board;
	/// let rect = Rect::new(6, 4);
	/// // Get the adjacency matrix.
	/// let adj = rect.adjacencies();
	/// // Every intersection point can be reached from every intersection point
	/// // in at most (6-1) + (4-1) = 8 steps.
	/// let two_steps = adj * adj;
	/// let four_steps = &two_steps * &two_steps;
	/// // So, this should be the all-true matrix
	/// let eight_steps = &four_steps * &four_steps;
	/// assert_eq!(eight_steps[((0, 0), (5, 3))], true);
	/// ```
	pub fn new(height: usize, width: usize) -> Rect {
		let indexer = indexer::Rect::new(height, width);
		let mut adj = BoolMat::id_matrix(indexer);
		{
			let mut sym_set = |a, b| {
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
		}
		let graph = Graph { adj };
		Rect { graph }
	}
}

impl Board for Rect {
	type I = indexer::Rect;

	fn adjacencies(&self) -> &BoolMat<Self::I, Self::I> {
		self.graph.adjacencies()
	}

	fn is_hoshi(&self, _i: <Self::I as Indexer>::Index) -> bool {
		false
	}
}

#[derive(PartialEq, Eq, Debug)]
pub struct Square {
	rect: Rect,
}

impl Square {
	pub fn new(length: usize) -> Square {
		let rect = Rect::new(length, length);
		Square { rect }
	}
}

impl Board for Square {
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
//       than whether a given intersection point is one.

// TODO: Throw out as soon as coverage tools can handle doctests.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn doctests() {
		let rect = Rect::new(6, 4);
		// Get the adjacency matrix.
		let adj = rect.adjacencies();
		// Every intersection point can be reached from every intersection point
		// in at most (6-1) + (4-1) = 8 steps.
		let two_steps = adj * adj;
		let four_steps = &two_steps * &two_steps;
		// So, this should be the all-true matrix
		let eight_steps = &four_steps * &four_steps;
		assert_eq!(eight_steps[((0, 0), (5, 3))], true);
	}
}
