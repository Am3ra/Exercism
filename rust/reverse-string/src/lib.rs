pub fn reverse(input: &str) -> String {
    std::iter::Iterator::collect::<String>(input.chars().rev())
}
