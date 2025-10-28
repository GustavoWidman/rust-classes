fn main() {
    // Part 1: Basic ownership move
    let s1 = String::from("hello");
    let s2 = s1;
    print(&s1); // ERROR! Why?
    print(&s2);

    // Part 2: Ownership with functions
    let name = String::from("Rust");
    print_name(name);
    print(&name); // ERROR! Why?

    // Part 3: References (borrowing)
    let language = String::from("Rust");
    print_language(&language);
    print(&language); // This should work!
}

fn print(s: &String) {
    println!("string: {s}");
}

fn print_name(n: String) {
    println!("The name is: {}", n);
}

fn print_language(lang: &String) {
    println!("The language is: {}", lang);
}
