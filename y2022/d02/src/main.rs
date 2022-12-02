use std::io::Read;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn from_char(letter: &char) -> Self {
        match letter {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn winning_choice(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn losing_choice(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    pub fn from_choices(me: Choice, opponent: Choice) -> Self {
        if me == opponent {
            Self::Draw
        } else if matches!(
            (me, opponent),
            (Choice::Rock, Choice::Scissors)
                | (Choice::Scissors, Choice::Paper)
                | (Choice::Paper, Choice::Rock)
        ) {
            Self::Win
        } else {
            Self::Lose
        }
    }

    pub fn from_char(letter: &char) -> Self {
        match letter {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

pub struct Game {
    choice_value: i32,
    result: GameResult,
}

impl Game {
    pub fn from_p1(opponent: Choice, me: &char) -> Self {
        let me = Choice::from_char(me);

        let result = GameResult::from_choices(me, opponent);
        let choice_value = me.value();

        Self {
            result,
            choice_value,
        }
    }

    pub fn from_p2(opponent: Choice, me: &char) -> Self {
        let result = GameResult::from_char(me);

        let me = match result {
            GameResult::Win => opponent.winning_choice(),
            GameResult::Lose => opponent.losing_choice(),
            GameResult::Draw => opponent,
        };

        Self {
            result,
            choice_value: me.value(),
        }
    }

    pub fn value(&self) -> i32 {
        self.choice_value + self.result.value()
    }
}

fn main() {
    let start = std::time::Instant::now();

    let mut input = std::fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut total_points_p1 = 0;
    let mut total_points_p2 = 0;
    for line in content.lines() {
        let mut chars = line.chars();
        let opponent = Choice::from_char(&chars.next().unwrap());
        let me = chars.nth(1).unwrap();

        total_points_p1 += Game::from_p1(opponent, &me).value();
        total_points_p2 += Game::from_p2(opponent, &me).value();
    }

    println!("Part 1: {total_points_p1} total points.");
    println!("Part 2: {total_points_p2} total points.");

    println!("Elapsed: {:?}", start.elapsed());
}
