
use std::iter::successors;

pub use pleco::
{
  core::Player,
  core::PieceType,
  core::Piece,
  board::piece_locations::PieceLocations, //Minimal board impl

  core::piece_move::MoveType,
  core::piece_move::BitMove as Move, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html,
  core::move_list::MoveList,

  core::sq::SQ as Cell,
  core::bitboard::BitBoard as CellsSet

};

/* Our structure:

Game = History + Board
   - history : Vec<Fen>
   - move
   - board
Board
  - pleco_board : pleco::Board
  - move
*/

/* List of resources to show

  tools::eval::Eval,
  helper::Helper,
  helper::prelude,
  bot_prelude

*/

/*
Commands

.game.new - creates game with default board
.game.from.fen - creates game [feature: game from fen]
[issue: implement command game.from.fen]

.games.list - list games [feature: persistence][issue: imlement persistency]
.game.open [id] - opens the game from storage [feature: persistence]
.game.save - saves cureent game state [feature: persistence]

.quit - exit
[issue: prompt for quit]
[issue: prompt for save][feature:persistence]

.resume [issue: implement timer ][feature: timer]
.pause [feature: timer]

.status - print board, current turn, last move
[issue:extend status to print score][feature:board score]

.move a1a2 - make a move

.moves.list - prints list of legal moves
[issue:moves list][feature:moves list]

.move.random - make a random legal move
[feature:moves list]
[issue:random move][feature:random move]

.moves.history - prints list of moves
[issue: history]

.move.undo - undo last move
[feature:history]
[issue:undo move][feature:undo move]

.gg - current player forfeits
[issue:forfeit][feature:forfeit]

.online.new
.online.list
.online.join
[feature: basic multiplayer]
[issue: multiplayer]

.online.spectate
[feature: basic multiplayer]
[feature: spectating]
[issue: spectating]

.online.msg
[feature: basic multiplayer]
[feature: chatting]
[issue: chatting]

*/

/*

Commands minimal

.game.new - creates game with default board
.quit - exit
.status - print board, current turn, last move
.move a1a2 - make a move

*/

pub struct Board
{
  pleco_board : pleco::Board
}

impl Board
{
  pub fn default() -> Self
  {
    Self
    {
      pleco_board : pleco::Board::start_pos()
    }
  }

  pub fn make_move( &mut self, uci_move : &str ) -> Option< Self >
  {
    let mut pleco_board : pleco::Board = self.pleco_board.clone();
    let result = pleco_board.apply_uci_move( &uci_move );
    if result
    {
      Some( Self { pleco_board } )
    }
    else
    {
      None
    }
  }

  pub fn move_is_valid( &self, uci_move : &str ) -> bool
  {
    match self.move_from_uci( uci_move )
    {
      Some( m ) => self.pleco_board.pseudo_legal_move( m ) && self.pleco_board.legal_move( m ),
      _ => false
    }
  }

  pub fn move_from_uci( &self, uci_move : &str ) -> Option< Move >
  {
    let all_moves: MoveList = self.pleco_board.generate_moves();
    all_moves.iter()
              .find(| m | m.stringify() == uci_move )
              .cloned()
  }

  pub fn print( &self )
  {
    self.pleco_board.pretty_print();
  }

  pub fn fen( &self ) -> Fen
  {
    self.pleco_board.fen()
  }
}

pub type Fen = String;

/// Interface for playing chess game
/// ```
/// let game = Game::default();
/// ```
pub struct Game
{
  board : Board,
  history : Vec<Fen>
}

/// Status of the game
#[derive( Debug )]
pub enum GameStatus
{
  Continuing,
  Checkmate,
  Stalemate
}

impl Game
{
  pub fn default() -> Self
  {
    Self
    {
      board : Board::default(),
      history : Vec::new(),
    }
  }

  pub fn make_move( &mut self, mv: &str ) -> bool
  {
    let new_board = self.board.make_move( mv );
    let success = !new_board.is_none();
    if success
    {
      self.board = new_board.unwrap();
      self.history.push( self.board.fen() );
    }
    success
  }

  // pub fn status( &self ) -> GameStatus
  // {
  //   if self.board.checkmate()
  //   {
  //     return GameStatus::Checkmate;
  //   }

  //   if self.board.stalemate()
  //   {
  //     return GameStatus::Stalemate
  //   }

  //   if !self.board.is_ok_quick()
  //   {
  //     return match self.board.is_okay()
  //     {
  //       Ok( _ ) => panic!( "Unexpected behavior: board.is_ok_quick() returned false, but board.is_okay() says that board is ok" ),
  //       Err( err ) => GameStatus::Error( err )
  //     }
  //   }

  //   return GameStatus::Continuing
  // }

  // pub fn apply_move_u8( &mut self, src : u8, dst : u8 ) -> bool
  // {
  //   let mv = BitMove::make_quiet( SQ( src ), SQ( dst ) );
  //   self.apply_move( mv )
  // }

  // pub fn apply_move_sq( &mut self, src : SQ, dst : SQ ) -> bool
  // {
  //   let mv = BitMove::make_quiet( src, dst );
  //   self.apply_move( mv )
  // }

  // pub fn apply_move( &mut self, mv : BitMove ) -> bool
  // {
  //   let result = self.move_is_valid( mv );

  //   if result
  //   {
  //     self.board.apply_move( mv );
  //   }

  //   result
  // }

  // pub fn move_is_valid( &self, mv : BitMove ) -> bool
  // {
  //   self.board.pseudo_legal_move( mv ) && self.board.legal_move( mv )
  // }

  // pub fn current_turn( &self ) -> Player
  // {
  //   self.board.turn()
  // }

  // pub fn print_board( &self )
  // {
  //   self.board.pretty_print();
  // }

  // pub fn print_current_turn( &self )
  // {
  //   self.print_board();
  //   println!( "Next move: {}", self.current_turn() );
  // }
}

/*
cargo test test_game -- --show-output
*/

#[test]
fn test_game()
{
  let mut game = Game::default();

  game.board.print();
  game.make_move( "a2a4" );
  game.board.print();

  // game.print_current_turn();
  // game.apply_move_u8( 8, 16 );
  // game.print_current_turn();
  // game.apply_move_sq( SQ( 49 ), SQ( 41 ) );
  // game.print_current_turn();
  // println!( "Game status: {:?}", game.status() );
}