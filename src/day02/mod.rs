use regex::Regex;
use std::fs;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

pub fn solve() {
    let content = fs::read_to_string("src/day02/input.txt").expect(
        "Issue in reading input.txt. Make sure the file exists and the permissions are right",
    );

    let id_parser =
        Regex::new(r"\D+(?<id>[0-9]+):\s(?<runs>[^\n]+)").expect("Incorrect regex pattern");
    let run_parser = Regex::new(r"([0-9]+)\s(red|blue|green)").expect("Incorrect regex pattern");

    let result_1: usize = id_parser
        .captures_iter(&content)
        .map(|c| {
            let id = c.name("id").unwrap().as_str();
            let runs = c.name("runs").unwrap().as_str();
            let run_list_len = run_parser
                .captures_iter(runs)
                .map(|r| {
                    let (_, [count, color]) = r.extract();
                    (count.parse().expect("Error in parsing number"), color)
                })
                .filter(|(count, color)| match color {
                    &"red" => count > &RED,
                    &"blue" => count > &BLUE,
                    &"green" => count > &GREEN,
                    _ => true,
                })
                .collect::<Vec<(usize, &str)>>()
                .len();
            (
                id.parse::<usize>().expect("Error in parsing ID"),
                run_list_len,
            )
        })
        .filter(|(_, invalidity_score)| invalidity_score == &0)
        .fold(0, |ans, (game, _)| ans + game);

    println!("Result 1: {result_1}");

    let result_2: usize = id_parser
        .captures_iter(&content)
        .map(|c| {
            let (mut r, mut g, mut b) = (0_usize, 0_usize, 0_usize);
            let runs = c.name("runs").unwrap().as_str();
            run_parser.captures_iter(runs).for_each(|run_parsed_value| {
                let (_, [count, color]) = run_parsed_value.extract();
                let count = count.parse().expect("Error parsing integer");
                match color {
                    "red" => r = r.max(count),
                    "blue" => b = b.max(count),
                    "green" => g = g.max(count),
                    _ => (),
                }
            });
            r * b * g
        })
        .sum();

    println!("Result 2: {result_2}");
}
