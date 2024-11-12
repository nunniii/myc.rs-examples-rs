


fn main() {
    let my_array = vec![0, 1, 2, 3];

    let cycled = my_array.iter().cycle();

    let output = cycled.take(70).collect::<Vec <&usize>>();

    let length = output.len();

    println!("{:?}", output);
    println!("{}", length);
}



