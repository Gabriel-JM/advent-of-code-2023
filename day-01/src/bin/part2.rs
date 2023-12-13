fn main() {
    let input = include_str!("./input2.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };
        
                result.to_digit(10)
            });

            let first = it.next().expect("Should be a number");
            
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }   
                .parse::<u32>()
                .expect("Should be a valid number")
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
