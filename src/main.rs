mod sportsdb;

use clap::Parser;

#[derive(Parser)]
#[command(name = "scorepulse", about = "NFL next game CLI")]
struct Args {
    #[arg(long)]
    team: String,
}

fn main() {
    let args = Args::parse();

    println!("\n\nScorePulse: Next game CLI (work in progress)");

    let next_game = sportsdb::get_next_game(&args.team);
    match next_game {
        Some(game) => {
            println!(
                "Next game: {} vs {} - {} @ {} - {}",
                game.home_team, game.away_team, game.date, game.time, game.venue,
            )
        }
        None => {
            eprintln!("\nNo upcoming games found for this team.");
        }
    }
}
