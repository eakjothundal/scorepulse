use clap::Parser;

#[derive(Parser)]
#[command(name = "scorepulse", about = "NFL next game CLI")]
struct Args {
    #[arg(long)]
    team: String
}

fn main() {
    let args = Args::parse();

    println!("\n\nScorePulse: NFL next game CLI (work in progress)");
    println!("Selected team: {}", args.team);
}
