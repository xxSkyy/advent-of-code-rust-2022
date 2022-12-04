use day1::get_summed_calories;
use shared::load_input;

fn main() {
    let mut calories_per_elf = get_summed_calories(
        load_input("input_data/day1.txt"),
    );

    println!(
        "Biggest calories: {}",
        calories_per_elf.iter().max().unwrap()
    );

    calories_per_elf.sort();

    println!(
        "Sum of 3 biggest calories: {:?}",
        calories_per_elf
            .iter()
            .rev()
            .collect::<Vec<&u32>>()[0..3]
            .to_owned()
            .iter()
            .fold(0, |acc, num| acc + num.to_owned())
    )
}
