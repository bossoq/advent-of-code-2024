use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, map_size) = parse_input(input);
    let path = find_all_path(start, &map, map_size);
    let results = path.len() as u32;
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, map_size) = parse_input(input);
    let path = find_all_path(start, &map, map_size);
    let mut results: u32 = 0;
    path.iter().for_each(|x| {
        if find_loop(start, &map, *x) {
            results += 1;
        }
    });
    Some(results)
}

fn parse_input(input: &str) -> (Vec<[usize; 2]>, [usize; 2], [usize; 2]) {
    let mut result = Vec::new();
    let mut start: [usize; 2] = [0, 0];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                result.push([i, j]);
            } else if char == '^' {
                start = [i, j];
            }
        }
    }
    let map_size = [input.lines().count(), input.lines().next().unwrap().len()];
    (result, start, map_size)
}

fn find_all_path(
    start: [usize; 2],
    map: &Vec<[usize; 2]>,
    map_size: [usize; 2],
) -> HashSet<[usize; 2]> {
    let map = map.clone();
    let mut path: HashSet<[usize; 2]> = HashSet::new();
    path.insert(start);
    let mut headings = 'N';
    let mut next = start;
    loop {
        match headings {
            'N' => {
                match map
                    .iter()
                    .filter(|&x| x[1] == next[1] && x[0] < next[0])
                    .max_by(|a, b| a[0].cmp(&b[0]))
                {
                    Some(x) => {
                        for i in x[0] + 1..next[0] {
                            path.insert([i, next[1]]);
                        }
                        next[0] = x[0] + 1;
                        headings = 'E';
                    }
                    None => {
                        for i in 0..next[0] {
                            path.insert([i, next[1]]);
                        }
                        break;
                    }
                }
            }
            'E' => {
                match map
                    .iter()
                    .filter(|&x| x[0] == next[0] && x[1] > next[1])
                    .min_by(|a, b| a[1].cmp(&b[1]))
                {
                    Some(x) => {
                        for i in next[1] + 1..x[1] {
                            path.insert([next[0], i]);
                        }
                        next[1] = x[1] - 1;
                        headings = 'S';
                    }
                    None => {
                        for i in next[1] + 1..map_size[1] {
                            path.insert([next[0], i]);
                        }
                        break;
                    }
                }
            }
            'S' => {
                match map
                    .iter()
                    .filter(|&x| x[1] == next[1] && x[0] > next[0])
                    .min_by(|a, b| a[0].cmp(&b[0]))
                {
                    Some(x) => {
                        for i in next[0] + 1..x[0] {
                            path.insert([i, next[1]]);
                        }
                        next[0] = x[0] - 1;
                        headings = 'W';
                    }
                    None => {
                        for i in next[0] + 1..map_size[0] {
                            path.insert([i, next[1]]);
                        }
                        break;
                    }
                }
            }
            'W' => {
                match map
                    .iter()
                    .filter(|&x| x[0] == next[0] && x[1] < next[1])
                    .max_by(|a, b| a[1].cmp(&b[1]))
                {
                    Some(x) => {
                        for i in x[1] + 1..next[1] {
                            path.insert([next[0], i]);
                        }
                        next[1] = x[1] + 1;
                        headings = 'N';
                    }
                    None => {
                        for i in 0..next[1] {
                            path.insert([next[0], i]);
                        }
                        break;
                    }
                }
            }
            _ => panic!("Invalid heading"),
        }
    }
    path
}

fn find_loop(start: [usize; 2], map: &Vec<[usize; 2]>, new_obstacle: [usize; 2]) -> bool {
    let mut map = map.clone();
    map.push(new_obstacle);
    let mut obstacles: HashMap<&[usize; 2], char> = HashMap::new();
    let mut headings = 'N';
    let mut next = start;
    let mut start_check = false;
    let mut is_loop = false;
    loop {
        match headings {
            'N' => {
                match map
                    .iter()
                    .filter(|&x| x[1] == next[1] && x[0] < next[0])
                    .max_by(|a, b| a[0].cmp(&b[0]))
                {
                    Some(x) => {
                        if x[0] == new_obstacle[0] && x[1] == new_obstacle[1] {
                            start_check = true;
                        }
                        if start_check {
                            if obstacles.contains_key(x) {
                                if obstacles.get(x) == Some(&'N') {
                                    is_loop = true;
                                    break;
                                }
                            }
                            obstacles.insert(x, 'N');
                        }
                        next[0] = x[0] + 1;
                        headings = 'E';
                    }
                    None => {
                        break;
                    }
                }
            }
            'E' => {
                match map
                    .iter()
                    .filter(|&x| x[0] == next[0] && x[1] > next[1])
                    .min_by(|a, b| a[1].cmp(&b[1]))
                {
                    Some(x) => {
                        if x[0] == new_obstacle[0] && x[1] == new_obstacle[1] {
                            start_check = true;
                        }
                        if start_check {
                            if obstacles.contains_key(x) {
                                if obstacles.get(x) == Some(&'E') {
                                    is_loop = true;
                                    break;
                                }
                            }
                            obstacles.insert(x, 'E');
                        }
                        next[1] = x[1] - 1;
                        headings = 'S';
                    }
                    None => {
                        break;
                    }
                }
            }
            'S' => {
                match map
                    .iter()
                    .filter(|&x| x[1] == next[1] && x[0] > next[0])
                    .min_by(|a, b| a[0].cmp(&b[0]))
                {
                    Some(x) => {
                        if x[0] == new_obstacle[0] && x[1] == new_obstacle[1] {
                            start_check = true;
                        }
                        if start_check {
                            if obstacles.contains_key(x) {
                                if obstacles.get(x) == Some(&'S') {
                                    is_loop = true;
                                    break;
                                }
                            }
                            obstacles.insert(x, 'S');
                        }
                        next[0] = x[0] - 1;
                        headings = 'W';
                    }
                    None => {
                        break;
                    }
                }
            }
            'W' => {
                match map
                    .iter()
                    .filter(|&x| x[0] == next[0] && x[1] < next[1])
                    .max_by(|a, b| a[1].cmp(&b[1]))
                {
                    Some(x) => {
                        if x[0] == new_obstacle[0] && x[1] == new_obstacle[1] {
                            start_check = true;
                        }
                        if start_check {
                            if obstacles.contains_key(x) {
                                if obstacles.get(x) == Some(&'W') {
                                    is_loop = true;
                                    break;
                                }
                            }
                            obstacles.insert(x, 'W');
                        }
                        next[1] = x[1] + 1;
                        headings = 'N';
                    }
                    None => {
                        break;
                    }
                }
            }
            _ => panic!("Invalid heading"),
        }
    }
    is_loop
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
