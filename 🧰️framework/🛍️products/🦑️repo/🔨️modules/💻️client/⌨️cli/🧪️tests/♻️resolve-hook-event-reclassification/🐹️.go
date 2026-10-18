// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook event reclassification and test info parsing.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"testing"
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
		{"ls piped", "ls .🧬semio/🦑️repo/ | head -20"},
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
				if event != HookAgentToolSearchStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolSearchStarting, event, sc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/searched", cl.name, sc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(sc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolSearchEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolSearchEnded, event, sc.cmd)
				}
			})
		}
		for _, ec := range editCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/editing", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolCodeEditStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolCodeEditStarting, event, ec.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/edited", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolCodeEditEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolCodeEditEnded, event, ec.cmd)
				}
			})
		}
		for _, tc := range terminalCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTerminalStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTerminalStarting, event, tc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTerminalEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTerminalEnded, event, tc.cmd)
				}
			})
		}
		for _, xc := range testCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/test-starting", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTestStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTestStarting, event, xc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/test-ended", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTestEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTestEnded, event, xc.cmd)
				}
			})
		}
	}
}

func TestParseTestInfoFromCommand(t *testing.T) {
	cases := []struct {
		name          string
		command       string
		expectTests   []string
		expectTimeout string
	}{
		{"go test all", "go test ./...", []string{""}, ""},
		{"go test -run", "go test -run TestFoo ./...", []string{"TestFoo"}, ""},
		{"go test -run -timeout", "go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"go test -timeout 2m", "go test -timeout 2m ./...", []string{""}, "120"},
		{"pytest all", "pytest", []string{""}, ""},
		{"pytest -k", "pytest -k test_foo", []string{"test_foo"}, ""},
		{"jest all", "jest", []string{""}, ""},
		{"jest -t", "jest -t MyTest", []string{"MyTest"}, ""},
		{"jest --testNamePattern", "jest --testNamePattern=MyTest", []string{"MyTest"}, ""},
		{"jest --timeout", "jest --timeout 5000", []string{""}, "5000"},
		{"mocha --grep", "mocha --grep mytest", []string{"mytest"}, ""},
		{"mocha --timeout", "mocha --timeout 3000 test/", []string{""}, "3000"},
		{"cargo test -- filter", "cargo test -- my_test", []string{"my_test"}, ""},
		{"dotnet --filter", "dotnet test --filter Category=Unit", []string{"Category=Unit"}, ""},
		{"rspec --example", "rspec --example myexample", []string{"myexample"}, ""},

		{"cd && go test -run", "cd /path && go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"cd && go test all", "cd /path && go test ./...", []string{""}, ""},
		{"cd && go test piped", "cd /workspaces/semio/repo/client && go test -v -run TestBar -timeout 60s 2>&1 | tail -80", []string{"TestBar"}, "60"},
		{"cd && pytest -k", "cd /app && pytest -k test_integration", []string{"test_integration"}, ""},
		{"cd && jest -t", "cd frontend && jest -t MyComponent", []string{"MyComponent"}, ""},
		{"cd; cargo test -- filter", "cd /path; cargo test -- my_test", []string{"my_test"}, ""},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v -run TestBaz ./...", []string{"TestBaz"}, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			tests, timeout := parseTestInfoFromCommand(tc.command)
			if len(tests) != len(tc.expectTests) {
				t.Errorf("parseTestInfoFromCommand(%q) tests = %v, want %v", tc.command, tests, tc.expectTests)
			} else {
				for i := range tests {
					if tests[i] != tc.expectTests[i] {
						t.Errorf("parseTestInfoFromCommand(%q) tests[%d] = %q, want %q", tc.command, i, tests[i], tc.expectTests[i])
					}
				}
			}
			if timeout != tc.expectTimeout {
				t.Errorf("parseTestInfoFromCommand(%q) timeout = %q, want %q", tc.command, timeout, tc.expectTimeout)
			}
		})
	}
}

func TestExtractTestSegmentFromCommand(t *testing.T) {
	cases := []struct {
		name      string
		command   string
		expectSeg string
		expectCwd string
	}{
		{"simple go test", "go test ./...", "go test ./...", ""},
		{"cd && go test", "cd /workspaces/semio/repo/client && go test -v -run TestFoo ./...", "go test -v -run TestFoo ./...", "/workspaces/semio/repo/client"},
		{"cd && go test piped", "cd /path && go test -v ./... 2>&1 | tail -80", "go test -v ./... 2>&1", "/path"},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /src && go test -v ./...", "go test -v ./...", "/src"},
		{"cd && npm test", "cd frontend && npm test", "npm test", "frontend"},
		{"cd; cargo test", "cd /path; cargo test", "cargo test", "/path"},
		{"no test segment", "cd /path && go build ./...", "", "/path"},
		{"empty", "", "", ""},
		{"just cd", "cd /path", "", ""},
		{"piped go test", "go test -v ./... | head -50", "go test -v ./...", ""},
		{"vitest piped", "vitest run 2>&1 | tail -20", "vitest run 2>&1", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			seg, cwd := extractTestSegmentFromCommand(tc.command)
			if seg != tc.expectSeg {
				t.Errorf("extractTestSegmentFromCommand(%q) seg = %q, want %q", tc.command, seg, tc.expectSeg)
			}
			if cwd != tc.expectCwd {
				t.Errorf("extractTestSegmentFromCommand(%q) cwd = %q, want %q", tc.command, cwd, tc.expectCwd)
			}
		})
	}
}
