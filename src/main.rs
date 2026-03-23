fn main() {
    // snake_case: hello_world - rust follow
    // camelCase: helloWorld - js follow

    let mut num: u8 = 255;
    println!("Strored Number: {num}");
    num = 199;
    println!("Strored Number: {num}");

    // String -> Dynamic Length String. - Heap Memory Allocation.
    // &str -> Fixed Length String. - Special Memory Allocation.(stack).
    let mut string_literals: String = String::from("Hello,Coders!!");
    string_literals.push_str(" What's Up.");
    println!("String Literals: {}", string_literals);

    // Tuple
    let emp_info: (&str, u8) = ("Soubhagya", 21);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("Employe Name: {}, Employe Age: {}", emp_name, emp_age);
}
