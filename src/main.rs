mod app;
mod input;
mod models;
mod ui;

use app::App;
use input::run_app;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    run_app(&mut app)
}
