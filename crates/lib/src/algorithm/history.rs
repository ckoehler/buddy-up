use crate::Person;
use anyhow::Result;
use glob::glob;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    map: HashMap<(usize, usize), usize>,
    #[serde(skip)]
    stats: HistoryStats,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn from_dir(dir: &str) -> Result<Self> {
        let mut history = Self::new();

        // read pairs from dir of files
        let pattern = format!("{dir}/*.json");
        for path in glob(&pattern).expect("Glob pattern works") {
            debug!("Reading history file {path:?}");
            let pairs = std::fs::read_to_string(path?)?;
            let pairs: Vec<(Person, Person)> = serde_json::from_str(&pairs)?;
            let pairs = pairs.iter().map(|p| (p.0.id, p.1.id)).collect();
            history.stats.files_read += 1;
            merge(&mut history, &pairs);
        }
        history.stats.pairs = history.len();
        Ok(history)
    }

    #[allow(dead_code)]
    fn max_iteration(&self) -> usize {
        *self.map.values().max().unwrap_or(&0)
    }

    fn new() -> Self {
        let scores = HashMap::new();
        Self {
            map: scores,
            stats: HistoryStats::default(),
        }
    }

    pub fn stats(&self) -> HistoryStats {
        self.stats
    }

    fn insert(&mut self, pair: (usize, usize), iteration: usize) {
        // if the first variation of the pair exists, insert it there; if not
        // it doesn't really matter if the other one exists, just insert it as the other.
        // Whether that one exists or not, it'll insert there.
        if self.map.contains_key(&pair) {
            self.map.insert(pair, iteration);
        } else {
            self.map.insert((pair.1, pair.0), iteration);
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains(&self, pair: &(usize, usize)) -> bool {
        self.map.contains_key(pair) || self.map.contains_key(&(pair.1, pair.0))
    }

    pub fn get(&self, pair: (usize, usize)) -> Option<usize> {
        Some(match self.map.get(&pair) {
            Some(x) => *x,
            None => *self.map.get(&(pair.1, pair.0))?,
        })
    }

    pub fn min(&self) -> usize {
        *self.map.values().min().unwrap_or(&0)
    }
    pub fn max(&self) -> usize {
        *self.map.values().max().unwrap_or(&0)
    }
}

fn merge(history: &mut History, pairs: &Vec<(usize, usize)>) {
    for p in pairs {
        if history.contains(p) {
            let it = history.get(*p).unwrap();

            history.insert(*p, it + 1);
        } else {
            history.insert(*p, 1);
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HistoryStats {
    pub files_read: usize,
    pub pairs: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_iteration_empty_history() {
        let h = History::new();
        assert_eq!(h.max_iteration(), 0);
    }
    #[test]
    fn test_max_iteration() {
        let mut h = History::new();
        h.insert((1, 2), 4);
        assert_eq!(h.max_iteration(), 4);
    }

    #[test]
    fn test_merge() {
        let mut h = History::new();
        let pairs = vec![(1, 2)];
        merge(&mut h, &pairs);
        assert_eq!(h.max_iteration(), 1);
        assert_eq!(h.len(), 1);

        // if we insert it again, like for another run, we expect the iteration to be incremented
        // by one
        merge(&mut h, &pairs);
        assert_eq!(h.max_iteration(), 2);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn test_merge_same() {
        let mut h = History::new();

        let pairs = vec![(1, 2)];
        let pairs2 = vec![(2, 1)];
        merge(&mut h, &pairs);
        assert_eq!(h.max_iteration(), 1);
        assert_eq!(h.len(), 1);
        merge(&mut h, &pairs2);
        assert_eq!(h.max_iteration(), 2);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn test_contains_either_order() {
        let mut h = History::new();
        let pairs = vec![(1, 2)];
        merge(&mut h, &pairs);
        assert_eq!(h.max_iteration(), 1);
        assert_eq!(h.len(), 1);

        // make both orders of pairs and see if they return the same value
        let pair1 = h.contains(&(1, 2));
        let pair2 = h.contains(&(2, 1));
        assert!(pair1);
        assert!(pair2);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn test_insert_same_pair() {
        let mut h = History::new();
        let pair1 = (1, 2);
        let pair2 = (2, 1);
        h.insert(pair1, 1);
        assert_eq!(h.len(), 1);
        h.insert(pair2, 2);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn test_get_either_order() {
        let mut h = History::new();
        let pairs = vec![(1, 2)];
        merge(&mut h, &pairs);
        assert_eq!(h.max_iteration(), 1);
        assert_eq!(h.len(), 1);

        // make both orders of pairs and see if they return the same value
        let pair1 = h.get((1, 2));
        let pair2 = h.get((2, 1));
        assert_eq!(pair1, Some(1));
        assert_eq!(pair2, Some(1));
        assert_eq!(h.len(), 1);
    }
}
