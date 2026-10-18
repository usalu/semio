// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook input extraction.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func extractTestStartingFromInput(input json.RawMessage, toolArgs string) (labs []string, tests []string, timeout string) {
	var toolInput map[string]interface{}
	var rawData map[string]interface{}
	if len(input) > 0 {
		if err := json.Unmarshal(input, &rawData); err == nil {
			toolInput = extractToolInputMapFromData(rawData)
		}
	}
	if toolInput == nil && toolArgs != "" {
		_ = json.Unmarshal([]byte(toolArgs), &toolInput)
	}
	if toolInput == nil {
		toolInput = map[string]interface{}{}
	}

	var targetFiles []string

	for _, k := range []string{"files"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					targetFiles = append(targetFiles, s)
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			targetFiles = append(targetFiles, v)
		}
	}
	for _, k := range []string{"testNames", "test_names", "tests"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					if looksLikeFilePath(s) {
						targetFiles = append(targetFiles, s)
					} else {
						tests = append(tests, s)
					}
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			if looksLikeFilePath(v) {
				targetFiles = append(targetFiles, v)
			} else {
				tests = append(tests, v)
			}
		}
	}
	for _, k := range []string{"timeout", "timeoutMs"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			timeout = v
			break
		}
		if v, ok := toolInput[k].(float64); ok {
			timeout = fmt.Sprintf("%d", int64(v))
			break
		}
	}

	cmd := extractCommandFromStdin(input)
	if cmd == "" {
		if c, ok := toolInput["command"].(string); ok {
			cmd = c
		}
	}
	if len(tests) == 0 && cmd != "" {
		parsedTests, parsedTimeout := parseTestInfoFromCommand(cmd)
		if len(parsedTests) > 0 {
			tests = parsedTests
		}
		if timeout == "" && parsedTimeout != "" {
			timeout = parsedTimeout
		}
	}

	if cmd != "" && len(targetFiles) == 0 {
		cwd := ""
		if rawData != nil {
			_, cwd = extractCommandCwdFromInput(input)
		}
		commandFiles := resolveTestFilesFromCommand(cmd, cwd)
		if len(commandFiles) > 0 {
			targetFiles = commandFiles
		}
	}

	fullRunSelection := false
	if len(targetFiles) > 0 && (len(tests) == 0 || (len(tests) == 1 && tests[0] == "")) {
		cwd := ""
		if rawData != nil {
			_, cwd = extractCommandCwdFromInput(input)
		}
		fullRunFiles := resolveFullTestFilesFromCommand(cmd, cwd)
		fullRunSelection = len(fullRunFiles) > 0 && sameStringSet(targetFiles, fullRunFiles)
	}
	if fullRunSelection {
		labs = resolveTestLabIDs(targetFiles)
		return labs, nil, timeout
	}

	if len(targetFiles) > 0 {
		resolvedTests := resolveSelectedTestDefinitionIDs(targetFiles, tests)
		if len(resolvedTests) == 0 {
			resolvedTests = resolveAllTestDefinitionIDs(targetFiles)
		}
		if len(resolvedTests) > 0 {
			tests = resolvedTests
		}
	}

	return nil, tests, timeout
}

// 🧲️extractTestEndedFromInput holds the data fields for a extractTestEndedFromInput record.
func extractTestEndedFromInput(input json.RawMessage) (files []string, succeeded []string, failed []string) {
	if len(input) == 0 {
		return nil, nil, nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil, nil, nil
	}

	if ti := extractToolInputMapFromData(data); ti != nil {
		for _, k := range []string{"files"} {
			if arr, ok := ti[k].([]interface{}); ok {
				for _, item := range arr {
					if s, ok := item.(string); ok && s != "" {
						files = append(files, s)
					}
				}
			} else if v, ok := ti[k].(string); ok && v != "" {
				files = append(files, v)
			}
		}
	}

	if len(files) == 0 {
		cmd := extractCommandFromStdin(input)
		if cmd != "" {
			_, cwd := extractCommandCwdFromInput(input)
			commandFiles := resolveTestFilesFromCommand(cmd, cwd)
			if len(commandFiles) > 0 {
				files = commandFiles
			}
		}
	}
	toolOutput := data
	if to, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = to
	}
	for _, k := range []string{"succeeded", "passed", "passing"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					succeeded = append(succeeded, s)
				}
			}
		}
	}
	for _, k := range []string{"failed", "failing", "errors"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					failed = append(failed, s)
				}
			}
		}
	}
	return files, succeeded, failed
}

func extractBuildBundlesFromInput(input json.RawMessage, toolArgs string) []string {
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
		return nil
	}
	var bundles []string
	for _, k := range []string{"bundles", "targets", "technologies", "label"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					bundles = append(bundles, s)
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			bundles = append(bundles, v)
		}
	}
	return bundles
}

// 🏗️extractBuildEndedFromInput holds the data fields for a extractBuildEndedFromInput record.
func extractBuildEndedFromInput(input json.RawMessage) (succeeded []string, failed []string) {
	if len(input) == 0 {
		return nil, nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil, nil
	}
	toolOutput := data
	if to, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = to
	}
	for _, k := range []string{"succeeded", "passed", "built"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					succeeded = append(succeeded, s)
				}
			}
		}
	}
	for _, k := range []string{"failed", "errors"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					failed = append(failed, s)
				}
			}
		}
	}
	return succeeded, failed
}

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
	return GetGitCheckpoint()
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
