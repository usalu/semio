//! 🕸️ CLI: `semio-os-mcp stdio [--folder <dir> | --hub <url> --space <id> [--token <t>]]
//! [--principal <id>] [--scopes a,b]` and `semio-os-mcp http [--port <p>] [--bind <addr>] --token <t>
//! [--folder <dir> | --hub <url> --space <id> [--token <t>]] [--principal <id>] [--scopes a,b]
//! [--audit-dir <dir>] [--allow-origin <origin>]… [--bridge-token-file <path>]` (P1b + P1c +
//! P7-headless-workspace) — this binary owns argv parsing only; all real logic lives in
//! `semio_framework_os_mcp::{run_stdio, run_http}` (P1a's brief §2.5, "keep main thin, all logic in
//! the lib" — mirrors `🏃️run/📦️bin.rs`'s own split). Unknown modes exit with a clear message rather
//! than silently doing nothing. `--folder`/`--hub` are mutually exclusive (`📋️master.md` §2.1:
//! "`--folder <space dir>`…`--hub <url> --space <id> --token <t>`"); `http`'s own `--token` (the
//! `/mcp` bearer) is a distinct flag from the hub's `--token`, so a hub-bound `http` session passes
//! `--token <bearer> --hub <url> --space <id> --token <hub token>` — the SECOND `--token` after
//! `--hub`/`--space` binds the hub, matching `--folder`'s own "whichever flag is still empty when
//! this one is seen" parsing order below. The `/bridge` websocket secret (P1c) is NEVER an argv value
//! at all — `run_http` mints it fresh every start; `--bridge-token-file` only chooses where that
//! minted secret is written (default `~/.semio/agent/bridge-token`).
use semio_framework_os_mcp::{HttpOptions, HubOptions, StdioOptions};

//#region 🔖️Args
enum Mode {
    Stdio(StdioOptions),
    Http(HttpOptions),
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(',').map(str::trim).filter(|scope| !scope.is_empty()).map(str::to_string).collect()
}

/// 🏠️ Shared `--hub <url> --space <id> [--token <t>]` builder — `hub_token` accumulates the token
/// seen AFTER `--hub`/`--space` (the hub's own auth token), distinct from `http`'s own bearer
/// `--token` flag which `parse_http_args` handles separately.
#[derive(Default)]
struct HubArgs {
    base_url: Option<String>,
    space_id: Option<String>,
    token: Option<String>,
}

impl HubArgs {
    fn into_options(self) -> Result<Option<HubOptions>, String> {
        match (self.base_url, self.space_id) {
            (None, None) => Ok(None),
            (Some(base_url), Some(space_id)) => Ok(Some(HubOptions { base_url, space_id, token: self.token })),
            _ => Err("--hub requires --space <id> (and vice versa)".to_string()),
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
            "--token" => hub.token = Some(argv.next().ok_or("--token requires a value")?),
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
    let mut bearer_token: Option<String> = None;
    let mut folder = None;
    let mut hub = HubArgs::default();
    let mut principal = None;
    let mut scopes = Vec::new();
    let mut audit_dir = None;
    let mut allow_origin = Vec::new();
    let mut bridge_token_file = None;
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--port" => port = argv.next().ok_or("--port requires a value")?.parse().map_err(|_| "--port must be a number".to_string())?,
            "--bind" => bind = argv.next().ok_or("--bind requires a value")?,
            // 🎯️ The FIRST `--token` seen binds the `/mcp` bearer (this mode's own required flag,
            // pre-existing since P1b); every subsequent `--token` binds the hub's own auth token —
            // matches this module doc's "second `--token`" convention. The `/bridge` secret is a
            // SEPARATE thing entirely (P1c): always freshly minted at startup, never taken from argv
            // — only WHERE it is written is configurable, via `--bridge-token-file`.
            "--token" if bearer_token.is_none() => bearer_token = Some(argv.next().ok_or("--token requires a value")?),
            "--token" => hub.token = Some(argv.next().ok_or("--token requires a value")?),
            "--folder" => folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--hub" => hub.base_url = Some(argv.next().ok_or("--hub requires a value")?),
            "--space" => hub.space_id = Some(argv.next().ok_or("--space requires a value")?),
            "--principal" => principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            "--audit-dir" => audit_dir = Some(argv.next().ok_or("--audit-dir requires a value")?),
            "--allow-origin" => allow_origin.push(argv.next().ok_or("--allow-origin requires a value")?),
            "--bridge-token-file" => bridge_token_file = Some(argv.next().ok_or("--bridge-token-file requires a value")?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let token = bearer_token.ok_or("http mode requires --token <t>")?;
    let hub = hub.into_options()?;
    if folder.is_some() && hub.is_some() {
        return Err("--folder and --hub are mutually exclusive".to_string());
    }
    Ok(HttpOptions { port, bind, token, folder, hub, principal, scopes, audit_dir, allow_origin, bridge_token_file })
}

fn parse_args() -> Result<Mode, String> {
    let mut argv = std::env::args().skip(1);
    let Some(mode) = argv.next() else {
        return Err("usage: semio-os-mcp <stdio|http> [--folder <dir> | --hub <url> --space <id> [--token <t>]] [--principal <id>] [--scopes a,b] [http-only: --port <p> --bind <addr> --token <t> --audit-dir <dir> --allow-origin <origin>]".to_string());
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
