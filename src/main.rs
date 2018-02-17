// Entrypoint for  the calculator

use std::io;

fn main() {
    
    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                //println!("{} bytes read", n);
                //println!("{}", input);
                let answer = process(&mut input);
                println!("{}", answer);

            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn add(v: &mut Vec<f64>) {
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    v.push(a + b)
}

fn subtract(v: &mut Vec<f64>) {
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    v.push(b - a);
}

fn multiply(v: &mut Vec<f64>) {
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    v.push(a * b);
}

fn divide(v: &mut Vec<f64>) {
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    v.push(b / a);
}

fn process(input: &mut String) -> f64 {
    let mut v: Vec<f64> = Vec::new();
    // Drop the newline
    input.pop();

    for s in input.split(" ") {
        match s.as_ref() {
            "+" => add(&mut v),
            "-" => subtract(&mut v),
            "*" => multiply(&mut v),
            "/" => divide(&mut v),
            " " => (),
            _ => match s.parse::<f64>() {
                Ok(n) => v.push(n),
                Err(e) => {
                    //println!("{}: '{}'", e, s);
                }
            }
        }
    }
    v.pop().unwrap()
}