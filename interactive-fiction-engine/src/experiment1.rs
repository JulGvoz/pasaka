struct PassageResult<T> {
    text: String,

    data: T,
}

struct Choice<T> {
    label: String,
    next: fn(&mut T) -> PassageResult<T>,
}

trait Passage {}

impl<T> Passage for fn(&mut T) -> PassageResult<T> {}
