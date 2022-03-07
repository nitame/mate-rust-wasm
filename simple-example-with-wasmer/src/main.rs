fn main() {
    println!("What's ye name?");
    let mut buffer = String::from("");
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer);
    println!("Ho hi, {}!", buffer.trim());
}