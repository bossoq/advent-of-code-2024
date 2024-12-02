use std::collections::VecDeque;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input
        .lines()
        .map(|line| {
            let orig_data = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>();
            if check_level(&orig_data, None) {
                1
            } else {
                0
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: u32 = input
        .lines()
        .map(|line| {
            let orig_data = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>();
            if check_level(&orig_data, None) {
                1
            } else {
                let mut result: u32 = 0;
                for i in 0..orig_data.len() {
                    if check_level(&orig_data, Some(i)) {
                        result = 1;
                        break;
                    }
                }
                result
            }
        })
        .sum();
    Some(result)
}

fn check_level(data: &VecDeque<i32>, remove_level: Option<usize>) -> bool {
    let mut data = data.clone();
    if let Some(level) = remove_level {
        data.remove(level);
    }
    let mut shift_data = data.clone();
    shift_data.rotate_left(1);
    data.pop_back();
    shift_data.pop_back();
    let subtract = data
        .into_iter()
        .zip(shift_data)
        .map(|(a, b)| a - b)
        .collect::<Vec<i32>>();
    subtract.iter().all(|x| x >= &1 && x <= &3) || subtract.iter().all(|x| x <= &-1 && x >= &-3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
