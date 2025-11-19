use serde::Deserialize;

#[derive(Deserialize)]
struct TeamList {
    teams: Vec<Team>,
}

#[derive(Deserialize, Clone)]
pub struct Team {
    #[serde(rename = "idTeam")]
    pub id: String
}

#[derive(Deserialize)]
struct GameList {
    events: Vec<Game>,
}

#[derive(Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "strHomeTeam")]
    pub home_team: String,
    #[serde(rename = "strAwayTeam")]
    pub away_team: String,
    #[serde(rename = "dateEventLocal")]
    pub date: String,
    #[serde(rename = "strTimeLocal")]
    pub time: String,
    #[serde(rename = "strVenue")]
    pub venue: String,
}

pub fn get_team(team: &str) -> Team {
    let url = format!("https://www.thesportsdb.com/api/v1/json/123/searchteams.php?t={}", team);

    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();

    let parsed: TeamList = serde_json::from_str(&body).unwrap();
    parsed.teams[0].clone()
}

pub fn get_next_game(team: &str) -> Game {
    let team_id = get_team(team).id;

    let url = format!("https://www.thesportsdb.com/api/v1/json/123/eventsnext.php?id={}", team_id);

    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();

    let parsed: GameList = serde_json::from_str(&body).unwrap();
    parsed.events[0].clone()
}
