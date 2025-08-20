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
    let mut i = 0;
    while running.load(Ordering::SeqCst) {
        info!("Updating file to {}...", i);
        let path = "./output_file.txt";
        fs::write(path, format!("Written from cycle {}", i))?;

        thread::sleep(Duration::from_secs(8));
        i += 1;
    }
    info!("Exiting...");
    Ok(())
}
