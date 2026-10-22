// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔖️micro-commit carries the micro-commit delegation, the git hook installation and the start of the script through the repo command.

// #endregion 🧲️Header

package hooks

import (
	context "context"
	errors "errors"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"

	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔖️MicroCommitDelegation

// 📜️MicroCommitArgv answers the argv the repo binary hands to its own script route for a micro-commit subcommand.
func MicroCommitArgv(repo string, args []string) []string {
	argv := []string{repo, "./📜️script.ts", "micro-commit"}
	return append(argv, args...)
}

// 🔖️RunMicroCommit delegates a micro-commit subcommand to the monorepo script through the process runner.
func RunMicroCommit(runner providers.ProcessRunner, repoRoot string, repo string, args []string) providers.ProcessOutcome {
	return runner.Run(providers.ProcessRequest{Argv: MicroCommitArgv(repo, args), Cwd: repoRoot})
}

// ♻️RunMicroCommitReset is the `micro-commit reset` delegation every post-checkpoint git hook performs.
func RunMicroCommitReset(runner providers.ProcessRunner, repoRoot string, repo string) providers.ProcessOutcome {
	return RunMicroCommit(runner, repoRoot, repo, []string{"reset"})
}

// #endregion 🔖️MicroCommitDelegation

// #region 🔷️Configure

// 🪝️repoManagedGitHooks lists git hook filenames this repo may install and must never leave enabled.
var repoManagedGitHooks = []string{
	"pre-commit",
	"post-commit",
	"prepare-commit-msg",
	"commit-msg",
	"pre-push",
	"pre-rebase",
	"post-merge",
	"post-checkout",
	"post-rewrite",
}

// 🔄️installMicroCommitHook copies a repo hook script into `.git/hooks` (must stay non-blocking).
func installMicroCommitHook(repoRoot, hookName string) error {
	if repoRoot == "" {
		return nil
	}
	src := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks", hookName)
	data, err := os.ReadFile(src)
	if err != nil {
		return fmt.Errorf("read micro-commit hook %s: %w", hookName, err)
	}
	hookPath := filepath.Join(repoRoot, ".git", "hooks", hookName)
	if err := os.WriteFile(hookPath, data, 0o755); err != nil {
		return fmt.Errorf("install micro-commit hook %s: %w", hookName, err)
	}
	return nil
}

// 🔄️installMicroCommitHooks installs micro-commit git hooks (prepare-commit-msg, post-commit reset).
func InstallMicroCommitHooks(repoRoot string) error {
	for _, name := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		if err := installMicroCommitHook(repoRoot, name); err != nil {
			return err
		}
	}
	return nil
}

// 🧹️removeGitHooks deletes repo-managed git hooks so commits, rebases, and squashes stay unblocked.
func RemoveGitHooks(repoRoot string) ([]string, error) {
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = workspace.FindRepoRoot(cwd)
	}
	if repoRoot == "" {
		return nil, nil
	}
	hooksDir := filepath.Join(repoRoot, ".git", "hooks")
	var removed []string
	for _, hookName := range repoManagedGitHooks {
		hookPath := filepath.Join(hooksDir, hookName)
		if err := os.Remove(hookPath); err != nil {
			if !os.IsNotExist(err) {
				return removed, fmt.Errorf("remove git hook %s: %w", hookName, err)
			}
			continue
		}
		removed = append(removed, hookName)
	}
	return removed, nil
}

// #endregion 🔷️Configure

// #region 🔖️MicroCommit

func RunMicroCommitScript(ctx context.Context, repoRoot string, args []string) error {
	repo, err := repoExecutable()
	if err != nil {
		return err
	}
	cmdArgs := append([]string{"./📜️script.ts", "micro-commit"}, args...)
	c := workspace.ToolCommandContext(ctx, repo, cmdArgs...)
	c.Dir = repoRoot
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	c.Stdin = os.Stdin
	if err := c.Run(); err != nil {
		var exitErr *exec.ExitError
		if errors.As(err, &exitErr) {
			return workspace.ExitError{Code: exitErr.ExitCode()}
		}
		return err
	}
	return nil
}

// 🚀️repoExecutable is the running repo command: the micro-commit script starts through its script route, which provisions and runs the pinned runtime, so no runtime is looked up here.
func repoExecutable() (string, error) { return os.Executable() }

// #endregion 🔖️MicroCommit
