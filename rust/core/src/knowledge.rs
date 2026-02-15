use crate::argument::Argument;

/// Knowledge is not a static fact, but a negotiated, revisable structure
/// that emerges from arguments and disagreements under fair conditions.
#[derive(Debug, Clone)]
pub struct Knowledge {
    pub statement: String,
    pub disagreements: usize,
}

impl Knowledge {
    pub fn from_argument(arg: &Argument) -> Self {
        Self {
            statement: arg.claim.clone(),
            disagreements: arg.counterpoints.len(),
        }
    }

    /// Returns true when this knowledge is grounded in at least one disagreement.
    pub fn is_earned(&self) -> bool {
        self.disagreements > 0
    }
}
