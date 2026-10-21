//! 🎟️ `semio-build-scheduler`: the `RUSTC_WRAPPER` of the release build driver.
//!
//! Every rustc invocation that compiles a crate asks the driver's broker for memory admission with its
//! unit identity, keeps the jobserver token Cargo gave its job while it waits (handing it back would let Cargo's
//! helper thread spend it on another unit while this one can never reacquire it), runs rustc, streams the
//! child's resident memory every second and reports the measured peak, CPU time and rmeta time. Any
//! other invocation (version and print probes) and any build without a broker runs rustc unchanged.
//! The wire protocol is the `🎟️admission/🧬️schema/🔣️.json` `$defs` `AdmitRequestV1`, `VerdictV1`,
//! `RssReportV1` and `DoneReportV1`.
//! @see ../../📓️2026-10-13-scalable-build-architecture.md §6.2 (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT)

use std::env;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 🧾️ The identity and graph facts of one rustc unit, read from its arguments and Cargo's environment.
#[derive(Debug, Clone, PartialEq)]
struct Unit {
    crate_name: String,
    package: String,
    version: String,
    target: String,
    crate_types: Vec<String>,
    out_dir: String,
    extra_filename: String,
    source: String,
    manifest_dir: String,
    primary: bool,
    build_script_out_dir: Option<String>,
    externs: Vec<String>,
}

/// 🧷️ Options whose value is the next argument, so it is never mistaken for the crate root source.
const VALUED: [&str; 16] = ["--crate-name", "--crate-type", "--edition", "--emit", "--out-dir", "--target", "--extern", "-C", "-L", "-l", "--cfg", "--check-cfg", "--cap-lints", "--error-format", "--json", "-Z"];

/// 🔎️ Reads the unit a compile invocation builds, `None` for probes (`-vV`, `--print`) and arguments that are not UTF-8.
fn parse_unit(args: &[String], var: &dyn Fn(&str) -> Option<String>) -> Option<Unit> {
    let mut crate_name = None;
    let (mut crate_types, mut externs) = (Vec::new(), Vec::new());
    let (mut target, mut out_dir, mut extra, mut source) = (None, None, String::new(), None);
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        let (key, inline) = match arg.split_once('=') {
            Some((key, value)) if key.starts_with("--") => (key, Some(value.to_string())),
            _ => (arg, None),
        };
        if arg == "-vV" || arg == "-V" || key == "--print" || arg == "-" { return None; }
        let joined_codegen = arg.strip_prefix("-C").filter(|rest| !rest.is_empty()).map(str::to_string);
        let value = if inline.is_some() || joined_codegen.is_some() { inline.clone().or(joined_codegen.clone()) } else if VALUED.contains(&key) { index += 1; args.get(index).cloned() } else { None };
        match (key, value) {
            ("--crate-name", Some(value)) => crate_name = Some(value),
            ("--crate-type", Some(value)) => crate_types.extend(value.split(',').map(str::to_string)),
            ("--target", Some(value)) => target = Some(value),
            ("--out-dir", Some(value)) => out_dir = Some(value),
            ("--extern", Some(value)) => { if let Some((_, path)) = value.split_once('=') { externs.push(path.to_string()); } }
            (flag, Some(value)) if flag.starts_with("-C") => { if let Some(name) = value.strip_prefix("extra-filename=") { extra = name.to_string(); } }
            (other, None) if !other.starts_with('-') && other.ends_with(".rs") => source = Some(other.to_string()),
            _ => {}
        }
        index += 1;
    }
    let crate_name = crate_name?;
    if crate_types.is_empty() { crate_types.push("lib".to_string()); }
    Some(Unit {
        crate_name,
        package: var("CARGO_PKG_NAME").unwrap_or_default(),
        version: var("CARGO_PKG_VERSION").unwrap_or_default(),
        target: target.unwrap_or_else(|| "host".to_string()),
        crate_types,
        out_dir: out_dir.unwrap_or_default(),
        extra_filename: extra,
        source: source.unwrap_or_default(),
        manifest_dir: var("CARGO_MANIFEST_DIR").unwrap_or_default(),
        primary: var("CARGO_PRIMARY_PACKAGE").is_some(),
        build_script_out_dir: var("OUT_DIR"),
        externs,
    })
}

/// 📍️ Resolves a path rustc received relative to its working directory, so the broker sees the same file.
fn absolute(path: &str) -> String {
    if path.is_empty() || std::path::Path::new(path).is_absolute() { return path.to_string(); }
    env::current_dir().map(|directory| directory.join(path).to_string_lossy().into_owned()).unwrap_or_else(|_| path.to_string())
}

/// 🔤️ Encodes one JSON string literal.
fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", control as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// 📨️ The canonical `AdmitRequestV1` line of a unit.
fn admit_request(unit: &Unit, pass: &str, pid: u32) -> String {
    let list = |values: &[String]| values.iter().map(|value| json_string(value)).collect::<Vec<_>>().join(",");
    format!(
        "{{\"v\":1,\"op\":\"admit\",\"pass\":{},\"pid\":{},\"crate\":{},\"package\":{},\"version\":{},\"target\":{},\"crateTypes\":[{}],\"outDir\":{},\"extraFilename\":{},\"source\":{},\"manifestDir\":{},\"primary\":{},\"buildScriptOutDir\":{},\"externs\":[{}]}}\n",
        json_string(pass), pid, json_string(&unit.crate_name), json_string(&unit.package), json_string(&unit.version), json_string(&unit.target), list(&unit.crate_types),
        json_string(&unit.out_dir), json_string(&unit.extra_filename), json_string(&unit.source), json_string(&unit.manifest_dir), unit.primary,
        unit.build_script_out_dir.as_deref().map(json_string).unwrap_or_else(|| "null".to_string()), list(&unit.externs)
    )
}

/// ⏱️ Milliseconds since the Unix epoch, the time base every driver process shares.
fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0)
}

/// 📏️ Peak resident bytes and CPU milliseconds of a reaped child.
struct Usage {
    exit: i32,
    peak_rss: u64,
    user_ms: u64,
    sys_ms: u64,
}

#[cfg(unix)]
mod unix {
    /// ⏲️ `struct timeval` of the 64-bit Unix ABIs.
    #[repr(C)]
    #[derive(Default)]
    pub struct Timeval {
        pub sec: i64,
        #[cfg(target_os = "macos")]
        pub usec: i32,
        #[cfg(target_os = "macos")]
        pub pad: i32,
        #[cfg(not(target_os = "macos"))]
        pub usec: i64,
    }

    /// 📊️ `struct rusage` of the 64-bit Unix ABIs.
    #[repr(C)]
    #[derive(Default)]
    pub struct Rusage {
        pub utime: Timeval,
        pub stime: Timeval,
        pub maxrss: i64,
        pub rest: [i64; 13],
    }

    /// 🍎️ `struct rusage_info_v2` (`<libproc.h>`), read for live resident bytes on macOS.
    #[cfg(target_os = "macos")]
    #[repr(C)]
    pub struct RusageInfoV2 {
        pub uuid: [u8; 16],
        pub head: [u64; 6],
        pub resident_size: u64,
        pub tail: [u64; 11],
    }

    unsafe extern "C" {
        pub fn wait4(pid: i32, status: *mut i32, options: i32, rusage: *mut Rusage) -> i32;
        #[cfg(target_os = "macos")]
        pub fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut RusageInfoV2) -> i32;
        #[cfg(not(target_os = "macos"))]
        pub fn sysconf(name: i32) -> i64;
    }
}

#[cfg(windows)]
mod windows {
    /// 🪟️ `PROCESS_MEMORY_COUNTERS` (`<psapi.h>`).
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryCounters {
        pub cb: u32,
        pub page_fault_count: u32,
        pub peak_working_set_size: usize,
        pub working_set_size: usize,
        pub rest: [usize; 6],
    }

    /// 🕰️ `FILETIME`, 100 ns ticks.
    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct FileTime {
        pub low: u32,
        pub high: u32,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn K32GetProcessMemoryInfo(process: isize, counters: *mut MemoryCounters, cb: u32) -> i32;
        pub fn GetProcessTimes(process: isize, creation: *mut FileTime, exit: *mut FileTime, kernel: *mut FileTime, user: *mut FileTime) -> i32;
    }
}

/// 🧮️ A copyable handle that reads a running child's live resident bytes from another thread.
#[derive(Clone, Copy)]
struct ProcessProbe {
    #[cfg(unix)]
    pid: u32,
    #[cfg(windows)]
    handle: isize,
}

impl ProcessProbe {
    /// 🪝️ Captures the probe of a spawned child.
    fn of(child: &std::process::Child) -> ProcessProbe {
        #[cfg(unix)]
        {
            ProcessProbe { pid: child.id() }
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            ProcessProbe { handle: child.as_raw_handle() as isize }
        }
    }

    /// 🧮️ The live resident bytes, `None` when the platform refuses the query.
    fn resident_bytes(&self) -> Option<u64> {
        #[cfg(target_os = "macos")]
        {
            let mut info = std::mem::MaybeUninit::<unix::RusageInfoV2>::zeroed();
            (unsafe { unix::proc_pid_rusage(self.pid as i32, 2, info.as_mut_ptr()) } == 0).then(|| unsafe { info.assume_init() }.resident_size)
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let statm = std::fs::read_to_string(format!("/proc/{}/statm", self.pid)).ok()?;
            let pages: u64 = statm.split_whitespace().nth(1)?.parse().ok()?;
            Some(pages * unsafe { unix::sysconf(30) }.max(1) as u64)
        }
        #[cfg(windows)]
        {
            let mut counters = windows::MemoryCounters { cb: std::mem::size_of::<windows::MemoryCounters>() as u32, ..Default::default() };
            (unsafe { windows::K32GetProcessMemoryInfo(self.handle, &mut counters, counters.cb) } != 0).then_some(counters.working_set_size as u64)
        }
    }
}

/// ⚰️ Reaps a child and reads its exit status, peak resident bytes and CPU time.
fn reap(child: &mut std::process::Child) -> Usage {
    #[cfg(unix)]
    {
        let (mut status, mut usage) = (0i32, unix::Rusage::default());
        while unsafe { unix::wait4(child.id() as i32, &mut status, 0, &mut usage) } < 0 {
            if std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted { return Usage { exit: 1, peak_rss: 0, user_ms: 0, sys_ms: 0 }; }
        }
        let signal = status & 0x7f;
        let exit = if signal == 0 { (status >> 8) & 0xff } else { 128 + signal };
        let scale = if cfg!(target_os = "macos") { 1 } else { 1024 };
        let millis = |time: &unix::Timeval| (time.sec as u64) * 1000 + (time.usec as u64) / 1000;
        Usage { exit, peak_rss: usage.maxrss.max(0) as u64 * scale, user_ms: millis(&usage.utime), sys_ms: millis(&usage.stime) }
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        let exit = child.wait().ok().and_then(|status| status.code()).unwrap_or(1);
        let handle = child.as_raw_handle() as isize;
        let mut counters = windows::MemoryCounters { cb: std::mem::size_of::<windows::MemoryCounters>() as u32, ..Default::default() };
        unsafe { windows::K32GetProcessMemoryInfo(handle, &mut counters, counters.cb) };
        let (mut creation, mut ended, mut kernel, mut user) = Default::default();
        unsafe { windows::GetProcessTimes(handle, &mut creation, &mut ended, &mut kernel, &mut user) };
        let millis = |time: windows::FileTime| (((time.high as u64) << 32) | time.low as u64) / 10_000;
        Usage { exit, peak_rss: counters.peak_working_set_size as u64, user_ms: millis(user), sys_ms: millis(kernel) }
    }
}

/// 🔗️ A line-oriented connection to the driver's broker: a Unix socket, or a named pipe on Windows.
struct Broker {
    writer: Mutex<Box<dyn Write + Send>>,
    reader: Mutex<Box<dyn BufRead + Send>>,
}

impl Broker {
    /// 🔌️ Connects to the broker endpoint.
    fn connect(endpoint: &str) -> std::io::Result<Broker> {
        #[cfg(unix)]
        let stream = std::os::unix::net::UnixStream::connect(endpoint)?;
        #[cfg(windows)]
        let stream = std::fs::OpenOptions::new().read(true).write(true).open(endpoint)?;
        let reader = stream.try_clone()?;
        Ok(Broker { writer: Mutex::new(Box::new(stream)), reader: Mutex::new(Box::new(BufReader::new(reader))) })
    }

    /// 📤️ Sends one protocol line.
    fn send(&self, line: &str) -> std::io::Result<()> {
        let mut writer = self.writer.lock().map_err(|_| std::io::Error::other("broker writer poisoned"))?;
        writer.write_all(line.as_bytes())?;
        writer.flush()
    }

    /// 📥️ Waits for the next verdict; `true` once admitted.
    fn verdict(&self) -> std::io::Result<bool> {
        let mut line = String::new();
        let mut reader = self.reader.lock().map_err(|_| std::io::Error::other("broker reader poisoned"))?;
        if reader.read_line(&mut line)? == 0 { return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "broker closed")); }
        match line.trim_end() {
            "{\"v\":1,\"op\":\"verdict\",\"admitted\":true}" => Ok(true),
            "{\"v\":1,\"op\":\"verdict\",\"admitted\":false}" => Ok(false),
            other => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("unexpected broker line: {other}"))),
        }
    }
}

/// 🏁️ Converts a rustc exit status into this process's exit code.
fn exit_code(code: i32) -> ExitCode {
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}

/// 🚪️ Runs rustc unchanged, replacing this process where the platform allows it.
fn passthrough(rustc: &OsString, args: &[OsString]) -> ExitCode {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let error = Command::new(rustc).args(args).exec();
        eprintln!("semio-build-scheduler: cannot run {}: {error}", rustc.to_string_lossy());
        ExitCode::from(127)
    }
    #[cfg(windows)]
    {
        match Command::new(rustc).args(args).status() {
            Ok(status) => exit_code(status.code().unwrap_or(1)),
            Err(error) => { eprintln!("semio-build-scheduler: cannot run {}: {error}", rustc.to_string_lossy()); ExitCode::from(127) }
        }
    }
}

/// ⏳️ Waits for the broker's admission while KEEPING the jobserver token Cargo gave this job. The token is never written back: Cargo's helper
/// thread reads every freed token to start or park for another unit, so a unit that handed its token back could not reacquire it while the
/// units that took it wait for admission in turn (no rustc running, fifo empty). Holding it cannot starve admission: a broker with nothing
/// running always admits its first waiter.
fn await_admission(mut verdict: impl FnMut() -> Result<bool, String>) -> Result<(), String> {
    while !verdict()? {}
    Ok(())
}

/// 🎟️ Admits, runs and reports one rustc unit through the broker.
fn scheduled(rustc: &OsString, args: &[OsString], unit: &Unit, endpoint: &str, pass: &str) -> Result<ExitCode, String> {
    let broker = Arc::new(Broker::connect(endpoint).map_err(|error| format!("build broker unreachable at {endpoint}: {error}"))?);
    broker.send(&admit_request(unit, pass, std::process::id())).map_err(|error| format!("broker admission request failed: {error}"))?;
    await_admission(|| broker.verdict().map_err(|error| format!("broker verdict failed: {error}")))?;
    let start_ms = now_ms();
    let mut child = Command::new(rustc).args(args).stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::piped()).spawn().map_err(|error| format!("cannot run {}: {error}", rustc.to_string_lossy()))?;
    let stderr = child.stderr.take().ok_or("rustc stderr unavailable")?;
    let rmeta_ms = Arc::new(Mutex::new(None::<u64>));
    let forwarder = {
        let rmeta_ms = Arc::clone(&rmeta_ms);
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut line = Vec::new();
            let mut out = std::io::stderr();
            while reader.read_until(b'\n', &mut line).map(|count| count > 0).unwrap_or(false) {
                if line.starts_with(b"{\"$message_type\":\"artifact\"") && line.windows(17).any(|window| window == b"\"emit\":\"metadata\"") {
                    if let Ok(mut slot) = rmeta_ms.lock() { slot.get_or_insert_with(now_ms); }
                }
                let _ = out.write_all(&line);
                line.clear();
            }
        })
    };
    let finished = Arc::new(AtomicBool::new(false));
    let sampler = {
        let (broker, finished, probe) = (Arc::clone(&broker), Arc::clone(&finished), ProcessProbe::of(&child));
        std::thread::spawn(move || {
            while !finished.load(Ordering::Acquire) {
                if let Some(bytes) = probe.resident_bytes() { let _ = broker.send(&format!("{{\"v\":1,\"op\":\"rss\",\"bytes\":{bytes}}}\n")); }
                for _ in 0..10 {
                    if finished.load(Ordering::Acquire) { break; }
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        })
    };
    let usage = reap(&mut child);
    finished.store(true, Ordering::Release);
    let _ = sampler.join();
    let _ = forwarder.join();
    let end_ms = now_ms();
    let rmeta = rmeta_ms.lock().ok().and_then(|slot| *slot).map(|at| at.to_string()).unwrap_or_else(|| "null".to_string());
    broker.send(&format!("{{\"v\":1,\"op\":\"done\",\"exit\":{},\"peakRssBytes\":{},\"userMs\":{},\"sysMs\":{},\"startMs\":{},\"endMs\":{},\"rmetaMs\":{}}}\n", usage.exit, usage.peak_rss, usage.user_ms, usage.sys_ms, start_ms, end_ms, rmeta)).map_err(|error| format!("broker report failed: {error}"))?;
    Ok(exit_code(usage.exit))
}

/// 🚀️ Entry: `semio-build-scheduler <rustc> <args…>`.
fn main() -> ExitCode {
    let mut arguments = env::args_os().skip(1);
    let Some(rustc) = arguments.next() else {
        eprintln!("semio-build-scheduler: expected <rustc> <arguments…>");
        return ExitCode::from(2);
    };
    let args: Vec<OsString> = arguments.collect();
    let text: Option<Vec<String>> = args.iter().map(|arg| arg.to_str().map(str::to_string)).collect();
    let unit = text.and_then(|text| parse_unit(&text, &|name| env::var(name).ok())).map(|unit| Unit { source: absolute(&unit.source), out_dir: absolute(&unit.out_dir), externs: unit.externs.iter().map(|path| absolute(path)).collect(), ..unit });
    match (unit, env::var("SEMIO_BUILD_BROKER").ok(), env::var("SEMIO_BUILD_PASS").ok()) {
        (Some(unit), Some(endpoint), Some(pass)) => scheduled(&rustc, &args, &unit, &endpoint, &pass).unwrap_or_else(|error| {
            eprintln!("semio-build-scheduler: {error}");
            ExitCode::from(1)
        }),
        _ => passthrough(&rustc, &args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The unit of a pipelined library compile, its externs and build-script output directory.
    #[test]
    fn reads_library_unit() {
        let args: Vec<String> = ["--crate-name", "beta", "--edition=2021", "src/lib.rs", "--error-format=json", "--json=artifacts", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=s", "-C", "extra-filename=-0a1b", "--out-dir", "/b/deps", "--target", "wasm32-wasip2", "--extern", "alpha=/b/deps/libalpha-9f.rmeta", "--extern", "proc_macro"].iter().map(|value| value.to_string()).collect();
        let env = |name: &str| match name { "CARGO_PKG_NAME" => Some("beta".to_string()), "CARGO_PKG_VERSION" => Some("0.1.0".to_string()), "OUT_DIR" => Some("/b/build/beta-1/out".to_string()), _ => None };
        let unit = parse_unit(&args, &env).expect("unit");
        assert_eq!(unit.crate_name, "beta");
        assert_eq!(unit.extra_filename, "-0a1b");
        assert_eq!(unit.target, "wasm32-wasip2");
        assert_eq!(unit.source, "src/lib.rs");
        assert_eq!(unit.externs, vec!["/b/deps/libalpha-9f.rmeta".to_string()]);
        assert_eq!(unit.build_script_out_dir.as_deref(), Some("/b/build/beta-1/out"));
        assert!(!unit.primary);
    }

    /// 🧪️ Version and print probes are never admitted.
    #[test]
    fn refuses_probes() {
        let env = |_: &str| None;
        for probe in [vec!["-vV"], vec!["-", "--crate-name", "___", "--print=file-names"]] {
            assert_eq!(parse_unit(&probe.iter().map(|value| value.to_string()).collect::<Vec<_>>(), &env), None);
        }
    }

    /// 🧪️ Admission waits through any number of refusals without asking for or giving back a token, and reports a broker fault.
    #[test]
    fn waits_through_refusals() {
        for refusals in [0usize, 1, 7] {
            let mut calls = 0;
            let result = await_admission(|| { calls += 1; Ok(calls > refusals) });
            assert_eq!(result, Ok(()));
            assert_eq!(calls, refusals + 1);
        }
        assert_eq!(await_admission(|| Err("broker closed".to_string())), Err("broker closed".to_string()));
    }

    /// 🧪️ Strings escape exactly the JSON control set.
    #[test]
    fn escapes_json() {
        assert_eq!(json_string("a\"b\\c\n🎟️"), "\"a\\\"b\\\\c\\n🎟️\"");
    }
}
