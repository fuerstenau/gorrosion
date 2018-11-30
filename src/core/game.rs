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

#[derive(Clone, Copy, Eq, PartialEq)]
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

impl<'a, T: 'a + Board> GameState<'a, T> {
	fn player_state(&mut self, color: Color) -> &mut PlayerState<'a, T> {
		match color {
			Color::Black => &mut self.black,
			Color::White => &mut self.white,
		}
	}

	fn place_stone(&mut self, i: T::Index, color: Color) {
		self.player_state(color).place_stone(i);
	}
}

#[derive(Copy, Clone)]
enum Action<I> {
	Pass,
	Resign,
	Place(I),
}

#[derive(Copy, Clone)]
struct Move<I> {
	player: Color,
	action: Action<I>,
}

struct LocalRules {
	suicide_allowed: bool,
	alternate_play: bool,
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
		let has_turn =
			(!rules.alternate_play) | (mov.player == self.to_move);
		let legal_action = match mov.action {
			Action::Pass => true,
			Action::Resign => true,
			Action::Place(i) => {
				self.legal_placement(i, mov.player, rules)
			}
		};
		has_turn & legal_action
	}

	fn legal_placement(
		&self,
		i: T::Index,
		player_color: Color,
		rules: LocalRules,
	) -> bool {
		let mut future = self.clone();
		match player_color {
			Color::Black => future.black.place_stone(i),
			Color::White => future.white.place_stone(i),
		};
		let liberties = future.free();
		let (player, other) = match player_color {
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

struct GameNode<'a, T: 'a + Board> {
	state: GameState<'a, T>,
	prev_state: Option<&'a GameNode<'a, T>>,
	last_move: Option<Move<T::Index>>,
	white_ghosts: BoolVec,
	black_ghosts: BoolVec,
}

impl<'a, T: 'a + Board> GameNode<'a, T> {
	fn legal_move(&self, mov: Move<T::Index>, rules: Rules) -> bool {
		let locally_legal =
			self.state.legal_move(mov, rules.local_rules);
		let mut state = self.state.clone();
		let ko = if let Action::Place(i) = mov.action {
			state.place_stone(i, mov.player);
			true
		} else {
			false
		};
		locally_legal & (!ko)
	}
}
