// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook dispatch.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func dispatchHook(ctx HookContext) interface{} {
	base := HookResultAgentBase{
		HookResultBase: HookResultBase{Allowed: true},
		Checkpoint:     extractCheckpointSHAFromInput(ctx.Input),
		Session:        extractSessionIDFromInput(ctx.Input),
		Second:         firstNonEmpty(findNestedStringValue(decodeHookInputMap(ctx.Input), "second"), ctx.Second),
		Client:         ctx.Client,
		LLM:            extractLLMFromInput(ctx.Input),
		Effort:         extractEffortFromInput(ctx.Input),
		Transcript:     extractTranscriptFromInput(ctx.Input),
		MessageID:      extractMessageIDFromInput(ctx.Input),
		Parent:         resolveParentSessionID("", ctx.Input),
	}
	if ctx.ParentInfo != "" {
		base.Parent = resolveParentSessionID(ctx.ParentInfo, ctx.Input)
		if ctx.ParentInfo == "subagent" {
			if agentID := findNestedStringValue(decodeHookInputMap(ctx.Input), "agent_id", "agentId"); agentID != "" {
				parentSession := extractParentFromInput(ctx.Input)
				if parentSession == "" {
					parentSession = extractSessionIDFromInput(ctx.Input)
				}
				base.Session = agentID
				base.Parent = normalizeParentSessionID(parentSession)
			} else if base.Parent == "" {
				base.Parent = base.Session
			}
		}
	}
	if len(ctx.Input) > 0 {
		var raw map[string]interface{}
		if err := json.Unmarshal(ctx.Input, &raw); err == nil {
			base.Raw = raw
		}
	}
	switch ctx.Event {
	case HookAgentStarted:
		return HookResultAgentStarted{HookResultAgentBase: base}
	case HookAgentEnded:
		return HookResultAgentEnded{HookResultAgentBase: base, Report: extractReportFromInput(ctx.Input)}
	case HookAgentPromptSubmitting:
		return HookResultAgentPromptSubmitting{HookResultAgentBase: base, Prompt: findNestedStringValue(decodeHookInputMap(ctx.Input), "prompt", "text")}
	case HookAgentCompacting:
		return HookResultAgentCompacting{HookResultAgentBase: base, Chat: extractChatFromInput(ctx.Input)}
	case HookAgentToolStarting:
		toolName := firstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input))
		toolInput := extractToolInputFromStdin(ctx.Input)
		blocked, reason := IsToolBlocked(toolName, firstNonEmpty(ctx.ToolArgs, extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs), extractCommandFromStdin(ctx.Input)))
		base.Allowed = !blocked
		base.Message = reason
		return HookResultAgentToolStarting{HookResultAgentBase: base, Name: toolName, Input: toolInput}
	case HookAgentToolEnded:
		response := extractToolResponseFromStdin(ctx.Input)
		if len(response) == 0 {
			if output := findNestedStringValue(decodeHookInputMap(ctx.Input), "tool_output", "toolOutput", "output", "response"); output != "" {
				response, _ = json.Marshal(output)
			}
		}
		return HookResultAgentToolEnded{HookResultAgentBase: base, Name: firstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: extractToolInputFromStdin(ctx.Input), Response: response}
	case HookAgentToolPlanUpdatingStarting, HookAgentToolPlanUpdatingEnded:
		return HookResultAgentToolPlanUpdating{HookResultAgentBase: base, Steps: extractPlanStepsFromInput(ctx.Input, ctx.ToolArgs)}
	case HookAgentToolSearchStarting:
		pages, ranges := extractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return HookResultAgentToolSearchStarting{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case HookAgentToolSearchEnded:
		pages, ranges := extractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return HookResultAgentToolSearchEnded{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case HookAgentToolCodeEditStarting:
		path, oldText, newText, all := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		return HookResultAgentToolCodeEditStarting{HookResultAgentBase: base, Path: path, Old: oldText, New: newText, All: all}
	case HookAgentToolCodeEditEnded:
		path, oldText, newText, _ := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		// Auto-format the edited file.
		if path != "" {
			runFormatterForFile(path)
		}
		return HookResultAgentToolCodeEditEnded{HookResultAgentBase: base, Path: path, Old: oldText, New: newText}
	case HookAgentToolTestStarting:
		labs, tests, timeout := extractTestStartingFromInput(ctx.Input, ctx.ToolArgs)
		return HookResultAgentToolTestStarting{HookResultAgentBase: base, Labs: labs, Tests: tests, Timeout: timeout}
	case HookAgentToolTestEnded:
		files, succeeded, failed := extractTestEndedFromInput(ctx.Input)
		return HookResultAgentToolTestEnded{HookResultAgentBase: base, Files: files, Succeeded: succeeded, Failed: failed}
	case HookAgentToolBuildStarting:
		bundles := extractBuildBundlesFromInput(ctx.Input, ctx.ToolArgs)
		payload, _ := json.Marshal(map[string]interface{}{"bundles": bundles})
		return HookResultAgentToolStarting{HookResultAgentBase: base, Name: firstNonEmpty(ctx.ToolName, "build"), Input: payload}
	case HookAgentToolBuildEnded:
		succeeded, failed := extractBuildEndedFromInput(ctx.Input)
		payload, _ := json.Marshal(map[string]interface{}{"succeeded": succeeded, "failed": failed})
		return HookResultAgentToolEnded{HookResultAgentBase: base, Name: firstNonEmpty(ctx.ToolName, "build"), Response: payload}
	case HookAgentToolTerminalStarting:
		command := extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs)
		blocked, reason := IsToolBlocked(firstNonEmpty(ctx.ToolName, "run_in_terminal"), firstNonEmpty(command, ctx.ToolArgs))
		base.Allowed = !blocked
		base.Message = reason
		return HookResultAgentToolTerminalStarting{HookResultAgentBase: base, Name: firstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: extractToolInputFromStdin(ctx.Input), Command: command}
	case HookAgentToolTerminalEnded:
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(ctx.Input)
		pidValue, _ := strconv.Atoi(pid)
		stdoutJSON, _ := json.Marshal(stdout)
		stderrJSON, _ := json.Marshal(stderr)
		return HookResultAgentToolTerminalEnded{HookResultAgentBase: base, Name: firstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: extractToolInputFromStdin(ctx.Input), Output: stdoutJSON, Command: command, PID: pidValue, Terminated: terminated, Stdout: stdoutJSON, Stderr: stderrJSON}
	case HookAgentThinkingStarting:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return HookResultAgentThinkingStarting{HookResultAgentBase: base}
	case HookAgentThinkingEnded:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return HookResultAgentThinkingEnded{HookResultAgentBase: base}
	case HookVersionCheckpointStarting:
		return runCheckpointStartingHook(ctx)
	case HookVersionCheckpointEnded:
		return HookResultVersionCheckpointEnded{HookResultBase: HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input), Description: extractCheckpointMessageFromInput(ctx.Input, ctx.RepoRoot)}
	case HookVersionCheckinStarting, HookVersionCheckinEnded, HookVersionCheckoutStarting, HookVersionCheckoutEnded:
		return HookResultVersionCheckpointEnded{HookResultBase: HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input)}
	default:
		return HookResultBase{Allowed: false, Message: "unknown event: " + string(ctx.Event)}
	}
}

func resolveEventCheckpointID(event interface{}) string {
	s, _ := event.(string)
	return strings.TrimSpace(s)
}

func resolveRangeRef(ref string) (string, error) {
	parts := strings.SplitN(strings.TrimSpace(ref), "#L", 2)
	if len(parts) != 2 {
		return "", fmt.Errorf("invalid range ref %q", ref)
	}
	fileID := resolvePathToFileID(parts[0])
	if fileID == "" {
		return "", fmt.Errorf("invalid file path %q", parts[0])
	}
	lineSpec := parts[1]
	if strings.Contains(lineSpec, "-L") {
		rangeParts := strings.SplitN(lineSpec, "-L", 2)
		start, errStart := strconv.Atoi(rangeParts[0])
		end, errEnd := strconv.Atoi(rangeParts[1])
		if errStart != nil || errEnd != nil {
			return "", fmt.Errorf("invalid line range %q", lineSpec)
		}
		return fileID + emojiText(EmojiLine) + strconv.Itoa(start) + emojiText(EmojiLine) + strconv.Itoa(end), nil
	}
	line, err := strconv.Atoi(lineSpec)
	if err != nil {
		return "", fmt.Errorf("invalid line %q", lineSpec)
	}
	return fileID + emojiText(EmojiLine) + strconv.Itoa(line), nil
}

func extractSearchCommandFilesAndPattern(command string, cwd string) ([]string, string) {
	fields := strings.Fields(command)
	if len(fields) == 0 {
		return nil, ""
	}
	var pattern string
	var files []string
	for i := 1; i < len(fields); i++ {
		field := strings.Trim(fields[i], `"'`)
		if field == "" {
			continue
		}
		if strings.HasPrefix(field, "-") {
			if field == "-e" && i+1 < len(fields) {
				pattern = strings.Trim(fields[i+1], `"'`)
				i++
			}
			continue
		}
		if pattern == "" {
			pattern = field
			continue
		}
		if !filepath.IsAbs(field) && cwd != "" {
			field = filepath.Join(cwd, field)
		}
		files = append(files, field)
	}
	if len(files) == 0 && cwd != "" && len(fields) > 0 {
		for _, field := range fields[1:] {
			trimmed := strings.Trim(field, `"'`)
			if strings.HasPrefix(trimmed, "-") || trimmed == pattern || trimmed == "" {
				continue
			}
			files = append(files, filepath.Join(cwd, trimmed))
		}
	}
	return files, pattern
}

func searchLinesInFile(filePath string, pattern string) []int {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return nil
	}
	var lines []int
	needle := strings.TrimSpace(pattern)
	for index, line := range strings.Split(string(content), "\n") {
		if needle == "" || strings.Contains(line, needle) {
			lines = append(lines, index+1)
		}
	}
	return lines
}
