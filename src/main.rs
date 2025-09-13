use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

#[derive(Debug, Clone)]
struct Request {
    name: String,
    url: String,
    method: String,
}

// impl Request {
//     fn new(name: String, url: String, method: String) -> Self {
//         Request { name, url, method }
//     }
// }

struct App {
    requests: Vec<Request>,
    selected: usize,
    screen: Screen,
    form: RequestForm,
}

impl App {
    fn new() -> Self {
        App {
            requests: Vec::new(),
            selected: 0,
            screen: Screen::Main,
            form: RequestForm::new(),
        }
    }

    fn next(&mut self) {
        if !self.requests.is_empty() {
            self.selected = (self.selected + 1) % self.requests.len();
        }
    }

    fn prev(&mut self) {
        if !self.requests.is_empty() {
            if self.selected == 0 {
                self.selected = self.requests.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    fn add_request(&mut self, name: String, url: String, method: String) {
        self.requests.push(Request { name, url, method });
    }
}

#[derive(Debug, Clone, Copy)]
enum Screen {
    Main,
    NewRequest,
}

struct RequestForm {
    name: String,
    url: String,
    method: String,
    field: FormField,
}

impl RequestForm {
    fn new() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            method: String::new(),
            field: FormField::Name,
        }
    }
}

enum FormField {
    Name,
    Url,
    Method,
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| match app.screen {
            Screen::Main => draw_main(f, &app),
            Screen::NewRequest => draw_new_request(f, &app),
        })?;

        // handle input
        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Main => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('a') => {
                        app.screen = Screen::NewRequest;
                        app.form = RequestForm::new();
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.prev(),
                    _ => {}
                },
                Screen::NewRequest => match key.code {
                    KeyCode::Esc => {
                        app.screen = Screen::Main;
                    }
                    KeyCode::Enter => match app.form.field {
                        FormField::Name => {
                            app.form.field = FormField::Url;
                        }
                        FormField::Url => app.form.field = FormField::Method,
                        FormField::Method => {
                            app.add_request(
                                app.form.name.clone(),
                                app.form.url.clone(),
                                app.form.method.clone(),
                            );
                            app.screen = Screen::Main;
                        }
                    },
                    KeyCode::Char(c) => match app.form.field {
                        FormField::Name => {
                            app.form.name.push(c);
                        }
                        FormField::Url => {
                            app.form.url.push(c);
                        }
                        FormField::Method => {
                            app.form.method.push(c);
                        }
                    },
                    KeyCode::Backspace => match app.form.field {
                        FormField::Name => {
                            app.form.name.pop();
                        }
                        FormField::Url => {
                            app.form.url.pop();
                        }
                        FormField::Method => {
                            app.form.method.pop();
                        }
                    },
                    _ => {}
                },
            }
        }
    }
}

fn draw_main(f: &mut Frame, app: &App) {
    let size = f.area();

    // Split screen horizontally into 2 panels
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(40), // Left panel (requests)
                Constraint::Percentage(60), // Right panel (responses)
            ]
            .as_ref(),
        )
        .split(size);

    let items: Vec<ListItem> = app
        .requests
        .iter()
        .map(|request| {
            let content = format!("{} - {} - {}", request.name, request.url, request.method);
            ListItem::new(content)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let requests_list = List::new(items)
        .block(Block::default().title("Requests").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    // Left panel: requests
    // let requests_panel = Block::default().title("Requests").borders(Borders::ALL);
    f.render_stateful_widget(requests_list, chunks[0], &mut state);

    // Right panel: responses
    let responses_panel = Block::default().title("Responses").borders(Borders::ALL);
    f.render_widget(responses_panel, chunks[1]);
}

fn draw_new_request(f: &mut Frame, app: &App) {
    let size = f.area();

    let lines = vec![
        format!(
            "Name:  {}{}",
            app.form.name,
            if matches!(app.form.field, FormField::Name) {
                " <"
            } else {
                ""
            }
        ),
        format!(
            "URL:   {}{}",
            app.form.url,
            if matches!(app.form.field, FormField::Url) {
                " <"
            } else {
                ""
            }
        ),
        format!(
            "Method:    {}{}",
            app.form.method,
            if matches!(app.form.field, FormField::Method) {
                " <"
            } else {
                ""
            }
        ),
        String::from("Press Enter to confirm each field, esc to cancel"),
    ];

    let form_widget = Paragraph::new(lines.join("\n"))
        .block(Block::default().title("New Request").borders(Borders::ALL));

    f.render_widget(form_widget, size);
}
