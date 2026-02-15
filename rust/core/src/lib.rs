pub mod ethics;
pub mod argument;
pub mod knowledge;
pub mod input;

/// Errority
/// Pronunciation: /ˈɛr.ɔːˌrɪ.ti/  (EH-raw-rih-tee)
/// Concept:
/// - Every input has potential value, no matter how pointless or worthless it seems.
/// - Knowledge arises from disagreement, argument, and the struggle for fair coexistence.
/// - Fairness is the earned right to exist freely, without domination by fear, pain, greed, or anger.
///
/// This crate exposes the core traits and types that other crates (CLI, WASM, services)
/// use to transform arbitrary user inputs into structured, disputable, and improvable knowledge.
pub struct ErrorityCore;

impl ErrorityCore {
    pub fn new() -> Self {
        Self
    }
}
