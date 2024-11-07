

fn make_map(numbers: &Vec<i32>) -> Vec<i32> {
    numbers.iter().map(|x| x * 2).collect()
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}



fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("\n🐈 Valores iniciais: {:?}\n", numbers);
    
    // Map
    let dobles: Vec<i32> = make_map(&numbers);

    // Filter
    let pares: Vec<i32> = numbers.iter().cloned().filter(|x| x % 2 == 0).collect();
    let primes: Vec<i32> = numbers.iter().cloned().filter(|&x| is_prime(x)).collect();

    // Fold --> .fold(0, |acc, x| acc + x)
    let soma: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    
    println!("👿 Dobros: {:?}", dobles); 
    println!("🐾 Pares: {:?}", pares); 
    println!("🐸 Primos: {:?}", primes);
    println!("👽 Soma: {}", soma);
}
