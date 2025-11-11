use test::{Estado, Pessoa, oi};
mod b64;
mod serde_test;
mod test;
mod utils;

fn main() {
    // slice
    let array: [String; 2] = ["oi".to_string(), "tchau".to_string()];
    println!("array {:?}", array);

    // for item in array.iter() {
    //     println!("item: {item}");
    // }

    array.iter().for_each(|item| {
        println!("item: {item}");
    });

    // vec
    let mut vector: Vec<String> = Vec::new();
    vector.push("oi".to_string());
    vector.push("tchau".to_string());

    // for item in &vector {
    //     println!("item: {item}");
    // }

    let novo_vector: Vec<String> = vector
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{i}. {item}, tudo bem?"))
        .collect();

    let vector_filtered: Vec<String> = vector
        .iter()
        .filter_map(|item| {
            if item.as_str() == "oi" {
                Some("oi, tudo bem".to_string())
            } else {
                None
            }
        })
        .collect();

    println!("vec: {:?}", vector);
    println!("vec novo: {:?}", novo_vector);

    // fold
    // reduce

    let array: [u8; 4] = [1, 2, 3, 4];

    oi();

    let estado: Estado = Estado::Correndo(1);

    let pessoa: Pessoa = Pessoa::new(173, "Gustavo".to_string());

    // println!("o nome eh {} e a altura eh {}", pessoa.nome, pessoa.altura);

    utils::log::print("oi, tudo bem".to_string());

    b64::encodar_base64_teste();
    serde_test::resposta_do_endpoint();
}
