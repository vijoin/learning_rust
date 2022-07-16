fn main() {
    println!("Please, type your name: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Good! Now please, type your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    // convert to integer
    let age_int: u8 = age.trim().parse().unwrap();

    println!("Hi!, welcome {} of {} years old", name, age_int);
}
