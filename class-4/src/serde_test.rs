use serde::Serialize;

#[derive(Serialize)]
pub struct Pessoa {
    altura: u8,
    pub nome: String,
}

pub fn resposta_do_endpoint() {
    let pessoa = Pessoa {
        altura: 173,
        nome: "Gustavo".to_string(),
    };

    let pessoa_json = serde_json::to_string(&pessoa);

    if let Ok(meu_json) = pessoa_json {
        println!("{}", meu_json);
    }
}
