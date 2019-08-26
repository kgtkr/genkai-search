use std::collections::{HashMap, HashSet};
use std::env;

use romaji::RomajiExt;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};

struct Dict(HashMap<(char, usize), Vec<String>>);

impl Dict {
    fn from_csv(lines: impl std::iter::Iterator<Item = String>) -> Dict {
        Dict(
            lines
                .filter_map(|x| x.split(',').nth(11).map(|x| x.to_string()))
                .filter(|x| x.chars().last() != Some('ン'))
                .collect::<HashSet<_>>()
                .into_iter()
                .filter_map(|x| {
                    x.chars()
                        .next()
                        .clone()
                        .map(|first| ((first, x.chars().count().min(9)), x))
                })
                .fold(HashMap::new(), |mut dict, (k, v)| {
                    dict.entry(k).or_insert_with(Vec::new).push(v);
                    dict
                }),
        )
    }

    fn dump(&self) -> Result<Vec<u8>, Box<std::error::Error>> {
        Ok(bincode::serialize(&self.0)?)
    }

    fn load(buf: &Vec<u8>) -> Result<Dict, Box<std::error::Error>> {
        Ok(Dict(bincode::deserialize(buf)?))
    }

    fn pick_and_sorted(
        &self,
        len: usize,
        start: char,
        end: Option<char>,
        showd: &HashSet<String>,
    ) -> Vec<String> {
        let mut picked = self.0.get(&(start, len)).cloned().unwrap_or_else(Vec::new);
        picked.sort_by_key(|x| {
            (
                showd.contains(x),
                !end.clone()
                    .map(|end| x.trim_end_matches("ー").chars().last() == Some(end))
                    .unwrap_or(true),
            )
        });
        picked
    }

    fn pick_and_sorted_and_limit(
        &self,
        len: usize,
        start: char,
        end: Option<char>,
        showd: &mut HashSet<String>,
        limit: usize,
    ) -> Vec<String> {
        let res = self.pick_and_sorted(len, start, end, showd);
        let mut res = res.into_iter().take(limit).collect::<Vec<_>>();
        for x in &res {
            showd.insert(x.clone());
        }
        res
    }
}

fn tokenlize_input(line: String) -> Vec<String> {
    line.trim_end_matches("\n")
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
}

fn satisfy<T>(items: Vec<T>, f: impl FnOnce(&T) -> bool) -> (Option<T>, Vec<T>) {
    let mut iters = items.into_iter();
    if let Some(first) = iters.next() {
        if f(&first) {
            (Some(first), iters.collect())
        } else {
            (None, std::iter::once(first).chain(iters).collect())
        }
    } else {
        (None, iters.collect())
    }
}

fn parse_input(line: String) -> (Option<String>, Vec<String>) {
    let tokens = tokenlize_input(line);
    let (cmd, params) = satisfy(tokens, |x| x.chars().next() == Some(':'));
    (cmd.map(|x| x.chars().skip(1).collect()), params)
}

fn main() -> Result<(), Box<std::error::Error>> {
    if env::args().nth(1) == Some("init".to_string()) {
        BufWriter::new(File::create("dict.bin")?).write_all(
            &Dict::from_csv(
                BufReader::new(File::open("dict.csv")?)
                    .lines()
                    .filter_map(|x| x.ok()),
            )
            .dump()?,
        )?;
    } else {
        let mut buf = Vec::new();
        BufReader::new(File::open("dict.bin")?).read_to_end(&mut buf)?;
        let data = Dict::load(&buf)?;
        let mut showd = HashSet::new();
        let mut default_end = Vec::new();
        let mut count = 0;
        loop {
            count += 1;
            println!("input:");
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let (cmd, params) = parse_input(input);
            match cmd.as_ref().map(String::as_str) {
                Some("d") => {
                    default_end = params;
                }
                Some("r") => showd.clear(),
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
                            let mut res =
                                data.pick_and_sorted_and_limit(len, start, end, &mut showd, 3);
                            if res.len() != 0 {
                                res.reverse();
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
