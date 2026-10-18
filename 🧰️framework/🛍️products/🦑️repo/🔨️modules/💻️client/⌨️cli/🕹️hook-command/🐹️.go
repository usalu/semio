// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook command and execution.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// 🔤️ValidateHookEvent checks if the given string is a valid hook event.
func ValidateHookEvent(s string) (HookEvent, error) {
	for _, e := range AllHookEvents {
		if string(e) == s {
			return e, nil
		}
	}
	return "", fmt.Errorf("invalid hook event %q, valid events: %s", s, strings.Join(hookEventStrings(), ", "))
}

func hookEventStrings() []string {
	result := make([]string, len(AllHookEvents))
	for i, e := range AllHookEvents {
		result[i] = string(e)
	}
	return result
}

// runHookExecution runs the hook pipeline (shared by CLI `hook` and per-IDE `RunHookFor`).
func runHookExecution(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	return runHookExecutionCtx(context.Background(), client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
}

func runHookExecutionCtx(ctx context.Context, client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	done := make(chan error, 1)
	go func() {
		done <- runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
	}()
	select {
	case err := <-done:
		return err
	case <-ctx.Done():
		return fmt.Errorf("hook timed out: %w", ctx.Err())
	}
}

func runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	if toolName == "" {
		if tn := extractToolNameFromStdin(input); tn != "" {
			toolName = tn
		}
	}
	event, resolvedParent, err := ResolveHookEvent(eventStr, client, toolName, input)
	if err != nil {
		return err
	}
	if parentInfo == "" {
		parentInfo = resolvedParent
	}
	hctx := HookContext{
		Event:      event,
		Client:     client,
		Second:     time.Now().UTC().Format(time.RFC3339),
		RepoRoot:   repoRoot,
		ToolName:   toolName,
		ToolArgs:   toolArgs,
		FilePath:   filePath,
		ParentInfo: parentInfo,
		Input:      input,
	}
	result := RunHook(hctx)
	provider := GetEditorProvider(client)
	if provider != nil && provider.Kind() == "copilot-chat" {
		hookEventName := extractHookEventNameFromStdin(input)
		if hookEventName == "" {
			hookEventName = provider.NativeEventFromHookEvent(event, parentInfo)
		}
		out := provider.FormatHookOutput(hookEventName, result)
		fmt.Fprintln(stdout, out)
		return nil
	}
	if jsonMode {
		out, _ := json.Marshal(result)
		fmt.Fprintln(stdout, string(out))
		return nil
	}
	if !result.IsAllowed() {
		fmt.Fprintln(stderr, result.GetMessage())
		return ExitError{Code: 2}
	}
	msg := result.GetMessage()
	if msg != "" {
		fmt.Fprintln(stdout, msg)
	}
	return nil
}

// 🆕️hookCommand creates the `hook <event> <client>` command command.
func hookCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "hook <event> <client>",
		Short: "Run a lifecycle hook (git or agent)",
		Long: `Run a lifecycle hook for a given event and client.
Accepts neutral repo events or native client events (inlet adapter resolves to neutral).

📦️ Version hooks:
  version.checkpoint.starting  Run before a version checkpoint
  version.checkpoint.ended     Run after a version checkpoint (e.g. post-checkpoint)
  version.checkin.starting     Run before a version checkin (e.g. fast-forward to main)
  version.checkin.ended        Run after a version checkin
  version.checkout.starting    Run before a version checkout (e.g. squash merge)
  version.checkout.ended       Run after a version checkout

🤖️ Neutral agent hooks:
  agent.started                Agent session started
  agent.ended                  Agent session stopped
  agent.prompt.submitting      User submitting a prompt
  agent.compacting             Context compaction event
  agent.tool.starting          Tool starting (generic, excludes plan, search, code, test, build, terminal)
  agent.tool.ended             Tool completed (generic, excludes plan, search, code, test, build, terminal)
  agent.tool.plan.updating.starting  Plan/task list updating start
  agent.tool.plan.updating.ended     Plan/task list updating end
  agent.file.read.starting     Code search/read starting
  agent.file.read.ended        Code search/read ended
  agent.tool.code.edit.starting Code edit starting
  agent.tool.code.edit.ended   Code edit completed
  agent.tool.test.starting     Test starting
  agent.tool.test.ended        Test completed
  agent.tool.build.starting    Build starting
  agent.tool.build.ended       Build completed
  agent.tool.terminal.starting Terminal command starting (supports blocking)
  agent.tool.terminal.ended    Terminal command completed
  agent.thinking.starting      Agent thinking started
  agent.thinking.ended         Agent thinking completed (cursor: afterAgentThought)

🏢️ Native client events (resolved by inlet adapter):
  copilot-chat:    SessionStart, Stop, SubagentStart, SubagentStop, UserPromptSubmit, PreCompact, PreToolUse, PostToolUse
  cursor-chat:     sessionStart, sessionEnd, stop, subagentStart, subagentStop, beforeSubmitPrompt, preCompact, preToolUse, postToolUse, beforeReadFile, afterFileEdit, beforeShellExecution, afterShellExecution, beforeMCPExecution, afterMCPExecution
  windsurf-chat:   pre_user_prompt, post_cascade_response, post_setup_worktree, pre_mcp_tool_use, post_mcp_tool_use, pre_read_code, post_read_code, pre_write_code, post_write_code, pre_run_command, post_run_command
  claude-code/droid/codex/antigravity: SessionStart, SessionEnd, SubagentStart, SubagentStop, Stop, UserPromptSubmit, PreCompact, PreToolUse, PostToolUse, PostToolUseFailure, TaskCompleted, Notification
  kiro-cli:        agentSpawn, userPromptSubmit, preToolUse, postToolUse, stop`,
		Args: command.RangeArgs(1, 2),
		RunE: func(cmd *command.Command, args []string) error {
			eventStr := args[0]
			client := ""
			if len(args) > 1 {
				client = args[1]
			}
			toolName, _ := cmd.Flags().GetString("tool-name")
			toolArgs, _ := cmd.Flags().GetString("tool-args")
			filePath, _ := cmd.Flags().GetString("file")
			parentInfo, _ := cmd.Flags().GetString("parent")
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, _ := os.Getwd()
				repoRoot = findRepoRoot(cwd)
			}
			var input json.RawMessage
			stdinReader := cmd.InOrStdin()
			if stdinReader == os.Stdin {
				if stat, err := os.Stdin.Stat(); err == nil && (stat.Mode()&os.ModeCharDevice) == 0 {
					if data, err := io.ReadAll(os.Stdin); err == nil && len(data) > 0 {
						input = json.RawMessage(data)
					}
				}
			} else {
				if data, err := io.ReadAll(stdinReader); err == nil && len(data) > 0 {
					input = json.RawMessage(data)
				}
			}
			ctx := cmd.Context()
			if config.Timeout > 0 {
				ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
				defer cancel()
				ctx = ctxWithTimeout
			}
			return runHookExecutionCtx(ctx, client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, config.IsJSON(), cmd.OutOrStdout(), cmd.ErrOrStderr())
		},
	}
	cmd.Flags().String("tool-name", "", "Tool name for tool-related events")
	cmd.Flags().String("tool-args", "", "Tool arguments for tool-related events")
	cmd.Flags().String("file", "", "File path for code events")
	cmd.Flags().String("parent", "", "Parent agent info for agent events")
	return cmd
}

// 🔶️ResolveHookEvent resolves a hook event.
func ResolveHookEvent(name string, client string, toolName string, input json.RawMessage) (HookEvent, string, error) {
	if event, err := ValidateHookEvent(name); err == nil {
		return event, "", nil
	}
	kind := ToolKindGeneric
	if strings.TrimSpace(toolName) != "" {
		kind = classifyTool(toolName)
	}
	if command := extractCommandFromStdin(input); command != "" {
		segment, _ := extractTestSegmentFromCommand(command)
		commandKind := classifyCommandKind(firstNonEmpty(segment, command))
		if kind == ToolKindGeneric || kind == ToolKindTerminal || commandKind == ToolKindTest || commandKind == ToolKindBuild || commandKind == ToolKindCodeSearch || commandKind == ToolKindCodeEdit {
			kind = commandKind
		}
	}
	if toolName == "Glob" && len(input) > 0 {
		kind = ToolKindGeneric
	}
	provider := GetEditorProvider(client)
	if provider != nil {
		return provider.ResolveNativeEvent(name, kind)
	}
	return resolveClaudeCompatibleEvent(name, kind)
}

// 🔹️RunHook runs a hook.
func RunHook(ctx HookContext) HookResult {
	previousRoot := rootDir
	defer func() {
		rootDir = previousRoot
		invalidateGitignore()
		InvalidateTechnologyCache()
	}()
	result := runHookCore(ctx)
	writeHookArtifacts(ctx, result)
	trackHookInOpenTicket(ctx, result)
	return result
}

func runHookCore(ctx HookContext) HookResult {
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = GetRootDir()
	}
	if repoRoot != "" {
		SetRootDir(repoRoot)
	}
	result := dispatchHook(ctx)
	hookResult, ok := result.(HookResult)
	if !ok {
		return HookResultBase{Allowed: false, Message: "invalid hook result"}
	}
	return hookResult
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		value = strings.TrimSpace(value)
		if value != "" {
			return value
		}
	}
	return ""
}

func writeHookArtifacts(ctx HookContext, result HookResult) {
	if HookEventKind(ctx.Event) == HookKindVersion {
		return
	}
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = GetRootDir()
	}
	if repoRoot == "" {
		return
	}
	repoRoot = findRepoRoot(repoRoot)
	SetRootDir(repoRoot)
	cfg := LoadRepoConfig(repoRoot)
	if !cfg.Logging.Session {
		return
	}
	now := time.Now().UTC()
	sessionID := extractSessionIDFromInput(ctx.Input)
	if sessionID == "" && ctx.Client == "kiro-cli" {
		sessionID = fmt.Sprintf("kiro-%d", os.Getppid())
	}
	if sessionID == "" {
		sessionID = "unknown"
	}
	logDir := sessionLogDir(repoRoot, now.Year(), int(now.Month()), now.Day(), sessionID)
	if err := os.MkdirAll(logDir, 0o755); err != nil {
		return
	}
	writeSessionHookLog(ctx, result, logDir, sessionID, cfg.Logging)
	if cfg.Logging.Operations {
		logRepoOperationHook(ctx, result, logDir, now, sessionID, cfg.Logging)
	}
}

// 📁️sessionLogDir answers the directory a session log lives in.
func sessionLogDir(repoRoot string, year int, month int, day int, sessionID string) string {
	return filepath.Join(RepoMetaDirForRoot(repoRoot), "⚡️cache", "🤖️generated", FormatYearDir(year%100), FormatMonthDir(month), FormatDayDir(day), sessionID)
}

func writeSessionHookLog(ctx HookContext, result HookResult, logDir string, sessionID string, lg LoggingConfig) {
	metaPath := filepath.Join(logDir, "session.json")
	var meta SessionMeta
	if data, err := os.ReadFile(metaPath); err == nil {
		_ = json.Unmarshal(data, &meta)
	}
	inputMap := decodeHookInputMap(ctx.Input)
	second := firstNonEmpty(ctx.Second, findNestedStringValue(inputMap, "second"))
	checkpoint := resolveEventCheckpointID(extractCheckpointSHAFromInput(ctx.Input))
	transcript := extractTranscriptFromInput(ctx.Input)
	if meta.ID == "" {
		meta.ID = sessionID
		meta.URI = "repo://session/" + sessionID
		meta.Client = ctx.Client
		meta.Second = second
		meta.Checkpoint = checkpoint
		meta.Contributor = "unknown"
		meta.Transcript = transcript
	} else {
		if meta.Client == "" {
			meta.Client = ctx.Client
		}
		if meta.Second == "" {
			meta.Second = second
		}
		if meta.Checkpoint == "" {
			meta.Checkpoint = checkpoint
		}
		if meta.Transcript == "" {
			meta.Transcript = transcript
		}
	}
	eventMap := map[string]interface{}{
		"kind":        string(ctx.Event),
		"client":      ctx.Client,
		"session":     resolveEventSessionID(sessionID),
		"second":      resolveEventSecondID(second),
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	}
	if transcript != "" {
		eventMap["transcript"] = transcript
	}
	eventJSON, _ := json.Marshal(eventMap)
	entry := EventEntry{Event: eventJSON}
	if lg.includeNative() && len(ctx.Input) > 0 {
		entry.Native = &struct {
			Event json.RawMessage `json:"event"`
		}{Event: ctx.Input}
	}
	if lg.includeResponse() {
		entry.Response = &struct {
			Blocked *bool  `json:"blocked,omitempty"`
			Message string `json:"message,omitempty"`
			Reason  string `json:"reason,omitempty"`
		}{}
		if !result.IsAllowed() {
			blocked := true
			entry.Response.Blocked = &blocked
			entry.Response.Message = result.GetMessage()
			entry.Response.Reason = result.GetMessage()
		}
	}
	meta.Events = append(meta.Events, entry)
	if lg.Plan && ctx.Event == HookAgentToolPlanUpdatingEnded {
		steps := extractPlanStepsFromInput(ctx.Input, ctx.ToolArgs)
		if len(steps) > 0 {
			var existing []TicketAgentPlanStep
			if meta.Plan != nil {
				existing = meta.Plan.Steps
			}
			meta.Plan = &TicketAgentPlan{Steps: mergeTicketAgentPlanSteps(existing, steps, firstNonEmpty(second, time.Now().UTC().Format(time.RFC3339)))}
		}
	}
	_ = WriteJSONFile(metaPath, meta)
}
