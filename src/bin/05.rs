advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (order_rule, mut parsed_input) = parse_input(input);
    let mut results = 0u32;
    parsed_input.iter_mut().for_each(|line| {
        let mut ordered = true;
        for current in 0..line.len() {
            let offset = line[current..]
                .iter()
                .enumerate()
                .position(|(i, &from)| {
                    line[current + i + 1..]
                        .iter()
                        .all(|&to| order_rule[from as usize][to as usize])
                })
                .unwrap();
            let next = current + offset;
            line.swap(current, next);
            ordered &= current == next;
        }
        if ordered {
            results += line[line.len() / 2];
        }
    });
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order_rule, mut parsed_input) = parse_input(input);
    let mut results = 0u32;
    parsed_input.iter_mut().for_each(|line| {
        let mut ordered = true;
        for current in 0..line.len() {
            let offset = line[current..]
                .iter()
                .enumerate()
                .position(|(i, &from)| {
                    line[current + i + 1..]
                        .iter()
                        .all(|&to| order_rule[from as usize][to as usize])
                })
                .unwrap();
            let next = current + offset;
            line.swap(current, next);
            ordered &= current == next;
        }
        if !ordered {
            results += line[line.len() / 2];
        }
    });
    Some(results)
}

fn parse_input(input: &str) -> ([[bool; 100]; 100], Vec<Vec<u32>>) {
    let mut raw_ordered: Vec<Vec<u32>> = Vec::new();
    let mut printed_page: Vec<Vec<u32>> = Vec::new();
    input.lines().for_each(|line| {
        if line.contains("|") {
            let ordered = line
                .split("|")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            raw_ordered.push(ordered);
        } else if line.contains(",") {
            let printed = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            printed_page.push(printed);
        }
    });
    let order_rule = sort_ordered(raw_ordered.clone());
    return (order_rule, printed_page);
}

fn sort_ordered(raw_ordered: Vec<Vec<u32>>) -> [[bool; 100]; 100] {
    let mut order_rule = [[true; 100]; 100];
    raw_ordered.iter().for_each(|x| {
        order_rule[x[1] as usize][x[0] as usize] = false;
    });
    return order_rule;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
