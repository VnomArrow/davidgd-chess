use std::ptr::read_unaligned;
use std::{collections::HashMap, convert::TryInto};
use std::cmp;

mod movement;

use movement::available_moves_for_piece;
use movement::get_all_attacked_squares;

#[non_exhaustive]
struct TYPES;

impl TYPES {
    pub const NONE: u8 = 0;
    pub const PAWN: u8 = 1;
    pub const KNIGHT: u8 = 2;
    pub const BISHOP: u8 = 4;
    pub const ROOK: u8 = 8;
    pub const QUEEN: u8 = 16;
    pub const KING: u8 = 32;
}

#[non_exhaustive]
struct COLORS;

impl COLORS {
    pub const WHITE: u8 = 64;
    pub const BLACK: u8 = 128;
}

#[derive(Clone)]
pub struct GAME {
    computed_distances: [[u8; 8]; 64],
    board: [u8; 64],
    turn: u8,
    moves: Vec<[u8; 2]>,
    tile_available_to_un_passant: u8,
    potential_tile_to_un_passant: u8,
    chastling_ability: [bool; 4],                               // KQkq
    check: bool,
    draw: bool,
    check_mate: bool,
    promoting: u8,
}

impl GAME {
    fn tiles_to_the_edge() -> [[u8; 8]; 64] {
        let mut precomputed_distances = [[0u8; 8]; 64];
    
        for file in 0..8 {
            for rank in 0..8 {
    
                let tiles_north: u8 = rank;
                let tiles_south: u8 = 7 - rank;
                let tiles_west: u8 = file;
                let tiles_east: u8 = 7 - file;
    
                let tile_index: usize = (rank * 8 + file) as usize;
    
                precomputed_distances[tile_index] = [tiles_north as u8,
                                    tiles_south as u8,
                                    tiles_west as u8,
                                    tiles_east as u8,
                                    cmp::min(tiles_north, tiles_west),
                                    cmp::min(tiles_south, tiles_east),
                                    cmp::min(tiles_north, tiles_east),
                                    cmp::min(tiles_south, tiles_west)];
            }
        }
        precomputed_distances
    }
    
    fn generate_board_array() -> [u8; 64] {
        [0u8; 64]
    }

    pub fn get_board(&self) -> [u8; 64] {
        self.board
    }

    pub fn get_played_moves(&self) -> &Vec<[u8; 2]> {
        &self.moves
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn is_check_mate(&self) -> bool {
        self.check_mate
    }

    pub fn is_draw(&self) -> bool {
        self.draw
    }

    pub fn is_whites_turn(&self) -> bool {
        if self.turn & COLORS::WHITE > 0 {
            return true
        } else {
            false
        }
    }

    pub fn get_game_status(&self) -> (bool, bool, bool, bool) {
        (self.is_whites_turn(), self.is_check(), self.is_draw(), self.is_check_mate())
    }
}

pub fn promote_pawn(new_type: &str, game: &mut GAME) -> bool {
    let mut if_valid_move = false;
    if game.promoting <= 63 {
        if_valid_move = true;
        let promotiong_piece_color: u8;
        let tile_promoting: usize = game.promoting as usize;

        if (game.turn & COLORS::WHITE) > 0 {
            promotiong_piece_color = COLORS::BLACK;
        } else {
            promotiong_piece_color = COLORS::WHITE;
        }

        if new_type.to_string() == "q" {
            game.board[tile_promoting] = TYPES::QUEEN + promotiong_piece_color;
            game.promoting = 100;
        } else 
        if new_type.to_string() == "r" {
            game.board[tile_promoting] = TYPES::ROOK + promotiong_piece_color;
            game.promoting = 100;
        } else 
        if new_type.to_string() == "b" {
            game.board[tile_promoting] = TYPES::BISHOP + promotiong_piece_color;
            game.promoting = 100;
        } else 
        if new_type.to_string() == "k" {
            game.board[tile_promoting] = TYPES::KNIGHT + promotiong_piece_color;
            game.promoting = 100;
        } else {
            if_valid_move = false;
        } 
    }
    if_valid_move
}

pub fn move_piece_from_to(from_tile: &str, to_tile: &str, game: &mut GAME) -> bool {
    println!("gameturn: {}", game.turn);
    let from_tile = algebraic_notation_to_memory_location(from_tile);
    let to_tile = algebraic_notation_to_memory_location(to_tile);
    println!("{}", from_tile);
    println!("{}", to_tile);
    let piece_to_move = game.board[from_tile];
    let mut if_valid_move = false;

    println!("piece: {}", piece_to_move);

    if game.promoting == 100 {
        if_valid_move = true;
    }
    
    if_valid_move = piece_is_correct_color(game, piece_to_move, if_valid_move);

    println!("correct color: {}", if_valid_move);

    if_valid_move = is_legal_move_for_piece(game, piece_to_move, from_tile, to_tile, if_valid_move);

    println!("valid move: {}", if_valid_move);

    if if_valid_move {
        handle_un_passant_logic(game, piece_to_move, from_tile, to_tile);
        handle_promote_logic(game, piece_to_move, to_tile);

        let mut game_clone = game.clone();
        move_the_piece(&mut game_clone, piece_to_move, from_tile, to_tile);
        
        check_if_allied_king_is_checked(&mut game_clone, game);

        if game.check {
            if_valid_move = false;
        } else {
            if_valid_move = true;
        }

        if if_valid_move {
        
            move_the_piece(game, piece_to_move, from_tile, to_tile);

            update_chastling_ability(game, from_tile, to_tile);

            check_if_enemy_king_is_checked(game);

            swap_turn(game);
            if_valid_move = true;
        }
    }
    if_valid_move
}

fn update_chastling_ability(game: &mut GAME, from_tile: usize, to_tile: usize) {
    if from_tile == 63 || to_tile == 63 {
        game.chastling_ability[0] = false;
    } 
    if from_tile == 56 || to_tile == 56 {
        game.chastling_ability[1] = false;
    } 
    if from_tile == 60 || to_tile == 60 {
        game.chastling_ability[0] = false;
        game.chastling_ability[1] = false;
    } 
    if from_tile == 7 || to_tile == 7 {
        game.chastling_ability[2] = false;
    }
    if from_tile == 0 || to_tile == 0 {
        game.chastling_ability[3] = false;
    }
    if from_tile == 4 || to_tile == 4 {
        game.chastling_ability[2] = false;
        game.chastling_ability[3] = false;
    }
}

fn piece_is_correct_color(game: &mut GAME, piece_to_move: u8, mut if_valid_move: bool) -> bool {
    if if_valid_move {
        if ((game.turn & COLORS::WHITE) > 0) & ((piece_to_move & COLORS::WHITE) > 0) {
            if_valid_move = true;
        } else if ((game.turn & COLORS::BLACK) > 0) & ((piece_to_move & COLORS::BLACK) > 0) {
            if_valid_move = true;
        } else {
            if_valid_move = false;
        }
    }
    if_valid_move
}

fn is_legal_move_for_piece(game: &mut GAME, piece_to_move: u8, from_tile: usize, to_tile: usize, mut if_valid_move: bool) -> bool {
    let available_moves_for_piece = available_moves_for_piece(piece_to_move, from_tile, game);

    if available_moves_for_piece[to_tile] & if_valid_move {
        if_valid_move = true;
    } else {
        if_valid_move = false;
    }
    if_valid_move
}

fn handle_un_passant_logic(game: &mut GAME, piece_to_move: u8, from_tile: usize, to_tile: usize) {
    if piece_to_move & TYPES::PAWN > 0 {
        if (from_tile >= 16 && from_tile <= 23) || (from_tile >= 40 && from_tile <= 47) {
            if piece_to_move & COLORS::WHITE > 0 {
                if to_tile == from_tile - 7 || to_tile == from_tile - 9 {
                    game.board[(game.tile_available_to_un_passant + 8) as usize] = TYPES::NONE;
                }
                if to_tile == (from_tile - 16) as usize {
                    game.tile_available_to_un_passant = game.potential_tile_to_un_passant;
                }
            } else if piece_to_move & COLORS::BLACK > 0 {
                if to_tile == from_tile + 7 || to_tile == from_tile + 9 {
                    game.board[(game.tile_available_to_un_passant - 8) as usize] = TYPES::NONE;
                }
                if to_tile == (from_tile + 16) as usize {
                    game.tile_available_to_un_passant = game.potential_tile_to_un_passant;
                }
            }
        }
    } else {
        game.tile_available_to_un_passant = 100;
    } 
}

fn handle_promote_logic(game: &mut GAME, piece_to_move: u8, to_tile: usize) {
    if piece_to_move & TYPES::PAWN > 0 {
        if piece_to_move & COLORS::WHITE > 0 {
            if to_tile <= 7 {
                game.promoting = to_tile as u8;
            }
        } else if piece_to_move & COLORS::BLACK > 0 {
            if to_tile >= 56 && to_tile <= 63 {
                game.promoting = to_tile as u8;
            }
        }
    } else {
        game.promoting = 100;
    }
}

fn check_if_allied_king_is_checked(game_clone: &mut GAME, game: &mut GAME) {
    let board = game_clone.board;
    let turn = game.turn;
    let color_is_playing;
    let color_king_checkable;

    if turn == COLORS::WHITE {
        color_is_playing = COLORS::BLACK;
        color_king_checkable = COLORS::WHITE;
    } else {
        color_is_playing = COLORS::WHITE;
        color_king_checkable = COLORS::BLACK;
    }

    let attacked_tiles = get_all_attacked_squares(color_is_playing, game_clone);

    for (tile, piece) in board.iter().enumerate() {
        if (piece & color_king_checkable > 0) && (piece & TYPES::KING > 0) {
            if attacked_tiles[tile] {
                game.check = true;
            } else {
                game.check = false;
            }
        }
    }
}

fn check_if_enemy_king_is_checked(game: &mut GAME) {
    let board = game.board;
    let turn = game.turn;
    let color_is_playing;
    let color_king_checkable;

    if turn == COLORS::BLACK {
        color_is_playing = COLORS::BLACK;
        color_king_checkable = COLORS::WHITE;
    } else {
        color_is_playing = COLORS::WHITE;
        color_king_checkable = COLORS::BLACK;
    }

    let attacked_tiles = get_all_attacked_squares(color_is_playing, game);

    for (tile, piece) in board.iter().enumerate() {
        if (piece & color_king_checkable > 0) && (piece & TYPES::KING > 0) {
            if attacked_tiles[tile] {
                game.check = true;
            } else {
                game.check = false;
            }
        }
    }
}

fn move_the_piece(game: &mut GAME, piece_to_move: u8, from_tile: usize, to_tile: usize) {
    if to_tile == 62 && (game.board[from_tile] & TYPES::KING > 0) && (game.board[to_tile] & TYPES::ROOK > 0) {          // King-side white chastling
        game.board[60] = TYPES::NONE;
        game.board[61] = TYPES::ROOK + COLORS::WHITE;
        game.board[62] = piece_to_move;
        game.board[63] = TYPES::NONE;
    } else {
        game.board[from_tile] = TYPES::NONE;
        game.board[to_tile] = piece_to_move;
    }
}

fn swap_turn(game: &mut GAME) {
    if game.turn == COLORS::WHITE {
        game.turn = COLORS::BLACK;
    } else {
        game.turn = COLORS::WHITE;
    }
}



pub fn init_game() -> GAME {
    let mut game = GAME {
        computed_distances: GAME::tiles_to_the_edge(),
        board: GAME::generate_board_array(),
        turn: COLORS::WHITE,
        moves: Vec::new(),
        tile_available_to_un_passant: 100,
        potential_tile_to_un_passant: 100,
        chastling_ability: [false, false, false, false],                    // KQkq
        check: false,
        draw: false,
        check_mate: false,
        promoting: 100,
    };
    let mut piece_type_from_symbol = HashMap::new();

    piece_type_from_symbol.insert('k', TYPES::KING);
    piece_type_from_symbol.insert('p', TYPES::PAWN);
    piece_type_from_symbol.insert('n', TYPES::KNIGHT);
    piece_type_from_symbol.insert('b', TYPES::BISHOP);
    piece_type_from_symbol.insert('r', TYPES::ROOK);
    piece_type_from_symbol.insert('q', TYPES::QUEEN);

    let (loaded_board, un_passant_default) = load_position_from_fen(STARTINGFEN, &mut game, &mut piece_type_from_symbol);
    game.board = loaded_board;
    game.tile_available_to_un_passant = un_passant_default;
    game
}

// const STARTINGFEN: &str = "rnbqkbnr/pppppppp/8/6P/6p/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
// const STARTINGFEN: &str = "rnbqkbnr/8/8/6P/6p/8/8/RNBQKBNR w KQkq - 0 1";
// const STARTINGFEN: &str = "r3k2r/8/8/8/8/7P/8/R3K2R w KQkq - 0 1";
// const STARTINGFEN: &str = "8/4PP/8/8/8/7P/5pp/8 w KQkq - 0 1";
const STARTINGFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// # Testing FEN algorithm
///```
/// use chess_logic::*;
/// use std::{collections::HashMap, convert::TryInto};
/// let mut piece_type_from_symbol = HashMap::new();
/// piece_type_from_symbol.insert('k', 32);
/// piece_type_from_symbol.insert('p', 1);
/// piece_type_from_symbol.insert('n', 2);
/// piece_type_from_symbol.insert('b', 4);
/// piece_type_from_symbol.insert('r', 8);
/// piece_type_from_symbol.insert('q', 16);
/// let empty_board = [0u8; 64];
/// let STARTINGFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let board = chess_logic::load_position_from_fen(STARTINGFEN, empty_board, &mut piece_type_from_symbol);
/// let expected_output = [136, 130, 132, 144, 160, 132, 130, 136, 129, 129, 129, 129, 129, 129, 129, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 65, 65, 65, 65, 65, 65, 65, 72, 66, 68, 80, 96, 68, 66, 72];
/// assert_eq!(board, expected_output);
///```

pub fn load_position_from_fen(fen: &str, game: &mut GAME, piece_type_from_symbol: &mut HashMap<char, u8>) ->  ([u8; 64], u8) {

    let mut board = game.board;
    let mut tile_available_to_un_passant: u8 = game.tile_available_to_un_passant;

    let mut fen_parts = fen.split_whitespace();

    let mut empty = false;
    let mut parts_index = 0;

    let mut positions: &str = "";
    let mut turn: &str = "";
    let mut castling_ability: &str = "";
    let mut moved_on_to_by_un_passant: &str = "";
    let mut halfmove: &str = "";
    let mut fullmove: &str = "";

    while !empty {
        let part = fen_parts.next();
        if part == None {
            empty = true;
        } else {
            if parts_index == 0 {
                positions = part.unwrap();
            } else if parts_index == 1 {
                turn = part.unwrap();
                if turn == ('w' as char).to_string() {
                    game.turn = COLORS::WHITE;
                } else if turn == ('b' as char).to_string() {
                    game.turn = COLORS::BLACK;
                }
            } else if parts_index == 2 {
                castling_ability = part.unwrap();
                for char in castling_ability.chars() {
                    if char == 'K' {
                        game.chastling_ability[0] = true;
                    } else if char == 'Q' {
                        game.chastling_ability[1] = true;
                    } else if char == 'k' {
                        game.chastling_ability[2] = true;
                    } else if char == 'q' {
                        game.chastling_ability[3] = true;
                    }
                }
            } else if parts_index == 3 {
                moved_on_to_by_un_passant = part.unwrap();
                if moved_on_to_by_un_passant.contains('-') {
                    tile_available_to_un_passant = 63;
                } else {
                    tile_available_to_un_passant = moved_on_to_by_un_passant.parse::<u8>().unwrap();
                }
            } else if parts_index == 4 {
                halfmove = part.unwrap();
            } else {
                fullmove = part.unwrap();
            }
        }
        parts_index += 1;
    }

    let mut file = 0;
    let mut rank = 0;

    for character in positions.chars() {
        if character == '/'{
            file = 0;
            rank += 1;
        } else {
            if character.is_digit(10) {
                file += character.to_digit(10).unwrap();
            } else {
                let piece_color = if character.is_uppercase() { COLORS::WHITE } else { COLORS::BLACK};
                let lowercase_char = &character.to_lowercase().collect::<Vec<_>>()[0];
                let piece_type = piece_type_from_symbol.get(lowercase_char);
                let square: usize = (rank * 8 + file).try_into().unwrap();
                board[square] = piece_type.unwrap() + piece_color;
                file += 1;
            }
        }
    }
    (board, tile_available_to_un_passant)
}

pub fn algebraic_notation_to_memory_location(algebraic_notation: &str) -> usize {
    let alphabet_to_index = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut rank: usize = 0;
    let mut file: usize = 0;
    for character in algebraic_notation.chars() {
        if character.is_alphabetic() {
            let lowercase_char = character.to_lowercase().collect::<Vec<_>>()[0];
            file = (alphabet_to_index.iter().position(|&r| r == lowercase_char).unwrap()) as usize;
        }
        if character.is_digit(10) {
            let int_rank = character.to_digit(10).unwrap() as i32;
            rank = (8 - int_rank) as usize;
        }
    }
    (rank * 8 + file) as usize
}


pub fn is_black_king(piece: u8) -> bool {
    if (piece & TYPES::KING > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_queen(piece: u8) -> bool {
    if (piece & TYPES::QUEEN > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_rook(piece: u8) -> bool {
    if (piece & TYPES::ROOK > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_bishop(piece: u8) -> bool {
    if (piece & TYPES::BISHOP > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_knight(piece: u8) -> bool {
    if (piece & TYPES::KNIGHT > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_pawn(piece: u8) -> bool {
    if (piece & TYPES::PAWN > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_king(piece: u8) -> bool {
    if (piece & TYPES::KING > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_queen(piece: u8) -> bool {
    if (piece & TYPES::QUEEN > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_rook(piece: u8) -> bool {
    if (piece & TYPES::ROOK > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_bishop(piece: u8) -> bool {
    if (piece & TYPES::BISHOP > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_knight(piece: u8) -> bool {
    if (piece & TYPES::KNIGHT > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_pawn(piece: u8) -> bool {
    if (piece & TYPES::PAWN > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}