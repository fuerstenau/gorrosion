mod bool_mat;

use self::bool_mat::*;

enum Color {
	Black,
	White,
}

struct Board {
	size: usize,
	adj: BoolMat,
	black_stones: BoolVec,
	white_stones: BoolVec,
	black_connections: Option<BoolMat>,
	white_connections: Option<BoolMat>,
}

impl Board {
	fn square(length: usize) -> Board {
		let size = length * length;
		let mut adj = BoolMat::from_diag(&BoolVec::trues(size));
		let index = |j, k| j * length + k;
		for j in 0..length {
			for k in 1..length {
				adj.sym_set(index(j, k - 1), index(j, k));
				adj.sym_set(index(k - 1, j), index(k, j));
			}
		}
		let black_stones = BoolVec::falses(size);
		let white_stones = BoolVec::falses(size);
		let black_connections = Some(BoolMat::falses(size, size));
		let white_connections = Some(BoolMat::falses(size, size));
		Board {
			size,
			adj,
			black_stones,
			white_stones,
			black_connections,
			white_connections,
		}
	}

	fn place_stone(
		adj: &BoolMat,
		i: usize,
		stones: &mut BoolVec,
		col_connections: &mut Option<BoolMat>,
	) {
		stones.set(i);
		if let Some(old_connections) = col_connections {
			let mult = BoolMat::mult;
			let diag = BoolMat::from_diag(stones);
			let restricted_adj = mult(&diag, &mult(adj, &diag));
			let new_connections = mult(
				&old_connections,
				&mult(&restricted_adj, &old_connections),
			);
			// TODO: Refactor this stuff. This is stupid.
			old_connections.assign(new_connections);
		}
	}

	fn place(&mut self, i: usize, col: Color) {
		match col {
			Color::Black => Board::place_stone(
				&self.adj,
				i,
				&mut self.black_stones,
				&mut self.black_connections,
			),
			Color::White => Board::place_stone(
				&self.adj,
				i,
				&mut self.white_stones,
				&mut self.white_connections,
			),
		}
	}
}
