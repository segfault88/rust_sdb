use anyhow::Result;
use bincode::{config::standard, decode_from_std_read, encode_into_std_write};
use clap::{Parser, Subcommand};
use firestore::*;
use game_data::Game;
use game_data::GameMap;
use indicatif::ProgressBar;
use rand::{rng, seq::IteratorRandom};
use regex::Regex;
use regex::RegexBuilder;
use rustls::crypto::CryptoProvider;
use serde_json::{from_reader, to_writer_pretty};
use std::collections::HashMap;
use std::fs::File;
use std::sync::LazyLock;
use std::time::Instant;

mod game_data;

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

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = CryptoProvider::install_default(rustls::crypto::ring::default_provider()) {
        // Only panic if the installation fails, which shouldn't happen here
        eprintln!("Failed to install rustls crypto provider: {:?}", e);
    }

    let cli = Cli::parse();

    return match cli.command {
        Commands::CreateSample { count } => {
            println!("create sample");
            create_sample(count)
        }
        Commands::TestFS {} => test_fs().await,
    };
}

async fn test_fs() -> Result<()> {
    let mut f = File::open("data/sample_1000.bin")?;
    let games: GameMap = decode_from_std_read(&mut f, standard())?;

    println!("loaded {} games", games.len());

    let project_id: String = std::env::var_os("PROJECT_ID")
        .expect("set PROJECT_ID env")
        .into_string()
        .unwrap();

    let db = FirestoreDb::with_options_service_account_key_file(
        FirestoreDbOptions::new(project_id).with_database_id("sdb-database2".into()),
        ".key.json".into(),
    )
    .await?;

    println!("listing collection ids");

    let list = db
        .list_collection_ids(FirestoreListCollectionIdsParams {
            parent: None,
            page_size: 100,
            page_token: None,
        })
        .await?;

    println!("collections:");

    for collection_id in list.collection_ids {
        println!("collection_id: {}", collection_id);
    }

    let bar = ProgressBar::new(games.len() as u64);
    for game in games.values() {
        bar.inc(1);
        let _result = db
            .fluent()
            .insert()
            .into("test")
            .generate_document_id()
            .object(game)
            .execute::<Game>()
            .await?;
    }
    bar.finish();

    Ok(())
}

fn is_dodgy(game: &Game) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        RegexBuilder::new(r"entai|exual|exy|🔞")
            .case_insensitive(true)
            .unicode(true)
            .build()
            .unwrap()
    });

    game.tags.keys().any(|key| key.contains("entai"))
        || RE.is_match(&game.name)
        || RE.is_match(&game.notes)
        || RE.is_match(&game.short_description)
        || RE.is_match(&game.detailed_description)
        || RE.is_match(&game.about_the_game)
        || RE.is_match(&game.website)
        || RE.is_match(&game.support_url)
        || RE.is_match(&game.support_email)
        || RE.is_match(&game.reviews)
        || game
            .publishers
            .iter()
            .any(|publisher| RE.is_match(publisher))
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
        .filter(|(_, game)| !is_dodgy(game))
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
