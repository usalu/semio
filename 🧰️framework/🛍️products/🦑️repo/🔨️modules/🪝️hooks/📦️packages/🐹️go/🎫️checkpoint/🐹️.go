// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️checkpoint carries ticket agent tracking and the checkpoint diff computation and storage.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	time "time"

	contributors "github.com/usalu/semio/repo/contributors"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	tickets "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔑️Engine Events

func spansEntireFile(lines map[int]struct{}, content string, lineCount int) bool {
	if len(lines) == 0 {
		return true
	}
	effectiveLineCount := lineCount
	if strings.HasSuffix(content, "\n") && effectiveLineCount > 1 {
		effectiveLineCount--
	}
	if len(lines) < effectiveLineCount {
		return false
	}
	for line := 1; line <= effectiveLineCount; line++ {
		if _, ok := lines[line]; !ok {
			return false
		}
	}
	return true
}

// #endregion 🔑️Engine Events

// #region 🦀️Hooks

// 🎫️ensureTicketAgent holds the data fields for a ensureTicketAgent record.
func ensureTicketAgent(ticket *model.Ticket, sessionID string, client string) int {
	for i := range ticket.Agents {
		if ticket.Agents[i].Session == sessionID {
			if ticket.Agents[i].Client == "" {
				ticket.Agents[i].Client = client
			}
			return i
		}
	}
	ticket.Agents = append(ticket.Agents, model.TicketAgent{
		Session:     sessionID,
		Contributor: contributors.GetGitAuthorAlias(),
		System:      languages.GetSystem(),
		Client:      client,
	})
	return len(ticket.Agents) - 1
}

// 📬️trackHookInOpenTicket updates agent metadata and plan in the latest open ticket.
func trackHookInOpenTicket(hctx model.HookContext, result model.HookResult) {
	ticket, err := todos.LatestOpenTicket()
	if err != nil || ticket == nil {
		return
	}
	sessionID := ExtractSessionIDFromInput(hctx.Input)
	if sessionID == "" {
		if len(ticket.Agents) > 0 {
			sessionID = ticket.Agents[len(ticket.Agents)-1].Session
		} else {
			sessionID = contributors.GenerateHookSessionID()
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
	transcript := ExtractTranscriptFromInput(hctx.Input)
	if transcript != "" && agent.Transcript == "" {
		agent.Transcript = transcript
	}
	if hctx.Event == model.HookAgentToolPlanUpdatingEnded {
		second := hctx.Second
		if second == "" {
			second = time.Now().UTC().Format(time.RFC3339)
		}
		if planResult, ok := result.(model.HookResultAgentToolPlanUpdating); ok && len(planResult.Steps) > 0 {
			var existing []model.TicketAgentPlanStep
			if agent.Plan != nil {
				existing = agent.Plan.Steps
			}
			merged := todos.MergeTicketAgentPlanSteps(existing, planResult.Steps, second)
			agent.Plan = &model.TicketAgentPlan{Steps: merged}
		}
	}
	_ = tickets.SaveTicket(ticket)
}

// ♻️computeCheckpointDiff computes a semantic code diff from git changes for a checkpoint.
func computeCheckpointDiff(repoRoot string, checkpointID string) *model.CheckpointDiff {
	diff := &model.CheckpointDiff{}
	stdout, _, exitCode := workspace.ExecCommand("git", []string{"diff-tree", "--no-commit-id", "-r", "--name-status", checkpointID}, repoRoot)
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
		fileID := model.BuildFileID(filePath, nil)
		dir := filepath.Dir(filePath)
		folderID := ""
		if dir != "." && dir != "" {
			folderID = model.BuildFolderID(dir, nil)
		}
		var bundleID, technologyID string
		bundle := model.GetBundleByPath(filePath)
		if bundle != nil {
			bundleID = bundle.GetID()
			technologyName := bundle.TechnologyName
			if technologyName == "" {
				nameParts := strings.SplitN(bundle.Name, "/", 2)
				technologyName = nameParts[0]
			}
			pKind := model.DeriveTechnologyKind(technologyName)
			technologyID = model.EmojiText(string(pKind)) + workspace.Flat(technologyName)
		}
		switch {
		case status == "A":
			diff.Files.Created = todos.AppendUniqueString(diff.Files.Created, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case status == "D":
			diff.Files.Deleted = todos.AppendUniqueString(diff.Files.Deleted, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case strings.HasPrefix(status, "R"):
			if len(parts) >= 3 {
				oldPath := parts[1]
				newPath := parts[2]
				oldFileID := model.BuildFileID(oldPath, nil)
				newFileID := model.BuildFileID(newPath, nil)
				diff.Files.Renamed = append(diff.Files.Renamed, model.CheckpointDiffRename{From: oldFileID, To: newFileID})
			} else {
				diff.Files.Modified = todos.AppendUniqueString(diff.Files.Modified, fileID)
			}
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		default:
			diff.Files.Modified = todos.AppendUniqueString(diff.Files.Modified, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		}
	}
	return diff
}

// 🏪️storeCheckpointDiff stores a semantic code diff at .🦑️repo/🔀️/YY/MM/DD/<checkpoint-id>.json.
func storeCheckpointDiff(repoRoot string, checkpointID string) {
	if checkpointID == "" || checkpointID == "unknown" {
		return
	}
	diff := computeCheckpointDiff(repoRoot, checkpointID)
	now := time.Now().UTC()
	yy := model.FormatYearDir(now.Year() % 100)
	mm := model.FormatMonthDir(int(now.Month()))
	dd := model.FormatDayDir(now.Day())
	repoRoot = workspace.FindRepoRoot(repoRoot)
	diffDir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🔀️diff", yy, mm, dd)
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
func runCheckpointStartingHook(hctx model.HookContext) model.HookResultVersionCheckpointStarting {
	repoRoot := hctx.RepoRoot
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return model.HookResultVersionCheckpointStarting{HookResultBase: model.HookResultBase{Allowed: false, Message: fmt.Sprintf("cannot determine cwd: %v", err)}}
		}
		repoRoot = workspace.FindRepoRoot(cwd)
	}
	workspace.SetRootDir(repoRoot)
	return model.HookResultVersionCheckpointStarting{
		HookResultBase: model.HookResultBase{Allowed: true, Message: "checkpoint starting hook passed"},
		Checkpoint:     extractCheckpointSHAFromInput(hctx.Input),
		Description:    extractCheckpointMessageFromInput(hctx.Input, repoRoot),
	}
}

// #endregion 🦀️Hooks
