fn main() {
    let mut s = Box::new(String::from("un text "));
    let mut line = String::new();
    println!("Textul citit este: ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    s.push_str(&line);
    println!("{}", s);
}