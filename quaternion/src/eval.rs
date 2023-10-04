use crate::game::*;

pub enum Mode {
    Norm,
    DS,
    Attack,
}
use Mode::*;

/// Heuristic Evaluator function. Currently just wraps Tetron's Evaluator.
pub fn evaluate (state: &State, mode: Mode) -> f32 {
    let mode = match mode {
        DS     => tetron::EvaluatorMode::DS,
        Norm   => tetron::EvaluatorMode::Norm,
        Attack => tetron::EvaluatorMode::Attack
    };

    tetron::evaluate(state, mode);
    0.0
}
