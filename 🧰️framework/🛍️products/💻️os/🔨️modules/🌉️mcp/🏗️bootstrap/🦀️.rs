//! 🕸️ CLI: `semio-os-mcp stdio [--folder <dir> | --hub <url> --space <id>]
//! [--principal <id>] [--scopes a,b] [--auto-approve never|readonly|all] [--no-bridge]` and
//! `semio-os-mcp http [--port <p>] [--bind <addr>]
//! [--folder <dir> | --hub <url> --space <id>] [--principal <id>] [--scopes a,b]
//! [--auto-approve never|readonly|all] [--audit-dir <dir>] [--allow-origin <origin>]…` (P1b + P1c +
//! P7-headless-workspace) — this binary owns argv parsing only; all real logic lives in
//! `semio_framework_os_mcp::{run_stdio, run_http}`. `semio-os-mcp schemas` additionally prints the
//! `os.mcp` draft-07 schema mirror on stdout (the `schema-mirror` nx target's generator) (P1a's brief §2.5, "keep main thin, all logic in
//! the lib" — mirrors `🏃️run/🏗️bootstrap/🦀️.rs`'s own split). Unknown modes exit with a clear message rather
//! than silently doing nothing. `--folder`/`--hub` are mutually exclusive (`📋️master.md` §2.1:
//! "`--folder <space dir>`…`--hub <url> --space <id>`"). Hub authority is claimed from protected fd 3
//! before argv parsing and never enters argv or workspace state. HTTP and bridge admission reuse
//! that protected authority without copying it into argv, a URL, a file, logs, or protocol output.
use semio_framework_os_mcp::{AgentCredentialSource, AutoApprovePolicy, HttpOptions, HubOptions, StdioOptions};

//#region 🔖️Args
enum Mode {
    Stdio(StdioOptions),
    Http(HttpOptions),
    /// 🚨️ `semio-os-mcp audit [--folder <dir>]` — compiles the catalog source from the committed
    /// plugin descriptors under `<dir>` and prints every `CatalogAuditFinding` (a gesture-named
    /// route published to agents with no declared audience; a delete/clear/replace mutation with
    /// `effects.destructive = false`). Exits 1 when the list is non-empty, so it is a gate.
    Audit { folder: String },
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(',').map(str::trim).filter(|scope| !scope.is_empty()).map(str::to_string).collect()
}

/// 🚦️ `--auto-approve never|readonly|all` — the launch-time human decision that waives the approval
/// gate when no `elicitation`-capable client and no attached OS shell can be asked. An unknown value
/// is a hard argv error, never a silent downgrade to `never`.
fn parse_auto_approve(raw: &str) -> Result<AutoApprovePolicy, String> {
    AutoApprovePolicy::parse(raw).ok_or_else(|| format!("--auto-approve expects never|readonly|all, got `{raw}`"))
}

/// 🏠️ Shared `--hub <url> --space <id>` selector, plus the *path* of the delegated agent
/// credential — never the credential itself, because argv is world-readable through `ps`.
///
/// Before this flag existed, `--hub` could only authenticate through the inherited fd-3
/// local-bootstrap envelope, which a `dev s` launcher mints for its own children. No MCP client
/// configuration — `claude_desktop_config.json`, `.mcp.json`, `.cursor/mcp.json` — can pass a file
/// descriptor, so `--hub` was mechanically unreachable for an end user. `--credential-file <path>`
/// is the reachable shape: a `0600` file the delegation UI hands the human once, named in the
/// client config.
#[derive(Default)]
struct HubArgs {
    base_url: Option<String>,
    space_id: Option<String>,
    credential_file: Option<String>,
    credential_fd: Option<i32>,
}

impl HubArgs {
    fn into_options(self) -> Result<Option<HubOptions>, String> {
        let credential = match (self.credential_file, self.credential_fd) {
            (Some(_), Some(_)) => return Err("--credential-file and --credential-fd are mutually exclusive".to_string()),
            (Some(path), None) => Some(AgentCredentialSource::File(path)),
            (None, Some(descriptor)) => Some(AgentCredentialSource::Descriptor(descriptor)),
            (None, None) => None,
        };
        match (self.base_url, self.space_id) {
            (None, None) if credential.is_some() => Err("an agent credential requires --hub <url> --space <id>".to_string()),
            (None, None) => Ok(None),
            (Some(base_url), Some(space_id)) => Ok(Some(HubOptions { base_url, space_id, credential })),
            (Some(_), None) => Err("--hub requires --space <id>".to_string()),
            (None, Some(_)) => Err("--space requires --hub <url>".to_string()),
        }
    }
}

/// 🔢️ `--credential-fd <n>`. In `stdio` mode descriptors 0/1/2 are this process's MCP framing
/// channel and descriptor 3 is the local-bootstrap envelope, so none of them may be re-used for a
/// delegation — a launcher that wants to pipe the credential passes another inherited descriptor.
fn parse_credential_fd(raw: &str, stdio_mode: bool) -> Result<i32, String> {
    let descriptor: i32 = raw.parse().map_err(|_| "--credential-fd must be a descriptor number".to_string())?;
    if descriptor < 0 {
        return Err("--credential-fd must be non-negative".to_string());
    }
    if descriptor == 3 {
        return Err("--credential-fd 3 is the local-bootstrap envelope; pass another descriptor".to_string());
    }
    if stdio_mode && descriptor <= 2 {
        return Err("--credential-fd 0/1/2 carry this process's stdio MCP framing; pass another descriptor or use --credential-file".to_string());
    }
    Ok(descriptor)
}

fn parse_stdio_args(argv: &mut impl Iterator<Item = String>) -> Result<StdioOptions, String> {
    let mut options = StdioOptions::default();
    let mut hub = HubArgs::default();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--folder" => options.folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--hub" => hub.base_url = Some(argv.next().ok_or("--hub requires a value")?),
            "--space" => hub.space_id = Some(argv.next().ok_or("--space requires a value")?),
            "--credential-file" => hub.credential_file = Some(argv.next().ok_or("--credential-file requires a path")?),
            "--credential-fd" => hub.credential_fd = Some(parse_credential_fd(&argv.next().ok_or("--credential-fd requires a descriptor number")?, true)?),
            "--principal" => options.principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => options.scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            "--auto-approve" => options.auto_approve = parse_auto_approve(&argv.next().ok_or("--auto-approve requires never|readonly|all")?)?,
            "--no-bridge" => options.no_bridge = true,
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
    let mut auto_approve = AutoApprovePolicy::default();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--port" => port = argv.next().ok_or("--port requires a value")?.parse().map_err(|_| "--port must be a number".to_string())?,
            "--bind" => bind = argv.next().ok_or("--bind requires a value")?,
            "--folder" => folder = Some(argv.next().ok_or("--folder requires a value")?),
            "--hub" => hub.base_url = Some(argv.next().ok_or("--hub requires a value")?),
            "--space" => hub.space_id = Some(argv.next().ok_or("--space requires a value")?),
            "--credential-file" => hub.credential_file = Some(argv.next().ok_or("--credential-file requires a path")?),
            "--credential-fd" => hub.credential_fd = Some(parse_credential_fd(&argv.next().ok_or("--credential-fd requires a descriptor number")?, false)?),
            "--principal" => principal = Some(argv.next().ok_or("--principal requires a value")?),
            "--scopes" => scopes = parse_scopes(&argv.next().ok_or("--scopes requires a comma-separated value")?),
            "--audit-dir" => audit_dir = Some(argv.next().ok_or("--audit-dir requires a value")?),
            "--allow-origin" => allow_origin.push(argv.next().ok_or("--allow-origin requires a value")?),
            "--auto-approve" => auto_approve = parse_auto_approve(&argv.next().ok_or("--auto-approve requires never|readonly|all")?)?,
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let hub = hub.into_options()?;
    if folder.is_some() && hub.is_some() {
        return Err("--folder and --hub are mutually exclusive".to_string());
    }
    Ok(HttpOptions { port, bind, folder, hub, principal, scopes, audit_dir, allow_origin, auto_approve })
}

/// 🚨️ `audit` takes one optional `--folder <dir>` (default `.`) and nothing else — it never opens a
/// workspace, never claims hub authority and never talks to a shell; it reads committed descriptors.
fn parse_audit_args(argv: &mut impl Iterator<Item = String>) -> Result<String, String> {
    let mut folder = ".".to_string();
    while let Some(flag) = argv.next() {
        match flag.as_str() {
            "--folder" => folder = argv.next().ok_or("--folder requires a value")?,
            other => return Err(format!("unknown flag {other}")),
        }
    }
    Ok(folder)
}

fn parse_args() -> Result<Mode, String> {
    let mut argv = std::env::args().skip(1);
    let Some(mode) = argv.next() else {
        return Err(
            "usage: semio-os-mcp <stdio|http|audit|schemas> [--folder <dir> | --hub <url> --space <id> [--credential-file <path> | --credential-fd <n>]] [--principal <id>] [--scopes a,b] [--auto-approve never|readonly|all] [stdio-only: --no-bridge] [http-only: --port <p> --bind <addr> --audit-dir <dir> --allow-origin <origin>]".to_string()
        );
    };
    match mode.as_str() {
        "stdio" => Ok(Mode::Stdio(parse_stdio_args(&mut argv)?)),
        "http" => Ok(Mode::Http(parse_http_args(&mut argv)?)),
        "audit" => Ok(Mode::Audit { folder: parse_audit_args(&mut argv)? }),
        other => Err(format!("unknown mode `{other}` — only `stdio`/`http`/`audit`/`schemas` are implemented by this binary")),
    }
}
//#endregion 🔖️Args

/// 🏷️ Exact hub-authority carriers the OS/hub processes export by name — `S_USER`/`VITE_S_USER`
/// (`🏛️ShellHost`'s identity pin), `S_HUB_URL`/`VITE_S_HUB_URL` (`🌐️vite`'s hub origin define) and
/// `S_SESSION` (the local-bootstrap session pin). Every one of them is stripped by
/// `🌎️hub/🔐️auth/📤️credential-delivery`'s `sealedDirectChildEnvironment`, so seeing one here means
/// this process was NOT spawned through the sealing path.
const PROTECTED_CREDENTIAL_NAMES: [&str; 6] = ["S_USER", "VITE_S_USER", "S_HUB_URL", "VITE_S_HUB_URL", "S_SESSION", "VITE_S_SESSION"];

/// 🧱 Transport-credential carriers that are authority by their own bare name, whatever sets them.
const PROTECTED_CREDENTIAL_HEADER_NAMES: [&str; 2] = ["AUTHORIZATION", "COOKIE"];

/// 🔤️ The only namespaces whose variables this OS product mints; a credential-shaped suffix under
/// one of them is a hub carrier, while the same word under a foreign namespace is not.
const PROTECTED_CREDENTIAL_NAMESPACES: [&str; 2] = ["S_", "VITE_S_"];

/// 🚨️ Credential-shaped suffixes, matched only inside a namespace above.
const PROTECTED_CREDENTIAL_MARKERS: [&str; 7] = ["TOKEN", "SESSION", "CREDENTIAL", "BEARER", "CAPABILITY", "AUTHORIZATION", "COOKIE"];

/// 🔍️ Decides whether one already-upper-cased variable name carries hub authority. Deliberately NOT
/// a bare substring scan of the whole environment: a host harness that names its own benign
/// variables `CLAUDE_CODE_SESSION_ID`, `TMUX_SESSION`, `GH_TOKEN` or `VSCODE_SESSION_ID` carries no
/// hub credential, and rejecting those made this binary unusable as the child of every session
/// oriented client — which is the one client `.mcp.json` exists for.
fn protected_credential_environment_name(key: &str) -> bool {
    PROTECTED_CREDENTIAL_NAMES.contains(&key)
        || PROTECTED_CREDENTIAL_HEADER_NAMES.contains(&key)
        || PROTECTED_CREDENTIAL_NAMESPACES
            .iter()
            .filter_map(|namespace| key.strip_prefix(namespace))
            .any(|suffix| PROTECTED_CREDENTIAL_MARKERS.iter().any(|marker| suffix.contains(marker)))
}

fn protected_credential_environment_is_absent() -> bool {
    std::env::vars_os().all(|(key, value)| {
        let key = key.to_string_lossy().to_ascii_uppercase();
        if key == "S_LOCAL_CREDENTIAL_FD" {
            return value == "3";
        }
        !protected_credential_environment_name(&key)
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
    // 🪞️ `semio-os-mcp schemas` prints the `os.mcp` scope's whole draft-07 schema document on stdout —
    // the generator behind `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`. It reads no
    // environment, no filesystem and no credential, so it deliberately runs BEFORE the process-entry
    // seal every serving mode is gated on.
    if std::env::args().nth(1).as_deref() == Some("schemas") {
        print!("{}", semio_framework_os_mcp::schema_mirror_json());
        return;
    }
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
    if let Mode::Audit { folder } = &mode {
        let source = semio_framework_os_mcp::registry::discover_catalog_source(Some(std::path::Path::new(folder)));
        let findings = semio_framework_os_mcp::catalog::audit_source(&source);
        for finding in &findings {
            println!("{}", finding.message());
        }
        println!("semio-os-mcp audit: {} finding(s) over {} descriptor(s) under {folder}", findings.len(), source.descriptors.len());
        std::process::exit(if findings.is_empty() { 0 } else { 1 });
    }
    let result = match mode {
        Mode::Stdio(options) => semio_framework_os_mcp::run_stdio(options),
        Mode::Http(options) => semio_framework_os_mcp::run_http(options),
        Mode::Audit { .. } => unreachable!("handled above"),
    };
    if let Err(error) = result {
        eprintln!("[semio-os-mcp] {:?}: {}", error.code, error.message);
        std::process::exit(1);
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️bin-quick/🦀️.rs"]
mod quick;
