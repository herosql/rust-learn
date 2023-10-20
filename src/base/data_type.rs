fn count() {
    let float = 0.3 - 0.2;
    println!("float is {float}");

    let int = 10;
    println!("int is {int}");

    let z: char = 'ℤ';

    let t = true;

    println!("z is {z}");

    println!("t is {t}");

    let tup = (500, 6.4, 1);

    let (n, w, c) = tup;

    println!("tup is {n}");
}

#[cfg(test)]
mod tests {

    use super::*;

    // 单元测试
    #[test]
    fn test_count() {
        count()
    }
}
