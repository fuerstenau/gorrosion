use core::board::Board;
use core::util::bool_mat::BoolMat;
use core::util::bool_vec::BoolVec;
use core::util::indexer::Indexer;

// TODO: Kick out the rules into their own module.
// TODO: There is some renaming to be done.
// TODO: We might be able to get rid of a few of those lifetimes.

// TODO: #[derive(Clone, PartialEq, Eq)]
#[derive(PartialEq, Eq)]
struct PlayerState<'a, 'board: 'a, T: 'board + Board> {
	board: &'board T,
	stones: BoolVec<'board, T::I>,
	connections: BoolMat<'a, 'board, 'board, T::I, T::I>,
	aga_captures: usize,
	captures: usize,
}

impl<'a, 'board, T> PlayerState<'a, 'board, T>
where
	T: Board,
{
	fn place_stone(&mut self, i: <T::I as Indexer>::Index) {
		self.stones[i] = true;
		let diag = &BoolMat::from_diag(&self.stones);
		let adj = self.board.adjacencies();
		// We restrict ourselves to the adjacencies within our stones.
		let adj = &(diag * &(adj * diag));
		let conn = &self.connections;
		let new_connections = conn * &(adj * conn);
		self.connections = new_connections;
	}

	fn survivors(
		&self,
		free: &BoolVec<'board, T::I>,
	) -> BoolVec<'board, T::I> {
		let adj = self.board.adjacencies();
		let conn = &self.connections;
		(conn * adj).eval(free)
	}

	fn kill(&mut self, zombies: &BoolVec<'board, T::I>) {
		// The zombies infect everything in contact with them
		let zombies = self.connections.eval(zombies);
		// Retain all those that have not become zombies
		self.stones = &self.stones & &!&zombies;
		// Keep only the connections of the surviving groups
		self.connections =
			&self.connections * &BoolMat::from_diag(&self.stones);
	}

	fn kill_dead(&mut self, liberties: &BoolVec<'board, T::I>) {
		let dead = self.survivors(liberties).complement();
		self.kill(&dead);
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Color {
	Black,
	White,
}

impl Color {
	fn other(&self) -> Color {
		match self {
			Color::Black => Color::White,
			Color::White => Color::Black,
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
struct GameState<'a, 'board: 'a, T: 'board + Board> {
	black: PlayerState<'a, 'board, T>,
	white: PlayerState<'a, 'board, T>,
	to_move: Color,
}

impl<'a, 'board: 'a, T: 'board + Board> GameState<'a, 'board, T> {
	fn free(&self) -> BoolVec<'board, T::I> {
		let black = &self.black.stones;
		let white = &self.white.stones;
		BoolVec::union(black, white).complement()
	}

	fn kill_dead(&mut self, color: Color) {
		let liberties = self.free();
		self.player_state(color).kill_dead(&liberties);
	}

	fn player_state(
		&mut self,
		color: Color,
	) -> &mut PlayerState<'a, 'board, T> {
		match color {
			Color::Black => &mut self.black,
			Color::White => &mut self.white,
		}
	}

	fn place_stone(&mut self, i: <T::I as Indexer>::Index, color: Color) {
		self.player_state(color).place_stone(i);
	}
}

#[derive(Copy, Clone)]
enum Action<T: Board> {
	Pass,
	Resign,
	Place(<T::I as Indexer>::Index),
}

// FIXME: #[derive(Copy, Clone)]
#[derive(Clone)]
struct Move<T: Board> {
	player: Color,
	action: Action<T>,
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

impl<'a, 'board: 'a, T: 'board + Board> GameState<'a, 'board, T> {
	fn legal_move(&self, mov: Move<T>, rules: LocalRules) -> bool {
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
		i: <T::I as Indexer>::Index,
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
		let is_free = self.free()[i];
		let kills_something =
			other.survivors(&liberties) == other.stones;
		let is_suicide = (!kills_something)
			& (player.survivors(&liberties) != player.stones);
		is_free & (!is_suicide | rules.suicide_allowed)
	}
}

struct GameNode<'a, 'b, 'board: 'a, T: 'board + 'b + Board> {
	state: GameState<'a, 'board, T>,
	// TODO: Think about this triple lifetime ('b, 'b, 'b).
	//       Is this actually the intended behaviour?
	prev_node: Option<&'b GameNode<'b, 'b, 'b, T>>,
	last_move: Option<Move<T>>,
	white_ghosts: BoolVec<'board, T::I>,
	black_ghosts: BoolVec<'board, T::I>,
}

impl<'a, 'b, 'board: 'a, T: 'board + Board> GameNode<'a, 'b, 'board, T> {
	fn legal_move(&self, mov: Move<T>, rules: Rules) -> bool {
		let locally_legal =
			self.state.legal_move(mov, rules.local_rules);
		let mut next_state = self.state.clone();
		let ko = if let Action::Place(i) = mov.action {
			next_state.place_stone(i, mov.player);
			next_state.kill_dead(mov.player.other());
			next_state.kill_dead(mov.player);
			if let Some(prev) = self.prev_node {
				prev.state == next_state
			} else {
				false
			}
		} else {
			false
		};
		locally_legal & (!ko)
	}
}
