use core::util::indexer::Indexer;
use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BoolVec<'a, I: Indexer> {
	data: Vec<bool>,
	indexer: &'a I,
}

// TODO: There is some renaming to be done.
// TODO: Implement a Set trait.
impl<'a, I> BoolVec<'a, I> {
	/// Create a new boolean vector with all positions being unset.
	pub fn falses(indexer: &I) -> BoolVec {
		let size = indexer.range();
		let data = vec![false; size];
		BoolVec { data, indexer }
	}

	/// Create a new boolean vector with all positions being set.
	pub fn trues(indexer: &I) -> BoolVec {
		let size = indexer.range();
		let data = vec![true; size];
		BoolVec { data, indexer }
	}

	/// This is deprecated.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Retrieve an entry.
	pub fn get(&self, i: I::Index) -> bool {
		self.data[self.indexer.to_num(i)]
	}

	/// Set an entry (to “true”).
	pub fn set(&mut self, i: usize) {
		self.data[self.indexer.to_num(i)] = true;
	}

	/// Get an iterator over all set / true positions.
	pub fn set_positions(&self) -> impl Iterator<Item = I::Index> {
		let len = self.data.len();
		let all_indices = 0..len;
		let res: Vec<usize> =
			all_indices.filter(|n| self.get(*n)).map(self.indexer.to_index).collect();
		res.into_iter()
	}


	/// Consider a vector as a map into the Booleans
	/// and apply a unary operation pointwise.
	// TODO: Find common abstraction of bit_map_unary and bit_map_binary.
	fn bit_map_unary<F: Fn(bool) -> bool>(&self, op: F) -> Self {
		let indexer = self.indexer;
		let size = self.data.len();
		let mut data = Vec::with_capacity(size);
		for i in 0..size {
			data.push(op(self.data[i]));
		}
		BoolVec { data, indexer }
	}

	/// Consider two vector as maps into the Booleans
	/// and apply a binary operation pointwise.
	fn bit_map_binary<F: Fn(bool) -> bool>(&self, other: &Self, op: F) -> Self {
		assert_eq!(self.indexer, other.indexer);
		let indexer = self.indexer;
		let size = self.data.len();
		let mut data = Vec::with_capacity(size);
		for i in 0..size {
			data.push(op(self.data[i], other.data[i]));
		}
		BoolVec { data, indexer }
	}

	/// Intersect two vectors considered as sets.
	pub fn intersection(&self, other: &Self) -> Self {
		self & other
	}

	/// Unite two vectors considered as sets.
	pub fn union(&self, other: &Self) -> Self {
		self | other
	}

	/// Take the complement of a vector
	/// considered as subset of the all-true vector of the same length.
	pub fn complement(&self) -> BoolVec {
		!self
	}
}

impl BitAnd for BoolVec {
	type Output = Self;

	fn bitand(&self, &other: Self) -> Self {
		BoolVec::bit_map_binary(self, other, BitAnd::bitand)
	}
}

impl BitOr for BoolVec {
	type Output = Self;

	fn bitor(&self, &other: Self) -> Self {
		BoolVec::bit_map_binary(self, other, BitOr::bitor)
	}
}

impl Not for BoolVec {
	type Output = Self;

	fn not(&self) -> Self {
		BoolVec::bit_map_unary(self, Not::not)
	}
}
