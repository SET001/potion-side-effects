use app::get_app;

pub mod app;
pub mod config;
pub mod core;
pub mod pawns;
pub mod states;

fn main() {
  get_app().run();
}
