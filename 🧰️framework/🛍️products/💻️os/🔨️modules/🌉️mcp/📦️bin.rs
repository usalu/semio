//! 🕸️ CLI: `semio-os-mcp stdio [--folder <dir>] [--principal <id>] [--scopes a,b]` — this binary owns
//! argv parsing only; all real logic lives in `semio_framework_os_mcp::run_stdio` (this packet's
//! brief §2.5, "keep main thin, all logic in the lib" — mirrors `🏃️run/📦️bin.rs`'s own split).
//! Unknown or not-yet-implemented modes (HTTP arrives with P1b) exit with a clear message rather than
//! silently doing nothing.

use semio_framework_os_mcp::StdioOptions;

//#region 🔖️Args
enum Mode {
    Stdio(StdioOptions),
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(',').map(str::trim).filter(|scope| !scope.is_empty()).map(str::to_string).collect()
}

fn parse_args() -> Result<Mode, String> {
    let mut argv = std::env::args().skip(1);
    let Some(mode) = argv.next() else {
        return Err("usage: semio-os-mcp <stdio> [--folder <dir>] [--principal <id>] [--scopes a,b]".to_string());
    };
    match mode.as_str() {
        "stdio" => {
            let mut options = StdioOptions::default();
            while let Some(flag) = argv.next() {
                match flag.as_str() {
                    "--folder" => options.folder = Some(argv.next().ok_or("--folder requires a value")?),
                    "--principal" => options.principal = Some(argv.next().ok_or("--principal requires a value")?),
                    "--scopes" => options.scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
                    other => return Err(format!("unknown flag {other}")),
                }
            }
            Ok(Mode::Stdio(options))
        }
        other => Err(format!("unknown mode `{other}` — only `stdio` is implemented by this packet (P1a); HTTP arrives with P1b")),
    }
}
//#endregion 🔖️Args

fn main() {
    let mode = match parse_args() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("[semio-os-mcp] {message}");
            std::process::exit(1);
        }
    };
    let result = match mode {
        Mode::Stdio(options) => semio_framework_os_mcp::run_stdio(options),
    };
    if let Err(error) = result {
        eprintln!("[semio-os-mcp] {:?}: {}", error.code, error.message);
        std::process::exit(1);
    }
}
