use unicode_prettytable::*;

mod cli;

fn main() {
    let cli = cli::create_cli();
    let matches = cli.get_matches();
}
