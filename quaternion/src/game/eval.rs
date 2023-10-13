#[cfg(test)]
mod tests;

use crate::game::*;


pub enum Mode {
    Norm,
    DS,
    Attack,
}


struct Factors {
    ideal_h: f32,
    well_threshold: f32,
    hole_depth_relevancy_threshold: u32 // holes deeper than this are ignored, since they
                                        // aren't important in the near future.
}

struct Weights {
    hole: f32,
    hole_depth: f32,
    h_local_deviation: f32,
    h_global_deviation: f32,
    well_v: f32,
    well_parity: f32,
    well_odd_par: f32,
    well_flat_parity: f32,
    tspin_bonus: f32,
    tspin_score: f32,
    average_h: f32,
    attack: f32,
    downstack: f32,
    eff: f32,
} 

const WEIGHTS_ATK: Weights = Weights {
    hole: -150.0,
    hole_depth: -12.0,
    h_local_deviation: -6.0,
    h_global_deviation: -5.0,
    well_v: 0.0,
    well_parity: 0.0,
    well_odd_par: 0.0,
    well_flat_parity: 0.0,
    tspin_bonus: 25.0,
    tspin_score: 25.0,
    average_h : 0.0,
    attack: 100.0,
    downstack: 10.0,
    eff: 130.0,
};

const WEIGHTS_DS: Weights = Weights {
    hole: -150.0,
    hole_depth: -10.0,
    h_local_deviation: -20.0,
    h_global_deviation: -8.0,
    well_v: 0.0,
    well_parity: 0.0,
    well_odd_par: 0.0,
    well_flat_parity: 0.0,
    tspin_bonus: 0.0,
    tspin_score: 0.0,
    average_h : -20.0,
    attack: 0.0,
    downstack: 100.0,
    eff: 0.0,
};

const FACTORS_ATK: Factors = Factors {
    ideal_h: 0.0,
    well_threshold: 3.0,
    hole_depth_relevancy_threshold: 6
};

const FACTORS_DS: Factors = Factors {
    ideal_h: 0.0,
    well_threshold: 20.0,
    hole_depth_relevancy_threshold: 6
};

const DS_HEIGHT_THRESHOLD: f32 = 10.0;
const DS_MODE_PENALTY    : f32 = 0.0;
const WELL_PLACEMENT_F   : f32 = 70.0;
const WELL_PLACEMENT     : [f32; 10] = [-0.5, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, -0.5];


struct Tspin {
    x: usize,
    overhang: bool,
    rows: u8,
}

#[derive(Default)]
struct Tspins {
    vx: Vec<usize>,
    pub overhangs: u8,
    pub score: u8
}

impl Tspins {
    fn count (&self) -> u32 {
        self.vx.len() as u32
    }

    fn contains (&self, x: usize) -> bool {
        self.vx.contains(&x)
    }

    fn find (board: &Board) -> Self {
        let mut out = Tspins::default();

        for y in 0..20 {
            for x in 0..10 {
                if let Some(tspin) = Self::is_tspin(&board, x, y) {
                    out.vx.push(tspin.x);
                    if tspin.overhang {
                        out.overhangs += 1;
                    }
                    out.score += tspin.rows + if tspin.overhang { 1 } else { 0 };
                }
            }
        }

        out
    }

    fn is_tspin (board: &Board, x: i32, y: i32) -> Option<Tspin> {
        if x <= 0 || x >= 9 || y == 0 {
            return None;
        }

        // Preliminary checks, check for shape.
        let prelim = 
            !board.occupied(x, y) &&
            !board.occupied(x, y-1) &&
            !board.occupied(x, y+1) &&
            !board.occupied(x-1, y) &&
            !board.occupied(x+1, y) &&
            (!board.occupied(x-1, y+1) || !board.occupied(x+1, y+1)) &&
            board.occupied(x-1, y-1) &&
            board.occupied(x+1, y-1);

        if !prelim { 
            return None;
        }

        // Check if overhang exists
        let overhang = board.occupied(x-1, y+1) ^ board.occupied(x+1, y+1);

        // Check for access (Harms dono's)
        let x = x as usize;
        let access =
            if board.v[x] >> y == 0 { 1 } else { 0 } +
            if board.v[x-1] >> y == 0 { 1 } else { 0 } +
            if board.v[x+1] >> y == 0 { 1 } else { 0 };

        // if there are less than two accessible columns OR
        // if there are only two but without an overhang => improper overhang
        if access < 2 || access == 2 && !overhang {
            return None;
        }

        // Check if rows are filled
        let row1 = board.v
            .iter()
            .enumerate()
            .fold(true, |a, (i, col)| 
                if i >= x-1 && i <= x+1 {
                    a
                } else {
                    a && (col & 1 << y != 0)
                }
            );
        let row2 = board.v
            .iter()
            .enumerate()
            .fold(true, |a, (i, col)| 
                if i == x {
                    a
                } else {
                    a && (col & 1 << (y-1) != 0)
                }
            );

        if !row1 && !row2 {
            return None
        }

        Some( Tspin { 
            x,
            overhang,
            rows: if row1 { 1 } else { 0 } + if row2 { 1 } else { 0 },
        } )
    }
}

/// Heuristic Evaluation function
pub fn evaluate (state: &State, meta: MoveStats, mode: Mode) -> f32 {


    let mut score = 0.0;
    let b = &state.board;

    // Find T-spin
    let tspins = Tspins::find(&state.board);

    // Calc heights
    let h = b.v.map(|col| 32-col.leading_zeros());
    let avg_h = h.iter().sum::<u32>() as f32 / 10.0;

    // Select weights
    // Will use DS if average height past threshold
    let (weights, factors) = {
        match mode {
            Mode::Norm =>
                if  avg_h >= DS_HEIGHT_THRESHOLD {
                    score += DS_MODE_PENALTY;
                    (WEIGHTS_DS, FACTORS_DS)
                } else {
                    (WEIGHTS_ATK, FACTORS_ATK)
                },
            Mode::DS => (WEIGHTS_DS, FACTORS_DS),
            Mode::Attack => (WEIGHTS_ATK, FACTORS_ATK),
        }
    };

    let (holes, depth_sum_sq): (u32, u32) = b.v
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, mut col)| {
            if tspins.contains(i + 1) || (i > 0 && tspins.contains(i - 1)) {
                return (0, 0)
            }

            let mut holes = 0;
            let mut depth = 0;
            let mut maxd = None;

            // Detect each hole by shifting the column down.
            // If the first bit is set and the second is not, it should be a hole.
            for y in 0..h[i] {
                if col & 1 == 0 {
                    maxd = Some( maxd.unwrap_or(0).max(h[i] - y) );
                } else if let Some(d) = maxd {
                    if h[i] - y > factors.hole_depth_relevancy_threshold {
                        continue;
                    }
                    depth += d * d;
                    holes += 1;

                    maxd = None;
                }
                col >>= 1;
            }

            (holes, depth)
        })
        .fold((0,0), |a, t| (a.0 + t.0, a.1 + t.1));

    // Score by Tspins
    score += tspins.count() as f32 * weights.tspin_bonus;
    score += tspins.score as f32 * weights.tspin_score;

    // Score by holes & depth (split from calculation because weight selection requires hole info)
    score += holes as f32 * weights.hole;
    score += depth_sum_sq as f32 * weights.hole_depth;

    // Find well (max negative deviation from avg_h > than threshold)
    let well = {
        let mut well = None;
        for x in 0..10 {
            let dh = avg_h - h[x] as f32;
            if dh >= factors.well_threshold {
                
                let well = well.get_or_insert((x, dh));
                if dh > well.1 {
                    *well = (x, dh);
                }
            }
        }
        well
    };

    // Remove well height from average height
    let avg_h = if let Some(well) = well {
        (avg_h * 10.0 - h[well.0] as f32) / 9.0 
    } else { avg_h };

    // Score by avg_h height
    // Must consider this score AFTER the well has been removed from the average.
    let dh = (avg_h - factors.ideal_h).abs();
    score += weights.average_h * dh * dh;

    // Score by dh from average
    {
        let sum_sq: f32 = h
            .iter()
            .enumerate()
            .map(|(x, h)| {
                // If well, ignore
                if x == well.unwrap_or_else(|| (100, 0.0)).0 {
                    0.0
                } else {
                    let dh = avg_h - *h as f32;
                    dh * dh
                }
            })
            .sum();

        score += sum_sq * weights.h_global_deviation;
    }

    // Local Height Deviation (from neighbor)
    {
        let mut sum_sq: f32 = 0.0;
        let mut prev = None;

        for x in 0..10 {
            // Score well by height (not clear value)
            if x == well.unwrap_or_else(|| (100, 0.0)).0 { continue }
            
            // Ignore T-spin
            //if tspins.contains(x) { continue }

            if let Some(prev) = prev {
                let d = h[x].abs_diff(prev);
                sum_sq += (d * d) as f32;
            }

            prev = Some(h[x]);
        }

        score += sum_sq * weights.h_local_deviation;
    }
    
    // === Well-related properties
    if let Some((well_x, _)) = well {
        // Well value: Number of lines clearable in well
        // '&' together all columns except for well, count ones.
        let well_value = b.v
            .iter()
            .enumerate()
            .fold(!0, |a, (x, col)| 
                if x == well_x { a } else { a & col }
            )
            .count_ones();

        score += well_value as f32 * weights.well_v;
        score += WELL_PLACEMENT_F * WELL_PLACEMENT[well_x];

        // Parity: penalize large parity diffs, bonus for flat well.
        let d = (if well_x != 0 {h[well_x-1]} else {h[well_x+1]}).abs_diff(if well_x != 9 {h[well_x+1]} else {h[well_x-1]});

        score +=  (d * d) as f32 * weights.well_parity;

        if d % 2 == 1 { score += weights.well_odd_par };
        if d == 0 { score += weights.well_flat_parity }

        // Tspins: 
        // > Subtract one from delta, due to inherent odd parity.
        // > Promote an even-residue overhang for better contiuation.
        // TODO: 
        /*
        if let Some(tspin) = tspin { if tspin.2 == w {
            score -= 4.0 * weights.well_parity;
            if d == 3 { score += weights.well_flat_parity }
        }}
        */
    }

    // clear and attack
    score += (meta.attacks as i32 - meta.ds as i32) as f32 * weights.eff;
    score += meta.attacks as f32 * weights.attack;
    score += meta.ds as f32 * weights.downstack;
    
    // if meta.tspin && meta.attacks == 4 {
    //     println!("tspin found with attack: {}, score: {}", meta.attacks, score);
    // }
    score
}
