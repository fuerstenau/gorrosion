use core::util::indexer::Indexer;
use std::ops::{BitAnd, BitOr, Index, IndexMut, Not};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BoolVec<'a, I: Indexer> {
	data: Vec<bool>,
	indexer: &'a I,
}

impl<'a, I: Indexer> Index<I::Index> for BoolVec<'a, I> {
	type Output = bool;

	fn index(&self, i: I::Index) -> &Self::Output {
		self.data[self.indexer.to_num(i)]
	}
}

impl<'a, I: Indexer> IndexMut<I::Index> for BoolVec<'a, I> {
	fn index_mut(&mut self, i: I::Index) -> &mut Self::Output {
		self.data[self.indexer.to_num(i)]
	}
}

// TODO: Allow iteration over set / true positions?
impl<'a, I> BoolVec<'a, I> {
	/// Create a new boolean vector with all positions being unset.
	pub fn falses(indexer: &I) -> Self {
		let size = indexer.range();
		let data = vec![false; size];
		BoolVec { data, indexer }
	}

	/// Create a new boolean vector with all positions being set.
	pub fn trues(indexer: &I) -> Self {
		let size = indexer.range();
		let data = vec![true; size];
		BoolVec { data, indexer }
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

	/// Consider two vectors as maps into the Booleans
	/// and apply a binary operation pointwise.
	fn bit_map_binary<F: Fn(bool) -> bool>(
		&self,
		other: &Self,
		op: F,
	) -> Self {
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
	pub fn complement(&self) -> Self {
		!self
	}
}

impl<'a, I> BitAnd for BoolVec<'a, I> {
	type Output = Self;

	fn bitand(&self, &other: Self) -> Self {
		BoolVec::bit_map_binary(self, other, BitAnd::bitand)
	}
}

impl<'a, I> BitOr for BoolVec<'a, I> {
	type Output = Self;

	fn bitor(&self, &other: Self) -> Self {
		BoolVec::bit_map_binary(self, other, BitOr::bitor)
	}
}

impl<'a, I> Not for BoolVec<'a, I> {
	type Output = Self;

	fn not(&self) -> Self {
		BoolVec::bit_map_unary(self, Not::not)
	}
}
