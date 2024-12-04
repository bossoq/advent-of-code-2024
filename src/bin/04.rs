advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_array(input);
    let target = "XMAS".chars().collect::<Vec<char>>();
    let dir_x = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_y = [-1, 0, 1, -1, 1, -1, 0, 1];
    let mut results = 0;

    for i in 0..array.len() {
        for j in 0..array[0].len() {
            if array[i][j] != target[0] {
                continue;
            }
            for k in 0..8 {
                let mut x = i as i32 + dir_x[k];
                let mut y = j as i32 + dir_y[k];
                let mut l = 1 as usize;
                while l < target.len() {
                    if x < 0 || x >= array.len() as i32 || y < 0 || y >= array[0].len() as i32 {
                        break;
                    }
                    if array[x as usize][y as usize] != target[l] {
                        break;
                    }
                    x += dir_x[k];
                    y += dir_y[k];
                    l += 1;
                }
                if l == target.len() {
                    results += 1;
                }
            }
        }
    }
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let array = parse_array(input);
    let dir_x = [-1, 1, -1, 1];
    let dir_y = [-1, 1, 1, -1];
    let mut results = 0;

    for i in 0..array.len() {
        for j in 0..array[0].len() {
            if array[i][j] != 'A' {
                continue;
            }
            let mut found = false;
            let mut target = String::from("MS");
            for k in 0..2 {
                let x = i as i32 + dir_x[k];
                let y = j as i32 + dir_y[k];
                if x < 0 || x >= array.len() as i32 || y < 0 || y >= array[0].len() as i32 {
                    break;
                }
                if target.contains(array[x as usize][y as usize]) {
                    target = target.replace(array[x as usize][y as usize], "");
                    continue;
                }
            }
            if target.len() == 0 {
                found = true;
            }
            if found {
                let mut target = String::from("MS");
                for k in 2..4 {
                    let x = i as i32 + dir_x[k];
                    let y = j as i32 + dir_y[k];
                    if x < 0 || x >= array.len() as i32 || y < 0 || y >= array[0].len() as i32 {
                        break;
                    }
                    if target.contains(array[x as usize][y as usize]) {
                        target = target.replace(array[x as usize][y as usize], "");
                        continue;
                    }
                }
                if target.len() == 0 {
                    results += 1;
                }
            }
        }
    }
    Some(results)
}

fn parse_array(input: &str) -> Vec<Vec<char>> {
    return input
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
