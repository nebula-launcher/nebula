use std::error::Error;

pub mod gui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    gui::app::init_gui();

    Ok(())
}