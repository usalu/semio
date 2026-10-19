// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Native hook event mapping from recorded native payloads.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestNativeHookEventMappingWithRealData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow native hook event mapping test in short mode")
	}
	cases := []struct {
		name        string
		nativeEvent string
		client      string
		toolName    string
		input       string
		expectEvent HookEvent
		expectPar   string
	}{
		{"copilot/SessionStart", "SessionStart", "copilot-chat", "", `{"hookEventName":"SessionStart","sessionId":"d765d480","second":"2026-02-19T10:44:08.112Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentStarted, ""},
		{"copilot/Stop", "Stop", "copilot-chat", "", `{"hookEventName":"Stop","sessionId":"2f1e87c2","second":"2026-02-18T18:51:54.315Z","stop_hook_active":false}`, HookAgentEnded, ""},
		{"copilot/SubagentStart", "SubagentStart", "copilot-chat", "", `{"hookEventName":"SubagentStart","sessionId":"ab58fc89","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentStarted, "subagent"},
		{"copilot/SubagentStop", "SubagentStop", "copilot-chat", "", `{"hookEventName":"SubagentStop","sessionId":"ab58fc89","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentEnded, "subagent"},
		{"copilot/UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", `{"hookEventName":"UserPromptSubmit","sessionId":"d765d480","second":"2026-02-19T10:44:16.328Z","transcript_path":"/tmp/t.jsonl","cwd":"/workspaces/semio"}`, HookAgentPromptSubmitting, ""},
		{"copilot/PreCompact", "PreCompact", "copilot-chat", "", `{"second":"2026-02-18T18:41:24.718Z","hookEventName":"PreCompact","sessionId":"2f1e87c2","transcript_path":"/tmp/t.jsonl","trigger":"auto","cwd":"/workspaces/semio"}`, HookAgentCompacting, ""},
		{"copilot/PreToolUse/read_file", "PreToolUse", "copilot-chat", "read_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"read_file","second":"2026-02-19T12:30:31.702Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/grep_search", "PreToolUse", "copilot-chat", "grep_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"grep_search","second":"2026-02-19T10:44:35.056Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/file_search", "PreToolUse", "copilot-chat", "file_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"file_search","second":"2026-02-19T10:50:09.443Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_dir", "PreToolUse", "copilot-chat", "list_dir", `{"second":"2026-02-19T10:44:16.328Z","hookEventName":"PreToolUse","sessionId":"d765d480","transcript_path":"/tmp/t.jsonl","tool_name":"list_dir","tool_input":{"path":"/workspaces/semio"}}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_code_usages", "PreToolUse", "copilot-chat", "list_code_usages", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"list_code_usages","second":"2026-02-19T11:31:35.416Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/replace_string_in_file", "PreToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"replace_string_in_file","second":"2026-02-19T10:50:46.160Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/multi_replace_string_in_file", "PreToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:17.358Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/create_file", "PreToolUse", "copilot-chat", "create_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"create_file","second":"2026-02-19T12:25:00.000Z"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"run_in_terminal","second":"2026-02-18T18:42:59.593Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolTerminalStarting, ""},
		{"copilot/PreToolUse/manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:13.780Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingStarting, ""},
		{"copilot/PreToolUse/runSubagent", "PreToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"runSubagent","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolStarting, ""},
		{"copilot/PostToolUse/read_file", "PostToolUse", "copilot-chat", "read_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"read_file","second":"2026-02-18T18:38:51.951Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"copilot/PostToolUse/replace_string_in_file", "PostToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"replace_string_in_file","second":"2026-02-18T18:37:05.471Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/multi_replace_string_in_file", "PostToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:26.261Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"d765d480","hookEventName":"PostToolUse","tool_name":"run_in_terminal","second":"2026-02-19T10:43:56.761Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolTerminalEnded, ""},
		{"copilot/PostToolUse/manage_todo_list", "PostToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:20.586Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"copilot/PostToolUse/runSubagent", "PostToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"runSubagent","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolEnded, ""},
		{"copilot/PostToolUse/grep_search", "PostToolUse", "copilot-chat", "grep_search", `{"sessionId":"8a40542e","hookEventName":"PostToolUse","tool_name":"grep_search","second":"2026-02-18T18:58:48.393Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"cursor/sessionStart", "sessionStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:00Z"}`, HookAgentStarted, ""},
		{"cursor/sessionEnd", "sessionEnd", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, HookAgentEnded, ""},
		{"cursor/subagentStart", "subagentStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:01:00Z"}`, HookAgentStarted, "subagent"},
		{"cursor/subagentStop", "subagentStop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:29:00Z"}`, HookAgentEnded, "subagent"},
		{"cursor/stop", "stop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, HookAgentEnded, ""},
		{"cursor/beforeSubmitPrompt", "beforeSubmitPrompt", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:01Z","prompt":"Fix bug"}`, HookAgentPromptSubmitting, ""},
		{"cursor/preCompact", "preCompact", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, HookAgentCompacting, ""},
		{"cursor/preToolUse/read_file", "preToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:02:00Z","tool_name":"read_file"}`, HookAgentToolSearchStarting, ""},
		{"cursor/preToolUse/edit", "preToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:03:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditStarting, ""},
		{"cursor/preToolUse/terminal", "preToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:04:00Z","tool_name":"terminal"}`, HookAgentToolTerminalStarting, ""},
		{"cursor/preToolUse/task", "preToolUse", "cursor-chat", "task", `{"sessionId":"cur-001","second":"2026-02-19T10:05:00Z","tool_name":"task"}`, HookAgentToolPlanUpdatingStarting, ""},
		{"cursor/postToolUse/read_file", "postToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:06:00Z","tool_name":"read_file"}`, HookAgentToolSearchEnded, ""},
		{"cursor/postToolUse/editfile", "postToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:07:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/postToolUse/terminal", "postToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:08:00Z","tool_name":"terminal"}`, HookAgentToolTerminalEnded, ""},
		{"cursor/postToolUseFailure/edit", "postToolUseFailure", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:09:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:10:00Z"}`, HookAgentToolStarting, ""},
		{"cursor/afterMCPExecution", "afterMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:11:00Z"}`, HookAgentToolEnded, ""},
		{"cursor/beforeReadFile", "beforeReadFile", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:12:00Z"}`, HookAgentToolSearchStarting, ""},
		{"cursor/afterFileEdit", "afterFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:13:00Z"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeShellExecution", "beforeShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:14:00Z"}`, HookAgentToolTerminalStarting, ""},
		{"cursor/afterShellExecution", "afterShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, HookAgentToolTerminalEnded, ""},
		{"cursor/afterAgentResponse", "afterAgentResponse", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:16:00Z"}`, HookAgentEnded, ""},
		{"cursor/afterAgentThought", "afterAgentThought", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:17:00Z"}`, HookAgentThinkingEnded, ""},
		{"cursor/beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:18:00Z"}`, HookAgentToolSearchStarting, ""},
		{"cursor/afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:19:00Z"}`, HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", `{"second":"2026-02-18T18:46:41.123Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentPromptSubmitting, ""},
		{"windsurf/post_cascade_response", "post_cascade_response", "windsurf-chat", "", `{"second":"2026-02-18T19:00:12.032Z","agent_action_name":"post_cascade_response","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentEnded, ""},
		{"windsurf/post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", `{"second":"2026-02-18T18:45:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentStarted, ""},
		{"windsurf/pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", `{"agent_action_name":"pre_mcp_tool_use","trajectory_id":"23e6dcf5","second":"2026-02-18T18:54:57.304Z","execution_id":"d9b64466","tool_info":{"mcp_server_name":"repo","mcp_tool_name":"tree"}}`, HookAgentToolStarting, ""},
		{"windsurf/post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", `{"second":"2026-02-18T18:55:28.469Z","agent_action_name":"post_mcp_tool_use","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolEnded, ""},
		{"windsurf/pre_read_code", "pre_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:48.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolSearchStarting, ""},
		{"windsurf/post_read_code", "post_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:50.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolSearchEnded, ""},
		{"windsurf/pre_write_code", "pre_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:30.780Z","agent_action_name":"pre_write_code","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolCodeEditStarting, ""},
		{"windsurf/post_write_code", "post_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:35.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_run_command", "pre_run_command", "windsurf-chat", "", `{"second":"2026-02-18T18:54:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolTerminalStarting, ""},
		{"windsurf/post_run_command", "post_run_command", "windsurf-chat", "", `{"agent_action_name":"post_run_command","trajectory_id":"23e6dcf5","second":"2026-02-18T18:57:49.375Z","execution_id":"d9b64466","tool_info":{"command_line":"npm install","cwd":"/workspaces/semio"}}`, HookAgentToolTerminalEnded, ""},
		{"claude/SessionStart", "SessionStart", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionStart","source":"startup"}`, HookAgentStarted, ""},
		{"claude/SessionEnd", "SessionEnd", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionEnd"}`, HookAgentEnded, ""},
		{"claude/SubagentStart", "SubagentStart", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStart"}`, HookAgentStarted, "subagent"},
		{"claude/SubagentStop", "SubagentStop", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStop"}`, HookAgentEnded, "subagent"},
		{"claude/Stop", "Stop", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"Stop","stop_hook_active":false}`, HookAgentEnded, ""},
		{"claude/UserPromptSubmit", "UserPromptSubmit", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"UserPromptSubmit","prompt":"Fix the bug"}`, HookAgentPromptSubmitting, ""},
		{"claude/PreCompact", "PreCompact", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreCompact"}`, HookAgentCompacting, ""},
		{"claude/TaskCompleted", "TaskCompleted", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TaskCompleted"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"claude/Notification", "Notification", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"Notification"}`, HookAgentToolStarting, ""},
		{"claude/TeammateIdle", "TeammateIdle", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TeammateIdle"}`, HookAgentToolStarting, ""},
		{"claude/PermissionRequest", "PermissionRequest", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PermissionRequest"}`, HookAgentToolStarting, ""},
		{"claude/PreToolUse/Bash", "PreToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"go test -v"},"tool_use_id":"toolu_01WKqqc5y27LZu1KTB5GsGDu"}`, HookAgentToolTestStarting, ""},
		{"claude/PreToolUse/Read", "PreToolUse", "claude-code", "Read", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Read","tool_input":{"file_path":"/workspaces/semio/main.go"},"tool_use_id":"toolu_01Md976UFvJmmL5KaH4xzsx8"}`, HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/Edit", "PreToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl","tool_name":"Edit"}`, HookAgentToolCodeEditStarting, ""},
		{"claude/PreToolUse/Glob", "PreToolUse", "claude-code", "Glob", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"Glob","tool_input":{"pattern":"**/*.json"}}`, HookAgentToolStarting, ""},
		{"claude/PreToolUse/Grep", "PreToolUse", "claude-code", "Grep", `{"session_id":"e51a2976","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreToolUse","tool_name":"Grep","tool_input":{"pattern":"BlockedToolPatterns"}}`, HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/mcp_tree", "PreToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"mcp__repo__tree","tool_input":{"query":"hooks"}}`, HookAgentToolStarting, ""},
		{"claude/PostToolUse/Bash", "PostToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","tool_name":"Bash","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl"}`, HookAgentToolTerminalEnded, ""},
		{"claude/PostToolUse/Edit", "PostToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","tool_name":"Edit","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"claude/PostToolUse/Read", "PostToolUse", "claude-code", "Read", `{"session_id":"167906cd","tool_name":"Read","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"claude/PostToolUse/TodoWrite", "PostToolUse", "claude-code", "TodoWrite", `{"session_id":"167906cd","tool_name":"TodoWrite","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"claude/PostToolUse/mcp_tree", "PostToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd","tool_name":"mcp__repo__tree","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolEnded, ""},
		{"claude/PostToolUseFailure/Bash", "PostToolUseFailure", "claude-code", "Bash", `{"session_id":"167906cd","tool_name":"Bash","transcript_path":"/tmp/t.jsonl","error":"command failed"}`, HookAgentToolTerminalEnded, ""},
		{"droid/SessionStart", "SessionStart", "droid", "", `{"session_id":"droid-001","second":"2026-02-18T18:47:06.000Z"}`, HookAgentStarted, ""},
		{"droid/PreToolUse/Bash", "PreToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:06.000Z"}`, HookAgentToolTerminalStarting, ""},
		{"droid/PostToolUse/Bash", "PostToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:10.000Z"}`, HookAgentToolTerminalEnded, ""},
		{"codex/SessionStart", "SessionStart", "codex", "", `{"session_id":"codex-001","second":"2026-02-18T18:50:00.000Z"}`, HookAgentStarted, ""},
		{"codex/PreToolUse/Read", "PreToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:05.000Z"}`, HookAgentToolSearchStarting, ""},
		{"codex/PostToolUse/Read", "PostToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:10.000Z"}`, HookAgentToolSearchEnded, ""},
		{"antigravity/SessionStart", "SessionStart", "antigravity-chat", "", `{"session_id":"ag-001","second":"2026-02-18T18:55:00.000Z"}`, HookAgentStarted, ""},
		{"antigravity/PreToolUse/Task", "PreToolUse", "antigravity-chat", "Task", `{"session_id":"ag-001","tool_name":"Task","second":"2026-02-18T18:55:05.000Z"}`, HookAgentToolPlanUpdatingStarting, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			input := json.RawMessage(tc.input)
			event, parent, err := ResolveHookEvent(tc.nativeEvent, tc.client, tc.toolName, input)
			if err != nil {
				t.Fatalf("ResolveHookEvent error: %v", err)
			}
			if event != tc.expectEvent {
				t.Errorf("event: want %s, got %s", tc.expectEvent, event)
			}
			if parent != tc.expectPar {
				t.Errorf("parent: want %q, got %q", tc.expectPar, parent)
			}
			tmpDir := t.TempDir()
			writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
			inputSecond := extractSecondFromInput(string(input))
			var secondStr string
			if inputSecond == 0 {
				secondStr = time.Now().UTC().Format(time.RFC3339)
			} else {
				secondStr = fmt.Sprintf("%d", inputSecond)
			}
			hctx := HookContext{
				Event:      event,
				Client:     tc.client,
				Second:     secondStr,
				RepoRoot:   tmpDir,
				ToolName:   tc.toolName,
				Input:      input,
				ParentInfo: parent,
			}
			RunHook(hctx)
			sessionID := extractSessionIDFromInput(input)
			if sessionID == "" {
				sessionID = "unknown"
			}
			logNow := time.Now().UTC()
			logDir := sessionLogDir(tmpDir, logNow.Year(), int(logNow.Month()), logNow.Day(), sessionID)
			entries, err := os.ReadDir(logDir)
			if err != nil {
				t.Fatalf("log dir: %v", err)
			}
			if len(entries) != 1 || entries[0].Name() != "session.json" {
				names := make([]string, len(entries))
				for i, e := range entries {
					names[i] = e.Name()
				}
				t.Fatalf("want only session.json in session dir, got %v", names)
			}
			data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
			if err != nil {
				t.Fatalf("read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err != nil {
				t.Fatalf("invalid session JSON: %v", err)
			}
			if len(meta.Events) == 0 {
				t.Fatalf("expected at least one event in session.json")
			}
			var entry *HookLogEntry
			for i := range meta.Events {
				var probe map[string]interface{}
				if err := json.Unmarshal(meta.Events[i].Event, &probe); err == nil && probe["kind"] == string(event) {
					entry = &meta.Events[i]
					break
				}
			}
			if entry == nil {
				t.Fatalf("could not find event kind %s in session.json events (%d total)", event, len(meta.Events))
			}
			var evt map[string]interface{}
			if err := json.Unmarshal(entry.Event, &evt); err != nil {
				t.Fatalf("cannot unmarshal event: %v", err)
			}
			if evt["kind"] != string(event) {
				t.Errorf("log kind: want %s, got %v", event, evt["kind"])
			}
			if evt["client"] != tc.client {
				t.Errorf("log client: want %s, got %v", tc.client, evt["client"])
			}
			if entry.Native.Event == nil {
				t.Error("log native.event: want non-nil")
			}
			wantSession := resolveEventSessionID(extractSessionIDFromInput(input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("log session: want %s, got %v", wantSession, evt["session"])
			}
			wantSecond := resolveEventSecondID(secondStr)
			if wantSecond != "" && evt["second"] != wantSecond {
				t.Errorf("log second: want %s, got %v", wantSecond, evt["second"])
			}
			wantTranscript := extractTranscriptFromInput(input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("log transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if entry.Response.Blocked != nil {
				t.Error("log response.blocked: want nil for non-blocked event")
			}
		})
	}
}
