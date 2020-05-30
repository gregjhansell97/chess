use serde::{Deserialize, Serialize};
use std::collections::HashSet; 

#[derive(Deserialize, Serialize)]
pub struct Board { 
    pieces: [[Option<piece::Piece>; 8]; 8]
}

impl Board {
    fn new() -> Board { 
        use piece::*;

        let mut b = Board {
            pieces: [[None; 8]; 8]
        };
        // assign pieces
        for &(r, t) in [(0, Team::Black), (7, Team::White)].iter() {
            b.pieces[r][0] =  Some(Piece::new(Shape::Castle, t));
            b.pieces[r][1] =  Some(Piece::new(Shape::Knight, t));
            b.pieces[r][2] =  Some(Piece::new(Shape::Bishop, t));
            b.pieces[r][3] =  Some(Piece::new(Shape::Queen, t));
            b.pieces[r][4] =  Some(Piece::new(Shape::King, t));
            b.pieces[r][5] =  Some(Piece::new(Shape::Bishop, t));
            b.pieces[r][6] =  Some(Piece::new(Shape::Knight, t));
            b.pieces[r][7] =  Some(Piece::new(Shape::Castle, t));
            let r = if r == 0 { r + 1 } else { r - 1 };
            // assign pawns
            for c in 0..8 {
                b.pieces[r][c] = Some(Piece::new(Shape::Pawn, t));
            }
        }
        return b;
    }
    pub fn move_piece(&mut self, old: piece::Loc, new: piece::Loc) -> bool {
        let mut current = self.get(old).unwrap(); 

        match self.pieces[new.row()][new.col()] {
            Some(next) => {
                if next.team() == current.team() { return false; }
            },
            None => {},
        }; 

        let valid_moves = match current.shape() {
            piece::Shape::Castle => self.valid_castle_moves(old),
            piece::Shape::Knight => self.valid_knight_moves(old),
            piece::Shape::Bishop => self.valid_bishop_moves(old),
            piece::Shape::Queen => self.valid_queen_moves(old),
            piece::Shape::King => self.valid_king_moves(old),
            piece::Shape::Pawn => self.valid_pawn_moves(old),
        };

        if valid_moves.contains(&new) {
            let backup = self.pieces;
            if let piece::Shape::King = current.shape() {
                // look for king castle switch 
                if !current.moved() {
                    if old.col() + 2 == new.col() {
                        // right-side king, castle switch
                        let mut castle = self.pieces[new.row()][7].unwrap();
                        castle.mark_as_moved();
                        self.pieces[new.row()][old.col() + 1] = Some(castle);
                        self.pieces[new.row()][7] = None;
                    } else if old.col() - 2 == new.col() {
                        // left-side king, castle switch
                        let mut castle = self.pieces[new.row()][0].unwrap();
                        castle.mark_as_moved();
                        self.pieces[new.row()][old.col() - 1] = Some(castle);
                        self.pieces[new.row()][0] = None;
                    }
                }
            }
            if let piece::Shape::Pawn = current.shape() {
                // check if pawn is at end
                if old.row() + 1 == new.row() { // moved forward
                    if new.row() == 7 { // made it to the end
                        current = piece::Piece::new(
                            piece::Shape::Queen, current.team());
                    }
                } else if old.row() - 1 == new.row() { // moved back
                    if new.row() == 0 { // made it to the end
                        current = piece::Piece::new(
                            piece::Shape::Queen, current.team());
                    }
                }
            }
            current.mark_as_moved();
            self.pieces[new.row()][new.col()] = Some(current);
            self.pieces[old.row()][old.col()] = None;
            if self.in_check(current.team()) {
                self.pieces = backup;
                return false;
            }
            return true;
        } else {
            return false; 
        }
    }

    pub fn in_check(&self, team: Team) -> bool{
        let mut opposing_moves = HashSet::new();
        let mut king_loc = piece::Loc::new(0, 0);
        let mut r = 0;
        for row in self.pieces.iter() {
            let mut c = 0;
            for p in row.iter() {
                if let None = p {
                    c += 1;
                    continue;
                }
                let p = p.unwrap();
                let l = piece::Loc::new(r, c);
                if p.team() != team {
                    opposing_moves.extend(match p.shape() {
                        piece::Shape::Castle => self.valid_castle_moves(l),
                        piece::Shape::Knight => self.valid_knight_moves(l),
                        piece::Shape::Bishop => self.valid_bishop_moves(l),
                        piece::Shape::Queen => self.valid_queen_moves(l),
                        piece::Shape::King => self.valid_king_moves(l),
                        piece::Shape::Pawn => self.valid_pawn_moves(l),
                    });
                } else if p.shape() == piece::Shape::King {
                    king_loc = l;
                }
                c += 1;
            }
            r += 1;
        }
        return opposing_moves.contains(&king_loc);
    }
    fn valid_castle_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let castle = self.get(l).unwrap();
        let mut moves = HashSet::new();
        // check verticle down motion
        for r in (l.row() + 1)..8 {
            let l_next = piece::Loc::new(r, l.col());
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != castle.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next);
        }
        // check verticle up motion
        let mut r = l.row();
        while r > 0 {
            r -= 1;
            let l_next = piece::Loc::new(r, l.col());
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != castle.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next);
        }
        // check horizontal right motion
        for c in (l.col() + 1)..8 {
            let l_next = piece::Loc::new(l.row(), c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != castle.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next);
        }
        // check horizontal left motion
        let mut c = l.col();
        while c > 0 {
            c -= 1;
            let l_next = piece::Loc::new(l.row(), c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != castle.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next); 
        }
        return moves;
    }
    fn valid_knight_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let mut moves = HashSet::new();
        let r = l.row() as i32;
        let c = l.col() as i32;
        let potential_moves = [
            (r + 1, c + 2),
            (r - 1, c + 2),
            (r + 1, c - 2),
            (r - 1, c - 2),
            (r + 2, c + 1),
            (r - 2, c + 1),
            (r + 2, c - 1),
            (r - 2, c - 1)];
        for &(r, c) in potential_moves.iter() {
            if r > 7 || r < 0 { continue; }
            if c > 7 || c < 0 { continue; }
            let l_next = piece::Loc::new(r as usize, c as usize);
            moves.insert(l_next); 
        }
        return moves;
    }
    fn valid_bishop_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let bishop = self.get(l).unwrap();
        let mut r = l.row();
        let mut c = l.col();
        let mut moves = HashSet::new();
        // upper right
        while r < 7 && c < 7 {
            r += 1;
            c += 1;
            let l_next = piece::Loc::new(r, c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != bishop.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next); 
        }
        // lower right
        r = l.row();
        c = l.col();
        while r > 0 && c < 7 {
            r -= 1;
            c += 1;
            let l_next = piece::Loc::new(r, c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != bishop.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next); 
        }
        // upper left
        r = l.row();
        c = l.col();
        while r < 7 && c > 0 {
            r += 1;
            c -= 1;
            let l_next = piece::Loc::new(r, c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != bishop.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next); 
        }
        // lower left
        r = l.row();
        c = l.col();
        while r > 0 && c > 0 {
            r -= 1;
            c -= 1;
            let l_next = piece::Loc::new(r, c);
            if !self.is_empty(l_next) { 
                if self.get(l_next).unwrap().team() != bishop.team() {
                    moves.insert(l_next);
                }
                break; 
            }
            moves.insert(l_next); 
        }
        return moves;
    }
    fn valid_queen_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let mut moves = self.valid_castle_moves(l);
        moves.extend(self.valid_bishop_moves(l));
        return moves;
    }
    fn valid_king_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let king = self.get(l).unwrap();
        let mut moves = HashSet::new();
        let row = l.row() as i32;
        let col = l.col() as i32;
        let potential_moves = [
            (row + 1, col + 1),
            (row, col + 1),
            (row - 1, col + 1),
            (row + 1, col),
            (row - 1, col),
            (row + 1, col - 1),
            (row, col - 1),
            (row - 1, col - 1),
        ];
        let row = l.row();
        let col = l.col();
        for &(r, c) in potential_moves.iter() {
            if r > 7 || r < 0 { continue; }
            if c > 7 || c < 0 { continue; }
            let l_next = piece::Loc::new(r as usize, c as usize);
            moves.insert(l_next); 
        }
        if !king.moved() {
            // potential king-castle switch
            // TODO see if in check
            // causes stackoverflow, gonna bend the rules
            // self.in_check(king.team());
            // check if can do right castle
            let mut right_castle = self.get(piece::Loc::new(row, 7));
            for c in (col+1)..7 { 
                if !self.is_empty(piece::Loc::new(row, c)) {
                    right_castle = &None;
                }
            }
            match right_castle {
                Some(castle) => {
                    if !castle.moved() {
                        moves.insert(piece::Loc::new(row, col + 2));
                    }
                },
                None => { }
            };
            // check if can do left castle
            let mut left_castle = self.get(piece::Loc::new(row, 0));
            // king hasn't moved yet, col gonna be greater than 0
            for c in (col-1)..0 { 
                if !self.is_empty(piece::Loc::new(row, c)) {
                    left_castle = &None;
                }
            }
            match left_castle {
                Some(castle) => {
                    if !castle.moved() {
                        moves.insert(piece::Loc::new(row, col - 2));
                    }
                },
                None => { }
            };
        }
        return moves;
    }
    fn valid_pawn_moves(&self, l: piece::Loc) -> HashSet<piece::Loc> {
        let pawn = self.get(l).unwrap();
        let mut moves = HashSet::new();
        let row = l.row() as i32;
        let offset: i32 = match pawn.team() {
            Team::White => -1,
            Team::Black => 1
        };
        let mut l_next = piece::Loc::new((row + offset) as usize, l.col());
        if self.is_empty(l_next) { 
            moves.insert(l_next); 
            // double step? 
            if !pawn.moved() {
                l_next = piece::Loc::new((row + 2*offset) as usize, l.col());
                if self.is_empty(l_next) {
                    moves.insert(l_next);
                }
            }
        }
        // edge checking 
        if l.col() > 0 {
            l_next = piece::Loc::new((row + offset) as usize, l.col() - 1);
            if !self.is_empty(l_next) {
               moves.insert(l_next); 
            }
        }
        if l.col() < 7 {
            l_next = piece::Loc::new((row + offset) as usize, l.col() + 1);
            if !self.is_empty(l_next) {
                moves.insert(l_next);
            }
        }
        return moves;
    }

    pub fn get(&self, l: piece::Loc) -> &Option<piece::Piece> {
        &self.pieces[l.row()][l.col()]
    }

    pub fn is_empty(&self, l: piece::Loc) -> bool {
        match self.pieces[l.row()][l.col()] {
            Some(_) => false,
            None => true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Team {
    White,
    Black,
}

#[derive(Deserialize, Serialize)]
pub struct Game { 
    board: Board,
    turn: Team
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            turn: Team::White
        }
    }
    pub fn move_piece(&mut self, old: piece::Loc, new: piece::Loc) -> bool {
        // verify right team is moving 
        let p = self.board.get(old).unwrap();
        if p.team() != self.turn { return false; }

        if self.board.move_piece(old, new) {
            // successful move
            // change turns
            self.turn = match self.turn {
                Team::White => Team::Black,
                Team::Black => Team::White,
            };
            return true; 
        } else {
            return false;
        }
    }
}

pub mod piece {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct Loc { 
        r: usize,
        c: usize
    }
    impl Loc {
        pub fn new(r: usize, c: usize) -> Loc {
            if r >= 8 || c >= 8 {
                panic!("r or c greater than 7, out of bounds!");
            }
            Loc { r, c }
        }
        pub fn row(&self) -> usize { self.r }
        pub fn col(&self) -> usize { self.c }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    pub enum Shape {
        Castle,
        Knight,
        Bishop,
        Queen,
        King,
        Pawn,
    }

    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct Piece { 
        shape: Shape,
        team: super::Team,
        moved: bool,
    }

    impl Piece {
        pub fn new(shape: Shape, team: super::Team) -> Piece {
            Piece { shape, team, moved: false }
        }    
        // accessors
        pub fn shape(&self) -> Shape { self.shape }
        pub fn team(&self) -> super::Team { self.team }
        pub fn moved(&self) -> bool { self.moved }
        pub fn mark_as_moved(&mut self) { self.moved = true; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "r or c greater than 7, out of bounds!")]
    fn invalid_loc_r() { let l = piece::Loc::new(8, 5); }

    #[test]
    #[should_panic(expected = "r or c greater than 7, out of bounds!")]
    fn invalid_loc_c() { let l = piece::Loc::new(2, 8); }

    #[test]
    fn valid_locs() {
        for r in 0..8 {
            for c in 0..8 {
                let l = piece::Loc::new(r, c); 
            }
        }
    }

    #[test]
    fn initial_game_state() {
        let g = Game::new();
        // list of locations and their expected shapes
        let locs = [
            (piece::Loc::new(0, 0), piece::Shape::Castle, Team::Black),
            (piece::Loc::new(0, 1), piece::Shape::Knight, Team::Black),
            (piece::Loc::new(0, 2), piece::Shape::Bishop, Team::Black),
            (piece::Loc::new(0, 3), piece::Shape::Queen, Team::Black),
            (piece::Loc::new(0, 4), piece::Shape::King, Team::Black),
            (piece::Loc::new(0, 5), piece::Shape::Bishop, Team::Black),
            (piece::Loc::new(0, 6), piece::Shape::Knight, Team::Black),
            (piece::Loc::new(0, 7), piece::Shape::Castle, Team::Black),
            (piece::Loc::new(7, 0), piece::Shape::Castle, Team::White),
            (piece::Loc::new(7, 1), piece::Shape::Knight, Team::White),
            (piece::Loc::new(7, 2), piece::Shape::Bishop, Team::White),
            (piece::Loc::new(7, 3), piece::Shape::Queen, Team::White),
            (piece::Loc::new(7, 4), piece::Shape::King, Team::White),
            (piece::Loc::new(7, 5), piece::Shape::Bishop, Team::White),
            (piece::Loc::new(7, 6), piece::Shape::Knight, Team::White),
            (piece::Loc::new(7, 7), piece::Shape::Castle, Team::White),
        ];

        // go through locs
        for &(l, s, t) in locs.iter() {
            match g.board.get(l) {
                Some(p) => {
                    assert_eq!(p.shape(), s);
                    assert_eq!(p.team(), t);
                },
                None => panic!("Expected shape!")
            }
        }

        // check for pawns
        for c in 0..8 {
            let l = piece::Loc::new(6, c);
            match g.board.get(l) {
                Some(p) => {
                    assert_eq!(p.shape(), piece::Shape::Pawn);
                    assert_eq!(p.team(), Team::White);
                }, 
                None => panic!("Expected shape!")
            }
            let l = piece::Loc::new(1, c);
            match g.board.get(l) {
                Some(p) => {
                    assert_eq!(p.shape(), piece::Shape::Pawn);
                    assert_eq!(p.team(), Team::Black);
                }, 
                None => panic!("Expected shape!")
            }
        }

        // check empty spots
        for r in 2..6 {
            for c in 0..8 {
                let l = piece::Loc::new(r, c);
                if let None = g.board.get(l) {
                    continue;
                } else {
                    panic!("Expected empty spot on board!");
                }
            }
        }
    }
}

