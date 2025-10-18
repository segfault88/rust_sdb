use anyhow::Result;
use game_data::Game;
use game_data::GameMap;
use rand::{rng, seq::IteratorRandom};
use serde_json::{from_reader, to_writer_pretty};
use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;

mod game_data;

const SAMPLE_SIZE: usize = 1_000;

fn main() -> Result<()> {
    println!("reading games.json");

    let start = Instant::now();

    let f = File::open("data/games.json")?;
    let games: game_data::GameMap = from_reader(f)?;

    let total_count = games.len();

    println!(
        "read games map, len: {:?} in {:?}",
        total_count,
        Instant::now().duration_since(start)
    );

    let start = Instant::now();

    let games: HashMap<u64, Game> = games
        .iter()
        .filter(|(_, game)| !game.notes.contains("sexual"))
        .map(|(id, game)| (*id, game.clone()))
        .collect();

    let filtered_count = games.len();

    println!(
        "filtered out {} games, remaining {}, in {:?}",
        total_count - filtered_count,
        filtered_count,
        Instant::now().duration_since(start)
    );

    let start = Instant::now();

    let f = File::create("data/games_filtered.json")?;
    to_writer_pretty(f, &games)?;

    println!(
        "saved filtered game list in {:?}",
        Instant::now().duration_since(start)
    );

    save_sample(&games)?;

    Ok(())
}

fn save_sample(games: &GameMap) -> Result<()> {
    let start = Instant::now();
    let mut rng = rng();

    let mut sample = game_data::GameMap::new();

    for key in games.keys().choose_multiple(&mut rng, 1000) {
        if let Some(game) = games.get(key) {
            sample.insert(*key, game.clone());
        }
    }

    println!(
        "sampled {} random games in {:?}",
        sample.len(),
        Instant::now().duration_since(start)
    );

    let start = Instant::now();

    let f = File::create(format!("data/sample_{}.json", SAMPLE_SIZE))?;

    serde_json::to_writer_pretty(f, &sample)?;

    println!(
        "sample saved to file in {:?}",
        Instant::now().duration_since(start)
    );

    Ok(())
}
