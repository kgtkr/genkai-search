use std::collections::{HashMap, HashSet};
use std::env;

use romaji::RomajiExt;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};

type DictType = HashMap<(char, usize), Vec<String>>;

fn main() -> Result<(), Box<std::error::Error>> {
    if env::args().nth(1) == Some("init".to_string()) {
        BufWriter::new(File::create("dict.bin")?).write_all(&bincode::serialize(
            &BufReader::new(File::open("dict.csv")?)
                .lines()
                .filter_map(|x| x.ok())
                .filter_map(|x| x.split(',').nth(11).map(|x| x.to_string()))
                .filter(|x| x.chars().last() != Some('ン'))
                .collect::<HashSet<_>>()
                .into_iter()
                .filter_map(|x| {
                    x.chars()
                        .next()
                        .clone()
                        .map(|first| ((first, x.chars().count()), x))
                })
                .fold(HashMap::new(), |mut dict, (k, v)| {
                    dict.entry(k).or_insert_with(Vec::new).push(v);
                    dict
                }),
        )?)?;
    } else {
        let mut buf = Vec::new();
        BufReader::new(File::open("dict.bin")?).read_to_end(&mut buf)?;
        let data = bincode::deserialize::<DictType>(&buf)?;
        loop {
            println!("input:");
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let mut splited = input.split(' ');
            match (
                splited.next().and_then(|x| x.to_katakana().chars().next()),
                splited.next().and_then(|x| x.parse::<usize>().ok()),
            ) {
                (Some(start), Some(len)) => {
                    let end = splited.next().and_then(|x| x.to_katakana().chars().next());
                    if let Some(res) = data.get(&(start, len)).cloned() {
                        println!(
                            "{}",
                            res.into_iter()
                                .filter(|x| {
                                    end.clone()
                                        .map(|end| x.chars().last() == Some(end))
                                        .unwrap_or(true)
                                })
                                .take(30)
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
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

    Ok(())
}
