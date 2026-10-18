// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook event validation and blocked tool/command detection.

// #endregion 🧲️Header

package client

import (
	"strings"
	"testing"
	"time"
)

// 📡️#region 🗂️Hook
func TestValidateHookEvent(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		valid  bool
		expect HookEvent
	}{
		{"version checkpoint starting", "version.checkpoint.starting", true, HookVersionCheckpointStarting},
		{"version checkpoint ended", "version.checkpoint.ended", true, HookVersionCheckpointEnded},
		{"version checkin starting", "version.checkin.starting", true, HookVersionCheckinStarting},
		{"version checkin ended", "version.checkin.ended", true, HookVersionCheckinEnded},
		{"version checkout starting", "version.checkout.starting", true, HookVersionCheckoutStarting},
		{"version checkout ended", "version.checkout.ended", true, HookVersionCheckoutEnded},
		{"agent starting", "agent.started", true, HookAgentStarted},
		{"agent ended", "agent.ended", true, HookAgentEnded},
		{"agent prompt submitting", "agent.prompt.submitting", true, HookAgentPromptSubmitting},
		{"agent compacting", "agent.compacting", true, HookAgentCompacting},
		{"agent tool starting", "agent.tool.starting", true, HookAgentToolStarting},
		{"agent tool ended", "agent.tool.ended", true, HookAgentToolEnded},
		{"agent tool plan updating starting", "agent.tool.plan.updating.starting", true, HookAgentToolPlanUpdatingStarting},
		{"agent tool plan updating ended", "agent.tool.plan.updating.ended", true, HookAgentToolPlanUpdatingEnded},
		{"agent tool code searching", "agent.file.read.starting", true, HookAgentToolSearchStarting},
		{"agent tool searched", "agent.file.read.ended", true, HookAgentToolSearchEnded},
		{"agent tool code editing", "agent.tool.code.edit.starting", true, HookAgentToolCodeEditStarting},
		{"agent tool code edited", "agent.tool.code.edit.ended", true, HookAgentToolCodeEditEnded},
		{"agent tool terminal starting", "agent.tool.terminal.starting", true, HookAgentToolTerminalStarting},
		{"agent tool terminal ended", "agent.tool.terminal.ended", true, HookAgentToolTerminalEnded},
		{"agent thinking starting", "agent.thinking.starting", true, HookAgentThinkingStarting},
		{"agent thinking ended", "agent.thinking.ended", true, HookAgentThinkingEnded},
		{"invalid", "invalid.event", false, ""},
		{"empty", "", false, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, err := ValidateHookEvent(tc.input)
			if tc.valid {
				if err != nil {
					t.Fatalf("expected valid event, got error: %v", err)
				}
				if event != tc.expect {
					t.Errorf("expected %s, got %s", tc.expect, event)
				}
			} else {
				if err == nil {
					t.Fatal("expected error for invalid event")
				}
			}
		})
	}
}

func TestHookEventKind(t *testing.T) {
	cases := []struct {
		name   string
		event  HookEvent
		expect HookKind
	}{
		{"version checkpoint starting is version", HookVersionCheckpointStarting, HookKindVersion},
		{"version checkpoint ended is version", HookVersionCheckpointEnded, HookKindVersion},
		{"version checkin starting is version", HookVersionCheckinStarting, HookKindVersion},
		{"version checkin ended is version", HookVersionCheckinEnded, HookKindVersion},
		{"version checkout starting is version", HookVersionCheckoutStarting, HookKindVersion},
		{"version checkout ended is version", HookVersionCheckoutEnded, HookKindVersion},
		{"agent starting is agent", HookAgentStarted, HookKindAgent},
		{"agent ended is agent", HookAgentEnded, HookKindAgent},
		{"agent prompt submitting is agent", HookAgentPromptSubmitting, HookKindAgent},
		{"agent compacting is agent", HookAgentCompacting, HookKindAgent},
		{"agent tool starting is agent", HookAgentToolStarting, HookKindAgent},
		{"agent tool ended is agent", HookAgentToolEnded, HookKindAgent},
		{"agent tool plan updating starting is agent", HookAgentToolPlanUpdatingStarting, HookKindAgent},
		{"agent tool plan updating ended is agent", HookAgentToolPlanUpdatingEnded, HookKindAgent},
		{"agent tool code searching is agent", HookAgentToolSearchStarting, HookKindAgent},
		{"agent tool searched is agent", HookAgentToolSearchEnded, HookKindAgent},
		{"agent tool code editing is agent", HookAgentToolCodeEditStarting, HookKindAgent},
		{"agent tool code edited is agent", HookAgentToolCodeEditEnded, HookKindAgent},
		{"agent tool terminal starting is agent", HookAgentToolTerminalStarting, HookKindAgent},
		{"agent tool terminal ended is agent", HookAgentToolTerminalEnded, HookKindAgent},
		{"agent thinking starting is agent", HookAgentThinkingStarting, HookKindAgent},
		{"agent thinking ended is agent", HookAgentThinkingEnded, HookKindAgent},
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

func TestSplitCommandSegments(t *testing.T) {
	cases := []struct {
		cmd      string
		expected []string
	}{
		{"git checkout main", []string{"git checkout main"}},
		{"cd /tmp && git checkout main", []string{"cd /tmp", "git checkout main"}},
		{"echo done; git stash", []string{"echo done", "git stash"}},
		{"ls | grep foo", []string{"ls", "grep foo"}},
		{"a || b", []string{"a", "b"}},
		{"  ", []string{}},
		{"", []string{}},
	}
	for _, tc := range cases {
		t.Run(tc.cmd, func(t *testing.T) {
			got := splitCommandSegments(tc.cmd)
			if len(got) != len(tc.expected) {
				t.Fatalf("expected %v, got %v", tc.expected, got)
			}
			for i := range tc.expected {
				if got[i] != tc.expected[i] {
					t.Errorf("segment %d: expected %q, got %q", i, tc.expected[i], got[i])
				}
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

func TestRunHookAgentEvents(t *testing.T) {
	cases := []struct {
		name    string
		event   HookEvent
		allowed bool
	}{
		{"agent starting", HookAgentStarted, true},
		{"agent ended", HookAgentEnded, true},
		{"agent prompt submitting", HookAgentPromptSubmitting, true},
		{"agent compacting", HookAgentCompacting, true},
		{"agent tool ended", HookAgentToolEnded, true},
		{"agent tool code searching", HookAgentToolSearchStarting, true},
		{"agent tool searched", HookAgentToolSearchEnded, true},
		{"agent tool code editing", HookAgentToolCodeEditStarting, true},
		{"agent tool code edited", HookAgentToolCodeEditEnded, true},
		{"agent tool plan updating starting", HookAgentToolPlanUpdatingStarting, true},
		{"agent tool plan updating ended", HookAgentToolPlanUpdatingEnded, true},
		{"agent tool terminal starting", HookAgentToolTerminalStarting, true},
		{"agent tool terminal ended", HookAgentToolTerminalEnded, true},
		{"agent thinking starting", HookAgentThinkingStarting, true},
		{"agent thinking ended", HookAgentThinkingEnded, true},
		{"version checkpoint ended", HookVersionCheckpointEnded, true},
		{"version checkin starting", HookVersionCheckinStarting, true},
		{"version checkin ended", HookVersionCheckinEnded, true},
		{"version checkout starting", HookVersionCheckoutStarting, true},
		{"version checkout ended", HookVersionCheckoutEnded, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "copilot-chat",
				Second:   time.Now().UTC().Format(time.RFC3339),
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if result.IsAllowed() != tc.allowed {
				t.Errorf("expected allowed=%v, got allowed=%v: %s", tc.allowed, result.IsAllowed(), result.GetMessage())
			}
		})
	}
	t.Run("version checkpoint starting", func(t *testing.T) {
		hctx := HookContext{
			Event:    HookVersionCheckpointStarting,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)

		_, ok := result.(HookResultVersionCheckpointStarting)
		if !ok {
			t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
		}
	})
}

func TestRunHookToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentToolStarting,
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
	hctx := HookContext{
		Event:    HookAgentToolStarting,
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

func TestRunHookUnknownEvent(t *testing.T) {
	hctx := HookContext{
		Event:    HookEvent("unknown.event"),
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected unknown event to be denied")
	}
}
