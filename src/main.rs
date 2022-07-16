fn main() {
    println!("Please, type your name: ");

    let mut name: String = String::new();

    std::io::stdin().read_line(&mut name).unwrap();

    println!("Hola, bienvenid@: {}", name);
}
