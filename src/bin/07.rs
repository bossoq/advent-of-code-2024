advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let results = input
        .lines()
        .into_iter()
        .map(|line| {
            let answer = line.split(": ").next().unwrap().parse::<u64>().unwrap();
            let numbers = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let possibility = 2u32.pow(numbers.len() as u32 - 1);
            let eval: Vec<u64> = (0..possibility)
                .map(|n| {
                    let mut eval = numbers[0];
                    for (i, num) in numbers[1..].iter().enumerate() {
                        if n & (1 << i) != 0 {
                            eval = add(eval, *num);
                        } else {
                            eval = multiply(eval, *num);
                        }
                        if eval > answer {
                            break;
                        }
                    }
                    eval
                })
                .collect();
            if eval.contains(&answer) {
                answer
            } else {
                0
            }
        })
        .sum::<u64>();
    Some(results)
}

pub fn part_two(input: &str) -> Option<u64> {
    let results = input
        .lines()
        .into_iter()
        .map(|line| {
            let answer = line.split(": ").next().unwrap().parse::<u64>().unwrap();
            let numbers = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let possibility = 3u32.pow(numbers.len() as u32 - 1);
            let eval: Vec<u64> = (0..possibility)
                .map(|mut n| {
                    let mut eval = numbers[0];
                    for num in numbers[1..].iter() {
                        if n % 3 == 0 {
                            eval = add(eval, *num);
                        } else if n % 3 == 1 {
                            eval = multiply(eval, *num);
                        } else {
                            eval = concat(eval, *num);
                        }
                        n /= 3;
                        if eval > answer {
                            break;
                        }
                    }
                    eval
                })
                .collect();
            if eval.contains(&answer) {
                answer
            } else {
                0
            }
        })
        .sum::<u64>();
    Some(results)
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
