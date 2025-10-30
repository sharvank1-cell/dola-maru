mod core;
mod cli;
mod gui;

use anyhow::Result;
use clap::Parser;
use std::sync::{Arc, Mutex};
use std::fs;

use crate::core::repository::RepoConfig;
use crate::cli::runner::run_cli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Commit message
    #[clap(short, long, default_value = "Auto commit")]
    message: String,

    /// Branch name
    #[clap(short, long, default_value = "main")]
    branch: String,
    
    /// Run in GUI mode
    #[clap(long, action)]
    gui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load repository configuration
    let config = load_repo_config()?;
    let config_arc = Arc::new(Mutex::new(config));
    
    if args.gui {
        // Run GUI application
        run_gui(config_arc)?;
    } else {
        // Run CLI application
        run_cli(config_arc, &args.message, &args.branch)?;
    }
    
    Ok(())
}

fn run_gui(config: Arc<Mutex<RepoConfig>>) -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]) // Set to 600x400 as per user preference
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };
    
    eframe::run_native(
        "संधि",
        native_options,
        Box::new(|cc| Box::new(gui::app::MultiRepoPusherApp::new(cc, config))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to start GUI: {}", e))
}

fn load_repo_config() -> Result<RepoConfig> {
    let config_str = fs::read_to_string("repos.json")?;
    let config: RepoConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}