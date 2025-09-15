use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;
use crate::models::FormField;

pub fn draw_main(f: &mut Frame, app: &App) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(size);

    // Left panel: requests
    let items: Vec<ListItem> = app
        .requests
        .iter()
        .map(|req| {
            let method_style = match req.method.as_str() {
                "GET" => Style::default().fg(Color::Green),
                "POST" => Style::default().fg(Color::Blue),
                "PUT" => Style::default().fg(Color::Yellow),
                "DELETE" => Style::default().fg(Color::Red),
                _ => Style::default(),
            };
            let content = Line::from(vec![
                Span::styled(req.method.clone(), method_style),
                Span::raw(" "),
                Span::raw(req.name.clone()),
            ]);
            ListItem::new(content)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let requests_list = List::new(items)
        .block(Block::default().title(" Requests ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Reset)
                .add_modifier(Modifier::REVERSED | Modifier::BOLD),
        );

    f.render_stateful_widget(requests_list, chunks[0], &mut state);

    // Right panel split into response & details
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // Response
    let response_text = app
        .requests
        .get(app.selected)
        .and_then(|r| r.response.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("No response yet");

    let response_widget = Paragraph::new(response_text)
        .block(Block::default().title(" Response ").borders(Borders::ALL));
    f.render_widget(response_widget, right_chunks[0]);

    // Details
    let details = if let Some(req) = app.requests.get(app.selected) {
        format!(
            "Name: {}\nURL: {}\nMethod: {}",
            req.name, req.url, req.method
        )
    } else {
        "No request selected".to_string()
    };
    let details_widget =
        Paragraph::new(details).block(Block::default().title(" Details ").borders(Borders::ALL));
    f.render_widget(details_widget, right_chunks[1]);
}

pub fn draw_new_request(f: &mut Frame, app: &App) {
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
        String::from("Press Enter to confirm each field, Esc to cancel"),
    ];

    let form_widget = Paragraph::new(lines.join("\n"))
        .block(Block::default().title("New Request").borders(Borders::ALL));

    f.render_widget(form_widget, size);
}
