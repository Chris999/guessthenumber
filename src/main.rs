use paris::{info, success, warn};
use std::io;
use rand::prelude::*;

fn main() {
    let mut correct_number = reset();

    loop {
        let mut number = String::new();
        info!("Guess a number:");
        io::stdin().read_line(&mut number).unwrap();
        println!("{}", number);
        let parsed_number = number.trim().parse::<i32>().unwrap_or(0);

        if let Some(response) = number_in_range(parsed_number, correct_number) {
            warn!("{}", response);
            continue;
        }

        if parsed_number == correct_number {
            success!("You guessed right! Would you like to try again? [Y/n]");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            match answer.to_lowercase().as_str().trim() {
                "y" => correct_number = reset(),
                _ => break
            }
        }
    }
    info!("The game is now over. Congratz, hope you had fun.");
}

fn reset() -> i32 {
    let mut max_number = String::new();
    info!("Select a max value");
    io::stdin().read_line(&mut max_number).unwrap();

    get_random_num(max_number.trim().parse::<i32>().unwrap_or(1000))
}

fn number_in_range(number: i32, correct_number: i32) -> Option<String> {
    if number > correct_number {
        return Some(format!("Number <blue>{}</> is too high", number));
    }
    if number < correct_number {
        return Some(format!("Number <blue>{}</> is too low", number));
    }

    None
}

fn get_random_num(max_value: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, max_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_num() {
        assert!(get_random_num(2) < 3);
    }
}
