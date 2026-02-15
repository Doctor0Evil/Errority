/// Raw user input, which might appear pointless, chaotic, or incomplete.
/// Errority does not discard this; it treats it as the seed of structure.
#[derive(Debug, Clone)]
pub struct RawInput {
    pub text: String,
}

impl RawInput {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}
