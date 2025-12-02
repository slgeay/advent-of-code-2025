advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-')?;
        let start: u64 = start.parse().ok()?;
        let end: u64 = end.parse().ok()?;
        for num in start..=end {
            let text = num.to_string();
            if text.len() % 2 == 0 {
                let (left, right) = text.split_at(text.len() / 2);
                if left == right {
                    sum += text.parse::<u64>().unwrap();
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-')?;
        let start: u64 = start.parse().ok()?;
        let end: u64 = end.parse().ok()?;
        for num in start..=end {
            let text = num.to_string();
            for divider in 2..=text.len() {
                if text.len() % divider == 0 {
                    let chunk_size = text.len() / divider;
                    let mut all_equal = true;
                    let first_chunk = &text[0..chunk_size];
                    for i in 1..divider {
                        let start = i * chunk_size;
                        let end = start + chunk_size;
                        if &text[start..end] != first_chunk {
                            all_equal = false;
                            break;
                        }
                    }
                    if all_equal {
                        sum += num;
                        break;
                    }
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
