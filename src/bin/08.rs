use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map_size: [usize; 2] = [input.lines().count(), input.lines().next().unwrap().len()];
    let map = parse_input(input);
    let mut antinodes: HashSet<[usize; 2]> = HashSet::new();
    map.iter().for_each(|(_, v)| {
        v.iter().combinations(2).for_each(|mut pair| {
            let dx = (pair[0][0] as i32 - pair[1][0] as i32).abs();
            let dy = (pair[0][1] as i32 - pair[1][1] as i32).abs();
            for _ in 0..2 {
                pair.reverse();
                let antinode_x = if pair[0][0] < pair[1][0] {
                    pair[0][0] as i32 - dx
                } else {
                    pair[0][0] as i32 + dx
                };
                let antinode_y = if pair[0][1] < pair[1][1] {
                    pair[0][1] as i32 - dy
                } else {
                    pair[0][1] as i32 + dy
                };
                if 0 <= antinode_x
                    && antinode_x < map_size[0] as i32
                    && 0 <= antinode_y
                    && antinode_y < map_size[1] as i32
                {
                    let antinode = [antinode_x as usize, antinode_y as usize];
                    antinodes.insert(antinode);
                }
            }
        });
    });
    let results = antinodes.len() as u32;
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_size: [usize; 2] = [input.lines().count(), input.lines().next().unwrap().len()];
    let map = parse_input(input);
    let mut antinodes: HashSet<[usize; 2]> = HashSet::new();
    map.iter().for_each(|(_, v)| {
        v.iter().combinations(2).for_each(|mut pair| {
            let dx = (pair[0][0] as i32 - pair[1][0] as i32).abs();
            let dy = (pair[0][1] as i32 - pair[1][1] as i32).abs();
            let mut i = 1;
            loop {
                let antinode_x = if pair[0][0] < pair[1][0] {
                    pair[0][0] as i32 - dx * i
                } else {
                    pair[0][0] as i32 + dx * i
                };
                let antinode_y = if pair[0][1] < pair[1][1] {
                    pair[0][1] as i32 - dy * i
                } else {
                    pair[0][1] as i32 + dy * i
                };
                if 0 <= antinode_x
                    && antinode_x < map_size[0] as i32
                    && 0 <= antinode_y
                    && antinode_y < map_size[1] as i32
                {
                    let antinode = [antinode_x as usize, antinode_y as usize];
                    antinodes.insert(antinode);
                    i += 1;
                } else {
                    break;
                }
            }
            pair.reverse();
            let mut i = 1;
            loop {
                let antinode_x = if pair[0][0] < pair[1][0] {
                    pair[0][0] as i32 - dx * i
                } else {
                    pair[0][0] as i32 + dx * i
                };
                let antinode_y = if pair[0][1] < pair[1][1] {
                    pair[0][1] as i32 - dy * i
                } else {
                    pair[0][1] as i32 + dy * i
                };
                if 0 <= antinode_x
                    && antinode_x < map_size[0] as i32
                    && 0 <= antinode_y
                    && antinode_y < map_size[1] as i32
                {
                    let antinode = [antinode_x as usize, antinode_y as usize];
                    antinodes.insert(antinode);
                    i += 1;
                } else {
                    break;
                }
            }
        });
    });
    map.iter().for_each(|(_, v)| {
        v.iter().for_each(|&node| {
            antinodes.insert(node);
        })
    });
    let results = antinodes.len() as u32;
    Some(results)
}

fn parse_input(input: &str) -> HashMap<char, HashSet<[usize; 2]>> {
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let set = map.entry(char).or_insert(HashSet::new());
            set.insert([i, j]);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
