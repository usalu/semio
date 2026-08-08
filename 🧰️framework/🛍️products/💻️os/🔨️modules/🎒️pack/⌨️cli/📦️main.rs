fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(semio_framework_os_kernel::os_pack::cli::main_impl(&args));
}
