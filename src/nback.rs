use super::cue::Cell;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::VecDeque;

pub struct Score {
    false_pos: usize,
    true_pos: usize,
    false_neg: usize,
    true_neg: usize,
}

impl Score {
    pub fn record_fp(&mut self) {
        self.false_pos += 1;
    }

    pub fn record_tp(&mut self) {
        self.true_pos += 1;
    }

    pub fn record_fn(&mut self) {
        self.false_neg += 1;
    }

    pub fn record_tn(&mut self) {
        self.true_neg += 1;
    }

    pub fn correct(&self) -> usize {
        self.true_pos + self.true_neg
    }

    pub fn wrong(&self) -> usize {
        self.false_pos + self.false_neg
    }

    pub fn f1_score(&self) -> f32 {
        if self.true_pos + self.false_neg == 0 {
            1.0
        } else {
            self.true_pos as f32
                / (self.true_pos as f32 + 0.5 * (self.false_pos as f32 + self.false_neg as f32))
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Score {
            false_pos: 0,
            true_pos: 0,
            false_neg: 0,
            true_neg: 0,
        }
    }
}

pub struct GameState {
    pub score: Score,
    pub cues: CueChain<Cell>,
}

impl GameState {
    pub fn restart(&mut self) {
        self.score = Default::default();
        self.cues = CueChain::with_n_back(self.cues.n_back());
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: Default::default(),
            cues: Default::default(),
        }
    }
}

/// Memorization and generation of new cues.
pub struct CueChain<T> {
    short_memory: VecDeque<T>,
}

impl<T: Default> Default for CueChain<T> {
    fn default() -> Self {
        CueChain::with_n_back(2)
    }
}

impl<T: Default> CueChain<T> {
    pub fn with_n_back(n: usize) -> Self {
        let mut cc = CueChain {
            short_memory: VecDeque::new(),
        };

        for _ in 0..n + 1 {
            cc.short_memory.push_front(Default::default());
        }

        cc
    }

    pub fn n_back(&self) -> usize {
        self.short_memory.len() - 1
    }
}

impl<T> CueChain<T>
where
    Standard: Distribution<T>,
    T: Clone + PartialEq + Default,
{
    pub fn gen(&mut self) -> T {
        let mut rng = rand::thread_rng();
        let y = rng.gen::<f64>();

        let cue = if y < 0.25 && *self.short_memory.front().unwrap() != Default::default() {
            self.short_memory.front().unwrap().clone()
        } else {
            rand::random()
        };

        self.short_memory.push_back(cue);
        self.short_memory.pop_front();

        (*self.short_memory.back().unwrap()).clone()
    }
}

impl<T: PartialEq + Default> CueChain<T> {
    pub fn is_match(&self) -> bool {
        if self.short_memory.front() != Some(&Default::default()) {
            self.short_memory.back() == self.short_memory.front()
        } else {
            false
        }
    }
}
