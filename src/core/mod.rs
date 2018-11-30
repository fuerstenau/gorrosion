mod board;
mod bool_mat;
mod game;

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
	black_connections: BoolMat,
	white_connections: BoolMat,
}

impl Board {
	fn living_stones(
		adj: &BoolMat,
		connections: &BoolMat,
		free: &BoolVec,
	) -> BoolVec {
		BoolMat::mult(&adj, &connections).eval(free)
	}

	fn kill(&mut self, col: Color) {
		let free =
			BoolVec::union(&self.black_stones, &self.white_stones)
				.complement();
		let living = match col {
			Color::Black => Board::living_stones(
				&self.adj,
				&self.black_connections,
				&free,
			),
			Color::White => Board::living_stones(
				&self.adj,
				&self.white_connections,
				&free,
			),
		};
		match col {
			Color::Black => {
				self.black_stones = living;
				self.black_connections = BoolMat::mult(
					&self.black_connections,
					&BoolMat::from_diag(&self.black_stones),
				)
			}
			Color::White => {
				self.white_stones = living;
				self.white_connections = BoolMat::mult(
					&self.white_connections,
					&BoolMat::from_diag(&self.white_stones),
				)
			}
		}
	}
}
