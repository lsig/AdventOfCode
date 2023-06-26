use std::{
    fmt::Error,
    fs::File,
    io::{BufRead, BufReader},
};

enum GameResult {
    Win,
    Draw,
    Loss,
}

enum Game {
    Rock,
    Paper,
    Scissors,
}

fn opponent(turn: &str) -> Result<Game, Error> {
    match turn {
        "A" => Ok(Game::Rock),
        "B" => Ok(Game::Paper),
        "C" => Ok(Game::Scissors),
        _ => Err(Error),
    }
}

fn player(turn: &str, opp_turn: &str) -> Result<Game, Error> {
    let turns: &str = &(turn.to_owned() + opp_turn);
    match turns {
        "AX" => Ok(Game::Scissors), /* lose */
        "AY" => Ok(Game::Rock),     /* draw */
        "AZ" => Ok(Game::Paper),    /* win */
        "BX" => Ok(Game::Rock),     /* lose */
        "BY" => Ok(Game::Paper),    /* draw */
        "BZ" => Ok(Game::Scissors), /* win */
        "CX" => Ok(Game::Paper),    /* lose */
        "CY" => Ok(Game::Scissors), /* draw */
        "CZ" => Ok(Game::Rock),     /* win */
        _ => Err(Error),
    }
}

fn get_content(file: File) -> Vec<String> {
    let content: Vec<String> = match BufReader::new(file).lines().collect() {
        Ok(data) => data,
        Err(_) => vec!["".to_string()],
    };
    return content;
}

fn get_games(file: File) -> Result<Vec<(Game, Game)>, Error> {
    let content = get_content(file);
    let mut games = vec![];
    for lines in content {
        let line: Vec<&str> = lines.split_whitespace().collect();
        for turns in line.chunks(2) {
            if let [p1, p2] = turns {
                let p_one = opponent(p1)?;
                // BUG: player funciton
                let p_two = player(p1, p2)?;
                let game = (p_one, p_two);
                games.push(game);
            }
        }
    }
    return Ok(games);
}

fn weapon_points(weapon: &Game) -> u32 {
    match weapon {
        Game::Rock => 1,
        Game::Paper => 2,
        Game::Scissors => 3,
    }
}

fn result_points(result: GameResult) -> u32 {
    match result {
        GameResult::Loss => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6,
    }
}

fn calculate_points(game: &(Game, Game)) -> u32 {
    let (p1, p2) = game;

    let result = match (p2, p1) {
        (Game::Rock, Game::Scissors) => GameResult::Win,
        (Game::Rock, Game::Paper) => GameResult::Loss,
        (Game::Paper, Game::Rock) => GameResult::Win,
        (Game::Paper, Game::Scissors) => GameResult::Loss,
        (Game::Scissors, Game::Paper) => GameResult::Win,
        (Game::Scissors, Game::Rock) => GameResult::Loss,
        _ => GameResult::Draw,
    };

    let points = result_points(result) + weapon_points(&p2);
    return points;
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let games = match get_games(file) {
        Ok(data) => data,
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };
    let points: u32 = games.iter().map(|game| calculate_points(game)).sum();
    println!("{:?}", points);
    Ok(())
}
