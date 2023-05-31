use std::fs;

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn covertedMarkdown() {
    let url = "http://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Coverted markdown has been saved in {}", output);
}
// 测试模块
#[cfg(test)]
mod tests {
    // 引入外部函数
    use super::*;

    // 单元测试
    #[test]
    fn test_apply() {
        assert_eq!(apply(2, square), 4);
        assert_eq!(apply(3, cube), 27);
    }

    #[test]
    fn test_pi() {
        let is_pi = pi();
        let is_unit1 = not_pi();
        let is_unit2 = {
            pi();
        };
        println!(
            "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
            is_pi, is_unit1, is_unit2
        );
        assert_eq!(is_pi, 3.1415926)
    }
}
