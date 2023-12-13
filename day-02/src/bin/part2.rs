use std::{fmt::Error, collections::BTreeMap};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32
}

#[derive(Debug)]
struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>
}

impl<'a> Game<'a> {
    fn mininum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds.iter().fold(map, |mut acc, round| {
            for cube in round.iter() {
                acc
                    .entry(cube.color)
                    .and_modify(|value| {
                        *value = (*value).max(cube.amount)
                    })
                    .or_insert(cube.amount);
            }

            acc
        })
        .values()
        .product()
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", process(input));
}

fn parse_games(input: &str) -> Result<(&str, Vec<Game>), Error> {
    let games = input.lines().map(|line| {
        let game_line = &line.trim()["Game ".len()..];
        let game_id_and_list = game_line.split(':').collect::<Vec<&str>>();
        let game_list = game_id_and_list[1];

        let cube_text_list = game_list.trim().split(';').collect::<Vec<&str>>();
        let cubes = cube_text_list.iter().map(|cube_text| {
            cube_text
                .trim()
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|cube| {
                    let cube_data = cube.trim().split(' ').collect::<Vec<&str>>();
                    let amount = cube_data[0].trim();
                    let color = cube_data[1].trim();
                    
                    Cube {
                        amount: amount.parse::<u32>().unwrap(),
                        color
                    }
                })
                .collect::<Vec<Cube>>()
        })
        .collect::<Vec<Vec<Cube>>>();

        Game { rounds: cubes }
    })
    .collect::<Vec<Game>>();
    
    Ok((input, games))
}

fn process(input: &str) -> String {
    let games = parse_games(input).expect("Should parse");
    
    games
        .1
        .iter()
        .map(|game| game.mininum_cube_set())
        .sum::<u32>()
        .to_string()    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", process(input));
    }
}
