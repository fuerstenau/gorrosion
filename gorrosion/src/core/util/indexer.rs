//! Abstract (bijective?) conversions between integer ranges and other types
//! to allow for a clean abstraction of indexing vectors
//! using these other types.

use std::fmt::Debug;

/// The heart of the operation.
/// Provides the methods for converting external indices
/// to internally used integers and vice versa.
/// The internal indices are assumed to be
/// a contiguous chunk of non-negative integers starting at zero.
// TODO: I do not want this to be Clone
//       and rather would have it accessed through references.
//       However, this leads to lifetime problems
//       and at the very least would require the use of rc::Rc.
pub trait Indexer: Eq + Debug + Clone {
	type Index: Copy;

	/// Convert an external index to an internal index.
	fn to_num(&self, index: Self::Index) -> usize;

	/// Convert an integral index back into an external index.
	fn to_index(&self, n: usize) -> Self::Index;

	/// Checks whether an external index is valid
	/// to allow an indexer
	/// to speak only about a subset of its official index set,
	/// e.g. a $19\times19$ square instead of the entire (usize, usize).
	fn is_valid(&self, index: Self::Index) -> bool;

	/// A strict upper bound on the internal indices
	/// this indexer will output.
	/// The upper bound is assumed to be sharp,
	/// i.e. exactly the numbers 0..range can be spat out by to_num.
	fn range(&self) -> usize;

	/// Determines whether an integer is within the range
	/// of the internal indices.
	fn in_range(&self, n: usize) -> bool {
		n < self.range()
	}
}

/// Index the singleton in the obvious (and pretty much only) way.
impl Indexer for () {
	type Index = ();

	fn to_num(&self, _i: Self::Index) -> usize {
		0
	}

	fn to_index(&self, n: usize) -> Self::Index {
		assert!(self.in_range(n));
	}

	fn range(&self) -> usize {
		1
	}

	fn is_valid(&self, _i: Self::Index) -> bool {
		true
	}
}

/// With the usual identification $n+1 = \{0, \dots, n\}$,
/// we know how an integer should provide indices.
impl Indexer for usize {
	type Index = usize;

	fn to_num(&self, i: Self::Index) -> usize {
		assert!(self.is_valid(i));
		i
	}

	fn to_index(&self, n: usize) -> Self::Index {
		assert!(self.in_range(n));
		n
	}

	fn range(&self) -> usize {
		*self
	}

	fn is_valid(&self, i: Self::Index) -> bool {
		self.in_range(i)
	}
}

// TODO: Find out if we can get this covered by tests instead of excluding it.
//       Rather low priority though,
//       as this probably does not actually need to be tested.
// LCOV_EXCL_START
/// Index a rectangle, traversing it row by row.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rect {
	height: usize,
	width: usize,
}
// LCOV_EXCL_STOP

impl Rect {
	pub fn new(height: usize, width: usize) -> Rect {
		Rect { height, width }
	}
}

impl Indexer for Rect {
	type Index = (usize, usize);

	fn to_num(&self, i: Self::Index) -> usize {
		assert!(self.is_valid(i));
		let (j, k) = i;
		j * self.width + k
	}

	fn to_index(&self, n: usize) -> Self::Index {
		assert!(self.in_range(n));
		let j = n / self.width;
		let k = n - j * self.width;
		(j, k)
	}

	fn range(&self) -> usize {
		self.width * self.height
	}

	fn is_valid(&self, (j, k): Self::Index) -> bool {
		(j < self.height) & (k < self.width)
	}
}

#[cfg(test)]
mod tests {
	use super::Indexer;

	#[test]
	fn unit() {
		assert_eq!(().range(), 1);
		assert_eq!(().to_index(0), ());
		assert_eq!(().to_num(()), 0);
		assert!(().is_valid(()));
	}

	#[test]
	#[should_panic]
	fn unit_num_oob() {
		().to_index(1);
	} // LCOV_EXCL_LINE

	#[test]
	fn integer() {
		let n = 12;
		assert_eq!(n.range(), n);
		assert_eq!(n.to_index(6), 6);
		assert_eq!(n.to_num(11), 11);
	}

	#[test]
	#[should_panic]
	fn integer_index_oob() {
		let n = 19;
		n.to_num(n);
	} // LCOV_EXCL_LINE

	#[test]
	fn rect() {
		use super::Rect;
		let h = 7;
		let w = 17;
		let rect = Rect::new(h, w);
		assert_eq!(rect.range(), h * w);
		assert_eq!(rect.to_index(0), (0, 0));
		assert_eq!(rect.to_index(w), (1, 0));
		assert_eq!(rect.to_num((0, 0)), 0);
		assert_eq!(rect.to_num((1, 0)), w);
	}

	#[test]
	#[should_panic]
	fn rect_index_oob() {
		use super::Rect;
		let h = 2;
		let w = 3;
		let rect = Rect::new(h, w);
		rect.to_num((2, 2));
	} // LCOV_EXCL_LINE

	#[test]
	#[should_panic]
	fn rect_num_oob() {
		use super::Rect;
		let h = 2;
		let w = 3;
		let rect = Rect::new(h, w);
		rect.to_index(rect.range());
	} // LCOV_EXCL_LINE
}
