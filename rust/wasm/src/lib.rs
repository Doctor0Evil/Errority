use wasm_bindgen::prelude::*;
use errority-core::argument::Argument;
use errority-core::input::RawInput;
use errority-core::knowledge::Knowledge;

/// Errority WASM entry point.
/// Takes arbitrary text and returns a JSON-encoded knowledge candidate.
/// This lets JS honor the same philosophy: disagreement, fairness, right to exist.
#[wasm_bindgen]
pub fn transform_to_knowledge(input: &str) -> String {
    let raw = RawInput::new(input);
    let mut argument = Argument::from_input(&raw);

    // Synthetic disagreement; future versions can accept external counterpoints.
    argument.add_counterpoint(
        "I disagree by principle: every claim must face resistance to become knowledge.",
    );

    let knowledge = Knowledge::from_argument(&argument);

    serde_json::json!({
        "statement": knowledge.statement,
        "disagreements": knowledge.disagreements,
        "earned": knowledge.is_earned(),
    })
    .to_string()
}
