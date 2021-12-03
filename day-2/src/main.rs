use std::time::Instant;

fn main() {
    let raw_input = include_str!("../input");

    let input = raw_input
        .lines();

    let start = Instant::now();
    println!("Part 1 solution: {}", part_1(input.clone()));
    println!("Finished after {:?} seconds", start.elapsed());

    let start = Instant::now();
    println!("Part 2 solution: {}", part_2(input));
    println!("Finished after {:?} seconds", start.elapsed())
}

fn part_1<'a>(input: impl Iterator<Item=&'a str>) -> usize {
    let (depth, forward) = input.fold(
        (0, 0),
        |(mut depth, mut forward), instruction| {
            let (command, operand) = instruction.split_at(instruction.find(' ').unwrap());
            let operand = operand.trim().parse::<usize>().unwrap();
            match command {
                "forward" => forward += operand,
                "down" => depth += operand,
                "up" => depth -= operand,
                _ => panic!("shit's broken")
            }
            (depth, forward)
        },
    );
    depth * forward
}

fn part_2<'a>(input: impl Iterator<Item=&'a str>) -> usize {
    let (depth, forward, _) = input.fold(
        (0, 0, 0),
        |(mut depth, mut forward, mut aim), instruction| {
            let (command, operand) = instruction.split_at(instruction.find(' ').unwrap());
            let operand = operand.trim().parse::<usize>().unwrap();
            match command {
                "forward" => {
                    forward += operand;
                    depth += aim * operand;
                }
                "down" => aim += operand,
                "up" => aim -= operand,
                _ => panic!("shit's broken")
            }
            (depth, forward, aim)
        },
    );
    depth * forward
}