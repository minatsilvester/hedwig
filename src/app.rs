use crate::models::{Request, RequestForm, Screen};

pub struct App {
    pub requests: Vec<Request>,
    pub selected: usize,
    pub screen: Screen,
    pub form: RequestForm,
    pub response: Option<String>, // store response text
}

impl App {
    pub fn new() -> Self {
        App {
            requests: Vec::new(),
            selected: 0,
            screen: Screen::Main,
            form: RequestForm::new(),
            response: None,
        }
    }

    pub fn next(&mut self) {
        if !self.requests.is_empty() {
            self.selected = (self.selected + 1) % self.requests.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.requests.is_empty() {
            if self.selected == 0 {
                self.selected = self.requests.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn add_request(&mut self, name: String, url: String, method: String) {
        self.requests.push(Request { name, url, method });
    }
}
