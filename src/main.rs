use AsyncTTRPG96::ui::run_gui;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Iniciando AsyncTTRPG96");
    run_gui()
}
