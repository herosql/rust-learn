fn borrow() {
    let data = vec![1, 2, 3, 4, 5];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));
    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}
fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
// 测试模块
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    // 引入外部函数
    use super::*;

    // 单元测试
    #[test]
    fn test_apply() {
        borrow();
        // assert_eq!(1, 2)
    }

    #[test]
    fn move_to_closure() {
        let arr = vec![1];
        std::thread::spawn(move || {
            println!("{:?}", arr);
        });
    }

    #[test]
    fn share_str() {
        let hello = "hello";
        let shared_str = Arc::new(hello.to_string());
        let cloned_str = Arc::clone(&shared_str);
        std::thread::spawn(move || {
            println!("{:?}", cloned_str);
        });
    }
}
