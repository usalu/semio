// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Checkpoint presence in agent and version events.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestCheckpointInAllAgentEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	bindRootForTest(t, tmpDir)

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
		event HookEvent
		input json.RawMessage
	}{
		{"agent.started", HookAgentStarted, sessionInput},
		{"agent.ended", HookAgentEnded, sessionInput},
		{"agent.prompt.submitting", HookAgentPromptSubmitting, sessionInput},
		{"agent.compacting", HookAgentCompacting, sessionInput},
		{"agent.tool.starting", HookAgentToolStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.ended", HookAgentToolEnded, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.plan.updating.starting", HookAgentToolPlanUpdatingStarting, sessionInput},
		{"agent.tool.plan.updating.ended", HookAgentToolPlanUpdatingEnded, sessionInput},
		{"agent.file.read.starting", HookAgentToolSearchStarting, sessionInput},
		{"agent.file.read.ended", HookAgentToolSearchEnded, sessionInput},
		{"agent.tool.code.edit.starting", HookAgentToolCodeEditStarting, sessionInput},
		{"agent.tool.code.edit.ended", HookAgentToolCodeEditEnded, sessionInput},
		{"agent.tool.test.starting", HookAgentToolTestStarting, sessionInput},
		{"agent.tool.test.ended", HookAgentToolTestEnded, sessionInput},
		{"agent.tool.build.starting", HookAgentToolBuildStarting, sessionInput},
		{"agent.tool.build.ended", HookAgentToolBuildEnded, sessionInput},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_input":{"command":"echo test"}}`)},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, sessionInput},
		{"agent.thinking.starting", HookAgentThinkingStarting, json.RawMessage(`{"session_id":"checkpoint-test","text":"Planning the approach"}`)},
		{"agent.thinking.ended", HookAgentThinkingEnded, json.RawMessage(`{"session_id":"checkpoint-test","text":"Decided to use X"}`)},
	}
	for _, tc := range agentEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
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
	bindRootForTest(t, tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	versionEvents := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"checkpoint.starting", HookVersionCheckpointStarting, nil},
		{"checkpoint.ended", HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin.starting", HookVersionCheckinStarting, nil},
		{"checkin.ended", HookVersionCheckinEnded, nil},
		{"checkout.starting", HookVersionCheckoutStarting, nil},
		{"checkout.ended", HookVersionCheckoutEnded, nil},
	}
	for _, tc := range versionEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
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
	bindRootForTest(t, tmpDir)

	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))

	events := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"checkpoint starting", HookVersionCheckpointStarting, nil},
		{"checkpoint ended", HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin starting", HookVersionCheckinStarting, nil},
		{"checkin ended", HookVersionCheckinEnded, nil},
		{"checkout starting", HookVersionCheckoutStarting, nil},
		{"checkout ended", HookVersionCheckoutEnded, nil},
	}

	for _, tc := range events {
		t.Run(tc.name, func(t *testing.T) {
			result := RunHook(HookContext{
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
	bindRootForTest(t, tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	sessionInput := json.RawMessage(`{"session_id":"checkpoint-log","llm":"opus-4-6","transcript_path":"/tmp/t.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
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
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	if meta.Checkpoint != expectedSHA {
		t.Errorf("expected session.json checkpoint=%s, got %q", expectedSHA, meta.Checkpoint)
	}

	expectedCheckpoint := resolveEventCheckpointID(expectedSHA)
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

func TestEventIDsUseComposeRepoFormat(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess event id test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	bindRootForTest(t, tmpDir)

	os.MkdirAll(filepath.Join(tmpDir, "src"), 0755)
	os.WriteFile(filepath.Join(tmpDir, "src", "main.go"), []byte("package main"), 0644)

	fileID := resolvePathToFileID(filepath.Join(tmpDir, "src", "main.go"))

	if fileID == "" {
		t.Error("expected non-empty file ID")
	}
	if strings.Contains(fileID, "/") {
		t.Errorf("file ID should not contain path separators: %s", fileID)
	}

	rangeRef, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10")
	if err != nil {
		t.Errorf("unexpected error resolving range ref: %v", err)
	}
	if rangeRef == "" {
		t.Error("expected non-empty range ref")
	}
	if !strings.Contains(rangeRef, emojiText(EmojiLine)) {
		t.Errorf("expected %s in range ref, got: %s", emojiText(EmojiLine), rangeRef)
	}
	if !strings.Contains(rangeRef, "10") {
		t.Errorf("expected line number 10 in range ref, got: %s", rangeRef)
	}

	rangeRefFull, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10-L20")
	if err != nil {
		t.Errorf("unexpected error resolving full range ref: %v", err)
	}
	if !strings.Contains(rangeRefFull, emojiText(EmojiLine)+"10"+emojiText(EmojiLine)+"20") {
		t.Errorf("expected %s10%s20 in full range ref, got: %s", emojiText(EmojiLine), emojiText(EmojiLine), rangeRefFull)
	}
}
