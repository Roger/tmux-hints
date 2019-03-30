use crate::settings::Settings;
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

    fn paint(&self, pos_color: &Color, text_color: &Color) {
        let pos = utils::cursor_at(self.screen_x, self.screen_y);

        if Settings::global().show_position {
            let prefix = pos_color.paint(&self.prefix);
            let text = &self.text[self.prefix.len()..self.text.len()];
            print!("{}{}{}", pos, prefix, text_color.paint(&text));
        } else {
            print!("{}{}", pos, text_color.paint(&self.text));
        }

    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    pub fn select(&self) {
        let settings = Settings::global();
        let text_color = &settings.hint.selected;
        let pos_color = &settings.position.selected;

        self.paint(pos_color, text_color);
    }

    pub fn unselect(&self) {
        let settings = Settings::global();
        let text_color = &settings.hint.unselected;
        let pos_color = &settings.position.unselected;

        self.paint(pos_color, text_color);
    }
}
