use genkai_search::{input_string, parse_command, ss, AnyError, Dict, Engine};

use std::env;

use romaji::RomajiExt;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match &env::args()
        .collect::<Vec<_>>()
        .iter()
        .skip(1)
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()[..]
    {
        &[] => run(),
        &["auto"] => run_auto(),
        &["learn"] => run_learn(),
        &["init"] => run_init(),
        _ => Err(Box::new(AnyError::new("".to_string()))),
    }
}

fn run_learn() -> Result<(), Box<dyn std::error::Error>> {
    ss::learn();
    Ok(())
}

fn run_auto() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    BufReader::new(File::open("dict.bin")?).read_to_end(&mut buf)?;
    let dict = Dict::load(&buf)?;
    let mut engine = Engine::new(&dict);
    let mut count = 0;
    let ssm = ss::SSManager::init();
    println!("input:");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let (cmd, params) = parse_command(input);
    if let None = cmd {
        let ends = params;
        loop {
            if let Some((start, len)) = ssm.cur() {
                count += 1;
                let end = if ends.len() != 0 {
                    ends.get(count % ends.len())
                        .cloned()
                        .and_then(|x| x.to_katakana().chars().next())
                } else {
                    None
                };
                let all_words = engine.find(start, end, len, len >= 8);
                if let Some(word) = all_words.first() {
                    engine.use_(word.clone());
                    println!("{}", word);
                    input_string(&word);
                } else {
                    println!("not fount");
                }
            }
        }
    } else {
        Ok(())
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    BufReader::new(File::open("dict.bin")?).read_to_end(&mut buf)?;
    let dict = Dict::load(&buf)?;
    let mut engine = Engine::new(&dict);
    let mut default_end = Vec::new();
    let mut count = 0;
    loop {
        count += 1;
        println!("input:");
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let (cmd, params) = parse_command(input);
        match cmd.as_ref().map(String::as_str) {
            Some("d") => {
                default_end = params;
            }
            Some("r") => engine.reset(),
            Some(x) => println!("not found command: ':{}'", x),
            None => {
                match (
                    params.get(0).and_then(|x| x.to_katakana().chars().next()),
                    params.get(1).and_then(|x| x.parse::<usize>().ok()),
                ) {
                    (Some(start), Some(len)) => {
                        let end = vec![
                            params.get(2).cloned().into_iter().collect::<Vec<_>>(),
                            default_end.clone(),
                        ]
                        .into_iter()
                        .filter_map(|x| {
                            if x.len() != 0 {
                                x.get(count % x.len()).cloned()
                            } else {
                                None
                            }
                        })
                        .next()
                        .and_then(|x| x.to_katakana().chars().next());
                        let all_words = engine.find(start, end, len, len >= 8);
                        let words = all_words.into_iter().take(3).collect::<Vec<_>>();
                        for word in &words {
                            engine.use_(word.clone());
                        }
                        if words.len() != 0 {
                            println!("{}", words.into_iter().rev().collect::<Vec<_>>().join("\n"));
                        } else {
                            println!("not fount");
                        }
                    }
                    _ => {
                        println!("input error");
                    }
                }
            }
        }
    }
}

fn run_init() -> Result<(), Box<dyn std::error::Error>> {
    BufWriter::new(File::create("dict.bin")?).write_all(
        &Dict::from_csv(
            BufReader::new(File::open("dict.csv")?)
                .lines()
                .filter_map(|x| x.ok()),
        )
        .dump()?,
    )?;

    Ok(())
}
