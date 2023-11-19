use std::{env, time::Instant};

mod lexer;
mod node;
mod parser;

use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args.get(1).expect("No input").as_str();

    println!("Parsing {}", s);


    let mut parser = Parser::new(s);

     let now = Instant::now();

    let result = parser.parse();

    match result {
        Ok(tree) => {
            println!(" => {} = {}", tree.print(), tree.eval());
        },
        Err(e) => println!("{:?}", e),
    }

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
