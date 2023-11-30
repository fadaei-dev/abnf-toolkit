mod cli;
mod lexer;
mod position;
mod report;
mod report_kind;
mod run;
mod token;
mod token_kind;

fn main() {
    run::run();
}
