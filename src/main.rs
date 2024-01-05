fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    let vaidee = Vaidee {
        age: 27,
        height: 180,
        email: String::from("vaideeswaran23@gmail.com"),
    };

    println!("{vaidee}")
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length
}
struct Vaidee {
    age: i32,
    height: i32,
    email: String,
}