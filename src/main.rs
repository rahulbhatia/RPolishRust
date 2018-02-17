// Entrypoint for  the calculator

use std::io;

fn main() {
    let mut v: Vec<f64> = Vec::new();
    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                //println!("{} bytes read", n);
                //println!("{}", input);
                tokenize(&mut input, &mut v);
                println!("Stack: {:?}", v);
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

fn tokenize(input: &mut String, v: &mut Vec<f64>) {
    
    input.pop();
    for s in input.split(" ") {
        match s.as_ref() {
            "+" => add(v),
            "-" => subtract(v),
            "*" => multiply(v),
            "/" => divide(v),
            _ => match s.parse::<f64>() {
                Ok(n) => v.push(n),
                Err(e) => {
                    println!("{}: {}", e, s);
                }
            }
        }
    }

}