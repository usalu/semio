//! 🚦️ The lifecycle of the workspace daemon: serving a workspace, starting the daemon detached, asking its status, stopping it, and a
//! connection that starts it first when none runs.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs

use super::client::{Connection, Message};
use super::control;
use super::ipc::{self, ClientMsg, ServerMsg};
use super::lease::{write_pid, WorkspaceLease};
use super::supervisor::Supervisor;
use super::transport::Listener;
use dashboard_labels::DashboardLabels;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// 🔗 A connection to the workspace daemon, which is started first when none runs.
pub fn connect_or_start(root: &Path, executable: &Path) -> std::io::Result<Connection> {
    if let Ok(connection) = Connection::connect(root) { return Ok(connection); }
    spawn_daemon(root, executable)?;
    Connection::connect(root)
}

/// 🌀 Serves the workspace until `running` is cleared or a view asks the daemon to shut down.
pub fn serve(root: &Path, running: &AtomicBool) -> std::io::Result<()> {
    let _lease = WorkspaceLease::acquire(root)?;
    write_pid(root)?;
    let listener = Listener::bind(root)?;
    let mut supervisor = Supervisor::new(root, Some(listener))?;
    let interrupts = ui_tui::tui::pty::interrupts();
    while running.load(Ordering::SeqCst) && !supervisor.is_shutdown() {
        if interrupts.load(Ordering::SeqCst) > 0 { supervisor.shut_down(); }
        supervisor.turn(Duration::from_millis(250))?;
    }
    supervisor.drain(Duration::from_secs(1));
    Ok(())
}

/// 🚀 Starts a detached daemon process (`semio daemon serve --root …`) and waits until it answers; the answer is its pid.
pub fn spawn_daemon(root: &Path, executable: &Path) -> std::io::Result<u32> {
    let spawned = ui_tui::tui::pty::spawn_detached(&executable.to_string_lossy(), &["daemon", "serve", "--root", &root.display().to_string()], &["SEMIO_BUILD_INVOCATION_PID"], root).map_err(|error| std::io::Error::other(error.message))?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(connection) = Connection::connect(root) { return Ok(connection.daemon_pid()); }
        std::thread::sleep(Duration::from_millis(20));
    }
    Err(std::io::Error::other(format!("daemon {spawned} did not become ready")))
}

/// 🌅️ Starts the workspace daemon unless one runs, and reports it.
pub fn start_detached(root: &Path, executable: &Path, text: &DashboardLabels) -> i32 {
    if Connection::connect(root).is_ok() {
        print!("{}", status(root, text));
        return 0;
    }
    match spawn_daemon(root, executable) {
        Ok(pid) => { println!("{}", text.cli_daemon_ready.fill(&[("pid", &pid.to_string())]).into_string()); 0 }
        Err(error) => { eprintln!("{}", text.cli_daemon_start_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }
    }
}

/// 📊 One line about the daemon of a workspace, and a second one when its build differs from this one.
pub fn status(root: &Path, text: &DashboardLabels) -> String {
    let Ok(mut connection) = Connection::connect(root) else { return format!("{}\n", text.cli_daemon_not_running.as_str()) };
    let count = control::tasks(&mut connection, Duration::from_secs(2)).map_or(0, |sessions| sessions.iter().filter(|session| session.status.live()).count());
    let mut report = format!("{}\n", text.cli_daemon_status.fill(&[("pid", &connection.daemon_pid().to_string()), ("build", connection.daemon_build()), ("count", &count.to_string()), ("endpoint", &ipc::endpoint(root))]).into_string());
    if let Some(skew) = connection.skew() { report.push_str(&format!("{}\n", text.cli_warning.fill(&[("message", &text.skew(&skew))]).into_string())); }
    report
}

/// 🌇️ Asks the daemon to end its tasks and itself, and waits for it to say so.
pub fn stop(root: &Path, text: &DashboardLabels) -> i32 {
    let outcome = Connection::connect(root).and_then(|mut connection| {
        connection.send(&ClientMsg::Shutdown {})?;
        let deadline = Instant::now() + Duration::from_secs(10);
        while Instant::now() < deadline {
            let messages = match connection.receive(Duration::from_millis(100)) { Ok(messages) => messages, Err(_) => return Ok(()) };
            if messages.iter().any(|message| matches!(message, Message::Control(ServerMsg::Shutdown {}) | Message::Disconnected(_))) { return Ok(()); }
        }
        Err(std::io::Error::other(text.cli_daemon_stop_timed_out.as_str()))
    });
    match outcome {
        Ok(()) => { println!("{}", text.cli_daemon_stopped.as_str()); 0 }
        Err(error) => { eprintln!("{}", text.cli_daemon_stop_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }
    }
}
