//! Vectors containing Boolean values are essentially sets
//! represented by their characteristic functions.
//! They are useful since they allow many computations
//! to be implemented as simple linear algebra over the two-element semi-ring.

use core::util::indexer::Indexer;
use std::ops::{BitAnd, BitOr, Index, IndexMut, Not};

/// A vector with values in `bool`, the two-element semi-ring.
/// For convenience and a poor emulation of type-checking
/// we do not index these over integers directly
/// but use an `Indexer`.
#[derive(Eq, PartialEq, Debug)]
pub struct BoolVec<'a, I: 'a + Indexer> {
	data: Vec<bool>,
	indexer: &'a I,
}

impl<'a, I> Clone for BoolVec<'a, I>
where
	I: Indexer,
{
	fn clone(&self) -> Self {
		let data = self.data.clone();
		let indexer = self.indexer;
		BoolVec { data, indexer }
	}
}

impl<'a, I: Indexer> Index<I::Index> for BoolVec<'a, I> {
	type Output = bool;

	fn index(&self, i: I::Index) -> &Self::Output {
		&self.data[self.indexer.to_num(i)]
	}
}

impl<'a, I: Indexer> IndexMut<I::Index> for BoolVec<'a, I> {
	fn index_mut(&mut self, i: I::Index) -> &mut Self::Output {
		&mut self.data[self.indexer.to_num(i)]
	}
}

// TODO: Allow iteration over set / true positions?
impl<'a, I> BoolVec<'a, I>
where
	I: Indexer,
{
	/// Wrap an existing Vec<bool> into a Boolen vector.
	pub fn from_data(data: Vec<bool>, indexer: &'a I) -> BoolVec<'a, I> {
		assert_eq!(data.len(), indexer.range());
		BoolVec { data, indexer }
	}

	/// Create a new Boolean vector with all positions being unset.
	pub fn falses(indexer: &'a I) -> Self {
		let size = indexer.range();
		let data = vec![false; size];
		BoolVec { data, indexer }
	}

	/// Create a new Boolean vector with all positions being set.
	pub fn trues(indexer: &'a I) -> Self {
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
	fn bit_map_binary<F: Fn(bool, bool) -> bool>(
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

	/// Change the way the vector is indexed.
	pub fn reindex<'b, J>(self, indexer: &'b J) -> BoolVec<'b, J>
	where
		J: Indexer,
	{
		assert_eq!(self.indexer.range(), indexer.range());
		let data = self.data;
		BoolVec { indexer, data }
	}

	/// Get a reference to the indexer.
	pub fn indexer(&self) -> &'a I {
		self.indexer
	}
}

impl<'now, 'a, I> BitAnd for &'now BoolVec<'a, I>
where
	I: Indexer,
{
	type Output = BoolVec<'a, I>;

	fn bitand(self, other: Self) -> Self::Output {
		BoolVec::bit_map_binary(self, other, BitAnd::bitand)
	}
}

impl<'now, 'a, I> BitOr for &'now BoolVec<'a, I>
where
	I: Indexer,
{
	type Output = BoolVec<'a, I>;

	fn bitor(self, other: Self) -> Self::Output {
		BoolVec::bit_map_binary(self, other, BitOr::bitor)
	}
}

impl<'now, 'a, I> Not for &'now BoolVec<'a, I>
where
	I: Indexer,
{
	type Output = BoolVec<'a, I>;

	fn not(self) -> Self::Output {
		BoolVec::bit_map_unary(self, Not::not)
	}
}
