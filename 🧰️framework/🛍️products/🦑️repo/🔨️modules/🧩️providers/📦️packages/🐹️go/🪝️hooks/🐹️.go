// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪝️hooks carries the native hook event resolvers and the hook output formatters.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🪝️Hooks

// 📡️resolveCopilotEvent maps a VS Code / Copilot Chat native event to a neutral HookEvent.
func resolveCopilotEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "SessionStart":
		return model.HookAgentStarted, "", nil
	case "Stop":
		return model.HookAgentEnded, "", nil
	case "SubagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "SubagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "UserPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "PreCompact":
		return model.HookAgentCompacting, "", nil
	case "PreToolUse":
		return resolvePreToolUse(kind), "", nil
	case "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return model.HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return model.HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return model.HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return model.HookAgentEnded, "", nil
	case "afterAgentThought":
		return model.HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for copilot-chat", nativeEvent)
	}
}

// 🗺️resolveCursorEvent maps a Cursor native event to a neutral HookEvent.
func resolveCursorEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "sessionStart":
		return model.HookAgentStarted, "", nil
	case "sessionEnd":
		return model.HookAgentEnded, "", nil
	case "subagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	case "userPromptSubmit", "beforeSubmitPrompt":
		return model.HookAgentPromptSubmitting, "", nil
	case "preCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return model.HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return model.HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return model.HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return model.HookAgentEnded, "", nil
	case "afterAgentThought":
		return model.HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for cursor-chat", nativeEvent)
	}
}

// 🔷️resolveWindsurfEvent maps a Windsurf native event to a neutral HookEvent.
func resolveWindsurfEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "pre_user_prompt":
		return model.HookAgentPromptSubmitting, "", nil
	case "post_cascade_response":
		return model.HookAgentEnded, "", nil
	case "post_setup_worktree":
		return model.HookAgentStarted, "", nil
	case "subagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	case "preCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse", "pre_mcp_tool_use":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "post_mcp_tool_use":
		return resolvePostToolUse(kind), "", nil
	case "pre_read_code":
		return model.HookAgentToolSearchStarting, "", nil
	case "post_read_code":
		return model.HookAgentToolSearchEnded, "", nil
	case "pre_write_code":
		return model.HookAgentToolCodeEditStarting, "", nil
	case "post_write_code":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "pre_run_command":
		return resolveShellPreToolUse(kind), "", nil
	case "post_run_command":
		return resolveShellPostToolUse(kind), "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for windsurf-chat", nativeEvent)
	}
}

// 🔶️resolveClaudeCompatibleEvent maps a Claude-compatible native event to a neutral HookEvent.
func ResolveClaudeCompatibleEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "start", "SessionStart":
		return model.HookAgentStarted, "", nil
	case "stop", "Stop", "SessionEnd":
		return model.HookAgentEnded, "", nil
	case "subagentStart", "SubagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop", "SubagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "userPromptSubmit", "UserPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "preCompact", "PreCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse", "PreToolUse", "PermissionRequest", "TeammateIdle", "Notification":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "TaskCompleted":
		return model.HookAgentToolPlanUpdatingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for claude-compatible", nativeEvent)
	}
}

// 🔹️resolveKiroEvent maps a Kiro native event to a neutral HookEvent.
func resolveKiroEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "agentSpawn":
		return model.HookAgentStarted, "", nil
	case "userPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse":
		return resolvePostToolUse(kind), "", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for kiro-cli", nativeEvent)
	}
}

// 💻️formatVSCodeHookOutput formats a hook output for VS Code.
func FormatVSCodeHookOutput(hookEventName string, result model.HookResult) string {
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
func vsCodeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	switch event {
	case model.HookAgentStarted:
		if parentInfo == "subagent" {
			return "SubagentStart"
		}
		return "SessionStart"
	case model.HookAgentEnded:
		if parentInfo == "subagent" {
			return "SubagentStop"
		}
		return "Stop"
	case model.HookAgentPromptSubmitting:
		return "UserPromptSubmit"
	case model.HookAgentCompacting:
		return "PreCompact"
	case model.HookAgentToolStarting,
		model.HookAgentToolPlanUpdatingStarting,
		model.HookAgentToolSearchStarting,
		model.HookAgentToolCodeEditStarting,
		model.HookAgentToolTestStarting,
		model.HookAgentToolBuildStarting,
		model.HookAgentToolTerminalStarting:
		return "PreToolUse"
	case model.HookAgentToolEnded,
		model.HookAgentToolPlanUpdatingEnded,
		model.HookAgentToolSearchEnded,
		model.HookAgentToolCodeEditEnded,
		model.HookAgentToolTestEnded,
		model.HookAgentToolBuildEnded,
		model.HookAgentToolTerminalEnded:
		return "PostToolUse"
	default:
		return ""
	}
}

// 🏷️resolvePreToolUse resolves the pre-tool-use event based on tool kind.
func resolvePreToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindPlan:
		return model.HookAgentToolPlanUpdatingStarting
	case model.ToolKindCodeSearch:
		return model.HookAgentToolSearchStarting
	case model.ToolKindCodeEdit:
		return model.HookAgentToolCodeEditStarting
	case model.ToolKindTest:
		return model.HookAgentToolTestStarting
	case model.ToolKindBuild:
		return model.HookAgentToolBuildStarting
	case model.ToolKindTerminal:
		return model.HookAgentToolTerminalStarting
	default:
		return model.HookAgentToolStarting
	}
}

// 🔸️resolvePostToolUse resolves the post-tool-use event based on tool kind.
func resolvePostToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindPlan:
		return model.HookAgentToolPlanUpdatingEnded
	case model.ToolKindCodeSearch:
		return model.HookAgentToolSearchEnded
	case model.ToolKindCodeEdit:
		return model.HookAgentToolCodeEditEnded
	case model.ToolKindTest:
		return model.HookAgentToolTestEnded
	case model.ToolKindBuild:
		return model.HookAgentToolBuildEnded
	case model.ToolKindTerminal:
		return model.HookAgentToolTerminalEnded
	default:
		return model.HookAgentToolEnded
	}
}

func resolveShellPreToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindGeneric:
		return model.HookAgentToolTerminalStarting
	default:
		return resolvePreToolUse(kind)
	}
}

func resolveShellPostToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindGeneric:
		return model.HookAgentToolTerminalEnded
	default:
		return resolvePostToolUse(kind)
	}
}

// #endregion 🪝️Hooks
