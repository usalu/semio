// 🔖️ Tests of the checkpoint hooks, native event mapping and version hook dispatch.

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

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
