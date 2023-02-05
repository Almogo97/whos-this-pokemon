use std::io;

use rand::Rng;
use reqwest::Error;
use serde::Deserialize;

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
async fn main() -> Result<(), Error> {
    println!("Welcome to the guess the Pokemon game! Guess the pokemon according to it's pokedex description");
    println!("You're playing with gen 1 Pokedex descriptions.\n");

    // Select language
    println!("What language do you want to play in?");
    let available_languages = [
        "ja-Hrkt", "roomaji", "ko", "zh-Hant", "fr", "de", "es", "it", "en", "cs", "ja", "zh-Hans",
        "pt-BR",
    ];

    let game_language;

    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();
        let trim_input = input.trim();

        if available_languages.contains(&trim_input) {
            game_language = trim_input.to_owned();
            break;
        } else {
            println!(
                "Language not recognized. Please choose one of the following languages: {:?}",
                available_languages
            );
        }
    }

    // Select language
    println!("What generation do you want to play with?");
    let available_versions = [
        "red",
        "blue",
        "yellow",
        "gold",
        "silver",
        "crystal",
        "ruby",
        "sapphire",
        "emerald",
        "firered",
        "leafgreen",
        "diamonnd",
        "pearl",
        "platinum",
        "heartgold",
        "soulsilver",
        "black",
        "white",
        "colosseum",
        "xd",
        "black-2",
        "white-2",
        "x",
        "y",
        "omega-ruby",
        "alpha-sapphire",
        "sun",
        "moon",
        "ultra-sun",
        "ultra-moon",
        "lets-go-pikachu",
        "lets-go-eevee",
        "sword",
        "shield",
        "the-isle-of-armor",
        "the-crown-tundra",
        "brilliant-diamond",
        "shining-pearl",
        "legends-arceus",
        "scarlet",
        "violet",
    ];

    let game_version;

    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();
        let trim_input = input.trim();

        if available_versions.contains(&trim_input) {
            game_version = trim_input.to_owned();
            break;
        } else {
            println!(
                "Version not recognized. Please choose one of the following languages: {:?}",
                available_versions
            );
        }
    }

    let mut rng = rand::thread_rng();

    let url = format!(
        "https://pokeapi.co/api/v2/pokemon-species/{}",
        rng.gen_range(1..152)
    );
    let response = reqwest::get(url).await?;

    let pokemon_serialized = response.json::<PokemonSpeciesResponse>().await?;
    let pokemon = pokemon_serialized.to_pokemon(&game_language, &game_version);
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
