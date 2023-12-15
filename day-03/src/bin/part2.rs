use std::collections::BTreeMap;

use itertools::Itertools;

enum CharacterValue {
    Empty,
    Symbol(char),
    Number(u32)
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> String {
    let map = input.lines().enumerate().flat_map(|(y, line)| {
        line.trim().chars().enumerate().map(move |(x, character)| {
            ((y as i32, x as i32), match character {
                '.' => CharacterValue::Empty,
                c if c.is_ascii_digit() => CharacterValue::Number(
                    c.to_digit(10).expect("Should be a number")
                ),
                c => CharacterValue::Symbol(c)
            })
        })
    })
    .collect::<BTreeMap<(i32, i32), CharacterValue>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];

    for ((y, x), value) in map.iter() {
        if let CharacterValue::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_number_x, _), _)) => {
                            if last_number_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");

                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num
                                )]);
                            }
                        },
                        None => unimplemented!("shouldn't happen")
                    }
                },
                None => { numbers.push(vec![((*x, *y), *num)]); }
            }
        }
    }

    let mut total = 0;
    for symbol in map.iter().filter(|(_, value)| matches!(value, CharacterValue::Symbol('*'))) {
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| {
                (outer_pos.0 + symbol.0.1, outer_pos.1 + symbol.0.0)
            })
            .collect();

        let mut numbers_index = vec![];
        
        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list
                    .iter()
                    .find(|(num_pos, _)| num_pos == &pos)
                    .is_some()
                {
                    numbers_index.push(i);
                }
            }
        }

        let is_gear = numbers_index.iter().unique().count() == 2;

        if is_gear {
            total += numbers_index
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }
    
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!("467835", process(input));
    }
}
