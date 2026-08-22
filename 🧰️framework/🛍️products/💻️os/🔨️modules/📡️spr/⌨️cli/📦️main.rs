fn main() {
    // 🚫️async: E5 executor bridge — bin entry point, sanctioned by R4 clause 1
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(semio_framework_async::block_on(semio_framework_os_kernel::os_spr::cli::main_impl(&args)));
}
