// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook session and ticket agent tracking.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	idgen "github.com/usalu/semio/repo/client/internal/id"
	"os"
	"path/filepath"
	"strings"
	"time"
)

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
func extractTranscriptFromInput(input json.RawMessage) string {
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

// 🛤️normalizeHookPath holds the data fields for a normalizeHookPath record.
func normalizeHookPath(path string) string {
	path = strings.TrimSpace(path)
	if path == "" {
		return ""
	}
	if filepath.IsAbs(path) {
		path = GetRelativePath(path)
	}
	path = NormalizePath(path)
	return strings.TrimPrefix(path, "./")
}

// 🎫️ensureTicketAgent holds the data fields for a ensureTicketAgent record.
func ensureTicketAgent(ticket *Ticket, sessionID string, client string) int {
	for i := range ticket.Agents {
		if ticket.Agents[i].Session == sessionID {
			if ticket.Agents[i].Client == "" {
				ticket.Agents[i].Client = client
			}
			return i
		}
	}
	ticket.Agents = append(ticket.Agents, TicketAgent{
		Session:     sessionID,
		Contributor: GetGitAuthorAlias(),
		System:      GetSystem(),
		Client:      client,
	})
	return len(ticket.Agents) - 1
}

// 📬️trackHookInOpenTicket updates agent metadata and plan in the latest open ticket.
func trackHookInOpenTicket(hctx HookContext, result HookResult) {
	ticket, err := latestOpenTicket()
	if err != nil || ticket == nil {
		return
	}
	sessionID := extractSessionIDFromInput(hctx.Input)
	if sessionID == "" {
		if len(ticket.Agents) > 0 {
			sessionID = ticket.Agents[len(ticket.Agents)-1].Session
		} else {
			sessionID = generateHookSessionID()
		}
	}
	agentIndex := ensureTicketAgent(ticket, sessionID, hctx.Client)
	agent := &ticket.Agents[agentIndex]
	llm := extractLLMFromInput(hctx.Input)
	if llm != "" && agent.LLM == "" {
		agent.LLM = llm
	}
	effort := extractEffortFromInput(hctx.Input)
	if effort != "" && agent.Effort == "" {
		agent.Effort = effort
	}
	transcript := extractTranscriptFromInput(hctx.Input)
	if transcript != "" && agent.Transcript == "" {
		agent.Transcript = transcript
	}
	if hctx.Event == HookAgentToolPlanUpdatingEnded {
		second := hctx.Second
		if second == "" {
			second = time.Now().UTC().Format(time.RFC3339)
		}
		if planResult, ok := result.(HookResultAgentToolPlanUpdating); ok && len(planResult.Steps) > 0 {
			var existing []TicketAgentPlanStep
			if agent.Plan != nil {
				existing = agent.Plan.Steps
			}
			merged := mergeTicketAgentPlanSteps(existing, planResult.Steps, second)
			agent.Plan = &TicketAgentPlan{Steps: merged}
		}
	}
	_ = SaveTicket(ticket)
}

// ♻️computeCheckpointDiff computes a semantic code diff from git changes for a checkpoint.
func computeCheckpointDiff(repoRoot string, checkpointID string) *CheckpointDiff {
	diff := &CheckpointDiff{}
	stdout, _, exitCode := ExecCommand("git", []string{"diff-tree", "--no-commit-id", "-r", "--name-status", checkpointID}, repoRoot)
	if exitCode != 0 {
		return diff
	}
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		if len(parts) < 2 {
			continue
		}
		status := parts[0]
		filePath := parts[1]
		fileID := buildFileID(filePath, nil)
		dir := filepath.Dir(filePath)
		folderID := ""
		if dir != "." && dir != "" {
			folderID = buildFolderID(dir, nil)
		}
		var bundleID, technologyID string
		bundle := GetBundleByPath(filePath)
		if bundle != nil {
			bundleID = bundle.GetID()
			technologyName := bundle.TechnologyName
			if technologyName == "" {
				nameParts := strings.SplitN(bundle.Name, "/", 2)
				technologyName = nameParts[0]
			}
			pKind := DeriveTechnologyKind(technologyName)
			technologyID = emojiText(string(pKind)) + Flat(technologyName)
		}
		switch {
		case status == "A":
			diff.Files.Created = appendUniqueString(diff.Files.Created, fileID)
			if folderID != "" {
				diff.Folders.Modified = appendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = appendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = appendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case status == "D":
			diff.Files.Deleted = appendUniqueString(diff.Files.Deleted, fileID)
			if folderID != "" {
				diff.Folders.Modified = appendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = appendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = appendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case strings.HasPrefix(status, "R"):
			if len(parts) >= 3 {
				oldPath := parts[1]
				newPath := parts[2]
				oldFileID := buildFileID(oldPath, nil)
				newFileID := buildFileID(newPath, nil)
				diff.Files.Renamed = append(diff.Files.Renamed, CheckpointDiffRename{From: oldFileID, To: newFileID})
			} else {
				diff.Files.Modified = appendUniqueString(diff.Files.Modified, fileID)
			}
			if folderID != "" {
				diff.Folders.Modified = appendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = appendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = appendUniqueString(diff.Technologies.Modified, technologyID)
			}
		default:
			diff.Files.Modified = appendUniqueString(diff.Files.Modified, fileID)
			if folderID != "" {
				diff.Folders.Modified = appendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = appendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = appendUniqueString(diff.Technologies.Modified, technologyID)
			}
		}
	}
	return diff
}

// 🏪️storeCheckpointDiff stores a semantic code diff at .🧬semio/🦑️repo/🔀️/YY/MM/DD/<checkpoint-id>.json.
func storeCheckpointDiff(repoRoot string, checkpointID string) {
	if checkpointID == "" || checkpointID == "unknown" {
		return
	}
	diff := computeCheckpointDiff(repoRoot, checkpointID)
	now := time.Now().UTC()
	yy := FormatYearDir(now.Year() % 100)
	mm := FormatMonthDir(int(now.Month()))
	dd := FormatDayDir(now.Day())
	repoRoot = findRepoRoot(repoRoot)
	diffDir := filepath.Join(RepoMetaDirForRoot(repoRoot), "⚡️cache", "🔀️diff", yy, mm, dd)
	if err := os.MkdirAll(diffDir, 0755); err != nil {
		return
	}
	data, err := json.MarshalIndent(diff, "", "  ")
	if err != nil {
		return
	}
	_ = os.WriteFile(filepath.Join(diffDir, checkpointID+".json"), data, 0644)
}

// ✔️runCheckpointStartingHook holds the data fields for a runCheckpointStartingHook record.
func runCheckpointStartingHook(hctx HookContext) HookResultVersionCheckpointStarting {
	repoRoot := hctx.RepoRoot
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return HookResultVersionCheckpointStarting{HookResultBase: HookResultBase{Allowed: false, Message: fmt.Sprintf("cannot determine cwd: %v", err)}}
		}
		repoRoot = findRepoRoot(cwd)
	}
	SetRootDir(repoRoot)
	return HookResultVersionCheckpointStarting{
		HookResultBase: HookResultBase{Allowed: true, Message: "checkpoint starting hook passed"},
		Checkpoint:     extractCheckpointSHAFromInput(hctx.Input),
		Description:    extractCheckpointMessageFromInput(hctx.Input, repoRoot),
	}
}

// 📡️resolveEventSessionID resolves the session ID from input.
func resolveEventSessionID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 📝️resolveEventSecondID resolves the second ID from input.
func resolveEventSecondID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 🧲️extractSecondFromInput extracts second from input.
func extractSecondFromInput(input string) int {
	// This is a placeholder implementation
	return 0
}

func generateHookSessionID() string {
	return idgen.New().String()
}

// 🧪️latestOpenTicket finds the latest open ticket.
func latestOpenTicket() (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	var latest *Ticket
	for i := range tickets {
		ticket := tickets[i]
		if ticket.Status != TicketStatusOpen {
			continue
		}
		if latest == nil ||
			ticket.Year > latest.Year ||
			(ticket.Year == latest.Year && ticket.Month > latest.Month) ||
			(ticket.Year == latest.Year && ticket.Month == latest.Month && ticket.Day > latest.Day) ||
			(ticket.Year == latest.Year && ticket.Month == latest.Month && ticket.Day == latest.Day && ticket.Slug > latest.Slug) {
			copy := ticket
			latest = &copy
		}
	}
	return latest, nil
}

// 🧲️extractSessionIDFromInput extracts session ID from input.
func extractSessionIDFromInput(input json.RawMessage) string {
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
	transcript := extractTranscriptFromInput(input)
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
