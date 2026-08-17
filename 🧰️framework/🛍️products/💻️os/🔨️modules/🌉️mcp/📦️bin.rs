//! 🕸️ CLI: `semio-os-mcp stdio [--folder <dir>] [--principal <id>] [--scopes a,b]` and
//! `semio-os-mcp http [--port <p>] [--bind <addr>] --token <t> [--folder <dir>] [--principal <id>]
//! [--scopes a,b] [--audit-dir <dir>] [--allow-origin <origin>]…` (P1b) — this binary owns argv
//! parsing only; all real logic lives in `semio_framework_os_mcp::{run_stdio, run_http}` (P1a's
//! brief §2.5, "keep main thin, all logic in the lib" — mirrors `🏃️run/📦️bin.rs`'s own split).
//! Unknown modes exit with a clear message rather than silently doing nothing.

use semio_framework_os_mcp::{HttpOptions, StdioOptions};

//#region 🔖️Args
enum Mode {
    Stdio(StdioOptions),
    Http(HttpOptions),
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(',').map(str::trim).filter(|scope| !scope.is_empty()).map(str::to_string).collect()
}

fn parse_stdio_args(argv: &mut impl Iterator<Item = String>) -> Result<StdioOptions, String> {
    let mut options = StdioOptions::default();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--folder" => options.folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--principal" => options.principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => options.scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    Ok(options)
}

fn parse_http_args(argv: &mut impl Iterator<Item = String>) -> Result<HttpOptions, String> {
    let mut port: u16 = 6300;
    let mut bind = "127.0.0.1".to_string();
    let mut token: Option<String> = None;
    let mut folder = None;
    let mut principal = None;
    let mut scopes = Vec::new();
    let mut audit_dir = None;
    let mut allow_origin = Vec::new();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--port" => port = argv.next().ok_or("--port requires a value")?.parse().map_err(|_| "--port must be a number".to_string())?,
            "--bind" => bind = argv.next().ok_or("--bind requires a value")?,
            "--token" => token = Some(argv.next().ok_or("--token requires a value")?),
            "--folder" => folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--principal" => principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            "--audit-dir" => audit_dir = Some(argv.next().ok_or("--audit-dir requires a value")?),
            "--allow-origin" => allow_origin.push(argv.next().ok_or("--allow-origin requires a value")?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let token = token.ok_or("http mode requires --token <t>")?;
    Ok(HttpOptions { port, bind, token, folder, principal, scopes, audit_dir, allow_origin })
}

fn parse_args() -> Result<Mode, String> {
    let mut argv = std::env::args().skip(1);
    let Some(mode) = argv.next() else {
        return Err("usage: semio-os-mcp <stdio|http> [--folder <dir>] [--principal <id>] [--scopes a,b] [http-only: --port <p> --bind <addr> --token <t> --audit-dir <dir> --allow-origin <origin>]".to_string());
    };
    match mode.as_str() {
        "stdio" => Ok(Mode::Stdio(parse_stdio_args(&mut argv)?)),
        "http" => Ok(Mode::Http(parse_http_args(&mut argv)?)),
        other => Err(format!("unknown mode `{other}` — only `stdio`/`http` are implemented by this binary")),
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
        Mode::Http(options) => semio_framework_os_mcp::run_http(options),
    };
    if let Err(error) = result {
        eprintln!("[semio-os-mcp] {:?}: {}", error.code, error.message);
        std::process::exit(1);
    }
}
