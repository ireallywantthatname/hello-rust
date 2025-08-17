use minigrep::{case_insensitive_search, case_sensitive_search};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Problem reading the file: {}", e);
        process::exit(1);
    }
}

#[derive(Debug)]
struct Config {
    word: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let word = args[1].clone();
        let filename = args[2].clone();

        let is_case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            word,
            filename,
            is_case_sensitive,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    println!("RUNNING IN CASE {} MODE\n", {
        if config.is_case_sensitive {
            "SENSITIVE"
        } else {
            "INSENSITIVE"
        }
    });

    let result = if config.is_case_sensitive {
        case_sensitive_search(&config.word, &content)
    } else {
        case_insensitive_search(&config.word, &content)
    };

    println!(
        "{} has the literal {} included in the following lines:\n",
        config.filename, config.word
    );

    for line in result {
        println!("{}", line);
    }

    Ok(())
}
