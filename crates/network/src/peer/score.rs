use serde::Serialize;

pub(crate) const DEFAULT_SCORE: f64 = 0.0;
const MAX_SCORE: f64 = 100.0;

#[derive(Clone, Debug, Serialize)]
pub struct PeerScore {
    score: f64,
}

impl Default for PeerScore {
    fn default() -> Self {
        PeerScore {
            score: DEFAULT_SCORE,
        }
    }
}

impl PeerScore {
    pub fn max() -> Self {
        PeerScore { score: MAX_SCORE }
    }
}
