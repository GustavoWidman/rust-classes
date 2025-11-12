use std::sync::{Arc, RwLock};

use crate::person::Person;

mod person;

#[tokio::main]
// async fn main() -> anyhow::Result<()> {
async fn main() -> eyre::Result<()> {
    println!("Hello, world!");

    let mut gustavo = Person::new("gustavo".into(), 150, 20);
    gustavo.print();

    synchronous(&mut gustavo, 170)?;

    let gustavo = Arc::new(RwLock::new(gustavo));
    let resultado = asynchronous(Arc::clone(&gustavo), 173);
    // match resultado {
    //     Ok(_) => gustavo.print(),
    //     Err(erro) => println!("deu erro: {erro}"),
    // }

    resultado.await?;
    gustavo.read().unwrap().print();

    Ok(())
}

fn synchronous(person: &mut Person, height: i64) -> eyre::Result<()> {
    if height <= 0 {
        eyre::bail!("altura de uma pessoa deve ser maior do que zero")
    }

    person.set_height(height as u8);

    Ok(())
}

async fn asynchronous(person: Arc<RwLock<Person>>, height: i64) -> eyre::Result<()> {
    if height <= 0 {
        eyre::bail!("altura de uma pessoa deve ser maior do que zero")
    }

    person.write().unwrap().set_height(height as u8);

    person.read().unwrap().print();

    Ok(())
}

fn multiply(numero: i32, multiplicador: i32) -> Option<i32> {
    if multiplicador == 0 {
        return None;
    }

    Some(numero * multiplicador)
}

fn duplicate(numero: i32) -> Option<i64> {
    let novo_numero = multiply(numero, 2)?;

    Some(novo_numero as i64)
}
