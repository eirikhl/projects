fn main() {
    let s = String::from("hello");

    let s = takes_ownership(s);

    let (s, length) = calculate_length(s);

    println!("{}", length);

    println!("The length of the string {} is {}", s, get_length(&s));
    
    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let mut new_s = String::from("hello");

    change(&mut new_s);
    
    println!("{}", new_s);
}

fn takes_ownership(string: String) -> String {
    println!("{}", string);
    string
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
