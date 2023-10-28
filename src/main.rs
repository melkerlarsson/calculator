

// use std::collections::HashMap;

// use logos::{Logos, Lexer};

use std::env;

mod lexer;

use crate::lexer::*;

// const CONSTANTS: HashMap<&str, f64> = HashMap::from([
//     ("G", 6.67430 * 10^(-11)),
//     ("g", 9.82)

// ]);

// fn constant(lex: &mut Lexer<Token>) -> Option<f64> {
//     Some(CONSTANTS[lex.slice()])
// }

// #[derive(Logos, Debug, PartialEq)]
// enum Token {
//     #[token("+")]
//     Plus,
        
//     #[token("-")]
//     Minus,

//     #[regex(r"\d+")]
//     Integer,

//     #[regex(r"\d?+\.\d+")]
//     Float,

//     #[regex("G|g", constant)]
//     Constant



// }


fn main() {



    let args: Vec<String> = env::args().collect();
    let s = args.get(1).expect("No input").as_str();

    println!("Parsing {}", s);

    let mut lexer = Lexer::new("1 2 *");

    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());



    

    // let scanner = Lexer::new(s);
    // let mut parser = Parser::new(scanner);

    // let now = Instant::now();

    // let result = parser.parse();

    // match result {
    //     Ok(tree) => {
    //         println!(" => {} = {}", tree.print(), tree.eval());
    //     },
    //     Err(e) => println!("{:?}", e),
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed time: {:.2?}", elapsed);

}

