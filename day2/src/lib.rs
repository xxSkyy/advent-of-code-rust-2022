use rps_round::{RPSPlayerPickLogic, RPSRound};

pub mod rps_round;

pub fn get_my_round_score(
    input: &str,
    pick_logic: RPSPlayerPickLogic,
) -> u32 {
    let input_split =
        input.split("\n").filter(|row| row != &"");

    let mut my_score = 0;

    for match_info in input_split {
        let mut match_info_split = match_info.split(" ");
        let (p2, p1) = (
            match_info_split.next().unwrap(),
            match_info_split.next().unwrap(),
        );

        let round = RPSRound::new(p1, p2, &pick_logic);

        if let Err(_) = round {
            println!("Failed parsing input numbers");
            continue;
        }

        let (_, (p1_score, _p2_score)) =
            round.unwrap().play();

        my_score += p1_score;
    }

    my_score
}
