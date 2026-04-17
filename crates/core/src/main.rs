use std::{
    io::{BufRead, stdin},
    process::exit,
};

// TODO: make proper CLI
// TODO: add integration tests
fn main() {
    let lines = &mut stdin().lock().lines().map_while(Result::ok);

    let rules = match parse::parse_make_p(lines) {
        Ok(rules) => rules,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    dot::print_dot_graph(rules);
}
