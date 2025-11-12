use std::ops::AddAssign;
use std::sync::Arc;
use std::thread;

fn main() {
    let contador = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clone = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let ptr = Arc::as_ptr(&contador_clone) as *mut i32;

            for _ in 0..1000 {
                unsafe {
                    let valor = *ptr;
                    *ptr = valor + 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let ptr = Arc::as_ptr(&contador) as *const i32;
    let resultado = unsafe { *ptr };

    println!("Esperado: 10000");
    println!("Resultado: {}", resultado);
    println!("Diferença: {}", 10000 - resultado);
}

// fn main() {
//     let contador = Arc::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let contador_clone = Arc::clone(&contador);
//         let handle = thread::spawn(move || {
//             for _ in 0..1000 {
//                 contador_clone.add_assign(1); // contador_clone += 1
//             }
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     let ptr = Arc::as_ptr(&contador) as *const i32;
//     let resultado = unsafe { *ptr };

//     println!("Esperado: 10000");
//     println!("Resultado: {}", resultado);
//     println!("Diferença: {}", 10000 - resultado);
// }
