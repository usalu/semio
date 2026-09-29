//! ⌨️ The `proctor` command line: `serve` (the §9 environment), `check <catalog>` (validate a
//! catalog and its quizzes, non-zero exit on issues) and `rebuild` (drop and refold every read
//! model from the event log with progress; Ctrl+C stops between batches and the next start
//! resumes from the last folded batch).
//!
//! Log lines go to stderr with an `[INFO]`/`[ERROR]` prefix; `check` reports to stdout.
//!
//! @see ../🎚️config/🦀️.rs — the environment
//! @see ../../README.md — the operator guide

use std::net::SocketAddr;
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use semio_framework_async::CancelToken;
use server::storage::StorageProfile;

use crate::catalog::{load_catalog, CatalogError, LoadedCatalog};
use crate::config::ProctorConfig;
use crate::instance::Proctor;
use crate::projections::{CatchUp, Progress};
use crate::site::SiteHost;

/// 📖️ The one-line usage.
pub const USAGE: &str = "usage: proctor serve | proctor check <catalog 🔣️.json> | proctor rebuild";

/// 🛑️ The exit code of a run stopped by Ctrl+C.
pub const INTERRUPTED: u8 = 130;

/// ▶️ Run one command line (without the program name) against an environment reader.
pub async fn run(arguments: &[String], environment: impl Fn(&str) -> Option<String>) -> ExitCode {
    let outcome = match arguments.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["serve"] => serve(&environment).await,
        ["check", catalog] => return check(Path::new(catalog)),
        ["rebuild"] => rebuild(&environment).await,
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    outcome.unwrap_or_else(|error| {
        eprintln!("[ERROR] {error}");
        ExitCode::FAILURE
    })
}

/// ✅️ Validate a catalog and every quiz it lists.
pub fn check(path: &Path) -> ExitCode {
    match load_catalog(path) {
        Ok(loaded) => {
            println!("catalog {} ({}): {} quizzes, {} badges, fingerprint {}", loaded.id(), path.display(), loaded.entries.len(), loaded.catalog.badges.len(), loaded.fingerprint);
            for entry in &loaded.entries {
                println!("  quiz {} ({} tasks) {} revision {}", entry.quiz.id, entry.quiz.tasks.len(), entry.entry, entry.revision);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            report(&error);
            ExitCode::FAILURE
        }
    }
}

fn report(error: &CatalogError) {
    match error {
        CatalogError::Invalid { issues } => {
            for issue in issues {
                match &issue.quiz {
                    Some(quiz) => println!("[ERROR] {} {} ({quiz})", issue.path, issue.code),
                    None => println!("[ERROR] {} {}", issue.path, issue.code),
                }
            }
            println!("[ERROR] {} issue(s); the catalog is refused", issues.len());
        }
        other => println!("[ERROR] {other}"),
    }
}

async fn serve(environment: &impl Fn(&str) -> Option<String>) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let config = ProctorConfig::from_environment(environment)?;
    eprintln!(
        "[INFO] proctor {} mode {}, data {}, catalog {}, site {}, cross-origin {}, trusted forwarding {}",
        env!("CARGO_PKG_VERSION"),
        config.mode.label(),
        config.data.display(),
        config.catalog.display(),
        config.site.as_ref().map_or_else(|| "none".to_string(), |site| site.display().to_string()),
        config.origins.label(),
        config.forwarding.label()
    );
    let Some(catalog) = loaded(&config) else { return Ok(ExitCode::FAILURE) };
    let site = config.site.as_deref().map(SiteHost::open).transpose()?;
    let interrupt = interrupt();
    let proctor = Proctor::assemble(profile(&config), Arc::new(catalog), config.gate(), site).await?;
    if proctor.prepare().await? {
        eprintln!("[INFO] projections were built against another catalog; rebuilding them");
    }
    if let Some(code) = settled(&proctor, &interrupt, "catch-up").await? {
        return Ok(code);
    }
    let bound = proctor.bind(SocketAddr::new(config.bind, config.port)).await?;
    eprintln!("[INFO] proctor listening on http://{}", bound.local_addr());
    bound.serve(interrupt).await?;
    eprintln!("[INFO] proctor stopped");
    Ok(ExitCode::SUCCESS)
}

async fn rebuild(environment: &impl Fn(&str) -> Option<String>) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let config = ProctorConfig::from_environment(environment)?;
    let Some(catalog) = loaded(&config) else { return Ok(ExitCode::FAILURE) };
    let interrupt = interrupt();
    let proctor = Proctor::assemble(profile(&config), Arc::new(catalog), config.gate(), None).await?;
    proctor.reset_projections().await?;
    eprintln!("[INFO] projections dropped; refolding every committed event");
    Ok(settled(&proctor, &interrupt, "rebuild").await?.unwrap_or(ExitCode::SUCCESS))
}

fn profile(config: &ProctorConfig) -> StorageProfile {
    StorageProfile::Embedded { data_dir: config.data.to_string_lossy().into_owned() }
}

fn loaded(config: &ProctorConfig) -> Option<LoadedCatalog> {
    match load_catalog(&config.catalog) {
        Ok(loaded) => {
            eprintln!("[INFO] catalog {}: {} quizzes, {} badges", loaded.id(), loaded.entries.len(), loaded.catalog.badges.len());
            Some(loaded)
        }
        Err(error) => {
            report(&error);
            None
        }
    }
}

async fn settled(proctor: &Proctor, interrupt: &CancelToken, label: &str) -> Result<Option<ExitCode>, Box<dyn std::error::Error>> {
    let relayed = proctor.reconcile().await?;
    if relayed > 0 {
        eprintln!("[INFO] {relayed} enrollment(s) relayed to their learners");
    }
    let mut reported = 0;
    match proctor.settle(interrupt, |progress| announce(label, progress, &mut reported)).await? {
        CatchUp::Current(at) => {
            eprintln!("[INFO] {label}: projections current at event {} ({} folded)", at.position, at.folded);
            Ok(None)
        }
        CatchUp::Cancelled(at) => {
            eprintln!("[INFO] {label} interrupted at event {} of {}; the next start resumes from there", at.position, at.head);
            Ok(Some(ExitCode::from(INTERRUPTED)))
        }
    }
}

/// 📶️ Report progress in steps of ten percent.
fn announce(label: &str, progress: Progress, reported: &mut u64) {
    let percent = progress.position.saturating_mul(100).checked_div(progress.head).unwrap_or(100);
    if percent >= *reported + 10 || progress.position == progress.head {
        *reported = percent;
        eprintln!("[INFO] {label}: event {} of {} ({percent}%)", progress.position, progress.head);
    }
}

/// 🛑️ A token cancelled by the first interrupt or termination request.
fn interrupt() -> CancelToken {
    let token = CancelToken::root_now();
    let fired = token.clone();
    tokio::spawn(async move {
        match termination().await {
            Ok(signal) => {
                eprintln!("[INFO] {signal} received; stopping");
                fired.cancel_now();
            }
            Err(error) => eprintln!("[ERROR] cannot listen for termination signals: {error}"),
        }
    });
    token
}

/// 🐧️ Ctrl+C or `SIGTERM` (what `docker stop` sends through tini).
#[cfg(unix)]
async fn termination() -> std::io::Result<&'static str> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::select! {
        result = tokio::signal::ctrl_c() => result.map(|()| "interrupt"),
        _ = terminate.recv() => Ok("termination"),
    }
}

/// 🪟️ Ctrl+C, Ctrl+Break (the signal a process group started without Ctrl+C receives), console close
/// or system shutdown.
#[cfg(windows)]
async fn termination() -> std::io::Result<&'static str> {
    let (mut interruption, mut close, mut shutdown) = (tokio::signal::windows::ctrl_break()?, tokio::signal::windows::ctrl_close()?, tokio::signal::windows::ctrl_shutdown()?);
    tokio::select! {
        result = tokio::signal::ctrl_c() => result.map(|()| "interrupt"),
        _ = interruption.recv() => Ok("break"),
        _ = close.recv() => Ok("console close"),
        _ = shutdown.recv() => Ok("shutdown"),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
