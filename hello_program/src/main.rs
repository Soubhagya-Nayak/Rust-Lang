// ---------------------------- Variables --------------------------------- //
/*fn main() {
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

    // Destructuring
    let (employe_name, employe_age) = emp_info;

    println!("Employe Name: {}, Employe Age: {}", emp_name, emp_age);
    println!(
        "Employee Name: {}, Employee Age: {}",
        employe_name, employe_age
    );
} */

// ---------------------------- Function --------------------------------- //

/*
fn main() {
    print_name();
    print_numeric(56);

    let num1: u8 = 10;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2);
    println!("Result: {}", result);
}

fn print_name() {
    println!("Soubhagya Nayak");
}

fn print_numeric(item: u8) {
    println!("Number is: {}", item);
}

fn add(num1: u8, num2: u8) -> u8 {
    return num1 + num2;
}
*/

// ---------------------------- Scope --------------------------------- //

fn main() {
    let outer_variable: u8 = 23;
    // Outer Variable
    {
        let inner_varibale: u8 = 34;
        // Inner Variable
        println!("Inner Variable: {}", inner_varibale);
        println!("Outer Varibale: {}", outer_variable);
    }

    // println!("Inner Variable: {}", inner_varibale); // Can't use inner_variable beacuse of scope validation.
    println!("Outer Varibale: {}", outer_variable);
}


