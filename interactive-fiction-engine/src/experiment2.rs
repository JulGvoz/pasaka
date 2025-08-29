struct PassageResult {
    text: String,
    choices: Vec<()>,
}

struct Choice {
    label: String,
    next: fn() -> Box<dyn ChoiceResult>,
}

// struct ChoiceResult<T> {
//     state: T,
//     passage: fn(&mut T) -> PassageResult,
// }

trait ChoiceResult {}
