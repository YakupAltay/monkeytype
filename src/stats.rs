use serde::Serialize;

#[derive(Serialize)]
pub struct SessionStats {
    pub typed: usize,
    pub correct: usize,
    pub accuracy: f64,
    pub wpm: f64,
}
