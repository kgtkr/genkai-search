use std::collections::{HashMap, HashSet};

pub struct Dict(pub HashMap<char, Vec<(usize, String)>>);

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
                        .map(|first| (first, (x.chars().count().min(9), x)))
                })
                .fold(HashMap::new(), |mut dict, (k, v)| {
                    dict.entry(k).or_insert_with(Vec::new).push(v);
                    dict
                })
                .into_iter()
                .map(|(k, mut v)| {
                    v.sort_by_key(|&(len, _)| len);
                    (k, v)
                })
                .collect::<HashMap<_, _>>(),
        )
    }

    pub fn dump(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(&self.0)?)
    }

    pub fn load(buf: &Vec<u8>) -> Result<Dict, Box<dyn std::error::Error>> {
        Ok(Dict(bincode::deserialize(buf)?))
    }

    pub fn find(&self, start: char, len: usize, gt: bool) -> HashSet<String> {
        self.0
            .get(&start)
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .filter(|&(s_len, _)| if gt { s_len >= len } else { s_len == len })
            .map(|(_, x)| x)
            .collect::<HashSet<_>>()
    }
}
