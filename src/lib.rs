use std::fs;
/*
 变量和函数
*/
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

/*
数据结构
 */
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}
/*
控制流程
 */
fn fib_loop(n: u8) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        change_value(&mut a, &mut b);
        i += 1;
        if i >= n {
            break;
        }
    }
    return b;
}

fn change_value(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}

pub fn fib_for(n: u8) -> i32 {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        change_value(&mut a, &mut b);
    }
    return b;
}

fn fib_while(n: u8) -> i32 {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        change_value(&mut a, &mut b);
        i += 1;
    }
    return b;
}

/*
模式匹配
 */
fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
    // other way
    // if let Event::Message((_, _, msg)) = event {        println!("broadcast: {}", msg);       }
}

/*
错误处理
*/
fn covertedMarkdownError(url: &str) -> Result<(), ()> {
    // let url = "http://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Coverted markdown has been saved in {}", output);

    Ok(())
}

/*
通过mod来组织代码,mod.rs来声明导出的方式
通过mod就可以用来导入文件夹中声明导出的代码
*/
pub mod base;
pub mod example;

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

    #[test]
    fn test_message() {
        let alice = User {
            id: UserId(1),
            name: "Alice".into(),
            gender: Gender::Female,
        };
        let bob = User {
            id: UserId(2),
            name: "Bob".into(),
            gender: Gender::Male,
        };
        let topic = Topic {
            id: TopicId(1),
            name: "rust".into(),
            owner: UserId(1),
        };
        let event1 = Event::Join((alice.id, topic.id));
        let event2 = Event::Join((bob.id, topic.id));
        let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
        println!(
            "event1: {:?}, event2: {:?}, event3: {:?}",
            event1, event2, event3
        );
    }

    #[test]
    fn test_fib_loop() {
        let n = 10;
        assert_eq!(fib_for(n), 55);
        assert_eq!(fib_while(n), 55);
        assert_eq!(fib_loop(n), 55);
    }

    #[test]
    fn test_process_event() {
        let alice = User {
            id: UserId(1),
            name: "Alice".into(),
            gender: Gender::Female,
        };
        let bob = User {
            id: UserId(2),
            name: "Bob".into(),
            gender: Gender::Male,
        };
        let topic = Topic {
            id: TopicId(1),
            name: "rust".into(),
            owner: UserId(1),
        };
        let event1 = Event::Join((alice.id, topic.id));
        process_event(&event1)
    }
}
