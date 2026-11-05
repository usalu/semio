fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(semio_os_kernel::os_semio::cli::main_impl(&args));
}
