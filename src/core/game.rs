use core::board::Board;
use core::bool_mat::*;

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
}
