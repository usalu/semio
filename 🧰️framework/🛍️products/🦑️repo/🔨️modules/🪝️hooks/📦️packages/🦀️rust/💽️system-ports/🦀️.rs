//! 💽️ System ports: micro-commit delegation and the system-backed environment and test file resolver.
//!
//! Holds the `🔖️MicroCommit`, `💽️SystemPorts` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🔖️MicroCommit
/// 📜️ The argv the repo binary hands to bun for a micro-commit subcommand.
pub fn micro_commit_argv(bun: &str, args: &[String]) -> Vec<String> {
    let mut argv = vec![bun.to_string(), "./📜️script.ts".to_string(), "micro-commit".to_string()];
    argv.extend(args.iter().cloned());
    argv
}

/// 🕯️ Every place a bun binary is looked for, in order: the pinned override, the repo's own pin
/// file, a `BUN_INSTALL` tree, the workspace's `node_modules/.bin`, then `PATH`.
pub fn bun_candidates(repo_root: &Path, pinned: Option<&str>, bun_install: Option<&str>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(pinned) = pinned {
        if !pinned.trim().is_empty() {
            candidates.push(PathBuf::from(pinned.trim()));
        }
    }
    candidates.push(repo_meta_dir_for_root(repo_root).join("🐹️compose-micro-commit-bun"));
    if let Some(bun_install) = bun_install {
        if !bun_install.trim().is_empty() {
            for name in ["bun", "bun.exe"] {
                candidates.push(Path::new(bun_install.trim()).join("bin").join(name));
            }
        }
    }
    for relative in ["node_modules/.bin/bun", "node_modules/.bin/bun.exe"] {
        candidates.push(repo_root.join(relative));
    }
    candidates
}

/// 🔖️ Delegates a micro-commit subcommand to the monorepo script through the process runner, so the
/// version control hooks (`post-commit`, `post-checkout`, …) reach exactly one implementation.
pub fn run_micro_commit<R: ProcessRunner>(runner: &R, repo_root: &str, bun: &str, args: &[String]) -> ProcessOutcome {
    runner.run(&ProcessRequest { argv: micro_commit_argv(bun, args), cwd: Some(repo_root.to_string()), stdin: None })
}

/// ♻️ The `micro-commit reset` delegation every post-checkpoint git hook performs.
pub fn run_micro_commit_reset<R: ProcessRunner>(runner: &R, repo_root: &str, bun: &str) -> ProcessOutcome {
    run_micro_commit(runner, repo_root, bun, &["reset".to_string()])
}
//#endregion 🔖️MicroCommit

//#region 💽️SystemPorts
// 💽️ The two ports of `🔌️Ports` bound to the real machine. Everything above stays pure: these are
// the only definitions in this crate that read a checkpoint, walk a codebase or spawn anything.

/// 💻️ The environment that reads the checkpoint identity and message off the real machine.
///
/// Twin of `SystemEnvironment` in `📦️packages/🐹️go/🐹️.go`. `format_file` is deliberately inert:
/// the Go twin delegates to `📜️statutes`' formatter registry, which this crate may not reach
/// without inverting the dependency — the CLI passes a formatting environment when it wants one.
#[derive(Debug, Clone)]
pub struct SystemHookEnvironment {
    pub(super) repo_root: PathBuf,
}

impl SystemHookEnvironment {
    /// 🆕️ Binds the environment to one repository root.
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self { repo_root: repo_root.into() }
    }
}

impl HookEnvironment for SystemHookEnvironment {
    fn current_checkpoint(&self) -> String {
        let Ok(output) = std::process::Command::new("git").args(["rev-parse", "HEAD"]).current_dir(&self.repo_root).output() else { return String::new() };
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn checkpoint_message(&self, repo_root: &str) -> String {
        let root = if repo_root.is_empty() { self.repo_root.clone() } else { PathBuf::from(repo_root) };
        std::fs::read_to_string(root.join(".git").join("COMMIT_EDITMSG")).map(|body| body.trim().to_string()).unwrap_or_default()
    }
}

/// 🏃️ The resolver that answers through the repository's own test runner and codebase.
///
/// Twin of `WorkspaceTestFileResolver`: the command is narrowed to its test segment, the segment
/// resolved into the files it selects, and each file mapped to the lab id `🗂️codebase` gives it.
pub struct WorkspaceTestFileResolver {
    pub(super) repo_root: PathBuf,
    snapshot: std::cell::OnceCell<semio_framework_repo_test_runner::FilesystemSnapshot>,
}

impl WorkspaceTestFileResolver {
    /// 🆕️ Binds the resolver to one repository root. The walk is deferred until a hook actually
    /// asks which test files a command selects — most hook events never do.
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self { repo_root: repo_root.into(), snapshot: std::cell::OnceCell::new() }
    }

    /// 🌍️ The repository snapshot, walked once on first use.
    fn snapshot(&self) -> &semio_framework_repo_test_runner::FilesystemSnapshot {
        self.snapshot.get_or_init(|| {
            let ignore = semio_framework_repo_workspace::GitIgnore::compile_file(&self.repo_root.join(".gitignore")).ok();
            semio_framework_repo_test_runner::FilesystemSnapshot::from_root(&self.repo_root, ignore.as_ref())
        })
    }
}

impl TestFileResolver for WorkspaceTestFileResolver {
    fn test_files(&self, command: &str, cwd: &str) -> Vec<String> {
        let command = command.trim();
        if command.is_empty() {
            return Vec::new();
        }
        let root = self.repo_root.to_string_lossy().replace('\\', "/");
        let mut working = if cwd.trim().is_empty() { root } else { cwd.replace('\\', "/") };
        let (segment, extracted) = semio_framework_repo_test_runner::extract_test_segment_from_command(command);
        let mut command = command.to_string();
        if !segment.is_empty() && segment != command {
            command = segment;
            if !extracted.is_empty() {
                working = if semio_framework_repo_test_runner::is_absolute_path(&extracted) { extracted } else { semio_framework_repo_test_runner::join_path(&working, &extracted) };
            }
        }
        semio_framework_repo_test_runner::resolve_test_files_from_command(self.snapshot(), &command, &working)
    }

    fn lab_ids(&self, file_paths: &[String]) -> Vec<String> {
        let codebase = semio_framework_repo_codebase::Codebase::new(self.repo_root.clone());
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut found = Vec::new();
        for path in file_paths {
            let id = codebase.build_file_id(path);
            if id.is_empty() || !seen.insert(id.clone()) {
                continue;
            }
            found.push(id);
        }
        found
    }
}
//#endregion 💽️SystemPorts
