mod cli;

fn main() {
    let args = cli::parse();
    println!("obs-client starting. Config = {:?}", args.config);
}
