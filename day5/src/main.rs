fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input.lines().filter_map(is_nice).count()
}

fn is_nice(line: &str) -> Option<&str> {
    let vowel_count = line
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .count();
    let has_double_row = line
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| pair[0] == pair[1]);
    let forbidden = ["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x));

    if vowel_count > 2 && has_double_row && !forbidden {
        Some(line)
    } else {
        None
    }
}
