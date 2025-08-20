use std::{sync::atomic::{AtomicBool, Ordering}, sync::Arc, thread, time::Duration, fs};

use log::info;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    info!("Starting the daemon");
    while running.load(Ordering::SeqCst) {
        info!("Checking...");
        let path = "/home/victim/stubborn_file.txt";
        if !std::path::Path::new(path).exists() {
            info!("Creating file {}", path);
            fs::write(path, "I refuse to be deleted.\n")?;
        }

        thread::sleep(Duration::from_secs(15));
    }
    info!("Exiting...");
    Ok(())
}
