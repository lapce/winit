/// Dummy menu: there is no generic menu for web applications
pub struct Menu;

impl Menu {
    pub fn new() -> Menu {
        Menu
    }

    pub fn new_for_popup() -> Menu {
        Menu
    }

    pub fn add_dropdown(&mut self, _menu: Menu, _text: &str, _enabled: bool) {}

    pub fn add_item(&mut self, _id: u32, _text: &str, _selected: Option<bool>, _enabled: bool) {}

    pub fn add_separator(&mut self) {}
}
