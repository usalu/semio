// 🔖️ Tests of the checkpoint hooks, native event mapping and version hook dispatch.

package hooks

import (
	json "encoding/json"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	todos "github.com/usalu/semio/repo/todos"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestCheckpointInAllAgentEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	sandbox.BindRoot(t, tmpDir)

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
	sandbox.BindRoot(t, tmpDir)
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
	sandbox.BindRoot(t, tmpDir)

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
	sandbox.BindRoot(t, tmpDir)
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
