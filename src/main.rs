use std::io;

use minesweeper_rust::multiplayer::simulation::*;
use minesweeper_rust::single_player::simulation::*;

fn main() -> io::Result<()> {
    let multiplayer_game = simulate_multiplayer()?;
    // let game = simulate_single_player();

    Ok(())    
}
