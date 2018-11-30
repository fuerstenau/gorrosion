use core::board::Board;
use core::bool_mat::*;

enum Color {
	Black,
	White,
}

struct PlayerState<'a, T: 'a + Board> {
	board: &'a T,
	stones: BoolVec,
	connections: BoolMat,
}

impl<'a, T: Board> PlayerState<'a, T> {
	fn place_stone(&mut self, i: T::Index) {
		self.stones.set(self.board.index_to_num(i));
		let mult = BoolMat::mult;
		let diag = BoolMat::from_diag(&self.stones);
		let adj = self.board.adjacencies();
		let restricted_adj = mult(&diag, &mult(adj, &diag));
		let new_connections = mult(
			&self.connections,
			&mult(&restricted_adj, &self.connections),
		);
		self.connections = new_connections;
	}

	fn survivors(&self, free: &BoolVec) -> BoolVec {
		let adj = self.board.adjacencies();
		BoolMat::mult(adj, &self.connections).eval(free)
	}

	fn kill(&mut self, zombies: &BoolVec) {
		// The zombies infect everything in contact with them
		let zombies = self.connections.eval(zombies);
		self.stones = BoolVec::intersection(
			&self.stones,
			&zombies.complement(),
		);
		self.connections = BoolMat::mult(
			&self.connections,
			&BoolMat::from_diag(&self.stones),
		);
	}
}
