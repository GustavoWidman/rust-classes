// Exercício de Sistema de Personagens de RPG
//
// Complete os TODOs para fazer todos os testes passarem!
// Este exercício cobre: structs, enums, traits, empréstimo/posse, e imutabilidade

// TODO 1: Defina um enum para classes de personagens
// Deve ter as variantes: Guerreiro, Mago, Ladino
enum ClassePersonagem {
    // Seu código aqui
    Guerreiro,
    Mago,
    Ladino,
}

// impl ToString for Boolean {
//     fn to_string(&self) -> String {
//         match self {
//             Boolean::True => "true".to_string(),
//             Boolean::False => "false".to_string(),
//             _ => "sla".to_string(),
//         }
//     }

impl ToString for ClassePersonagem {
    fn to_string(&self) -> String {
        match self {
            ClassePersonagem::Guerreiro => "Guerreiro".to_string(),
            ClassePersonagem::Mago => "Mago".to_string(),
            ClassePersonagem::Ladino => "Ladino".to_string(),
        }
    }
}

// TODO 2: Defina um enum para tipos de armas que carrega dados
// Espada deve carregar um u32 para afiacao (0-100)
// Cajado deve carregar um u32 para poder_magico (0-100)
// Adaga deve carregar um u32 para nivel_veneno (0-100)
enum Arma {
    Espada(u32),
    Cajado(u32),
    Adaga(u32),
}

// TODO 3: Defina uma struct Personagem com:
// - nome: String
// - classe: ClassePersonagem
// - vida: u32
// - arma: Arma
struct Personagem {
    nome: String,
    vida: u32,
    classe: ClassePersonagem,
    arma: Arma,
}

// TODO 4: Defina uma trait chamada Descritivel com um método:
// fn descrever(&self) -> String;
trait Descritivel {
    fn descrever(&self) -> String;
    // Seu código aqui
}

// TODO 5: Implemente a trait Descritivel para Personagem
// O método descrever deve retornar uma string como:
// "Aldric o Guerreiro com 100 de vida"
// impl<T: ToString> Pessoa<T> {
// fn greet(&self) {
//     println!(
//         "ola, sou {}, tenho {}, e {}cm de altura",
//         self.nome.to_string(),
//         self.idade.to_string(),
//         self.altura
//     )
// }
impl Descritivel for Personagem {
    // Seu código aqui
    fn descrever(&self) -> String {
        format!(
            "oi tudo bem, eu sou {} e tenho {} de vida e sou da classe {}",
            self.nome,
            self.vida,
            self.classe.to_string()
        )
        .to_string()
    }
}

// TODO 6: Implemente um método para Personagem
impl Personagem {
    // Cria um novo personagem (observe: isso toma posse do nome)
    fn novo(nome: String, classe: ClassePersonagem, arma: Arma) -> Self {
        let novo_personagem = Personagem {
            nome: nome,
            classe: classe,
            arma: arma,
            vida: 100,
        };
        return novo_personagem;
    }

    // TODO 7: Este método deve emprestar self imutavelmente
    // Retorna o valor de dano da arma baseado no tipo de arma
    fn obter_dano_arma(&self) -> u32 {
        let dano = match self.arma {
            Arma::Espada(dano) => dano,
            Arma::Adaga(dano) => dano,
            Arma::Cajado(dano) => dano,
        };
        return dano;
    }

    // TODO 8: Este método deve emprestar self mutavelmente
    // Reduz a vida pela quantidade de dano, mas não deixe ir abaixo de 0
    fn receber_dano(&mut self, dano: u32) {
        match dano > self.vida {
            true => self.vida = 0,
            false => self.vida = self.vida - dano,
        };
    }

    // TODO 9
    fn esta_vivo(&self) -> bool {
        self.vida != 0 // fix: trocar == por != (se estou vivo, eh por que minha vida NAO eh 0)
    }
}

// TODO 10: Escreva uma função que recebe duas referências de personagem
// e retorna true se eles têm a mesma classe de personagem
// Isso demonstra empréstimo sem tomar posse
// Dica: Use match para comparar as classes. Por exemplo:
// match (&pers1.classe, &pers2.classe) {
//     (ClassePersonagem::Guerreiro, ClassePersonagem::Guerreiro) => true,
//     ... continue para as outras combinações
// }
fn mesma_classe(pers1: &Personagem, pers2: &Personagem) -> bool {
    match (&pers1.classe, &pers2.classe) {
        (ClassePersonagem::Guerreiro, ClassePersonagem::Guerreiro) => true,
        (ClassePersonagem::Ladino, ClassePersonagem::Ladino) => true,
        (ClassePersonagem::Mago, ClassePersonagem::Mago) => true,
        _ => false,
    }
}

#[cfg(test)]
mod testes;

fn main() {
    let classe = ClassePersonagem::Ladino;
    let arma = Arma::Espada(30);
    let nome = "ian".to_string();
    Personagem::novo(nome, classe, arma);
}
