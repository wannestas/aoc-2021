use std::time::Instant;

fn main() {
    let raw_input = include_str!("../input.txt");

    let input = raw_input
        .lines()
        .map(|x| x.parse::<usize>().unwrap());

    let start = Instant::now();
    println!("Part 1 solution: {}", part_1(input.clone()));
    println!("Finished after {:?} seconds", start.elapsed());

    let start = Instant::now();
    println!("Part 2 solution: {}", part_2(input));
    println!("Finished after {:?} seconds", start.elapsed());
}

fn part_1(mut input: impl Iterator<Item=usize>) -> usize {
    let first_element = input.next().unwrap();

    input.fold(
        (first_element, 0),
        |(previous, count), x| (x, if x > previous { count + 1 } else { count }),
    ).1
}

fn part_2(mut input: impl Iterator<Item=usize>) -> usize {
    let first_window: Vec<usize> = input.by_ref().take(3).collect();

    input.fold(
        (first_window, 0),
        |(mut previous_window, mut count), x|
            {
                previous_window.push(x);
                if previous_window[..3].iter().sum::<usize>() < previous_window[1..].iter().sum() {
                    count += 1;
                }
                previous_window.remove(0);
                (previous_window, count)
            },
    ).1
}