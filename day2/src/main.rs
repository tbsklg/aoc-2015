fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
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

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(map_to_dimensions)
        .map(|(l, w, h)| {
            let x = 2 * l + 2 * w;
            let y = 2 * l + 2 * h;
            let z = 2 * w + 2 * h;
            let r = l * w * h;

            x.min(y).min(z) + r
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
    use crate::{part_1, part_2};

    #[test]
    fn should_return_square_feet_of_paper() {
        assert_eq!(part_1("1x1x10"), 43);
        assert_eq!(part_1("2x3x4"), 58);
    }

    #[test]
    fn should_return_feet_of_ribbon() {
        assert_eq!(part_2("1x1x10"), 14);
        assert_eq!(part_2("2x3x4"), 34);
    }
}
