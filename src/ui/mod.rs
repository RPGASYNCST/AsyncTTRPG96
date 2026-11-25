pub mod app;
pub mod models;
pub mod sprite;
pub mod theme;
pub mod update;
pub mod views;

#[cfg(test)]
mod tests;

pub use app::run_gui;
