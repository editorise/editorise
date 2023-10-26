use std::collections::HashMap;

pub struct BufferOptions {
    pub show_info_column: bool,
    pub show_border: bool,
    pub chars: HashMap<char, char>,
}

impl Default for BufferOptions {
    fn default() -> Self {
        let mut options = Self {
            show_info_column: true,
            show_border: false,
            chars: HashMap::new(),
        };
        options.chars.insert(' ', '•');
        options
    }
}

impl BufferOptions {
    pub fn replace_chars(&self, text: &str) -> String {
        let mut result = String::new();

        for c in text.chars() {
            if let Some(m) = self.chars.get(&c) {
                result.push(m.clone());
            } else {
                result.push(c);
            }
        }

        result
    }
}
