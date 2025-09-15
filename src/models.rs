#[derive(Debug, Clone)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: String,
}

pub struct RequestForm {
    pub name: String,
    pub url: String,
    pub method: String,
    pub field: FormField,
}

impl RequestForm {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            method: String::new(),
            field: FormField::Name,
        }
    }
}

impl Default for RequestForm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Main,
    NewRequest,
}

#[derive(Debug, Clone, Copy)]
pub enum FormField {
    Name,
    Url,
    Method,
}
