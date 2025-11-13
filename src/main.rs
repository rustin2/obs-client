mod cli;
mod domain;

fn main() {
    let args = cli::parse();
    println!("obs-client starting. Config = {:?}", args.config);

    // Temporary: dummy config to prove types compile
    let dummy = domain::config::RecordingConfig {
        profile_name: "default".to_string(),
        resolution: domain::resolution::Resolution::P720,
        fps: 60,
        video_bitrate_kbps: 800,
        audio_bitrate_kbps: 320,
        encoder: domain::encoder::Encoder::X264,
        container: domain::container::ContainerFormat::Mp4,
        capture: domain::capture::CaptureTarget::Monitor { index: 0 },
        output_dir: ".".into(),
    };

    println!("Dummy config: {:?}", dummy);
}
