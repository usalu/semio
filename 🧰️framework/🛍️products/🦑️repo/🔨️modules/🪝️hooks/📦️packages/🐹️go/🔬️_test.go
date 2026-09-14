// 🔬️ Tests of the hooks domain, split out of the pre-split godfile suite.

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
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

func TestExtractSessionIDFromInput(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", ``, ""},
		{"invalid json", `{invalid}`, ""},
		{"trajectory_id", `{"trajectory_id": "test-1"}`, "test-1"},
		{"trajectoryId", `{"trajectoryId": "test-2"}`, "test-2"},
		{"sessionId", `{"sessionId": "test-3"}`, "test-3"},
		{"session_id", `{"session_id": "test-4"}`, "test-4"},
		{"conversationId", `{"conversationId": "test-5"}`, "test-5"},
		{"conversation_id", `{"conversation_id": "test-6"}`, "test-6"},
		{"agent_id", `{"agent_id": "test-7"}`, "test-7"},
		{"agentId", `{"agentId": "test-8"}`, "test-8"},
		{"nested conversation id", `{"native":{"event":{"conversation_id":"test-9"}}}`, "test-9"},
		{"transcript basename fallback", `{"native":{"event":{"transcript_path":"/tmp/transcripts/test-10.jsonl"}}}`, "test-10"},
		{"whitespace", `{"sessionId": " test-9 "}`, "test-9"},
		{"missing", `{"other": "value"}`, ""},
		{"wrong type", `{"sessionId": 123}`, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ExtractSessionIDFromInput(json.RawMessage(tt.input))
			if result != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, result)
			}
		})
	}
}

func TestExtractTranscriptAndLLMFromNestedInput(t *testing.T) {
	input := json.RawMessage(`{
		"native": {
			"event": {
				"model": "composer-1.5",
				"transcript_path": "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl"
			}
		}
	}`)

	if got := ExtractTranscriptFromInput(input); got != "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl" {
		t.Fatalf("expected nested transcript path, got %q", got)
	}

	if got := extractLLMFromInput(input); got != "composer-1.5" {
		t.Fatalf("expected nested model, got %q", got)
	}
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

func TestRunHookAgentEvents(t *testing.T) {
	cases := []struct {
		name    string
		event   model.HookEvent
		allowed bool
	}{
		{"agent starting", model.HookAgentStarted, true},
		{"agent ended", model.HookAgentEnded, true},
		{"agent prompt submitting", model.HookAgentPromptSubmitting, true},
		{"agent compacting", model.HookAgentCompacting, true},
		{"agent tool ended", model.HookAgentToolEnded, true},
		{"agent tool code searching", model.HookAgentToolSearchStarting, true},
		{"agent tool searched", model.HookAgentToolSearchEnded, true},
		{"agent tool code editing", model.HookAgentToolCodeEditStarting, true},
		{"agent tool code edited", model.HookAgentToolCodeEditEnded, true},
		{"agent tool plan updating starting", model.HookAgentToolPlanUpdatingStarting, true},
		{"agent tool plan updating ended", model.HookAgentToolPlanUpdatingEnded, true},
		{"agent tool terminal starting", model.HookAgentToolTerminalStarting, true},
		{"agent tool terminal ended", model.HookAgentToolTerminalEnded, true},
		{"agent thinking starting", model.HookAgentThinkingStarting, true},
		{"agent thinking ended", model.HookAgentThinkingEnded, true},
		{"version checkpoint ended", model.HookVersionCheckpointEnded, true},
		{"version checkin starting", model.HookVersionCheckinStarting, true},
		{"version checkin ended", model.HookVersionCheckinEnded, true},
		{"version checkout starting", model.HookVersionCheckoutStarting, true},
		{"version checkout ended", model.HookVersionCheckoutEnded, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := model.HookContext{
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
		hctx := model.HookContext{
			Event:    model.HookVersionCheckpointStarting,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)

		_, ok := result.(model.HookResultVersionCheckpointStarting)
		if !ok {
			t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
		}
	})
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

func TestRunHookUnknownEvent(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookEvent("unknown.event"),
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected unknown event to be denied")
	}
}

func TestRunHookThinkingEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow run hook thinking events test in short mode")
	}
	t.Run("thinking ended extracts text from top-level", func(t *testing.T) {
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think","text":"Planning repo modifications"}`),
		}
		result := RunHook(hctx)
		res, ok := result.(model.HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Planning repo modifications" {
			t.Errorf("expected thinking text in message, got %q", res.GetMessage())
		}
		if !res.IsAllowed() {
			t.Error("expected thinking ended to be allowed")
		}
	})
	t.Run("thinking ended extracts text from native.event", func(t *testing.T) {
		input := `{"native":{"event":{"text":"Thinking deeply","hook_event_name":"afterAgentThought"}},"session_id":"sess-think2"}`
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(input),
		}
		result := RunHook(hctx)
		res, ok := result.(model.HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Thinking deeply" {
			t.Errorf("expected thinking text from native.event, got %q", res.GetMessage())
		}
	})
	t.Run("thinking starting is allowed", func(t *testing.T) {
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingStarting,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think3","text":"About to think"}`),
		}
		result := RunHook(hctx)
		if _, ok := result.(model.HookResultAgentThinkingStarting); !ok {
			t.Fatalf("expected HookResultAgentThinkingStarting, got %T", result)
		}
		if !result.IsAllowed() {
			t.Error("expected thinking starting to be allowed")
		}
	})
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

func TestHookCommandJSONOutput(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	out, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON marshaling, got: %v", err)
	}
	var parsed model.HookResultAgentStarted
	if err := json.Unmarshal(out, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	if !parsed.Allowed {
		t.Error("expected allowed=true")
	}
	if parsed.Second == "" {
		t.Error("expected non-empty second")
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

func TestExtractToolNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"vscode tool_name", `{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"ls"}}`, "run_in_terminal"},
		{"claude code tool_name", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "Bash"},
		{"no tool_name", `{"tool_input":{"command":"ls"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractToolNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractHookEventNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"PreToolUse", `{"hookEventName":"PreToolUse","tool_name":"editFiles"}`, "PreToolUse"},
		{"PostToolUse", `{"hookEventName":"PostToolUse","tool_name":"editFiles"}`, "PostToolUse"},
		{"SessionStart", `{"hookEventName":"SessionStart","source":"new"}`, "SessionStart"},
		{"no hookEventName", `{"tool_name":"Bash"}`, ""},
		{"empty", `{}`, ""},
		{"invalid json", `bad`, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractHookEventNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestHookCommandCopilotChatVSCodeOutput(t *testing.T) {
	t.Run("PreToolUse allow produces VS Code JSON", func(t *testing.T) {
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "read_file",
			ToolArgs: "/tmp/file.ts",
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if !result.IsAllowed() {
			t.Fatal("expected allowed")
		}
		out := providers.FormatVSCodeHookOutput("PreToolUse", result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected allow, got %v", hso["permissionDecision"])
		}
	})
	t.Run("PreToolUse blocked produces VS Code deny JSON", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git checkout main"}}`)
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "run_in_terminal",
			Input:    payload,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if result.IsAllowed() {
			t.Fatal("expected blocked")
		}
		hookEventName := extractHookEventNameFromStdin(payload)
		out := providers.FormatVSCodeHookOutput(hookEventName, result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected deny, got %v", hso["permissionDecision"])
		}
		reason, _ := hso["permissionDecisionReason"].(string)
		if !strings.Contains(reason, "blocked") {
			t.Errorf("expected blocked reason, got: %s", reason)
		}
	})
	t.Run("tool_name extracted from stdin", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git stash"}}`)
		toolName := extractToolNameFromStdin(payload)
		if toolName != "run_in_terminal" {
			t.Errorf("expected run_in_terminal, got %s", toolName)
		}
		cmd := extractCommandFromStdin(payload)
		if cmd != "git stash" {
			t.Errorf("expected git stash, got %s", cmd)
		}
	})
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

func testLoggingConfigFull() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func testLoggingConfigSession() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "standard"}
}

func writeRepoLoggingConfig(t *testing.T, root string, lg workspace.LoggingConfig) {
	t.Helper()
	repoDir := filepath.Join(root, ".🧬semio", "🦑️repo")
	if err := os.MkdirAll(repoDir, 0o755); err != nil {
		t.Fatalf("mkdir .🦑️repo: %v", err)
	}
	content := fmt.Sprintf("[logging]\nsession = %t\noperations = %t\nplan = %t\ndetail = \"%s\"\n",
		lg.Session, lg.Operations, lg.Plan, lg.Detail)
	if err := os.WriteFile(filepath.Join(repoDir, workspace.ConfigFileName), []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", workspace.ConfigFileName, err)
	}
}

func TestRepoConfig(t *testing.T) {
	t.Run("defaults when file missing", func(t *testing.T) {
		cfg := workspace.LoadRepoConfig(t.TempDir())
		if cfg.Logging.Session {
			t.Error("expected session logging off by default")
		}
		if !cfg.Logging.Operations || !cfg.Logging.Plan {
			t.Error("expected operations and plan enabled by default")
		}
		if cfg.Logging.Detail != "standard" {
			t.Errorf("expected detail standard, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("parse logging table", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{
			Session: true, Operations: false, Plan: false, Detail: "minimal",
		})
		cfg := workspace.LoadRepoConfig(tmpDir)
		if !cfg.Logging.Session || cfg.Logging.Operations || cfg.Logging.Plan {
			t.Errorf("unexpected logging config: %+v", cfg.Logging)
		}
		if cfg.Logging.Detail != "minimal" {
			t.Errorf("expected detail minimal, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("session off by default", func(t *testing.T) {
		tmpDir := t.TempDir()
		RunHook(model.HookContext{
			Event:    model.HookAgentStarted,
			Client:   "claude-code",
			Second:   time.Now().UTC().Format(time.RFC3339),
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"off-by-default"}`),
		})
		assertNoHookLogFiles(t, tmpDir)
	})
	t.Run("detail levels", func(t *testing.T) {
		tmpDir := t.TempDir()
		logDir := SessionLogDir(tmpDir, 2026, 5, 30, "detail-sess")
		if err := os.MkdirAll(logDir, 0o755); err != nil {
			t.Fatal(err)
		}
		hctx := model.HookContext{Event: model.HookAgentStarted, Client: "claude-code", Input: json.RawMessage(`{"x":1}`)}
		result := model.HookResultBase{Allowed: true}
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Detail: "minimal"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", workspace.LoadRepoConfig(tmpDir).Logging, InertEnvironment{})
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 event, got %d", len(meta.Events))
		}
		if meta.Events[0].Native != nil {
			t.Error("minimal must not include native")
		}
		if meta.Events[0].Response != nil {
			t.Error("minimal must not include response")
		}
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Detail: "full"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", workspace.LoadRepoConfig(tmpDir).Logging, InertEnvironment{})
		data, _ = os.ReadFile(filepath.Join(logDir, "session.json"))
		json.Unmarshal(data, &meta)
		if len(meta.Events) < 2 {
			t.Fatalf("expected 2 events after full detail append")
		}
		last := meta.Events[len(meta.Events)-1]
		if last.Native == nil || last.Response == nil {
			t.Error("full detail must include native and response")
		}
	})
	t.Run("plan off", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Plan: false, Detail: "standard"})
		RunHook(model.HookContext{
			Event:    model.HookAgentToolPlanUpdatingEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"plan-off","tool_input":{"todoList":[{"title":"A","status":"pending"}]}}`),
		})
		logFiles := getLogFiles(t, tmpDir)
		if len(logFiles) != 1 {
			t.Fatalf("expected session.json, got %v", logFiles)
		}
		var meta model.SessionMeta
		data, _ := os.ReadFile(logFiles[0])
		json.Unmarshal(data, &meta)
		if meta.Plan != nil {
			t.Error("expected no plan when plan=false")
		}
	})
}

func TestHookLogging(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess-log")
	payload := json.RawMessage(`{"session_id":"sess-log","second":"2026-02-20T10:00:00Z","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-02-20T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Fatalf("expected allowed=true, got: %s", result.GetMessage())
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json for session events, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json to be the only file, got %s", entries[0].Name())
	}
	metaBytes, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("expected session.json to exist, got error: %v", err)
	}
	var meta model.SessionMeta
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("expected valid JSON in session.json, got: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected exactly 1 session event, got %d", len(meta.Events))
	}
	if meta.ID == "" {
		t.Errorf("expected id in session.json, got: %v", meta.ID)
	}
	if meta.URI == "" {
		t.Errorf("expected uri in session.json, got: %v", meta.URI)
	}
	if meta.Contributor == "" {
		t.Errorf("expected contributor in session.json, got: %v", meta.Contributor)
	}
	if meta.Client != "claude-code" {
		t.Errorf("expected client claude-code in session.json, got: %v", meta.Client)
	}
	if meta.Second == "" {
		t.Errorf("expected second in session.json, got: %v", meta.Second)
	}
	if meta.Transcript != "/tmp/transcript.jsonl" {
		t.Errorf("expected transcript in session.json, got: %v", meta.Transcript)
	}
	entry := meta.Events[0]
	if entry.Native.Event == nil {
		t.Error("expected native.event to be populated")
	}
	var evt map[string]interface{}
	if err := json.Unmarshal(entry.Event, &evt); err != nil {
		t.Fatalf("cannot unmarshal event: %v", err)
	}
	if evt["kind"] != string(model.HookAgentStarted) {
		t.Errorf("expected event.kind agent.started in log, got: %v", evt["kind"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code in log, got: %v", evt["client"])
	}
	if _, ok := evt["contributor"]; !ok {
		t.Error("expected event.contributor to be present in log entry")
	}
	expectedSession := ResolveEventSessionID("sess-log")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s in log, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := ResolveEventSecondID("2026-02-20T10:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/transcript.jsonl" {
		t.Errorf("expected event.transcript in log, got: %v", evt["transcript"])
	}
	if entry.Response.Blocked != nil {
		t.Error("expected response.blocked to be nil for allowed event")
	}
	if len(meta.Events) > 0 && len(meta.Events[0].Event) == 0 {
		t.Error("expected non-empty serialized event payload in session event")
	}
	for _, forbidden := range []string{"uuid", "started_at", "first_event"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent in session.json", forbidden)
		}
	}
	for _, forbidden := range []string{"native", "event", "response"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent at root of session.json", forbidden)
		}
	}
	if len(meta.Events) == 0 {
		t.Fatalf("expected session.json to be written alongside agent events")
	}
}

func TestSessionJsonTracksPlan(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	sessionID := "plan-track-session"
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	startPayload := json.RawMessage(`{"session_id":"plan-track-session","second":"2026-03-02T10:00:00Z"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-03-02T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    startPayload,
	})

	plan1Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"in-progress"},{"title":"Step B","status":"pending"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    plan1Payload,
	})

	metaPath := filepath.Join(logDir, "session.json")
	metaBytes, err := os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after plan update, got error: %v", err)
	}
	var meta model.SessionMeta
	assertHasLifecycle := func(step model.TicketAgentPlanStep) {
		if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
			t.Errorf("expected step %q to include at least one lifecycle timestamp, got %+v", step.Name, step)
		}
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 2 {
		t.Fatalf("expected 2 plan steps, got %v", meta.Plan)
	}
	stepA := meta.Plan.Steps[0]
	if stepA.Name != "Step A" {
		t.Errorf("expected Step A, got %s", stepA.Name)
	}
	if stepA.Ideated == "" {
		t.Errorf("expected Step A ideated timestamp, got empty")
	}
	if stepA.Started == "" {
		t.Errorf("expected Step A started timestamp, got empty")
	}
	if stepA.Completed != "" {
		t.Errorf("expected Step A not completed, got %s", stepA.Completed)
	}
	stepB := meta.Plan.Steps[1]
	if stepB.Name != "Step B" {
		t.Errorf("expected Step B, got %s", stepB.Name)
	}
	if stepB.Ideated == "" {
		t.Errorf("expected Step B ideated timestamp, got empty")
	}
	if stepB.Started != "" || stepB.Completed != "" {
		t.Errorf("expected Step B pending, got started=%s completed=%s", stepB.Started, stepB.Completed)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan2Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"completed"},{"title":"Step B","status":"in-progress"},{"title":"Step C","status":"pending"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    plan2Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after second plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after second update, got %v", meta.Plan)
	}
	stepA2 := meta.Plan.Steps[0]
	if stepA2.Name != "Step A" || stepA2.Completed == "" {
		t.Errorf("expected Step A completed, got %+v", stepA2)
	}
	if stepA2.Ideated != stepA.Ideated {
		t.Errorf("expected Step A ideated timestamp preserved, got %s vs %s", stepA2.Ideated, stepA.Ideated)
	}
	if stepA2.Started != stepA.Started {
		t.Errorf("expected Step A started timestamp preserved, got %s vs %s", stepA2.Started, stepA.Started)
	}
	stepB2 := meta.Plan.Steps[1]
	if stepB2.Name != "Step B" || stepB2.Started == "" {
		t.Errorf("expected Step B in-progress, got %+v", stepB2)
	}
	stepC := meta.Plan.Steps[2]
	if stepC.Name != "Step C" || stepC.Ideated == "" || stepC.Started != "" || stepC.Completed != "" {
		t.Errorf("expected Step C pending, got %+v", stepC)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan3Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step B","status":"completed"},{"title":"Step C","status":"in-progress"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:03:00Z",
		RepoRoot: tmpDir,
		Input:    plan3Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after third plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after third update (Step A preserved as completed), got %v", meta.Plan)
	}
	stepB3 := meta.Plan.Steps[0]
	if stepB3.Name != "Step B" || stepB3.Completed == "" {
		t.Errorf("expected Step B completed, got %+v", stepB3)
	}
	if stepB3.Started != stepB2.Started {
		t.Errorf("expected Step B started timestamp preserved, got %s vs %s", stepB3.Started, stepB2.Started)
	}
	stepC2 := meta.Plan.Steps[1]
	if stepC2.Name != "Step C" || stepC2.Started == "" {
		t.Errorf("expected Step C in-progress, got %+v", stepC2)
	}
	stepACompleted := meta.Plan.Steps[2]
	if stepACompleted.Name != "Step A" || stepACompleted.Completed == "" || stepACompleted.Abandoned != "" {
		t.Errorf("expected Step A to remain completed and not abandoned, got %+v", stepACompleted)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}
}

func TestHookLoggingToolBlocked(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "unknown")
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: tmpDir,
		ToolName: "bash",
		ToolArgs: "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected blocked tool to be denied")
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}
	var meta model.SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("expected valid session.json: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if entry.Response.Blocked == nil || !*entry.Response.Blocked {
		t.Error("expected response.blocked=true for blocked tool")
	}
	if !strings.Contains(entry.Response.Reason, "blocked") {
		t.Errorf("expected blocked reason in log, got: %s", entry.Response.Reason)
	}
}

func TestHookLoggingStdinInput(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "abc123")
	payload := json.RawMessage(`{"session_id":"abc123","second":"2026-02-20T12:00:00Z","tool_name":"Bash","tool_input":{"command":"ls"},"transcript_path":"/tmp/t.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-02-20T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta model.SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if len(entry.Native.Event) == 0 {
		t.Error("expected native.event from HookContext.Input")
	}
	var wantMap, gotMap map[string]interface{}
	json.Unmarshal(payload, &wantMap)
	json.Unmarshal(entry.Native.Event, &gotMap)
	wantBytes, _ := json.Marshal(wantMap)
	gotBytes, _ := json.Marshal(gotMap)
	if string(gotBytes) != string(wantBytes) {
		t.Errorf("expected raw %s in log, got: %s", wantBytes, gotBytes)
	}
	var evt map[string]interface{}
	json.Unmarshal(entry.Event, &evt)
	expectedSession := ResolveEventSessionID("abc123")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := ResolveEventSecondID("2026-02-20T12:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/t.jsonl" {
		t.Errorf("expected event.transcript, got: %v", evt["transcript"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code, got: %v", evt["client"])
	}
}

func TestLogRepoOperationHookMCPTool(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess1")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for mcp__repo__tree", func(t *testing.T) {
		input := json.RawMessage(`{"query":"contributors"}`)
		result := model.HookResultAgentToolStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name:  "mcp__repo__tree",
			Input: input,
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"sess1"}`),
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json to be created for derived repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.tree.starting" {
			t.Errorf("expected kind agent.tree.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for mcp__repo__ticket_open", func(t *testing.T) {
		result := model.HookResultAgentToolEnded{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name: "mcp__repo__ticket_open",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra files beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.ended" {
			t.Errorf("expected kind agent.ticket.open.ended, got: %v", evt["kind"])
		}
	})

	t.Run("no extra file for non-compose tool", func(t *testing.T) {
		result := model.HookResultAgentToolStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
			},
			Name: "Bash",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Errorf("expected no new files for Bash tool, got %d new files", len(after)-len(before))
		}
	})
}

func TestLogRepoOperationHookCLI(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess2")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for CLI ticket open", func(t *testing.T) {
		result := model.HookResultAgentToolTerminalStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' claude-code sonnet-4-5",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolTerminalStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json creation for CLI repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.starting" {
			t.Errorf("expected kind agent.ticket.open.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for CLI goal close", func(t *testing.T) {
		result := model.HookResultAgentToolTerminalEnded{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "/workspace/repo/client/client goal close MY-GOAL 'Summary'",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolTerminalEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra file beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.goal.close.ended" {
			t.Errorf("expected kind agent.goal.close.ended, got: %v", evt["kind"])
		}
	})
}

func TestRunHookAgentToolStartingDerivedRepoEvents(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	sessionID := "test-repo-events-session"
	payload := json.RawMessage(`{"session_id":"test-repo-events-session","tool_name":"mcp__repo__ticket_open","tool_input":{"title":"My Ticket","prompt":"My Prompt","client":"claude-code","llm":"sonnet-4-5","goal":"MY-GOAL"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-03-05T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)

	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 || entries[0].Name() != "session.json" {
		names := make([]string, len(entries))
		for i, e := range entries {
			names[i] = e.Name()
		}
		t.Fatalf("expected only session.json, got %d: %v", len(entries), names)
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta model.SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 2 {
		t.Fatalf("expected 2 session events (agent + derived repo operation), got %d", len(meta.Events))
	}
	found := false
	for _, entry := range meta.Events {
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] == "agent.ticket.open.starting" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected derived agent.ticket.open.starting event in session.json events")
	}
}

func setupTicketDir(t *testing.T) (string, string) {
	t.Helper()
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets",
		model.FormatYearDir(now.Year()%100),
		model.FormatMonthDir(int(now.Month())),
		model.FormatDayDir(now.Day()),
		"TEST-TICKET")
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "🎫️ticket.json")
	initialTicket := `{"title":"Test Ticket","status":"open","goal":"TEST/GOAL"}`
	if err := os.WriteFile(ticketJSON, []byte(initialTicket), 0644); err != nil {
		t.Fatal(err)
	}
	importantPath := filepath.Join(ticketDir, "📌️important", "📝️.md")
	if err := os.MkdirAll(filepath.Dir(importantPath), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(importantPath, nil, 0644); err != nil {
		t.Fatal(err)
	}
	return tmpDir, ticketJSON
}

func readTicketJSON(t *testing.T, ticketJSON string) map[string]interface{} {
	t.Helper()
	data, err := os.ReadFile(ticketJSON)
	if err != nil {
		t.Fatalf("cannot read ticket.json: %v", err)
	}
	var result map[string]interface{}
	if err := json.Unmarshal(data, &result); err != nil {
		t.Fatalf("invalid ticket.json: %v\n%s", err, string(data))
	}
	return result
}

func getLogFiles(t *testing.T, tmpDir string) []string {
	t.Helper()
	outBase := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache")
	var logFiles []string
	filepath.WalkDir(outBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return nil
		}
		if !d.IsDir() && strings.HasSuffix(d.Name(), ".json") {
			logFiles = append(logFiles, path)
		}
		return nil
	})
	return logFiles
}

func assertNoHookLogFiles(t *testing.T, tmpDir string) {
	t.Helper()
	logFiles := getLogFiles(t, tmpDir)
	if len(logFiles) != 0 {
		t.Fatalf("expected no hook log files under .🦑️repo/⚡️, got %v", logFiles)
	}
}

func TestTrackHookAllEventsLogged(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow track hook all-events test in short mode")
	}
	tmpDir, ticketJSON := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	workspace.SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"test-session-1","llm":"opus-4-6","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Fix the bug",
	}
	RunHook(hctx)
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	agentEvents := []struct {
		name  string
		event model.HookEvent
		input json.RawMessage
	}{
		{"agent.started", model.HookAgentStarted, sessionInput},
		{"agent.tool.starting", model.HookAgentToolStarting, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.file.read.starting", model.HookAgentToolSearchStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"hello","includePattern":"src/**"}}`)},
		{"agent.file.read.ended", model.HookAgentToolSearchEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"world"}}`)},
		{"agent.tool.code.edit.starting", model.HookAgentToolCodeEditStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.code.edit.ended", model.HookAgentToolCodeEditEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.terminal.starting", model.HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"command":"go test ./..."}}`)},
		{"agent.tool.terminal.ended", model.HookAgentToolTerminalEnded, json.RawMessage(`{"session_id":"test-session-1","command":"go test ./...","pid":"12345","stdout":"PASS"}`)},
		{"agent.tool.test.starting", model.HookAgentToolTestStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"files":["/tmp/test.go"],"timeout":"30000"}}`)},
		{"agent.tool.test.ended", model.HookAgentToolTestEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["TestA"],"failed":["TestB"]}}`)},
		{"agent.tool.build.starting", model.HookAgentToolBuildStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"bundles":["compose/js"]}}`)},
		{"agent.tool.build.ended", model.HookAgentToolBuildEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["compose/js"]}}`)},
		{"agent.tool.ended", model.HookAgentToolEnded, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.tool.plan.updating.starting", model.HookAgentToolPlanUpdatingStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"todoList":[{"title":"Step 1","status":"completed"},{"title":"Step 2","status":"in-progress"}]}}`)},
		{"agent.ended", model.HookAgentEnded, sessionInput},
		{"agent.thinking.starting", model.HookAgentThinkingStarting, json.RawMessage(`{"session_id":"test-session-1","text":"Planning the approach"}`)},
		{"agent.thinking.ended", model.HookAgentThinkingEnded, json.RawMessage(`{"session_id":"test-session-1","text":"Decided to use X"}`)},
	}
	for _, ae := range agentEvents {
		hctx := model.HookContext{
			Event:    ae.event,
			Client:   "copilot-chat",
			Second:   "2026-02-23T10:01:00Z",
			RepoRoot: tmpDir,
			Input:    ae.input,
		}
		RunHook(hctx)
	}

	logFiles := getLogFiles(t, tmpDir)

	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for all events, got %d", sessionCount)
	}

	// 🪪️Verify that all 16 events are in the session.json file
	var sessionFile string
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionFile = f
			break
		}
	}

	data, err := os.ReadFile(sessionFile)
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}

	var meta model.SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	// Should have 18 events (1 prompt + 15 agent events + 2 derived events from search operations)
	expectedEventCount := 18
	if len(meta.Events) != expectedEventCount {
		t.Fatalf("expected %d events in session.json, got %d", expectedEventCount, len(meta.Events))
	}

	ticket = readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Fatal("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookSearchingPattern(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	workspace.SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"search-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Search test",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"search-session","tool_input":{"query":"findMe","includePattern":"src/**/*.ts"}}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for search-session, got %d", sessionCount)
	}

	// Check that session.json contains the search event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			if strings.Contains(string(data), "findMe") {
				found = true
				break
			}
		}
	}
	if !found {
		t.Error("expected search query in session.json")
	}
}

func TestTrackHookBlockedEvent(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	workspace.SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"blocked-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Do something",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"blocked-session","tool_input":{"command":"git checkout main"}}`),
		ToolName: "run_in_terminal",
		ToolArgs: "git checkout main",
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for blocked-session, got %d", sessionCount)
	}
	// Check that session.json contains the blocked event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta model.SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					if entry.Response != nil && entry.Response.Blocked != nil && *entry.Response.Blocked {
						found = true
						break
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected blocked event in session.json")
	}
}

func TestTrackHookTerminalEvents(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	workspace.SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"terminal-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Run command",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","tool_input":{"command":"npm test"}}`),
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","command":"npm test","pid":"999","stdout":"all passed"}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for terminal-session, got %d", sessionCount)
	}
	// Check that session.json contains terminal events
	startFound := false
	endFound := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta model.SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if kind, ok := evt["kind"].(string); ok {
							if kind == "agent.tool.terminal.starting" {
								startFound = true
							} else if kind == "agent.tool.terminal.ended" {
								endFound = true
							}
						}
					}
				}
			}
		}
	}
	if !startFound {
		t.Error("expected agent-tool-terminal-starting event in session.json")
	}
	if !endFound {
		t.Error("expected agent-tool-terminal-ended event in session.json")
	}
}

func TestTrackHookTranscriptInSession(t *testing.T) {
	tmpDir, ticketJSON := setupTicketDir(t)
	workspace.SetRootDir(tmpDir)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"transcript-session","transcript_path":"/home/user/.vscode/transcripts/abc.jsonl"}`),
		ToolArgs: "test",
	})
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Error("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookCodeEditedLogsToFile(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	workspace.SetRootDir(tmpDir)
	tsContent := "// #region \U0001F516Functions\n\n// doWork MUST work.\nexport function doWork(): void {}\n\n// #endregion \U0001F516Functions\n"
	tsFile := filepath.Join(tmpDir, "proj", "kit", "file.ts")
	if err := os.MkdirAll(filepath.Dir(tsFile), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(tsFile, []byte(tsContent), 0644); err != nil {
		t.Fatal(err)
	}
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"def-id-session"}`),
		ToolArgs: "Edit file",
	})
	payload := fmt.Sprintf(`{"session_id":"def-id-session","tool_input":{"filePath":%q,"oldString":"","newString":"export function doWork(): void {}"}}`, tsFile)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(payload),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for def-id-session, got %d", sessionCount)
	}

	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta model.SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if evt["kind"] == "agent.tool.code.edit.ended" {
							found = true
							break
						}
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected agent-tool-code-edit-ended event in session.json")
	}
}

func TestExtractCommandFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"claude code tool_input.command", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "git checkout main"},
		{"cursor beforeShellExecution", `{"command":"git stash pop"}`, "git stash pop"},
		{"windsurf tool_info.command_line", `{"tool_info":{"command_line":"git reset --hard"}}`, "git reset --hard"},
		{"no command", `{"tool_name":"ReadFile","tool_input":{"path":"/tmp"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractCommandFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractCommandFromStdinBlocking(t *testing.T) {
	payload := json.RawMessage(`{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		Input:    payload,
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected stdin-based git checkout to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestResolveHookEventCommandReclassification(t *testing.T) {
	searchCommands := []struct {
		name string
		cmd  string
	}{
		{"grep", "grep -r pattern ."},
		{"rg", "rg pattern"},
		{"ripgrep", "ripgrep pattern"},
		{"ag", "ag pattern"},
		{"find", "find . -name '*.go'"},
		{"fd", "fd pattern"},
		{"ls", "ls -la"},
		{"ls piped", "ls .🦑️repo/ | head -20"},
		{"cat", "cat file.txt"},
		{"head", "head -20 file.txt"},
		{"tail", "tail -f log.txt"},
		{"wc", "wc -l file.txt"},
		{"diff", "diff file1 file2"},
		{"jq", "jq '.name' package.json"},
		{"sort", "sort file.txt"},
		{"echo", "echo hello"},
		{"pwd", "pwd"},
		{"ps", "ps aux"},
		{"which", "which go"},
		{"tree", "tree -L 2"},
		{"stat", "stat file.txt"},
		{"sed read-only", "sed 's/old/new/g' file.txt"},
		{"sed -n print lines", "sed -n '1,50p' file.txt"},
		{"awk read-only", "awk '{print $1}' file.txt"},
		{"gawk read-only", "gawk '{print $1}' file.txt"},
	}
	editCommands := []struct {
		name string
		cmd  string
	}{
		{"sed -i in-place", "sed -i 's/old/new/g' file.txt"},
		{"sed -i.bak with backup", "sed -i.bak 's/old/new/g' file.txt"},
		{"rm", "rm -rf temp/"},
		{"mv", "mv old.txt new.txt"},
		{"cp", "cp src.txt dst.txt"},
		{"mkdir", "mkdir -p newdir"},
		{"touch", "touch new.txt"},
		{"chmod", "chmod 644 file.txt"},
		{"ln", "ln -s target link"},
		{"tee", "tee output.log"},
		{"patch", "patch -p1 < fix.patch"},
		{"tar", "tar -xzf archive.tar.gz"},
		{"zip", "zip archive.zip file.txt"},
	}
	terminalCommands := []struct {
		name string
		cmd  string
	}{
		{"git", "git status"},
		{"npm", "npm install"},
		{"go build", "go build ./..."},
		{"cargo build", "cargo build"},
		{"python", "python script.py"},
		{"node", "node script.js"},
		{"make build", "make build"},
		{"docker", "docker build ."},
		{"curl", "curl https://example.com"},
	}
	testCommands := []struct {
		name string
		cmd  string
	}{
		{"go test", "go test ./..."},
		{"cargo test", "cargo test"},
		{"cargo nextest", "cargo nextest run"},
		{"npm test", "npm test"},
		{"pnpm test", "pnpm test"},
		{"yarn test", "yarn test"},
		{"bun test", "bun test"},
		{"npx jest", "npx jest"},
		{"npx vitest", "npx vitest run"},
		{"jest direct", "jest --testPathPattern=foo"},
		{"vitest direct", "vitest run"},
		{"mocha direct", "mocha test/"},
		{"pytest direct", "pytest -k test_foo"},
		{"python -m pytest", "python -m pytest"},
		{"uv run pytest", "uv run pytest"},
		{"make test", "make test"},
		{"dotnet test", "dotnet test"},
		{"swift test", "swift test"},
		{"mix test", "mix test"},
		{"mvn test", "mvn test"},
		{"gradle test", "gradle test"},
		{"rspec direct", "rspec spec/"},
		{"phpunit direct", "phpunit tests/"},
		{"phpunit vendor", "./vendor/bin/phpunit tests/"},
		{"cargo nextest", "cargo nextest run"},

		{"cd && go test", "cd /workspaces/semio/repo/client && go test -v -run TestFoo -timeout 60s 2>&1 | tail -80"},
		{"cd && cargo test", "cd /path/to/technology && cargo test"},
		{"cd && npm test", "cd frontend && npm test"},
		{"cd && pytest", "cd tests && pytest -k test_integration"},
		{"cd && dotnet test", "cd /app && dotnet test"},

		{"cd; go test", "cd /path; go test ./..."},

		{"go test piped", "go test -v ./... | head -50"},

		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v ./..."},
	}
	clients := []struct {
		name      string
		client    string
		preEvent  string
		postEvent string
		toolName  string
		mkInput   func(cmd string) json.RawMessage
	}{
		{
			"claude-code", "claude-code", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"copilot-chat", "copilot-chat", "PreToolUse", "PostToolUse", "run_in_terminal",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"droid", "droid", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"codex", "codex", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"antigravity-chat", "antigravity-chat", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"windsurf-chat", "windsurf-chat", "pre_run_command", "post_run_command", "",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"input":{"command_line":%q}}`, cmd))
			},
		},
		{
			"cursor-chat", "cursor-chat", "preToolUse", "postToolUse", "terminal",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"terminal","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"cursor-chat-shell", "cursor-chat", "beforeShellExecution", "afterShellExecution", "",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":%q}}`, cmd))
			},
		},
	}
	for _, cl := range clients {
		for _, sc := range searchCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/searching", cl.name, sc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(sc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolSearchStarting {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolSearchStarting, event, sc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/searched", cl.name, sc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(sc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolSearchEnded {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolSearchEnded, event, sc.cmd)
				}
			})
		}
		for _, ec := range editCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/editing", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolCodeEditStarting {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolCodeEditStarting, event, ec.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/edited", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolCodeEditEnded {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolCodeEditEnded, event, ec.cmd)
				}
			})
		}
		for _, tc := range terminalCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolTerminalStarting {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolTerminalStarting, event, tc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolTerminalEnded {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolTerminalEnded, event, tc.cmd)
				}
			})
		}
		for _, xc := range testCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/test-starting", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolTestStarting {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolTestStarting, event, xc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/test-ended", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != model.HookAgentToolTestEnded {
					t.Errorf("expected %s, got %s for command %q", model.HookAgentToolTestEnded, event, xc.cmd)
				}
			})
		}
	}
}

func TestExtractTestStartingFromInputResolvesTestIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "mypackage")
	os.MkdirAll(goDir, 0755)
	goContent := `package mypackage
func TestOne(t *testing.T) {}
func TestTwo(t *testing.T) {}
`
	os.WriteFile(filepath.Join(goDir, "my_test.go"), []byte(goContent), 0644)

	pyDir := filepath.Join(tmpDir, "tests")
	os.MkdirAll(pyDir, 0755)
	pyContent := `def test_alpha():
    pass

def test_beta():
    pass
`
	os.WriteFile(filepath.Join(pyDir, "test_sample.py"), []byte(pyContent), 0644)

	jsDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(jsDir, 0755)
	jsContent := `import { describe, it } from 'vitest'

describe('suite', () => {
  it('alpha', () => {})
  it('beta', () => {})
})
`
	os.WriteFile(filepath.Join(jsDir, "app.test.ts"), []byte(jsContent), 0644)

	rubyDir := filepath.Join(tmpDir, "spec")
	os.MkdirAll(rubyDir, 0755)
	rubyContent := `describe 'suite' do
  it 'alpha' do
  end

  it 'beta' do
  end
end
`
	os.WriteFile(filepath.Join(rubyDir, "app_spec.rb"), []byte(rubyContent), 0644)

	phpDir := filepath.Join(tmpDir, "tests", "Feature")
	os.MkdirAll(phpDir, 0755)
	phpContent := `<?php
final class ExampleTest extends TestCase {
    public function testAlpha(): void {}
    public function testBeta(): void {}
}
`
	os.WriteFile(filepath.Join(phpDir, "ExampleTest.php"), []byte(phpContent), 0644)

	t.Run("command_from_tool_info", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved from command, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
		if !strings.Contains(labs[0], model.EmojiText(model.EmojiFileLab)) {
			t.Errorf("expected normalized lab IDs, got %v", labs)
		}
	})

	t.Run("command_from_tool_input", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":"go test -v ./mypackage/..."},"tool_info":{"cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
	})

	t.Run("explicit_test_names_not_overwritten", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"tests":["TestSpecific"],"files":["some_test.go"]}}`)
		_, tests, _ := extractTestStartingFromInput(input, "")

		if len(tests) != 1 || tests[0] != "TestSpecific" {
			t.Errorf("expected explicit test name to be preserved, got %v", tests)
		}
	})

	t.Run("go_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full suite, got %v", tests)
		}
		if !strings.Contains(labs[0], model.EmojiText(model.EmojiFileLab)) || !strings.Contains(labs[0], "mypackage") {
			t.Errorf("expected normalized lab id for my_test.go, got %v", labs)
		}
	})

	t.Run("go_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -run TestOne ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testone") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted TestOne id, got %v", tests)
		}
	})

	t.Run("python_wrapped_runner_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"python -m pytest tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped pytest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped pytest full suite, got %v", tests)
		}
	})

	t.Run("uv_pytest_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"uv run pytest -k test_alpha tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted uv pytest run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted uv pytest run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testalpha") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted test_alpha id, got %v", tests)
		}
	})

	t.Run("vitest_wrapped_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"npx vitest run src/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped vitest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped vitest full suite, got %v", tests)
		}
	})

	t.Run("bundle_rspec_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"bundle exec rspec --example alpha","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted rspec run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted rspec run, got none")
		}
	})

	t.Run("phpunit_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"./vendor/bin/phpunit tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for phpunit full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for phpunit full suite, got %v", tests)
		}
	})
}

func TestExtractTestEndedFromInputResolvesFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "pkg")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "pkg_test.go"), []byte("package pkg\nfunc TestX(t *testing.T) {}\n"), 0644)

	t.Run("resolves_files_from_command", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test ./pkg/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) == 0 {
			t.Fatalf("expected files to be resolved from command, got empty")
		}
	})

	t.Run("explicit_files_preserved", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"files":["explicit_test.go"]},"tool_output":{"succeeded":["TestX"]}}`)
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) != 1 || files[0] != "explicit_test.go" {
			t.Errorf("expected explicit file, got %v", files)
		}
	})
}

func TestResolveHookEvent(t *testing.T) {
	cases := []struct {
		name      string
		eventStr  string
		client    string
		toolName  string
		expectEvt model.HookEvent
		expectPar string
		expectErr bool
	}{
		{"neutral agent.started", "agent.started", "copilot-chat", "", model.HookAgentStarted, "", false},
		{"neutral agent.tool.code.edit.starting", "agent.tool.code.edit.starting", "copilot-chat", "", model.HookAgentToolCodeEditStarting, "", false},
		{"copilot SessionStart", "SessionStart", "copilot-chat", "", model.HookAgentStarted, "", false},
		{"copilot Stop", "Stop", "copilot-chat", "", model.HookAgentEnded, "", false},
		{"copilot SubagentStart", "SubagentStart", "copilot-chat", "", model.HookAgentStarted, "subagent", false},
		{"copilot SubagentStop", "SubagentStop", "copilot-chat", "", model.HookAgentEnded, "subagent", false},
		{"copilot UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", model.HookAgentPromptSubmitting, "", false},
		{"copilot PreCompact", "PreCompact", "copilot-chat", "", model.HookAgentCompacting, "", false},
		{"copilot PreToolUse generic", "PreToolUse", "copilot-chat", "runSubagent", model.HookAgentToolStarting, "", false},
		{"copilot PreToolUse read_file", "PreToolUse", "copilot-chat", "read_file", model.HookAgentToolSearchStarting, "", false},
		{"copilot PreToolUse create_file", "PreToolUse", "copilot-chat", "create_file", model.HookAgentToolCodeEditStarting, "", false},
		{"copilot PreToolUse run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", model.HookAgentToolTerminalStarting, "", false},
		{"copilot PreToolUse manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", model.HookAgentToolPlanUpdatingStarting, "", false},
		{"copilot PostToolUse read_file", "PostToolUse", "copilot-chat", "read_file", model.HookAgentToolSearchEnded, "", false},
		{"copilot PostToolUse create_file", "PostToolUse", "copilot-chat", "create_file", model.HookAgentToolCodeEditEnded, "", false},
		{"copilot PostToolUse run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", model.HookAgentToolTerminalEnded, "", false},
		{"copilot PostToolUse generic", "PostToolUse", "copilot-chat", "runSubagent", model.HookAgentToolEnded, "", false},
		{"cursor sessionStart", "sessionStart", "cursor-chat", "", model.HookAgentStarted, "", false},
		{"cursor sessionEnd", "sessionEnd", "cursor-chat", "", model.HookAgentEnded, "", false},
		{"cursor subagentStart", "subagentStart", "cursor-chat", "", model.HookAgentStarted, "subagent", false},
		{"cursor beforeReadFile", "beforeReadFile", "cursor-chat", "", model.HookAgentToolSearchStarting, "", false},
		{"cursor afterFileEdit", "afterFileEdit", "cursor-chat", "", model.HookAgentToolCodeEditEnded, "", false},
		{"cursor beforeShellExecution", "beforeShellExecution", "cursor-chat", "", model.HookAgentToolTerminalStarting, "", false},
		{"cursor afterShellExecution", "afterShellExecution", "cursor-chat", "", model.HookAgentToolTerminalEnded, "", false},
		{"cursor beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", model.HookAgentToolStarting, "", false},
		{"cursor afterMCPExecution", "afterMCPExecution", "cursor-chat", "", model.HookAgentToolEnded, "", false},
		{"cursor afterAgentResponse", "afterAgentResponse", "cursor-chat", "", model.HookAgentEnded, "", false},
		{"cursor afterAgentThought", "afterAgentThought", "cursor-chat", "", model.HookAgentThinkingEnded, "", false},
		{"cursor beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", model.HookAgentToolSearchStarting, "", false},
		{"cursor afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", model.HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", model.HookAgentPromptSubmitting, "", false},
		{"windsurf post_cascade_response", "post_cascade_response", "windsurf-chat", "", model.HookAgentEnded, "", false},
		{"windsurf post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", model.HookAgentStarted, "", false},
		{"windsurf pre_read_code", "pre_read_code", "windsurf-chat", "", model.HookAgentToolSearchStarting, "", false},
		{"windsurf pre_write_code", "pre_write_code", "windsurf-chat", "", model.HookAgentToolCodeEditStarting, "", false},
		{"windsurf post_write_code", "post_write_code", "windsurf-chat", "", model.HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_run_command", "pre_run_command", "windsurf-chat", "", model.HookAgentToolTerminalStarting, "", false},
		{"windsurf post_run_command", "post_run_command", "windsurf-chat", "", model.HookAgentToolTerminalEnded, "", false},
		{"windsurf pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", model.HookAgentToolStarting, "", false},
		{"windsurf post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", model.HookAgentToolEnded, "", false},
		{"claude SessionStart", "SessionStart", "claude-code", "", model.HookAgentStarted, "", false},
		{"claude SessionEnd", "SessionEnd", "claude-code", "", model.HookAgentEnded, "", false},
		{"claude SubagentStart", "SubagentStart", "claude-code", "", model.HookAgentStarted, "subagent", false},
		{"claude SubagentStop", "SubagentStop", "claude-code", "", model.HookAgentEnded, "subagent", false},
		{"claude TaskCompleted", "TaskCompleted", "claude-code", "", model.HookAgentToolPlanUpdatingEnded, "", false},
		{"claude PermissionRequest", "PermissionRequest", "claude-code", "", model.HookAgentToolStarting, "", false},
		{"claude TeammateIdle", "TeammateIdle", "claude-code", "", model.HookAgentToolStarting, "", false},
		{"claude Notification", "Notification", "claude-code", "", model.HookAgentToolStarting, "", false},
		{"claude PreToolUse Bash", "PreToolUse", "claude-code", "Bash", model.HookAgentToolTerminalStarting, "", false},
		{"claude PostToolUse Bash", "PostToolUse", "claude-code", "Bash", model.HookAgentToolTerminalEnded, "", false},
		{"claude PreToolUse Read", "PreToolUse", "claude-code", "Read", model.HookAgentToolSearchStarting, "", false},
		{"claude PreToolUse Glob", "PreToolUse", "claude-code", "Glob", model.HookAgentToolSearchStarting, "", false},
		{"claude PostToolUse Glob", "PostToolUse", "claude-code", "Glob", model.HookAgentToolSearchEnded, "", false},
		{"claude PreToolUse Edit", "PreToolUse", "claude-code", "Edit", model.HookAgentToolCodeEditStarting, "", false},
		{"claude PostToolUse Edit", "PostToolUse", "claude-code", "Edit", model.HookAgentToolCodeEditEnded, "", false},
		{"droid PreToolUse", "PreToolUse", "droid", "Bash", model.HookAgentToolTerminalStarting, "", false},
		{"codex PreToolUse", "PreToolUse", "codex", "Read", model.HookAgentToolSearchStarting, "", false},
		{"antigravity PreToolUse", "PreToolUse", "antigravity-chat", "Task", model.HookAgentToolPlanUpdatingStarting, "", false},
		{"unknown client defaults to claude-compatible", "SessionStart", "unknown-client", "", model.HookAgentStarted, "", false},
		{"kiro agentSpawn", "agentSpawn", "kiro-cli", "", model.HookAgentStarted, "", false},
		{"kiro userPromptSubmit", "userPromptSubmit", "kiro-cli", "", model.HookAgentPromptSubmitting, "", false},
		{"kiro preToolUse fs_read", "preToolUse", "kiro-cli", "fs_read", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse fs_write", "preToolUse", "kiro-cli", "fs_write", model.HookAgentToolCodeEditStarting, "", false},
		{"kiro preToolUse execute_bash", "preToolUse", "kiro-cli", "execute_bash", model.HookAgentToolTerminalStarting, "", false},
		{"kiro preToolUse code", "preToolUse", "kiro-cli", "code", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse grep", "preToolUse", "kiro-cli", "grep", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse glob", "preToolUse", "kiro-cli", "glob", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_search", "preToolUse", "kiro-cli", "web_search", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_fetch", "preToolUse", "kiro-cli", "web_fetch", model.HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse use_subagent", "preToolUse", "kiro-cli", "use_subagent", model.HookAgentToolStarting, "", false},
		{"kiro postToolUse fs_write", "postToolUse", "kiro-cli", "fs_write", model.HookAgentToolCodeEditEnded, "", false},
		{"kiro postToolUse execute_bash", "postToolUse", "kiro-cli", "execute_bash", model.HookAgentToolTerminalEnded, "", false},
		{"kiro stop", "stop", "kiro-cli", "", model.HookAgentEnded, "", false},
		{"kiro invalid event", "UnknownEvent", "kiro-cli", "", "", "", true},
		{"invalid copilot event", "UnknownEvent", "copilot-chat", "", "", "", true},
		{"invalid cursor event", "UnknownEvent", "cursor-chat", "", "", "", true},
		{"invalid windsurf event", "UnknownEvent", "windsurf-chat", "", "", "", true},
		{"invalid claude event", "UnknownEvent", "claude-code", "", "", "", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, parent, err := ResolveHookEvent(tc.eventStr, tc.client, tc.toolName, nil)
			if tc.expectErr {
				if err == nil {
					t.Errorf("expected error, got event=%s parent=%s", event, parent)
				}
				return
			}
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if event != tc.expectEvt {
				t.Errorf("expected event %s, got %s", tc.expectEvt, event)
			}
			if parent != tc.expectPar {
				t.Errorf("expected parent %q, got %q", tc.expectPar, parent)
			}
		})
	}
}

func TestPopulateEventDataAgentStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-abc","parent":"subagent"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T10:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "sess-abc" {
		t.Errorf("expected session=sess-abc, got %s", res.Session)
	}
	if res.Second != "2026-02-19T10:00:00Z" {
		t.Errorf("expected second=2026-02-19T10:00:00Z, got %s", res.Second)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
	if res.Parent != "" {
		t.Errorf("expected empty parent, got %s", res.Parent)
	}
	if res.Raw == nil {
		t.Error("expected raw to be populated")
	}
}

func TestPopulateEventDataAgentStartingSubagentParentFromContextUsesSession(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-sub"}`)
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "codex",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "sess-sub" {
		t.Errorf("expected parent=sess-sub, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingSubagentUsesAgentIDAsSession(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-parent","agent_id":"agent-child"}`)
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "copilot-chat",
		Second:     "2026-03-06T19:54:55.558Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "agent-child" {
		t.Errorf("expected session=agent-child, got %s", res.Session)
	}
	if res.Parent != "sess-parent" {
		t.Errorf("expected parent=sess-parent, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingParentFromContext(t *testing.T) {
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "claude-code",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		ParentInfo: "parent-agent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "parent-agent" {
		t.Errorf("expected parent=parent-agent, got %s", res.Parent)
	}
	if res.Client != "claude-code" {
		t.Errorf("expected client=claude-code, got %s", res.Client)
	}
}

func TestPopulateEventDataAgentEnded(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-end","tool_info":{"response":"final report text"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentEnded,
		Client:   "cursor-chat",
		Second:   "2026-02-19T11:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentEnded, got %T", result)
	}
	if res.Session != "sess-end" {
		t.Errorf("expected session=sess-end, got %s", res.Session)
	}
	if res.Second != "2026-02-19T11:00:00Z" {
		t.Errorf("expected second, got %s", res.Second)
	}
	if res.Client != "cursor-chat" {
		t.Errorf("expected client=cursor-chat, got %s", res.Client)
	}
	if res.Report != "final report text" {
		t.Errorf("expected report to be extracted, got %q", res.Report)
	}
}

func TestPopulateEventDataPromptSubmitting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-p","prompt":"Fix the bug in main.go"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T12:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentPromptSubmitting)
	if !ok {
		t.Fatalf("expected HookResultAgentPromptSubmitting, got %T", result)
	}
	if res.Session != "sess-p" {
		t.Errorf("expected session=sess-p, got %s", res.Session)
	}
	if res.Prompt != "Fix the bug in main.go" {
		t.Errorf("expected prompt, got %s", res.Prompt)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
}

func TestPopulateEventDataCompacting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-c","chat":"previous conversation context"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentCompacting,
		Client:   "claude-code",
		Second:   "2026-02-19T13:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentCompacting)
	if !ok {
		t.Fatalf("expected HookResultAgentCompacting, got %T", result)
	}
	if res.Session != "sess-c" {
		t.Errorf("expected session=sess-c, got %s", res.Session)
	}
	if res.Chat != "previous conversation context" {
		t.Errorf("expected chat content, got %s", res.Chat)
	}
}

func TestPopulateEventDataToolStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-t","tool_name":"runSubagent","tool_input":{"prompt":"do something"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:00:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Session != "sess-t" {
		t.Errorf("expected session=sess-t, got %s", res.Session)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	var inputData map[string]interface{}
	if err := json.Unmarshal(res.Input, &inputData); err != nil {
		t.Fatalf("expected valid JSON input, got: %v", err)
	}
	if inputData["prompt"] != "do something" {
		t.Errorf("expected prompt in input, got %v", inputData["prompt"])
	}
}

func TestPopulateEventDataToolEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-te","tool_name":"runSubagent","tool_input":{"prompt":"do something"},"tool_output":"done"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:30:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolEnded, got %T", result)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	if res.Response == nil {
		t.Error("expected response to be populated")
	}
}

func TestPopulateEventDataPlanUpdating(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-plan","tool_input":{"todoList":[{"id":1,"title":"Step 1","status":"completed"},{"id":2,"title":"Step 2","status":"in-progress"},{"id":3,"title":"Step 3","status":"not-started"}]}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T15:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolPlanUpdating)
	if !ok {
		t.Fatalf("expected HookResultAgentToolPlanUpdating, got %T", result)
	}
	if res.Session != "sess-plan" {
		t.Errorf("expected session=sess-plan, got %s", res.Session)
	}
	if len(res.Steps) != 3 {
		t.Fatalf("expected 3 steps, got %d", len(res.Steps))
	}
	if res.Steps[0].Name != "Step 1" || res.Steps[0].Status != "completed" {
		t.Errorf("expected Step 1 completed, got %+v", res.Steps[0])
	}
	if res.Steps[1].Name != "Step 2" || res.Steps[1].Status != "in-progress" {
		t.Errorf("expected Step 2 in-progress, got %+v", res.Steps[1])
	}
	if res.Steps[2].Name != "Step 3" || res.Steps[2].Status != "not-started" {
		t.Errorf("expected Step 3 not-started, got %+v", res.Steps[2])
	}
}

func TestPopulateEventDataCodeSearching(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cs","tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T16:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if res.Session != "sess-cs" {
		t.Errorf("expected session=sess-cs, got %s", res.Session)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestPopulateEventDataCodeEditing(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ce","tool_input":{"filePath":"/workspaces/semio/test.go","oldString":"old code","newString":"new code"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T17:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if res.Session != "sess-ce" {
		t.Errorf("expected session=sess-ce, got %s", res.Session)
	}
	if res.Path != "/workspaces/semio/test.go" {
		t.Errorf("expected path, got %s", res.Path)
	}
	if res.Old != "old code" {
		t.Errorf("expected old=old code, got %s", res.Old)
	}
	if res.New != "new code" {
		t.Errorf("expected new=new code, got %s", res.New)
	}
}

func TestPopulateEventDataCodeEditingWithAll(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cea","tool_input":{"filePath":"/tmp/file.ts","oldString":"x","newString":"y","all":true}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditStarting,
		Client:   "cursor-chat",
		Second:   "2026-02-19T17:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if !res.All {
		t.Error("expected all=true")
	}
}

func TestPopulateEventDataCodeEdited(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ced","tool_input":{"filePath":"/tmp/edited.ts","oldString":"before","newString":"after"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T18:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != "/tmp/edited.ts" {
		t.Errorf("expected path=/tmp/edited.ts, got %s", res.Path)
	}
	if res.Old != "before" {
		t.Errorf("expected old=before, got %s", res.Old)
	}
	if res.New != "after" {
		t.Errorf("expected new=after, got %s", res.New)
	}
}

func TestCodeEditEndedRunsFormatter(t *testing.T) {
	// 📋️Track whether the formatter was invoked.
	var formatterCalled bool
	var formatterBinary string
	origRun := workspace.FormatterCommandRun
	workspace.FormatterCommandRun = func(binary string, args []string, workDir string) error {
		formatterCalled = true
		formatterBinary = binary
		return nil
	}
	defer func() { workspace.FormatterCommandRun = origRun }()

	// Stub binary lookup so the formatter plan is considered available.
	origLookup := workspace.FormatterBinaryLookup
	workspace.FormatterBinaryLookup = func(file string) (string, error) { return "/usr/bin/" + file, nil }
	defer func() { workspace.FormatterBinaryLookup = origLookup }()

	tmpDir := t.TempDir()
	// Create requirement files so the plan is available (prettier needs .prettierrc.json and node_modules/.bin/prettier).
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin")
	_ = os.MkdirAll(prettierBin, 0o755)
	_ = os.WriteFile(filepath.Join(prettierBin, "prettier"), []byte("#!/bin/sh\n"), 0o755)
	_ = os.WriteFile(filepath.Join(tmpDir, ".prettierrc.json"), []byte("{}"), 0o644)

	origRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = origRoot }()

	relPath := "src/example.ts"
	payload := json.RawMessage(`{"sessionId":"sess-fmt","tool_input":{"filePath":"` + relPath + `","oldString":"old","newString":"new"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
		Client:   "claude-code",
		Second:   "2026-03-16T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := dispatchHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != relPath {
		t.Errorf("expected path=%s, got %s", relPath, res.Path)
	}
	if !formatterCalled {
		t.Error("expected formatter to be called on code edit ended, but it was not")
	}
	if !strings.Contains(formatterBinary, "prettier") {
		t.Errorf("expected prettier formatter for .ts file, got %s", formatterBinary)
	}
}

func TestPopulateEventDataTerminalStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ts","tool_input":{"command":"npm test"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolTerminalStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalStarting, got %T", result)
	}
	if res.Session != "sess-ts" {
		t.Errorf("expected session=sess-ts, got %s", res.Session)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
}

func TestPopulateEventDataTerminalEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tse","tool_input":{"command":"npm test"},"pid":"12345","terminated":true,"stdout":"all passed","stderr":""}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolTerminalEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalEnded, got %T", result)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
	if res.PID != 12345 {
		t.Errorf("expected pid=12345, got %d", res.PID)
	}
	if !res.Terminated {
		t.Error("expected terminated=true")
	}
	if res.Output != nil {
		var outputStr string
		if err := json.Unmarshal(res.Output, &outputStr); err != nil {
			t.Errorf("expected output to be JSON string, got error: %v", err)
		} else if outputStr != "all passed" {
			t.Errorf("expected stdout=all passed, got %s", outputStr)
		}
	}
}

func TestPopulateEventDataVersionCheckpointEnded(t *testing.T) {
	tmpDir := t.TempDir()
	payload := json.RawMessage(`{"sha":"abc123def","message":"feat: add hooks"}`)
	hctx := model.HookContext{
		Event:    model.HookVersionCheckpointEnded,
		Client:   "",
		Second:   "2026-02-19T20:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultVersionCheckpointEnded)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointEnded, got %T", result)
	}
	if res.Checkpoint != "abc123def" {
		t.Errorf("expected checkpoint=abc123def, got %s", res.Checkpoint)
	}
	if res.Description != "feat: add hooks" {
		t.Errorf("expected description=feat: add hooks, got %s", res.Description)
	}
}

func TestPopulateEventDataVersionCheckpointStartingFromFile(t *testing.T) {
	tmpDir := t.TempDir()
	gitDir := filepath.Join(tmpDir, ".git")
	if err := os.MkdirAll(gitDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: resolve issue"), 0644); err != nil {
		t.Fatal(err)
	}
	hctx := model.HookContext{
		Event:    model.HookVersionCheckpointStarting,
		Client:   "",
		RepoRoot: tmpDir,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultVersionCheckpointStarting)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
	}
	if res.Description != "fix: resolve issue" {
		t.Errorf("expected description=fix: resolve issue, got %s", res.Description)
	}
}

func TestPopulateEventDataRawField(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"raw-test","some":"data"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T21:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw == nil {
		t.Fatal("expected raw to be populated")
	}
	var rawData map[string]interface{}
	body, _ := json.Marshal(res.Raw)
	if err := json.Unmarshal(body, &rawData); err != nil {
		t.Fatalf("expected valid JSON raw, got: %v", err)
	}
	if rawData["sessionId"] != "raw-test" {
		t.Errorf("expected sessionId=raw-test in raw, got %v", rawData["sessionId"])
	}
	if rawData["some"] != "data" {
		t.Errorf("expected some=data in raw, got %v", rawData["some"])
	}
}

func TestPopulateEventDataNoInputNoRaw(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T22:00:00Z",
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw != nil {
		t.Error("expected raw to be nil when no input")
	}
}

func TestPopulateEventDataToolNameFromStdin(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tn","tool_name":"mcp_custom_tool","tool_input":{"arg":"val"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Name != "mcp_custom_tool" {
		t.Errorf("expected name=mcp_custom_tool, got %s", res.Name)
	}
}

func TestPopulateEventDataCodeSearchWithExclude(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ex","tool_input":{"query":"test","include":["*.ts","*.tsx"],"exclude":["node_modules"]}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestExtractToolInputMapFromData(t *testing.T) {
	t.Run("direct tool_input", func(t *testing.T) {
		data := map[string]interface{}{"tool_input": map[string]interface{}{"key": "val"}}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "val" {
			t.Errorf("expected tool_input with key=val, got %v", ti)
		}
	})
	t.Run("native.event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "native" {
			t.Errorf("expected native tool_input with key=native, got %v", ti)
		}
	})
	t.Run("event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"event": map[string]interface{}{
				"tool_input": map[string]interface{}{"key": "event"},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "event" {
			t.Errorf("expected event tool_input with key=event, got %v", ti)
		}
	})
	t.Run("direct takes precedence over native", func(t *testing.T) {
		data := map[string]interface{}{
			"tool_input": map[string]interface{}{"key": "direct"},
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "direct" {
			t.Errorf("expected direct tool_input to take precedence, got %v", ti)
		}
	})
	t.Run("empty data returns nil", func(t *testing.T) {
		ti := extractToolInputMapFromData(map[string]interface{}{})
		if ti != nil {
			t.Errorf("expected nil for empty data, got %v", ti)
		}
	})
}

func TestExtractSearchDefinitionReadsFromInput(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc First() {\n\tprintln(\"alpha\")\n}\n\nfunc Second() {\n\tprintln(\"beta\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	input := json.RawMessage(`{"tool_info":{"command_line":"grep -n \"beta\" sample.go","cwd":"` + filepath.ToSlash(tmpDir) + `"}}`)
	definitions := extractSearchDefinitionReadsFromInput(input, "")
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition read, got %v", definitions)
	}
	foundSecond := false
	for _, definition := range definitions {
		if strings.Contains(strings.ToLower(definition.ID), "second") {
			foundSecond = true
			if definition.Loc <= 0 {
				t.Fatalf("expected positive loc for Second, got %d", definition.Loc)
			}
		}
	}
	if !foundSecond {
		t.Fatalf("expected Second definition in %v", definitions)
	}
}

func TestRunHookSearchStartingIncludesDefinitions(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc Alpha() {\n\tprintln(\"one\")\n}\n\nfunc Beta() {\n\tprintln(\"needle\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	payload := json.RawMessage(`{"trajectory_id":"trace-1","tool_info":{"command_line":"grep -n \"needle\" sample.go","cwd":"` + filepath.ToSlash(tmpDir) + `"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected definitions in event, got %v", res.Definitions)
	}
	if res.Definitions[0].Loc <= 0 {
		t.Fatalf("expected loc > 0, got %d", res.Definitions[0].Loc)
	}
}

func TestBuildDefinitionReadsFullFileReturnsFileID(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "full.go")
	content := "package main\n\nfunc Alpha() {}\n\nfunc Beta() {}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(origRoot)

	lineSet := map[int]struct{}{1: {}, 2: {}, 3: {}, 4: {}, 5: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) != 1 {
		t.Fatalf("expected exactly one file-id entry for full file read, got %d: %v", len(definitions), definitions)
	}
	if definitions[0].Loc != 0 {
		t.Errorf("expected loc=0 for file-id entry, got %d", definitions[0].Loc)
	}
	if definitions[0].ID == "" {
		t.Errorf("expected non-empty file ID")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	if strings.Contains(definitions[0].ID, absPrefix) {
		t.Errorf("file ID %q contains absolute tmpDir path %q", definitions[0].ID, absPrefix)
	}
}

func TestBuildDefinitionReadsUsesRepoRelativePaths(t *testing.T) {
	tmpDir := t.TempDir()
	subDir := filepath.Join(tmpDir, "pkg")
	if err := os.MkdirAll(subDir, 0o755); err != nil {
		t.Fatalf("failed to create subdir: %v", err)
	}
	filePath := filepath.Join(subDir, "util.go")
	content := "package pkg\n\nfunc Helper() {\n\tprintln(\"found\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(origRoot)
	lineSet := map[int]struct{}{3: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition, got none")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute tmpDir path %q", def.ID, absPrefix)
		}
	}
}

func TestRunHookPreReadCodeDefinitionIDsAreRepoRelative(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "cmd.go")
	content := "package main\n\nfunc Run() {\n\tprintln(\"run\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	payload := json.RawMessage(`{"tool_info":{"file_path":"` + filepath.ToSlash(filePath) + `"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected at least one definition")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range res.Definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute path %q", def.ID, absPrefix)
		}
	}
}

func TestShouldExecuteSearchCommand(t *testing.T) {
	tests := []struct {
		name    string
		command string
		want    bool
	}{
		{name: "grep command", command: "grep -n needle file.go", want: true},
		{name: "cat command", command: "cat file.go", want: true},
		{name: "blocked redirection", command: "grep -n needle file.go > out.txt", want: false},
		{name: "edit command", command: "sed -i 's/a/b/' file.go", want: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := shouldExecuteSearchCommand(tt.command)
			if got != tt.want {
				t.Fatalf("shouldExecuteSearchCommand(%q) = %v, want %v", tt.command, got, tt.want)
			}
		})
	}
}

func TestExtractCodeEditFromInput(t *testing.T) {
	t.Run("replace_string_in_file style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/file.go","oldString":"old","newString":"new"}}`)
		path, old, new_, all := extractCodeEditFromInput(input, "")
		if path != "/tmp/file.go" {
			t.Errorf("expected path=/tmp/file.go, got %s", path)
		}
		if old != "old" {
			t.Errorf("expected old=old, got %s", old)
		}
		if new_ != "new" {
			t.Errorf("expected new=new, got %s", new_)
		}
		if all {
			t.Error("expected all=false")
		}
	})
	t.Run("with replaceAll", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/f.go","oldString":"a","newString":"b","replaceAll":true}}`)
		_, _, _, all := extractCodeEditFromInput(input, "")
		if !all {
			t.Error("expected all=true")
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		path, _, _, _ := extractCodeEditFromInput(nil, `{"filePath":"/from/args.go"}`)
		if path != "/from/args.go" {
			t.Errorf("expected path=/from/args.go, got %s", path)
		}
	})
}

func TestExtractTerminalEndedFromInput(t *testing.T) {
	t.Run("full payload", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"command":"ls -la"},"pid":"999","terminated":true,"stdout":"file1\nfile2","stderr":""}`)
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
		if command != "ls -la" {
			t.Errorf("expected command=ls -la, got %s", command)
		}
		if pid != "999" {
			t.Errorf("expected pid=999, got %s", pid)
		}
		if !terminated {
			t.Error("expected terminated=true")
		}
		if stdout != "file1\nfile2" {
			t.Errorf("expected stdout, got %s", stdout)
		}
		if stderr != "" {
			t.Errorf("expected empty stderr, got %s", stderr)
		}
	})
	t.Run("numeric pid", func(t *testing.T) {
		input := json.RawMessage(`{"pid":42}`)
		_, pid, _, _, _ := extractTerminalEndedFromInput(input)
		if pid != "42" {
			t.Errorf("expected pid=42, got %s", pid)
		}
	})
	t.Run("empty", func(t *testing.T) {
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(nil)
		if command != "" || pid != "" || terminated || stdout != "" || stderr != "" {
			t.Error("expected all empty for nil input")
		}
	})
}

func TestExtractChatFromInput(t *testing.T) {
	t.Run("chat string", func(t *testing.T) {
		input := json.RawMessage(`{"chat":"conversation context"}`)
		result := extractChatFromInput(input)
		if result != "conversation context" {
			t.Errorf("expected conversation context, got %s", result)
		}
	})
	t.Run("messages array", func(t *testing.T) {
		input := json.RawMessage(`{"messages":[{"role":"user","content":"hello"},{"role":"assistant","content":"hi"}]}`)
		result := extractChatFromInput(input)
		if result == "" {
			t.Error("expected non-empty chat from messages array")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractChatFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractReportFromInput(t *testing.T) {
	t.Run("report field", func(t *testing.T) {
		input := json.RawMessage(`{"report":"agent summary"}`)
		result := extractReportFromInput(input)
		if result != "agent summary" {
			t.Errorf("expected agent summary, got %s", result)
		}
	})
	t.Run("tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"tool_info":{"response":"from tool info"}}`)
		result := extractReportFromInput(input)
		if result != "from tool info" {
			t.Errorf("expected from tool info, got %s", result)
		}
	})
	t.Run("native event tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_info":{"response":"nested response"}}}}`)
		result := extractReportFromInput(input)
		if result != "nested response" {
			t.Errorf("expected nested response, got %s", result)
		}
	})
	t.Run("empty input", func(t *testing.T) {
		result := extractReportFromInput(nil)
		if result != "" {
			t.Errorf("expected empty report, got %s", result)
		}
	})
}

func TestExtractCheckpointMessageFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"message":"feat: new feature"}`)
		result := extractCheckpointMessageFromInput(input, "/nonexistent")
		if result != "feat: new feature" {
			t.Errorf("expected feat: new feature, got %s", result)
		}
	})
	t.Run("from COMMIT_EDITMSG file", func(t *testing.T) {
		tmpDir := t.TempDir()
		gitDir := filepath.Join(tmpDir, ".git")
		os.MkdirAll(gitDir, 0755)
		os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: bug fix"), 0644)
		result := extractCheckpointMessageFromInput(nil, tmpDir)
		if result != "fix: bug fix" {
			t.Errorf("expected fix: bug fix, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractCheckpointMessageFromInput(nil, "/nonexistent")
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractCheckpointSHAFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"sha":"deadbeef123"}`)
		result := extractCheckpointSHAFromInput(input)
		if result != "deadbeef123" {
			t.Errorf("expected deadbeef123, got %s", result)
		}
	})
	t.Run("empty falls back to git", func(t *testing.T) {
		result := extractCheckpointSHAFromInput(nil)
		if result == "" {
			t.Skip("no git repo available")
		}
	})
}

func TestExtractParentFromInput(t *testing.T) {
	t.Run("parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"parent-session"}`)
		result := extractParentFromInput(input)
		if result != "parent-session" {
			t.Errorf("expected parent-session, got %s", result)
		}
	})
	t.Run("nested event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"event":{"parent":"parent-from-event"}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-event" {
			t.Errorf("expected parent-from-event, got %s", result)
		}
	})
	t.Run("nested native event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"parent":"parent-from-native-event"}}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-native-event" {
			t.Errorf("expected parent-from-native-event, got %s", result)
		}
	})
	t.Run("sentinel parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"subagent"}`)
		result := extractParentFromInput(input)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractParentFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestNativeHookEventMappingFromRealLogFiles(t *testing.T) {
	repoRoot := workspace.FindRepoRoot(".")
	logDir := filepath.Join(repoRoot, ".🧬semio", "🦑️repo", "📜️")
	dirEntries, err := os.ReadDir(logDir)
	if err != nil {
		t.Skipf("no log directory: %v", err)
	}
	type oldLogEntry struct {
		Context struct {
			Event    string          `json:"event"`
			Client   string          `json:"client"`
			ToolName string          `json:"toolName"`
			Input    json.RawMessage `json:"input"`
		} `json:"context"`
	}
	seen := map[string]bool{}
	for _, de := range dirEntries {
		if de.IsDir() || !strings.HasSuffix(de.Name(), ".json") {
			continue
		}
		data, err := os.ReadFile(filepath.Join(logDir, de.Name()))
		if err != nil {
			continue
		}
		var old oldLogEntry
		if err := json.Unmarshal(data, &old); err != nil {
			continue
		}
		if old.Context.Event == "" || old.Context.Client == "" {
			continue
		}
		key := old.Context.Client + "|" + old.Context.Event + "|" + old.Context.ToolName
		if seen[key] {
			continue
		}
		seen[key] = true
		t.Run(fmt.Sprintf("%s/%s/%s", old.Context.Client, strings.ReplaceAll(old.Context.Event, ".", "-"), old.Context.ToolName), func(t *testing.T) {
			tmpDir := t.TempDir()
			writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
			hctx := model.HookContext{
				Event:    model.HookEvent(old.Context.Event),
				Client:   old.Context.Client,
				Second:   time.Now().UTC().Format(time.RFC3339),
				RepoRoot: tmpDir,
				ToolName: old.Context.ToolName,
				Input:    old.Context.Input,
			}
			result := RunHook(hctx)
			outBase := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache")
			var logFiles []string
			filepath.WalkDir(outBase, func(path string, d os.DirEntry, walkErr error) error {
				if walkErr != nil {
					return nil
				}
				if !d.IsDir() && strings.HasSuffix(d.Name(), ".json") {
					logFiles = append(logFiles, path)
				}
				return nil
			})
			if len(logFiles) != 1 {
				t.Fatalf("want 1 log file under ⚡️, got %d", len(logFiles))
			}
			outData, err := os.ReadFile(logFiles[0])
			if err != nil {
				t.Fatalf("read log: %v", err)
			}
			var entry model.HookLogEntry
			if err := json.Unmarshal(outData, &entry); err != nil {
				t.Fatalf("invalid log JSON: %v", err)
			}
			var evt map[string]interface{}
			if err := json.Unmarshal(entry.Event, &evt); err != nil {
				t.Fatalf("cannot unmarshal event: %v", err)
			}
			if evt["kind"] != string(old.Context.Event) {
				t.Errorf("kind: want %s, got %v", old.Context.Event, evt["kind"])
			}
			if evt["client"] != old.Context.Client {
				t.Errorf("client: want %s, got %v", old.Context.Client, evt["client"])
			}
			if old.Context.Input != nil && entry.Native.Event == nil {
				t.Error("native.event not preserved")
			}
			wantSession := ResolveEventSessionID(ExtractSessionIDFromInput(old.Context.Input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("session: want %s, got %v", wantSession, evt["session"])
			}
			wantTranscript := ExtractTranscriptFromInput(old.Context.Input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if result.IsAllowed() && entry.Response.Blocked != nil {
				t.Error("response.blocked should be nil for allowed event")
			}
		})
	}
}

func TestCheckpointInAllAgentEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	workspace.SetRootDir(tmpDir)

	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	sessionInput := json.RawMessage(`{"session_id":"checkpoint-test","llm":"opus-4-6"}`)
	agentEvents := []struct {
		name  string
		event model.HookEvent
		input json.RawMessage
	}{
		{"agent.started", model.HookAgentStarted, sessionInput},
		{"agent.ended", model.HookAgentEnded, sessionInput},
		{"agent.prompt.submitting", model.HookAgentPromptSubmitting, sessionInput},
		{"agent.compacting", model.HookAgentCompacting, sessionInput},
		{"agent.tool.starting", model.HookAgentToolStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.ended", model.HookAgentToolEnded, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.plan.updating.starting", model.HookAgentToolPlanUpdatingStarting, sessionInput},
		{"agent.tool.plan.updating.ended", model.HookAgentToolPlanUpdatingEnded, sessionInput},
		{"agent.file.read.starting", model.HookAgentToolSearchStarting, sessionInput},
		{"agent.file.read.ended", model.HookAgentToolSearchEnded, sessionInput},
		{"agent.tool.code.edit.starting", model.HookAgentToolCodeEditStarting, sessionInput},
		{"agent.tool.code.edit.ended", model.HookAgentToolCodeEditEnded, sessionInput},
		{"agent.tool.test.starting", model.HookAgentToolTestStarting, sessionInput},
		{"agent.tool.test.ended", model.HookAgentToolTestEnded, sessionInput},
		{"agent.tool.build.starting", model.HookAgentToolBuildStarting, sessionInput},
		{"agent.tool.build.ended", model.HookAgentToolBuildEnded, sessionInput},
		{"agent.tool.terminal.starting", model.HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_input":{"command":"echo test"}}`)},
		{"agent.tool.terminal.ended", model.HookAgentToolTerminalEnded, sessionInput},
		{"agent.thinking.starting", model.HookAgentThinkingStarting, json.RawMessage(`{"session_id":"checkpoint-test","text":"Planning the approach"}`)},
		{"agent.thinking.ended", model.HookAgentThinkingEnded, json.RawMessage(`{"session_id":"checkpoint-test","text":"Decided to use X"}`)},
	}
	for _, tc := range agentEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := model.HookContext{
				Event:    tc.event,
				Client:   "copilot-chat",
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			}
			result := dispatchHook(hctx)

			data, _ := json.Marshal(result)
			var m map[string]interface{}
			json.Unmarshal(data, &m)
			checkpoint, ok := m["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint field in %s result, got: %v", tc.name, m)
			}
			if checkpoint != expectedSHA {
				t.Errorf("expected checkpoint=%s, got %v", expectedSHA, checkpoint)
			}
		})
	}
}

func TestCheckpointInAllVersionEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	workspace.SetRootDir(tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	versionEvents := []struct {
		name  string
		event model.HookEvent
		input json.RawMessage
	}{
		{"checkpoint.starting", model.HookVersionCheckpointStarting, nil},
		{"checkpoint.ended", model.HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin.starting", model.HookVersionCheckinStarting, nil},
		{"checkin.ended", model.HookVersionCheckinEnded, nil},
		{"checkout.starting", model.HookVersionCheckoutStarting, nil},
		{"checkout.ended", model.HookVersionCheckoutEnded, nil},
	}
	for _, tc := range versionEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := model.HookContext{
				Event:    tc.event,
				Client:   "",
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			}
			result := dispatchHook(hctx)
			data, _ := json.Marshal(result)
			var m map[string]interface{}
			json.Unmarshal(data, &m)
			checkpoint, ok := m["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint field in %s result, got: %v", tc.name, m)
			}
			if checkpoint != expectedSHA {
				t.Errorf("expected checkpoint=%s, got %v", expectedSHA, checkpoint)
			}
		})
	}
}

func TestVersionHooksDoNotWriteSessionLogs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess version hook test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	workspace.SetRootDir(tmpDir)

	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))

	events := []struct {
		name  string
		event model.HookEvent
		input json.RawMessage
	}{
		{"checkpoint starting", model.HookVersionCheckpointStarting, nil},
		{"checkpoint ended", model.HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin starting", model.HookVersionCheckinStarting, nil},
		{"checkin ended", model.HookVersionCheckinEnded, nil},
		{"checkout starting", model.HookVersionCheckoutStarting, nil},
		{"checkout ended", model.HookVersionCheckoutEnded, nil},
	}

	for _, tc := range events {
		t.Run(tc.name, func(t *testing.T) {
			result := RunHook(model.HookContext{
				Event:    tc.event,
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			})
			if !result.IsAllowed() {
				t.Fatalf("expected %s to be allowed, got %s", tc.event, result.GetMessage())
			}
			assertNoHookLogFiles(t, tmpDir)
		})
	}
}

func TestCheckpointInLoggedEventJSON(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint logging test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	workspace.SetRootDir(tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	sessionInput := json.RawMessage(`{"session_id":"checkpoint-log","llm":"opus-4-6","transcript_path":"/tmp/t.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-25T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
	}
	RunHook(hctx)

	agentEventsDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache", "🤖️generated")
	var sessionJSONPath string
	filepath.WalkDir(agentEventsDir, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return nil
		}
		if !d.IsDir() && d.Name() == "session.json" {
			sessionJSONPath = path
		}
		return nil
	})
	if sessionJSONPath == "" {
		t.Fatal("expected session.json to be written for agent.started event")
	}
	data, err := os.ReadFile(sessionJSONPath)
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}
	var meta model.SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	if meta.Checkpoint != expectedSHA {
		t.Errorf("expected session.json checkpoint=%s, got %q", expectedSHA, meta.Checkpoint)
	}

	expectedCheckpoint := todos.ResolveEventCheckpointID(expectedSHA)
	found := false
	for _, entry := range meta.Events {
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if kind, _ := evt["kind"].(string); kind == "agent.started" {
			checkpoint, ok := evt["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint in agent.started event, got: %v", evt)
			}
			if checkpoint != expectedCheckpoint {
				t.Errorf("expected agent.started checkpoint=%s, got %v", expectedCheckpoint, checkpoint)
			}
			found = true
			break
		}
	}
	if !found {
		t.Error("expected agent.started event in session.json events array")
	}
}

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

func TestVersionHookEventsDispatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow version hook events dispatch test in short mode")
	}
	cases := []struct {
		name  string
		event model.HookEvent
	}{
		{"checkin starting", model.HookVersionCheckinStarting},
		{"checkin ended", model.HookVersionCheckinEnded},
		{"checkout starting", model.HookVersionCheckoutStarting},
		{"checkout ended", model.HookVersionCheckoutEnded},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := model.HookContext{
				Event:    tc.event,
				Client:   "",
				Second:   "2026-02-24T12:00:00Z",
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if !result.IsAllowed() {
				t.Errorf("expected allowed for %s, got denied: %s", tc.event, result.GetMessage())
			}
		})
	}
}
