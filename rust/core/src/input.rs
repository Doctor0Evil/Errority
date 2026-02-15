#[derive(Debug, Clone)]
pub struct RawInput {
    pub text: String,
}

impl RawInput {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}
