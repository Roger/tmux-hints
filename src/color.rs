#[derive(Debug, Default)]
pub struct Color {
    pub background: usize,
    pub foreground: usize,
    pub bold: bool,
    pub dim: bool,
    pub blink: bool,
    pub reverse: bool,
    pub hidden: bool,
    pub underlined: bool,
}

impl Color {
    pub fn paint(&self, text: &str) -> String {
        let colorstr = format!("\x1B[38;5;{}m\x1B[48;5;{}m", self.foreground, self.background);
        let mut attrs = String::new();

        if self.bold {
            attrs.push_str("1;");
        }
        if self.dim {
            attrs.push_str("3;");
        }
        if self.underlined {
            attrs.push_str("4;");
        }
        if self.blink {
            attrs.push_str("5;");
        }
        if self.reverse {
            attrs.push_str("7;");
        }
        if self.hidden {
            attrs.push_str("8;");
        }
        if !attrs.is_empty() {
            attrs = format!("\x1B[{}m", attrs.trim_matches(';'));
        }

        format!("{}{}{}\x1B[0m", colorstr, attrs, text)
    }
}
