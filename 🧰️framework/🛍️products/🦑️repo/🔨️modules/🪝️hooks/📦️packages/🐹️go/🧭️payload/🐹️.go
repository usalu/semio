// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧭️payload carries the hook payload readers: nested lookups, tool, command, session, checkpoint and search extraction, and the plan steps.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	todos "github.com/usalu/semio/repo/todos"
)

// #region 🧭️PayloadReading

// 🔍️DecodeHookInput answers the payload object, when the input is a JSON object at all.
func DecodeHookInput(input json.RawMessage) map[string]interface{} {
	return decodeHookInputMap(input)
}

// 🪆️FindNestedString answers the first non-blank string reachable under any of keys, searched
// breadth-first at each level then depth-first into every child, visiting children in key order.
func FindNestedString(input json.RawMessage, keys ...string) string {
	if len(input) == 0 {
		return ""
	}
	var decoded interface{}
	if err := json.Unmarshal(input, &decoded); err != nil {
		return ""
	}
	return findNestedOrdered(decoded, keys)
}

func findNestedOrdered(value interface{}, keys []string) string {
	switch typed := value.(type) {
	case map[string]interface{}:
		for _, key := range keys {
			if raw, ok := typed[key].(string); ok && strings.TrimSpace(raw) != "" {
				return strings.TrimSpace(raw)
			}
		}
		names := make([]string, 0, len(typed))
		for name := range typed {
			names = append(names, name)
		}
		sort.Strings(names)
		for _, name := range names {
			if found := findNestedOrdered(typed[name], keys); found != "" {
				return found
			}
		}
	case []interface{}:
		for _, nested := range typed {
			if found := findNestedOrdered(nested, keys); found != "" {
				return found
			}
		}
	}
	return ""
}

// 🗺️ExtractToolInputMap answers the `tool_input` object, at the top level or under `event` or `native.event`.
func ExtractToolInputMap(input json.RawMessage) map[string]interface{} {
	return extractToolInputMapFromData(decodeHookInputMap(input))
}

// 🔹️ExtractCommand answers the shell command the payload carries.
func ExtractCommand(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var literal string
	if err := json.Unmarshal(input, &literal); err == nil {
		return strings.TrimSpace(literal)
	}
	decoded := decodeHookInputMap(input)
	if decoded == nil {
		return ""
	}
	if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
		if value, ok := toolInput["command"].(string); ok {
			return strings.TrimSpace(value)
		}
	}
	return FindNestedString(input, "command", "command_line", "commandLine")
}

// 📁️ExtractCommandAndCwd answers the command and the working directory it runs in; an absent
// directory stays empty rather than silently becoming the host's cwd.
func ExtractCommandAndCwd(input json.RawMessage) (string, string) {
	command := ExtractCommand(input)
	cwd := ""
	if toolInput := ExtractToolInputMap(input); len(toolInput) > 0 {
		if value, ok := toolInput["cwd"].(string); ok {
			cwd = strings.TrimSpace(value)
		}
	}
	if cwd == "" {
		cwd = FindNestedString(input, "cwd", "working_directory", "workingDirectory")
	}
	return command, cwd
}

// 🔷️ExtractToolName answers the tool name the payload names.
func ExtractToolName(input json.RawMessage) string {
	return FindNestedString(input, "tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName")
}

// 📡️ExtractHookEventName answers the native hook event name the payload names.
func ExtractHookEventName(input json.RawMessage) string {
	return FindNestedString(input, "hookEventName", "hook_event_name", "hook_event", "hookEvent", "event")
}

// 🧾️ExtractTranscript answers the transcript path the payload names.
func ExtractTranscript(input json.RawMessage) string {
	return FindNestedString(input, "transcript", "transcript_path", "transcriptPath", "log_path", "logPath")
}

// 🧠️ExtractLLM answers the model the payload names.
func ExtractLLM(input json.RawMessage) string {
	return FindNestedString(input, "llm", "model", "model_name", "modelName")
}

// 🏋️ExtractEffort answers the reasoning effort the payload names.
func ExtractEffort(input json.RawMessage) string {
	return FindNestedString(input, "effort", "reasoning_effort", "reasoningEffort")
}

func topLevelString(input json.RawMessage, keys ...string) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	for _, key := range keys {
		if raw, ok := data[key].(string); ok && raw != "" {
			return strings.TrimSpace(raw)
		}
	}
	return ""
}

// ▪️ExtractMessageID answers the message identifier of the turn.
func ExtractMessageID(input json.RawMessage) string {
	return topLevelString(input, "messageId", "message_id", "turnId", "turn_id", "requestId", "request_id")
}

// ▫️ExtractParentMessageID answers the message identifier of the parent turn.
func ExtractParentMessageID(input json.RawMessage) string {
	return topLevelString(input, "parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id")
}

// 🪪️ExtractSessionID answers the session identifier, falling back to the basename of the transcript path.
func ExtractSessionID(input json.RawMessage) string {
	session := FindNestedString(input, "session_id", "sessionId", "trajectory_id", "trajectoryId", "conversation_id", "conversationId", "agent_id", "agentId")
	if session != "" {
		return session
	}
	transcript := ExtractTranscript(input)
	if transcript == "" {
		return ""
	}
	base := BaseName(strings.TrimSpace(transcript))
	if index := strings.LastIndex(base, "."); index > 0 {
		return strings.TrimSpace(base[:index])
	}
	return strings.TrimSpace(base)
}

// 🪪️NormalizeParentSessionID normalises a parent hint: the literal `subagent` is a shape, not a session id.
func NormalizeParentSessionID(parent string) string { return normalizeParentSessionID(parent) }

// 👪️ExtractParent answers the parent session the payload names, at the top level, under `event` or under `native`.
func ExtractParent(input json.RawMessage) string { return extractParentFromInput(input) }

// 🔳️ResolveParentSessionID answers the parent session: the caller's hint when it names one, otherwise the payload's.
func ResolveParentSessionID(parentInfo string, input json.RawMessage) string {
	return resolveParentSessionID(parentInfo, input)
}

// 💬️ExtractChat answers the conversation the compaction event is about.
func ExtractChat(input json.RawMessage) string { return extractChatFromInput(input) }

// 📦️ExtractReport answers the report an agent-ended payload carries.
func ExtractReport(input json.RawMessage) string { return extractReportFromInput(input) }

// 🔖️ExtractCheckpointID answers the checkpoint identifier the payload names, falling back to the environment.
func ExtractCheckpointID(input json.RawMessage, environment HookEnvironment) string {
	if checkpoint := topLevelString(input, "sha", "checkpoint_sha", "checkpointSha", "hash"); checkpoint != "" {
		return checkpoint
	}
	return environment.CurrentCheckpoint()
}

// 📝️ExtractCheckpointMessage answers the checkpoint message the payload names, falling back to the environment.
func ExtractCheckpointMessage(input json.RawMessage, repoRoot string, environment HookEnvironment) string {
	if message := topLevelString(input, "message", "commit_message", "commitMessage"); message != "" {
		return message
	}
	return environment.CheckpointMessage(repoRoot)
}

// 📝️CodeEdit is what a code edit event carries.
type CodeEdit struct {
	Path string `json:"path"`
	Old  string `json:"old"`
	New  string `json:"new"`
	All  bool   `json:"all"`
}

// 📝️ExtractCodeEdit reads a code edit out of the payload, or out of the raw tool argument string.
func ExtractCodeEdit(input json.RawMessage, toolArgs string) CodeEdit {
	path, old, replacement, all := extractCodeEditFromInput(input, toolArgs)
	return CodeEdit{Path: path, Old: old, New: replacement, All: all}
}

// 🟫️ExtractTerminalCommand answers the terminal command the payload or the tool argument string carries.
func ExtractTerminalCommand(input json.RawMessage, toolArgs string) string {
	if command := ExtractCommand(input); command != "" {
		return command
	}
	args := parseToolArgs(toolArgs)
	if args == nil {
		return ""
	}
	if command, ok := args["command"].(string); ok {
		return command
	}
	return ""
}

// 💠️TerminalOutcome is what a finished terminal invocation reports.
type TerminalOutcome struct {
	Command    string `json:"command"`
	PID        string `json:"pid"`
	Terminated bool   `json:"terminated"`
	Stdout     string `json:"stdout"`
	Stderr     string `json:"stderr"`
}

// 💠️ExtractTerminalOutcome reads a finished terminal invocation out of the payload.
func ExtractTerminalOutcome(input json.RawMessage) TerminalOutcome {
	command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
	return TerminalOutcome{Command: command, PID: pid, Terminated: terminated, Stdout: stdout, Stderr: stderr}
}

// 🔎️SearchReads is the web pages and file line ranges a search event touched.
type SearchReads struct {
	Pages  []string `json:"pages"`
	Ranges []string `json:"ranges"`
}

// 🔎️ExtractSearchReads reads the search coverage out of the payload, or out of the raw tool argument string.
func ExtractSearchReads(input json.RawMessage, toolArgs string) SearchReads {
	reads := SearchReads{Pages: []string{}, Ranges: []string{}}
	source := planSource(input, toolArgs)
	if source == nil {
		return reads
	}
	if url, ok := source["url"].(string); ok && url != "" {
		reads.Pages = append(reads.Pages, url)
	}
	if pages, ok := source["pages"].([]interface{}); ok {
		for _, page := range pages {
			if text, ok := page.(string); ok && (strings.HasPrefix(text, "http://") || strings.HasPrefix(text, "https://")) {
				reads.Pages = append(reads.Pages, text)
			}
		}
	}
	filePath := ""
	if raw, ok := source["file_path"].(string); ok {
		filePath = raw
	} else if raw, ok := source["filePath"].(string); ok {
		filePath = raw
	}
	if filePath == "" {
		return reads
	}
	number := func(key string) int64 {
		if raw, ok := source[key].(float64); ok {
			return int64(raw)
		}
		return 0
	}
	startLine := number("startLine")
	endLine := number("endLine")
	limit := number("limit")
	offset := int64(1)
	if _, present := source["offset"]; present {
		offset = number("offset")
	}
	switch {
	case startLine > 0:
		if endLine <= 0 {
			endLine = startLine
		}
		if startLine == endLine {
			reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d", filePath, startLine))
		} else {
			reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d-L%d", filePath, startLine, endLine))
		}
	case limit > 0:
		start := offset
		if start <= 0 {
			start = 1
		}
		reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d-L%d", filePath, start, start+limit-1))
	default:
		reads.Ranges = append(reads.Ranges, filePath)
	}
	return reads
}

func parseToolArgs(toolArgs string) map[string]interface{} {
	if toolArgs == "" {
		return nil
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(toolArgs), &parsed); err != nil {
		return nil
	}
	return parsed
}

func planSource(input json.RawMessage, toolArgs string) map[string]interface{} {
	data := decodeHookInputMap(input)
	if data == nil {
		data = parseToolArgs(toolArgs)
	}
	if data == nil {
		return nil
	}
	raw, present := data["tool_input"]
	if !present {
		return data
	}
	toolInput, ok := raw.(map[string]interface{})
	if !ok {
		return nil
	}
	return toolInput
}

// #endregion 🧭️PayloadReading

// #region 🗺️PlanSteps

// 🔁️HookPlanStep is one step of a plan / task list update event.
type HookPlanStep = model.HookPlanStep

// 🗺️ExtractPlanSteps reads the plan steps out of a payload: VS Code's `todoList` entries and the
// neutral `steps` entries, in that order, from `tool_input` when present.
func ExtractPlanSteps(input json.RawMessage, toolArgs string) []HookPlanStep {
	steps := []HookPlanStep{}
	source := planSource(input, toolArgs)
	if source == nil {
		return steps
	}
	if entries, ok := source["todoList"].([]interface{}); ok {
		for _, entry := range entries {
			if fields, ok := entry.(map[string]interface{}); ok {
				name, _ := fields["title"].(string)
				status, _ := fields["status"].(string)
				if name != "" {
					steps = append(steps, HookPlanStep{Name: name, Status: status})
				}
			}
		}
	}
	if entries, ok := source["steps"].([]interface{}); ok {
		for _, entry := range entries {
			if fields, ok := entry.(map[string]interface{}); ok {
				name, _ := fields["name"].(string)
				status, _ := fields["status"].(string)
				if name != "" {
					steps = append(steps, HookPlanStep{Name: name, Status: status})
				}
			}
		}
	}
	return steps
}

// 🔀️MergePlanSteps folds an incoming plan into the recorded one, keeping every lifecycle timestamp.
func MergePlanSteps(existing []model.TicketAgentPlanStep, incoming []HookPlanStep, second string) []model.TicketAgentPlanStep {
	return todos.MergeTicketAgentPlanSteps(existing, incoming, second)
}

// #endregion 🗺️PlanSteps
