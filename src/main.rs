
fn main() {
    let mut args = std::env::args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result: f32 = operate(operator, first_number, second_number);

    println!("-----------");
    println!("{:?}", output(first_number, second_number, operator, result));
    output(first_number, second_number, operator, result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X'=> first_number * second_number,
        '/'  => first_number / second_number,
        _ => panic!("Invalid operator")
    }
}

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number,operator ,second_number, result)
}