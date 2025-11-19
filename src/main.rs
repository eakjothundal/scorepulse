mod nfl;

use clap::Parser;

#[derive(Parser)]
#[command(name = "scorepulse", about = "NFL next game CLI")]
struct Args {
    #[arg(long)]
    team: String,
}

fn main() {
    let args = Args::parse();

    println!("\n\nScorePulse: NFL next game CLI (work in progress)");

    println!("\nSelected team: {}", args.team);

    let next_game = nfl::get_next_game(&args.team);
    println!(
        "Next game: {} vs {} - {} @ {} - {}",
        next_game.home_team, next_game.away_team, next_game.date, next_game.time, next_game.venue,
    )
}
