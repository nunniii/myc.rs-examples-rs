use rayon::prelude::*; 

use std::sync::Arc;
use std::sync::Mutex;
// use std::thread;
// use std::time::Duration;

/// retorna o n-ésimo termo da série de Leibniz para pi
fn leibniz_term(n: u64) -> f64 {
    // Função baseada na fórmuda de Leibniz
    4.0 * ((-1.0f64).powi(n as i32) / (2.0 * n as f64 + 1.0))
}

/// Calcula o valor de pi usando paralelismo com várias threads.
fn calculate_pi_parallel(terms: u64, num_threads: u64) -> f64 {
    // dividir os termos entre as threads
    let chunk_size = terms / num_threads;

    // Cria um Mutex para compartilhar o resultado com threads
    let result = Arc::new(Mutex::new(0.0));

    // calcula a soma da série de Leibniz em paralelo com várias threads
    (0..num_threads).into_par_iter().for_each(|i| {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 { terms } else { (i + 1) * chunk_size };

        let partial_sum: f64 = (start..end).map(|n| leibniz_term(n)).sum();

        // attualiza o resultado compartilhado com Mutex
        let mut result = result.lock().unwrap();
        *result += partial_sum;
    });

    // para capturar o resultado final
    let result = result.lock().unwrap();
    *result  }

fn main() {
    let mut terms: u64 = 0;  
    let num_threads: u64 = 3; // Número de threads desejadas // Importante 
    // certificar a quantidade de núcleos da máquina que estiver rodando o código,
    // sugiro usar algo em torno de 3, oq é razoável 

    loop {
        terms += 100_000;  // para aumentar os termos a cada iteração 
        let pi = calculate_pi_parallel(terms, num_threads);

        println!("Aproximação de Pi ({} termos): {}", terms, pi);
        // thread::sleep(Duration::from_secs(1));
    }
}
