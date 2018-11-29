mod bool_mat;

use self::bool_mat::*;

pub struct Field {
	size: usize,
	adj: BoolMat,
	black_stones: BoolVec,
	white_stones: BoolVec,
}

impl Field {
	pub fn square(length: usize) -> Field {
		let size = length * length;
		let mut adj = BoolMat::from_diag(BoolVec::trues(size));
		let index = |j, k| j * length + k;
		for j in 0..length {
			for k in 1..length {
				adj.sym_set(index(j, k - 1), index(j, k));
				adj.sym_set(index(k - 1, j), index(k, j));
			}
		}
		let black_stones = BoolVec::falses(size);
		let white_stones = BoolVec::falses(size);
		Field {
			size,
			adj,
			black_stones,
			white_stones,
		}
	}
}
