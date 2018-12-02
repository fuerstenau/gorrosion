use core::board::Board;
use core::util::bool_mat::BoolMat;
use core::util::bool_vec::BoolVec;
use core::util::indexer::Indexer;

// TODO: Kick out the rules into their own module.
// TODO: There is some renaming to be done.
// TODO: We might be able to get rid of a few of those lifetimes.

#[derive(PartialEq, Eq)]
struct PlayerState<'board, T: 'board + Board> {
	board: &'board T,
	stones: BoolVec<T::I>,
	connections: BoolMat<T::I, T::I>,
}

impl<'board, T> Clone for PlayerState<'board, T>
where
	T: Board,
{
	fn clone(&self) -> Self {
		let board = self.board;
		let stones = self.stones.clone();
		let connections = self.connections.clone();
		PlayerState {
			board,
			stones,
			connections,
		}
	}
}

impl<'board, T> PlayerState<'board, T>
where
	T: Board,
{
	fn place_stone(&mut self, i: <T::I as Indexer>::Index) {
		self.stones[i] = true;
		let diag = &BoolMat::from_diag(&self.stones);
		let adj = self.board.adjacencies();
		// We restrict ourselves to the adjacencies within our stones.
		let adj = diag * adj * diag;
		self.connections = {
			let conn = &self.connections;
			let new_connections = conn * adj * conn;
			new_connections
		};
	}

	fn survivors(&self, free: &BoolVec<T::I>) -> BoolVec<T::I> {
		let adj = self.board.adjacencies();
		let conn = &self.connections;
		(conn * adj).eval(free)
	}

	fn kill(&mut self, zombies: &BoolVec<T::I>) {
		// The zombies infect everything in contact with them
		let zombies = self.connections.eval(zombies);
		// Retain all those that have not become zombies
		self.stones = &self.stones & &!&zombies;
		// Keep only the connections of the surviving groups
		self.connections =
			&self.connections * &BoolMat::from_diag(&self.stones);
	}

	fn kill_dead(&mut self, liberties: &BoolVec<T::I>) {
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

#[derive(PartialEq, Eq)]
struct GameState<'board, T: 'board + Board> {
	black: PlayerState<'board, T>,
	white: PlayerState<'board, T>,
	to_move: Color,
}

impl<'board, T> Clone for GameState<'board, T>
where
	T: Board,
{
	fn clone(&self) -> Self {
		let black = self.black.clone();
		let white = self.white.clone();
		let to_move = self.to_move;
		GameState {
			black,
			white,
			to_move,
		}
	}
}

impl<'board, T> GameState<'board, T>
where
	T: Board,
{
	fn free(&self) -> BoolVec<T::I> {
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
	) -> &mut PlayerState<'board, T> {
		match color {
			Color::Black => &mut self.black,
			Color::White => &mut self.white,
		}
	}

	fn place_stone(&mut self, i: <T::I as Indexer>::Index, color: Color) {
		self.player_state(color).place_stone(i);
	}
}

// TODO: The following implementation feels more idiomatic
//       than the actual implementation following afterwards.
//       However, then rustc refuses to place the Copy marker on Action.
//       This feels like a bug and should be reported,
//       even if only to learn the errors of our ways.
/*

#[derive(Copy, Clone)]
enum Action<T: Board> {
	Pass,
	Resign,
	Place(<T::I as Indexer>::Index),
}

#[derive(Copy, Clone)]
struct Move<T: Board>
where Action<T>: Copy {
	player: Color,
	action: Action<T>,
}

*/

#[derive(Copy, Clone)]
enum Action<Index: Copy> {
	Pass,
	Resign,
	Place(Index),
}

#[derive(Copy, Clone)]
struct Move<T: Board> {
	player: Color,
	action: Action<<T::I as Indexer>::Index>,
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

struct GameNode<'a, 'board: 'a, T: 'board + Board> {
	state: GameState<'board, T>,
	// TODO: Think about this triple lifetime ('b, 'b, 'b).
	//       Is this actually the intended behaviour?
	prev_node: Option<&'a GameNode<'a, 'a, T>>,
	last_move: Option<Move<T>>,
}
