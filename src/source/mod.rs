#[derive(Debug)]
pub struct Source {
    pub file_name: String,
    pub text: String,
    pub lines: Vec<usize>,
}

impl Source {
    pub fn new(file_name: String, text: String) -> Source {
        let lines = find_lines(&text);
        Source {
            file_name,
            text,
            lines,
        }
    }

    // Returns line number/index, for the given offset
    pub fn get_line(&self, offset: usize) -> usize {
        self.lines.binary_search(&offset).unwrap_or_else(|i| i - 1)
    }

    pub fn get_line_column(&self, offset: usize) -> (usize, usize) {
        let line = self.get_line(offset);
        let start = self.lines[line];
        let column = self.text[start..offset].chars().count();
        (line, column)
    }

    pub fn get_line_slice(&self, line: usize) -> &str {
        let start = self.lines[line];
        let &end = self.lines.get(line + 1).unwrap_or(&self.text.len());
        &self.text[start..end]
    }
}

// Returns a vector of indices of line beginnings
fn find_lines(text: &str) -> Vec<usize> {
    let mut lines = vec![0];
    for (i, c) in text.char_indices() {
        if c == '\n' {
            lines.push(i + 1);
        }
    }
    lines
}
