pub enum Estado {
    Correndo(u8),
    Andando(u8),
    Parado,
}

pub struct Pessoa {
    altura: u8,
    pub nome: String,
}

impl Pessoa {
    pub fn new(altura: u8, nome: String) -> Pessoa {
        Pessoa { altura, nome }
    }
}

pub fn oi() {
    println!("oi")
}
