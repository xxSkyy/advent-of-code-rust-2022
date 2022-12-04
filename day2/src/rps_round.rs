#[derive(PartialEq, Eq, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub enum RPSDesiredResult {
    Win,
    Lose,
    Draw,
}

pub enum RPSResult {
    P1,
    P2,
    Draw,
}

pub enum RPSPlayerPickLogic {
    Strict,
    Lose,
}

pub type ScoreTuple = (u32, u32);

pub struct RPSRound {
    p1: RPS,
    p2: RPS,
}

impl RPSRound {
    pub fn new(
        p1: &str,
        p2: &str,
        p1_pick_logic: &RPSPlayerPickLogic,
    ) -> Result<Self, ()> {
        let p2_pick = Self::get_rps_from_string(p2)?;

        let p1_pick = match p1_pick_logic {
            RPSPlayerPickLogic::Strict => {
                Self::get_rps_from_string(p1)?
            }
            RPSPlayerPickLogic::Lose => {
                let desired_result =
                    Self::get_result_from_string(p1)?;
                Self::get_rps_from_desired_result(
                    &desired_result,
                    &p2_pick,
                )
            }
        };

        Ok(RPSRound {
            p1: p1_pick,
            p2: p2_pick,
        })
    }

    pub fn play(&self) -> (RPSResult, ScoreTuple) {
        let result = self.get_winner();
        let mut result_score =
            Self::get_result_score(&result);

        result_score.0 += Self::get_pick_score(&self.p1);
        result_score.1 += Self::get_pick_score(&self.p2);

        (result, result_score)
    }

    fn get_result_score(result: &RPSResult) -> ScoreTuple {
        match result {
            RPSResult::P1 => (6, 0),
            RPSResult::P2 => (0, 6),
            RPSResult::Draw => (3, 3),
        }
    }

    fn get_pick_score(pick: &RPS) -> u32 {
        match pick {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn get_rps_from_desired_result(
        result: &RPSDesiredResult,
        pick: &RPS,
    ) -> RPS {
        match result {
            RPSDesiredResult::Win => match pick {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            RPSDesiredResult::Lose => match pick {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            RPSDesiredResult::Draw => pick.clone(),
        }
    }

    fn get_result_from_string(
        input: &str,
    ) -> Result<RPSDesiredResult, ()> {
        match input {
            "X" => Ok(RPSDesiredResult::Lose),
            "Y" => Ok(RPSDesiredResult::Draw),
            "Z" => Ok(RPSDesiredResult::Win),
            _ => Err(()),
        }
    }

    fn get_rps_from_string(input: &str) -> Result<RPS, ()> {
        match input {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }

    fn get_winner(&self) -> RPSResult {
        if self.p1 == self.p2 {
            return RPSResult::Draw;
        }

        match (&self.p1, &self.p2) {
            (RPS::Rock, RPS::Scissors) => RPSResult::P1,
            (RPS::Paper, RPS::Rock) => RPSResult::P1,
            (RPS::Scissors, RPS::Paper) => RPSResult::P1,
            _ => RPSResult::P2,
        }
    }
}
