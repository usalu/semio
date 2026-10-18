package todos

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧰️Hook Tools

// 📖️resolveAllTestDefinitionIDs resolves all test definition IDs.
func ResolveAllTestDefinitionIDs(files []string) []string {
	var ids []string
	seen := map[string]bool{}

	for _, file := range files {
		normalized := workspace.NormalizeHookPath(file)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(workspace.GetRootDir(), normalized)
		}
		if _, err := os.Stat(absPath); os.IsNotExist(err) {
			continue
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		definitions := codebase.CollectTestDefinitionsFromContent(string(content), normalized)
		for i := range definitions {
			id := definitions[i].GetID()
			if id == "" || seen[id] {
				continue
			}
			seen[id] = true
			ids = append(ids, id)
		}
	}

	return ids
}

func ResolvePHPTestFiles(cwd string) []string {
	var files []string
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			return nil
		}
		if strings.HasSuffix(path, ".php") {
			base := strings.ToLower(filepath.Base(path))
			if strings.HasSuffix(base, "test.php") || strings.HasSuffix(base, "tests.php") || strings.Contains(strings.ToLower(filepath.Dir(path)), "tests") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🧪️latestOpenTicket finds the latest open ticket.
func LatestOpenTicket() (*model.Ticket, error) {
	tickets, err := ticketspkg.ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	var latest *model.Ticket
	for i := range tickets {
		ticket := tickets[i]
		if ticket.Status != model.TicketStatusOpen {
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

func FirstNonEmpty(values ...string) string {
	for _, value := range values {
		value = strings.TrimSpace(value)
		if value != "" {
			return value
		}
	}
	return ""
}

// 🏛️classifyTool classifies a tool name into a ToolKind.
func ClassifyTool(toolName string) model.ToolKind {
	switch toolName {
	case "manage_todo_list", "Task", "task", "todo_tool", "TodoWrite":
		return model.ToolKindPlan
	case "read_file", "grep_search", "rg", "ripgrep", "file_search", "semantic_search", "list_dir", "list_code_usages", "get_errors", "Read", "fetch_webpage", "open_simple_browser", "Grep", "Glob",
		"fs_read", "code", "grep", "glob", "web_search", "web_fetch":
		return model.ToolKindCodeSearch
	case "replace_string_in_file", "create_file", "multi_replace_string_in_file", "Edit", "Write", "editfile",
		"fs_write":
		return model.ToolKindCodeEdit
	case "run_in_terminal", "get_terminal_output", "Bash", "terminal",
		"execute_bash":
		return model.ToolKindTerminal
	case "runTests", "run_tests":
		return model.ToolKindTest
	case "run_task", "create_and_run_task":
		return model.ToolKindBuild
	case "tool_search_tool_regex":
		return model.ToolKindGeneric
	case "use_subagent", "use_aws":
		return model.ToolKindGeneric
	default:
		return model.ToolKindGeneric
	}
}

// 🏷️mergeTicketAgentPlanSteps merges ticket agent plan steps.
func MergeTicketAgentPlanSteps(existing []model.TicketAgentPlanStep, newSteps []model.HookPlanStep, second string) []model.TicketAgentPlanStep {
	existingByName := map[string]model.TicketAgentPlanStep{}
	for _, step := range existing {
		existingByName[step.Name] = step
	}
	activeNames := map[string]struct{}{}
	result := make([]model.TicketAgentPlanStep, 0, len(existing)+len(newSteps))
	for _, incoming := range newSteps {
		name := strings.TrimSpace(incoming.Name)
		if name == "" {
			continue
		}
		step := existingByName[name]
		step.Name = name
		step.Status = incoming.Status
		if step.Ideated == "" {
			step.Ideated = second
		}
		switch strings.TrimSpace(strings.ToLower(incoming.Status)) {
		case "in-progress", "in_progress", "started":
			if step.Started == "" {
				step.Started = second
			}
		case "completed", "done":
			if step.Started != "" && step.Completed == "" {
				step.Completed = second
			}
		}
		activeNames[name] = struct{}{}
		result = append(result, step)
	}
	for _, step := range existing {
		if _, ok := activeNames[step.Name]; ok {
			continue
		}
		if step.Name == "" {
			continue
		}
		if step.Abandoned != "" || step.Completed != "" || step.Started != "" {
			result = append(result, step)
			continue
		}
		if step.Ideated == "" {
			step.Ideated = second
		}
		step.Abandoned = second
		result = append(result, step)
	}
	return result
}

func ExtractHookResultToolInfo(result interface{}) (string, string) {
	switch value := result.(type) {
	case model.HookResultAgentToolStarting:
		return value.Name, ""
	case model.HookResultAgentToolEnded:
		return value.Name, ""
	case model.HookResultAgentToolTerminalStarting:
		return value.Name, value.Command
	case model.HookResultAgentToolTerminalEnded:
		return value.Name, value.Command
	default:
		return "", ""
	}
}

// ➡️appendUniqueString appends a string to a slice if it's not already present.
func AppendUniqueString(slice []string, str string) []string {
	for _, item := range slice {
		if item == str {
			return slice
		}
	}
	return append(slice, str)
}

// 🔺️deriveRepoOpFromMCPTool derives repo operation from MCP tool.
func DeriveRepoOpFromMCPTool(tool string) string {
	if tool == "" {
		return ""
	}
	const prefix = "mcp__repo__"
	if !strings.HasPrefix(tool, prefix) {
		return ""
	}
	operation := strings.TrimPrefix(tool, prefix)
	operation = strings.TrimSpace(operation)
	if operation == "" {
		return ""
	}
	return strings.ReplaceAll(operation, "_", ".")
}

// 🔻️deriveRepoOpFromCLICommand derives repo operation from CLI command.
func DeriveRepoOpFromCLICommand(cmd string) string {
	if cmd == "" {
		return ""
	}
	fields := strings.Fields(cmd)
	if len(fields) < 2 {
		return ""
	}
	cliIndex := -1
	for i, field := range fields {
		base := filepath.Base(strings.ReplaceAll(field, "\\", "/"))
		if base == "cli" || base == "cli.exe" || base == "client" || base == "client.exe" {
			cliIndex = i
			break
		}
	}
	if cliIndex == -1 {
		for i := 0; i < len(fields)-1; i++ {
			if fields[i] == "go" && fields[i+1] == "run" {
				for j := i + 2; j < len(fields); j++ {
					base := filepath.Base(strings.ReplaceAll(fields[j], "\\", "/"))
					if base == "cli" || base == "cli.exe" || base == "client" || base == "client.exe" || fields[j] == "./repo/client/mcp/go" || fields[j] == "repo/client/mcp/go" {
						cliIndex = j
						break
					}
				}
			}
			if cliIndex != -1 {
				break
			}
		}
	}
	if cliIndex == -1 || cliIndex+1 >= len(fields) {
		return ""
	}
	remaining := fields[cliIndex+1:]
	if len(remaining) == 0 || strings.HasPrefix(remaining[0], "-") {
		return ""
	}
	if len(remaining) >= 2 {
		compound := remaining[0] + "." + remaining[1]
		switch compound {
		case "ticket.open", "ticket.close", "ticket.reopen", "ticket.read",
			"goal.open", "goal.close", "goal.reopen",
			"contributor.add", "contributor.remove",
			"draft.create", "draft.delete",
			"file.create", "file.move", "file.delete",
			"folder.create", "folder.move", "folder.delete",
			"section.create", "section.move", "section.delete",
			"policy.check":
			return compound
		}
	}
	switch remaining[0] {
	case "integrate", "extract", "export", "analyze", "fix", "tree", "graphql", "move":
		return remaining[0]
	default:
		return ""
	}
}

// #endregion 🧰️Hook Tools
