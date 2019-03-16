use crate::color::Color;
use crate::utils;

#[derive(Debug)]
pub struct Hint {
    pub text: String,
    prefix: String,
    screen_x: usize,
    screen_y: usize,
}

/// Hint represents a match in the screen
impl Hint {
    /// Returns a Hint
    ///
    /// # Example
    ///
    /// ```
    /// use hint::Hint;
    /// let hint = Hint::new("http://www.google.com", 10, 4);
    /// ```
    pub fn new(text: String, screen_x: usize, screen_y: usize) -> Hint {
        Hint {
            prefix: String::new(),
            text,
            screen_x,
            screen_y,
        }
    }

    fn paint(&self, color: &Color) {
        let pos = utils::cursor_at(self.screen_x, self.screen_y);
        let text = &self.text[self.prefix.len()..self.text.len()];
        print!("{}{}{}", pos, self.prefix, color.paint(&text));
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    pub fn select(&self) {
        let color = Color {
            foreground: 2,
            background: 16,
            blink: true,
            ..Default::default()
        };
        self.paint(&color);
    }

    pub fn unselect(&self) {
        let color = Color {
            foreground: 2,
            background: 18,
            ..Default::default()
        };
        self.paint(&color);
    }
}
