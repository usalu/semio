// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Configure and micro-commit commands.

// #endregion 🧲️Header

package client

import (
	"context"
	"errors"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

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
func installMicroCommitHooks(repoRoot string) error {
	for _, name := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		if err := installMicroCommitHook(repoRoot, name); err != nil {
			return err
		}
	}
	return nil
}

// 🧹️removeGitHooks deletes repo-managed git hooks so commits, rebases, and squashes stay unblocked.
func removeGitHooks(repoRoot string) ([]string, error) {
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = findRepoRoot(cwd)
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

// 🆕️configureCommand creates the `configure` command command.
func configureCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "configure",
		Short: "Remove blocking git hooks; repo config files are edited manually",
		RunE: func(cmd *command.Command, args []string) error {
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, err := os.Getwd()
				if err != nil {
					return err
				}
				repoRoot = findRepoRoot(cwd)
			}
			removed, err := removeGitHooks(repoRoot)
			if err != nil {
				return err
			}
			if err := installMicroCommitHooks(repoRoot); err != nil {
				return err
			}
			fmt.Fprintln(cmd.OutOrStdout(), "repo config generation is disabled; edit checked-in config files manually")
			if len(removed) == 0 {
				fmt.Fprintln(cmd.OutOrStdout(), "git hooks: none to remove; micro-commit hooks installed")
				return nil
			}
			fmt.Fprintf(cmd.OutOrStdout(), "git hooks removed: %s; micro-commit hooks installed\n", strings.Join(removed, ", "))
			return nil
		},
	}
	return cmd
}

func microCommitCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "micro-commit [subcommand] [args...]",
		Short: "Micro-commit workflow (reset, prepare-commit-msg, stage, diff, prepare)",
		RunE: func(cmd *command.Command, args []string) error {
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, err := os.Getwd()
				if err != nil {
					return err
				}
				repoRoot = findRepoRoot(cwd)
			}
			if repoRoot == "" {
				return fmt.Errorf("repository root not found")
			}
			ctx := cmd.Context()
			if config.Timeout > 0 {
				ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
				defer cancel()
				ctx = ctxWithTimeout
			}
			return runMicroCommitScript(ctx, repoRoot, args)
		},
	}
}

func runMicroCommitScript(ctx context.Context, repoRoot string, args []string) error {
	bun, err := resolveBunBinary(repoRoot)
	if err != nil {
		return err
	}
	cmdArgs := append([]string{"./📜️script.ts", "micro-commit"}, args...)
	c := exec.CommandContext(ctx, bun, cmdArgs...)
	c.Dir = repoRoot
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	c.Stdin = os.Stdin
	if err := c.Run(); err != nil {
		var exitErr *exec.ExitError
		if errors.As(err, &exitErr) {
			return ExitError{Code: exitErr.ExitCode()}
		}
		return err
	}
	return nil
}

func resolveBunBinary(repoRoot string) (string, error) {
	if pinned := strings.TrimSpace(os.Getenv("COMPOSE_BUN")); pinned != "" {
		if _, err := os.Stat(pinned); err == nil {
			return pinned, nil
		}
	}
	pinPath := filepath.Join(RepoMetaDirForRoot(repoRoot), "🐹️compose-micro-commit-bun")
	if data, err := os.ReadFile(pinPath); err == nil {
		if line := strings.TrimSpace(strings.Split(string(data), "\n")[0]); line != "" {
			if _, statErr := os.Stat(line); statErr == nil {
				return line, nil
			}
		}
	}
	if bunInstall := strings.TrimSpace(os.Getenv("BUN_INSTALL")); bunInstall != "" {
		for _, name := range []string{"bun", "bun.exe"} {
			candidate := filepath.Join(bunInstall, "bin", name)
			if _, err := os.Stat(candidate); err == nil {
				return candidate, nil
			}
		}
	}
	for _, rel := range []string{"node_modules/.bin/bun", "node_modules/.bin/bun.exe"} {
		candidate := filepath.Join(repoRoot, rel)
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
	}
	if path, err := exec.LookPath("bun"); err == nil {
		return path, nil
	}
	return "", fmt.Errorf("bun not found — install bun or set COMPOSE_BUN")
}
