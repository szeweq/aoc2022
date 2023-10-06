use std::{io::{self, Write}, fs::{self, OpenOptions, File}, process::exit, path::Path};

fn main() {
    let Some(day) = day_from_arg() else {
        eprintln!("No day specified!");
        exit(1);
    };

    match check_day(day) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("FIX issues!");
            exit(1);
        }
        Err(e) => {
            eprintln!("FATAL error: {e}");
            exit(1);
        }
    }
}

fn day_from_arg() -> Option<u8> {
    let a = std::env::args().nth(1)?;
    a.parse::<u8>().ok()
}

fn make_new_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn touch_new_file(path: &Path) -> Result<File, io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn is_file_empty(path: &Path) -> Result<bool, io::Error> {
    Ok(fs::metadata(path)?.len() == 0)
}

fn file_empty_error(path: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("File \"{path}\" is empty"))
}

fn check_input_file(path: &str) -> Result<bool, io::Error> {
    let p = Path::new(path);
    if p.exists() {
        if is_file_empty(p)? {
            return Err(file_empty_error(path))
        }
        Ok(true)
    } else {
        touch_new_file(p)?;
        Ok(false)
    }
}

fn check_day(day: u8) -> Result<bool, io::Error> {
    let fday = format!("{day:02}");
    let path_input = format!("inputs/{fday}.txt");
    let path_example = format!("examples/{fday}.txt");
    let path_bin = format!("src/bin/{fday}.rs");

    if !Path::new(&path_bin).exists() {
        let tf = fs::read_to_string("src/template.rs")?;
        let mut f = make_new_file(&path_bin)?;
        f.write_all(tf.replace("00", &day.to_string()).as_bytes())?;
        println!("Created bin file \"{}\"", &path_bin);
    }

    let has_example = check_input_file(&path_example)?;
    if !has_example {
        println!("Created example file \"{}\"", &path_example);
    }

    let has_input = check_input_file(&path_input)?;
    if !has_input {
        println!("Created input file \"{}\"", &path_input);
    }

    Ok(has_example && has_input)
}
