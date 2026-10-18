// 🛡️ Tests of the tool and command blocking policy.

package hooks

import (
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
)

func TestIsToolBlocked(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		toolArgs string
		blocked  bool
	}{
		{"git checkout blocked", "run_in_terminal", "git checkout main", true},
		{"git add blocked", "run_in_terminal", "git add .", true},
		{"git branch blocked", "run_in_terminal", "git branch feature/x", true},
		{"git cherry-pick blocked", "run_in_terminal", "git cherry-pick abc123", true},
		{"git clone blocked", "run_in_terminal", "git clone https://example.com/repo.git", true},
		{"git commit blocked", "run_in_terminal", "git commit -m test", true},
		{"git config blocked", "run_in_terminal", "git config user.name compose", true},
		{"git fetch blocked", "run_in_terminal", "git fetch origin", true},
		{"git init blocked", "run_in_terminal", "git init", true},
		{"git merge blocked", "run_in_terminal", "git merge main", true},
		{"git mv blocked", "run_in_terminal", "git mv a b", true},
		{"git pull blocked", "run_in_terminal", "git pull origin main", true},
		{"git push blocked", "run_in_terminal", "git push origin main", true},
		{"git rebase blocked", "run_in_terminal", "git rebase main", true},
		{"git remote blocked", "run_in_terminal", "git remote add origin https://example.com/repo.git", true},
		{"git reset blocked", "run_in_terminal", "git reset --hard", true},
		{"git restore blocked", "run_in_terminal", "git restore .", true},
		{"git revert blocked", "run_in_terminal", "git revert abc123", true},
		{"git rm blocked", "run_in_terminal", "git rm file.txt", true},
		{"git stash blocked", "bash", "git stash", true},
		{"git stash pop blocked", "shell", "git stash pop", true},
		{"git stash drop blocked", "terminal", "git stash drop", true},
		{"git stash apply blocked", "run", "git stash apply", true},
		{"git switch blocked", "run_in_terminal", "git switch main", true},
		{"git tag blocked", "run_in_terminal", "git tag v1.2.3", true},
		{"git clean fd blocked", "terminal", "git clean -fd", true},
		{"git with global option blocked", "terminal", "git -C /workspaces/semio stash", true},
		{"git with config option blocked", "terminal", "git -c core.hooksPath=/tmp commit -m test", true},
		{"env wrapped git blocked", "terminal", "GIT_TRACE=1 git stash push", true},
		{"env command git blocked", "terminal", "env GIT_TRACE=1 git checkout main", true},
		{"command wrapped git blocked", "terminal", "command git switch main", true},
		{"sudo wrapped git blocked", "terminal", "sudo git reset --hard", true},
		{"absolute git path blocked", "terminal", "/usr/bin/git pull origin main", true},
		{"shell wrapped git blocked", "terminal", `bash -lc "git stash && echo done"`, true},
		{"git checkout in args blocked", "", "git checkout feature/branch", true},
		{"regular tool allowed", "read_file", "/path/to/file.ts", false},
		{"git status allowed", "terminal", "git status", false},
		{"git log allowed", "terminal", "git log --oneline -n 5", false},
		{"git diff allowed", "terminal", "git diff", false},
		{"git rev-parse allowed", "terminal", "git rev-parse HEAD", false},
		{"shell wrapped git status allowed", "terminal", `bash -lc "git status"`, false},
		{"empty allowed", "", "", false},
		{"case insensitive", "TERMINAL", "GIT CHECKOUT main", true},
		{"grep with git checkout pattern not blocked", "", `grep "git checkout" file.go`, false},
		{"echo with git stash not blocked", "", `echo "git stash"`, false},
		{"compose cli command not blocked", "", `go run ./repo/client/mcp/go tree "hooks events inlet adapter cli"`, false},
		{"cd then git checkout blocked", "", "cd /workspaces && git checkout feature", true},
		{"pipe grep allowed", "", `ls | grep "git checkout"`, false},
		{"git checkout after semicolon blocked", "", "echo done; git checkout main", true},
		{"git restore after semicolon blocked", "", "echo done; git restore --staged .", true},
		{"grep for git reset not blocked", "bash", `grep -rn "git reset --hard" .`, false},
		{"python subprocess git stash blocked", "run_in_terminal", `python3 -c "import subprocess; subprocess.run(['git', 'stash'])"`, true},
		{"python os.system git checkout blocked", "run_in_terminal", `python3 -c "import os; os.system('git checkout main')"`, true},
		{"node exec git stash blocked", "run_in_terminal", `node -e "require('child_process').exec('git stash')"`, true},
		{"perl system git checkout blocked", "run_in_terminal", `perl -e "system('git checkout main')"`, true},
		{"ruby system git stash blocked", "run_in_terminal", `ruby -e "system('git stash')"`, true},
		{"fish git stash blocked", "run_in_terminal", `fish -c "git stash"`, true},
		{"ksh git reset blocked", "run_in_terminal", `ksh -c "git reset --hard"`, true},
		{"xargs git stash blocked", "run_in_terminal", "xargs git stash", true},
		{"python git status allowed", "run_in_terminal", `python3 -c "import subprocess; subprocess.call(['git', 'status'])"`, false},
		{"kill lsof port blocked", "terminal", "kill $(lsof -t -i:9876)", true},
		{"kill -9 lsof port blocked", "terminal", "kill -9 $(lsof -t -i:3000)", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			blocked, _ := IsToolBlocked(tc.toolName, tc.toolArgs)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v, got blocked=%v", tc.blocked, blocked)
			}
		})
	}
}

func TestIsCommandSegmentBlocked(t *testing.T) {
	cases := []struct {
		segment string
		blocked bool
	}{
		{"git add .", true},
		{"git branch feature/x", true},
		{"git checkout main", true},
		{"git cherry-pick abc123", true},
		{"git clone https://example.com/repo.git", true},
		{"git commit -m msg", true},
		{"git config user.email dev@example.com", true},
		{"git fetch origin", true},
		{"git init", true},
		{"git merge main", true},
		{"git mv a b", true},
		{"git pull origin main", true},
		{"git push origin main", true},
		{"git rebase main", true},
		{"git remote add origin https://example.com/repo.git", true},
		{"git restore --staged .", true},
		{"git revert abc123", true},
		{"git rm file.txt", true},
		{"git stash", true},
		{"git reset --hard", true},
		{"git switch feature/x", true},
		{"git tag v1.2.3", true},
		{"git clean -fd", true},
		{"git -C /tmp stash", true},
		{"git -c core.editor=true commit -m msg", true},
		{"GIT_TRACE=1 git stash", true},
		{"env GIT_TRACE=1 git checkout main", true},
		{"command git switch main", true},
		{"sudo /usr/bin/git reset --hard", true},
		{`bash -lc "git stash && echo done"`, true},
		{`grep "git checkout" file.go`, false},
		{`echo "git stash"`, false},
		{"go run ./repo/client/mcp/go tree hooks", false},
		{"git status", false},
		{"git log --oneline -n 5", false},
		{"git diff", false},
		{"git rev-parse HEAD", false},
		{`bash -lc "git status"`, false},
		{"GIT CHECKOUT branch", true},
		{"", false},
		// Script interpreters with inline git commands.
		{`python -c "import subprocess; subprocess.run(['git', 'stash'])"`, true},
		{`python3 -c "import os; os.system('git checkout main')"`, true},
		{`python3 -c "import subprocess; subprocess.call(['git', 'status'])"`, false},
		{`node -e "require('child_process').exec('git stash')"`, true},
		{`node -e "require('child_process').exec('git status')"`, false},
		{`perl -e "system('git checkout main')"`, true},
		{`ruby -e "system('git stash')"`, true},
		{`ruby -e "system('git status')"`, false},
		// Additional shells.
		{`fish -c "git stash"`, true},
		{`ksh -c "git reset --hard"`, true},
		{`dash -c "git checkout main"`, true},
		// xargs forwarding git.
		{"xargs git stash", true},
		{"xargs git checkout", true},
		{"xargs git status", false},
		// kill+lsof port killing is denied (can terminate devcontainer).
		{"kill $(lsof -t -i:9876)", true},
		{"kill -9 $(lsof -t -i:9876)", true},
	}
	for _, tc := range cases {
		t.Run(tc.segment, func(t *testing.T) {
			blocked, _ := isCommandSegmentBlocked(tc.segment)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v for %q", tc.blocked, tc.segment)
			}
		})
	}
}

func TestRunHookToolBlocking(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "run_in_terminal",
		ToolArgs: "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestRunHookToolAllowed(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "cursor-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "read_file",
		ToolArgs: "/workspaces/semio/repo/client/main.go",
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Errorf("expected tool to be allowed, got: %s", result.GetMessage())
	}
}

func TestHookCommandToolBlocking(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "terminal",
		ToolArgs: "git stash pop",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "git stash") {
		t.Errorf("expected git stash in message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "other developers and agents may be editing the same files concurrently") {
		t.Errorf("expected concurrent edit warning in message, got: %s", result.GetMessage())
	}
}

func TestBlockedToolPatterns(t *testing.T) {
	if len(BlockedToolPatterns) < 20 {
		t.Errorf("expected at least 20 blocked patterns, got %d", len(BlockedToolPatterns))
	}
	expectedPatterns := []string{
		"git add",
		"git branch",
		"git checkout",
		"git cherry-pick",
		"git clean",
		"git clone",
		"git commit",
		"git config",
		"git fetch",
		"git init",
		"git merge",
		"git mv",
		"git pull",
		"git push",
		"git rebase",
		"git remote",
		"git reset",
		"git restore",
		"git revert",
		"git rm",
		"git stash",
		"git switch",
		"git tag",
	}
	for _, ep := range expectedPatterns {
		found := false
		for bp := range BlockedToolPatterns {
			if bp == ep {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing blocked pattern: %s", ep)
		}
	}
}
