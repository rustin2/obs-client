mod cli;
mod config;
mod domain;
mod engine;

use engine::ObsEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();
    println!("obs-client starting. Config path = {:?}", args.config);

    let (recording, cloud) = config::load(args.config)?;

    println!("Loaded recording config:\n{:#?}", recording);
    println!("Loaded cloud config:\n{:#?}", cloud);

    // Initialize OBS engine
    let _engine = ObsEngine::new(recording, cloud)?;
    println!("\nOBS Engine initialized successfully");

    // For now, just demonstrate the API
    println!("Ready to start recording (stubbed implementation)");

    Ok(())
}
