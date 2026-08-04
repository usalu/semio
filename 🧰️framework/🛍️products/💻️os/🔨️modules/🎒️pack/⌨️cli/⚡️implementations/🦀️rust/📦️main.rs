fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(pack_cli::main_impl(&args));
}
