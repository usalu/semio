// 🦀️ Tests of the hook events and results.

package model

import (
	json "encoding/json"
	testing "testing"
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

func TestAllHookEventsCompleteness(t *testing.T) {
	expected := []HookEvent{
		HookVersionCheckpointStarting, HookVersionCheckpointEnded,
		HookVersionCheckinStarting, HookVersionCheckinEnded,
		HookVersionCheckoutStarting, HookVersionCheckoutEnded,
		HookAgentStarted, HookAgentEnded,
		HookAgentPromptSubmitting, HookAgentCompacting,
		HookAgentToolStarting, HookAgentToolEnded,
		HookAgentToolPlanUpdatingStarting, HookAgentToolPlanUpdatingEnded,
		HookAgentToolSearchStarting, HookAgentToolSearchEnded,
		HookAgentToolCodeEditStarting, HookAgentToolCodeEditEnded,
		HookAgentToolTestStarting, HookAgentToolTestEnded,
		HookAgentToolBuildStarting, HookAgentToolBuildEnded,
		HookAgentToolTerminalStarting, HookAgentToolTerminalEnded,
		HookAgentThinkingStarting, HookAgentThinkingEnded,
	}
	if len(AllHookEvents) != len(expected) {
		t.Errorf("expected %d events, got %d", len(expected), len(AllHookEvents))
	}
	for _, e := range expected {
		found := false
		for _, a := range AllHookEvents {
			if a == e {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing event: %s", e)
		}
	}
}

func TestClassifyCommandKind(t *testing.T) {
	cases := []struct {
		name    string
		command string
		expect  ToolKind
	}{
		{"empty", "", ToolKindTerminal},
		{"whitespace only", "   ", ToolKindTerminal},
		{"grep", "grep -r foo .", ToolKindCodeSearch},
		{"grep with path", "/usr/bin/grep -rn pattern file.go", ToolKindCodeSearch},
		{"rg", "rg --type go pattern", ToolKindCodeSearch},
		{"ripgrep", "ripgrep --type go pattern", ToolKindCodeSearch},
		{"ag", "ag pattern src/", ToolKindCodeSearch},
		{"ack", "ack --go pattern", ToolKindCodeSearch},
		{"ack-grep", "ack-grep pattern", ToolKindCodeSearch},
		{"find", "find . -name '*.go'", ToolKindCodeSearch},
		{"fd", "fd -e go pattern", ToolKindCodeSearch},
		{"fdfind", "fdfind pattern", ToolKindCodeSearch},
		{"locate", "locate main.go", ToolKindCodeSearch},
		{"mlocate", "mlocate something", ToolKindCodeSearch},
		{"ls", "ls -la", ToolKindCodeSearch},
		{"ls piped", "ls .🦑️repo/ | head -20", ToolKindCodeSearch},
		{"ls with redirect", "ls .🦑️repo/🎯️/ 2>/dev/null | head -20 || ls .🦑️repo/ | head -20", ToolKindCodeSearch},
		{"exa", "exa --long", ToolKindCodeSearch},
		{"eza", "eza --tree", ToolKindCodeSearch},
		{"tree", "tree -L 2", ToolKindCodeSearch},
		{"dir", "dir /w", ToolKindCodeSearch},
		{"cat", "cat file.txt", ToolKindCodeSearch},
		{"bat", "bat --style=numbers file.go", ToolKindCodeSearch},
		{"batcat", "batcat file.go", ToolKindCodeSearch},
		{"less", "less file.txt", ToolKindCodeSearch},
		{"more", "more file.txt", ToolKindCodeSearch},
		{"head", "head -20 file.txt", ToolKindCodeSearch},
		{"tail", "tail -f log.txt", ToolKindCodeSearch},
		{"wc", "wc -l file.txt", ToolKindCodeSearch},
		{"file", "file binary.dat", ToolKindCodeSearch},
		{"stat", "stat file.txt", ToolKindCodeSearch},
		{"du", "du -sh .", ToolKindCodeSearch},
		{"which", "which go", ToolKindCodeSearch},
		{"whereis", "whereis python", ToolKindCodeSearch},
		{"type", "type ls", ToolKindCodeSearch},
		{"command", "command -v node", ToolKindCodeSearch},
		{"hash", "hash -r", ToolKindCodeSearch},
		{"diff", "diff file1.txt file2.txt", ToolKindCodeSearch},
		{"cmp", "cmp file1 file2", ToolKindCodeSearch},
		{"comm", "comm sorted1 sorted2", ToolKindCodeSearch},
		{"strings", "strings binary", ToolKindCodeSearch},
		{"od", "od -x file", ToolKindCodeSearch},
		{"xxd", "xxd file", ToolKindCodeSearch},
		{"hexdump", "hexdump -C file", ToolKindCodeSearch},
		{"readlink", "readlink -f symlink", ToolKindCodeSearch},
		{"realpath", "realpath relative/path", ToolKindCodeSearch},
		{"basename", "basename /path/to/file.txt", ToolKindCodeSearch},
		{"dirname", "dirname /path/to/file.txt", ToolKindCodeSearch},
		{"jq", "jq '.name' package.json", ToolKindCodeSearch},
		{"yq", "yq '.spec' config.yaml", ToolKindCodeSearch},
		{"xq", "xq '.root' config.xml", ToolKindCodeSearch},
		{"sort", "sort file.txt", ToolKindCodeSearch},
		{"uniq", "uniq -c file.txt", ToolKindCodeSearch},
		{"cut", "cut -d: -f1 /etc/passwd", ToolKindCodeSearch},
		{"tr", "tr '[:lower:]' '[:upper:]'", ToolKindCodeSearch},
		{"paste", "paste file1 file2", ToolKindCodeSearch},
		{"column", "column -t file.txt", ToolKindCodeSearch},
		{"rev", "rev file.txt", ToolKindCodeSearch},
		{"fold", "fold -w 80 file.txt", ToolKindCodeSearch},
		{"fmt", "fmt -w 72 file.txt", ToolKindCodeSearch},
		{"nl", "nl file.txt", ToolKindCodeSearch},
		{"expand", "expand file.txt", ToolKindCodeSearch},
		{"unexpand", "unexpand file.txt", ToolKindCodeSearch},
		{"echo", "echo hello", ToolKindCodeSearch},
		{"printf", "printf '%s\\n' hello", ToolKindCodeSearch},
		{"env", "env", ToolKindCodeSearch},
		{"printenv", "printenv HOME", ToolKindCodeSearch},
		{"set", "set", ToolKindCodeSearch},
		{"export", "export FOO=bar", ToolKindCodeSearch},
		{"pwd", "pwd", ToolKindCodeSearch},
		{"id", "id", ToolKindCodeSearch},
		{"whoami", "whoami", ToolKindCodeSearch},
		{"hostname", "hostname", ToolKindCodeSearch},
		{"uname", "uname -a", ToolKindCodeSearch},
		{"date", "date +%Y-%m-%d", ToolKindCodeSearch},
		{"uptime", "uptime", ToolKindCodeSearch},
		{"free", "free -h", ToolKindCodeSearch},
		{"df", "df -h", ToolKindCodeSearch},
		{"ps", "ps aux", ToolKindCodeSearch},
		{"top", "top -bn1", ToolKindCodeSearch},
		{"htop", "htop", ToolKindCodeSearch},
		{"lsof", "lsof -i :8080", ToolKindCodeSearch},
		{"netstat", "netstat -tlnp", ToolKindCodeSearch},
		{"ss", "ss -tlnp", ToolKindCodeSearch},
		{"test", "test -f file.txt", ToolKindCodeSearch},
		{"bracket test", "[ -f file.txt ]", ToolKindCodeSearch},
		{"sed read-only (no -i)", "sed 's/old/new/g' file.txt", ToolKindCodeSearch},
		{"awk read-only", "awk '{print $1}' file.txt", ToolKindCodeSearch},
		{"gawk read-only", "gawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"mawk read-only", "mawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"nawk read-only", "nawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"rm", "rm -rf temp/", ToolKindCodeEdit},
		{"mv", "mv old.txt new.txt", ToolKindCodeEdit},
		{"cp", "cp src.txt dst.txt", ToolKindCodeEdit},
		{"install", "install -m 755 bin /usr/local/bin/", ToolKindCodeEdit},
		{"mkdir", "mkdir -p new/dir", ToolKindCodeEdit},
		{"rmdir", "rmdir empty/", ToolKindCodeEdit},
		{"touch", "touch new.txt", ToolKindCodeEdit},
		{"chmod", "chmod 644 file.txt", ToolKindCodeEdit},
		{"chown", "chown user:group file.txt", ToolKindCodeEdit},
		{"chgrp", "chgrp group file.txt", ToolKindCodeEdit},
		{"ln", "ln -s target link", ToolKindCodeEdit},
		{"tee", "tee output.log", ToolKindCodeEdit},
		{"patch", "patch -p1 < fix.patch", ToolKindCodeEdit},
		{"truncate", "truncate -s 0 file.log", ToolKindCodeEdit},
		{"dd", "dd if=/dev/zero of=file bs=1M count=1", ToolKindCodeEdit},
		{"shred", "shred -u file.txt", ToolKindCodeEdit},
		{"tar", "tar -xzf archive.tar.gz", ToolKindCodeEdit},
		{"zip", "zip archive.zip file.txt", ToolKindCodeEdit},
		{"unzip", "unzip archive.zip", ToolKindCodeEdit},
		{"gzip", "gzip file.txt", ToolKindCodeEdit},
		{"gunzip", "gunzip file.txt.gz", ToolKindCodeEdit},
		{"bzip2", "bzip2 file.txt", ToolKindCodeEdit},
		{"bunzip2", "bunzip2 file.txt.bz2", ToolKindCodeEdit},
		{"xz", "xz file.txt", ToolKindCodeEdit},
		{"unxz", "unxz file.txt.xz", ToolKindCodeEdit},
		{"zstd", "zstd file.txt", ToolKindCodeEdit},
		{"git", "git status", ToolKindTerminal},
		{"npm install", "npm install", ToolKindTerminal},
		{"npm test", "npm test", ToolKindTest},
		{"npm run test", "npm run test", ToolKindTest},
		{"pnpm test", "pnpm test", ToolKindTest},
		{"yarn test", "yarn test", ToolKindTest},
		{"bun test", "bun test", ToolKindTest},
		{"npx vitest run", "npx vitest run", ToolKindTest},
		{"npx jest", "npx jest", ToolKindTest},
		{"npx mocha", "npx mocha", ToolKindTest},
		{"go build", "go build ./...", ToolKindTerminal},
		{"go test all", "go test ./...", ToolKindTest},
		{"go test specific", "go test -run TestFoo ./...", ToolKindTest},
		{"cargo build", "cargo build", ToolKindTerminal},
		{"cargo test", "cargo test", ToolKindTest},
		{"cargo nextest", "cargo nextest run", ToolKindTest},
		{"pip", "pip install requests", ToolKindTerminal},
		{"uv pip install", "uv pip install requests", ToolKindTerminal},
		{"uv run pytest", "uv run pytest", ToolKindTest},
		{"uvx pytest", "uvx pytest", ToolKindTest},
		{"python -m pytest", "python -m pytest", ToolKindTest},
		{"python -m unittest", "python -m unittest", ToolKindTest},
		{"python3 script", "python3 script.py", ToolKindTerminal},
		{"node", "node script.js", ToolKindTerminal},
		{"make build", "make build", ToolKindTerminal},
		{"make test", "make test", ToolKindTest},
		{"make check", "make check", ToolKindTest},
		{"dotnet build", "dotnet build", ToolKindTerminal},
		{"dotnet test", "dotnet test", ToolKindTest},
		{"swift test", "swift test", ToolKindTest},
		{"dart test", "dart test", ToolKindTest},
		{"flutter test", "flutter test", ToolKindTest},
		{"mix test", "mix test", ToolKindTest},
		{"mvn test", "mvn test", ToolKindTest},
		{"mvn verify", "mvn verify", ToolKindTest},
		{"gradle test", "gradle test", ToolKindTest},
		{"gradlew test", "./gradlew test", ToolKindTest},
		{"cabal test", "cabal test", ToolKindTest},
		{"stack test", "stack test", ToolKindTest},
		{"lein test", "lein test", ToolKindTest},
		{"sbt test", "sbt test", ToolKindTest},
		{"bundle exec rspec", "bundle exec rspec", ToolKindTest},
		{"jest direct", "jest --testPathPattern=foo", ToolKindTest},
		{"vitest direct", "vitest run", ToolKindTest},
		{"mocha direct", "mocha test/", ToolKindTest},
		{"pytest direct", "pytest -k test_foo", ToolKindTest},
		{"tox direct", "tox -e py311", ToolKindTest},
		{"rspec direct", "rspec spec/", ToolKindTest},
		{"phpunit direct", "phpunit tests/", ToolKindTest},
		{"phpunit vendor", "./vendor/bin/phpunit tests/", ToolKindTest},
		{"ctest direct", "ctest --test-dir build/", ToolKindTest},
		{"bats direct", "bats tests/", ToolKindTest},
		{"docker", "docker build .", ToolKindTerminal},
		{"kubectl", "kubectl get pods", ToolKindTerminal},
		{"curl", "curl https://example.com", ToolKindTerminal},
		{"wget", "wget https://example.com", ToolKindTerminal},
		{"ssh", "ssh user@host", ToolKindTerminal},
		{"custom-tool", "./custom-tool --flag", ToolKindTerminal},

		{"cd && go test", "cd /workspaces/semio/repo/client && go test ./...", ToolKindTest},
		{"cd && go test piped", "cd /workspaces/semio/repo/client && go test -v -run TestFoo -timeout 60s 2>&1 | tail -80", ToolKindTest},
		{"cd && cargo test", "cd /path && cargo test", ToolKindTest},
		{"cd && npm test", "cd technology && npm test", ToolKindTest},
		{"cd && pytest", "cd tests && pytest -k test_foo", ToolKindTest},
		{"cd && python -m pytest", "cd /app && python -m pytest", ToolKindTest},
		{"cd && dotnet test", "cd /app && dotnet test", ToolKindTest},
		{"cd && jest", "cd frontend && jest --testPathPattern=foo", ToolKindTest},
		{"cd && vitest", "cd app && vitest run", ToolKindTest},

		{"cd && go build", "cd /path && go build ./...", ToolKindTerminal},
		{"cd && npm install", "cd /path && npm install", ToolKindTerminal},

		{"cd; go test", "cd /path; go test ./...", ToolKindTest},

		{"go test || echo", "go test ./... || echo 'failed'", ToolKindTest},

		{"go test piped", "go test -v ./... | head -50", ToolKindTest},
		{"cargo test piped", "cargo test 2>&1 | tail -20", ToolKindTest},

		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v ./...", ToolKindTest},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := ClassifyCommandKind(tc.command)
			if result != tc.expect {
				t.Errorf("classifyCommandKind(%q) = %s, want %s", tc.command, result, tc.expect)
			}
		})
	}
}

func TestHookResultJSONFields(t *testing.T) {
	result := HookResultAgentToolTerminalEnded{
		HookResultAgentBase: HookResultAgentBase{
			HookResultBase: HookResultBase{
				Allowed: true,
				Raw:     json.RawMessage(`{"raw":"data"}`),
			},
			Session:   "sess-1",
			Second:    "2026-02-19T10:00:00Z",
			Client:    "copilot-chat",
			MessageID: "msg-123",
		},
		Command:    "npm test",
		PID:        12345,
		Terminated: true,
		Stdout:     json.RawMessage(`"passed"`),
		Stderr:     json.RawMessage(`"warn"`),
	}
	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON, got: %v", err)
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	expectedKeys := []string{"allowed", "message", "session", "second", "client", "command", "pid", "terminated", "stdout", "stderr"}
	for _, key := range expectedKeys {
		if _, ok := parsed[key]; !ok {
			t.Errorf("missing JSON key: %s", key)
		}
	}
}

func TestHookResultOmitEmpty(t *testing.T) {
	result := HookResultAgentStarted{
		HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}},
	}
	data, _ := json.Marshal(result)
	var parsed map[string]interface{}
	json.Unmarshal(data, &parsed)
	if _, ok := parsed["session"]; ok {
		t.Error("expected session to be omitted when empty")
	}
	if _, ok := parsed["raw"]; ok {
		t.Error("expected raw to be omitted when nil")
	}
}
