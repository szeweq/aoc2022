
pub fn split_str_on(s: &str, ch: char) -> (&str, &str) {
    s.split_once(ch).expect("Cannot split string")
}
