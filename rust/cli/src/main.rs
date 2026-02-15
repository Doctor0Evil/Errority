use rust_core::argument::Argument;
use rust_core::ethics::EthicsContext;
use rust_core::input::RawInput;
use rust_core::knowledge::Knowledge;

/// Simple CLI demo:
/// - Accepts a line of text as "pointless" input
/// - Wraps it as an Argument
/// - Adds a synthetic counterpoint to ensure disagreement
/// - Produces Knowledge that is "earned" by argument
fn main() {
    let input_text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if input_text.is_empty() {
        eprintln!("Usage: errority \"your input text\"");
        return;
    }

    let raw = RawInput::new(input_text);
    let mut ethics = EthicsContext::new();

    // Fear/pain are treated as triggers to keep the channel open.
    ethics.process_signal(rust_core::ethics::Signal::Fear);
    ethics.process_signal(rust_core::ethics::Signal::Pain);

    if !ethics.permits_existence() {
        eprintln!("Input was suppressed by ethics context (should not happen in Open state).");
        return;
    }

    let mut arg = Argument::from_input(&raw);
    // Placeholder disagreement: in a real engine, this would come from other agents or models.
    arg.add_counterpoint("I disagree, and here is why: the concept can be refined further.");

    let knowledge = Knowledge::from_argument(&arg);

    println!("Errority input: {}", knowledge.statement);
    println!("Disagreements recorded: {}", knowledge.disagreements);
    println!(
        "Knowledge is {}earned by disagreement.",
        if knowledge.is_earned() { "" } else { "not " }
    );
}
