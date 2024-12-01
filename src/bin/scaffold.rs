use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const TEMPLATE: &str = r#"pub fn part1(input: &str) -> Option<u32> {
    None
}

pub fn part2(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = crusty::get_input("inputs", {{DAY}});
    crusty::display(1, part1(&input));
    crusty::display(2, part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crusty::get_input("samples", {{DAY}});
        assert_eq!(part1(&input), None);
    }

    #[test]
    fn test_part2() {
        let input = crusty::get_input("samples", {{DAY}});
        assert_eq!(part2(&input), None);
    }
}
"#;

fn create_file(path: String) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn main() {
    let day = match env::args().last() {
        Some(day) => day,
        None => {
            eprintln!("Please supply a day to scaffold.");
            process::exit(1);
        }
    };
    let padded_day = format!("{day:0>2}");
    let sol_path = format!("src/bin/day{padded_day}.rs");
    let sample_path = format!("src/samples/{padded_day}");
    let input_path = format!("src/inputs/{padded_day}");

    create_file(sol_path)
        .expect("Failed creating file")
        .write(TEMPLATE.replace("{{DAY}}", &day).as_bytes())
        .expect("Failed writing file");

    create_file(sample_path)
        .expect("Failed creating file")
        .write(b"")
        .expect("Failed writing file");
    
    create_file(input_path)
        .expect("Failed creating file")
        .write(b"")
        .expect("Failed writing file");
}
