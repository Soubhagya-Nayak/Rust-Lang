// ---------------------------- Ownership --------------------------------- //

// -> Reverse OwnerShip.
/*
fn main() {
    let s1: String = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    return (s, length);
}*/

// -> Using Clone
/*fn main() {
    let s1:String = String::from("Hello");
    let len:usize = calculate_length(s1.clone());
    println!("The length of {} is {}", s1, len)
}
fn calculate_length(s:String)->usize {
    let length = s.len();
    return length;
}*/

// -> Using Reference or Borrow operation
/*
fn main() {
    let s1:String = String::from("Hello");
    let len:usize = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}
fn calculate_length(s:&String)->usize {
    let length:usize = s.len();
    return length;
}*/

// --------------------------Borrow-------------------------- //

/*fn main() {
    let mut s1:String = String::from("Hello ");
    append_string(&mut s1);
    println!("The new string is {}", s1);
}

fn append_string(s:&mut String) {
    s.push_str("World!");
}*/

// --------------------------Reference Rule-------------------------- //

/*fn main() {
    let mut s1: String = String::from("Hello");
    let w1 = &mut s1;
    w1.push_str(" World");
    println!("W1: {}", w1);

    let w2 = &mut s1;
    w2.push_str(" Code");
    println!("W2: {}", w2);
} */

// -------------------------- Referencing and Deferencing -------------------------- //

/* fn main() {
    let x: u8 = 45;
    println!("Address of X = {:p}", &x);
    println!("Value of X = {}", x);
    let y = &x;
    println!("Address of Y = {:p}", y);
    println!("Value of Y = {}", y); // Auto Dereferncing.
} */

/*fn main() {
    let mut x: u8 = 5;
    x += 1; // 6
    let y = &mut x;
    *y += 1; // 7
    println!("X: {}", y);
}*/

fn main() {
    let name: String = String::from("Soubhagya Nayak");
    let len: usize = calculate_length(&name);
    print!("Length of {} is {}", name, len);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
