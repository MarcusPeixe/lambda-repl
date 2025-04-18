#[derive(Debug)]
pub struct Source {
    pub file_name: Option<String>,
    pub text: String,
    pub lines: Vec<usize>,
}

impl Source {
    pub fn from_file(file_name: String) -> std::io::Result<Source> {
        // Read source code
        let text = std::fs::read_to_string(&file_name)?;

        let lines = find_lines(&text);
        Ok(Source {
            file_name: Some(file_name),
            text,
            lines,
        })
    }

    pub fn from_string(text: String) -> Source {
        let lines = find_lines(&text);
        Source {
            file_name: None,
            text,
            lines,
        }
    }

    // Returns line number/index, for the given offset
    pub fn get_line(&self, offset: usize) -> usize {
        self.lines.partition_point(|&line| line <= offset) - 1
    }

    pub fn get_line_column(&self, offset: usize) -> (usize, usize) {
        let line = self.get_line(offset);
        let start = self.lines[line];
        let column = self.text[start..offset].chars().count();
        (line, column)
    }

    pub fn get_line_offset(&self, line: usize) -> (usize, usize) {
        let start = self.lines[line];
        let &end = self.lines.get(line + 1).unwrap_or(&self.text.len());
        (start, end)
    }
}

// Returns a vector of indices of line beginnings
fn find_lines(text: &str) -> Vec<usize> {
    text.lines()
        .map(|line| line.as_ptr() as usize - text.as_ptr() as usize)
        .collect()
}
