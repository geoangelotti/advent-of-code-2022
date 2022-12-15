use advent::utils::get_lines;

#[derive(Clone, Debug, Copy)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn get_result(input: &str) -> GameResult {
    match input {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("invalid input"),
    }
}

#[derive(Clone, Debug, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Hand {
    pub fn play(self, other: Hand) -> GameResult {
        match self as isize - other as isize {
            0 => GameResult::Draw,
            1 | -2 => GameResult::Win,
            2 | -1 => GameResult::Lose,
            _ => panic!("invalid math"),
        }
    }

    pub fn decode(self, result: GameResult) -> Hand {
        match result {
            GameResult::Draw => self,
            GameResult::Lose => match self {
                Hand::Paper => Hand::Rock,
                Hand::Rock => Hand::Scissor,
                Hand::Scissor => Hand::Paper,
            },
            GameResult::Win => match self {
                Hand::Scissor => Hand::Rock,
                Hand::Paper => Hand::Scissor,
                Hand::Rock => Hand::Paper,
            },
        }
    }
}

fn get_hand(input: &str) -> Hand {
    match input {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissor,
        _ => panic!("invalid input"),
    }
}

#[derive(Clone, Debug, Copy)]
struct Game {
    player: Hand,
    opponent: Hand,
    result: GameResult,
    score: u64,
}

impl Game {
    pub fn new(tokens: String) -> Game {
        let mut it = tokens.split(" ");
        let opponent = get_hand(it.next().unwrap());
        let player = get_hand(it.next().unwrap());
        let result = player.play(opponent);
        let score = (player as isize + result as isize) as u64;
        Game {
            opponent,
            player,
            result,
            score,
        }
    }

    pub fn calculate(tokens: String) -> Game {
        let mut it = tokens.split(" ");
        let opponent = get_hand(it.next().unwrap());
        let result = get_result(it.next().unwrap());
        let player = opponent.decode(result);
        let score = (player as isize + result as isize) as u64;
        Game {
            player,
            opponent,
            result,
            score,
        }
    }
}

fn get_games_score<T>(reader: &Vec<String>, method: T) -> u64
where
    T: Fn(String) -> Game,
{
    let games = reader.to_vec().into_iter().map(method);
    games.map(|g| g.score).sum()
}

fn main() {
    let lines = get_lines("input.txt");
    let score1 = get_games_score(&lines, |str: String| Game::new(str));
    println!("{}", score1);
    let score2 = get_games_score(&lines, |str: String| Game::calculate(str));
    println!("{}", score2);
}
