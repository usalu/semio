//! 🌐️ Opens verified ready addresses through explicit system argument vectors and cancellable owned helpers.

use std::io;
use std::process::{Command, Stdio};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc};
use std::time::{Duration, Instant};

const DEADLINE: Duration = Duration::from_secs(5);

/// 🖥️ The operating system that owns the default URL handler.
#[derive(Clone, Copy)]
pub(crate) enum Platform { Windows, Macos, Linux }

/// 🔎️ Admits only an exact successful HTTP readiness address.
pub fn ready_url(status: dashboard_wire::ipc::SessionStatus, url: Option<&str>) -> Option<&str> {
    url.filter(|url| status == dashboard_wire::ipc::SessionStatus::Running && valid_word(url) && url.strip_prefix("http://").or_else(|| url.strip_prefix("https://")).is_some_and(|address| !address.is_empty()))
}

fn valid_word(word: &str) -> bool { !word.is_empty() && !word.contains(['\0', '\r', '\n']) }

/// 🧭️ Resolves an optional authored JSON opener vector and appends the URL as one literal word.
pub(crate) fn plan(url: &str, configured: Option<&str>, platform: Platform) -> io::Result<Vec<String>> {
    if ready_url(dashboard_wire::ipc::SessionStatus::Running, Some(url)).is_none() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid ready address")); }
    let mut words: Vec<String> = match configured {
        Some(text) => serde_json::from_str(text).map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?,
        None => match platform { Platform::Windows => vec!["rundll32.exe".into(), "url.dll,FileProtocolHandler".into()], Platform::Macos => vec!["open".into()], Platform::Linux => vec!["xdg-open".into()] },
    };
    if words.is_empty() || words.iter().any(|word| !valid_word(word)) { return Err(io::Error::new(io::ErrorKind::InvalidInput, "URL opener requires a nonempty argument vector")); }
    words.push(url.into());
    Ok(words)
}

fn execute(words: &[String], cancelled: &AtomicBool) -> io::Result<()> {
    if cancelled.load(Ordering::Acquire) { return Err(io::Error::new(io::ErrorKind::Interrupted, "opening cancelled")); }
    let mut command = Command::new(&words[0]);
    command.args(&words[1..]).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    for name in ["SEMIO_SCRIPT_PROCESS_INVOCATION", "SEMIO_SCRIPT_CAPABILITIES", "SEMIO_SCRIPT_POLICY", "SEMIO_PROCESS_OWNER_CONTEXT"] { command.env_remove(name); }
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; command.creation_flags(0x08000000); }
    let mut child = command.spawn()?;
    let started = Instant::now();
    loop {
        let refusal = if cancelled.load(Ordering::Acquire) { Some(io::Error::new(io::ErrorKind::Interrupted, "opening cancelled")) } else if started.elapsed() >= DEADLINE { Some(io::Error::new(io::ErrorKind::TimedOut, "URL opener exceeded its deadline")) } else { None };
        if let Some(error) = refusal { let _ = child.kill(); let _ = child.wait(); return Err(error); }
        match child.try_wait() {
            Ok(Some(status)) => return if status.success() { Ok(()) } else { Err(io::Error::other(format!("URL opener exited with {status}"))) },
            Ok(None) => std::thread::sleep(Duration::from_millis(10)),
            Err(error) => { let _ = child.kill(); let _ = child.wait(); return Err(error); }
        }
    }
}

fn selected_plan(url: &str) -> io::Result<Vec<String>> {
    let configured = std::env::var("SEMIO_DASHBOARD_URL_OPENER").ok();
    plan(url, configured.as_deref(), if cfg!(windows) { Platform::Windows } else if cfg!(target_os = "macos") { Platform::Macos } else { Platform::Linux })
}

/// 🌐️ Runs the same verified opener for a noninteractive caller.
pub(crate) fn open(url: &str) -> io::Result<()> { execute(&selected_plan(url)?, &AtomicBool::new(false)) }

/// ⏳️ One bounded asynchronous opener whose owned helper is cancelled when the view discards it.
pub struct Job { cancelled: Arc<AtomicBool>, pub receiver: mpsc::Receiver<io::Result<()>> }

impl Job {
    pub fn start(url: &str, waker: Option<semio_framework_ui_tui_terminal::backend::Waker>) -> io::Result<Self> {
        let words = selected_plan(url)?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let signal = cancelled.clone();
        let (sender, receiver) = mpsc::channel();
        std::thread::Builder::new().name("dashboard-open-ready".into()).spawn(move || { let result = execute(&words, &signal); let _ = sender.send(result); if let Some(waker) = waker { waker.wake(); } })?;
        Ok(Self { cancelled, receiver })
    }
}

impl Drop for Job { fn drop(&mut self) { self.cancelled.store(true, Ordering::Release); } }

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
