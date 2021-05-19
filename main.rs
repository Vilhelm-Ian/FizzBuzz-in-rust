struct Divisor {
    number: i32,
    message: String,
}

impl Divisor {
    fn new(number: i32, message: String) -> Divisor {
        Divisor { number, message }
    }
}

fn main() {
    let vec_of_divisors = [
        Divisor::new(3, String::from("Fizz")),
        Divisor::new(5, String::from("Buzz")),
    ];

    for i in 1..100 {
        let mut result = String::new();
        for divisor in vec_of_divisors.iter() {
            if i % divisor.number == 0 {
                result.push_str(&divisor.message);
            }
        }
        if result == String::new() {
            result = i.to_string();
        }
        println!("{:?}", result);
    }
}
