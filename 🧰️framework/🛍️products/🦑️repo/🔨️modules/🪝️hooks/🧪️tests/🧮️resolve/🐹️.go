// 🧮️ Tests of the hook event resolution and command reclassification.

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

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
