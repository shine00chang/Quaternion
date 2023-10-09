use crate::game::*;


pub enum Mode {
    Norm,
    DS,
    Attack,
}


struct Factors {
    ideal_h: f32,
    well_threshold: f32,
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
    tspin_flat_bonus: f32,
    tspin_dist: f32,
    tspin_completeness: f32,
    average_h: f32,
    sum_attack: f32,
    sum_downstack: f32,
    attack: f32,
    downstack: f32,
    eff: f32,
} 


const WEIGHTS_ATK: Weights = Weights {
    hole: -100.0,
    hole_depth: -10.0,
    h_local_deviation: -3.0,
    h_global_deviation: -2.0,
    well_v: 1.0,
    well_parity: -3.0,
    well_odd_par: -30.0,
    well_flat_parity: 40.0,
    tspin_flat_bonus: 40.0,
    tspin_dist: -2.0,
    tspin_completeness: 4.0,
    average_h : 0.0,
    sum_attack: 25.0,
    sum_downstack: 15.0,
    attack: 50.0,
    downstack: 10.0,
    eff: 100.0,
};

const WEIGHTS_DS: Weights = Weights {
    hole: -150.0,
    hole_depth: -20.0,
    h_local_deviation: -10.0,
    h_global_deviation: -8.0,
    well_v: 0.0,
    well_parity: 0.0,
    well_odd_par: 0.0,
    well_flat_parity: 0.0,
    tspin_flat_bonus: -150.0, // Same as hole
    tspin_dist: 0.0,
    tspin_completeness: 0.0,
    average_h : -20.0,
    sum_attack: 0.0,
    sum_downstack: 35.0,
    attack: 0.0,
    downstack: 50.0,
    eff: 0.0,
};
const FACTORS_ATK: Factors = Factors {
    ideal_h: 0.0,
    well_threshold: 3.0,
};
const FACTORS_DS: Factors = Factors {
    ideal_h: 0.0,
    well_threshold: 20.0,
};

const DS_HEIGHT_THRESHOLD: f32 = 14.0;
const DS_HOLE_THRESHOLD  : u32 = 2;
const DS_MODE_PENALTY    : f32 = -400.0;
const WELL_PLACEMENT_F   : f32 = 70.0;
const WELL_PLACEMENT     : [f32; 10] = [-1.0, -1.0, 0.2, 1.2, 1.0, 1.0, 1.2, 0.2, -1.0, -1.0];


/// Heuristic Evaluation function
pub fn evaluate (state: &State, meta: MoveStats, mode: Mode) -> f32 {
    let mut score = 0.0;
    let b = &state.board;

    // Calc heights
    let h = b.v.map(|col| 32-col.leading_zeros());
    let avg_h = h.iter().sum::<u32>() as f32 / 10.0;

    let (holes, depth_sum_sq): (Vec<_>, Vec<_>) = b.v
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, mut col)| {
            let mut holes = 0;
            let mut depth = 0;

            // Detect each hole by shifting the column down.
            // If the first bit is not set, it should be a hole.
            for y in 0..h[i] {
                if col & 1 == 0 {
                    holes += 1;
                    depth += (h[i] - y) * (h[i] - y);
                }
                col >>= 1;
            }
            (holes, depth)
        })
        .unzip();

    let depth_sum_sq = depth_sum_sq.iter().sum::<u32>();
    let holes = holes.iter().sum::<u32>();

    // TODO: Find T-spin hole

    // Select weights
    // Will use DS if
    // -> holes
    // -> average height past threshold
    let (weights, factors) = {
        match mode {
            Mode::Norm =>
                if  avg_h >= DS_HEIGHT_THRESHOLD || holes >= DS_HOLE_THRESHOLD {
                    score += DS_MODE_PENALTY;
                    (WEIGHTS_DS, FACTORS_DS)
                } else {
                    (WEIGHTS_ATK, FACTORS_ATK)
                },
            Mode::DS => (WEIGHTS_DS, FACTORS_DS),
            Mode::Attack => (WEIGHTS_ATK, FACTORS_ATK),
        }
    };

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
    {
        let dh = (avg_h - factors.ideal_h).abs();
        score += weights.average_h * dh * dh;
    }

    // Score by dh from average
    {
        let sum_sq: f32 = h
            .iter()
            .enumerate()
            .map(|(x, h)| {
                // If x == well, ignore
                if x == well.unwrap_or_else(|| (100, 0.0)).0 {
                    0.0
                } else {
                    let dh = avg_h - *h as f32;
                    dh * dh
                }
            })
            .sum();

        // If tspin, compensate w/ [2, 1, 0]
        // TODO: T-spin compensation
        /*
        if tspin.is_some() {
            dev_log!("t-spin compensation: {}", 5.0 * weights.h_global_deviation);
            score -= 5.0 * weights.h_global_deviation;
        }
        */

        score += sum_sq * weights.h_global_deviation;
    }

    // Local Height Deviation (from neighbor)
    {
        let mut sum_sq: f32 = 0.0;
        let mut prev = None;

        for x in 0..10 {
            // Score well by height (not clear value)
            if x == well.unwrap_or_else(|| (100, 0.0)).0 { continue }
            
            // TODO: Ignore T-spin

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
    
    score
}
