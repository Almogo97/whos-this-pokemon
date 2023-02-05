use std::io;

use rand::Rng;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Language {
    #[serde(rename = "name")]
    language: String,
}

#[derive(Deserialize)]
struct Version {
    #[serde(rename = "name")]
    version: String,
}

#[derive(Deserialize)]
struct FlavorTestEntries {
    flavor_text: String,
    language: Language,
    version: Version,
}

#[derive(Deserialize)]
struct PokemonSpeciesResponse {
    name: String,
    flavor_text_entries: Vec<FlavorTestEntries>,
}

struct Pokemon {
    name: String,
    description: String,
}

impl Pokemon {
    fn from_response(response: PokemonSpeciesResponse) -> Pokemon {
        Pokemon {
            name: response.name,
            description: sanitize_text(&response.flavor_text_entries[0].flavor_text),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Welcome to the guess the Pokemon game! Guess the pokemon according to it's pokedex description");
    println!("You're playing with gen 1 Pokedex descriptions.");

    let mut rng = rand::thread_rng();

    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", "poliwrath");
    let response = reqwest::get(url).await?;

    let pokemon_serialized = response.json::<PokemonSpeciesResponse>().await?;
    let pokemon = Pokemon::from_response(pokemon_serialized);
    println!("{}", pokemon.description);

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();

    if input.to_lowercase().trim() == pokemon.name {
        println!("Congrats! You win!");
    } else {
        println!("Sorry, wrong guess. It was {}", pokemon.name);
    }

    Ok(())
}

fn sanitize_text(text: &str) -> String {
    text.replace("\n", " ").replace("\u{0C}", " ")
}
