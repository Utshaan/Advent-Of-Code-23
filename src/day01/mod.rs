use std::fs;

pub fn solve() {
    let content = fs::read_to_string("src/day01/input.txt").expect(
        "Issue in reading input.txt. Make sure the file exists and the permissions are right",
    );

    let calibration_1: u32 = content
        .lines()
        .into_iter()
        .map(|x| {
            let temp = x
                .chars()
                .filter(|x| x.is_digit(10))
                .map(|c| c.to_digit(10).expect("Unexpected special character"))
                .collect::<Vec<u32>>();
            temp.first().expect("Len of temp == 0") * 10 + temp.last().expect("Lenght of temp == 1")
        })
        .sum();

    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let calibration_2: usize = content
        .lines()
        .into_iter()
        .map(|l| {
            let mut new_l = l.to_string();

            for d in 0..9 {
                new_l = new_l.replace(digits[d + 9], digits[d]);
            }

            let mut f = 0;
            if let Some((first, _)) = (1..10)
                .map(|d| (d, new_l.find(digits[d - 1])))
                .filter_map(|(d, val)| match val {
                    Some(idx) => Some((d, idx)),
                    None => None,
                })
                .min_by(|a, b| (a.1).cmp(&b.1))
            {
                f = first;
            }

            let mut l = 0;
            if let Some((last, _)) = (1..10)
                .map(|d| (d, new_l.rfind(digits[d - 1])))
                .filter_map(|(d, val)| match val {
                    Some(idx) => Some((d, idx)),
                    None => None,
                })
                .max_by(|a, b| (a.1).cmp(&b.1))
            {
                l = last;
            }

            println!("{new_l}: {f}{l}");

            f * 10 + l
        })
        .sum();

    println!("Part 1: {}", calibration_1);
    println!("Part 2: {}", calibration_2);
}
