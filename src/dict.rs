use std::collections::{HashMap, HashSet};

pub struct Dict(pub HashMap<(char, usize), Vec<String>>);

impl Dict {
    pub fn from_csv(lines: impl std::iter::Iterator<Item = String>) -> Dict {
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

    pub fn dump(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(&self.0)?)
    }

    pub fn load(buf: &Vec<u8>) -> Result<Dict, Box<dyn std::error::Error>> {
        Ok(Dict(bincode::deserialize(buf)?))
    }

    pub fn pick_and_sorted(
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

    pub fn pick_and_sorted_and_limit(
        &self,
        len: usize,
        start: char,
        end: Option<char>,
        showd: &mut HashSet<String>,
        limit: usize,
    ) -> Vec<String> {
        let res = self.pick_and_sorted(len, start, end, showd);
        let res = res.into_iter().take(limit).collect::<Vec<_>>();
        for x in &res {
            showd.insert(x.clone());
        }
        res
    }
}
