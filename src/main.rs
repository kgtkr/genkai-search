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
                        .map(|first| ((first, x.chars().count().min(8)), x))
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
        let mut showd = HashSet::new();
        let mut default_end = None;
        loop {
            println!("input:");
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let input = input.trim_end_matches("\n");
            let splited = input.split(' ').map(|x| x.to_string()).collect::<Vec<_>>();
            match splited.get(0).map(|x| x.as_ref()) {
                Some(":e") => {
                    default_end = splited.get(1).cloned();
                }
                Some(":r") => showd.clear(),
                _ => {
                    match (
                        splited.get(0).and_then(|x| x.to_katakana().chars().next()),
                        splited.get(1).and_then(|x| x.parse::<usize>().ok()),
                    ) {
                        (Some(start), Some(len)) => {
                            let end = vec![splited.get(2), default_end.as_ref()]
                                .into_iter()
                                .filter_map(|x| x)
                                .next()
                                .and_then(|x| x.to_katakana().chars().next());
                            if let Some(res) = data.get(&(start, len)).cloned() {
                                let mut res = res.into_iter().collect::<Vec<_>>();
                                res.sort_by_key(|x| {
                                    let not_contains = !showd.contains(x);
                                    let is_end = end
                                        .clone()
                                        .map(|end| {
                                            x.trim_end_matches("ー").chars().last() == Some(end)
                                        })
                                        .unwrap_or(true);
                                    if not_contains && is_end {
                                        0
                                    } else if not_contains {
                                        1
                                    } else {
                                        2
                                    }
                                });
                                let mut res = res.into_iter().take(3).collect::<Vec<_>>();
                                res.reverse();
                                for x in &res {
                                    showd.insert(x.clone());
                                }
                                println!("{}", res.join("\n"));
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

    Ok(())
}
