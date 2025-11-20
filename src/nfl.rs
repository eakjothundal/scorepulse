use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct TeamList {
    teams: Vec<Team>,
}

#[derive(Deserialize, Clone)]
pub struct Team {
    #[serde(rename = "idTeam")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
struct GameList {
    events: Vec<Game>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Game {
    #[serde(rename = "strHomeTeam")]
    pub home_team: String,
    #[serde(rename = "strAwayTeam")]
    pub away_team: String,
    #[serde(rename = "dateEventLocal")]
    pub date: String,
    #[serde(rename = "strTime")]
    pub time: String,
    #[serde(rename = "strVenue")]
    pub venue: String,
}

fn game_utc_datetime(game: &Game) -> DateTime<Utc> {
    let parsed_date = NaiveDate::parse_from_str(&game.date, "%Y-%m-%d").unwrap();
    let parsed_time = NaiveTime::parse_from_str(&game.time, "%H:%M:%S").unwrap();
    let naive_dt = parsed_date.and_time(parsed_time);

    DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc)
}

pub fn get_team(team: &str) -> Team {
    let url = format!(
        "https://www.thesportsdb.com/api/v1/json/123/searchteams.php?t={}",
        team
    );

    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();

    let parsed: TeamList = serde_json::from_str(&body).unwrap();
    parsed.teams[0].clone()
}

pub fn get_next_game(team: &str) -> Option<Game> {
    let team_id = get_team(team).id;

    let url = format!(
        "https://www.thesportsdb.com/api/v1/json/123/eventsnext.php?id={}",
        team_id
    );

    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();

    let parsed: GameList = serde_json::from_str(&body).unwrap();

    let next_game = parsed
        .events
        .iter()
        .filter(|game| game_utc_datetime(game) > Utc::now())
        .next();

    let mut game = match next_game {
        Some(g) => g.clone(),
        None => return None,
    };

    let formatted_time = game_utc_datetime(&game).format("%-I:%M %p").to_string();
    game.time = formatted_time;

    Some(game)
}
