advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut group_1: Vec<i32> = Vec::new();
    let mut group_2: Vec<i32> = Vec::new();
    input
        .lines()
        .map(|line| line.split("   "))
        .for_each(|mut group| {
            group_1.push(group.next().unwrap().parse().unwrap());
            group_2.push(group.next().unwrap().parse().unwrap());
        });
    group_1.sort();
    group_2.sort();
    let mut result = 0;
    for i in 0..group_1.len() {
        result += (group_1[i] - group_2[i]).abs();
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut group_1: Vec<i32> = Vec::new();
    let mut group_2: Vec<i32> = Vec::new();
    input
        .lines()
        .map(|line| line.split("   "))
        .for_each(|mut group| {
            group_1.push(group.next().unwrap().parse().unwrap());
            group_2.push(group.next().unwrap().parse().unwrap());
        });
    let mut result = 0;
    for num in group_1.iter() {
        let cnt = group_2.clone().into_iter().filter(|x| x == num).count() as i32;
        result += num * cnt;
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
