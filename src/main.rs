mod app;
mod math;
mod physics;
mod render;
mod terminal;
mod ui;

fn main() -> std::io::Result<()> {
    terminal::run()
}
