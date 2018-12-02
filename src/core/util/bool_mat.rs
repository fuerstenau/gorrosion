use self::indexer::Indexer;
use core::util::bool_vec::BoolVec;
use core::util::indexer;
use std::ops::Mul;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoolMat<'a, J: Indexer, K: Indexer> {
	rows: &'a J,
	columns: &'a K,
	contents: BoolVec<'a, indexer::Rect>,
}

impl<'a, J, K> BoolMat<'a, J, K> {
	pub fn falses(rows: &J, columns: &K) -> Self {
		let rect = indexer::Rect::new(rows.range(), columns.range());
		let contents = BoolVec::falses(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}

	pub fn falses(rows: &J, columns: &K) -> Self {
		let rect = indexer::Rect::new(rows.range(), columns.range());
		let contents = BoolVec::trues(rect);
		BoolMat {
			rows,
			columns,
			contents,
		}
	}

	pub fn set(&mut self, i: usize, k: usize) {
		self.contents.set(i * self.width + k);
	}

	pub fn sym_set(&mut self, i: usize, k: usize) {
		self.set(i, k);
		self.set(k, i);
	}

	pub fn get(&self, i: usize, k: usize) -> bool {
		self.contents.get(i * self.width + k)
	}

	pub fn eval(&self, v: &BoolVec<'a, K>) -> BoolVec<'a, J> {
		let matrix = self * BoolMat::column(v);
		let indexer = matrix.rows;
		let data = matrix.contents.data;
		BoolVec { indexer, data }
	}
}

impl<'a, I> BoolMat<'a, I, ()> {
	fn column(vec: &BoolVec<'a, I>) -> Self {
		let rows = contents.indexer;
		let indexer = indexer::Rect::new(rows.range(), 1);
		let data = contents.data.clone();
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
	pub fn from_diag(diag: &BoolVec<'a, I>) -> Self {
		let rows = diag.indexer;
		let columns = diag.indexer;
		let mut res = BoolMat::falses(rows, columns);
		for i in diag.set_positions() {
			res.contents.set((i, i));
		}
		res
	}

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
