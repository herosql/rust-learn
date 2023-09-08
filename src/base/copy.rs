fn copy_demo() {
    let data = vec![1, 2, 3, 4, 5];
    let data1 = data;
    println!("sum of data1:{}", sum(data1));
    // println!("data1:{:?}", data1);
    // println!("sum of data:{}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

// 测试模块
#[cfg(test)]
mod tests {
    // 引入外部函数
    use super::*;

    // 单元测试
    #[test]
    fn test_apply() {
        copy_demo()
    }

    #[test]
    fn test_stack_to_heap() {
        // stack
        // heap
        let mut a = vec![1, 2, 3];
        let b = 4;
        a.push(b);
        assert_eq!(b.to_string(), a.get(3).unwrap().to_string());
    }

    #[test]
    fn change() {
        let mut arr = vec![1, 2, 3]; // cache the last item
        arr.push(4); // consume previously stored last item
        let last = arr.last();
        println!("last: {:?}", last);
    }
}
