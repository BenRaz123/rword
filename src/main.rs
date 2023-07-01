use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use simple_colors::*;
use std::error::Error;
use webster::*;

#[derive(Parser, Debug)]
#[command(name = "rword")]
struct Args {
    #[arg(help = "The word to define. Defaults to a random word.")]
    word: Option<String>,
}

struct Word {
    name: String,
    definition: String,
}

impl Word {
    fn new(name: &str, definition: &Option<&str>) -> Option<Word> {
        if definition.is_none() {
            return None;
        }
        Some(Word {
            name: name.into(),
            definition: definition.unwrap().into(),
        })
    }
    fn display(&self) {
        println!("{} - {}", bold!(blue!(self.name)), self.definition);
    }
}

fn get_random_word() -> Result<String, Box<dyn Error>> {
    let word_list: String = std::fs::read_to_string("/usr/share/dict/words")?;
    let mut words: Vec<&str> = word_list.split("\n").collect();
    words.pop();
    let word: String = words.choose(&mut thread_rng()).unwrap().to_string();
    Ok(word)
}

fn main() {
    let args = Args::parse();

    if args.word.is_some() {
        let name = args.word.unwrap();
        let definition = dictionary(&name);
        let word = Word::new(&name, &definition);
        if word.is_none() {
            eprintln!("{}: That word has no definition!", red!(bold!("error")));
            std::process::exit(1)
        }
        word.unwrap().display();
        std::process::exit(0);
    }

    loop {
        if get_random_word().is_err() {
            eprintln!(
                "{}: could not read from {}.",
                red!(bold!("error")),
                gray!("/usr/share/dict/words")
            );
            std::process::exit(1);
        }

        let name = get_random_word().unwrap();
        let definition = dictionary(&name);

        let current_word = Word::new(&name, &definition);

        if current_word.is_none() {
            continue;
        }

        current_word.unwrap().display();

        break;
    }
}
