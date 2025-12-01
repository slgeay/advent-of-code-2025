advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter = 0;
    let mut dial = 50;
    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let number: i32 = number.parse().unwrap();
        dial = (dial + match direction {
            "L" => -number,
            "R" => number,
            _ => 0,
        }) % 100;
        if dial == 0 {
            counter += 1;
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter = 0;
    let mut dial = 50;
    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let mut number: i32 = number.parse().unwrap();
        let previous_dial = dial;
        counter += (number / 100) as u64;
        number = number % 100;
        dial += match direction {
            "L" => -number,
            "R" => number,
            _ => 0,
        };
        if dial <= 0 || dial >= 100 {
            if previous_dial != 0  { counter += 1; }
            dial = dial.rem_euclid(100);
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
