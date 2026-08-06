fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(crate::os_pack::cli::main_impl(&args));
}
