use core::board::Board;
use core::bool_mat::*;

#[derive(Clone)]
struct PlayerState<'a, T: 'a + Board> {
	board: &'a T,
	stones: BoolVec,
	connections: BoolMat,
	aga_captures: usize,
	captures: usize,
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

#[derive(Clone, Copy)]
enum Color {
	Black,
	White,
}

#[derive(Clone)]
struct GameState<'a, T: 'a + Board> {
	black: PlayerState<'a, T>,
	white: PlayerState<'a, T>,
	to_move: Color,
}

enum Move<I> {
	Pass,
	Place(I),
}

struct LocalRules {
	suicide_allowed: bool,
}

struct Rules {
	local_rules: LocalRules,
	superko: bool,
	// This is twice the value of komi to allow for half points
	bikomi: usize,
	fixed_handicap: bool,
}

impl<'a, T: 'a + Board> GameState<'a, T> {
	fn free(&self) -> BoolVec {
		let black = &self.black.stones;
		let white = &self.white.stones;
		BoolVec::union(black, white).complement()
	}

	fn legal_move(&self, mov: Move<T::Index>, rules: LocalRules) -> bool {
		match mov {
			Move::Pass => true,
			Move::Place(i) => self.legal_placement(i, rules),
		}
	}

	fn legal_placement(&self, i: T::Index, rules: LocalRules) -> bool {
		let mut future = self.clone();
		match self.to_move {
			Color::Black => future.black.place_stone(i),
			Color::White => future.white.place_stone(i),
		};
		let liberties = future.free();
		let (player, other) = match self.to_move {
			Color::Black => (future.black, future.white),
			Color::White => (future.white, future.black),
		};
		let is_free = self.free().get(player.board.index_to_num(i));
		let kills_something =
			other.survivors(&liberties) == other.stones;
		let is_suicide = (!kills_something)
			& (player.survivors(&liberties) != player.stones);
		is_free & (!is_suicide | rules.suicide_allowed)
	}
}
