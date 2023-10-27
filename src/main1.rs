fn main() {
    let input: Vec<String> = std::env::args().collect();

    println!("{}", evalutate(&input[1], 0));
}

fn evalutate(s: &str, index: usize) -> i32 {
    let mut stack: Vec<i32> = vec![];
    let mut sign: Operator = Operator::Addition;

    let mut num: i32 = 0;
    let length = s.len();

    for (i, c) in s.chars().enumerate().filter(|(i, _c)| i >= &index) {
        println!("{i}");
        if c >= '0' && c <= '9' {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
        }
        if c == '+' || c == '-' || c == '/' || c == '*' || c == '(' || c == ')' || i == length - 1 {
            
            if c == '(' {
                num = evalutate(&s, i+1);
            }

            match sign {
                Operator::Addition => stack.push(num as i32),
                Operator::Subtraction => stack.push((num as i32) * -1),
                Operator::Division => {
                    if let Some(last) = stack.last_mut() {
                        *last = *last / num as i32;
                    }
                }
                Operator::Multiplication => {
                    if let Some(last) = stack.last_mut() {
                        *last = *last * num as i32;
                    }
                }
            }

            match c {
                '+' => sign = Operator::Addition,
                '-' => sign = Operator::Subtraction,
                '/' => sign = Operator::Division,
                '*' => sign = Operator::Multiplication,
                ')' => {
                },
                _ => {}
            }
            num = 0;

        }
    }

    stack.iter().sum()
}

#[derive(PartialEq)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
