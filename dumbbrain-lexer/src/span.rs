#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub first_line: usize,
    pub first_column: usize,
    pub last_line: usize,
    pub last_column: usize,
}
