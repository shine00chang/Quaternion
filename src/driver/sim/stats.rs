use std::time::Instant;

use crate::colors::*;

// Tracks: Over time, over iterations
pub struct Stats {
    iters: u64,
    start_time: Instant,
    vals: Vec<(&'static str, Vec<AverageMethod>, f64)>,
}

enum AverageMethod {
    Move,
    Second,
    Minute,
}
use AverageMethod::*;

impl Stats {
    pub fn new() -> Self {
        Self {
            iters: 0, 
            start_time: Instant::now(),
            vals: vec![
                ("nodes", vec![Move], 0.0),
                ("pieces", vec![Second], 0.0),
                ("attacks", vec![Minute, Move], 0.0)
            ]
        }
    }

    fn get (&mut self, k: &str) -> Option<&mut f64> { 
        for i in 0..self.vals.len() {
            if self.vals[i].0 == k {
                return Some(&mut self.vals[i].2)
            }
        }
        None
    }
}
impl std::fmt::Display for Stats {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        let seconds = self.start_time.elapsed().as_secs() as f64;
        write!(f, "{BLD} == Stats == {RST}\n")?;

        for (k, tv, s) in self.vals.iter() {
            for t in tv {
                let suffix = match t {
                    Move => "/i",
                    Minute => "/m",
                    Second => "/s",
                };

                let v = match t {
                    Move    => s / self.iters as f64,
                    Minute  => s / seconds * 60.0,
                    Second  => s / seconds,
                };

                write!(f, "{k:<8}{suffix} : {:.3}\n", v)?;
            }
        } 
        Ok(())
    }
}

impl Stats {
    pub fn accumulate (&mut self, state: &quaternion::game::State, bot_state: &quaternion::worker::State) {

        *self.get("nodes").unwrap() += bot_state.nodes as f64;
        *self.get("pieces").unwrap() += 1.0;
        *self.get("attacks").unwrap() += state.props.atk as f64;

        self.iters += 1;
    }
}
