use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    let contador = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clone = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = contador_clone.write();
                // if let Ok(mut num) = result {
                //     *num += 1;
                // }
                match result {
                    Ok(mut num) => *num += 1,
                    Err(erro) => println!("{erro}"),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let resultado = *contador.read().unwrap();

    println!("Esperado: 10000");
    println!("Resultado: {}", resultado);
}
