//! Matrices are at the heart of finite-dimensional linear algebra
//! and we, too, will use them to perform the calculations we want to do.
//! While any matrix can be considered to be a vector,
//! matrices have the additional structure of multiplication
//! which allows us to elegantly chain linear transformations.
//! Non-square matrices allow us to change the underlying set
//! when applying them to characteristic functions.

use self::indexer::Indexer;
use core::util::bool_vec::BoolVec;
use core::util::indexer;
use std::ops::{Index, IndexMut, Mul};

// TODO: There is some renaming to be done.
//       * s/eval/apply/ ?

/// A matrix with values in `bool`, the two-element semi-ring.
/// Since not all matrices represent endomorphisms,
/// rows and columns each have their own `Indexer`.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoolMat<J: Indexer, K: Indexer> {
	rows: J,
	columns: K,
	contents: BoolVec<indexer::Rect>,
}

impl<J, K> Index<(J::Index, K::Index)> for BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
{
	type Output = bool;

	fn index(&self, (j, k): (J::Index, K::Index)) -> &Self::Output {
		let i = (self.rows.to_num(j), self.columns.to_num(k));
		&self.contents[i]
	}
}

impl<J, K> IndexMut<(J::Index, K::Index)> for BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
{
	fn index_mut(
		&mut self,
		(j, k): (J::Index, K::Index),
	) -> &mut Self::Output {
		let i = (self.rows.to_num(j), self.columns.to_num(k));
		&mut self.contents[i]
	}
}

impl<J, K> BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
{
	/// Create a new Boolean matrix with all entries unset.
	pub fn falses(rows: J, columns: K) -> Self {
		let rect = indexer::Rect::new(rows.range(), columns.range());
		let contents = BoolVec::falses(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}

	/// Create a new Boolean matrix with all entries set.
	pub fn trues(rows: J, columns: K) -> Self {
		let rect = indexer::Rect::new(rows.range(), columns.range());
		let contents = BoolVec::trues(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}

	/// Evaluate the matrix on a vector,
	/// which is considered as a column vector.
	pub fn eval(&self, v: &BoolVec<K>) -> BoolVec<J> {
		let matrix = self * BoolMat::column(v);
		let indexer = matrix.rows;
		matrix.contents.reindex(indexer)
	}
}

impl<I> BoolMat<I, ()>
where
	I: Indexer,
{
	/// Take a vector and write it as column,
	/// i.e. a matrix where the columns are indexed by the unit type.
	fn column(vec: &BoolVec<I>) -> Self {
		let rows = vec.indexer().clone();
		let columns = ();
		let rect = indexer::Rect::new(rows.range(), 1);
		let contents = vec.clone().reindex(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}
}

impl<I> BoolMat<I, I>
where
	I: Indexer,
{
	/// Create a diagonal matrix
	/// whose diagonal entries are given by a vector.
	pub fn from_diag(diag: &BoolVec<I>) -> Self {
		let indexer = diag.indexer();
		let rows = indexer.clone();
		let columns = indexer.clone();
		let mut res = BoolMat::falses(rows, columns);
		let len = indexer.range();
		for i in 0..len {
			res.contents[(i, i)] = diag[indexer.to_index(i)];
		}
		res
	}

	/// Create the identity matrix to a given `Indexer`.
	pub fn id_matrix(indexer: I) -> Self {
		BoolMat::from_diag(&BoolVec::trues(indexer))
	}
}

// TODO: Implement consuming versions.
impl<'now, J, K, L> Mul<&'now BoolMat<K, L>> for &'now BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
	L: Indexer,
{
	type Output = BoolMat<J, L>;

	fn mul(self, other: &BoolMat<K, L>) -> Self::Output {
		assert_eq!(self.columns, other.rows);
		let len = self.columns.range();
		let rows = self.rows.clone();
		let columns = other.columns.clone();
		let height = rows.range();
		let width = columns.range();
		let entry = |j, l| {
			(0..len).map(|k| {
				self.contents[(j, k)] & other.contents[(k, l)]
			}).any(|b| b)
		};
		let mut data = Vec::with_capacity(height * width);
		for j in 0..height {
			for l in 0..width {
				data.push(entry(j, l));
			}
		}
		let rect = indexer::Rect::new(height, width);
		let contents = BoolVec::from_data(data, rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}
}

impl<'now, J, K, L> Mul<BoolMat<K, L>> for &'now BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
	L: Indexer,
{
	type Output = BoolMat<J, L>;

	fn mul(self, other: BoolMat<K, L>) -> Self::Output {
		self * &other
	}
}

impl<'now, J, K, L> Mul<&'now BoolMat<K, L>> for BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
	L: Indexer,
{
	type Output = BoolMat<J, L>;

	fn mul(self, other: &'now BoolMat<K, L>) -> Self::Output {
		&self * other
	}
}

impl<J, K, L> Mul<BoolMat<K, L>> for BoolMat<J, K>
where
	J: Indexer,
	K: Indexer,
	L: Indexer,
{
	type Output = BoolMat<J, L>;

	fn mul(self, other: BoolMat<K, L>) -> Self::Output {
		&self * &other
	}
}
