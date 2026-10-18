// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo operation derivation and logging from MCP and CLI hooks.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestDeriveRepoOpFromMCPTool(t *testing.T) {
	cases := []struct {
		name     string
		tool     string
		expected string
	}{
		{"ticket open", "mcp__repo__ticket_open", "ticket.open"},
		{"ticket close", "mcp__repo__ticket_close", "ticket.close"},
		{"ticket reopen", "mcp__repo__ticket_reopen", "ticket.reopen"},
		{"ticket read", "mcp__repo__ticket_read", "ticket.read"},
		{"goal open", "mcp__repo__goal_open", "goal.open"},
		{"goal close", "mcp__repo__goal_close", "goal.close"},
		{"goal reopen", "mcp__repo__goal_reopen", "goal.reopen"},
		{"contributor add", "mcp__repo__contributor_add", "contributor.add"},
		{"contributor remove", "mcp__repo__contributor_remove", "contributor.remove"},
		{"draft create", "mcp__repo__draft_create", "draft.create"},
		{"draft delete", "mcp__repo__draft_delete", "draft.delete"},
		{"file create", "mcp__repo__file_create", "file.create"},
		{"file move", "mcp__repo__file_move", "file.move"},
		{"file delete", "mcp__repo__file_delete", "file.delete"},
		{"folder create", "mcp__repo__folder_create", "folder.create"},
		{"folder move", "mcp__repo__folder_move", "folder.move"},
		{"folder delete", "mcp__repo__folder_delete", "folder.delete"},
		{"section create", "mcp__repo__section_create", "section.create"},
		{"section move", "mcp__repo__section_move", "section.move"},
		{"section delete", "mcp__repo__section_delete", "section.delete"},
		{"integrate", "mcp__repo__integrate", "integrate"},
		{"extract", "mcp__repo__extract", "extract"},
		{"export", "mcp__repo__export", "export"},
		{"analyze", "mcp__repo__analyze", "analyze"},
		{"fix", "mcp__repo__fix", "fix"},
		{"tree", "mcp__repo__tree", "tree"},
		{"graphql", "mcp__repo__graphql", "graphql"},
		{"move", "mcp__repo__move", "move"},
		{"policy check", "mcp__repo__policy_check", "policy.check"},
		{"non-compose tool", "mcp__other__thing", ""},
		{"bash tool", "Bash", ""},
		{"empty", "", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := deriveRepoOpFromMCPTool(tc.tool)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromMCPTool(%q) = %q, want %q", tc.tool, got, tc.expected)
			}
		})
	}
}

func TestDeriveRepoOpFromCLICommand(t *testing.T) {
	cases := []struct {
		name     string
		cmd      string
		expected string
	}{
		{"ticket open full path", "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket open exe path", ".\\repo\\cli\\cli.exe ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket close", "./cli ticket close 26 03 05 MY-SLUG 'Summary' compose/go/compose.go", "ticket.close"},
		{"ticket reopen", "/workspaces/semio/repo/client/client ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"ticket reopen go run", "go run ./repo/client/mcp/go ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"goal open", "./cli goal open 'Title' 'Desc' 'Prompt' claude-code sonnet-4-5", "goal.open"},
		{"goal close", "./cli goal close MY-GOAL 'Summary'", "goal.close"},
		{"contributor add", "./cli contributor add github-user", "contributor.add"},
		{"single subcommand", "./cli analyze", "analyze"},
		{"non-cli binary", "./other-tool ticket open", ""},
		{"empty", "", ""},
		{"just cli", "./cli", ""},
		{"cli with flag first", "./cli -v ticket open", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := deriveRepoOpFromCLICommand(tc.cmd)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromCLICommand(%q) = %q, want %q", tc.cmd, got, tc.expected)
			}
		})
	}
}

func TestLogRepoOperationHookMCPTool(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess1")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for mcp__repo__tree", func(t *testing.T) {
		input := json.RawMessage(`{"query":"contributors"}`)
		result := HookResultAgentToolStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name:  "mcp__repo__tree",
			Input: input,
		}
		hctx := HookContext{
			Event:    HookAgentToolStarting,
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
		var meta SessionMeta
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
		result := HookResultAgentToolEnded{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name: "mcp__repo__ticket_open",
		}
		hctx := HookContext{
			Event:    HookAgentToolEnded,
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
		var meta SessionMeta
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
		result := HookResultAgentToolStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
			},
			Name: "Bash",
		}
		hctx := HookContext{
			Event:    HookAgentToolStarting,
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
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess2")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for CLI ticket open", func(t *testing.T) {
		result := HookResultAgentToolTerminalStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' claude-code sonnet-4-5",
		}
		hctx := HookContext{
			Event:    HookAgentToolTerminalStarting,
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
		var meta SessionMeta
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
		result := HookResultAgentToolTerminalEnded{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "/workspace/repo/client/client goal close MY-GOAL 'Summary'",
		}
		hctx := HookContext{
			Event:    HookAgentToolTerminalEnded,
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
		var meta SessionMeta
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
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-03-05T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)

	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 || entries[0].Name() != "session.json" {
		names := make([]string, len(entries))
		for i, e := range entries {
			names[i] = e.Name()
		}
		t.Fatalf("expected only session.json, got %d: %v", len(entries), names)
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta SessionMeta
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
