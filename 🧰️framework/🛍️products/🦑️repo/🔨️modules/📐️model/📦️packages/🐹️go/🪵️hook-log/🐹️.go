// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪵️hook_log carries the session metadata, event entries and command classification of the hook log.

// #endregion 🧲️Header

package model

import (
	json "encoding/json"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🪵️Hook Log

// 🔸️SessionMeta holds session metadata.
type SessionMeta struct {
	ID          string           `json:"id"`
	URI         string           `json:"uri,omitempty"`
	Client      string           `json:"client,omitempty"`
	Second      string           `json:"second,omitempty"`
	Checkpoint  string           `json:"checkpoint,omitempty"`
	Contributor string           `json:"contributor,omitempty"`
	Transcript  string           `json:"transcript,omitempty"`
	Events      []EventEntry     `json:"events,omitempty"`
	Plan        *TicketAgentPlan `json:"plan,omitempty"`
}

// 📍️EventEntry represents an event entry in session metadata.
type EventEntry struct {
	Event  json.RawMessage `json:"event"`
	Native *struct {
		Event json.RawMessage `json:"event"`
	} `json:"native,omitempty"`
	Response *struct {
		Blocked *bool  `json:"blocked,omitempty"`
		Message string `json:"message,omitempty"`
		Reason  string `json:"reason,omitempty"`
	} `json:"response,omitempty"`
}

// 🔤️classifyCommandKind classifies a command string into a ToolKind.
func ClassifyCommandKind(command string) ToolKind {
	trimmed := strings.TrimSpace(command)
	if trimmed == "" {
		return ToolKindTerminal
	}
	segments := workspace.SplitCommandSegments(trimmed)
	if len(segments) > 1 {
		best := ToolKindTerminal
		for _, segment := range segments {
			kind := ClassifyCommandKind(segment)
			if kind == ToolKindTest || kind == ToolKindBuild || kind == ToolKindCodeEdit || kind == ToolKindPlan {
				return kind
			}
			if kind == ToolKindCodeSearch {
				best = ToolKindCodeSearch
			}
		}
		if best != ToolKindTerminal {
			return best
		}
	}

	parts := strings.Fields(trimmed)
	if len(parts) == 0 {
		return ToolKindTerminal
	}

	cmd := parts[0]
	baseCmd := filepath.Base(cmd)
	if baseCmd == "test" || baseCmd == "[" {
		return ToolKindCodeSearch
	}
	if strings.HasPrefix(baseCmd, "phpunit") || baseCmd == "rspec" || baseCmd == "pytest" || baseCmd == "py.test" || baseCmd == "jest" || baseCmd == "vitest" || baseCmd == "mocha" || baseCmd == "gradle" || baseCmd == "mvn" || baseCmd == "bundle" {
		return ToolKindTest
	}
	if strings.Contains(trimmed, "cargo nextest") || strings.HasPrefix(trimmed, "cargo nextest") {
		return ToolKindTest
	}
	testCmds := []string{"pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"}
	for _, testCmd := range testCmds {
		if strings.Contains(trimmed, testCmd) {
			return ToolKindTest
		}
	}

	if strings.HasPrefix(trimmed, "npm test") || strings.HasPrefix(trimmed, "npm run test") ||
		strings.HasPrefix(trimmed, "pnpm test") || strings.HasPrefix(trimmed, "yarn test") ||
		strings.HasPrefix(trimmed, "bun test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "go test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "cargo test") || strings.HasPrefix(trimmed, "cargo nextest") {
		return ToolKindTest
	}
	if strings.Contains(trimmed, "pytest") || strings.Contains(trimmed, "unittest") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "make test") || strings.HasPrefix(trimmed, "make check") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "dotnet test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "swift test") || strings.HasPrefix(trimmed, "dart test") ||
		strings.HasPrefix(trimmed, "flutter test") || strings.HasPrefix(trimmed, "mix test") ||
		strings.HasPrefix(trimmed, "mvn test") || strings.HasPrefix(trimmed, "mvn verify") ||
		strings.HasPrefix(trimmed, "gradle test") || strings.HasPrefix(trimmed, "./gradlew test") || strings.HasPrefix(trimmed, "gradlew test") ||
		strings.HasPrefix(trimmed, "cabal test") || strings.HasPrefix(trimmed, "stack test") ||
		strings.HasPrefix(trimmed, "lein test") || strings.HasPrefix(trimmed, "sbt test") ||
		strings.HasPrefix(trimmed, "bundle exec rspec") || strings.HasPrefix(trimmed, "tox") || strings.Contains(trimmed, "phpunit") || strings.HasPrefix(trimmed, "rspec ") {
		return ToolKindTest
	}

	searchCmds := []string{"grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind",
		"locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less",
		"more", "head", "tail", "wc", "file", "stat", "du", "which", "whereis", "type", "command",
		"hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath",
		"basename", "dirname", "jq", "yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column",
		"rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env", "printenv",
		"set", "export", "pwd", "id", "whoami", "hostname", "uname", "date", "uptime", "free",
		"df", "ps", "top", "htop", "lsof", "netstat", "ss", "test"}
	for _, searchCmd := range searchCmds {
		if baseCmd == searchCmd {
			return ToolKindCodeSearch
		}
	}
	if baseCmd == "sed" && !strings.Contains(trimmed, "-i") {
		return ToolKindCodeSearch
	}
	if (baseCmd == "awk" || baseCmd == "gawk" || baseCmd == "mawk" || baseCmd == "nawk") && !strings.Contains(trimmed, "-i") {
		return ToolKindCodeSearch
	}

	editCmds := []string{"rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown",
		"chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip",
		"gunzip", "bzip2", "bunzip2", "xz", "unxz", "zstd"}
	for _, editCmd := range editCmds {
		if baseCmd == editCmd {
			return ToolKindCodeEdit
		}
	}
	if baseCmd == "sed" && strings.Contains(trimmed, "-i") {
		return ToolKindCodeEdit
	}
	if (baseCmd == "awk" || baseCmd == "gawk" || baseCmd == "mawk" || baseCmd == "nawk") && strings.Contains(trimmed, "-i") {
		return ToolKindCodeEdit
	}

	return ToolKindTerminal
}

// #endregion 🪵️Hook Log
