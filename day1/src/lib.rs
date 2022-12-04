pub fn get_summed_calories(file: String) -> Vec<u32> {
    let elves = file.split("\n\n");
    let mut calories_per_elf: Vec<u32> = Vec::new();

    for elf in elves {
        let mut calories = 0;

        for cal in elf.split("\n").filter(|row| row != &"")
        {
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

    calories_per_elf
}
