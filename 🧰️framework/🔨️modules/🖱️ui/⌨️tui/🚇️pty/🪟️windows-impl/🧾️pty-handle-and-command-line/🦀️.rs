//! 🧾️ Windows pty handle and command line: the `Pty` handle, the wide string and batch file helpers, the command line and environment block builders and the program resolver. See [`super`].

use super::*;

/// 🪞️ Windows ConPTY master pipes plus child process.
///
/// The console's pipes have no readiness to wait for, so one thread blocks on the output pipe, one
/// on the input pipe and one on the process; each fills or drains a bounded queue and calls the
/// notifier. Every method of `Pty` therefore returns at once, which is what lets a single thread
/// serve many sessions; termination closes the job, which ends all three threads.
pub struct Pty {
    pub(super) input: Arc<Pipe>,
    pub(super) output: Arc<Pipe>,
    pub(super) hpcon: Option<OwnedPseudoConsole>,
    pub(super) process: Arc<Shared>,
    pub(super) _thread: OwnedHandle,
    pub(super) job: OwnedHandle,
    pub(super) notifier: Arc<Mutex<Option<Notifier>>>,
    pub(super) batch: bool,
}

pub(super) fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn is_command_interpreter(program: &str) -> bool {
    Path::new(program).file_stem().is_some_and(|stem| stem.eq_ignore_ascii_case("cmd"))
}

fn is_batch_file(program: &str) -> bool {
    Path::new(program).extension().is_some_and(|extension| extension.eq_ignore_ascii_case("cmd") || extension.eq_ignore_ascii_case("bat"))
}

/// 🔧 Whether the program is a batch file or a command interpreter asked to run a string, which is what asks
/// "Terminate batch job (Y/N)?" when it is interrupted.
pub(super) fn runs_batch(cmd: &str, args: &[&str]) -> bool {
    is_batch_file(cmd) || is_command_interpreter(cmd) && args.iter().any(|arg| arg.eq_ignore_ascii_case("/c") || arg.eq_ignore_ascii_case("/k"))
}

/// 📜 The command line of a program. `cmd.exe` is the one program that does not read its line with the C runtime's
/// rules: asked to run a string (`/c` or `/k`) it gets `/s` and the string, joined by spaces, inside one pair of
/// quotes that it removes and nothing else, so the string reaches the shell exactly as written. Everything else
/// is quoted for the C runtime.
///
/// @see https://learn.microsoft.com/windows-server/administration/windows-commands/cmd
pub(super) fn build_cmdline(cmd: &str, args: &[&str]) -> Vec<u16> {
    if is_command_interpreter(cmd) {
        if let Some(at) = args.iter().position(|arg| arg.eq_ignore_ascii_case("/c") || arg.eq_ignore_ascii_case("/k")) {
            let mut line = quote_argument(cmd);
            for flag in &args[..at] {
                line.push(' ');
                if flag.contains([' ', '\t', '"']) { line.push_str(&quote_argument(flag)); } else { line.push_str(flag); }
            }
            if !args[..at].iter().any(|flag| flag.eq_ignore_ascii_case("/s")) { line.push_str(" /s"); }
            line.push(' ');
            line.push_str(args[at]);
            line.push_str(" \"");
            line.push_str(&args[at + 1..].join(" "));
            line.push('"');
            return to_wide(&line);
        }
    }
    let mut line = String::new();
    for arg in std::iter::once(cmd).chain(args.iter().copied()) {
        if !line.is_empty() { line.push(' '); }
        line.push_str(&quote_argument(arg));
    }
    to_wide(&line)
}

fn quote_argument(arg: &str) -> String {
    let mut line = String::new();
    line.push('"');
    let mut slashes = 0;
    for ch in arg.chars() {
        if ch == '\\' { slashes += 1; continue; }
        line.extend(std::iter::repeat_n('\\', if ch == '"' { slashes * 2 + 1 } else { slashes }));
        line.push(ch);
        slashes = 0;
    }
    line.extend(std::iter::repeat_n('\\', slashes * 2));
    line.push('"');
    line
}

fn encode_env_block(mut merged: std::collections::BTreeMap<String, (std::ffi::OsString, std::ffi::OsString)>) -> Vec<u16> {
    let mut block = Vec::new();
    if let Some((_, value)) = merged.get_mut("PATH") {
        let mut seen = std::collections::HashSet::new();
        let paths: Vec<_> = std::env::split_paths(value).filter(|path| seen.insert(path.to_string_lossy().to_uppercase())).collect();
        if let Ok(path) = std::env::join_paths(paths) { *value = path; }
    }
    for (k, v) in merged.values() {
        block.extend(k.encode_wide());
        block.push(b'=' as u16);
        block.extend(v.encode_wide());
        block.push(0);
    }
    block.push(0);
    block
}

pub(super) fn build_env_block(env: &[(&str, &str)], remove_env: &[&str]) -> Option<Vec<u16>> {
    let mut merged: std::collections::BTreeMap<String, (std::ffi::OsString, std::ffi::OsString)> = std::env::vars_os().map(|(key, value)| (key.to_string_lossy().to_uppercase(), (key.clone(), std::env::var_os(&key).unwrap_or(value)))).collect();
    if let Some(path) = std::env::var_os("PATH") { merged.insert("PATH".into(), ("PATH".into(), path)); }
    for key in remove_env { merged.remove(&key.to_uppercase()); }
    for (key, value) in env { merged.insert(key.to_uppercase(), (key.into(), value.into())); }
    Some(encode_env_block(merged))
}

/// 🔍 The file `cmd` names in `env`: a path as given, a bare name on the `PATH` of `env` with the
/// extensions of its `PATHEXT`, relative paths from `cwd`.
///
/// @see https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
pub fn resolve_program(cmd: &str, env: &[(String, String)], cwd: Option<&Path>) -> Option<std::path::PathBuf> {
    let value = |name: &str| env.iter().find(|(key, _)| key.eq_ignore_ascii_case(name)).map(|(_, value)| value.as_str());
    let extensions: Vec<&str> = value("PATHEXT").unwrap_or(".COM;.EXE;.BAT;.CMD").split(';').filter(|extension| !extension.is_empty()).collect();
    let is_script = |extension: &str| extension.eq_ignore_ascii_case(".bat") || extension.eq_ignore_ascii_case(".cmd");
    let (scripts, executables): (Vec<&str>, Vec<&str>) = extensions.iter().copied().partition(|extension| is_script(extension));
    let candidate = |base: std::path::PathBuf, extensions: &[&str]| -> Option<std::path::PathBuf> {
        if base.extension().is_some() && base.is_file() { return Some(base); }
        extensions.iter().map(|extension| std::path::PathBuf::from(format!("{}{extension}", base.display()))).find(|path| path.is_file())
    };
    if cmd.contains(['\\', '/']) || Path::new(cmd).is_absolute() {
        let path = Path::new(cmd);
        let base = if path.is_absolute() { path.to_path_buf() } else { cwd.map_or_else(|| path.to_path_buf(), |cwd| cwd.join(path)) };
        return candidate(base.clone(), &executables).or_else(|| candidate(base, &scripts));
    }
    let directories: Vec<std::path::PathBuf> = std::env::split_paths(value("PATH")?).collect();
    directories.iter().find_map(|directory| candidate(directory.join(cmd), &executables)).or_else(|| directories.iter().find_map(|directory| candidate(directory.join(cmd), &scripts)))
}

pub(super) fn build_exact_env_block(env: &[(String, String)]) -> Vec<u16> {
    encode_env_block(env.iter().map(|(key, value)| (key.to_uppercase(), (key.into(), value.into()))).collect())
}
