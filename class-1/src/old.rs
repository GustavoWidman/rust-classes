fn main() {
    let mut var = "string".to_string();

    // var = "asd".to_string();

    let var = print(var);
    let var = print(var);
    // print_mut(&mut var);
    // print_mut(&mut var);
}

fn print(x: String) -> String {
    println!("{x}");

    return x;
}

fn print_mut(x: &mut String) {
    println!("{x}");
    *x = "oi".to_string();
}
