use std::fs;

pub mod util;

pub fn read_input_file(name: &str) -> String {
    read_file_string("inputs/", name)
}
pub fn read_example_file(name: &str) -> String {
    read_file_string("examples/", name)
}

#[macro_export]
macro_rules! solve {
    ($solver1:ident, $solver2:ident) => {
        fn main() {
            let input = &aoc2022::read_input_file(env!("CARGO_BIN_NAME"));
            aoc2022::print_result(1, $solver1, input);
            aoc2022::print_result(2, $solver2, input);
        }
    };
}

#[macro_export]
macro_rules! assert_ex {
    ($solver:ident, $val:expr) => {
        let input = aoc2022::read_example_file(env!("CARGO_BIN_NAME"));
        assert_eq!($solver(&input).unwrap(), $val)
    };
}

#[macro_export]
macro_rules! type_err {
    ($et:ident, $s: expr) => {
        #[derive(Debug, Clone)]
        pub struct $et;
        impl std::fmt::Display for $et {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $s)
            }
        }
        impl std::error::Error for $et {}
    };
}

pub fn print_result<T: std::fmt::Display>(part: u32, func: impl FnOnce(&str) -> Option<T>, input: &str) {
    let tim = std::time::Instant::now();
    let result = func(input).unwrap();
    let el = tim.elapsed();
    println!("> Part {}\n{}\n @ ELAPSED: {:.2?}\n", part, result, el);
}

fn read_file_string(folder: &str, name: &str) -> String {
    fs::read_to_string(folder.to_string() + name + ".txt").unwrap()
}