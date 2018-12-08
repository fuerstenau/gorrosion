use core::board::Board;

trait Rules<T: Board> {
	type Game;
	type Move;
	type Config;
	type Outcome;

	fn new(config: Self::Config) -> Self;
	fn legal_move(&self, game: &Self::Game, m: &Self::Move) -> bool;
	fn make_move(&self, game: &mut Self::Game, m: Self::Move);
	fn outcome(&self, game: &Self::Game) -> Option<Self::Outcome>;
	fn has_ended(&self, game: &Self::Game) -> bool {
		!self.outcome(game).is_none()
	}
}
