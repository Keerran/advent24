use std::{env, fs, fmt::Display};

pub fn display<T: Display>(part: u8, solution: Option<T>) {
    if let Some(solution) = solution {
        println!("{part}: {solution}");
    }
}

pub fn get_input(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let path = cwd.join("src").join(folder).join(format!("{day:0>2}"));

    fs::read_to_string(path).expect("Could not open input file")
}
