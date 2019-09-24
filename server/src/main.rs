mod board;

fn main() {
    let x = [1, 2, 3, 4];
    let i = 4;
    let y = x[i];
    println!("Hello, world!");
    let k = board::pieces::Piece::king(board::Team::White);
}
