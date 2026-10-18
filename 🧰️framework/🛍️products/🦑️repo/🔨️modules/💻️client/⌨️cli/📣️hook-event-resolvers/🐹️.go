// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Editor hook event resolvers.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"strings"
)

// 🪪️normalizeTicketSessionID normalizes a ticket session ID.
func normalizeTicketSessionID(sessionID string) string {
	sessionID = strings.TrimSpace(sessionID)
	if sessionID == "" {
		return ""
	}
	sessionEmoji := emojiText(EmojiSession)
	sessionStateEmojis := []string{
		emojiText(EmojiSession),
		emojiText(EmojiSessionRunning),
		emojiText(EmojiSessionCompleted),
		emojiText(EmojiSessionInterrupted),
	}
	prefix := ""
	payload := sessionID
	markerIndex := -1
	markerLen := 0
	for _, marker := range sessionStateEmojis {
		if marker == "" {
			continue
		}
		index := strings.Index(sessionID, marker)
		if index >= 0 && (markerIndex == -1 || index < markerIndex) {
			markerIndex = index
			markerLen = len(marker)
		}
	}
	if markerIndex >= 0 {
		prefix = sessionID[:markerIndex]
		payload = sessionID[markerIndex+markerLen:]
	}
	payload = Flat(payload)
	if payload == "" {
		return strings.TrimSpace(prefix)
	}
	return prefix + sessionEmoji + payload
}

// 🎫️currentTicketSessionID returns the current ticket session ID.
func currentTicketSessionID() string {
	if testSessionIDOverride != "" {
		return testSessionIDOverride
	}
	return generateHookSessionID()
}

// 📡️resolveCopilotEvent maps a VS Code / Copilot Chat native event to a neutral HookEvent.
func resolveCopilotEvent(nativeEvent string, kind ToolKind) (HookEvent, string, error) {
	switch nativeEvent {
	case "SessionStart":
		return HookAgentStarted, "", nil
	case "Stop":
		return HookAgentEnded, "", nil
	case "SubagentStart":
		return HookAgentStarted, "subagent", nil
	case "SubagentStop":
		return HookAgentEnded, "subagent", nil
	case "UserPromptSubmit":
		return HookAgentPromptSubmitting, "", nil
	case "PreCompact":
		return HookAgentCompacting, "", nil
	case "PreToolUse":
		return resolvePreToolUse(kind), "", nil
	case "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return HookAgentEnded, "", nil
	case "afterAgentThought":
		return HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for copilot-chat", nativeEvent)
	}
}

// 🗺️resolveCursorEvent maps a Cursor native event to a neutral HookEvent.
func resolveCursorEvent(nativeEvent string, kind ToolKind) (HookEvent, string, error) {
	switch nativeEvent {
	case "sessionStart":
		return HookAgentStarted, "", nil
	case "sessionEnd":
		return HookAgentEnded, "", nil
	case "subagentStart":
		return HookAgentStarted, "subagent", nil
	case "subagentStop":
		return HookAgentEnded, "subagent", nil
	case "stop":
		return HookAgentEnded, "", nil
	case "userPromptSubmit", "beforeSubmitPrompt":
		return HookAgentPromptSubmitting, "", nil
	case "preCompact":
		return HookAgentCompacting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return HookAgentEnded, "", nil
	case "afterAgentThought":
		return HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for cursor-chat", nativeEvent)
	}
}

// 🔷️resolveWindsurfEvent maps a Windsurf native event to a neutral HookEvent.
func resolveWindsurfEvent(nativeEvent string, kind ToolKind) (HookEvent, string, error) {
	switch nativeEvent {
	case "pre_user_prompt":
		return HookAgentPromptSubmitting, "", nil
	case "post_cascade_response":
		return HookAgentEnded, "", nil
	case "post_setup_worktree":
		return HookAgentStarted, "", nil
	case "subagentStart":
		return HookAgentStarted, "subagent", nil
	case "subagentStop":
		return HookAgentEnded, "subagent", nil
	case "stop":
		return HookAgentEnded, "", nil
	case "preCompact":
		return HookAgentCompacting, "", nil
	case "preToolUse", "pre_mcp_tool_use":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "post_mcp_tool_use":
		return resolvePostToolUse(kind), "", nil
	case "pre_read_code":
		return HookAgentToolSearchStarting, "", nil
	case "post_read_code":
		return HookAgentToolSearchEnded, "", nil
	case "pre_write_code":
		return HookAgentToolCodeEditStarting, "", nil
	case "post_write_code":
		return HookAgentToolCodeEditEnded, "", nil
	case "pre_run_command":
		return resolveShellPreToolUse(kind), "", nil
	case "post_run_command":
		return resolveShellPostToolUse(kind), "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for windsurf-chat", nativeEvent)
	}
}

// 🔶️resolveClaudeCompatibleEvent maps a Claude-compatible native event to a neutral HookEvent.
func resolveClaudeCompatibleEvent(nativeEvent string, kind ToolKind) (HookEvent, string, error) {
	switch nativeEvent {
	case "start", "SessionStart":
		return HookAgentStarted, "", nil
	case "stop", "Stop", "SessionEnd":
		return HookAgentEnded, "", nil
	case "subagentStart", "SubagentStart":
		return HookAgentStarted, "subagent", nil
	case "subagentStop", "SubagentStop":
		return HookAgentEnded, "subagent", nil
	case "userPromptSubmit", "UserPromptSubmit":
		return HookAgentPromptSubmitting, "", nil
	case "preCompact", "PreCompact":
		return HookAgentCompacting, "", nil
	case "preToolUse", "PreToolUse", "PermissionRequest", "TeammateIdle", "Notification":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "TaskCompleted":
		return HookAgentToolPlanUpdatingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for claude-compatible", nativeEvent)
	}
}

// 🔹️resolveKiroEvent maps a Kiro native event to a neutral HookEvent.
func resolveKiroEvent(nativeEvent string, kind ToolKind) (HookEvent, string, error) {
	switch nativeEvent {
	case "agentSpawn":
		return HookAgentStarted, "", nil
	case "userPromptSubmit":
		return HookAgentPromptSubmitting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse":
		return resolvePostToolUse(kind), "", nil
	case "stop":
		return HookAgentEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for kiro-cli", nativeEvent)
	}
}

// 💻️formatVSCodeHookOutput formats a hook output for VS Code.
func formatVSCodeHookOutput(hookEventName string, result HookResult) string {
	payload := map[string]interface{}{}
	out, _ := json.Marshal(result)
	_ = json.Unmarshal(out, &payload)
	hookSpecificOutput := map[string]interface{}{}
	if hookEventName != "" {
		hookSpecificOutput["hookEventName"] = hookEventName
	}
	if hookEventName == "PreToolUse" {
		if result.IsAllowed() {
			hookSpecificOutput["permissionDecision"] = "allow"
		} else {
			hookSpecificOutput["permissionDecision"] = "deny"
			if reason := strings.TrimSpace(result.GetMessage()); reason != "" {
				hookSpecificOutput["permissionDecisionReason"] = reason
			}
		}
	} else if message := strings.TrimSpace(result.GetMessage()); message != "" {
		hookSpecificOutput["additionalContext"] = message
	}
	if len(hookSpecificOutput) > 0 {
		payload["hookSpecificOutput"] = hookSpecificOutput
	}
	finalOut, _ := json.Marshal(payload)
	return string(finalOut)
}

// 🔤️vsCodeEventFromHookEvent converts a HookEvent to a VS Code event string.
func vsCodeEventFromHookEvent(event HookEvent, parentInfo string) string {
	switch event {
	case HookAgentStarted:
		if parentInfo == "subagent" {
			return "SubagentStart"
		}
		return "SessionStart"
	case HookAgentEnded:
		if parentInfo == "subagent" {
			return "SubagentStop"
		}
		return "Stop"
	case HookAgentPromptSubmitting:
		return "UserPromptSubmit"
	case HookAgentCompacting:
		return "PreCompact"
	case HookAgentToolStarting,
		HookAgentToolPlanUpdatingStarting,
		HookAgentToolSearchStarting,
		HookAgentToolCodeEditStarting,
		HookAgentToolTestStarting,
		HookAgentToolBuildStarting,
		HookAgentToolTerminalStarting:
		return "PreToolUse"
	case HookAgentToolEnded,
		HookAgentToolPlanUpdatingEnded,
		HookAgentToolSearchEnded,
		HookAgentToolCodeEditEnded,
		HookAgentToolTestEnded,
		HookAgentToolBuildEnded,
		HookAgentToolTerminalEnded:
		return "PostToolUse"
	default:
		return ""
	}
}

// 🏷️resolvePreToolUse resolves the pre-tool-use event based on tool kind.
func resolvePreToolUse(kind ToolKind) HookEvent {
	switch kind {
	case ToolKindPlan:
		return HookAgentToolPlanUpdatingStarting
	case ToolKindCodeSearch:
		return HookAgentToolSearchStarting
	case ToolKindCodeEdit:
		return HookAgentToolCodeEditStarting
	case ToolKindTest:
		return HookAgentToolTestStarting
	case ToolKindBuild:
		return HookAgentToolBuildStarting
	case ToolKindTerminal:
		return HookAgentToolTerminalStarting
	default:
		return HookAgentToolStarting
	}
}

// 🔸️resolvePostToolUse resolves the post-tool-use event based on tool kind.
func resolvePostToolUse(kind ToolKind) HookEvent {
	switch kind {
	case ToolKindPlan:
		return HookAgentToolPlanUpdatingEnded
	case ToolKindCodeSearch:
		return HookAgentToolSearchEnded
	case ToolKindCodeEdit:
		return HookAgentToolCodeEditEnded
	case ToolKindTest:
		return HookAgentToolTestEnded
	case ToolKindBuild:
		return HookAgentToolBuildEnded
	case ToolKindTerminal:
		return HookAgentToolTerminalEnded
	default:
		return HookAgentToolEnded
	}
}

func resolveShellPreToolUse(kind ToolKind) HookEvent {
	switch kind {
	case ToolKindGeneric:
		return HookAgentToolTerminalStarting
	default:
		return resolvePreToolUse(kind)
	}
}

func resolveShellPostToolUse(kind ToolKind) HookEvent {
	switch kind {
	case ToolKindGeneric:
		return HookAgentToolTerminalEnded
	default:
		return resolvePostToolUse(kind)
	}
}
