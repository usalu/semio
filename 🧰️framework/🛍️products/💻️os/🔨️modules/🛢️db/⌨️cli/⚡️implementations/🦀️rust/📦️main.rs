fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(db_cli::main_impl(&args));
}
