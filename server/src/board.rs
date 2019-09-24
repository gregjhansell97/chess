
#[derive(Copy, Clone)]
pub enum Team {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum Move {
    Valid,
    Invalid
}

pub struct Board {
    //board:[Option<pieces::Piece>; 64]
}

pub struct Loc {
    x: usize,
    y: usize,
}
impl Loc {
    fn new(x: usize, y: usize) -> Option<Loc> {
     /*   if(x + 8*y >= 64) {
            return None;
        } else {
            return Some((x, y));
        }*/
        return None;
    }
}

impl Board {
    fn get(&self, l: Loc) -> Option<pieces::Piece> {
        /*let index = x + 8*y;
        if(index >= 64) {
            return None;
        }
        return self.board[x + 8*y];*/
        return None;
    }
}

// implement piece as a struct, with an attribut of type and match the type for
// move_to ect...
pub mod pieces {
    #[derive(Copy, Clone)]
    enum Shape {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }
    #[derive(Copy, Clone)]
    pub struct Piece {
        shape: Shape,
        team: crate::board::Team,
    }
    impl Piece {
        fn create(team: crate::board::Team, shape: Shape) -> Piece {
            return Piece {shape, team}
        }
        pub fn king(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::King)
        }
        pub fn queen(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::Queen)
        }
        pub fn rook(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::Rook)
        }
        pub fn bishop(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::Bishop)
        }
        pub fn knight(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::Knight)
        }
        pub fn pawn(team: crate::board::Team) -> Piece {
            return Piece::create(team, Shape::Pawn)
        }
    }
}


/*
pub trait Piece {
    /**
     * moves piece to new location
     */
    fn create(b: &Board);
    fn move_to(&self, l: Loc) -> Move;
    fn team(&self) -> Team;
}

pub mod pieces {
}
*/
