use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt")
        .expect("Failed to read the file");
    let elves = file.split("\n\n");
    let mut calories_per_elf: Vec<u32> = Vec::new();

    for elf in elves {
        let mut calories = 0;

        for cal in elf.split("\n") {
            if cal == "" {
                continue;
            };
            match cal.parse::<u32>() {
                Ok(num) => calories += num,
                Err(_) => println!(
                    "Error parsing number: {}",
                    cal
                ),
            }
        }

        calories_per_elf.push(calories)
    }

    println!(
        "Biggest calories: {}",
        calories_per_elf.iter().max().unwrap()
    )
}
