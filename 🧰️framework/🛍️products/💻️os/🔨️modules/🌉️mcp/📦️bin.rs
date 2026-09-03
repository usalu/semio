//! 🕸️ CLI: `semio-os-mcp stdio [--folder <dir> | --hub <url> --space <id>]
//! [--principal <id>] [--scopes a,b]` and `semio-os-mcp http [--port <p>] [--bind <addr>]
//! [--folder <dir> | --hub <url> --space <id>] [--principal <id>] [--scopes a,b]
//! [--audit-dir <dir>] [--allow-origin <origin>]…` (P1b + P1c +
//! P7-headless-workspace) — this binary owns argv parsing only; all real logic lives in
//! `semio_framework_os_mcp::{run_stdio, run_http}` (P1a's brief §2.5, "keep main thin, all logic in
//! the lib" — mirrors `🏃️run/📦️bin.rs`'s own split). Unknown modes exit with a clear message rather
//! than silently doing nothing. `--folder`/`--hub` are mutually exclusive (`📋️master.md` §2.1:
//! "`--folder <space dir>`…`--hub <url> --space <id>`"). Hub authority is claimed from protected fd 3
//! before argv parsing and never enters argv or workspace state. HTTP and bridge admission reuse
//! that protected authority without copying it into argv, a URL, a file, logs, or protocol output.
use semio_framework_os_mcp::{HttpOptions, HubOptions, StdioOptions};

//#region 🔖️Args
enum Mode {
    Stdio(StdioOptions),
    Http(HttpOptions),
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(',').map(str::trim).filter(|scope| !scope.is_empty()).map(str::to_string).collect()
}

/// 🏠️ Shared credential-free `--hub <url> --space <id>` selector.
#[derive(Default)]
struct HubArgs {
    base_url: Option<String>,
    space_id: Option<String>,
}

impl HubArgs {
    fn into_options(self) -> Result<Option<HubOptions>, String> {
        match (self.base_url, self.space_id) {
            (None, None) => Ok(None),
            (Some(base_url), Some(space_id)) => Ok(Some(HubOptions { base_url, space_id })),
            (Some(_), None) => Err("--hub requires --space <id>".to_string()),
            (None, Some(_)) => Err("--space requires --hub <url>".to_string()),
        }
    }
}

fn parse_stdio_args(argv: &mut impl Iterator<Item = String>) -> Result<StdioOptions, String> {
    let mut options = StdioOptions::default();
    let mut hub = HubArgs::default();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--folder" => options.folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--hub" => hub.base_url = Some(argv.next().ok_or("--hub requires a value")?),
            "--space" => hub.space_id = Some(argv.next().ok_or("--space requires a value")?),
            "--principal" => options.principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => options.scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    options.hub = hub.into_options()?;
    if options.folder.is_some() && options.hub.is_some() {
        return Err("--folder and --hub are mutually exclusive".to_string());
    }
    Ok(options)
}

fn parse_http_args(argv: &mut impl Iterator<Item = String>) -> Result<HttpOptions, String> {
    let mut port: u16 = 6300;
    let mut bind = "127.0.0.1".to_string();
    let mut folder = None;
    let mut hub = HubArgs::default();
    let mut principal = None;
    let mut scopes = Vec::new();
    let mut audit_dir = None;
    let mut allow_origin = Vec::new();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--port" => port = argv.next().ok_or("--port requires a value")?.parse().map_err(|_| "--port must be a number".to_string())?,
            "--bind" => bind = argv.next().ok_or("--bind requires a value")?,
            "--folder" => folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--hub" => hub.base_url = Some(argv.next().ok_or("--hub requires a value")?),
            "--space" => hub.space_id = Some(argv.next().ok_or("--space requires a value")?),
            "--principal" => principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            "--audit-dir" => audit_dir = Some(argv.next().ok_or("--audit-dir requires a value")?),
            "--allow-origin" => allow_origin.push(argv.next().ok_or("--allow-origin requires a value")?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let hub = hub.into_options()?;
    if folder.is_some() && hub.is_some() {
        return Err("--folder and --hub are mutually exclusive".to_string());
    }
    Ok(HttpOptions { port, bind, folder, hub, principal, scopes, audit_dir, allow_origin })
}

fn parse_args() -> Result<Mode, String> {
    let mut argv = std::env::args().skip(1);
    let Some(mode) = argv.next() else {
        return Err(
            "usage: semio-os-mcp <stdio|http> [--folder <dir> | --hub <url> --space <id>] [--principal <id>] [--scopes a,b] [http-only: --port <p> --bind <addr> --audit-dir <dir> --allow-origin <origin>]".to_string()
        );
    };
    match mode.as_str() {
        "stdio" => Ok(Mode::Stdio(parse_stdio_args(&mut argv)?)),
        "http" => Ok(Mode::Http(parse_http_args(&mut argv)?)),
        other => Err(format!("unknown mode `{other}` — only `stdio`/`http` are implemented by this binary")),
    }
}
//#endregion 🔖️Args

fn protected_credential_environment_is_absent() -> bool {
    std::env::vars_os().all(|(key, value)| {
        let key = key.to_string_lossy().to_ascii_uppercase();
        if key == "S_LOCAL_CREDENTIAL_FD" {
            return value == "3";
        }
        !(key == "S_USER"
                || key == "VITE_S_USER"
                || key == "S_HUB_URL"
                || key.contains("TOKEN")
                || key.contains("SESSION")
                || key.contains("CREDENTIAL")
                || key.contains("BEARER")
                || key.contains("CAPABILITY")
                || key.contains("AUTHORIZATION")
                || key.contains("COOKIE"))
    })
}

#[cfg(unix)]
fn inherited_credential_fd_is_closed() -> bool {
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    (unsafe { fcntl(3, 1) }) < 0
}

#[cfg(windows)]
fn inherited_credential_fd_is_closed() -> bool {
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
    }
    (unsafe { _get_osfhandle(3) }) == -1
}

fn benign_direct_child_environment_is_preserved() -> bool {
    std::env::var("SEMIO_DIRECT_CHILD_BENIGN").ok().as_deref() == Some("preserved")
}

fn main() {
    if !protected_credential_environment_is_absent() {
        eprintln!("[semio-os-mcp] protected parent environment was not sealed");
        std::process::exit(1);
    }
    let has_local_credential = std::env::var("S_LOCAL_CREDENTIAL_FD").ok().as_deref() == Some("3");
    if has_local_credential && !benign_direct_child_environment_is_preserved() {
        eprintln!("[semio-os-mcp] benign direct-child environment was not preserved");
        std::process::exit(1);
    }
    if has_local_credential
        && semio_framework_os_kernel::os_directory::identity::claim_inherited_local_hub_credential("mcp").is_err()
    {
        eprintln!("[semio-os-mcp] protected MCP credential claim failed");
        std::process::exit(1);
    }
    if std::env::args().any(|arg| arg == "--assert-no-local-credential-state") {
        std::process::exit(if inherited_credential_fd_is_closed() && protected_credential_environment_is_absent() && benign_direct_child_environment_is_preserved() { 0 } else { 1 });
    }
    if std::env::var("SEMIO_DIRECT_CHILD_PROBE").ok().as_deref() == Some("1") {
        let status = std::env::current_exe().ok().and_then(|executable| {
            std::process::Command::new(executable)
                .arg("--assert-no-local-credential-state")
                .env_remove("S_LOCAL_CREDENTIAL_FD")
                .env_remove("SEMIO_DIRECT_CHILD_PROBE")
                .status()
                .ok()
        });
        if !status.is_some_and(|status| status.success()) {
            eprintln!("[semio-os-mcp] direct-child descendant seal failed");
            std::process::exit(1);
        }
    }
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

#[cfg(test)]
mod quick {
    use super::*;

    #[test]
    fn authenticated_hub_workspace_cli_contains_no_hub_credential_carrier() {
        let stdio = parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
        assert_eq!(stdio.hub, Some(HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into() }));
        assert!(parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a", "--token", "forbidden"].into_iter().map(str::to_string)).is_err());

        let options = parse_http_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
        let hub = options.hub.expect("authenticated hub binding");
        assert_eq!(hub, HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into() });
        assert!(parse_http_args(&mut ["--token", "forbidden"].into_iter().map(str::to_string)).is_err());
        assert!(parse_http_args(&mut ["--bridge-token-file", "forbidden"].into_iter().map(str::to_string)).is_err());
    }
}
