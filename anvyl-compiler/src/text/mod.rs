pub struct SourceText {
    text: String,
}

impl SourceText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn line_index(&self, position: usize) -> usize {
        let count = self.text[..position].lines().count();
        match count {
            0 => 0,
            _ => count - 1,
        }
    }

    pub fn get_line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap()
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text
            .lines()
            .take(index)
            .map(|line| line.len() + 1)
            .sum()
    }
}
