use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pessoa {
    altura: u8,
    pub nome: String,
}

pub fn resposta_do_endpoint() -> eyre::Result<()> {
    let pessoa = Pessoa {
        altura: 173,
        nome: "Gustavo".to_string(),
    };

    let pessoa_json = serde_json::to_string(&pessoa)?;

    println!("{}", pessoa_json);

    let pessoa_nova = serde_json::from_str::<Pessoa>(&pessoa_json)?;

    Ok(())
}
