// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook classification and logging.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

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

// 🏛️classifyTool classifies a tool name into a ToolKind.
func classifyTool(toolName string) ToolKind {
	switch toolName {
	case "manage_todo_list", "Task", "task", "todo_tool", "TodoWrite":
		return ToolKindPlan
	case "read_file", "grep_search", "rg", "ripgrep", "file_search", "semantic_search", "list_dir", "list_code_usages", "get_errors", "Read", "fetch_webpage", "open_simple_browser", "Grep", "Glob",
		"fs_read", "code", "grep", "glob", "web_search", "web_fetch":
		return ToolKindCodeSearch
	case "replace_string_in_file", "create_file", "multi_replace_string_in_file", "Edit", "Write", "editfile",
		"fs_write":
		return ToolKindCodeEdit
	case "run_in_terminal", "get_terminal_output", "Bash", "terminal",
		"execute_bash":
		return ToolKindTerminal
	case "runTests", "run_tests":
		return ToolKindTest
	case "run_task", "create_and_run_task":
		return ToolKindBuild
	case "tool_search_tool_regex":
		return ToolKindGeneric
	case "use_subagent", "use_aws":
		return ToolKindGeneric
	default:
		return ToolKindGeneric
	}
}

// 🔤️classifyCommandKind classifies a command string into a ToolKind.
func classifyCommandKind(command string) ToolKind {
	trimmed := strings.TrimSpace(command)
	if trimmed == "" {
		return ToolKindTerminal
	}
	segments := splitCommandSegments(trimmed)
	if len(segments) > 1 {
		best := ToolKindTerminal
		for _, segment := range segments {
			kind := classifyCommandKind(segment)
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

// 🔬️parseTestInfoFromCommand parses test information from a command string.
// ⏰️Returns a slice of test names/patterns and a timeout duration string.
func parseTestInfoFromCommand(command string) ([]string, string) {
	segment, _ := extractTestSegmentFromCommand(command)
	if segment != "" {
		command = segment
	}
	command = strings.TrimSpace(command)
	fields := strings.Fields(command)
	var tests []string
	timeout := ""

	if strings.Contains(command, "-timeout ") {
		parts := strings.Split(command, "-timeout ")
		if len(parts) > 1 {
			timeoutPart := strings.Fields(parts[1])[0]
			if strings.HasSuffix(timeoutPart, "s") {
				timeout = strings.TrimSuffix(timeoutPart, "s")
			} else if strings.HasSuffix(timeoutPart, "m") {
				mins := strings.TrimSuffix(timeoutPart, "m")
				if minsInt, err := strconv.Atoi(mins); err == nil {
					timeout = strconv.Itoa(minsInt * 60)
				}
			} else {
				timeout = timeoutPart
			}
		}
	} else if strings.Contains(command, "--timeout ") {
		parts := strings.Split(command, "--timeout ")
		if len(parts) > 1 {
			timeout = strings.Fields(parts[1])[0]
		}
	}
	flagValue := func(flags ...string) string {
		for i := 0; i < len(fields); i++ {
			for _, flag := range flags {
				if fields[i] == flag && i+1 < len(fields) {
					return fields[i+1]
				}
				prefix := flag + "="
				if strings.HasPrefix(fields[i], prefix) {
					return strings.TrimPrefix(fields[i], prefix)
				}
			}
		}
		return ""
	}
	joinAfterMarker := func(marker string) string {
		for i := 0; i < len(fields); i++ {
			if fields[i] == marker && i+1 < len(fields) {
				return fields[i+1]
			}
		}
		return ""
	}

	switch {
	case strings.HasPrefix(command, "go test"):
		if value := flagValue("-run"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "pytest"), strings.HasPrefix(command, "python -m pytest"), strings.HasPrefix(command, "python3 -m pytest"), strings.HasPrefix(command, "uv run pytest"), strings.HasPrefix(command, "uvx pytest"):
		if value := flagValue("-k"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "jest"), strings.HasPrefix(command, "npx jest"), strings.HasPrefix(command, "bunx jest"), strings.HasPrefix(command, "pnpx jest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "mocha"), strings.HasPrefix(command, "npx mocha"), strings.HasPrefix(command, "bunx mocha"), strings.HasPrefix(command, "pnpx mocha"):
		if value := flagValue("--grep"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo test"):
		if value := joinAfterMarker("--"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo nextest"):
		tests = []string{""}
	case strings.HasPrefix(command, "dotnet test"):
		if value := flagValue("--filter"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "rspec"), strings.HasPrefix(command, "bundle exec rspec"):
		if value := flagValue("--example"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "vitest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "phpunit"), strings.Contains(command, "npm test"), strings.Contains(command, "pnpm test"), strings.Contains(command, "yarn test"), strings.Contains(command, "bun test"):
		tests = []string{""}
	default:
		tests = []string{""}
	}

	return tests, timeout
}

// 🏷️mergeTicketAgentPlanSteps merges ticket agent plan steps.
func mergeTicketAgentPlanSteps(existing []TicketAgentPlanStep, newSteps []HookPlanStep, second string) []TicketAgentPlanStep {
	existingByName := map[string]TicketAgentPlanStep{}
	for _, step := range existing {
		existingByName[step.Name] = step
	}
	activeNames := map[string]struct{}{}
	result := make([]TicketAgentPlanStep, 0, len(existing)+len(newSteps))
	for _, incoming := range newSteps {
		name := strings.TrimSpace(incoming.Name)
		if name == "" {
			continue
		}
		step := existingByName[name]
		step.Name = name
		step.Status = incoming.Status
		if step.Ideated == "" {
			step.Ideated = second
		}
		switch strings.TrimSpace(strings.ToLower(incoming.Status)) {
		case "in-progress", "in_progress", "started":
			if step.Started == "" {
				step.Started = second
			}
		case "completed", "done":
			if step.Started != "" && step.Completed == "" {
				step.Completed = second
			}
		}
		activeNames[name] = struct{}{}
		result = append(result, step)
	}
	for _, step := range existing {
		if _, ok := activeNames[step.Name]; ok {
			continue
		}
		if step.Name == "" {
			continue
		}
		if step.Abandoned != "" || step.Completed != "" || step.Started != "" {
			result = append(result, step)
			continue
		}
		if step.Ideated == "" {
			step.Ideated = second
		}
		step.Abandoned = second
		result = append(result, step)
	}
	return result
}

func extractHookResultToolInfo(result interface{}) (string, string) {
	switch value := result.(type) {
	case HookResultAgentToolStarting:
		return value.Name, ""
	case HookResultAgentToolEnded:
		return value.Name, ""
	case HookResultAgentToolTerminalStarting:
		return value.Name, value.Command
	case HookResultAgentToolTerminalEnded:
		return value.Name, value.Command
	default:
		return "", ""
	}
}

// ➡️appendUniqueString appends a string to a slice if it's not already present.
func appendUniqueString(slice []string, str string) []string {
	for _, item := range slice {
		if item == str {
			return slice
		}
	}
	return append(slice, str)
}

// 📜️HookLogEntry is an alias for EventEntry for hook log entries.
type HookLogEntry = EventEntry

// 🔺️deriveRepoOpFromMCPTool derives repo operation from MCP tool.
func deriveRepoOpFromMCPTool(tool string) string {
	if tool == "" {
		return ""
	}
	const prefix = "mcp__repo__"
	if !strings.HasPrefix(tool, prefix) {
		return ""
	}
	operation := strings.TrimPrefix(tool, prefix)
	operation = strings.TrimSpace(operation)
	if operation == "" {
		return ""
	}
	return strings.ReplaceAll(operation, "_", ".")
}

// 🔻️deriveRepoOpFromCLICommand derives repo operation from CLI command.
func deriveRepoOpFromCLICommand(cmd string) string {
	if cmd == "" {
		return ""
	}
	fields := strings.Fields(cmd)
	if len(fields) < 2 {
		return ""
	}
	cliIndex := -1
	for i, field := range fields {
		base := filepath.Base(strings.ReplaceAll(field, "\\", "/"))
		if base == "cli" || base == "cli.exe" || base == "client" || base == "client.exe" {
			cliIndex = i
			break
		}
	}
	if cliIndex == -1 {
		for i := 0; i < len(fields)-1; i++ {
			if fields[i] == "go" && fields[i+1] == "run" {
				for j := i + 2; j < len(fields); j++ {
					base := filepath.Base(strings.ReplaceAll(fields[j], "\\", "/"))
					if base == "cli" || base == "cli.exe" || base == "client" || base == "client.exe" || fields[j] == "./repo/client/mcp/go" || fields[j] == "repo/client/mcp/go" {
						cliIndex = j
						break
					}
				}
			}
			if cliIndex != -1 {
				break
			}
		}
	}
	if cliIndex == -1 || cliIndex+1 >= len(fields) {
		return ""
	}
	remaining := fields[cliIndex+1:]
	if len(remaining) == 0 || strings.HasPrefix(remaining[0], "-") {
		return ""
	}
	if len(remaining) >= 2 {
		compound := remaining[0] + "." + remaining[1]
		switch compound {
		case "ticket.open", "ticket.close", "ticket.reopen", "ticket.read",
			"goal.open", "goal.close", "goal.reopen",
			"contributor.add", "contributor.remove",
			"draft.create", "draft.delete",
			"file.create", "file.move", "file.delete",
			"folder.create", "folder.move", "folder.delete",
			"section.create", "section.move", "section.delete",
			"policy.check":
			return compound
		}
	}
	switch remaining[0] {
	case "integrate", "extract", "export", "analyze", "fix", "tree", "graphql", "move":
		return remaining[0]
	default:
		return ""
	}
}

// ⬛️logRepoOperationHook logs repo operation hook.
func logRepoOperationHook(hctx HookContext, result interface{}, operation string, t time.Time, msg string, lg LoggingConfig) {
	if !lg.Operations {
		return
	}
	logDir := strings.TrimSpace(operation)
	sessionID := strings.TrimSpace(msg)
	if logDir == "" || sessionID == "" {
		return
	}
	resultToolName, resultCommand := extractHookResultToolInfo(result)
	toolName := firstNonEmpty(resultToolName, hctx.ToolName, extractToolNameFromStdin(hctx.Input))
	repoOperation := deriveRepoOpFromMCPTool(toolName)
	if repoOperation == "" {
		repoOperation = deriveRepoOpFromCLICommand(firstNonEmpty(resultCommand, hctx.ToolArgs, extractCommandFromStdin(hctx.Input)))
	}
	if repoOperation == "" {
		return
	}
	phase := "starting"
	switch hctx.Event {
	case HookAgentToolEnded, HookAgentToolPlanUpdatingEnded, HookAgentToolSearchEnded, HookAgentToolCodeEditEnded, HookAgentToolTestEnded, HookAgentToolBuildEnded, HookAgentToolTerminalEnded:
		phase = "ended"
	}
	metaPath := filepath.Join(logDir, "session.json")
	var meta SessionMeta
	if data, err := os.ReadFile(metaPath); err == nil {
		_ = json.Unmarshal(data, &meta)
	}
	inputMap := decodeHookInputMap(hctx.Input)
	second := firstNonEmpty(findNestedStringValue(inputMap, "second"), hctx.Second, t.UTC().Format(time.RFC3339))
	checkpoint := resolveEventCheckpointID(extractCheckpointSHAFromInput(hctx.Input))
	eventJSON, _ := json.Marshal(map[string]interface{}{
		"kind":        "agent." + repoOperation + "." + phase,
		"client":      hctx.Client,
		"session":     resolveEventSessionID(sessionID),
		"second":      resolveEventSecondID(second),
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	})
	entry := EventEntry{Event: eventJSON}
	meta.Events = append(meta.Events, entry)
	_ = WriteJSONFile(metaPath, meta)
}
