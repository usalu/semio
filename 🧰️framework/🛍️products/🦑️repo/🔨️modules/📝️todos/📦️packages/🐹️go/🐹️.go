// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📝️todos is the todos domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package todos

import (
	bytes "bytes"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	unicode "unicode"
	utf8 "unicode/utf8"

	codebase "github.com/usalu/semio/repo/codebase"
	events "github.com/usalu/semio/repo/events"
	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 📰️Todos

// 🚚️RemoveLineFromMarkdown removes a todo item from a markdown file on disk.
func RemoveLineFromMarkdown(path, name string) {
	content, err := os.ReadFile(path)
	if err != nil {
		return
	}
	os.WriteFile(path, []byte(RemoveFromMarkdown(string(content), name)), 0644)
}

// 🚚️RemoveLineFromFile removes a one-based line from a file on disk.
func RemoveLineFromFile(path string, lineNum int) {
	content, err := os.ReadFile(path)
	if err != nil {
		return
	}
	os.WriteFile(path, []byte(RemoveFromFile(string(content), int64(lineNum))), 0644)
}

// 📝️ReplaceLineInMarkdown rewrites a todo markdown entry of a file on disk.
func ReplaceLineInMarkdown(path, oldName, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	rewritten, err := ReplaceInMarkdown(string(content), oldName, newName, newDescription)
	if err != nil {
		return fmt.Errorf("todo entry not found in %s", path)
	}
	return os.WriteFile(path, []byte(rewritten), 0644)
}

// 📝️ReplaceLineInFile rewrites an inline todo comment of a file on disk.
func ReplaceLineInFile(path string, lineNum int, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	rewritten, err := ReplaceInFile(string(content), int64(lineNum), newName, newDescription)
	if err != nil {
		return fmt.Errorf("todo comment not found at %s:%d", path, lineNum)
	}
	return os.WriteFile(path, []byte(rewritten), 0644)
}

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

// 📋️extractPlanStepsFromInput extracts plan steps from input JSON or toolArgs string.
func ExtractPlanStepsFromInput(input json.RawMessage, toolArgs string) []model.HookPlanStep {
	var data map[string]interface{}
	var source interface{}

	if input != nil {
		if err := json.Unmarshal(input, &data); err != nil {
			if toolArgs != "" {
				if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
					return nil
				}
			} else {
				return nil
			}
		}
	} else if toolArgs != "" {
		if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
			return nil
		}
	} else {
		return nil
	}

	if ti, ok := data["tool_input"]; ok {
		source = ti
	} else {
		source = data
	}

	sourceMap, ok := source.(map[string]interface{})
	if !ok {
		return nil
	}

	var steps []model.HookPlanStep
	if todoList, ok := sourceMap["todoList"]; ok {
		if todoListSlice, ok := todoList.([]interface{}); ok {
			for _, item := range todoListSlice {
				if itemMap, ok := item.(map[string]interface{}); ok {
					name, _ := itemMap["title"].(string)
					status, _ := itemMap["status"].(string)
					if name != "" {
						steps = append(steps, model.HookPlanStep{Name: name, Status: status})
					}
				}
			}
		}
	}
	if stepsList, ok := sourceMap["steps"]; ok {
		if stepsSlice, ok := stepsList.([]interface{}); ok {
			for _, item := range stepsSlice {
				if itemMap, ok := item.(map[string]interface{}); ok {
					name, _ := itemMap["name"].(string)
					status, _ := itemMap["status"].(string)
					if name != "" {
						steps = append(steps, model.HookPlanStep{Name: name, Status: status})
					}
				}
			}
		}
	}

	return steps
}

// 🔎️extractSearchFromInput extracts search information from input JSON or toolArgs string.
func ExtractSearchFromInput(input json.RawMessage, toolArgs string) ([]string, []string) {
	var data map[string]interface{}
	var source interface{}

	if input != nil {
		if err := json.Unmarshal(input, &data); err != nil {
			if toolArgs != "" {
				if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
					return nil, nil
				}
			} else {
				return nil, nil
			}
		}
	} else if toolArgs != "" {
		if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
			return nil, nil
		}
	} else {
		return nil, nil
	}

	if ti, ok := data["tool_input"]; ok {
		source = ti
	} else {
		source = data
	}

	sourceMap, ok := source.(map[string]interface{})
	if !ok {
		return nil, nil
	}

	var pages []string
	var ranges []string
	if url, ok := sourceMap["url"].(string); ok && url != "" {
		pages = append(pages, url)
	}
	if pagesList, ok := sourceMap["pages"].([]interface{}); ok {
		for _, page := range pagesList {
			if pageStr, ok := page.(string); ok && pageStr != "" {
				if strings.HasPrefix(pageStr, "http://") || strings.HasPrefix(pageStr, "https://") {
					pages = append(pages, pageStr)
				}
			}
		}
	}

	filePath := ""
	if fp, ok := sourceMap["file_path"].(string); ok {
		filePath = fp
	} else if fp, ok := sourceMap["filePath"].(string); ok {
		filePath = fp
	}
	if filePath != "" {
		startLine := 0
		endLine := 0
		if v, ok := sourceMap["startLine"].(float64); ok {
			startLine = int(v)
		}
		if v, ok := sourceMap["endLine"].(float64); ok {
			endLine = int(v)
		}
		limit := 0
		offset := 1
		if l, ok := sourceMap["limit"].(float64); ok {
			limit = int(l)
		}
		if o, ok := sourceMap["offset"].(float64); ok {
			offset = int(o)
		}
		if startLine > 0 {
			if endLine <= 0 {
				endLine = startLine
			}
			if startLine == endLine {
				ranges = append(ranges, fmt.Sprintf("%s#L%d", filePath, startLine))
			} else {
				ranges = append(ranges, fmt.Sprintf("%s#L%d-L%d", filePath, startLine, endLine))
			}
		} else if limit == 0 {
			if content, err := os.ReadFile(filePath); err == nil {
				trimmed := strings.TrimRight(string(content), "\n")
				lines := 1
				if trimmed != "" {
					lines = len(strings.Split(trimmed, "\n"))
				}
				ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", filePath, lines))
			}
		} else {
			endLine := offset + limit - 1
			ranges = append(ranges, fmt.Sprintf("%s#L%d-L%d", filePath, offset, endLine))
		}
	}

	if native, ok := data["native"].(map[string]interface{}); ok {
		if event, ok := native["event"].(map[string]interface{}); ok {
			if toolName, ok := event["tool_name"].(string); ok && toolName == "Read" {
				if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
					if fp, ok := toolInput["file_path"].(string); ok && fp != "" {
						if l, ok := toolInput["limit"].(float64); ok {
							ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", fp, int(l)))
						} else if content, err := os.ReadFile(fp); err == nil {
							trimmed := strings.TrimRight(string(content), "\n")
							lines := 1
							if trimmed != "" {
								lines = len(strings.Split(trimmed, "\n"))
							}
							ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", fp, lines))
						}
					}
				}
			}
		}
	}

	return pages, ranges
}

// 🟥️extractToolInputFromStdin extracts tool input from stdin JSON.
func ExtractToolInputFromStdin(input json.RawMessage) json.RawMessage {
	if input == nil {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	if native, ok := data["native"].(map[string]interface{}); ok {
		if event, ok := native["event"].(map[string]interface{}); ok {
			if toolInput, ok := event["tool_input"]; ok {
				if bytes, err := json.Marshal(toolInput); err == nil {
					return json.RawMessage(bytes)
				}
			}
		}
	}
	if toolInput, ok := data["tool_input"]; ok {
		if bytes, err := json.Marshal(toolInput); err == nil {
			return json.RawMessage(bytes)
		}
	}
	return nil
}

// 📩️extractToolResponseFromStdin extracts tool response from stdin JSON.
func ExtractToolResponseFromStdin(input json.RawMessage) json.RawMessage {
	if input == nil {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	if toolResponse, ok := data["tool_response"]; ok {
		if bytes, err := json.Marshal(toolResponse); err == nil {
			return json.RawMessage(bytes)
		}
	}
	if toolOutput, ok := data["tool_output"]; ok {
		if bytes, err := json.Marshal(toolOutput); err == nil {
			return json.RawMessage(bytes)
		}
	}
	return nil
}

func ResolveEventCheckpointID(event interface{}) string {
	s, _ := event.(string)
	return strings.TrimSpace(s)
}

func resolveRangeRef(ref string) (string, error) {
	parts := strings.SplitN(strings.TrimSpace(ref), "#L", 2)
	if len(parts) != 2 {
		return "", fmt.Errorf("invalid range ref %q", ref)
	}
	fileID := codebase.ResolvePathToFileID(parts[0])
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
		return fileID + model.EmojiText(model.EmojiLine) + strconv.Itoa(start) + model.EmojiText(model.EmojiLine) + strconv.Itoa(end), nil
	}
	line, err := strconv.Atoi(lineSpec)
	if err != nil {
		return "", fmt.Errorf("invalid line %q", lineSpec)
	}
	return fileID + model.EmojiText(model.EmojiLine) + strconv.Itoa(line), nil
}

func ExtractSearchCommandFilesAndPattern(command string, cwd string) ([]string, string) {
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

func SearchLinesInFile(filePath string, pattern string) []int {
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

// #endregion 📰️Todos

// #endregion 🚚️Split

// #region 🚪️Ports

// #region 🔁️Reexports

// ✅️Todo is the todo shape of 📐️model, reexported so a client needs one import.
type Todo = model.Todo

// 📍️Location is where a todo sits.
type Location = model.Location

// 🎨️Draft is a named set of files staged before a ticket exists.
type Draft = model.Draft

// 🆕️TodoCreateInput is the input a create takes.
type TodoCreateInput = model.TodoCreateInput

// ♻️TodoChangeInput is the input a change takes.
type TodoChangeInput = model.TodoChangeInput

// #endregion 🔁️Reexports

// #region ❗️Errors

// ❗️TodoError is everything the todos domain can refuse, as a class rather than a rendered string.
type TodoError struct {
	Kind    string
	Message string
}

// 🔤️Error renders the refusal.
func (err TodoError) Error() string { return err.Message }

// 🏷️Class returns the stable class of the refusal.
func (err TodoError) Class() string { return err.Kind }

// 🏷️ErrorClass returns the stable class of any refusal this domain produced.
func ErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var todoError TodoError
	if errors.As(err, &todoError) {
		return todoError.Kind
	}
	return "port"
}

// 🔍️errTodoNotFound refuses an identifier no todo carries.
func errTodoNotFound() error { return TodoError{Kind: "not-found", Message: "todo not found"} }

// 📍️errNoLocation refuses a todo nothing says the line of.
func errNoLocation() error { return TodoError{Kind: "no-location", Message: "todo has no location"} }

// 🚫️errInvalidParent refuses a parent that is neither a directory nor an existing file.
func errInvalidParent() error {
	return TodoError{Kind: "invalid-parent", Message: "invalid parent id (must be path to folder or file)"}
}

// 📛️errDraftExists refuses an identifier a draft already took.
func errDraftExists(id string) error {
	return TodoError{Kind: "draft-exists", Message: "draft already exists: " + id}
}

// 🔤️errInvalidTitle refuses a draft title that carries no slug.
func errInvalidTitle() error { return TodoError{Kind: "invalid-title", Message: "invalid draft title"} }

// 🗄️errTodoPort refuses when the tree or a port refused.
func errTodoPort(message string) error { return TodoError{Kind: "port", Message: message} }

// #endregion ❗️Errors

// #region 🔍️Parsing

// 📄️TodoMarkdownName is the file a directory records its own todos in.
const TodoMarkdownName = ".todos.md"

// 🏷️TodoMarkdownPrefix is the item prefix a todo markdown entry carries.
const TodoMarkdownPrefix = "- TODO "

// 🗂️TodoScanExtensions are the file extensions a scan reads todo comments out of.
var TodoScanExtensions = []string{".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json"}

// 💬️TodoCommentOpeners are the comment openers a todo comment may start with.
var TodoCommentOpeners = []string{"//", "#", "--"}

// 🗑️TodoSkippedDirectories are the directory names a scan never descends into.
var TodoSkippedDirectories = []string{"node_modules", "dist", "build"}

// ✂️SplitTodoCommentParts returns the untouched prefix, the name and the description of a todo
// comment line, so a rewrite can put a new name and description back without disturbing the
// indentation or the comment opener.
func SplitTodoCommentParts(line string) (string, string, string, bool) {
	rest := strings.TrimLeft(line, " \t\r\n\v\f")
	opener := ""
	for _, candidate := range TodoCommentOpeners {
		if strings.HasPrefix(rest, candidate) {
			opener = candidate
			break
		}
	}
	if opener == "" {
		return "", "", "", false
	}
	afterOpener := strings.TrimLeft(rest[len(opener):], " \t\r\n\v\f")
	if !strings.HasPrefix(afterOpener, "TODO") {
		return "", "", "", false
	}
	keyword := afterOpener[len("TODO"):]
	if keyword == "" {
		return "", "", "", false
	}
	first, size := utf8.DecodeRuneInString(keyword)
	if size == 0 || !unicode.IsSpace(first) {
		return "", "", "", false
	}
	afterKeyword := strings.TrimLeft(keyword, " \t\r\n\v\f")
	if afterKeyword == "" {
		return "", "", "", false
	}
	colon := strings.Index(afterKeyword, ":")
	if colon < 0 {
		return "", "", "", false
	}
	name := afterKeyword[:colon]
	description := afterKeyword[colon+1:]
	prefix := line[:len(line)-len(afterKeyword)]
	return prefix, strings.TrimSpace(name), strings.TrimSpace(description), true
}

// 📰️ParseTodoMarkdown parses a todo markdown document. Every item becomes a todo whose name is the
// text before the first colon and whose description is everything after it.
func ParseTodoMarkdown(content string, parentPath string) []*model.Todo {
	found := make([]*model.Todo, 0)
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if !strings.HasPrefix(trimmed, TodoMarkdownPrefix) {
			continue
		}
		rest := trimmed[len(TodoMarkdownPrefix):]
		name, description := rest, ""
		if colon := strings.Index(rest, ":"); colon >= 0 {
			name, description = rest[:colon], rest[colon+1:]
		}
		name = strings.TrimSpace(name)
		description = strings.TrimSpace(description)
		found = append(found, &model.Todo{
			ID:          identity.Slugify(name),
			Name:        name,
			Description: description,
			ParentID:    parentPath,
			Location:    &model.Location{FilePath: joinTodoPath(parentPath, TodoMarkdownName)},
		})
	}
	return found
}

// 💬️ParseTodoComments parses the todo comments of a source file. The line number is one based.
func ParseTodoComments(content string, filePath string) []*model.Todo {
	found := make([]*model.Todo, 0)
	for index, line := range strings.Split(content, "\n") {
		_, name, description, ok := SplitTodoCommentParts(line)
		if !ok {
			continue
		}
		found = append(found, &model.Todo{
			ID:          identity.Slugify(name),
			Name:        name,
			Description: description,
			ParentID:    filePath,
			Location:    &model.Location{FilePath: filePath, Line: index + 1, Column: 1},
		})
	}
	return found
}

// 💬️TodoCommentOpener returns the comment opener a todo written into a file gets, by extension.
func TodoCommentOpener(path string) string {
	switch todoExtensionOf(path) {
	case ".py", ".sh", ".yaml", ".yml":
		return "#"
	case ".sql", ".lua":
		return "--"
	}
	return "//"
}

// 🗂️todoExtensionOf returns the extension of the last segment of a path.
func todoExtensionOf(path string) string {
	name := path
	if index := strings.LastIndexAny(path, "/\\"); index >= 0 {
		name = path[index+1:]
	}
	if index := strings.LastIndex(name, "."); index > 0 {
		return name[index:]
	}
	return ""
}

// 🛤️joinTodoPath joins a parent path and a name with a forward slash.
func joinTodoPath(parent, name string) string {
	if parent == "" || parent == "." {
		return name
	}
	return strings.TrimRight(parent, "/") + "/" + name
}

// #endregion 🔍️Parsing

// #region 🌲️Tree

// 📍️TreeEntry is one entry of a scanned tree.
type TreeEntry struct {
	Path  string `json:"path"`
	IsDir bool   `json:"isDir"`
}

// 📄️TreeFile is one path and content pair of a tree.
type TreeFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 🌲️TodoTree is the tree a scan reads and a rewrite writes back to.
type TodoTree interface {
	// 🚶️Walk returns every entry in pre-order, a directory before everything it contains.
	Walk() []TreeEntry
	// 📖️Read returns the content of one file.
	Read(path string) (string, bool)
	// 💾️Write replaces or creates one file.
	Write(path string, content string) error
	// 📁️IsDir reports whether a path is a directory that exists.
	IsDir(path string) bool
	// 🔍️Exists reports whether a path exists at all.
	Exists(path string) bool
}

// ➕️appendToTree appends to a file of a tree, creating it when it does not exist.
func appendToTree(tree TodoTree, path, text string) error {
	existing, _ := tree.Read(path)
	return tree.Write(path, existing+text)
}

// 🧠️MemoryTodoTree is the in-memory tree: a set of directories and a map of file contents.
type MemoryTodoTree struct {
	directories []string
	files       map[string]string
}

// 🌱️NewMemoryTodoTree seeds a tree with files. Every ancestor directory of every file exists.
func NewMemoryTodoTree(files []TreeFile) *MemoryTodoTree {
	tree := &MemoryTodoTree{files: map[string]string{}}
	for _, file := range files {
		prefix := ""
		segments := strings.Split(file.Path, "/")
		for _, segment := range segments[:len(segments)-1] {
			if prefix == "" {
				prefix = segment
			} else {
				prefix = prefix + "/" + segment
			}
			if !containsTodoString(tree.directories, prefix) {
				tree.directories = append(tree.directories, prefix)
			}
		}
		tree.files[file.Path] = file.Content
	}
	return tree
}

// 📸️Snapshot returns every path and content pair, in path order.
func (tree *MemoryTodoTree) Snapshot() []TreeFile {
	paths := make([]string, 0, len(tree.files))
	for path := range tree.files {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	snapshot := make([]TreeFile, 0, len(paths))
	for _, path := range paths {
		snapshot = append(snapshot, TreeFile{Path: path, Content: tree.files[path]})
	}
	return snapshot
}

// 🚶️Walk returns the root, every directory and every file, in path order.
func (tree *MemoryTodoTree) Walk() []TreeEntry {
	entries := []TreeEntry{{Path: "", IsDir: true}}
	for _, path := range tree.directories {
		entries = append(entries, TreeEntry{Path: path, IsDir: true})
	}
	for path := range tree.files {
		entries = append(entries, TreeEntry{Path: path, IsDir: false})
	}
	sort.SliceStable(entries, func(left, right int) bool {
		if entries[left].Path != entries[right].Path {
			return entries[left].Path < entries[right].Path
		}
		return entries[left].IsDir && !entries[right].IsDir
	})
	return entries
}

// 📖️Read returns the content of one file.
func (tree *MemoryTodoTree) Read(path string) (string, bool) {
	content, found := tree.files[path]
	return content, found
}

// 💾️Write replaces or creates one file.
func (tree *MemoryTodoTree) Write(path string, content string) error {
	tree.files[path] = content
	return nil
}

// 📁️IsDir reports whether a path is a directory that exists.
func (tree *MemoryTodoTree) IsDir(path string) bool {
	return path == "" || containsTodoString(tree.directories, path)
}

// 🔍️Exists reports whether a path exists at all.
func (tree *MemoryTodoTree) Exists(path string) bool {
	if tree.IsDir(path) {
		return true
	}
	_, found := tree.files[path]
	return found
}

// 🔎️containsTodoString reports whether a slice carries a value.
func containsTodoString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 💽️FsTodoTree is the tree over a real checkout, applying the scan's own skip rules.
type FsTodoTree struct {
	root string
}

// 🆕️NewFsTodoTree builds the tree rooted at a directory.
func NewFsTodoTree(root string) *FsTodoTree { return &FsTodoTree{root: root} }

// 🛤️absolute resolves a tree relative path against the root.
func (tree *FsTodoTree) absolute(path string) string {
	segments := make([]string, 0, 4)
	segments = append(segments, tree.root)
	for _, segment := range strings.Split(path, "/") {
		if segment != "" {
			segments = append(segments, segment)
		}
	}
	return filepath.Join(segments...)
}

// 🚶️Walk returns the root and everything the scan rules let it descend into.
func (tree *FsTodoTree) Walk() []TreeEntry {
	found := []TreeEntry{{Path: "", IsDir: true}}
	tree.descend("", &found)
	return found
}

// 🚶️descend collects one directory level and recurses into the directories it keeps.
func (tree *FsTodoTree) descend(relative string, found *[]TreeEntry) {
	entries, err := os.ReadDir(tree.absolute(relative))
	if err != nil {
		return
	}
	names := make([]TreeEntry, 0, len(entries))
	for _, entry := range entries {
		names = append(names, TreeEntry{Path: entry.Name(), IsDir: entry.IsDir()})
	}
	sort.SliceStable(names, func(left, right int) bool {
		if names[left].Path != names[right].Path {
			return names[left].Path < names[right].Path
		}
		return !names[left].IsDir && names[right].IsDir
	})
	for _, entry := range names {
		path := joinTodoPath(relative, entry.Path)
		if !entry.IsDir {
			*found = append(*found, TreeEntry{Path: path, IsDir: false})
			continue
		}
		if containsTodoString(TodoSkippedDirectories, entry.Path) {
			continue
		}
		if strings.HasPrefix(entry.Path, ".") && entry.Path != workspace.SemioDirName {
			continue
		}
		*found = append(*found, TreeEntry{Path: path, IsDir: true})
		tree.descend(path, found)
	}
}

// 📖️Read returns the content of one file.
func (tree *FsTodoTree) Read(path string) (string, bool) {
	data, err := os.ReadFile(tree.absolute(path))
	if err != nil {
		return "", false
	}
	return string(data), true
}

// 💾️Write replaces or creates one file.
func (tree *FsTodoTree) Write(path string, content string) error {
	target := tree.absolute(path)
	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		return errTodoPort(err.Error())
	}
	if err := os.WriteFile(target, []byte(content), 0o644); err != nil {
		return errTodoPort(err.Error())
	}
	return nil
}

// 📁️IsDir reports whether a path is a directory that exists.
func (tree *FsTodoTree) IsDir(path string) bool {
	info, err := os.Stat(tree.absolute(path))
	return err == nil && info.IsDir()
}

// 🔍️Exists reports whether a path exists at all.
func (tree *FsTodoTree) Exists(path string) bool {
	_, err := os.Stat(tree.absolute(path))
	return err == nil
}

// 🛤️LocateTodosOnDisk rewrites the tree relative locations of scanned todos into host paths
// under a tree root, which is what a caller reading and rewriting real files needs.
func LocateTodosOnDisk(root string, found []*model.Todo) []*model.Todo {
	for _, todo := range found {
		todo.ParentID = filepath.Join(root, filepath.FromSlash(todo.ParentID))
		if todo.Location != nil {
			todo.Location.FilePath = filepath.Join(root, filepath.FromSlash(todo.Location.FilePath))
		}
	}
	return found
}

// 🎯️ScanTodos returns every todo a tree carries: each directory's own todo markdown items when
// the directory is entered, and each scanned file's todo comments when the file is reached.
func ScanTodos(tree TodoTree) []*model.Todo {
	found := make([]*model.Todo, 0)
	for _, entry := range tree.Walk() {
		if entry.IsDir {
			if content, ok := tree.Read(joinTodoPath(entry.Path, TodoMarkdownName)); ok {
				found = append(found, ParseTodoMarkdown(content, entry.Path)...)
			}
			continue
		}
		if strings.HasSuffix(entry.Path, TodoMarkdownName) {
			continue
		}
		if !containsTodoString(TodoScanExtensions, todoExtensionOf(entry.Path)) {
			continue
		}
		if content, ok := tree.Read(entry.Path); ok {
			found = append(found, ParseTodoComments(content, entry.Path)...)
		}
	}
	return found
}

// 🔎️SearchTodos returns every todo whose name or description contains the term.
func SearchTodos(tree TodoTree, term string) []*model.Todo {
	needle := strings.ToLower(term)
	found := make([]*model.Todo, 0)
	for _, todo := range ScanTodos(tree) {
		if needle == "" || strings.Contains(strings.ToLower(todo.Name), needle) || strings.Contains(strings.ToLower(todo.Description), needle) {
			found = append(found, todo)
		}
	}
	return found
}

// #endregion 🌲️Tree

// #region ✏️Rewrites

// ✏️ReplaceInMarkdown rewrites the item of a markdown document with a new name and description,
// leaving every other line untouched.
func ReplaceInMarkdown(document, oldName, newName, newDescription string) (string, error) {
	prefix := TodoMarkdownPrefix + oldName + ":"
	lines := strings.Split(document, "\n")
	for index, line := range lines {
		if strings.HasPrefix(strings.TrimLeft(line, " \t\r\n\v\f"), prefix) {
			lines[index] = TodoMarkdownPrefix + newName + ": " + newDescription
			return strings.Join(lines, "\n"), nil
		}
	}
	return "", errTodoNotFound()
}

// 🗑️RemoveFromMarkdown removes the item of a markdown document.
func RemoveFromMarkdown(document, name string) string {
	prefix := TodoMarkdownPrefix + name + ":"
	kept := make([]string, 0)
	for _, line := range strings.Split(document, "\n") {
		if strings.HasPrefix(strings.TrimLeft(line, " \t\r\n\v\f"), prefix) {
			continue
		}
		kept = append(kept, line)
	}
	return strings.Join(kept, "\n")
}

// ✏️ReplaceInFile rewrites the todo comment on a one-based line, keeping its indentation and
// comment opener.
func ReplaceInFile(document string, lineNumber int64, newName, newDescription string) (string, error) {
	lines := strings.Split(document, "\n")
	if lineNumber <= 0 || lineNumber > int64(len(lines)) {
		return "", errNoLocation()
	}
	index := int(lineNumber) - 1
	prefix, _, _, ok := SplitTodoCommentParts(lines[index])
	if !ok {
		return "", errTodoNotFound()
	}
	lines[index] = prefix + newName + ": " + newDescription
	return strings.Join(lines, "\n"), nil
}

// 🗑️RemoveFromFile removes the one-based line from a document.
func RemoveFromFile(document string, lineNumber int64) string {
	lines := strings.Split(document, "\n")
	if lineNumber > 0 && lineNumber <= int64(len(lines)) {
		index := int(lineNumber) - 1
		lines = append(lines[:index], lines[index+1:]...)
	}
	return strings.Join(lines, "\n")
}

// #endregion ✏️Rewrites

// #region 🎫️TicketOpener

// 🎫️TicketOpener turns a promoted todo into a ticket and returns the ticket identifier.
type TicketOpener interface {
	// 🆕️Open opens a ticket with a title and a prompt.
	Open(title, prompt string) (string, error)
}

// 🧠️RecordingTicketOpener hands out deterministic identifiers and keeps every request.
type RecordingTicketOpener struct {
	opened []string
}

// 🆕️NewRecordingTicketOpener builds an opener that has opened nothing.
func NewRecordingTicketOpener() *RecordingTicketOpener { return &RecordingTicketOpener{} }

// 📜️Opened returns every title and prompt opened so far, in order.
func (opener *RecordingTicketOpener) Opened() []string {
	return append([]string{}, opener.opened...)
}

// 🆕️Open records the request and hands out the next deterministic identifier.
func (opener *RecordingTicketOpener) Open(title, prompt string) (string, error) {
	opener.opened = append(opener.opened, title+"\t"+prompt)
	return fmt.Sprintf("ticket-%d", len(opener.opened)), nil
}

// #endregion 🎫️TicketOpener

// #region 🔓️Lifecycle

// 📝️Todos is the todo aggregate bound to its ports.
type Todos struct {
	tree    TodoTree
	emitter events.Emitter
	author  string
}

// 🆕️NewTodos binds the aggregate to a tree, an emitter and an author alias.
func NewTodos(tree TodoTree, emitter events.Emitter, author string) *Todos {
	return &Todos{tree: tree, emitter: emitter, author: author}
}

// 📋️List returns every todo the tree carries.
func (aggregate *Todos) List() []*model.Todo { return ScanTodos(aggregate.tree) }

// 🔍️Find returns the first todo carrying an identifier.
func (aggregate *Todos) Find(id string) (*model.Todo, error) {
	for _, todo := range aggregate.List() {
		if todo.ID == id {
			return todo, nil
		}
	}
	return nil, errTodoNotFound()
}

// 🆕️Create writes a todo: an item appended to the parent directory's own todo markdown, or a
// comment appended to the parent file in that file's comment style.
func (aggregate *Todos) Create(input model.TodoCreateInput) (*model.Todo, error) {
	var todo *model.Todo
	switch {
	case aggregate.tree.IsDir(input.ParentID):
		path := joinTodoPath(input.ParentID, TodoMarkdownName)
		if err := appendToTree(aggregate.tree, path, TodoMarkdownPrefix+input.Name+": "+input.Description+"\n"); err != nil {
			return nil, err
		}
		todo = &model.Todo{ID: identity.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
	case aggregate.tree.Exists(input.ParentID):
		opener := TodoCommentOpener(input.ParentID)
		if err := appendToTree(aggregate.tree, input.ParentID, "\n"+opener+" TODO "+input.Name+": "+input.Description+"\n"); err != nil {
			return nil, err
		}
		todo = &model.Todo{
			ID: identity.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID,
			Location: &model.Location{FilePath: input.ParentID},
		}
	default:
		return nil, errInvalidParent()
	}
	aggregate.emit(events.EventTodoCreateEnded, events.TodoPayload{ID: todo.ID, ParentID: todo.ParentID, Name: todo.Name, Author: aggregate.author})
	return todo, nil
}

// ♻️Change rewrites the source line a todo is, in place.
func (aggregate *Todos) Change(input model.TodoChangeInput) (*model.Todo, error) {
	todo, err := aggregate.Find(input.ID)
	if err != nil {
		return nil, err
	}
	if todo.Location == nil {
		return nil, errNoLocation()
	}
	location := *todo.Location
	name, description := todo.Name, todo.Description
	if input.Name != nil {
		name = *input.Name
	}
	if input.Description != nil {
		description = *input.Description
	}
	document, ok := aggregate.tree.Read(location.FilePath)
	if !ok {
		return nil, errTodoNotFound()
	}
	var rewritten string
	switch {
	case strings.HasSuffix(location.FilePath, TodoMarkdownName):
		rewritten, err = ReplaceInMarkdown(document, todo.Name, name, description)
	case location.Line > 0:
		rewritten, err = ReplaceInFile(document, int64(location.Line), name, description)
	default:
		return nil, errNoLocation()
	}
	if err != nil {
		return nil, err
	}
	if err := aggregate.tree.Write(location.FilePath, rewritten); err != nil {
		return nil, err
	}
	updated := &model.Todo{ID: identity.Slugify(name), Name: name, Description: description, ParentID: todo.ParentID, Location: &location}
	aggregate.emit(events.EventTodoChangeEnded, events.TodoChangePayload{
		TodoPayload: events.TodoPayload{ID: updated.ID, ParentID: todo.ParentID, Author: aggregate.author},
		Name:        input.Name,
		Description: input.Description,
	})
	return updated, nil
}

// 🗑️Delete removes the source line a todo is.
func (aggregate *Todos) Delete(id string) (bool, error) {
	todo, err := aggregate.Find(id)
	if err != nil {
		return false, err
	}
	if todo.Location == nil {
		return false, errNoLocation()
	}
	document, ok := aggregate.tree.Read(todo.Location.FilePath)
	if !ok {
		return false, errTodoNotFound()
	}
	var rewritten string
	switch {
	case strings.HasSuffix(todo.Location.FilePath, TodoMarkdownName):
		rewritten = RemoveFromMarkdown(document, todo.Name)
	case todo.Location.Line > 0:
		rewritten = RemoveFromFile(document, int64(todo.Location.Line))
	default:
		return false, errNoLocation()
	}
	if err := aggregate.tree.Write(todo.Location.FilePath, rewritten); err != nil {
		return false, err
	}
	aggregate.emit(events.EventTodoDeleteEnded, events.TodoPayload{ID: todo.ID, ParentID: todo.ParentID, Name: todo.Name, Author: aggregate.author})
	return true, nil
}

// 🎫️ToTicket promotes a todo into a ticket and removes the todo once the ticket exists.
func (aggregate *Todos) ToTicket(id string, opener TicketOpener, title, prompt string) (string, error) {
	todo, err := aggregate.Find(id)
	if err != nil {
		return "", err
	}
	ticket, err := opener.Open(title, todo.Description+"\n\n"+prompt)
	if err != nil {
		return "", err
	}
	if _, err := aggregate.Delete(id); err != nil {
		return "", err
	}
	return ticket, nil
}

// 📤️emit sends one envelope to the bound emitter.
func (aggregate *Todos) emit(kind events.EventKind, payload interface{}) {
	if aggregate.emitter == nil {
		return
	}
	aggregate.emitter.Emit(kind, "repo-cli", payload)
}

// #endregion 🔓️Lifecycle

// #region 🎨️DraftStore

// 🪪️DraftID returns the artifact identifier of a draft.
func DraftID(draft *model.Draft) string {
	return identity.EmojiText(identity.Entity("draft")) + workspace.Flat(draft.ID)
}

// 🔗️DraftURI returns the artifact URI of a draft.
func DraftURI(draft *model.Draft) string { return "repo://draft/" + DraftID(draft) }

// 🗄️DraftStore is where drafts live: one directory of copied files per draft identifier.
type DraftStore interface {
	// 📋️IDs returns every draft identifier, in ascending order.
	IDs() []string
	// 🔍️Exists reports whether a draft exists.
	Exists(id string) bool
	// 🆕️Create creates a draft holding these files.
	Create(id string, files []TreeFile) error
	// 📋️Files returns the files of one draft, in file name order.
	Files(id string) []TreeFile
	// 🗑️Delete removes a draft, which is a no-operation when it does not exist.
	Delete(id string) error
}

// 🧠️MemoryDraftStore is the in-memory draft store.
type MemoryDraftStore struct {
	drafts map[string][]TreeFile
}

// 🆕️NewMemoryDraftStore builds an empty store.
func NewMemoryDraftStore() *MemoryDraftStore {
	return &MemoryDraftStore{drafts: map[string][]TreeFile{}}
}

// 📋️IDs returns every draft identifier, in ascending order.
func (store *MemoryDraftStore) IDs() []string {
	ids := make([]string, 0, len(store.drafts))
	for id := range store.drafts {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// 🔍️Exists reports whether a draft exists.
func (store *MemoryDraftStore) Exists(id string) bool {
	_, found := store.drafts[id]
	return found
}

// 🆕️Create creates a draft holding these files.
func (store *MemoryDraftStore) Create(id string, files []TreeFile) error {
	stored := append([]TreeFile{}, files...)
	sort.SliceStable(stored, func(left, right int) bool { return stored[left].Path < stored[right].Path })
	store.drafts[id] = stored
	return nil
}

// 📋️Files returns the files of one draft, in file name order.
func (store *MemoryDraftStore) Files(id string) []TreeFile {
	return append([]TreeFile{}, store.drafts[id]...)
}

// 🗑️Delete removes a draft.
func (store *MemoryDraftStore) Delete(id string) error {
	delete(store.drafts, id)
	return nil
}

// 💽️FsDraftStore is the store over a real notes tree of a repository root.
type FsDraftStore struct {
	root string
}

// 🆕️NewFsDraftStore builds the store of the notes directory of a repository root.
func NewFsDraftStore(root string) *FsDraftStore { return &FsDraftStore{root: root} }

// 📋️IDs returns every draft identifier, in ascending order.
func (store *FsDraftStore) IDs() []string {
	entries, err := os.ReadDir(store.root)
	if err != nil {
		return nil
	}
	ids := make([]string, 0, len(entries))
	for _, entry := range entries {
		if entry.IsDir() {
			ids = append(ids, entry.Name())
		}
	}
	sort.Strings(ids)
	return ids
}

// 🔍️Exists reports whether a draft directory exists.
func (store *FsDraftStore) Exists(id string) bool {
	info, err := os.Stat(filepath.Join(store.root, id))
	return err == nil && info.IsDir()
}

// 🆕️Create writes a draft directory holding these files.
func (store *FsDraftStore) Create(id string, files []TreeFile) error {
	directory := filepath.Join(store.root, id)
	if err := os.MkdirAll(directory, 0o755); err != nil {
		return errTodoPort(err.Error())
	}
	for _, file := range files {
		if err := os.WriteFile(filepath.Join(directory, file.Path), []byte(file.Content), 0o644); err != nil {
			return errTodoPort(err.Error())
		}
	}
	return nil
}

// 📋️Files reads the files of one draft, in file name order.
func (store *FsDraftStore) Files(id string) []TreeFile {
	entries, err := os.ReadDir(filepath.Join(store.root, id))
	if err != nil {
		return nil
	}
	files := make([]TreeFile, 0, len(entries))
	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		data, err := os.ReadFile(filepath.Join(store.root, id, entry.Name()))
		if err != nil {
			continue
		}
		files = append(files, TreeFile{Path: entry.Name(), Content: string(data)})
	}
	sort.SliceStable(files, func(left, right int) bool { return files[left].Path < files[right].Path })
	return files
}

// 🗑️Delete removes a draft directory.
func (store *FsDraftStore) Delete(id string) error {
	if !store.Exists(id) {
		return nil
	}
	if err := os.RemoveAll(filepath.Join(store.root, id)); err != nil {
		return errTodoPort(err.Error())
	}
	return nil
}

// 📥️LoadTreeFiles reads host files into tree files carrying their base name and their content.
func LoadTreeFiles(paths []string) ([]TreeFile, error) {
	files := make([]TreeFile, 0, len(paths))
	for _, path := range paths {
		content, err := os.ReadFile(path)
		if err != nil {
			return nil, errTodoPort(err.Error())
		}
		files = append(files, TreeFile{Path: filepath.Base(path), Content: string(content)})
	}
	return files, nil
}

// 📋️ListDrafts returns every draft the store carries.
func ListDrafts(store DraftStore) []*model.Draft {
	ids := store.IDs()
	drafts := make([]*model.Draft, 0, len(ids))
	for _, id := range ids {
		drafts = append(drafts, &model.Draft{ID: id})
	}
	return drafts
}

// 📝️CreateDraft creates a draft from a title and a set of files. The identifier is the title's
// slug, a title carrying no slug is refused, an existing identifier is refused, and each file is
// copied under its base name.
func CreateDraft(store DraftStore, title string, files []TreeFile) (*model.Draft, error) {
	id := identity.Slugify(title)
	if id == "" {
		return nil, errInvalidTitle()
	}
	if store.Exists(id) {
		return nil, errDraftExists(id)
	}
	copied := make([]TreeFile, 0, len(files))
	for _, file := range files {
		name := file.Path
		if index := strings.LastIndexAny(name, "/\\"); index >= 0 {
			name = name[index+1:]
		}
		copied = append(copied, TreeFile{Path: name, Content: file.Content})
	}
	if err := store.Create(id, copied); err != nil {
		return nil, err
	}
	return &model.Draft{ID: id}, nil
}

// 🗑️DeleteDraft removes a draft. Removing one that does not exist is not an error.
func DeleteDraft(store DraftStore, id string) error { return store.Delete(id) }

// #endregion 🎨️DraftStore

// #region 🧾️Recording

// 🧠️MemoryEmitter keeps kind, source and payload lines instead of reaching a coordinator.
type MemoryEmitter struct {
	envelopes []string
}

// 🆕️NewMemoryEmitter builds an emitter with an empty log.
func NewMemoryEmitter() *MemoryEmitter { return &MemoryEmitter{} }

// 📤️Emit records one envelope with its payload rendered as canonical JSON.
func (emitter *MemoryEmitter) Emit(kind events.EventKind, source string, payload interface{}) {
	emitter.envelopes = append(emitter.envelopes, fmt.Sprintf("%s\t%s\t%s", kind, source, canonicalPayload(payload)))
}

// 📜️Envelopes returns every envelope emitted so far, in order.
func (emitter *MemoryEmitter) Envelopes() []string {
	return append([]string{}, emitter.envelopes...)
}

// 🧮️canonicalPayload renders a payload as compact JSON with object members in ascending key order
// and no HTML escaping, which is the one rendering both implementations agree on.
func canonicalPayload(payload interface{}) string {
	data, err := json.Marshal(payload)
	if err != nil {
		return ""
	}
	var generic interface{}
	if err := json.Unmarshal(data, &generic); err != nil {
		return ""
	}
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(generic); err != nil {
		return ""
	}
	return strings.TrimRight(buffer.String(), "\n")
}

// #endregion 🧾️Recording

// #endregion 🚪️Ports
