use super::types::BallType;

/// Cronologia di una singola estrazione
#[derive(Debug, Clone)]
pub struct DrawHistory {
    pub time: String,
    pub white_balls: usize,
    pub traits: Vec<usize>,
    pub red_balls: usize,
    pub misfortunes: [usize; 4],
    pub first_draw: Vec<BallType>,
    pub risked: bool,
    pub risk_draw: Vec<BallType>,
    pub confused: bool,
    pub adrenalined: bool,
}

impl DrawHistory {
    pub fn format_balls(&self, balls: &[BallType]) -> String {
        if balls.is_empty() {
            return String::from("-");
        }

        balls
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
