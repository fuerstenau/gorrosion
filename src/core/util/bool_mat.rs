//! Matrices are a the heart of finite-dimensional linear algebra
//! and we, too, will use them to perform the calculations we want to do.
//! While any matrix can be considered to be a vector,
//! matrices have the additional structure of multiplication
//! which allows us to change the underlying set
//! when transforming characteristic functions.

use self::indexer::Indexer;
use core::util::bool_vec::BoolVec;
use core::util::indexer;
use std::ops::{Index, IndexMut, Mul};

/// A matrix with values in `bool`, the two-element semi-ring.
/// Since not all matrices represent endomorphisms,
/// rows and columns each have their own `Indexer`.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoolMat<'a, J: Indexer, K: Indexer> {
	rows: &'a J,
	columns: &'a K,
	contents: BoolVec<'a, indexer::Rect>,
}

impl<'a, J, K> Index<(J::Index, K::Index)> for BoolMat<'a, J, K>
where
	J: Indexer,
	K: Indexer,
{
	type Output = bool;

	fn index(&self, (j, k): (J::Index, K::Index)) -> &Self::Output {
		let i = (self.rows.to_num(j), self.columns.to_num(k));
		self.contents[i]
	}
}

impl<'a, J, K> IndexMut<(J::Index, K::Index)> for BoolMat<'a, J, K>
where
	J: Indexer,
	K: Indexer,
{
	fn index_mut(
		&mut self,
		(j, k): (J::Index, K::Index),
	) -> &mut Self::Output {
		let i = (self.rows.to_num(j), self.columns.to_num(k));
		self.contents[i]
	}
}

impl<'a, J, K> BoolMat<'a, J, K> {
	/// Create a new Boolean matrix with all entries unset.
	pub fn falses(rows: &J, columns: &K) -> Self {
		let rect = indexer::Rect::new(rows.range(), columns.range());
		let contents = BoolVec::falses(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}

	/// Create a new Boolean matrix with all entries set.
	pub fn trues(rows: &J, columns: &K) -> Self {
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
	pub fn eval(&self, v: &BoolVec<'a, K>) -> BoolVec<'a, J> {
		let matrix = self * BoolMat::column(v);
		let indexer = matrix.rows;
		let data = matrix.contents.data;
		BoolVec { indexer, data }
	}
}

impl<'a, I> BoolMat<'a, I, ()> {
	/// Take a vector and write it as column,
	/// i.e. a matrix where the columns are indexed by the unit type.
	fn column(vec: &BoolVec<'a, I>) -> Self {
		let rows = vec.contents.indexer;
		let indexer = indexer::Rect::new(rows.range(), 1);
		let data = vec.data.clone();
		let contents = BoolVec { indexer, data };
		let columns = ();
		BoolMat {
			rows,
			columns,
			contents,
		}
	}
}

impl<'a, I> BoolMat<'a, I, I> {
	/// Create a diagonal matrix
	/// whose diagonal entries are given by a vector.
	pub fn from_diag(diag: &BoolVec<'a, I>) -> Self {
		let rows = diag.indexer;
		let columns = diag.indexer;
		let mut res = BoolMat::falses(rows, columns);
		for i in diag.set_positions() {
			res.contents.set((i, i));
		}
		res
	}

	/// Create the identity matrix to a given `Indexer`.
	pub fn id_matrix(indexer: &'a I) -> Self {
		BoolMat::from_diag(&BoolVec::ones(indexer))
	}
}

impl<'a, J, K, L> Mul<BoolMat<'a, K, L>> for BoolMat<'a, J, K> {
	type Output = BoolMat<'a, J, L>;

	fn mul(&self, &other: BoolMat<'a, K, L>) -> Self::Output {
		assert_eq!(self.columns, other.rows);
		let len = self.columns.range();
		let rows = self.rows;
		let columns = other.columns;
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
		let indexer = indexer::Rect::new(height, width);
		let contents = BoolVec { indexer, data };
		BoolMat {
			rows,
			columns,
			contents,
		}
	}
}
