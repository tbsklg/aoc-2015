fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(map_to_dimensions)
        .map(|(l, w, h)| {
            let x = l * w;
            let y = w * h;
            let z = l * h;

            2 * x + 2 * y + 2 * z + x.min(y).min(z)
        })
        .sum()
}

fn map_to_dimensions(line: &str) -> (usize, usize, usize) {
    let mut elements = line.split('x').map(|x| x.parse().unwrap());

    (
        elements.next().unwrap(),
        elements.next().unwrap(),
        elements.next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn should_return_square_feet_of_paper() {
        assert_eq!(part_1("1x1x10"), 43);
        assert_eq!(part_1("2x3x4"), 58);
    }
}
