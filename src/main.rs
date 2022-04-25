fn main() {
    let measure_question = "Choose either Fahrenheit (f) or Celsius (c)";

    let value_input: i32;
    
    let measure_input = get_input(measure_question);
    println!("{}", measure_input.eq("f"));
    if measure_input.eq("f") {
        println!("You want to convert Fahrenheit to Celsius.");
        let value_question = "What is the value in Fahrenheit?";
        value_input = get_input(value_question).trim().parse().unwrap();
        println!("{} degrees fahrenheit converts to {} degrees celsius.", value_input, value_input - 32)

    } else if measure_input.eq("c") {
        println!("You want to convert Celsius to Fahrenheit.");
        let value_question = "What is the value in Celsius?";
        value_input = get_input(value_question).trim().parse().unwrap();
        println!("{} degrees fahrenheit converts to {} degrees celsius.", value_input, value_input + 32)
    } else {
        println!("Please either choose (c) or (f)")
    }
}

fn get_input(question:&str) -> String {
    let mut buffer = String::new();
    println!("{}", question);
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
