use super::Dict;
use std::collections::HashSet;

pub struct Engine<'a> {
    dict: &'a Dict,
    used: HashSet<String>,
}

impl<'a> Engine<'a> {
    pub fn new(dict: &'a Dict) -> Engine<'a> {
        Engine {
            dict,
            used: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.used.clear();
    }

    pub fn find(&self, start: char, end: Option<char>, len: usize, gt: bool) -> Vec<String> {
        let mut list = self
            .dict
            .find(start, len, gt)
            .into_iter()
            .collect::<Vec<_>>();
        list.sort_by_key(|x| {
            (
                self.used.contains(x),
                !end.map(|end| x.trim_end_matches("ー").chars().last() == Some(end))
                    .unwrap_or(true),
            )
        });
        list
    }

    pub fn use_(&mut self, word: String) {
        self.used.insert(word);
    }
}
