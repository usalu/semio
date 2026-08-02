fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(protocol_cli::main_impl(&args));
}
