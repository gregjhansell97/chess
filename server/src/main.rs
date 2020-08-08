mod chess;

use rouille::Response;
use rouille::router;
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
struct MovePieceJSON {
    old: chess::piece::Loc,
    new: chess::piece::Loc,
}

fn main() {
    let g = Arc::new(Mutex::new(chess::Game::new()));
    rouille::start_server("0.0.0.0:80", move |request| {
        let response = rouille::match_assets(&request, "public");
        if response.is_success() {
            return response;
        } else {
            return rouille::router!(request,
                (GET) (/game) => {
                    Response::json(&g.lock().unwrap().deref())
                },
                (POST) (/move_piece) => {
                    let loc: MovePieceJSON;
                    loc = rouille::try_or_400!(rouille::input::json_input(request));
                    g.lock().unwrap().move_piece(loc.old, loc.new); 
                    Response::json(&g.lock().unwrap().deref())
                },

                _ => Response::text("failed to find files")
            );
        }
    });
}
