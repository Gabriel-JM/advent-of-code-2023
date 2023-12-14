use std::collections::BTreeMap;

// use itertools::Itertools;

enum CharacterValue {
    Empty,
    Symbol(char),
    Number(u32)
}

fn main() {

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
    for num_list in numbers {
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

        let num_positions: Vec<(i32, i32)> = num_list
            .iter()
            .map(|((y, x), _)| (*x as i32, *y as i32))
            .collect();

        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    (outer_pos.0 + pos.1 as i32, outer_pos.1 + pos.0 as i32)
                })
            })
            // .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = pos_to_check.iter().any(|pos| {
            let value = map.get(pos);
            
            if let Some(CharacterValue::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        }

        let n = num_list
            .iter()
            .map(|(_, num)| num.to_string())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        println!("{n} - {is_part_number}");
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

        assert_eq!("4361", process(input));
    }
}
