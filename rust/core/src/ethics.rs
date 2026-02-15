/// Ethics primitives for Errority.
///
/// FEAR  = perception of uncertainty
/// PAIN  = signal of transformation
/// GREED = unbalanced desire to control or consume
/// ANGER = reactive distortion against perceived unfairness
///
/// The engine treats FEAR and PAIN as sources of learning,
/// and regulates GREED and ANGER so that no actor may deny
/// another's right to exist or argue for fairness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Fear,
    Pain,
    Greed,
    Anger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FairnessState {
    /// A state where all inputs are allowed to exist and be argued about.
    Open,
    /// A state where some voices or inputs are suppressed.
    Suppressed,
}

#[derive(Debug, Clone)]
pub struct EthicsContext {
    pub fairness: FairnessState,
    pub allow_disagreement: bool,
}

impl EthicsContext {
    pub fn new() -> Self {
        Self {
            fairness: FairnessState::Open,
            allow_disagreement: true,
        }
    }

    /// Interpret an incoming signal and adjust fairness if necessary.
    pub fn process_signal(&mut self, signal: Signal) {
        match signal {
            Signal::Fear | Signal::Pain => {
                // Treated as opportunities to learn, not reasons to silence.
                self.fairness = FairnessState::Open;
                self.allow_disagreement = true;
            }
            Signal::Greed | Signal::Anger => {
                // Prevent these from dictating who is allowed to exist or speak.
                if self.fairness == FairnessState::Open {
                    self.allow_disagreement = true;
                }
            }
        }
    }

    /// Returns true when the system considers it fair to allow this input to exist.
    pub fn permits_existence(&self) -> bool {
        matches!(self.fairness, FairnessState::Open)
    }
}
