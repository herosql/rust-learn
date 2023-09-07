use rust_learn::fib_for;
fn main() {
    // println!("HelloWorld")
    // fib_for(10);
    for arg in std::env::args(){
        println!("{}",arg)
    }
}
