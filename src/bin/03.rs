use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let results = input
        .lines()
        .map(|line| {
            pattern
                .captures_iter(line)
                .map(|m| {
                    let mut nums = m
                        .extract::<2>()
                        .1
                        .map(|n| n.parse::<u32>().unwrap())
                        .into_iter();
                    nums.next().unwrap() * nums.next().unwrap()
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut content = String::new();
    input.lines().for_each(|line| {
        content.push_str(line.replace("\n", "").as_str());
    });
    let mut results = 0u32;
    loop {
        let do_map = do_pattern.find(&content).map(|m| m.start());
        let dont_map = dont_pattern.find(&content).map(|m| m.start());
        let pattern_map = pattern.find(&content);
        if pattern_map.is_none() {
            break;
        }
        let pattern_map = pattern_map.unwrap();
        let do_map = do_map.unwrap_or(usize::MAX);
        let dont_map = dont_map.unwrap_or(usize::MAX);
        if do_map < pattern_map.start() && dont_map < pattern_map.start() {
            if do_map < dont_map {
                content = content[pattern_map.end()..].to_string();
                continue;
            }
        } else {
            if dont_map < pattern_map.start() {
                if do_map != usize::MAX {
                    content = content[do_map + 4..].to_string();
                    continue;
                }
                break;
            }
        }
        let pattern_capture = pattern.captures(&content).unwrap();
        let mut nums = pattern_capture
            .extract::<2>()
            .1
            .map(|n| n.parse::<u32>().unwrap())
            .into_iter();
        results += nums.next().unwrap() * nums.next().unwrap();
        content = content[pattern_map.end()..].to_string();
    }
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
