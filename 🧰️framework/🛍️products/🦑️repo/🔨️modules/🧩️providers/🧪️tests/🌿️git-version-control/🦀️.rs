//! 🦀️ Rust side of the git version control case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_providers::{GitVersionControlProvider, ProcessRunner, ProcessRequest, ProcessRunners, SystemProcessRunner, VersionControlProvider};
    use semio_repo_test_host::{Context, Json, Outcome};
    use std::path::PathBuf;

    /// 🧪️ A throwaway repository, one per scenario so nothing is shared and nothing accumulates.
    ///
    /// Deliberately NOT under `ctx.work_dir`: the work directory lives inside this repository's own
    /// cache, so `git rev-parse` there would answer from the surrounding checkout and the
    /// not-a-repository scenario could never fail. The name is fixed per implementation and scenario,
    /// so a re-run reuses one directory instead of leaving a new one behind.
    fn scratch(ctx: &Context, initialised: bool) -> Result<String, String> {
        let dir: PathBuf = std::env::temp_dir().join(format!("semio-providers-rust-{}", ctx.scenario.id));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
        let root = dir.to_string_lossy().to_string();
        if !initialised {
            return Ok(root);
        }
        let setup = SystemProcessRunner::new();
        let run = |args: &[&str]| -> Result<(), String> {
            let owned: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
            let outcome = setup.run(&ProcessRequest::new("git", &owned, &root));
            if outcome.status != 0 {
                return Err(format!("git {args:?} failed: {}", outcome.stderr.trim()));
            }
            Ok(())
        };
        run(&["init", "-b", "main"])?;
        run(&["config", "user.email", "test@test.com"])?;
        run(&["config", "user.name", "Test"])?;
        run(&["config", "commit.gpgsign", "false"])?;
        std::fs::write(dir.join("file.txt"), "hello\n").map_err(|error| error.to_string())?;
        run(&["add", "-A"])?;
        run(&["commit", "-m", "initial"])?;
        Ok(root)
    }

    fn provider() -> GitVersionControlProvider {
        GitVersionControlProvider::new(ProcessRunners::from(SystemProcessRunner::new()))
    }

    fn is_object_name(value: &str) -> bool {
        value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
    }

    /// 🌿️ A fresh repository reports its branch and a stable HEAD.
    pub fn a_fresh_repository_reports_its_branch_and_head(ctx: &Context) -> Result<Outcome, String> {
        let root = scratch(ctx, true)?;
        let provider = provider();
        let branch = provider.current_branch(&root).map_err(|error| error.message)?;
        let first = provider.current_checkpoint(&root).map_err(|error| error.message)?;
        let second = provider.current_checkpoint(&root).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("branch".to_string(), Json::String(branch)),
            ("headIsAnObjectName".to_string(), Json::Bool(is_object_name(&first))),
            ("bothReadsAgree".to_string(), Json::Bool(first == second)),
            ("kind".to_string(), Json::String(provider.kind().to_string())),
        ])))
    }

    /// 📄️ Staging every change lists exactly the added paths.
    pub fn staging_lists_the_added_paths(ctx: &Context) -> Result<Outcome, String> {
        let root = scratch(ctx, true)?;
        let provider = provider();
        let before = provider.staged_files(&root).map_err(|error| error.message)?;
        std::fs::write(PathBuf::from(&root).join("a.txt"), "a\n").map_err(|error| error.to_string())?;
        std::fs::write(PathBuf::from(&root).join("b.txt"), "b\n").map_err(|error| error.to_string())?;
        provider.stage_all(&root).map_err(|error| error.message)?;
        let mut after = provider.staged_files(&root).map_err(|error| error.message)?;
        after.sort();
        Ok(Outcome::projection(Json::Object(vec![
            ("before".to_string(), Json::Array(before.into_iter().map(Json::String).collect())),
            ("after".to_string(), Json::Array(after.into_iter().map(Json::String).collect())),
        ])))
    }

    /// 💾️ A checkpoint commits the working tree and moves HEAD forward.
    pub fn a_checkpoint_commits_and_advances_head(ctx: &Context) -> Result<Outcome, String> {
        let root = scratch(ctx, true)?;
        let provider = provider();
        let before = provider.current_checkpoint(&root).map_err(|error| error.message)?;
        std::fs::write(PathBuf::from(&root).join("file2.txt"), "world\n").map_err(|error| error.to_string())?;
        let checkpoint = provider.checkpoint(&root, "add file2").map_err(|error| error.message)?;
        let after = provider.current_checkpoint(&root).map_err(|error| error.message)?;
        let staged = provider.staged_files(&root).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("headAdvanced".to_string(), Json::Bool(checkpoint != before)),
            ("checkpointIsAnObjectName".to_string(), Json::Bool(is_object_name(&checkpoint))),
            ("checkpointIsHead".to_string(), Json::Bool(checkpoint == after)),
            ("stagedAfterCheckpoint".to_string(), Json::Array(staged.into_iter().map(Json::String).collect())),
        ])))
    }

    /// ⚠️ Reading a branch outside a repository is reported, never defaulted.
    pub fn reading_outside_a_repository_is_an_error(ctx: &Context) -> Result<Outcome, String> {
        let root = scratch(ctx, false)?;
        let provider = provider();
        Ok(Outcome::projection(Json::Object(vec![
            ("branchFailed".to_string(), Json::Bool(provider.current_branch(&root).is_err())),
            ("checkpointFailed".to_string(), Json::Bool(provider.current_checkpoint(&root).is_err())),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("a-fresh-repository-reports-its-branch-and-head", subject::a_fresh_repository_reports_its_branch_and_head)
        .subject("staging-lists-the-added-paths", subject::staging_lists_the_added_paths)
        .subject("a-checkpoint-commits-and-advances-head", subject::a_checkpoint_commits_and_advances_head)
        .subject("reading-outside-a-repository-is-an-error", subject::reading_outside_a_repository_is_an_error);
    registered
}
//#endregion 🔖️Registration
