use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..100 {
            println!("Thread 1: {}", i);
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..100 {
            println!("Thread 2: {}", i);
        }
    });

    // Espera as threads terminarem
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread finalizado!");
}
