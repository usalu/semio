// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Git, devcontainer providers and provider registry.

// #endregion 🧲️Header

package client

import (
	"fmt"
	workspace "github.com/usalu/semio/repo/workspace"
	"strings"
	"time"
)

// 🔌️GitVersionControlProvider holds the data fields for a git version control provider record.
type GitVersionControlProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *GitVersionControlProvider) Kind() string { return "git" }

// 🌐️RepoURL holds the data fields for a RepoURL record.
func (p *GitVersionControlProvider) RepoURL() (string, error) {
	out, err := workspace.ToolCommand("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// ⚙️Configure holds the data fields for a Configure record.
func (p *GitVersionControlProvider) Configure(repoRoot string) error { return nil }

// 💾️Checkpoint holds the data fields for a Checkpoint record.
func (p *GitVersionControlProvider) Checkpoint(repoRoot string, description string) (string, error) {
	if _, _, exitCode := ExecCommand("git", []string{"diff", "--cached", "--quiet"}, repoRoot); exitCode == 0 {
		if err := p.StageAll(repoRoot); err != nil {
			return "", fmt.Errorf("stage all failed: %w", err)
		}
	}
	args := []string{"commit", "-m", description}
	stdout, stderr, exitCode := ExecCommand("git", args, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit failed (exit %d): %s", exitCode, strings.TrimSpace(stderr))
	}
	_ = stdout
	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", fmt.Errorf("failed to get checkpoint sha after checkpoint: %w", err)
	}
	return sha, nil
}

func (p *GitVersionControlProvider) CurrentCheckpoint(repoRoot string) (string, error) {
	stdout, stderr, exitCode := ExecCommand("git", []string{"rev-parse", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// ✔️Checkin holds the data fields for a Checkin record.
func (p *GitVersionControlProvider) Checkin(repoRoot string, contributor string) error {
	contributorBranch := contributor + "/latest"

	_, stderr, exitCode := ExecCommand("git", []string{"fetch", "origin", "main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fetch origin main failed: %s", strings.TrimSpace(stderr))
	}

	currentBranch, err := p.CurrentBranch(repoRoot)
	if err != nil {
		return err
	}

	if currentBranch != contributorBranch {
		_, stderr, exitCode = ExecCommand("git", []string{"switch", contributorBranch}, repoRoot)
		if exitCode != 0 {

			_, stderr, exitCode = ExecCommand("git", []string{"switch", "-c", contributorBranch}, repoRoot)
			if exitCode != 0 {
				return fmt.Errorf("git switch to %s failed: %s", contributorBranch, strings.TrimSpace(stderr))
			}
		}
	}

	_, stderr, exitCode = ExecCommand("git", []string{"merge", "--ff-only", "origin/main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fast-forward merge to main failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔷️Checkout holds the data fields for a Checkout record.
func (p *GitVersionControlProvider) Checkout(repoRoot string, contributor string, description string) (string, error) {
	contributorBranch := contributor + "/latest"
	now := time.Now().UTC()
	archiveBranch := fmt.Sprintf("%s/%04d/%02d/%02d", contributor, now.Year(), now.Month(), now.Day())

	_, stderr, exitCode := ExecCommand("git", []string{"branch", archiveBranch, contributorBranch}, repoRoot)
	if exitCode != 0 {

		for i := 2; i <= 99; i++ {
			candidate := fmt.Sprintf("%s-%d", archiveBranch, i)
			_, _, ec := ExecCommand("git", []string{"branch", candidate, contributorBranch}, repoRoot)
			if ec == 0 {
				archiveBranch = candidate
				break
			}
		}
	}

	_, stderr, exitCode = ExecCommand("git", []string{"switch", "main"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git switch to main failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = ExecCommand("git", []string{"merge", "--squash", contributorBranch}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git squash merge failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = ExecCommand("git", []string{"commit", "-m", description}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit squash merge failed: %s", strings.TrimSpace(stderr))
	}

	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", err
	}
	return sha, nil
}

// 🌿️CurrentBranch holds the data fields for a CurrentBranch record.
func (p *GitVersionControlProvider) CurrentBranch(repoRoot string) (string, error) {
	stdout, stderr, exitCode := ExecCommand("git", []string{"rev-parse", "--abbrev-ref", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse --abbrev-ref HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 📄️StagedFiles holds the data fields for a StagedFiles record.
func (p *GitVersionControlProvider) StagedFiles(repoRoot string) ([]string, error) {
	stdout, stderr, exitCode := ExecCommand("git", []string{"diff", "--cached", "--name-only"}, repoRoot)
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff --cached --name-only failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.ReplaceAll(stdout, "\r\n", "\n"), "\n") {
		if line != "" {
			files = append(files, line)
		}
	}
	return files, nil
}

// 🏷️StageAll holds the data fields for a StageAll record.
func (p *GitVersionControlProvider) StageAll(repoRoot string) error {
	_, stderr, exitCode := ExecCommand("git", []string{"add", "-A"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git add -A failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🐳️DevcontainerSandboxProvider holds the data fields for a devcontainer sandbox provider record.
type DevcontainerSandboxProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *DevcontainerSandboxProvider) Kind() string { return "devcontainer" }

// ⚙️Configure holds the data fields for a Configure record.
func (p *DevcontainerSandboxProvider) Configure(repoRoot string) error { return nil }

// 🔌️AllAgentClientProviders returns all registered agent-client providers.
func AllAgentClientProviders() []AgentClientProvider {
	return []AgentClientProvider{
		&CopilotAgentClientProvider{},
		&CursorAgentClientProvider{},
		&WindsurfAgentClientProvider{},
		&ClaudeCodeAgentClientProvider{},
		&DroidAgentClientProvider{},
		&CodexAgentClientProvider{},
		&AntigravityAgentClientProvider{},
		&KiroAgentClientProvider{},
	}
}

// 💻️GetAgentClientProvider returns the agent-client provider for the given client slug.
func GetAgentClientProvider(client string) AgentClientProvider {
	for _, p := range AllAgentClientProviders() {
		if p.Kind() == client {
			return p
		}
	}
	return nil
}

// 🐙️DefaultManagementProvider returns the default management provider (GitHub).
func DefaultManagementProvider() ManagementProvider {
	return &GitHubManagementProvider{}
}

// 📌️DefaultVersionControlProvider returns the default version control provider (Git).
func DefaultVersionControlProvider() VersionControlProvider {
	return &GitVersionControlProvider{}
}

// 🐳️DefaultSandboxProvider returns the default sandbox provider (Devcontainer).
func DefaultSandboxProvider() SandboxProvider {
	return &DevcontainerSandboxProvider{}
}

// 💿️mgmtProvider holds the data fields for a mgmtProvider record.
var mgmtProvider ManagementProvider = DefaultManagementProvider()

// 🔷️GetManagementProvider holds the data fields for a GetManagementProvider record.
func GetManagementProvider() ManagementProvider { return mgmtProvider }
