use anyhow::Result;
use serde_json::from_reader;
use std::time::Instant;

mod game_data;

fn main() -> Result<()> {
    println!("reading games.json");

    let start = Instant::now();

    let f = std::fs::File::open("data/games.json")?;
    let games: game_data::GameMap = from_reader(f)?;

    println!(
        "read games map, len: {:?} in {:?}",
        games.len(),
        Instant::now().duration_since(start)
    );

    Ok(())
}
