fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut it = line.trim().chars().filter_map(|char| {
                char.to_digit(10)
            });
            let first = it.next().expect("Should be a number");
            let last = it.last();
            
            match last {
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
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!("142", process(input));
    }
}
