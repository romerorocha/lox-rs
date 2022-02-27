mod scanning;

fn main() {
    if let Err(e) = scanning::get_args().and_then(scanning::run_file) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
