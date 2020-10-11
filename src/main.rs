#![allow(dead_code)]

mod tokenizer;
mod io;
mod cli;
mod compile;
mod parser;
mod llvm;
mod irgen;
mod codegen;

fn main()
{
    match cli::run(&cli::Options::new(std::env::args().collect()))
    {
        Ok(()) => {},
        Err(error) => {eprintln!("{}\nCompilation Terminated", error);}
    }
}