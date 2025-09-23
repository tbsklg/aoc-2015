fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
}

fn part_1(input: &str) -> usize {
    let mut total_code_chars = 0;
    let mut total_memory_chars = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (code_chars, memory_chars) = process_string_literal(trimmed);
        total_code_chars += code_chars;
        total_memory_chars += memory_chars;
    }

    total_code_chars - total_memory_chars
}

fn process_string_literal(s: &str) -> (usize, usize) {
    let code_chars = s.len();

    if s.len() < 2 || !s.starts_with('"') || !s.ends_with('"') {
        return (code_chars, 0);
    }

    let content = &s[1..s.len() - 1];
    let memory_chars = count_memory_chars(content);

    (code_chars, memory_chars)
}

fn count_memory_chars(content: &str) -> usize {
    let mut count = 0;
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('\\') => count += 1,
                Some('"') => count += 1,
                Some('x') => {
                    chars.next();
                    chars.next();
                    count += 1;
                }
                _ => count += 1,
            }
        } else {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::count_memory_chars;

    #[test]
    fn count_characters() {
        assert_eq!(count_memory_chars(""), 2);
        assert_eq!(count_memory_chars("abc"), 5);
        assert_eq!(count_memory_chars("aaa\"aaa"), 10);
    }
}
