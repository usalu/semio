//! 🔌️ Git source: the `GitLogSource` door with the system git and the recorded transcript.

use super::{join_numstat_chunks, numstat_log_args, numstat_log_range_args, numstat_log_ranges};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

//#region 🔌️Git Source

/// 🔌️ The only door this crate has to git. Everything else is a pure function over its output.
pub trait GitLogSource: Sync {
    /// 📜️ The raw `git log --numstat` stream for a ref (`""` meaning `HEAD`).
    fn numstat_log(&self, git_ref: &str) -> Result<String, String>;
    /// 🧾️ The repository-relative tracked paths at a ref (`""` meaning the working tree).
    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String>;
    /// 📄️ The bytes of one tracked path at a ref (`""` meaning the working tree).
    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String>;
}

/// 🖥️ The real git, reached through `std::process`.
#[derive(Clone, Debug)]
pub struct SystemGit {
    repo: PathBuf,
}

impl SystemGit {
    /// 🖥️ Binds the source to one repository root.
    pub fn new(repo: impl AsRef<Path>) -> Self {
        SystemGit { repo: repo.as_ref().to_path_buf() }
    }

    /// 🖥️ Runs git in the bound repository and returns stdout, or the trimmed stderr as an error.
    fn run(&self, args: &[String]) -> Result<Vec<u8>, String> {
        let output = Command::new("git").args(args).current_dir(&self.repo).output().map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
        }
        Ok(output.stdout)
    }
}

impl SystemGit {
    /// 🧵️ The first-parent chain of a ref, newest commit first, or nothing when git cannot name one.
    fn first_parent_chain(&self, git_ref: &str) -> Vec<String> {
        let mut args = vec!["rev-list".to_string(), "--first-parent".to_string()];
        args.push(if git_ref.trim().is_empty() { "HEAD".to_string() } else { git_ref.trim().to_string() });
        let Ok(stdout) = self.run(&args) else { return Vec::new() };
        String::from_utf8_lossy(&stdout).lines().map(str::trim).filter(|line| !line.is_empty()).map(str::to_string).collect()
    }

    /// 🧵️ How many ranges the chain is split into: enough to keep every core busy on a chain whose
    /// per-commit cost is heavily skewed towards the newest commits, and one for a short history.
    fn numstat_chunks(commits: usize) -> usize {
        if commits <= MINIMUM_CHUNKED_COMMITS {
            return 1;
        }
        let workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).clamp(1, MAXIMUM_NUMSTAT_WORKERS);
        (workers * NUMSTAT_CHUNKS_PER_WORKER).min(commits)
    }
}

/// 🧵️ A history shorter than this is walked in one `git log`; the split would cost more than it saves.
const MINIMUM_CHUNKED_COMMITS: usize = 32;

/// 🧵️ The most concurrent `git log` processes a numstat walk starts.
pub(super) const MAXIMUM_NUMSTAT_WORKERS: usize = 16;

/// 🧵️ Ranges per worker, so a worker that draws a cheap range picks up another instead of idling.
const NUMSTAT_CHUNKS_PER_WORKER: usize = 4;

impl GitLogSource for SystemGit {
    fn numstat_log(&self, git_ref: &str) -> Result<String, String> {
        let chain = self.first_parent_chain(git_ref);
        let chunks = Self::numstat_chunks(chain.len());
        if chunks < 2 {
            return Ok(String::from_utf8_lossy(&self.run(&numstat_log_args(git_ref))?).into_owned());
        }
        let ranges = numstat_log_ranges(&chain, chunks);
        let outputs: Vec<Mutex<Option<Result<String, String>>>> = ranges.iter().map(|_| Mutex::new(None)).collect();
        let next = AtomicUsize::new(0);
        let workers = ranges.len().min(MAXIMUM_NUMSTAT_WORKERS);
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| loop {
                    let index = next.fetch_add(1, Ordering::SeqCst);
                    let Some(range) = ranges.get(index) else { return };
                    let answer = self.run(&numstat_log_range_args(range)).map(|stdout| String::from_utf8_lossy(&stdout).into_owned());
                    *outputs[index].lock().expect("numstat chunk") = Some(answer);
                });
            }
        });
        let mut collected = Vec::with_capacity(outputs.len());
        for slot in outputs {
            collected.push(slot.into_inner().expect("numstat chunk").expect("every chunk ran")?);
        }
        Ok(join_numstat_chunks(&collected))
    }

    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String> {
        let args: Vec<String> = if git_ref.trim().is_empty() {
            ["ls-files", "-z"].iter().map(|value| (*value).to_string()).collect()
        } else {
            ["ls-tree", "-r", "--name-only", "-z", git_ref].iter().map(|value| (*value).to_string()).collect()
        };
        Ok(String::from_utf8_lossy(&self.run(&args)?).split('\0').filter(|value| !value.is_empty()).map(|value| value.replace('\\', "/")).collect())
    }

    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String> {
        if git_ref.trim().is_empty() {
            return std::fs::read(self.repo.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR))).map_err(|error| error.to_string());
        }
        self.run(&["show".to_string(), format!("{git_ref}:{rel}")])
    }
}

/// 🎞️ A recorded git session: exactly what the real CLI answered, keyed by ref.
///
/// A transcript is what makes every metrics test hermetic and cross-language: the same JSON file
/// drives the Rust subject, the Go subject and the TypeScript oracle, and the oracle additionally
/// replays the recipe against the real `git` binary to prove the recording is still faithful.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitTranscript {
    /// 🆔️ The transcript's own identifier, used in test projections.
    #[serde(default)]
    pub id: String,
    /// 📜️ `git log --numstat` stdout keyed by the ref it was recorded for (`""` = `HEAD`).
    #[serde(default)]
    pub logs: BTreeMap<String, String>,
    /// 🧾️ Tracked paths keyed by ref.
    #[serde(default)]
    pub tracked: BTreeMap<String, Vec<String>>,
    /// 📄️ File bodies keyed by `"<ref>\u{1}<path>"`, stored as UTF-8 text.
    #[serde(default)]
    pub blobs: BTreeMap<String, String>,
}

impl GitTranscript {
    /// 🔑️ The blob key for a `(ref, path)` pair.
    pub fn blob_key(git_ref: &str, rel: &str) -> String {
        format!("{git_ref}\u{1}{rel}")
    }

    /// 📥️ Parses a transcript from its JSON encoding.
    pub fn from_json(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|error| error.to_string())
    }
}

impl GitLogSource for GitTranscript {
    fn numstat_log(&self, git_ref: &str) -> Result<String, String> {
        self.logs.get(git_ref).cloned().ok_or_else(|| format!("transcript has no log for ref {git_ref:?}"))
    }

    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String> {
        self.tracked.get(git_ref).cloned().ok_or_else(|| format!("transcript has no tracked path list for ref {git_ref:?}"))
    }

    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String> {
        self.blobs.get(&GitTranscript::blob_key(git_ref, rel)).map(|body| body.as_bytes().to_vec()).ok_or_else(|| format!("transcript has no blob for {git_ref:?}:{rel}"))
    }
}

//#endregion 🔌️Git Source
