
struct Point(f64, f64);

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String
}

fn main() {

    let mut points: Vec<Point> = Vec::new();
    let point_one: Point = Point(3.0, 4.0);
    let point_two: Point = Point(6.0, 8.0);

    points.push(point_one);
    points.push(point_two);


    println!("with for loop: ");
    for point in &points {
        println!("(x: {}, y: {})", point.0, point.1);
    }

    println!("with map method: ");
    points.iter()
    .map(|point| format!("(x: {}, y: {})", point.0, point.1))  // Mapeando para a string formatada
    .for_each(|point_str| println!("{}", point_str)); 


    let some_person:Person = Person {
        name: "Alice".to_string(),
        age: 30,
        address: "123 Main St".to_string()
    };

    println!("{:?}", some_person);
    println!("Acesso aos parâmetros da Struct:\nName: {}, Age: {}, Address: {}", some_person.name, some_person.age, some_person.address);



}

