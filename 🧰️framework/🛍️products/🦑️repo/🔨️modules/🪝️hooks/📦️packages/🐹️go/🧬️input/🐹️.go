// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧬️input carries the extraction of code edit, terminal, chat, report, checkpoint and parent fields from a hook input.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🦀️Hooks

// 📝️extractCodeEditFromInput holds the data fields for a extractCodeEditFromInput record.
func extractCodeEditFromInput(input json.RawMessage, toolArgs string) (path string, old string, new_ string, all bool) {
	var toolInput map[string]interface{}
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			toolInput = extractToolInputMapFromData(data)
		}
	}
	if toolInput == nil && toolArgs != "" {
		_ = json.Unmarshal([]byte(toolArgs), &toolInput)
	}
	if toolInput == nil {
		return "", "", "", false
	}
	for _, k := range []string{"filePath", "path", "file_path", "file"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			path = v
			break
		}
	}
	for _, k := range []string{"oldString", "old", "old_string", "search"} {
		if v, ok := toolInput[k].(string); ok {
			old = v
			break
		}
	}
	for _, k := range []string{"newString", "new", "new_string", "replace"} {
		if v, ok := toolInput[k].(string); ok {
			new_ = v
			break
		}
	}
	if v, ok := toolInput["all"].(bool); ok {
		all = v
	} else if v, ok := toolInput["replaceAll"].(bool); ok {
		all = v
	}
	return path, old, new_, all
}

// 🟫️extractTerminalCommandFromInput holds the data fields for a extractTerminalCommandFromInput record.
func extractTerminalCommandFromInput(input json.RawMessage, toolArgs string) string {
	cmd := extractCommandFromStdin(input)
	if cmd != "" {
		return cmd
	}
	if toolArgs != "" {
		var args map[string]interface{}
		if err := json.Unmarshal([]byte(toolArgs), &args); err == nil {
			if c, ok := args["command"].(string); ok {
				return c
			}
		}
	}
	return ""
}

// 💠️extractTerminalEndedFromInput holds the data fields for a extractTerminalEndedFromInput record.
func extractTerminalEndedFromInput(input json.RawMessage) (command string, pid string, terminated bool, stdout string, stderr string) {
	if len(input) == 0 {
		return "", "", false, "", ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return "", "", false, "", ""
	}
	toolInput := data
	if ti, ok := data["tool_input"].(map[string]interface{}); ok {
		toolInput = ti
	}
	for _, k := range []string{"command", "command_line"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			command = v
			break
		}
	}
	if ti, ok := data["tool_info"].(map[string]interface{}); ok {
		if v, ok := ti["command_line"].(string); ok && v != "" {
			command = v
		}
	}
	for _, k := range []string{"pid", "process_id", "execution_id", "id"} {
		if v, ok := data[k].(string); ok && v != "" {
			pid = v
			break
		}
		if v, ok := data[k].(float64); ok {
			pid = fmt.Sprintf("%d", int64(v))
			break
		}
	}
	for _, k := range []string{"terminated", "has_terminated", "exited", "stopped"} {
		if v, ok := data[k].(bool); ok {
			terminated = v
			break
		}
	}
	for _, k := range []string{"stdout", "output", "tool_output"} {
		if v, ok := data[k].(string); ok {
			stdout = v
			break
		}
	}
	for _, k := range []string{"stderr", "error_output"} {
		if v, ok := data[k].(string); ok {
			stderr = v
			break
		}
	}
	return command, pid, terminated, stdout, stderr
}

func extractChatFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"chat", "conversation", "context", "messages"} {
		if value, ok := data[key].(string); ok {
			return strings.TrimSpace(value)
		}
		if arr, ok := data[key].([]interface{}); ok {
			bytes, err := json.Marshal(arr)
			if err == nil {
				return string(bytes)
			}
		}
	}
	return ""
}

// 📦️extractReportFromInput extracts a text report from agent-ended payloads.
func extractReportFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	getReport := func(node map[string]interface{}) string {
		if node == nil {
			return ""
		}
		if report, ok := node["report"].(string); ok && strings.TrimSpace(report) != "" {
			return strings.TrimSpace(report)
		}
		if toolInfo, ok := node["tool_info"].(map[string]interface{}); ok {
			if response, ok := toolInfo["response"].(string); ok && strings.TrimSpace(response) != "" {
				return strings.TrimSpace(response)
			}
		}
		return ""
	}
	if report := getReport(data); report != "" {
		return report
	}
	if nested, ok := data["event"].(map[string]interface{}); ok {
		if report := getReport(nested); report != "" {
			return report
		}
	}
	if nested, ok := data["input"].(map[string]interface{}); ok {
		if report := getReport(nested); report != "" {
			return report
		}
	}
	if native, ok := data["native"].(map[string]interface{}); ok {
		if report := getReport(native); report != "" {
			return report
		}
		if eventData, ok := native["event"].(map[string]interface{}); ok {
			if report := getReport(eventData); report != "" {
				return report
			}
		}
	}
	return ""
}

func extractCheckpointMessageFromInput(input json.RawMessage, repoRoot string) string {
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			for _, key := range []string{"message", "commit_message", "commitMessage"} {
				if v, ok := data[key].(string); ok && v != "" {
					return strings.TrimSpace(v)
				}
			}
		}
	}
	checkpointMsgFile := filepath.Join(repoRoot, ".git", "COMMIT_EDITMSG")
	if data, err := os.ReadFile(checkpointMsgFile); err == nil {
		msg := strings.TrimSpace(string(data))
		if msg != "" {
			return msg
		}
	}
	return ""
}

// 💾️extractCheckpointSHAFromInput holds the data fields for a extractCheckpointSHAFromInput record.
func extractCheckpointSHAFromInput(input json.RawMessage) string {
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			for _, key := range []string{"sha", "checkpoint_sha", "checkpointSha", "hash"} {
				if v, ok := data[key].(string); ok && v != "" {
					return strings.TrimSpace(v)
				}
			}
		}
	}
	return workspace.GetGitCheckpoint(workspace.GetRootDir())
}

// 🪪️normalizeParentSessionID holds the data fields for a normalizeParentSessionID record.
func normalizeParentSessionID(parent string) string {
	parent = strings.TrimSpace(parent)
	if parent == "" {
		return ""
	}
	if strings.EqualFold(parent, "subagent") {
		return ""
	}
	return parent
}

func extractParentFromMap(data map[string]interface{}) string {
	for _, key := range []string{"parent", "parentInfo", "parent_info", "parentSessionId", "parent_session_id", "parentId", "parent_id"} {
		if v, ok := data[key].(string); ok {
			parent := normalizeParentSessionID(v)
			if parent != "" {
				return parent
			}
		}
	}
	return ""
}

func extractParentFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	if parent := extractParentFromMap(data); parent != "" {
		return parent
	}
	for _, key := range []string{"event", "native"} {
		if nested, ok := data[key].(map[string]interface{}); ok {
			if parent := extractParentFromMap(nested); parent != "" {
				return parent
			}
			if key == "native" {
				if nativeEvent, ok := nested["event"].(map[string]interface{}); ok {
					if parent := extractParentFromMap(nativeEvent); parent != "" {
						return parent
					}
				}
			}
		}
	}
	return ""
}

// 🔳️resolveParentSessionID holds the data fields for a resolveParentSessionID record.
func resolveParentSessionID(parentInfo string, input json.RawMessage) string {
	parent := normalizeParentSessionID(parentInfo)
	if parent != "" {
		return parent
	}
	return extractParentFromInput(input)
}

func decodeHookInputMap(input json.RawMessage) map[string]interface{} {
	if len(input) == 0 {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	return data
}

func findNestedStringValue(value interface{}, keys ...string) string {
	switch typed := value.(type) {
	case map[string]interface{}:
		for _, key := range keys {
			if raw, ok := typed[key].(string); ok && strings.TrimSpace(raw) != "" {
				return strings.TrimSpace(raw)
			}
		}
		for _, nested := range typed {
			if found := findNestedStringValue(nested, keys...); found != "" {
				return found
			}
		}
	case []interface{}:
		for _, nested := range typed {
			if found := findNestedStringValue(nested, keys...); found != "" {
				return found
			}
		}
	}
	return ""
}

// 🔲️extractTranscriptFromInput holds the data fields for a extractTranscriptFromInput record.
func ExtractTranscriptFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "transcript", "transcript_path", "transcriptPath", "log_path", "logPath")
}

// ▪️extractMessageIDFromInput holds the data fields for a extractMessageIDFromInput record.
func extractMessageIDFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"messageId", "message_id", "turnId", "turn_id", "requestId", "request_id"} {
		if v, ok := data[key].(string); ok && v != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

func extractParentMessageIDFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id"} {
		if v, ok := data[key].(string); ok && v != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

// #endregion 🦀️Hooks

// #region 💾️Missing Utility Functions

// 📡️resolveEventSessionID resolves the session ID from input.
func ResolveEventSessionID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 📝️resolveEventSecondID resolves the second ID from input.
func ResolveEventSecondID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 🗺️extractToolInputMapFromData extracts tool input map from data.
func extractToolInputMapFromData(data interface{}) map[string]interface{} {
	if m, ok := data.(map[string]interface{}); ok {
		if toolInput, ok := m["tool_input"].(map[string]interface{}); ok {
			return toolInput
		}
		if event, ok := m["event"].(map[string]interface{}); ok {
			if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
				return toolInput
			}
		}
		if native, ok := m["native"].(map[string]interface{}); ok {
			if event, ok := native["event"].(map[string]interface{}); ok {
				if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
					return toolInput
				}
			}
		}
		return nil
	}
	return nil
}

// 🔶️looksLikeFilePath checks if a string looks like a file path.
func looksLikeFilePath(s string) bool {
	// Check if string contains path separators or file extensions
	return strings.Contains(s, "/") || strings.Contains(s, "\\") ||
		strings.Contains(s, ".") && !strings.HasPrefix(s, ".")
}

// 🔹️extractCommandFromStdin extracts command from stdin.
func extractCommandFromStdin(data json.RawMessage) string {
	var cmd string
	if err := json.Unmarshal(data, &cmd); err == nil {
		return strings.TrimSpace(cmd)
	}
	decoded := decodeHookInputMap(data)
	if decoded == nil {
		return ""
	}
	if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
		if value, ok := toolInput["command"].(string); ok {
			return strings.TrimSpace(value)
		}
	}
	return strings.TrimSpace(findNestedStringValue(decoded, "command", "command_line", "commandLine"))
}

// 🔸️extractCommandCwdFromInput extracts command and cwd from input.
func extractCommandCwdFromInput(input json.RawMessage) (string, string) {
	decoded := decodeHookInputMap(input)
	command := extractCommandFromStdin(input)
	cwd := ""
	if decoded != nil {
		if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
			if value, ok := toolInput["cwd"].(string); ok {
				cwd = strings.TrimSpace(value)
			}
		}
		if cwd == "" {
			cwd = strings.TrimSpace(findNestedStringValue(decoded, "cwd", "working_directory", "workingDirectory"))
		}
	}
	if cwd == "" {
		cwd, _ = os.Getwd()
	}
	return command, cwd
}

// #endregion 💾️Missing Utility Functions

// #region 📰️Todos

// 🧲️extractSessionIDFromInput extracts session ID from input.
func ExtractSessionIDFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	sessionID := findNestedStringValue(
		data,
		"session_id",
		"sessionId",
		"trajectory_id",
		"trajectoryId",
		"conversation_id",
		"conversationId",
		"agent_id",
		"agentId",
	)
	if sessionID != "" {
		return sessionID
	}
	transcript := ExtractTranscriptFromInput(input)
	if transcript == "" {
		return ""
	}
	base := strings.TrimSpace(filepath.Base(transcript))
	ext := filepath.Ext(base)
	if ext != "" {
		base = strings.TrimSuffix(base, ext)
	}
	return strings.TrimSpace(base)
}

// 📝️extractLLMFromInput extracts LLM from input.
func extractLLMFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "llm", "model", "model_name", "modelName")
}

// 🏋️extractEffortFromInput extracts reasoning effort from input.
func extractEffortFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "effort", "reasoning_effort", "reasoningEffort")
}

// 🔷️extractToolNameFromStdin extracts tool name from stdin.
func extractToolNameFromStdin(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName")
}

// 📡️extractHookEventNameFromStdin extracts hook event name from stdin.
func extractHookEventNameFromStdin(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data,
		"hookEventName",
		"hook_event_name",
		"hook_event",
		"hookEvent",
		"event",
	)
}

// #endregion 📰️Todos
