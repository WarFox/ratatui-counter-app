// App State
#[derive(Default)]
pub struct App {
    counter: u8,
    should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn counter(&self) -> u8 {
        return self.counter;
    }

    pub fn should_quit(&self) -> bool {
        return self.should_quit;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::default();
        assert_eq!(app.counter(), 0);
        assert_eq!(app.should_quit(), false);
    }

    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);

        app.increment_counter();
        assert_eq!(app.counter, 2);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
