//! ⌨️ The `proctor` command line.
//!
//! | Verb | What it does | Needs |
//! |---|---|---|
//! | `serve` | serves the API (the §9 environment) | everything |
//! | `check <catalog>` | validates a catalog and its quizzes, non-zero exit on issues | nothing |
//! | `rebuild` | drops and refolds every read model from the event log, with progress; Ctrl+C stops between batches and the next start resumes | a stopped proctor |
//! | `health` | exits 0 exactly when the listener of this environment answers `GET /instance` with `200`, asked as the TLS-terminating proxy asks | `PROCTOR_BIND`, `PROCTOR_PORT` |
//! | `backup <directory/\|file\|->` | writes a whole, consistent, verified copy of the database while a proctor serves it: into a directory (an existing one, or a path ending in `/`) as `proctor-<UTC time>.sqlite`, to a file that does not exist yet, or to stdout; prints the path; never overwrites; progress on stderr, Ctrl+C removes the partial copy | `PROCTOR_DATA` |
//! | `restore <file\|->` | replaces the database of a stopped proctor with such a copy (a file, or stdin) after verifying it | `PROCTOR_DATA`, a stopped proctor |
//! | `erase (--handle <handle>\|--tag <tag>\|--learner <id>) [--dry-run]` | removes one learner on request: its stream, the stream of every handle it holds, their receipts, outbox rows and snapshots, and every read model; `--dry-run` prints what would go and changes nothing | `PROCTOR_DATA`, a stopped proctor |
//! | `prune --older-than <age> [--dry-run]` | removes the registrations nobody played under: every learner that never submitted a run and registered longer ago than the age (`90m`, `36h`, `7d`, `2w`) with the handles it holds, and every handle claimed that long ago beside the identity of its learner — by the rules of `erase`, in batches, with progress; Ctrl+C stops between batches; `--dry-run` counts and changes nothing | `PROCTOR_DATA`, a stopped proctor |
//!
//! `-` is stdout or stdin, which is how a container without a shell hands a backup to its host
//! and takes one back. Log lines go to stderr with an `[INFO]`/`[ERROR]` prefix; `check`, the path
//! of a backup and the reports of `erase` and `prune` go to stdout.
//!
//! @see ../🎚️config/🦀️.rs — the environment
//! @see ../🗄️storage/🦀️.rs — `Database::snapshot`, `Database::adopt`, `Database::erase`, `Database::remove`
//! @see ../../README.md — the operator guide

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use quiz::{Event, Identity};
use semio_framework_async::CancelToken;
use server::contract::{ActorKey, IdempotencyKey};
use server::storage::{AuthorityStore, StorageProfile};

use crate::actors::{enrollment_receipt, handle_key, learner_key, HANDLE, LEARNER, RUN_SUBMITTED};
use crate::catalog::{load_catalog, CatalogError, LoadedCatalog};
use crate::config::ProctorConfig;
use crate::instance::Proctor;
use crate::projections::{CatchUp, Progress};
use crate::storage::{Database, Erasure, Removed, SqliteAuthorityStore, DATABASE_FILE};

/// 📖️ The one-line usage.
pub const USAGE: &str = "usage: proctor serve | proctor check <catalog 🔣️.json> | proctor rebuild | proctor health | proctor backup <directory/|file|-> | proctor restore <file|-> | proctor erase (--handle <handle>|--tag <tag>|--learner <id>) [--dry-run] | proctor prune --older-than <age: 90m|36h|7d|2w> [--dry-run]";

/// 🧺️ The learners one transaction of a `prune` removes: what an interrupt waits for at most.
pub const PRUNE_BATCH: usize = 512;

/// 🛑️ The exit code of a run stopped by Ctrl+C.
pub const INTERRUPTED: u8 = 130;

/// ⏳️ How long `health` waits for the listener to accept and to answer.
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// ▶️ Run one command line (without the program name) against an environment reader.
pub async fn run(arguments: &[String], environment: impl Fn(&str) -> Option<String>) -> ExitCode {
    let outcome = match arguments.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["serve"] => serve(&environment).await,
        ["check", catalog] => return check(Path::new(catalog)),
        ["rebuild"] => rebuild(&environment).await,
        ["health"] => health(&environment),
        ["backup", target] => backup(&environment, target).await,
        ["restore", source] => restore(&environment, source),
        ["erase", selection @ ..] => match Selector::parse(selection) {
            Some((selector, dry_run)) => erase(&environment, &selector, dry_run).await,
            None => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        },
        ["prune", request @ ..] => match Pruning::parse(request) {
            Some(pruning) => prune(&environment, &pruning).await,
            None => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        },
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
        "[INFO] proctor {} mode {}, data {}, catalog {}, cross-origin {}, trusted forwarding {}, presence tick {} ms, at most {} learners and {} started runs per learner, sign-ups per address {} at once and {} per hour",
        env!("CARGO_PKG_VERSION"),
        config.mode.label(),
        config.data.display(),
        config.catalog.display(),
        config.origins.describe(),
        config.forwarding.label(),
        config.presence_tick.as_millis(),
        config.caps.learners,
        config.caps.runs,
        config.sign_ups().burst,
        config.sign_ups().tokens
    );
    let Some(catalog) = loaded(&config) else { return Ok(ExitCode::FAILURE) };
    let interrupt = interrupt();
    let proctor = Proctor::assemble(profile(&config), Arc::new(catalog), config.gate(), config.presence()).await?.capped(config.caps);
    if proctor.prepare().await? {
        eprintln!("[INFO] projections were built for another catalog or projector revision; rebuilding them");
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
    let proctor = Proctor::assemble(profile(&config), Arc::new(catalog), config.gate(), config.presence()).await?;
    proctor.reset_projections().await?;
    eprintln!("[INFO] projections dropped; refolding every committed event");
    Ok(settled(&proctor, &interrupt, "rebuild").await?.unwrap_or(ExitCode::SUCCESS))
}

/// 🩺️ Ask the listener of this environment for `GET /instance` the way the TLS-terminating proxy
/// does; anything but `200` is not healthy.
fn health(environment: &impl Fn(&str) -> Option<String>) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let address = ProctorConfig::probe_address(environment)?;
    let mut stream = TcpStream::connect_timeout(&address, PROBE_TIMEOUT).map_err(|error| format!("{address} does not accept: {error}"))?;
    stream.set_read_timeout(Some(PROBE_TIMEOUT))?;
    stream.set_write_timeout(Some(PROBE_TIMEOUT))?;
    stream.write_all(format!("GET /instance HTTP/1.1\r\nHost: {address}\r\nX-Forwarded-Proto: https\r\nConnection: close\r\n\r\n").as_bytes())?;
    let mut status = String::new();
    BufReader::new(stream).read_line(&mut status)?;
    if status.split_whitespace().nth(1) == Some("200") {
        return Ok(ExitCode::SUCCESS);
    }
    eprintln!("[ERROR] GET /instance on {address} answered {:?}", status.trim_end());
    Ok(ExitCode::FAILURE)
}

/// 🗓️ A time as `YYYYMMDDTHHMMSSZ` in UTC (the civil date of its days since the epoch).
pub fn timestamp(time: SystemTime) -> String {
    let seconds = time.duration_since(UNIX_EPOCH).map_or(0, |since| since.as_secs());
    let (days, rest) = ((seconds / 86_400) as i64 + 719_468, seconds % 86_400);
    let (era, day_of_era) = (days.div_euclid(146_097), days.rem_euclid(146_097));
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let march_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * march_month + 2) / 5 + 1;
    let month = if march_month < 10 { march_month + 3 } else { march_month - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}{month:02}{day:02}T{:02}{:02}{:02}Z", rest / 3_600, rest % 3_600 / 60, rest % 60)
}

/// 📁️ The file a backup to `target` is written as at `time`: inside a directory — an existing one, or
/// a path ending in a separator, which is created — the timestamped name `proctor-<UTC time>.sqlite`;
/// any other path is the file itself.
pub fn backup_file(target: &str, time: SystemTime) -> std::io::Result<PathBuf> {
    let path = PathBuf::from(target);
    if !path.is_dir() && !target.ends_with(['/', '\\']) {
        return Ok(path);
    }
    std::fs::create_dir_all(&path)?;
    Ok(path.join(format!("proctor-{}.sqlite", timestamp(time))))
}

/// 📸️ Write a whole, verified copy of the database into a directory, to a file or to stdout (through
/// a spool file in the data directory, the one place a read-only container may write), reporting
/// progress; an interrupt removes the partial copy.
async fn backup(environment: &impl Fn(&str) -> Option<String>, target: &str) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let data = ProctorConfig::data_directory(environment)?;
    let piped = target == "-";
    let file = if piped { spool(&data, "backup") } else { backup_file(target, SystemTime::now())? };
    let (source, destination, cancel) = (data.clone(), file.clone(), interrupt());
    let copying = tokio::task::spawn_blocking(move || {
        let mut reported = 0;
        Database::snapshot(&source, &destination, &cancel, |written, size| announce_bytes(written, size, &mut reported))
    });
    let Some(inspection) = copying.await?? else {
        eprintln!("[INFO] backup interrupted; nothing was kept");
        return Ok(ExitCode::from(INTERRUPTED));
    };
    let bytes = match piped {
        true => {
            let streamed = std::fs::File::open(&file).and_then(|mut spooled| std::io::copy(&mut spooled, &mut std::io::stdout().lock()));
            let _ = std::fs::remove_file(&file);
            streamed?
        }
        false => {
            println!("{}", file.display());
            std::fs::metadata(&file)?.len()
        }
    };
    eprintln!("[INFO] backup: {bytes} bytes, {} events up to position {}, verified", inspection.events, inspection.head);
    Ok(ExitCode::SUCCESS)
}

/// 📊️ Report the bytes copied in steps of ten percent.
fn announce_bytes(written: u64, size: u64, reported: &mut u64) {
    let percent = written.saturating_mul(100).checked_div(size).unwrap_or(100).min(100);
    if percent >= *reported + 10 {
        *reported = percent;
        eprintln!("[INFO] backup: {written} of {size} bytes ({percent}%)");
    }
}

/// ♻️ Replace the database of a stopped proctor with the copy at `source` (or on stdin), spooled into
/// the data directory and adopted only when it is a whole proctor database.
fn restore(environment: &impl Fn(&str) -> Option<String>, source: &str) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let data = ProctorConfig::data_directory(environment)?;
    std::fs::create_dir_all(&data)?;
    let staged = spool(&data, "restore");
    let adopted = (|| -> Result<u64, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(&staged)?;
        match source {
            "-" => std::io::copy(&mut std::io::stdin().lock(), &mut file)?,
            path => std::io::copy(&mut std::fs::File::open(path)?, &mut file)?,
        };
        file.sync_all()?;
        drop(file);
        Ok(Database::adopt(&data, &staged)?)
    })();
    if adopted.is_err() {
        let _ = std::fs::remove_file(&staged);
    }
    eprintln!("[INFO] restore: {} holds every event up to position {}; start the proctor", data.join(DATABASE_FILE).display(), adopted?);
    Ok(ExitCode::SUCCESS)
}

/// 🎯️ Whom `erase` removes: the holder of a handle, the learner behind a public tag, or a learner id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    Handle(String),
    Tag(String),
    Learner(String),
}

impl Selector {
    /// 🧩️ The selector and whether it is a dry run, from the arguments after `erase`; `None` for
    /// anything but exactly one selector and at most one `--dry-run`.
    pub fn parse(arguments: &[&str]) -> Option<(Self, bool)> {
        let dry_run = arguments.contains(&"--dry-run");
        let rest: Vec<&str> = arguments.iter().copied().filter(|argument| *argument != "--dry-run").collect();
        if arguments.len() - rest.len() > 1 {
            return None;
        }
        match rest.as_slice() {
            ["--handle", handle] => Some((Self::Handle((*handle).to_string()), dry_run)),
            ["--tag", tag] => Some((Self::Tag((*tag).to_string()), dry_run)),
            ["--learner", learner] => Some((Self::Learner((*learner).to_string()), dry_run)),
            _ => None,
        }
    }
}

/// 🧑‍🎓️ One learner an erasure removes: its catalog, its id, how it registered if it did, and the
/// handle keys it holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subject {
    pub tenant: String,
    pub learner: String,
    pub identity: Option<Identity>,
    pub handles: Vec<String>,
}

impl Subject {
    /// 🗝️ Every actor of the learner: itself and the handles it holds.
    pub fn actors(&self) -> Vec<ActorKey> {
        std::iter::once(learner_key(&self.tenant, &self.learner)).chain(self.handles.iter().map(|key| handle_key(&self.tenant, key))).collect()
    }
}

/// 🔦️ Every learner of the log a selector names, with the handles each holds: read from the streams
/// themselves, never from a read model. A handle is normalized like any other; a registration
/// that cannot be read is an error, because an erasure must not miss a name.
pub async fn subjects(log: &SqliteAuthorityStore, selector: &Selector) -> Result<Vec<Subject>, Box<dyn std::error::Error>> {
    let wanted = match selector {
        Selector::Handle(handle) => Some(quiz::normalize_handle(handle).ok_or("the handle is outside the handle policy, so nobody can hold it")?.key),
        Selector::Tag(tag) if tag.len() == 8 && tag.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) => None,
        Selector::Tag(_) => return Err("a tag is 8 lowercase hex digits".into()),
        Selector::Learner(learner) if quiz::is_id(learner) => None,
        Selector::Learner(_) => return Err("a learner id is 32 lowercase hex digits".into()),
    };
    let mut found = Vec::new();
    for tenant in log.tenants()? {
        let mut holders: Vec<(String, String)> = Vec::new();
        for event in log.stream_events(&tenant, HANDLE)? {
            let unreadable = || format!("event {} of {}/{} ({}) is no registration this proctor can read", event.seq, event.stream.kind, event.stream.id, event.kind);
            let key = quiz::handle_key_of(&event.stream.id).ok_or_else(unreadable)?;
            let Ok(Event::LearnerRegistered { learner, .. }) = serde_json::from_slice::<Event>(&event.payload) else { return Err(unreadable().into()) };
            holders.push((key, learner));
        }
        let streams = log.streams(&tenant, LEARNER)?;
        let mut learners: Vec<&String> = match selector {
            Selector::Handle(_) => holders.iter().filter(|(key, _)| Some(key) == wanted.as_ref()).map(|(_, learner)| learner).collect(),
            Selector::Tag(tag) => streams.iter().chain(holders.iter().map(|(_, learner)| learner)).filter(|learner| quiz::learner_tag(learner) == *tag).collect(),
            Selector::Learner(id) => streams.iter().chain(holders.iter().map(|(_, learner)| learner)).filter(|learner| *learner == id).collect(),
        };
        learners.sort();
        learners.dedup();
        for learner in learners {
            let registration = log.events_since(&learner_key(&tenant, learner), 0).await?.into_iter().find_map(|event| serde_json::from_slice::<Event>(&event.payload).ok());
            let identity = match registration {
                Some(Event::LearnerRegistered { identity, .. }) => Some(identity),
                _ => None,
            };
            found.push(Subject { tenant: tenant.clone(), learner: learner.clone(), identity, handles: holders.iter().filter(|(_, holder)| holder == learner).map(|(key, _)| key.clone()).collect() });
        }
    }
    Ok(found)
}

/// 🧨️ Erase one learner on request from the database of a stopped proctor, or with `dry_run` only
/// report what would go. Refused unless the selector names exactly one learner.
async fn erase(environment: &impl Fn(&str) -> Option<String>, selector: &Selector, dry_run: bool) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let data = ProctorConfig::data_directory(environment)?;
    let database = Database::open_offline(&data)?;
    let found = subjects(&SqliteAuthorityStore::new(database.clone()), selector).await?;
    let [subject] = found.as_slice() else {
        match found.len() {
            0 => eprintln!("[ERROR] erase: nobody in {} matches; nothing was changed", data.join(DATABASE_FILE).display()),
            count => eprintln!("[ERROR] erase: {count} learners match ({}); name one with --handle or --learner; nothing was changed", found.iter().map(described).collect::<Vec<_>>().join(", ")),
        }
        return Ok(ExitCode::FAILURE);
    };
    let actors = subject.actors();
    let erasure = if dry_run { database.erasure(&actors)? } else { database.erase(&actors)? };
    print!("{}", erasure_report(subject, &erasure, dry_run));
    Ok(ExitCode::SUCCESS)
}

/// 🪪️ A learner as the operator recognises it: the public tag and how it registered.
fn described(subject: &Subject) -> String {
    let identity = match &subject.identity {
        Some(Identity::Anonymous) => "anonymous".to_string(),
        Some(Identity::Pseudonym { handle }) => format!("pseudonym {handle:?}"),
        Some(Identity::Name { handle }) => format!("name {handle:?}"),
        None => "never registered".to_string(),
    };
    format!("#{} {identity}", quiz::learner_tag(&subject.learner))
}

/// 📋️ What an erasure removed, or with `dry_run` would remove, line by line.
pub fn erasure_report(subject: &Subject, erasure: &Erasure, dry_run: bool) -> String {
    let verb = if dry_run { "would erase" } else { "erased" };
    let mut lines = vec![format!("{verb} learner {} of catalog {}", described(subject), subject.tenant)];
    for erased in &erasure.actors {
        let stream = match erased.actor.kind.as_str() {
            HANDLE => format!("handle {:?}", quiz::handle_key_of(&erased.actor.id).unwrap_or_default()),
            _ => "learner stream".to_string(),
        };
        lines.push(format!("  {stream}: {} events, {} receipts, {} outbox rows, {} snapshots, {} leases", erased.events, erased.receipts, erased.outbox, erased.snapshots, erased.leases));
    }
    lines.push(format!("  read models: {} rows dropped; the next start rebuilds them from the events that are left", erasure.projections));
    lines.push(if dry_run { "dry run: nothing was changed".to_string() } else { "done: the handles are free again, the learner id is unknown to the proctor, the scores are gone".to_string() });
    lines.join("\n") + "\n"
}

/// ✂️ What `prune` is asked: the age a registration must have, as typed and as a span, and whether
/// only to count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pruning {
    pub older_than: String,
    pub age: Duration,
    pub dry_run: bool,
}

impl Pruning {
    /// 🧷️ The request from the arguments after `prune`; `None` for anything but exactly one
    /// `--older-than <age>` and at most one `--dry-run`.
    pub fn parse(arguments: &[&str]) -> Option<Self> {
        let dry_run = arguments.contains(&"--dry-run");
        let rest: Vec<&str> = arguments.iter().copied().filter(|argument| *argument != "--dry-run").collect();
        if arguments.len() - rest.len() > 1 {
            return None;
        }
        match rest.as_slice() {
            ["--older-than", older_than] => age(older_than).map(|age| Self { older_than: (*older_than).to_string(), age, dry_run }),
            _ => None,
        }
    }
}

/// 🍂️ An age as `prune` takes it: a whole number of seconds (`s`), minutes (`m`), hours (`h`), days
/// (`d`) or weeks (`w`) — `90m`, `36h`, `7d`, `2w`. A number without a unit is no age.
pub fn age(text: &str) -> Option<Duration> {
    let (count, unit) = text.split_at_checked(text.len().checked_sub(1)?)?;
    let seconds: u64 = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3_600,
        "d" => 86_400,
        "w" => 604_800,
        _ => return None,
    };
    if count.is_empty() || !count.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    count.parse::<u64>().ok()?.checked_mul(seconds).map(Duration::from_secs)
}

/// 🕸️ The registrations of one catalog nobody played under, as of a horizon: the learners that
/// never submitted a run and registered before it, each with the handles it holds; the handles
/// claimed before it beside the identity of a learner that stays (a learner is the identity it
/// registered first — a later claim only occupies a name); and what stays: every other learner,
/// how many of them submitted a run, and the registrations the proctor will count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stale {
    pub tenant: String,
    pub learners: Vec<Subject>,
    pub surplus: Vec<String>,
    pub kept_learners: u64,
    pub kept_played: u64,
    pub kept_handles: u64,
    pub registrations_left: u64,
}

impl Stale {
    /// 🧮️ How many registrations go: the anonymous learners, the handles of the learners and the
    /// handles beside an identity — what the proctor's count of registrations drops by.
    pub fn registrations(&self) -> u64 {
        self.learners.iter().map(|subject| u64::from(subject.identity == Some(Identity::Anonymous)) + subject.handles.len() as u64).sum::<u64>() + self.surplus.len() as u64
    }

    /// 🪣️ What to remove, in the batches it is removed in: at most [`PRUNE_BATCH`] learners per
    /// batch, every learner together with all of its handles — a handle left without its learner
    /// would be relayed to it again at the next start —, then the handles beside an identity, each
    /// with the receipt its relay left with the learner that stays.
    pub fn batches(&self) -> Vec<Batch> {
        let learners = self.learners.chunks(PRUNE_BATCH).map(|batch| Batch { actors: batch.iter().flat_map(Subject::actors).collect(), relays: Vec::new() });
        let surplus = self.surplus.chunks(PRUNE_BATCH).map(|batch| Batch { actors: batch.iter().map(|key| handle_key(&self.tenant, key)).collect(), relays: batch.iter().map(|key| enrollment_receipt(&self.tenant, &quiz::handle_actor_id(key))).collect() });
        learners.chain(surplus).collect()
    }
}

/// 📦️ What one transaction of a `prune` removes: whole actors, and the receipts that learners
/// which stay hold of the relays of handles which go — left behind, such a receipt would refuse
/// the relay of the next claim of that handle.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Batch {
    pub actors: Vec<ActorKey>,
    pub relays: Vec<IdempotencyKey>,
}

/// 🕯️ What a `prune` of everything registered before `before` (milliseconds since the epoch) finds,
/// catalog by catalog: read from the streams themselves, never from a read model. A registration
/// that cannot be read is an error, like for an erasure.
pub fn stale(log: &SqliteAuthorityStore, before: u64) -> Result<Vec<Stale>, Box<dyn std::error::Error>> {
    let registration = |event: &server::contract::EventRecord| match serde_json::from_slice::<Event>(&event.payload) {
        Ok(Event::LearnerRegistered { learner, identity, .. }) => Ok((learner, identity)),
        _ => Err(format!("event {} of {}/{} ({}) is no registration this proctor can read", event.seq, event.stream.kind, event.stream.id, event.kind)),
    };
    let mut found = Vec::new();
    for tenant in log.tenants()? {
        let spans = log.stream_spans(&tenant, LEARNER, RUN_SUBMITTED)?;
        let mut identities: HashMap<String, Identity> = HashMap::new();
        for event in log.first_events(&tenant, LEARNER)? {
            identities.insert(event.stream.id.clone(), registration(&event)?.1);
        }
        let mut held: BTreeMap<String, Vec<(String, u64)>> = BTreeMap::new();
        for event in log.stream_events(&tenant, HANDLE)? {
            let key = quiz::handle_key_of(&event.stream.id).ok_or_else(|| format!("{}/{} is no handle stream this proctor can read", event.stream.kind, event.stream.id))?;
            held.entry(registration(&event)?.0).or_default().push((key, event.hlc.millis));
        }
        let doomed: BTreeSet<&str> = spans.iter().filter(|span| span.marked == 0 && span.since < before).map(|span| span.id.as_str()).collect();
        let learners: Vec<Subject> = doomed.iter().map(|learner| Subject { tenant: tenant.clone(), learner: (*learner).to_string(), identity: identities.get(*learner).cloned(), handles: held.get(*learner).into_iter().flatten().map(|(key, _)| key.clone()).collect() }).collect();
        let own = |learner: &str| match identities.get(learner) {
            Some(Identity::Pseudonym { handle } | Identity::Name { handle }) => quiz::normalize_handle(handle).map(|normalized| normalized.key),
            _ => None,
        };
        let staying = |learner: &str| identities.contains_key(learner) && !doomed.contains(learner);
        let surplus: Vec<String> = held.iter().filter(|(learner, _)| staying(learner)).flat_map(|(learner, handles)| handles.iter().filter(|(key, since)| *since < before && own(learner).as_ref() != Some(key)).map(|(key, _)| key.clone())).collect();
        let kept_handles = held.values().map(Vec::len).sum::<usize>() - learners.iter().map(|subject| subject.handles.len()).sum::<usize>() - surplus.len();
        let kept_anonymous = spans.iter().filter(|span| !doomed.contains(span.id.as_str()) && identities.get(&span.id) == Some(&Identity::Anonymous)).count();
        found.push(Stale {
            tenant,
            kept_learners: (spans.len() - learners.len()) as u64,
            kept_played: spans.iter().filter(|span| span.marked > 0).count() as u64,
            kept_handles: kept_handles as u64,
            registrations_left: (kept_anonymous + kept_handles) as u64,
            learners,
            surplus,
        });
    }
    Ok(found)
}

/// 🧹️ Prune the database of a stopped proctor of the registrations nobody played under, or with
/// `dry_run` only count them. Whole learners go batch by batch — each batch one transaction —
/// with progress; an interrupt stops between batches, and what was removed until then is removed
/// for good. The file is rewritten at the end, so the removed names are no bytes of it.
async fn prune(environment: &impl Fn(&str) -> Option<String>, pruning: &Pruning) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let data = ProctorConfig::data_directory(environment)?;
    let database = Database::open_offline(&data)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| u64::try_from(since.as_millis()).unwrap_or(u64::MAX));
    let before = now.saturating_sub(u64::try_from(pruning.age.as_millis()).unwrap_or(u64::MAX));
    let found = stale(&SqliteAuthorityStore::new(database.clone()), before)?;
    print!("{}", pruning_report(&found, pruning, UNIX_EPOCH + Duration::from_millis(before)));
    let batches: Vec<Batch> = found.iter().flat_map(Stale::batches).collect();
    if pruning.dry_run || batches.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let interrupt = interrupt();
    let (streams, mut done, mut reported, mut removed) = (batches.iter().map(|batch| batch.actors.len()).sum::<usize>(), 0, 0, Removed::default());
    for batch in &batches {
        if interrupt.is_cancelled_now() {
            break;
        }
        removed += database.remove(&batch.actors, &batch.relays)?;
        done += batch.actors.len();
        announce_pruned(done, streams, &mut reported);
    }
    if done > 0 {
        eprintln!("[INFO] prune: rewriting the database without the removed bytes");
        database.scrub()?;
    }
    println!("removed {} of {streams} streams: {} events, {} receipts, {} outbox rows, {} snapshots, {} leases; read models: {} rows dropped, the next start rebuilds them from the events that are left", done, removed.events, removed.receipts, removed.outbox, removed.snapshots, removed.leases, removed.projections);
    if done < streams {
        eprintln!("[INFO] prune interrupted; what was removed is removed for good, run it again for the rest");
        return Ok(ExitCode::from(INTERRUPTED));
    }
    println!("done: the handles are free again, the learner ids are unknown to the proctor; {} registrations are left", found.iter().map(|stale| stale.registrations_left).sum::<u64>());
    Ok(ExitCode::SUCCESS)
}

/// 🛎️ Report the streams removed in steps of ten percent.
fn announce_pruned(done: usize, streams: usize, reported: &mut usize) {
    let percent = done.saturating_mul(100).checked_div(streams).unwrap_or(100);
    if percent >= *reported + 10 || done == streams {
        *reported = percent;
        eprintln!("[INFO] prune: {done} of {streams} streams ({percent}%)");
    }
}

/// 🫙️ What a `prune` removes, or with `dry_run` would remove, catalog by catalog: counts, never
/// names.
pub fn pruning_report(found: &[Stale], pruning: &Pruning, before: SystemTime) -> String {
    let verb = if pruning.dry_run { "would prune" } else { "pruning" };
    let mut lines = Vec::new();
    for stale in found {
        let kind = |wanted: fn(&Identity) -> bool| stale.learners.iter().filter(|subject| subject.identity.as_ref().is_some_and(wanted)).count();
        lines.push(format!("{verb} {} registrations of catalog {} older than {} (registered before {})", stale.registrations(), stale.tenant, pruning.older_than, timestamp(before)));
        lines.push(format!(
            "  learners that never submitted a run: {} ({} anonymous, {} under a pseudonym, {} under a name), holding {} handles",
            stale.learners.len(),
            kind(|identity| matches!(identity, Identity::Anonymous)),
            kind(|identity| matches!(identity, Identity::Pseudonym { .. })),
            kind(|identity| matches!(identity, Identity::Name { .. })),
            stale.learners.iter().map(|subject| subject.handles.len()).sum::<usize>()
        ));
        lines.push(format!("  handles claimed beside the identity of a learner that stays: {}", stale.surplus.len()));
        lines.push(format!("  staying: {} learners ({} of them submitted a run), {} handles, {} registrations", stale.kept_learners, stale.kept_played, stale.kept_handles, stale.registrations_left));
    }
    if found.is_empty() {
        lines.push("the database holds no registration".to_string());
    }
    if pruning.dry_run {
        lines.push("dry run: nothing was changed".to_string());
    } else if found.iter().all(|stale| stale.learners.is_empty() && stale.surplus.is_empty()) {
        lines.push("nothing to prune: nothing was changed".to_string());
    }
    lines.join("\n") + "\n"
}

/// 🧵️ A spool file of this process beside the database.
fn spool(data: &Path, purpose: &str) -> PathBuf {
    data.join(format!("{DATABASE_FILE}.{purpose}-{}", std::process::id()))
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

/// ✋️ A token cancelled by the first interrupt or termination request.
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
