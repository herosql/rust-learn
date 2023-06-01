use prost_build::Config;

fn main() {
    println!("Building........");
    Config::new()
        .out_dir("src/example")
        .compile_protos(&["src/example/abi.proto"], &["."])
        .unwrap();
}
