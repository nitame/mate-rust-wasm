fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("Ho hi, {}!", line.trim());
}