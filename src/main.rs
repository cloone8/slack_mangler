use std::env;

fn map_special_chars(to_map: char) -> String {
    match to_map {
        '!' => "exclamation".to_string(),
        '?' => "question".to_string(),
        _ => to_map.to_string(),
    }
}

fn convert_word(word: &str) -> String {
    return word.chars()
        .map(|char| map_special_chars(char))
        .map(|char| format!(":alphabet-white-{}:", char.to_lowercase()))
        .collect();
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let converted: Vec<String> = args.iter()
        .map(|word| convert_word(word))
        .collect();

    println!("{}", converted.join(" "));
}
