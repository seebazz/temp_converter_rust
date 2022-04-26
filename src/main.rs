fn main() {
    let measure_question = "Choose either Fahrenheit (f) or Celsius (c)";

    let value_input: i32;
    
    let measure_input = get_input(measure_question);

    match strip_trailing_newline(measure_input.as_str()) {
        "f" | "F" | "fahrenheit" | "Fahrenheit" => {
            println!("You want to convert Fahrenheit to Celsius.");
            let value_question = "What is the value in Fahrenheit?";
            value_input = get_input(value_question).trim().parse().unwrap();
            println!("{} degrees fahrenheit converts to {} degrees celsius.", value_input, value_input - 32)
        },
        "c" | "celsius" | "Celsius" => {
            println!("You want to convert Celsius to Fahrenheit.");
            let value_question = "What is the value in Celsius?";
            value_input = get_input(value_question).trim().parse().unwrap();
            println!("{} degrees celsius converts to {} degrees fahrenheit.", value_input, value_input + 32)
        },
        _ => {
            println!("{}", measure_input.as_str());
            println!("Please either choose (c) or (f)");
        }
    }
}

fn get_input(question:&str) -> String {
    let mut buffer = String::new();
    println!("{}", question);
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}