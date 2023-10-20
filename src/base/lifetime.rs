use std::str::Chars;

pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn return_str() -> Result<String, ()> {
    return Ok(("String".to_string()));
}

fn lifetime1() -> String {
    let name = "Try".to_string();
    name[1..].to_string()
}

// fn lifetime0() -> str {
//     let name = "Try";
//     name[1..]
// }

fn lifetime2(name: String) -> String {
    name[1..].to_string()
}

fn lifetime3(name: &str) -> Chars {
    name.chars()
}

fn lifetime4() -> i32 {
    let i = 1;
    i
}

fn lifetime5() -> &'static str {
    let name = "Try";
    name
}

#[cfg(test)]
mod tests {
    use std::{mem::size_of, string};

    use super::*;

    #[test]
    fn test_strtok() {
        let mut demo_str = "1,2,3,4,5";
        let delimiter = ',';
        let result = strtok(&mut demo_str, delimiter);
        assert_eq!("1", result);
    }

    #[test]
    fn test_string_size() {
        // Size of Result<String, ()>: 24 bytes
        // Size of String: 24 bytes
        // Result 占用内存?
        let result_size = size_of::<Result<String, ()>>();
        println!("Size of Result<String, ()>: {} bytes", result_size);

        let string_size = size_of::<String>();
        println!("Size of String: {} bytes", string_size);
    }
}
