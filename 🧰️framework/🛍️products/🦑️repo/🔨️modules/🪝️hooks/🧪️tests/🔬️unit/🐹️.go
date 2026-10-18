// 🔬️ Tests of the hooks domain, split out of the pre-split godfile suite.

package hooks

import (
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

func findTestRepoRoot(start string) string {
	for _, candidate := range []string{start, func() string {
		_, file, _, ok := runtime.Caller(0)
		if !ok {
			return ""
		}
		return filepath.Dir(file)
	}()} {
		dir := candidate
		if dir == "" {
			continue
		}
		for {
			if _, err := os.Stat(filepath.Join(dir, "repo", "client", "main.go")); err == nil {
				return dir
			}
			if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
				return dir
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break
			}
			dir = parent
		}
	}
	return start
}

func TestHookEventKind(t *testing.T) {
	cases := []struct {
		name   string
		event  model.HookEvent
		expect model.HookKind
	}{
		{"version checkpoint starting is version", model.HookVersionCheckpointStarting, model.HookKindVersion},
		{"version checkpoint ended is version", model.HookVersionCheckpointEnded, model.HookKindVersion},
		{"version checkin starting is version", model.HookVersionCheckinStarting, model.HookKindVersion},
		{"version checkin ended is version", model.HookVersionCheckinEnded, model.HookKindVersion},
		{"version checkout starting is version", model.HookVersionCheckoutStarting, model.HookKindVersion},
		{"version checkout ended is version", model.HookVersionCheckoutEnded, model.HookKindVersion},
		{"agent starting is agent", model.HookAgentStarted, model.HookKindAgent},
		{"agent ended is agent", model.HookAgentEnded, model.HookKindAgent},
		{"agent prompt submitting is agent", model.HookAgentPromptSubmitting, model.HookKindAgent},
		{"agent compacting is agent", model.HookAgentCompacting, model.HookKindAgent},
		{"agent tool starting is agent", model.HookAgentToolStarting, model.HookKindAgent},
		{"agent tool ended is agent", model.HookAgentToolEnded, model.HookKindAgent},
		{"agent tool plan updating starting is agent", model.HookAgentToolPlanUpdatingStarting, model.HookKindAgent},
		{"agent tool plan updating ended is agent", model.HookAgentToolPlanUpdatingEnded, model.HookKindAgent},
		{"agent tool code searching is agent", model.HookAgentToolSearchStarting, model.HookKindAgent},
		{"agent tool searched is agent", model.HookAgentToolSearchEnded, model.HookKindAgent},
		{"agent tool code editing is agent", model.HookAgentToolCodeEditStarting, model.HookKindAgent},
		{"agent tool code edited is agent", model.HookAgentToolCodeEditEnded, model.HookKindAgent},
		{"agent tool terminal starting is agent", model.HookAgentToolTerminalStarting, model.HookKindAgent},
		{"agent tool terminal ended is agent", model.HookAgentToolTerminalEnded, model.HookKindAgent},
		{"agent thinking starting is agent", model.HookAgentThinkingStarting, model.HookKindAgent},
		{"agent thinking ended is agent", model.HookAgentThinkingEnded, model.HookKindAgent},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := HookEventKind(tc.event)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func shellCommand(t *testing.T, scriptPath string) *exec.Cmd {
	t.Helper()
	if runtime.GOOS != "windows" {
		return exec.Command(scriptPath)
	}
	shell, err := exec.LookPath("sh")
	if err != nil {
		t.Skipf("no POSIX shell on PATH to run %s: %v", scriptPath, err)
	}
	return exec.Command(shell, filepath.ToSlash(scriptPath))
}

func TestMicroCommitPostCommitHookResetsTemplates(t *testing.T) {
	repoRoot := t.TempDir()
	sourceRepoRoot := findTestRepoRoot(".")
	initGit := exec.Command("git", "init")
	initGit.Dir = repoRoot
	if out, err := initGit.CombinedOutput(); err != nil {
		t.Fatalf("git init: %v\n%s", err, out)
	}
	gitDir := filepath.Join(repoRoot, ".git")
	hooksDir := filepath.Join(gitDir, "hooks")
	if err := os.MkdirAll(hooksDir, 0o755); err != nil {
		t.Fatalf("mkdir hooks: %v", err)
	}
	hookSourcesDir := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks")
	if err := os.MkdirAll(hookSourcesDir, 0o755); err != nil {
		t.Fatalf("mkdir repo hooks: %v", err)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		hookSrc := filepath.Join(sourceRepoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks", hookName)
		data, err := os.ReadFile(hookSrc)
		if err != nil {
			t.Fatalf("read hook source %s: %v", hookName, err)
		}
		if err := os.WriteFile(filepath.Join(hookSourcesDir, hookName), data, 0o755); err != nil {
			t.Fatalf("write hook source %s: %v", hookName, err)
		}
	}
	if err := InstallMicroCommitHooks(repoRoot); err != nil {
		t.Fatalf("install hooks: %v", err)
	}
	templatePath := filepath.Join(gitDir, "gkcommittemplate.txt")
	editMsgPath := filepath.Join(gitDir, "COMMIT_EDITMSG")
	for _, p := range []string{templatePath, editMsgPath} {
		if err := os.WriteFile(p, []byte("draft\n"), 0o644); err != nil {
			t.Fatalf("seed %s: %v", p, err)
		}
	}
	hookPath := filepath.Join(hooksDir, "post-commit")
	cmd := shellCommand(t, hookPath)
	cmd.Dir = repoRoot
	cmd.Env = append(os.Environ(), "GIT_DIR="+gitDir, "GIT_WORK_TREE="+repoRoot)
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("run post-commit: %v\n%s", err, out)
	}
	b, err := os.ReadFile(editMsgPath)
	if err != nil {
		t.Fatalf("read %s: %v", editMsgPath, err)
	}
	if len(b) != 0 {
		t.Fatalf("expected empty %s after post-commit, got %q", editMsgPath, b)
	}
	tplCfg := exec.Command("git", "config", "--local", "--get", "commit.template")
	tplCfg.Dir = repoRoot
	tplOut, err := tplCfg.Output()
	if err != nil {
		t.Fatalf("expected commit.template set to empty GK file after post-commit: %v", err)
	}
	if !strings.Contains(string(tplOut), "gkcommittemplate.txt") {
		t.Fatalf("expected commit.template to point at gkcommittemplate.txt, got %q", tplOut)
	}
	legacy, err := os.ReadFile(templatePath)
	if err != nil {
		t.Fatalf("read legacy GK template: %v", err)
	}
	if len(legacy) != 0 {
		t.Fatalf("expected empty gkcommittemplate.txt after post-commit, got %q", legacy)
	}
}
