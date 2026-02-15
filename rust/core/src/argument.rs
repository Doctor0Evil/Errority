use crate::input::RawInput;

/// An argument is a shaped form of input, suitable for disagreement and refinement.
/// Knowledge cannot form without the friction of opposing views.
#[derive(Debug, Clone)]
pub struct Argument {
    pub claim: String,
    pub counterpoints: Vec<String>,
}

impl Argument {
    pub fn from_input(input: &RawInput) -> Self {
        // Initial naive transformation; later we can enrich with NLP/ML.
        Self {
            claim: input.text.clone(),
            counterpoints: Vec::new(),
        }
    }

    pub fn add_counterpoint<T: Into<String>>(&mut self, text: T) {
        self.counterpoints.push(text.into());
    }

    pub fn has_disagreement(&self) -> bool {
        !self.counterpoints.is_empty()
    }
}
