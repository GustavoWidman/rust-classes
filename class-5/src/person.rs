use std::fmt::Display;

pub struct Person {
    pub name: String,
    height: u8,
    age: u8,
}

impl Person {
    pub fn new(name: String, height: u8, age: u8) -> Self {
        Self { age, height, name }
    }

    pub fn print(&self) {
        println!("{self}");
    }

    /// define uma nova altura para a nossa pessoa, retorna a altura velha
    pub fn set_height(&mut self, new_height: u8) -> u8 {
        let old_height = self.height.clone();
        self.height = new_height;
        old_height
    }
}

// implementa o traco "Display" (da biblioteca std) para o nosso struct "Pessoa"
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Person {{ name: {}, height: {}, age: {} }}",
            self.name, self.height, self.age
        )
    }
}
