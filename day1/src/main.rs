use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("input.txt")?;
    let content: Vec<String> = match BufReader::new(file).lines().collect() {
        Ok(data) => data,
        Err(_) => vec!["".to_string()],
    };
    let mut calories_vector: Vec<i32> = vec![];

    let mut sum = 0;

    for cal in content {
        if cal.is_empty() {
            calories_vector.push(sum);
            sum = 0;
        }

        sum += cal.parse().unwrap_or(0);
    }
    calories_vector.sort();

    let max_calories_vec = &calories_vector[calories_vector.len() - 3..];
    let top_elf = max_calories_vec[2];
    let total_cal: i32 = max_calories_vec.iter().sum();

    println!("{:?}", top_elf);
    println!("{:?}", total_cal);

    Ok(())
}
