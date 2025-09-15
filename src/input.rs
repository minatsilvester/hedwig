use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::app::App;
use crate::models::{FormField, Screen};
use crate::ui::{draw_main, draw_new_request};

pub fn run_app(app: &mut App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = loop {
        terminal.draw(|f| match app.screen {
            Screen::Main => draw_main(f, app),
            Screen::NewRequest => draw_new_request(f, app),
        })?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Main => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Char('a') => {
                        app.screen = Screen::NewRequest;
                        app.form = Default::default();
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.prev(),
                    _ => {}
                },
                Screen::NewRequest => match key.code {
                    KeyCode::Esc => app.screen = Screen::Main,
                    KeyCode::Enter => match app.form.field {
                        FormField::Name => app.form.field = FormField::Url,
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
                        FormField::Name => app.form.name.push(c),
                        FormField::Url => app.form.url.push(c),
                        FormField::Method => app.form.method.push(c),
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
    };

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
