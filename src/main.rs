use std::io;

use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    language: Option<String>,
    #[arg(short, long)]
    version: Option<String>,
    #[arg(short, long)]
    generation: Option<usize>,
    #[arg(short, long)]
    rounds: Option<u32>,
    /// Updates the configuration values in disk to be the ones used in this call
    #[arg(short, long)]
    save: bool,
    /// Use together with --save when you only want to change the config but not play the game
    #[arg(short, long)]
    no_play: bool,
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: String,
    language: String,
    generation: usize,
    rounds: u32,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            version: "x".into(),
            language: "en".into(),
            generation: 4,
            rounds: 10,
        }
    }
}

#[derive(Deserialize)]
struct Language {
    name: String,
}

#[derive(Deserialize)]
struct Version {
    name: String,
}

#[derive(Deserialize)]
struct FlavorTextEntries {
    flavor_text: String,
    language: Language,
    version: Version,
}

#[derive(Deserialize)]
struct PokemonSpeciesResponse {
    name: String,
    flavor_text_entries: Vec<FlavorTextEntries>,
}

impl PokemonSpeciesResponse {
    fn to_pokemon(self, lang: &str, gen: &str) -> Pokemon {
        let desc = &self
            .flavor_text_entries
            .iter()
            .find(|entry| entry.language.name == lang && entry.version.name == gen)
            .expect(&format!(
                "{} does not have a description for version {} and in {} language",
                self.name, gen, lang
            ))
            .flavor_text;
        Pokemon {
            name: self.name,
            description: sanitize_text(desc),
        }
    }
}

struct Pokemon {
    name: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const APP_NAME: &str = "whos-this-pokemon";
    const POKEMON_IN_GENERATION: [u16; 9] = [151, 251, 386, 493, 649, 721, 809, 905, 1008];

    let mut cfg = confy::load::<MyConfig>(APP_NAME, None)?;

    let args = Args::parse();
    if let Some(language) = args.language {
        cfg.language = language
    }
    if let Some(version) = args.version {
        cfg.version = version
    }
    if let Some(generation) = args.generation {
        cfg.generation = generation
    }
    if let Some(rounds) = args.rounds {
        cfg.rounds = rounds
    }
    if args.save {
        confy::store(APP_NAME, None, &cfg)?;
    }
    if args.no_play {
        return Ok(());
    }

    println!("Welcome \"Who's this Pokemon!\". Guess the pokemon by it's pokedex description");
    println!(
        "You're playing {} rounds with gen {} Pokemons with {} Pokedex version descriptions in {}.\n",
        cfg.rounds, cfg.generation, cfg.version, cfg.language
    );

    let mut rng = rand::thread_rng();

    let mut hits = 0;
    for i in 1..=cfg.rounds {
        let url = format!(
            "https://pokeapi.co/api/v2/pokemon-species/{}",
            rng.gen_range(1..POKEMON_IN_GENERATION[cfg.generation - 1])
        );
        let response = reqwest::get(url).await?;

        let pokemon_serialized = response.json::<PokemonSpeciesResponse>().await?;
        let pokemon = pokemon_serialized.to_pokemon(&cfg.language, &cfg.version);
        println!("#{} - {}", i, pokemon.description);

        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();

        if input.to_lowercase().trim() == pokemon.name {
            println!("Correct!");
            hits += 1;
        } else {
            println!("Wrong! It was {}", pokemon.name.to_uppercase());
        }
        println!()
    }

    println!("{}/{} correct guesses.", hits, cfg.rounds);

    Ok(())
}

fn sanitize_text(text: &str) -> String {
    text.replace("\n", " ").replace("\u{0C}", " ")
}
