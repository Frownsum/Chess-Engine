// use std::mem::{variant_count}; // from nightly, but don't feel like setting this up right now.

use std::convert::TryFrom;
use crate::board;

/// The primary trait used to represent current board state
pub trait Board {
    /// Set the piece for a tile
    fn set_tile(&mut self, coord: Coordinate, piece: Piece);

    /// clear_tile
    fn clear_tile(&mut self, coord: Coordinate);

    /// get the available legal moves
    fn get_moves(&self) -> Vec<Move>;
}

/// Kinds of available moves
#[repr(u8)]
pub enum MoveKind {
    QuietMove,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EPCapture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
}

/// Information for a move
pub struct Move {
    kind: MoveKind,

    origin: Coordinate,
    target: Coordinate,

    piece: Piece, //todo: use both halves of a u8 to store these 2 instead?
    capture: Piece,
}

/// Errors for the board
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfBoundsAxis,
    OutOfBoundsIndex,
}

/// The teams that are playing a game of chess
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Team { White = 0, Black = 1 }

/// bit mask to get the team bit
const MASK_TEAM: u8 = 0b1000;

/// Amount to shift a bit to check for team
const SHIFT_TEAM: u8 = 3;

/// Number of teams
const NUM_TEAMS: usize = 2;
// const NUM_TEAMS: usize = variant_count::<Team>();

/// The kinds of valid pieces on the chess board
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Chessman { King = 0, Queen = 1, Bishop = 2, Knight = 3, Rook = 4, Pawn = 5 }

/// Bit mask for the chessman
const MASK_CHESSMAN: u8 = 0b111;

/// Number of kinds of pieces
const NUM_CHESSMEN: usize = 6;
// const NUM_PIECE_KINDS: usize = variant_count::<Team>();

/// struct to represent the piece information
pub struct Piece {
    value: u8,
}

impl Piece {
    const MASK_UNOCCUPIED: u8 = 0b11110000;

    /// Creates a new piece
    pub fn new(value: Option<(Team, Chessman)>) -> Self {
        match value {
            Some((team, chessman)) => Piece { value: (((team as u8) << SHIFT_TEAM) | (chessman as u8)) },
            None => Piece { value: Piece::MASK_UNOCCUPIED },
        }
    }

    /// gets the data held by the struct
    pub fn data(&self) -> Option<(Team, Chessman)> {
        if self.value & Piece::MASK_UNOCCUPIED != 0 {
            None
        } else {
            let team = if self.value & MASK_TEAM == 0 {
                Team::White
            } else {
                Team::Black
            };

            let chessman = match MASK_CHESSMAN & self.value {
                0 => Chessman::King,
                1 => Chessman::Queen,
                2 => Chessman::Bishop,
                3 => Chessman::Knight,
                4 => Chessman::Rook,
                5 => Chessman::Pawn,
                _ => panic!("Invalid Chessman found.")
            };

            Some((team, chessman))
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece { value: 0 }
    }
}

/// represents coordinates, should only ever be 0 <= value < 64
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    value: u8,
}

impl Coordinate {
    /// gets the rank of the coordinate
    fn rank(&self) -> u8 {
        self.value / BOARD_LENGTH as u8
    }

    /// gets the file of the coordinate
    fn file(&self) -> u8 {
        self.value % BOARD_LENGTH as u8
    }
}

/// the length of the chess board
pub const BOARD_LENGTH: usize = 8;

/// the number of tiles in the chess board
pub const NUM_TILES: usize = BOARD_LENGTH * BOARD_LENGTH;

impl TryFrom<(u8, u8)> for Coordinate {
    type Error = Error;

    fn try_from(rank_file: (u8, u8)) -> Result<Self, Self::Error> {
        match rank_file {
            (rank, file) if rank < 8 && file < 8 => Ok(Coordinate { value: rank * BOARD_LENGTH as u8 + file }),
            _ => Err(Error::OutOfBoundsAxis),
        }
    }
}

impl TryFrom<u8> for Coordinate {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < NUM_TILES as u8 {
            Ok(Coordinate { value })
        } else {
            Err(Error::OutOfBoundsIndex)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece() {
        for team in [Team::White, Team::Black] {
            for chessman in [
                Chessman::King,
                Chessman::Pawn,
                Chessman::Knight,
                Chessman::Bishop,
                Chessman::Queen,
                Chessman::Rook] {
                let piece = Piece::new(Some((team, chessman)));

                assert_eq!(piece.data(), Some((team, chessman)));
            }
        }

        let piece = Piece::new(None);
        assert_eq!(piece.data(), None);
    }

    #[test]
    fn test_coordinate() {
        for file in 0..=BOARD_LENGTH {
            for rank in 0..=BOARD_LENGTH {
                let file = file as u8;
                let rank = rank as u8;

                let test_coord = Coordinate::try_from((rank, file));

                if file < BOARD_LENGTH as u8 && rank < BOARD_LENGTH as u8 {
                    let test_coord = test_coord.expect("try_from failed");
                    assert_eq!(test_coord.rank(), rank);
                    assert_eq!(test_coord.file(), file);
                } else {
                    assert_eq!(test_coord, Err(Error::OutOfBoundsAxis));
                }

                let index = (rank * BOARD_LENGTH as u8) | file;
                let test_coord = Coordinate::try_from(index);

                if index < NUM_TILES as u8 {
                    let test_coord = test_coord.expect("try_from failed");
                    // just comparing to rank/file doesnt work because adding too high file to not too big rank might not be out of range
                    assert_eq!(test_coord.rank(), index / BOARD_LENGTH as u8);
                    assert_eq!(test_coord.file(), index % BOARD_LENGTH as u8);
                } else {
                    assert_eq!(test_coord, Err(Error::OutOfBoundsIndex))
                }
            }
        }
    }
}




