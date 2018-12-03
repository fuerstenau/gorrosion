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
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolVec<I: Indexer> {
	data: Vec<bool>,
	indexer: I,
}

impl<I: Indexer> Index<I::Index> for BoolVec<I> {
	type Output = bool;

	fn index(&self, i: I::Index) -> &Self::Output {
		&self.data[self.indexer.to_num(i)]
	}
}

impl<I: Indexer> IndexMut<I::Index> for BoolVec<I> {
	fn index_mut(&mut self, i: I::Index) -> &mut Self::Output {
		&mut self.data[self.indexer.to_num(i)]
	}
}

// TODO: Allow iteration over set / true positions?
impl<I> BoolVec<I>
where
	I: Indexer,
{
	/// Wrap an existing Vec<bool> into a Boolen vector.
	pub fn from_data(data: Vec<bool>, indexer: I) -> BoolVec<I> {
		assert_eq!(data.len(), indexer.range());
		BoolVec { data, indexer }
	}

	/// Create a new Boolean vector with all positions being unset.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// let indexer = 17;
	/// let falses = BoolVec::falses(indexer);
	/// for i in 0..17 {
	///	assert_eq!(falses[i], false);
	/// }
	/// ```
	pub fn falses(indexer: I) -> Self {
		let size = indexer.range();
		let data = vec![false; size];
		BoolVec { data, indexer }
	}

	/// Create a new Boolean vector with all positions being set.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// let indexer = 23;
	/// let trues = BoolVec::trues(indexer);
	/// for i in 0..23 {
	///	assert_eq!(trues[i], true);
	/// }
	/// ```
	pub fn trues(indexer: I) -> Self {
		let size = indexer.range();
		let data = vec![true; size];
		BoolVec { data, indexer }
	}

	/// Consider a vector as a map into the Booleans
	/// and apply a unary operation pointwise.
	// TODO: Find common abstraction of bit_map_unary and bit_map_binary.
	// TODO: Implement consuming version.
	fn bit_map_unary<F: Fn(bool) -> bool>(&self, op: F) -> Self {
		let indexer = self.indexer.clone();
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
		let indexer = self.indexer.clone();
		let size = self.data.len();
		let mut data = Vec::with_capacity(size);
		for i in 0..size {
			data.push(op(self.data[i], other.data[i]));
		}
		BoolVec { data, indexer }
	}

	/// Intersect two vectors considered as sets.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// let indexer = 3;
	/// let a = BoolVec::from_data(vec![true, true, false], indexer);
	/// let b = BoolVec::from_data(vec![false, true, true], indexer);
	/// let c = BoolVec::from_data(vec![false, true, false], indexer);
	/// assert_eq!(a.intersection(&b), c);
	/// ```
	pub fn intersection(&self, other: &Self) -> Self {
		self & other
	}

	/// Unite two vectors considered as sets.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// let indexer = 3;
	/// let a = BoolVec::from_data(vec![false, true, false], indexer);
	/// let b = BoolVec::from_data(vec![false, false, true], indexer);
	/// let c = BoolVec::from_data(vec![false, true, true], indexer);
	/// assert_eq!(a.union(&b), c);
	/// ```
	pub fn union(&self, other: &Self) -> Self {
		self | other
	}

	/// Take the complement of a vector
	/// considered as subset of the all-true vector of the same length.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// let indexer = 3;
	/// let a = BoolVec::from_data(vec![true, true, false], indexer);
	/// let b = BoolVec::from_data(vec![false, false, true], indexer);
	/// assert_eq!(a.complement(), b);
	/// ```
	pub fn complement(&self) -> Self {
		!self
	}

	/// Change the way the vector is indexed.
	///
	/// # Examples
	///
	/// ```
	/// # use gorrosion::core::util::bool_vec::BoolVec;
	/// # use gorrosion::core::util::indexer;
	/// let rect = indexer::Rect::new(2, 3);
	/// let num = 6;
	/// let vec_rect = BoolVec::falses(rect);
	/// let vec_num = BoolVec::falses(num);
	/// assert_eq!(vec_num, vec_rect.reindex(num));
	/// ```
	pub fn reindex<J>(self, indexer: J) -> BoolVec<J>
	where
		J: Indexer,
	{
		assert_eq!(self.indexer.range(), indexer.range());
		let data = self.data;
		BoolVec { indexer, data }
	}

	/// Get a reference to the indexer.
	pub fn indexer(&self) -> &I {
		&self.indexer
	}
}

impl<'now, I> BitAnd for &'now BoolVec<I>
where
	I: Indexer,
{
	type Output = BoolVec<I>;

	fn bitand(self, other: Self) -> Self::Output {
		BoolVec::bit_map_binary(self, other, BitAnd::bitand)
	}
}

impl<'now, I> BitOr for &'now BoolVec<I>
where
	I: Indexer,
{
	type Output = BoolVec<I>;

	fn bitor(self, other: Self) -> Self::Output {
		BoolVec::bit_map_binary(self, other, BitOr::bitor)
	}
}

impl<'now, I> Not for &'now BoolVec<I>
where
	I: Indexer,
{
	type Output = BoolVec<I>;

	fn not(self) -> Self::Output {
		BoolVec::bit_map_unary(self, Not::not)
	}
}

// TODO: Only keep until code coverage tools can handle doc tests.
#[cfg(test)]
mod tests {
	use super::BoolVec;
	use core::util::indexer;

	#[test]
	fn doc_tests() {
		let indexer = 17;
		let falses = BoolVec::falses(indexer);
		for i in 0..17 {
			assert_eq!(falses[i], false);
		}
		let indexer = 23;
		let trues = BoolVec::trues(indexer);
		for i in 0..23 {
			assert_eq!(trues[i], true);
		}
		let indexer = 3;
		let a = BoolVec::from_data(vec![true, true, false], indexer);
		let b = BoolVec::from_data(vec![false, true, true], indexer);
		let c = BoolVec::from_data(vec![false, true, false], indexer);
		assert_eq!(a.intersection(&b), c);
		let indexer = 3;
		let a = BoolVec::from_data(vec![false, true, false], indexer);
		let b = BoolVec::from_data(vec![false, false, true], indexer);
		let c = BoolVec::from_data(vec![false, true, true], indexer);
		assert_eq!(a.union(&b), c);
		let indexer = 3;
		let a = BoolVec::from_data(vec![true, true, false], indexer);
		let b = BoolVec::from_data(vec![false, false, true], indexer);
		assert_eq!(a.complement(), b);
		let rect = indexer::Rect::new(2, 3);
		let num = 6;
		let vec_rect = BoolVec::falses(rect);
		let vec_num = BoolVec::falses(num);
		assert_eq!(vec_num, vec_rect.reindex(num));
	}
}
