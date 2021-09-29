use crate::COLORS;
use crate::TYPES;
use crate::GAME;

pub fn king_attacks_from_tile(game: &mut GAME, piece: u8, tile: usize) -> [bool; 64] {
    let precomputed_distances = game.computed_distances;
    let board = game.board;
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let target_tile = tile as i8 + offset;
        let distances_to_edge = precomputed_distances[tile];
        if distances_to_edge[index] > 0 {
            if board[target_tile as usize] & piece_color > 0 {
                continue;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

pub fn king_movement_from_tile(game: &mut GAME, piece: u8, tile: usize) -> [bool; 64] {
    let precomputed_distances = game.computed_distances;
    let board = game.board;
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let target_tile = tile as i8 + offset;
        let distances_to_edge = precomputed_distances[tile];
        if distances_to_edge[index] > 0 {
            if board[target_tile as usize] & piece_color > 0 {
                continue;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }

    if game.chastling_ability[0] {              // King-side white chastling
        let attacked_tiles = get_all_attacked_squares(enemy_color, game);
        if !attacked_tiles[60] && !attacked_tiles[61] && !attacked_tiles[62] {
            if board[61] == 0 && board[62] == 0 {
                available_moves_board[62] = true;
            }
        }
    }
    if game.chastling_ability[1] {              // Queen-side white chastling
        let attacked_tiles = get_all_attacked_squares(enemy_color, game);
        if !attacked_tiles[60] && !attacked_tiles[59] && !attacked_tiles[58] {
            if board[59] == 0 && board[58] == 0 && board[57] == 0 {
                available_moves_board[58] = true;
            }
        }
    }
    if game.chastling_ability[2] {              // King-side black chastling
        let attacked_tiles = get_all_attacked_squares(enemy_color, game);
        if !attacked_tiles[4] && !attacked_tiles[5] && !attacked_tiles[6] {
            if board[5] == 0 && board[6] == 0 {
                available_moves_board[6] = true;
            }
        }
    }
    if game.chastling_ability[3] {              // Queen-side white chastling
        let attacked_tiles = get_all_attacked_squares(enemy_color, game);
        if !attacked_tiles[2] && !attacked_tiles[3] && !attacked_tiles[4] {
            if board[1] == 0 && board[2] == 0 && board[3] == 0 {
                available_moves_board[2] = true;
            }
        }
    }

    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn queen_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

fn rook_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets[0..4].iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

fn bishop_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets[4..8].iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index + 4] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

fn knight_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
    } else {
        piece_color = COLORS::BLACK;
    }
    let offsets: [i8; 8] = [-15, -6, 10, 17, 15, 6, -10, -17];
    let precomputed_distances_to_edge = [precomputed_distances[tile][0], 
                                                precomputed_distances[tile][3], 
                                                precomputed_distances[tile][1], 
                                                precomputed_distances[tile][2]];
    for (index, offset) in offsets.iter().enumerate() {
        let target_tile = tile as i8 + offset;
        if index == 0 {
            if precomputed_distances_to_edge[0] > 1 && precomputed_distances_to_edge[1] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 1 {
            if precomputed_distances_to_edge[0] > 0 && precomputed_distances_to_edge[1] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 2 {
            if precomputed_distances_to_edge[1] > 1 && precomputed_distances_to_edge[2] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 3 {
            if precomputed_distances_to_edge[1] > 0 && precomputed_distances_to_edge[2] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 4 {
            if precomputed_distances_to_edge[2] > 1 && precomputed_distances_to_edge[3] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 5 {
            if precomputed_distances_to_edge[2] > 0 && precomputed_distances_to_edge[3] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 6 {
            if precomputed_distances_to_edge[3] > 1 && precomputed_distances_to_edge[0] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 7 {
            if precomputed_distances_to_edge[3] > 0 && precomputed_distances_to_edge[0] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

fn pawn_movement_from_tile(game: &mut GAME, piece: u8, tile: usize) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let precomputed_distances = game.computed_distances;
    let un_passant_tile = game.tile_available_to_un_passant;
    let board = game.board;
    let enemy_piece_color: u8;
    let offsets: [i8; 6] = [-9, -8, -7, 9, 8, 7];
    let precomputed_distances_to_edge = [precomputed_distances[tile][4], 
                                                precomputed_distances[tile][0], 
                                                precomputed_distances[tile][6],
                                                precomputed_distances[tile][5],
                                                precomputed_distances[tile][1],
                                                precomputed_distances[tile][7]];
    if piece & COLORS::WHITE > 0 {
        enemy_piece_color = COLORS::BLACK;
        for (index, offset) in offsets[0..3].iter().enumerate() {
            let target_tile: i8 = tile as i8 + offset;
            if index == 0 {
                if precomputed_distances_to_edge[index] > 0 {       // tile diagonaly left from white pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            } else if index == 1 {
                if precomputed_distances_to_edge[index] > 0 {       // tiles in front of white pawn
                    if tile > 47 && tile < 56 {
                        if board[(target_tile - 8) as usize] == 0 && board[target_tile as usize] == 0 {
                            available_moves_board[(target_tile) as usize] = true;
                            available_moves_board[(target_tile - 8) as usize] = true;
                            game.potential_tile_to_un_passant = target_tile as u8;
                        } else if board[target_tile as usize] == 0 {
                            available_moves_board[target_tile as usize] = true;
                        } else {
                            continue;
                        }
                    } else {
                        if board[target_tile as usize] == 0 {
                            available_moves_board[target_tile as usize] = true;
                        } else {
                            continue;
                        }
                    }
                }
            } else if index == 2 {
                if precomputed_distances_to_edge[index] > 0 {       // tile diagonaly right from white pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            }
        }
    } else {
        enemy_piece_color = COLORS::WHITE;
        for (index, offset) in offsets[3..6].iter().enumerate() {
            let target_tile: i8 = tile as i8 + offset;
            if index == 0 {
                if precomputed_distances_to_edge[index + 3] > 0 {       // tile diagonaly left from black pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            } else if index == 1 {
                if precomputed_distances_to_edge[index + 3] > 0 {       // tiles in front of black pawn
                    if tile > 7 && tile < 16 {
                        if board[(target_tile + 8) as usize] == 0 && board[target_tile as usize] == 0 {
                            available_moves_board[(target_tile) as usize] = true;
                            available_moves_board[(target_tile + 8) as usize] = true;
                            game.potential_tile_to_un_passant = target_tile as u8;
                        } else if board[target_tile as usize] == 0 {
                            available_moves_board[target_tile as usize] = true;
                        } else {
                            continue;
                        }
                    } else {
                        if board[target_tile as usize] == 0 {
                            available_moves_board[target_tile as usize] = true;
                        } else {
                            continue;
                        }
                    }
                }
            } else if index == 2 {
                if precomputed_distances_to_edge[index + 3] > 0 {       // tile diagonaly right from black pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}


pub fn available_moves_for_piece(piece_to_move: u8, from_tile: usize, game: &mut GAME) -> [bool; 64] {
    let mut moves = [false; 64];
    if (piece_to_move & TYPES::KING) > 0 {
        moves = king_movement_from_tile(game, piece_to_move, from_tile);
    } else if (piece_to_move & TYPES::QUEEN) > 0 {
        moves = queen_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::ROOK) > 0 {
        moves = rook_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::BISHOP) > 0 {
        moves = bishop_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::KNIGHT) > 0 {
        moves = knight_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::PAWN) > 0 {
        moves = pawn_movement_from_tile(game, piece_to_move, from_tile);
    }
    moves
}


fn pawn_attack_from_tile(game: &mut GAME, piece: u8, tile: usize) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let precomputed_distances = game.computed_distances;
    let un_passant_tile = game.tile_available_to_un_passant;
    let board = game.board;
    let enemy_piece_color: u8;
    let offsets: [i8; 4] = [-9, -7, 9, 7];
    let precomputed_distances_to_edge = [precomputed_distances[tile][4], 
                                                precomputed_distances[tile][6],
                                                precomputed_distances[tile][5],
                                                precomputed_distances[tile][7]];
    if piece & COLORS::WHITE > 0 {
        enemy_piece_color = COLORS::BLACK;
        for (index, offset) in offsets[0..2].iter().enumerate() {
            let target_tile: i8 = tile as i8 + offset;
            if index == 0 {
                if precomputed_distances_to_edge[index] > 0 {       // tile diagonaly left from white pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile || board[target_tile as usize] == 0 {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            } else if index == 1 {
                if precomputed_distances_to_edge[index] > 0 {       // tile diagonaly right from white pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile || board[target_tile as usize] == 0 {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            }
        }
    } else {
        enemy_piece_color = COLORS::WHITE;
        for (index, offset) in offsets[2..4].iter().enumerate() {
            let target_tile: i8 = tile as i8 + offset;
            if index == 0 {
                if precomputed_distances_to_edge[index + 2] > 0 {       // tile diagonaly left from black pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile || board[target_tile as usize] == 0 {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            } else if index == 1 {
                if precomputed_distances_to_edge[index + 2] > 0 {       // tile diagonaly right from black pawn
                    if (board[target_tile as usize] & enemy_piece_color > 0) || target_tile as u8 == un_passant_tile || board[target_tile as usize] == 0 {
                        available_moves_board[target_tile as usize] = true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    // draw_movement_board(available_moves_board);
    return available_moves_board
}

pub fn available_attacks_for_piece(piece_to_move: u8, from_tile: usize, game: &mut GAME) -> [bool; 64] {
    let mut moves = [false; 64];
    if (piece_to_move & TYPES::KING) > 0 {
        moves = king_attacks_from_tile(game, piece_to_move, from_tile);
    } else if (piece_to_move & TYPES::QUEEN) > 0 {
        moves = queen_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::ROOK) > 0 {
        moves = rook_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::BISHOP) > 0 {
        moves = bishop_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::KNIGHT) > 0 {
        moves = knight_movement_from_tile(game.board, piece_to_move, from_tile, game.computed_distances);
    } else if (piece_to_move & TYPES::PAWN) > 0 {
        moves = pawn_attack_from_tile(game, piece_to_move, from_tile);
    }
    moves
}

pub fn get_all_attacked_squares(enemy_color: u8, game:&mut GAME) -> [bool; 64] {
    let mut attacked_tiles = [false; 64];
    let board: [u8; 64] = game.board.clone();
    for (tile, piece) in board.iter().enumerate() {
        if enemy_color & piece > 0 {
            let piece_attacks_these_tiles = available_attacks_for_piece(*piece, tile, game);

            for tile in 0..board.len() {
                if attacked_tiles[tile as usize] || piece_attacks_these_tiles[tile as usize] {
                    attacked_tiles[tile as usize] = true;
                } else {
                    attacked_tiles[tile as usize] = false;
                }
            }
        }
    }
    // draw_movement_board(attacked_tiles);
    return attacked_tiles
}


fn draw_movement_board(board: [bool; 64]) {
    let mut rank  = 1;
    for available in board.iter() {
        if *available {
            print!("|{} ", available);
        } else if !available {
            print!("|{}", available);
        }
        if rank % 8 == 0 {
            print!("|{}", "\n");
        }
        rank += 1;
    }
}
