mod chess {




    #[derive(Copy, Clone)]
    pub enum Piece {
        Castle,
        Knight,
        Bishop,
        Queen,
        King,
        Pawn,
    }

    impl Piece {
        pub fn get_path(&self) {
        }

    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum Team {
        White, 
        Black,
    } 

    pub struct Game {
        turn: Team,
        board: [[Option<(Piece, Team)>; 8]; 8],
    }

    impl Game {
        pub fn new() -> Game {
            let mut g = Game {
                turn: Team::White,
                board: [[None; 8]; 8],
            };

            // setting intial position of pieces
            let pieces: [(usize, usize, Piece); 16] = [
                (0, 0, Piece::Castle),
                (0, 1, Piece::Knight),
                (0, 2, Piece::Bishop),
                (0, 3, Piece::Queen),
                (0, 4, Piece::King),
                (0, 5, Piece::Bishop),
                (0, 6, Piece::Knight),
                (0, 7, Piece::Castle),
                (7, 0, Piece::Castle),
                (7, 1, Piece::Knight),
                (7, 2, Piece::Bishop),
                (7, 3, Piece::Queen),
                (7, 4, Piece::King),
                (7, 5, Piece::Bishop),
                (7, 6, Piece::Knight),
                (7, 7, Piece::Castle),
            ];
            // add pieces
            for &(r, c, p) in pieces.iter() {
                let t = if r == 0 { Team::White } else { Team::Black };
                g.board[r][c] = Some((p, t));
            }
            // add pawns
            for i in 0..7 {
                g.board[1][i] = Some((Piece::Pawn, Team::White));
                g.board[6][i] = Some((Piece::Pawn, Team::Black));
            }
            return g;
        }

        fn is_valid_path(&self, path: Vec<(usize, usize)>) -> bool {
            for (r, c) in path {
                if let Some(_) = self.board[r][c] {
                    // piece in the way invalid
                    return false;
                }
            }
            return true;
        }


        pub fn move_piece(
                &mut self,
                old_loc: (usize, usize),
                new_loc: (usize, usize)) -> bool {
            // store old board and change to new
            let old_board = self.board;
            // extract location variables
            let (old_r, old_c) = old_loc;
            let (new_r, new_c) = new_loc;

            // extract piece & team
            let (piece, team);
            if let Some((p, t)) = self.board[old_r][old_c] {
                piece = p;
                team = t;
            } else {
                return false;
            }
            // check if teams turn
            if team != self.turn { return false; }

            // obtain path if valid
            /*
            let path;
            if let Some(p) = piece.get_path(old_loc, new_loc) {
                path = p;
            } else {
                return false;
            }*/

            return true;
                /*
                    // not the teams turn
                    if team != self.turn { return false; }
                    match piece {
                        Piece::Castle => { 
                            let path: Vec<usize, usize>;
                            if old_r != new_r && old_c == new_c {
                                // verticle motion
                                for r in old_r..new_c {
                                    // skip initial piece
                                    if old_r == r { continue; }
                                    if let Some(_) = self.board[r][old_c] {
                                        // piece in the way... invalid move
                                        return false;
                                    }
                                }
                                match self.board[new_r][new_c] {
                                    None => { 
                                        self.board[new_r][new_c] = Some((piece, team));
                                    },
                                    Some((p, t)) => {
                                        if t != team {
                                            self.board[new_r][new_c] = Some((piece, team));
                                        } else {
                                            // invalid move
                                            return false;
                                        }
                                    }
                                }
                            } else if old_r == new_r && old_c != new_c {
                                for c in old_c..new_c {
                                    // skip initial piece

                                // horizontal motion
                                // Add king castle switch
                            } else {
                                // neighter option, invalid move
                                return false;
                            }
                        }
                        Piece::Knight => { }
                        Piece::Bishop => { }
                        Piece::Queen => { }
                        Piece::King => { }
                        Piece::Pawn => { }
                    }
                    return true;
                }
            }*/
        }
    }

}

fn main() {
    //let w, b = chess::Game::new();
    //g.board[1] = chess::Piece::King(chess::Team::White);
    //let x = [1, 2, 3, 4];
    //let i = 4;
    //let y = x[i];
    println!("Hello, world!");
    //let k = chess::Piece::King(chess::Team::White);
    //let b: [u32; 10];
    //let mut b: [Piece; 64] = [Piece::Empty; 64];
    //b[1].hello();
    //b[1] = Piece::Queen(Team::White);
    //b[1].hello();
    //b[1].hello();
    //let b = [board::TestX, 10]; //[board::Piece, 10];
    //let k = board::pieces::Piece::king(board::Team::White);
}
