use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
    let now = std::time::Instant::now();
    println!("part 2: {} ({:?})", part_2(&input), now.elapsed());
}

fn part_1(input: &str) -> usize {
    input.lines().filter_map(is_nice_1).count()
}

fn is_nice_1(line: &str) -> Option<&str> {
    has_double_row(line)
        .and_then(vowel_count_of_at_least_two)
        .and_then(not_forbidden)
}

fn part_2(input: &str) -> usize {
    input.lines().filter_map(is_nice_2).count()
}

fn is_nice_2(line: &str) -> Option<&str> {
    has_double_row_not_overlapping(line).and_then(has_one_letter_in_between)
}

fn has_double_row(line: &str) -> Option<&str> {
    if line.as_bytes().windows(2).any(|pair| pair[0] == pair[1]) {
        Some(line)
    } else {
        None
    }
}

fn has_double_row_not_overlapping(line: &str) -> Option<&str> {
    let chars: Vec<char> = line.chars().collect();
    if chars.len() < 4 {
        return None;
    }

    let mut first_pos: HashMap<(char, char), usize> = HashMap::new();

    for i in 0..chars.len() - 1 {
        let pair = (chars[i], chars[i + 1]);
        if let Some(&j) = first_pos.get(&pair) {
            if i >= j + 2 {
                return Some(line);
            }
        } else {
            first_pos.insert(pair, i);
        }
    }
    None
}

fn has_one_letter_in_between(line: &str) -> Option<&str> {
    if line.as_bytes().windows(3).any(|w| w[0] == w[2]) {
        Some(line)
    } else {
        None
    }
}

fn vowel_count_of_at_least_two(line: &str) -> Option<&str> {
    let vowel_count = line
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .count();

    if vowel_count > 2 { Some(line) } else { None }
}

fn not_forbidden(line: &str) -> Option<&str> {
    if ["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x)) {
        None
    } else {
        Some(line)
    }
}
