fn apply() {
    let x = 5;
    println!("x is {x}");
    let mut x = 6;
    println!("x is {x}");

    const Y: i32 = 1;
    println!("y is {Y}");
}
#[cfg(test)]
mod tests {

    use super::*;

    // 单元测试
    #[test]
    fn test_apply() {
        apply()
    }
}
