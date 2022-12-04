use day2::{
    get_my_round_score, rps_round::RPSPlayerPickLogic,
};
use shared::load_input;

fn main() {
    let input = load_input("input_data/day2.txt");

    let score_part1 = get_my_round_score(
        &input,
        RPSPlayerPickLogic::Strict,
    );

    println!("Your score part1: {}", score_part1);

    let score_part2 = get_my_round_score(
        &input,
        RPSPlayerPickLogic::Lose,
    );

    println!("Your score part2: {}", score_part2);
}
