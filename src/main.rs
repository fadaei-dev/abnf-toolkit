mod cli;
mod lexer;
mod position;
mod report;
mod run;
mod token;
mod token_kind;

fn main() {
    run::run();
}
