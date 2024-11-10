

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn separate_even_odd(my_array: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {

    let mut even: Vec<i32> = Vec::new();
    let mut odd: Vec<i32> = Vec::new();

    for &i in my_array {
        if is_even(i) { even.push(i) } else { odd.push(i) }
    }
    (even, odd)
}

fn main() {

    let my_array: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let (evens, odds) = separate_even_odd(&my_array); 

    println!("{:?}\n even --> {:?}\n odd --> {:?}, ", my_array, evens, odds);

}
