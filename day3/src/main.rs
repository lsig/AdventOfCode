use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_content(file: File) -> Vec<String> {
    let content: Vec<String> = match BufReader::new(file).lines().collect() {
        Ok(data) => data,
        Err(_) => vec!["".to_string()],
    };
    return content;
}

fn find_letters(string1: &String, string2: &String) -> HashSet<char> {
    let str1 = string1.to_owned();
    let str2 = string2.to_owned();

    let common_letters = str1
        .chars()
        .filter(|&c| str2.contains(c))
        .collect::<HashSet<char>>();
    return common_letters;
}

fn convert_letters(letters: HashSet<char>) -> u32 {
    return letters
        .iter()
        .map(|&c| match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => 0,
        })
        .sum();
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let file_content = get_content(file);

    let rugsacks: Vec<(&str, &str)> = file_content
        .iter()
        .map(|sack| sack.split_at(sack.len() / 2))
        .collect();

    let sorted_rugsacks: Vec<(String, String)> = rugsacks
        .iter()
        .map(|(left, right)| {
            (
                left.to_owned().chars().sorted().collect::<String>(),
                right.to_owned().chars().sorted().collect::<String>(),
            )
        })
        .collect();

    let priority: u32 = sorted_rugsacks
        .iter()
        .map(|(left, right)| {
            let letters = find_letters(left, right);
            return convert_letters(letters);
        })
        .sum();

    println!("{:?}", priority);
    Ok(())
}
