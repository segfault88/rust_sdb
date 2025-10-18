use anyhow::Result;
use bincode::{config::standard, encode_into_std_write};
use clap::{Parser, Subcommand};
use game_data::Game;
use game_data::GameMap;
use rand::{rng, seq::IteratorRandom};
use serde_json::{from_reader, to_writer_pretty};
use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;

mod game_data;

const SAMPLE_SIZE: usize = 1_000;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    // This field holds the parsed subcommand and its arguments
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read data/games.json, filter some of the dodgey stuff, take random sample and save into new files
    CreateSample {
        /// The number of random items to select
        #[clap(short, long, default_value_t = 1000)]
        count: usize,
    },

    /// Try pushing some stuff to fs
    TestFS {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    return match cli.command {
        Commands::CreateSample { count } => {
            println!("create sample");
            create_sample(count)
        }
        Commands::TestFS {} => test_fs(),
    };
}

fn test_fs() -> Result<()> {
    Ok(())
}

fn create_sample(count: usize) -> Result<()> {
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

    let start = Instant::now();
    let mut f = File::create("data/games_filtered.bin")?;
    encode_into_std_write(&games, &mut f, standard())?;

    println!(
        "saved filtered game list bincode in {:?}",
        Instant::now().duration_since(start)
    );

    save_sample(&games, count)?;

    Ok(())
}

fn save_sample(games: &GameMap, count: usize) -> Result<()> {
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

    let f = File::create(format!("data/sample_{}.json", count))?;

    serde_json::to_writer_pretty(f, &sample)?;

    println!(
        "sample saved to file in {:?}",
        Instant::now().duration_since(start)
    );

    let start = Instant::now();

    let mut f = File::create(format!("data/sample_{}.bin", count))?;

    encode_into_std_write(&sample, &mut f, standard())?;

    println!(
        "sample saved to file in {:?}",
        Instant::now().duration_since(start)
    );

    Ok(())
}
