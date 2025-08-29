struct PassageResult {
    text: String,
    choices: Vec<()>,
}

struct Choice {
    label: String,
    next: fn() -> ChoiceResult,
}

// struct ChoiceResult<T> {
//     state: T,
//     passage: fn(&mut T) -> PassageResult,
// }

struct ChoiceResult {
    state: Box<dyn std::any::Any>,
    next_passage: fn(&mut dyn std::any::Any) -> PassageResult,
}
