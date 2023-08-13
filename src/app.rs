use tui_input::Input;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: Input,
    /// Current search value for pokemon
    pub pokemon_search: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            pokemon_search: "25".to_string(),
        }
    }
}
