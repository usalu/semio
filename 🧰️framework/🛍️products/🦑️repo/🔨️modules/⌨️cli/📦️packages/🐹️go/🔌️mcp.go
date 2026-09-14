// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔌️mcp carries the MCP handler surface of the cli: argument and path guards, tool and prompt handlers, resource handlers, descriptions, the server factory and the server adapter.

// #endregion 🧲️Header

package cli

import (
	bufio "bufio"
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	math "math"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	template "text/template"
	time "time"

	codebasepkg "github.com/usalu/semio/repo/codebase"
	goalspkg "github.com/usalu/semio/repo/goals"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	hooks "github.com/usalu/semio/repo/hooks"
	languagespkg "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔌️Mcp Surface

// #region 🦀️Mcp

// 💿️runMcpServer is kept for command wiring compatibility.
func runMcpServer(cmd *Command, args []string) error {
	kind := providers.McpClientGeneric
	if len(args) > 0 {
		parsed, err := providers.ParseMcpClientKind(args[0])
		if err != nil {
			return err
		}
		kind = parsed
	}
	return serveMcp(cmd.Context(), kind, DefaultCommandTimeout)
}

// 📝️textResult holds the data fields for a textResult record.
func textResult(text string) *CallToolResult {
	return NewToolResultText(text)
}

// 🔷️toolResultToMCP holds the data fields for a toolResultToMCP record.
func toolResultToMCP(result workspace.ToolResult) (*CallToolResult, error) {
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return NewToolResultText(strings.Join(lines, "\n")), nil
}

// #endregion 🦀️Mcp

// #region 🎼️Args
// ⌨️Argument parsing utilities for CLI and MCP commands.
func getArgs(request CallToolRequest) map[string]interface{} {
	if args, ok := request.Params.Arguments.(map[string]interface{}); ok {
		return args
	}
	return make(map[string]interface{})
}

func getStringArg(args map[string]interface{}, key string) (string, bool, error) {
	value, ok := args[key]
	if !ok {
		return "", false, nil
	}
	str, ok := value.(string)
	if !ok || str == "" {
		return "", true, fmt.Errorf("invalid %s", key)
	}
	return str, true, nil
}

// 💿️requireStringArg holds the data fields for a requireStringArg record.
func requireStringArg(args map[string]interface{}, key string) (string, error) {
	value, ok, err := getStringArg(args, key)
	if err != nil {
		return "", err
	}
	if !ok {
		return "", fmt.Errorf("missing %s", key)
	}
	return value, nil
}

// 🔷️getIntArg holds the data fields for a getIntArg record.
func getIntArg(args map[string]interface{}, key string) (int, bool, error) {
	value, ok := args[key]
	if !ok {
		return 0, false, nil
	}
	number, ok := value.(float64)
	if !ok || number != math.Trunc(number) {
		return 0, true, fmt.Errorf("invalid %s", key)
	}
	return int(number), true, nil
}

// 🔶️requireIntArg holds the data fields for a requireIntArg record.
func requireIntArg(args map[string]interface{}, key string) (int, error) {
	value, ok, err := getIntArg(args, key)
	if err != nil {
		return 0, err
	}
	if !ok {
		return 0, fmt.Errorf("missing %s", key)
	}
	return value, nil
}

// 🔤️getStringSliceArg holds the data fields for a getStringSliceArg record.
func getStringSliceArg(args map[string]interface{}, key string) ([]string, bool, error) {
	value, ok := args[key]
	if !ok {
		return nil, false, nil
	}
	list, ok := value.([]interface{})
	if !ok || len(list) == 0 {
		return nil, true, fmt.Errorf("invalid %s", key)
	}
	result := make([]string, 0, len(list))
	for _, item := range list {
		str, ok := item.(string)
		if !ok || str == "" {
			return nil, true, fmt.Errorf("invalid %s", key)
		}
		result = append(result, str)
	}
	return result, true, nil
}

// 🔘️getBoolArg holds the data fields for a getBoolArg record.
func getBoolArg(args map[string]interface{}, key string) (bool, bool, error) {
	value, ok := args[key]
	if !ok {
		return false, false, nil
	}
	boolVal, ok := value.(bool)
	if !ok {
		return false, true, fmt.Errorf("invalid %s", key)
	}
	return boolVal, true, nil
}

// #endregion 🎼️Args

// #region 🔧️Paths
// 🛠️Path resolution utilities for file and folder operations.
func requireFilePath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid file path: %s", path)
	}
	if info.IsDir() {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

// 📁️requireFolderPath holds the data fields for a requireFolderPath record.
func requireFolderPath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	if !info.IsDir() {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// 💿️requireFileTargetPath holds the data fields for a requireFileTargetPath record.
func requireFileTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if info.IsDir() {
			return fmt.Errorf("invalid file path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

// 🛤️requireFolderTargetPath holds the data fields for a requireFolderTargetPath record.
func requireFolderTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if !info.IsDir() {
			return fmt.Errorf("invalid folder path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// #endregion 🔧️Paths

// #region 🪄️Handlers
// Request handler functions for CLI and MCP operations.
// ⏹️renderPromptTemplate holds the data fields for a renderPromptTemplate record.
func renderPromptTemplate(name string, data map[string]string) (string, error) {
	path := workspace.GetRepoMetaPath(filepath.Join("💬️prompts", name+".tpl"))
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	tmpl, err := template.New(name).Funcs(model.TxtFuncMap()).Parse(string(content))
	if err != nil {
		return "", err
	}
	var out strings.Builder
	if err := tmpl.Execute(&out, data); err != nil {
		return "", err
	}
	return out.String(), nil
}

// 💿️handleEnhancePrompt holds the data fields for a handleEnhancePrompt record.
func handleEnhancePrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("enhance", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Enhance the implementation by adding more features and enhance the existing tests to cover the new features.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

func handleRefactorPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("refactor", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Refactor the implementation and dont stop until all tests pass.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

// 🧪️handleTestPrompt holds the data fields for a handleTestPrompt record.
func handleTestPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("test", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Extend the current tests by testing more features.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

// 🔷️handleComplyPrompt holds the data fields for a handleComplyPrompt record.
func handleComplyPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("comply", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Get the implementation to comply the a set of tests. Dont remove any functionality from the tests.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

func analyze(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "compose"
	}
	result := ToolAnalyze(scope, nil)
	return toolResultToMCP(result)
}

// 📜️policyCheck holds the data fields for a policyCheck record.
func policyCheck(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "compose"
	}
	result := ToolPolicyCheck(id, scope)
	return toolResultToMCP(result)
}

// 🎫️ticketOpenWithKind handles MCP ticket_open for a specific IDE MCP surface.
func ticketOpenWithKind(ctx context.Context, request CallToolRequest, kind providers.McpClientKind) (*CallToolResult, error) {
	args := getArgs(request)
	emoji, _, _ := getStringArg(args, "emoji")
	title, _, _ := getStringArg(args, "title")
	prompt, _, _ := getStringArg(args, "prompt")
	goal, _, _ := getStringArg(args, "goal")
	client, _, _ := getStringArg(args, "client")
	llm, _, _ := getStringArg(args, "llm")
	effort, _, _ := getStringArg(args, "effort")
	draft, _, _ := getStringArg(args, "draft")
	parent, _, _ := getStringArg(args, "parent")
	issue, _, _ := getStringArg(args, "issue")
	planID, _, _ := getStringArg(args, "plan_id")
	specID, _, _ := getStringArg(args, "spec_id")
	client = providers.ResolveMcpTicketClient(kind, client)

	result := ToolTicketOpen(emoji, title, prompt, llm, effort, client, draft, false, goal, parent, false, issue, kind, planID, specID)
	return toolResultToMCP(result)
}

func ticketRead(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}

	result := ToolTicketRead(year, month, day, slug)
	return toolResultToMCP(result)
}

// 🔤️parseTicketPath parses a ticket path string (e.g. "26/03/27/SLUG") into year, month, day, slug.
// 🗺️Normalizes 4-digit years to 2-digit (2026 → 26).
func parseTicketPath(path string) (int, int, int, string, error) {
	parts := strings.Split(strings.TrimSpace(path), "/")
	if len(parts) < 4 {
		return 0, 0, 0, "", fmt.Errorf("invalid ticket path %q: expected format 🎆️YY/🌙️MM/☀️DD/SLUG", path)
	}
	yearPart := strings.TrimPrefix(parts[0], model.EmojiYear)
	monthPart := strings.TrimPrefix(parts[1], model.EmojiMonth)
	dayPart := strings.TrimPrefix(parts[2], model.EmojiDay)
	year, err := strconv.Atoi(yearPart)
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid year in ticket path %q", path)
	}
	if year >= 2000 {
		year = year % 100
	}
	month, err := strconv.Atoi(monthPart)
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid month in ticket path %q", path)
	}
	day, err := strconv.Atoi(dayPart)
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid day in ticket path %q", path)
	}
	slug := strings.Join(parts[3:], "/")
	if slug == "" {
		return 0, 0, 0, "", fmt.Errorf("missing slug in ticket path %q", path)
	}
	return year, month, day, slug, nil
}

// 📪️resolveTicketForClose resolves a ticket for closing: by path or latest open ticket.
func resolveTicketForClose(path string) (int, int, int, string, error) {
	if path != "" {
		return parseTicketPath(path)
	}
	ticket, err := todos.LatestOpenTicket()
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and failed to find latest open ticket: %w", err)
	}
	if ticket == nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and no open tickets found")
	}
	return ticket.Year, ticket.Month, ticket.Day, ticket.Slug, nil
}

// 🔓️resolveTicketForReopen resolves a ticket for reopening: by path or latest closed ticket.
func resolveTicketForReopen(path string) (int, int, int, string, error) {
	if path != "" {
		return parseTicketPath(path)
	}
	ticket, err := ticketspkg.LatestTicket()
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and failed to find latest ticket: %w", err)
	}
	if ticket == nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and no tickets found")
	}
	return ticket.Year, ticket.Month, ticket.Day, ticket.Slug, nil
}

// 🔶️ticketClose holds the data fields for a ticketClose record.
func ticketClose(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, _, _ := getStringArg(args, "path")
	summary, _, _ := getStringArg(args, "summary")
	filesSlice, _, _ := getStringSliceArg(args, "files")
	title, _, _ := getStringArg(args, "title")
	noManagement, _, _ := getBoolArg(args, "no_management")

	year, month, day, slug, err := resolveTicketForClose(path)
	if err != nil {
		return nil, err
	}

	result := ToolTicketClose(year, month, day, slug, summary, filesSlice, title, noManagement)
	return toolResultToMCP(result)
}

// 📬️ticketReopenWithKind handles MCP ticket_reopen for a specific IDE MCP surface.
func ticketReopenWithKind(ctx context.Context, request CallToolRequest, kind providers.McpClientKind) (*CallToolResult, error) {
	args := getArgs(request)
	path, _, _ := getStringArg(args, "path")
	prompt, _, _ := getStringArg(args, "prompt")
	client, _, _ := getStringArg(args, "client")
	llm, _, _ := getStringArg(args, "llm")
	effort, _, _ := getStringArg(args, "effort")
	title, _, _ := getStringArg(args, "title")
	draft, _, _ := getStringArg(args, "draft")
	noManagement, _, _ := getBoolArg(args, "no_management")
	planID, _, _ := getStringArg(args, "plan_id")
	specID, _, _ := getStringArg(args, "spec_id")
	client = providers.ResolveMcpTicketClient(kind, client)

	year, month, day, slug, err := resolveTicketForReopen(path)
	if err != nil {
		return nil, err
	}

	result := ToolTicketReopen(year, month, day, slug, prompt, llm, effort, client, draft, title, "", "", noManagement, kind, planID, specID)
	return toolResultToMCP(result)
}

// 🗑️draftDelete holds the data fields for a draftDelete record.
func draftDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	result := ToolDraftDelete(slug)
	return toolResultToMCP(result)
}

func todoCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	parent, _, _ := getStringArg(args, "parent")
	name, _, _ := getStringArg(args, "name")
	description, _, _ := getStringArg(args, "description")

	variables := map[string]interface{}{
		"input": map[string]interface{}{
			"parentId":    parent,
			"name":        name,
			"description": description,
		},
	}
	query := `mutation TodoCreate($input: TodoCreateInput!) { todoCreate(input: $input) { id name description } }`
	payload, err := graphqlpkg.Gql(query, variables)
	if err != nil {
		return nil, err
	}
	return textResult(payload), nil
}

func todoDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	id := path
	if strings.HasPrefix(path, "repo://") {
		id = model.UriToId(path)
	}
	query := `mutation TodoDelete($id: ID!) { todoDelete(id: $id) }`
	payload, err := graphqlpkg.Gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	return textResult(payload), nil
}

// ⛳️goalOpen holds the data fields for a goalOpen record.
func goalOpen(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	title, _, _ := getStringArg(args, "title")
	description, _, _ := getStringArg(args, "description")
	prompt, _, _ := getStringArg(args, "prompt")
	dueDate, _, _ := getStringArg(args, "due_date")
	llm, _, _ := getStringArg(args, "llm")
	client, _, _ := getStringArg(args, "client")
	noManagement, _, _ := getBoolArg(args, "no_github")
	parent, _, _ := getStringArg(args, "parent")
	milestone, _, _ := getStringArg(args, "milestone")

	result := ToolGoalCreate(title, description, prompt, dueDate, llm, client, noManagement, parent, milestone)
	return toolResultToMCP(result)
}

// 🔹️goalClose holds the data fields for a goalClose record.
func goalClose(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	summary, err := requireStringArg(args, "summary")
	if err != nil {
		return nil, err
	}
	noManagement, _, _ := getBoolArg(args, "no_github")

	result := ToolGoalClose(id, summary, noManagement)
	return toolResultToMCP(result)
}

func goalReopen(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	llm, err := requireStringArg(args, "llm")
	if err != nil {
		return nil, err
	}
	client, err := requireStringArg(args, "client")
	if err != nil {
		return nil, err
	}
	title, _, _ := getStringArg(args, "title")
	description, _, _ := getStringArg(args, "description")
	dueDate, _, _ := getStringArg(args, "due_date")
	noManagement, _, _ := getBoolArg(args, "no_github")

	result := ToolGoalReopen(id, prompt, llm, client, title, description, dueDate, noManagement)
	return toolResultToMCP(result)
}

// 📤️export holds the data fields for a export record.
func export(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	output, _, _ := getStringArg(args, "output")
	result := ToolExport(output)
	return toolResultToMCP(result)
}

// 🤝️contributorAdd holds the data fields for a contributorAdd record.
func contributorAdd(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	result := ToolContributorAdd(github)
	return toolResultToMCP(result)
}

// 🚚️contributorRemove holds the data fields for a contributorRemove record.
func contributorRemove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	result := ToolContributorRemove(github)
	return toolResultToMCP(result)
}

// 📁️folderCreate holds the data fields for a folderCreate record.
func folderCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderCreate(path)
	return toolResultToMCP(result)
}

// 🔸️folderMove holds the data fields for a folderMove record.
func folderMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderMove(source, target)
	return toolResultToMCP(result)
}

// 🔺️folderDelete holds the data fields for a folderDelete record.
func folderDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderDelete(path)
	return toolResultToMCP(result)
}

// 🆕️fileCreate holds the data fields for a fileCreate record.
func fileCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileCreate(path)
	return toolResultToMCP(result)
}

// 📄️fileMove holds the data fields for a fileMove record.
func fileMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileMove(source, target)
	return toolResultToMCP(result)
}

// 🔻️fileDelete holds the data fields for a fileDelete record.
func fileDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileDelete(path)
	return toolResultToMCP(result)
}

// 📑️sectionCreate holds the data fields for a sectionCreate record.
func sectionCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionCreate(file, section)
	return toolResultToMCP(result)
}

// ⬛️sectionMove holds the data fields for a sectionMove record.
func sectionMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	oldName, err := requireStringArg(args, "old_name")
	if err != nil {
		return nil, err
	}
	newName, err := requireStringArg(args, "new_name")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionMove(file, oldName, newName)
	return toolResultToMCP(result)
}

// ⬜️sectionDelete holds the data fields for a sectionDelete record.
func sectionDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionDelete(file, section)
	return toolResultToMCP(result)
}

func sectionIntegrate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetSection, err := requireStringArg(args, "target_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	targetParentSection, _, err := getStringArg(args, "target_parent_section")
	if err != nil {
		return nil, err
	}
	result := move.ToolIntegrate(source, targetSection, targetFile, targetParentSection)
	return toolResultToMCP(result)
}

// 🧲️sectionExtract holds the data fields for a sectionExtract record.
func sectionExtract(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	sourceFile, err := requireStringArg(args, "source_file")
	if err != nil {
		return nil, err
	}
	sourceSection, err := requireStringArg(args, "source_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	result := move.ToolExtract(sourceFile, sourceSection, targetFile)
	return toolResultToMCP(result)
}

// 🏺️artifactMove holds the data fields for a artifactMove record.
func artifactMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	sourceID, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetID, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}

	source := model.ParseArtifactRef(sourceID)
	target := model.ParseArtifactRef(targetID)

	var result workspace.ToolResult

	switch {
	case source.Kind == "file" && target.Kind == "file":
		result = move.ToolFileMove(source.Path, target.Path)
	case source.Kind == "folder" && target.Kind == "folder":
		result = move.ToolFolderMove(source.Path, target.Path)
	case source.Kind == "section" && target.Kind == "section" && source.Path == target.Path:
		if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 {
			return nil, fmt.Errorf("missing section path")
		}
		oldName := codebasepkg.ResolveSectionName(source.Path, source.SectionParts[len(source.SectionParts)-1])
		newName := codebasepkg.ResolveSectionName(target.Path, target.SectionParts[len(target.SectionParts)-1])
		result = move.ToolSectionMove(source.Path, oldName, newName)
	case source.Kind == "file" && target.Kind == "section":
		targetFile := target.Path
		var sectionName, parentSection string
		if len(target.SectionParts) > 0 {
			sectionName = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
		}
		if len(target.SectionParts) > 1 {
			parentSection = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
		}
		result = move.ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
		if result.Error == "" {
			absSource := filepath.Join(workspace.RootDir, source.Path)
			os.Remove(absSource)
			ticketspkg.RemoveAgentsDocsEntry(source.Path)
		}
	case source.Kind == "section" && target.Kind == "file":
		if len(source.SectionParts) == 0 {
			return nil, fmt.Errorf("missing section path")
		}
		sourceFile := source.Path
		sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
		result = move.ToolExtract(sourceFile, sectionName, target.Path)
	default:
		return nil, fmt.Errorf("unsupported move: %s → %s", source.Kind, target.Kind)
	}

	return toolResultToMCP(result)
}

func graphqlQuery(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	query, err := requireStringArg(args, "query")
	if err != nil {
		return nil, err
	}
	variablesStr, _, err := getStringArg(args, "variables")
	if err != nil {
		return nil, err
	}
	var variables map[string]interface{}
	if variablesStr != "" {
		if err := json.Unmarshal([]byte(variablesStr), &variables); err != nil {
			return nil, fmt.Errorf("invalid variables JSON: %w", err)
		}
	}
	result, gqlErr := graphqlpkg.Gql(query, variables)
	if gqlErr != nil {
		return textResult(fmt.Sprintf(`{"error": %q}`, gqlErr.Error())), nil
	}
	return textResult(result), nil
}

// 🟥️navigateTool holds the data fields for a navigateTool record.
func navigateTool(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	var uri string
	if strings.HasPrefix(target, "repo://") {
		uri = target
	} else {
		uri = model.IdToUri(target)
		if uri == "" {
			return nil, fmt.Errorf("could not resolve target: %s", target)
		}
	}
	id := model.UriToId(uri)
	result := map[string]string{"uri": uri, "id": id}
	bytes, _ := json.Marshal(result)
	return textResult(string(bytes)), nil
}

// #endregion 🪄️Handlers

// #region 🏬️Mcp Resources Handlers
// MCP resource handler functions for resource listing and reading.
// 📩️handleRepoResource holds the data fields for a handleRepoResource record.
func handleRepoResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Repo { repo { id name bundles { id } tickets { id } policies { id } contributors { id } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 💿️handleBundlesResource holds the data fields for a handleBundlesResource record.
func handleBundlesResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Bundles { repo { bundles { id name root sourceRoot projectType tags kind } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📦️handleBundleResource holds the data fields for a handleBundleResource record.
func handleBundleResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid bundle URI: %s", request.Params.URI)
	}
	// Resolve bundle name from ID by walking bundles
	bundles := codebasepkg.LoadBundles()
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	var bundleName string
	for _, b := range bundles {
		if norm(b.GetID()) == norm(id) {
			bundleName = b.Name
			break
		}
	}
	if bundleName == "" {
		bundleName = id
	}
	query := `query Bundle($id: String!) { bundle(name: $id) { id name root sourceRoot projectType tags kind } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"id": bundleName})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFoldersResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Folders { repo { folders { id path name kind } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFolderResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	path := model.IdToPath(id)
	query := `query Folder($path: String!) { folder(path: $path) { id path name kind parent { path } children { path name kind } files { path name kind } breachs { id } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFilesResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Files { repo { files { id path name kind extension } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📄️handleFileResource holds the data fields for a handleFileResource record.
func handleFileResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	path := model.IdToPath(id)
	query := `query File($path: String!) { file(path: $path) { id path name kind extension folder { path } bundle { name } breachs { id } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleSectionsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	path := model.IdToPath(id)
	query := `query Sections($path: String!) { file(path: $path) { sections { id path name range { start end } definitions { name } children { name } } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📑️handleSectionResource holds the data fields for a handleSectionResource record.
func handleSectionResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid section URI: %s", request.Params.URI)
	}
	path := model.IdToPath(id)
	sectionSlugs := model.IdToSectionPath(id)
	if len(sectionSlugs) == 0 {
		return nil, fmt.Errorf("invalid section URI: %s", request.Params.URI)
	}
	sectionPath := make([]string, len(sectionSlugs))
	for i, s := range sectionSlugs {
		sectionPath[i] = workspace.TitleizeSlug(s)
	}

	query := `query Section($path: String!, $sectionPath: [String!]!) { section(path: $path, sectionPath: $sectionPath) { id path name range { start end } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path, "sectionPath": sectionPath})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📖️handleDefinitionsResource holds the data fields for a handleDefinitionsResource record.
func handleDefinitionsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	path := model.IdToPath(id)
	query := `query Definitions($path: String!) { file(path: $path) { definitions { id name kind range { start end } } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔷️handleDefinitionResource holds the data fields for a handleDefinitionResource record.
func handleDefinitionResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid definition URI: %s", request.Params.URI)
	}
	path := model.IdToPath(id)
	defName := model.IdToDefinitionName(id)
	if defName == "" {
		return nil, fmt.Errorf("invalid definition URI: %s", request.Params.URI)
	}
	name := workspace.TitleizeSlug(defName)

	query := `query Definition($path: String!, $name: String!) { definition(path: $path, name: $name) { id name kind range { start end } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"path": path, "name": name})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🎫️handleTicketsResource holds the data fields for a handleTicketsResource record.
func handleTicketsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Tickets { repo { tickets { id slug title status prompt interactions { prompt } } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔶️handleTicketResource holds the data fields for a handleTicketResource record.
func handleTicketResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid ticket URI: %s", request.Params.URI)
	}
	// Parse ticket ID to extract year/month/day/slug
	year, month, day, slug := ticketspkg.TicketIdToParts(id)
	if slug == "" {
		return nil, fmt.Errorf("invalid ticket ID: %s", id)
	}

	query := `query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id slug title status prompt interactions { prompt dates { started finished } } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"year": year, "month": month, "day": day, "slug": slug})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleGoalsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	goals, err := goalspkg.ListGoals()
	if err != nil {
		return nil, err
	}
	bytes, err := json.Marshal(goals)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(string(bytes))
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// ⛳️handleGoalResource holds the data fields for a handleGoalResource record.
func handleGoalResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid goal URI: %s", request.Params.URI)
	}
	// Convert emoji goal ID back to goal path
	goalPath := model.ComposeIDToGoalPath(id)
	goals, err := goalspkg.ListGoals()
	if err != nil {
		return nil, err
	}
	for _, g := range goals {
		if g.ID == goalPath {
			bytes, err := json.Marshal(g)
			if err != nil {
				return nil, err
			}
			yaml, err := languagespkg.JsonToYaml(string(bytes))
			if err != nil {
				return nil, err
			}
			return []ResourceContents{
				TextResourceContents{
					URI:      request.Params.URI,
					MIMEType: "text/plain",
					Text:     yaml,
				},
			}, nil
		}
	}
	return nil, fmt.Errorf("goal not found: %s", goalPath)
}

// 🔹️handlePoliciesResource holds the data fields for a handlePoliciesResource record.
func handlePoliciesResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Policies { repo { policies { id description breachs { id } } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📜️handlePolicyResource holds the data fields for a handlePolicyResource record.
func handlePolicyResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid policy URI: %s", request.Params.URI)
	}
	query := `query Policy($id: String!) { policy(id: $id) { id description breachs { id } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔸️handleStatutesResource holds the data fields for a handleStatutesResource record.
func handleStatutesResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Statutes { repo { statutes { id priority autofixable reason solution } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔺️handleStatuteResource holds the data fields for a handleStatuteResource record.
func handleStatuteResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid statute URI: %s", request.Params.URI)
	}
	query := `query Statute($id: String!) { statute(id: $id) { id priority autofixable reason solution } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🤝️handleContributorsResource holds the data fields for a handleContributorsResource record.
func handleContributorsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Contributors { repo { contributors { id emails name contributions { checkpoints { id } tickets { id } } } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔻️handleContributorResource holds the data fields for a handleContributorResource record.
func handleContributorResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	id := model.UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid contributor URI: %s", request.Params.URI)
	}
	query := `query Contributor($id: String!) { contributor(id: $id) { id emails name contributions { checkpoints { id } tickets { id } } } }`
	result, err := graphqlpkg.Gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 💾️handleCheckpointsResource holds the data fields for a handleCheckpointsResource record.
func handleCheckpointsResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	query := `query Checkpoints { repo { checkpoints { id sha title date } } }`
	result, err := graphqlpkg.Gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := languagespkg.JsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []ResourceContents{
		TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleCheckpointResource(ctx context.Context, request ReadResourceRequest) ([]ResourceContents, error) {
	return nil, fmt.Errorf("checkpoint resource not implemented")
}

// #endregion 🏬️Mcp Resources Handlers

// #region 🗣️McpDescriptions

func mcpDesc(kind providers.McpClientKind, key string) string {
	if kind == "" {
		kind = providers.McpClientGeneric
	}
	if m, ok := mcpDescriptionTable[key]; ok {
		if s, ok2 := m[kind]; ok2 && s != "" {
			return s
		}
		if s, ok3 := m[providers.McpClientGeneric]; ok3 {
			return s
		}
	}
	return ""
}

// mcpDescriptionTable: keys = stable ids; inner map must include generic + each IDE kind used.
var mcpDescriptionTable = map[string]map[providers.McpClientKind]string{
	"prompt_enhance": {
		providers.McpClientGeneric: "Use when you are about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCursor:  "Use when the Cursor agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientKiro:    "Use when the Kiro agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCopilot: "Use when the Copilot agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientClaude:  "Use when Claude Code is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCodex:   "Use when Codex is about to add behavior and must align the change with repository rules before editing.",
	},
	"prompt_refactor": {
		providers.McpClientGeneric: "Use when a refactor is required and you must not regress behavior or tests.",
		providers.McpClientCursor:  "Use when the Cursor agent must refactor without regressing behavior or tests.",
		providers.McpClientKiro:    "Use when the Kiro agent must refactor without regressing behavior or tests.",
		providers.McpClientCopilot: "Use when the Copilot agent must refactor without regressing behavior or tests.",
		providers.McpClientClaude:  "Use when Claude Code must refactor without regressing behavior or tests.",
		providers.McpClientCodex:   "Use when Codex must refactor without regressing behavior or tests.",
	},
	"prompt_test": {
		providers.McpClientGeneric: "Use when tests must be extended before you claim coverage for new paths.",
		providers.McpClientCursor:  "Use when the Cursor agent must extend tests before claiming coverage for new paths.",
		providers.McpClientKiro:    "Use when the Kiro agent must extend tests before claiming coverage for new paths.",
		providers.McpClientCopilot: "Use when the Copilot agent must extend tests before claiming coverage for new paths.",
		providers.McpClientClaude:  "Use when Claude Code must extend tests before claiming coverage for new paths.",
		providers.McpClientCodex:   "Use when Codex must extend tests before claiming coverage for new paths.",
	},
	"prompt_comply": {
		providers.McpClientGeneric: "Use when the tree is red and you must converge implementation to passing tests without deleting assertions.",
		providers.McpClientCursor:  "Use when the Cursor agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientKiro:    "Use when the Kiro agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientCopilot: "Use when the Copilot agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientClaude:  "Use when Claude Code must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientCodex:   "Use when Codex must converge a red workspace to passing tests without deleting assertions.",
	},
	"res_root": {
		providers.McpClientGeneric: "Fetch when the session anchor for the repository root is required before any scoped read.",
		providers.McpClientCursor:  "Fetch when the Cursor session anchor for the repository root is required before any scoped read.",
		providers.McpClientKiro:    "Fetch when the Kiro session anchor for the repository root is required before any scoped read.",
		providers.McpClientCopilot: "Fetch when the Copilot session anchor for the repository root is required before any scoped read.",
		providers.McpClientClaude:  "Fetch when Claude Code needs the repository root anchor before any scoped read.",
		providers.McpClientCodex:   "Fetch when Codex needs the repository root anchor before any scoped read.",
	},
	"res_bundles": {
		providers.McpClientGeneric: "Fetch when bundle inventory is required before choosing a technology subtree.",
		providers.McpClientCursor:  "Fetch when bundle inventory is required before the Cursor agent chooses a technology subtree.",
		providers.McpClientKiro:    "Fetch when bundle inventory is required before the Kiro agent chooses a technology subtree.",
		providers.McpClientCopilot: "Fetch when bundle inventory is required before the Copilot agent chooses a technology subtree.",
		providers.McpClientClaude:  "Fetch when bundle inventory is required before Claude Code chooses a technology subtree.",
		providers.McpClientCodex:   "Fetch when bundle inventory is required before Codex chooses a technology subtree.",
	},
	"res_bundle_one": {
		providers.McpClientGeneric: "Fetch when a single bundle context gate is required before editing that bundle.",
		providers.McpClientCursor:  "Fetch when the Cursor agent needs one bundle context gate before editing that bundle.",
		providers.McpClientKiro:    "Fetch when the Kiro agent needs one bundle context gate before editing that bundle.",
		providers.McpClientCopilot: "Fetch when the Copilot agent needs one bundle context gate before editing that bundle.",
		providers.McpClientClaude:  "Fetch when Claude Code needs one bundle context gate before editing that bundle.",
		providers.McpClientCodex:   "Fetch when Codex needs one bundle context gate before editing that bundle.",
	},
	"res_folders": {
		providers.McpClientGeneric: "Fetch when folder enumeration is required before path-sensitive edits.",
		providers.McpClientCursor:  "Fetch when folder enumeration is required before the Cursor agent performs path-sensitive edits.",
		providers.McpClientKiro:    "Fetch when folder enumeration is required before the Kiro agent performs path-sensitive edits.",
		providers.McpClientCopilot: "Fetch when folder enumeration is required before the Copilot agent performs path-sensitive edits.",
		providers.McpClientClaude:  "Fetch when folder enumeration is required before Claude Code performs path-sensitive edits.",
		providers.McpClientCodex:   "Fetch when folder enumeration is required before Codex performs path-sensitive edits.",
	},
	"res_folder_one": {
		providers.McpClientGeneric: "Fetch when one folder subtree must be confirmed before moves or renames under it.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one folder subtree before moves or renames under it.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one folder subtree before moves or renames under it.",
	},
	"res_files": {
		providers.McpClientGeneric: "Fetch when file listing is required before batch reads across a scope.",
		providers.McpClientCursor:  "Fetch when file listing is required before the Cursor agent batch-reads a scope.",
		providers.McpClientKiro:    "Fetch when file listing is required before the Kiro agent batch-reads a scope.",
		providers.McpClientCopilot: "Fetch when file listing is required before the Copilot agent batch-reads a scope.",
		providers.McpClientClaude:  "Fetch when file listing is required before Claude Code batch-reads a scope.",
		providers.McpClientCodex:   "Fetch when file listing is required before Codex batch-reads a scope.",
	},
	"res_file_one": {
		providers.McpClientGeneric: "Fetch when a single file identity must be verified before a targeted edit.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must verify a single file identity before a targeted edit.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must verify a single file identity before a targeted edit.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must verify a single file identity before a targeted edit.",
		providers.McpClientClaude:  "Fetch when Claude Code must verify a single file identity before a targeted edit.",
		providers.McpClientCodex:   "Fetch when Codex must verify a single file identity before a targeted edit.",
	},
	"res_sections": {
		providers.McpClientGeneric: "Fetch when section discovery is required before region-scoped refactors.",
		providers.McpClientCursor:  "Fetch when section discovery is required before the Cursor agent runs region-scoped refactors.",
		providers.McpClientKiro:    "Fetch when section discovery is required before the Kiro agent runs region-scoped refactors.",
		providers.McpClientCopilot: "Fetch when section discovery is required before the Copilot agent runs region-scoped refactors.",
		providers.McpClientClaude:  "Fetch when section discovery is required before Claude Code runs region-scoped refactors.",
		providers.McpClientCodex:   "Fetch when section discovery is required before Codex runs region-scoped refactors.",
	},
	"res_section_one": {
		providers.McpClientGeneric: "Fetch when one section boundary must be loaded before a surgical edit inside that section.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientClaude:  "Fetch when Claude Code must load one section boundary before a surgical edit inside that section.",
		providers.McpClientCodex:   "Fetch when Codex must load one section boundary before a surgical edit inside that section.",
	},
	"res_definitions": {
		providers.McpClientGeneric: "Fetch when definition discovery is required before symbol-level work.",
		providers.McpClientCursor:  "Fetch when definition discovery is required before the Cursor agent performs symbol-level work.",
		providers.McpClientKiro:    "Fetch when definition discovery is required before the Kiro agent performs symbol-level work.",
		providers.McpClientCopilot: "Fetch when definition discovery is required before the Copilot agent performs symbol-level work.",
		providers.McpClientClaude:  "Fetch when definition discovery is required before Claude Code performs symbol-level work.",
		providers.McpClientCodex:   "Fetch when definition discovery is required before Codex performs symbol-level work.",
	},
	"res_definition_one": {
		providers.McpClientGeneric: "Fetch when one definition record must be confirmed before renaming or extracting it.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one definition record before renaming or extracting it.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one definition record before renaming or extracting it.",
	},
	"res_tickets": {
		providers.McpClientGeneric: "Fetch when ticket inventory is required before choosing where to attach work.",
		providers.McpClientCursor:  "Fetch when ticket inventory is required before the Cursor agent chooses where to attach work.",
		providers.McpClientKiro:    "Fetch when ticket inventory is required before the Kiro agent chooses where to attach work.",
		providers.McpClientCopilot: "Fetch when ticket inventory is required before the Copilot agent chooses where to attach work.",
		providers.McpClientClaude:  "Fetch when ticket inventory is required before Claude Code chooses where to attach work.",
		providers.McpClientCodex:   "Fetch when ticket inventory is required before Codex chooses where to attach work.",
	},
	"res_ticket_one": {
		providers.McpClientGeneric: "Fetch when one ticket record must be read before closing or reopening that ticket.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one ticket record before closing or reopening that ticket.",
		providers.McpClientCodex:   "Fetch when Codex must read one ticket record before closing or reopening that ticket.",
	},
	"res_goals": {
		providers.McpClientGeneric: "Fetch when goal document is required before opening or linking a ticket.",
		providers.McpClientCursor:  "Fetch when goal document is required before the Cursor agent opens or links a ticket.",
		providers.McpClientKiro:    "Fetch when goal document is required before the Kiro agent opens or links a ticket.",
		providers.McpClientCopilot: "Fetch when goal document is required before the Copilot agent opens or links a ticket.",
		providers.McpClientClaude:  "Fetch when goal document is required before Claude Code opens or links a ticket.",
		providers.McpClientCodex:   "Fetch when goal document is required before Codex opens or links a ticket.",
	},
	"res_goal_one": {
		providers.McpClientGeneric: "Fetch when one goal record must be confirmed before milestone or ticket association.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one goal record before milestone or ticket association.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one goal record before milestone or ticket association.",
	},
	"res_policies": {
		providers.McpClientGeneric: "Fetch when policy inventory is required before an audit or autofix pass.",
		providers.McpClientCursor:  "Fetch when policy inventory is required before the Cursor agent starts an audit or autofix pass.",
		providers.McpClientKiro:    "Fetch when policy inventory is required before the Kiro agent starts an audit or autofix pass.",
		providers.McpClientCopilot: "Fetch when policy inventory is required before the Copilot agent starts an audit or autofix pass.",
		providers.McpClientClaude:  "Fetch when policy inventory is required before Claude Code starts an audit or autofix pass.",
		providers.McpClientCodex:   "Fetch when policy inventory is required before Codex starts an audit or autofix pass.",
	},
	"res_policy_one": {
		providers.McpClientGeneric: "Fetch when one policy scope must be verified before interpreting breaches.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must verify one policy scope before interpreting breaches.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must verify one policy scope before interpreting breaches.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must verify one policy scope before interpreting breaches.",
		providers.McpClientClaude:  "Fetch when Claude Code must verify one policy scope before interpreting breaches.",
		providers.McpClientCodex:   "Fetch when Codex must verify one policy scope before interpreting breaches.",
	},
	"res_statutes": {
		providers.McpClientGeneric: "Fetch when breach-kind listing is required before triage ordering.",
		providers.McpClientCursor:  "Fetch when breach-kind listing is required before the Cursor agent orders triage.",
		providers.McpClientKiro:    "Fetch when breach-kind listing is required before the Kiro agent orders triage.",
		providers.McpClientCopilot: "Fetch when breach-kind listing is required before the Copilot agent orders triage.",
		providers.McpClientClaude:  "Fetch when breach-kind listing is required before Claude Code orders triage.",
		providers.McpClientCodex:   "Fetch when breach-kind listing is required before Codex orders triage.",
	},
	"res_statute_one": {
		providers.McpClientGeneric: "Fetch when one breach-kind record must be read before applying a fix strategy.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one breach-kind record before applying a fix strategy.",
		providers.McpClientCodex:   "Fetch when Codex must read one breach-kind record before applying a fix strategy.",
	},
	"res_contributors": {
		providers.McpClientGeneric: "Fetch when contributor listing is required before attributing sessions or checkpoints.",
		providers.McpClientCursor:  "Fetch when contributor listing is required before the Cursor agent attributes sessions or checkpoints.",
		providers.McpClientKiro:    "Fetch when contributor listing is required before the Kiro agent attributes sessions or checkpoints.",
		providers.McpClientCopilot: "Fetch when contributor listing is required before the Copilot agent attributes sessions or checkpoints.",
		providers.McpClientClaude:  "Fetch when contributor listing is required before Claude Code attributes sessions or checkpoints.",
		providers.McpClientCodex:   "Fetch when contributor listing is required before Codex attributes sessions or checkpoints.",
	},
	"res_contributor_one": {
		providers.McpClientGeneric: "Fetch when one contributor record must be read before ownership-sensitive edits.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one contributor record before ownership-sensitive edits.",
		providers.McpClientCodex:   "Fetch when Codex must read one contributor record before ownership-sensitive edits.",
	},
	"res_checkpoints": {
		providers.McpClientGeneric: "Fetch when checkpoint inventory is required before comparing versions.",
		providers.McpClientCursor:  "Fetch when checkpoint inventory is required before the Cursor agent compares versions.",
		providers.McpClientKiro:    "Fetch when checkpoint inventory is required before the Kiro agent compares versions.",
		providers.McpClientCopilot: "Fetch when checkpoint inventory is required before the Copilot agent compares versions.",
		providers.McpClientClaude:  "Fetch when checkpoint inventory is required before Claude Code compares versions.",
		providers.McpClientCodex:   "Fetch when checkpoint inventory is required before Codex compares versions.",
	},
	"res_checkpoint_one": {
		providers.McpClientGeneric: "Fetch when one checkpoint record must be read before tying hooks or tickets to history.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientCodex:   "Fetch when Codex must read one checkpoint record before tying hooks or tickets to history.",
	},
	"tool_ticket_open": {
		providers.McpClientGeneric: "Use at the start of any tracked task that will touch the repository and needs a durable workspace folder.",
		providers.McpClientCursor:  "Use at the start of a Cursor agent task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		providers.McpClientKiro:    "Use at the start of a Kiro agent task that will touch the repository and needs a durable workspace folder; use when a spec id should be bound for later archival on close.",
		providers.McpClientCopilot: "Use at the start of a Copilot agent task that will touch the repository and needs a durable workspace folder; use when a memory plan id should be bound for later archival on close.",
		providers.McpClientClaude:  "Use at the start of a Claude Code task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		providers.McpClientCodex:   "Use at the start of a Codex task that will touch the repository and needs a durable workspace folder; use when a memory id should be bound for later archival on close.",
	},
	"tool_ticket_close": {
		providers.McpClientGeneric: "Use only when the tracked task is finished, the summary is final, and the touched paths list is complete. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCursor:  "Use only when the Cursor agent task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientKiro:    "Use only when the Kiro agent task is finished, the summary is final, the touched paths list is complete, and any bound spec should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCopilot: "Use only when the Copilot agent task is finished, the summary is final, the touched paths list is complete, and any bound memory plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientClaude:  "Use only when the Claude Code task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCodex:   "Use only when the Codex task is finished, the summary is final, the touched paths list is complete, and any bound memory should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
	},
	"tool_ticket_reopen": {
		providers.McpClientGeneric: "Use when work must continue on a closed ticket before any new edits land.",
		providers.McpClientCursor:  "Use when the Cursor agent must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		providers.McpClientKiro:    "Use when the Kiro agent must continue a closed ticket before new edits; use when rebinding a spec id for archival on the next close.",
		providers.McpClientCopilot: "Use when the Copilot agent must continue a closed ticket before new edits; use when rebinding a memory plan id for archival on the next close.",
		providers.McpClientClaude:  "Use when Claude Code must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		providers.McpClientCodex:   "Use when Codex must continue a closed ticket before new edits; use when rebinding a memory id for archival on the next close.",
	},
	"tool_section_move": {
		providers.McpClientGeneric: "Use when a region rename is required and the surrounding file context is already loaded.",
		providers.McpClientCursor:  "Use when the Cursor agent must rename a region and the surrounding file context is already loaded.",
		providers.McpClientKiro:    "Use when the Kiro agent must rename a region and the surrounding file context is already loaded.",
		providers.McpClientCopilot: "Use when the Copilot agent must rename a region and the surrounding file context is already loaded.",
		providers.McpClientClaude:  "Use when Claude Code must rename a region and the surrounding file context is already loaded.",
		providers.McpClientCodex:   "Use when Codex must rename a region and the surrounding file context is already loaded.",
	},
	"tool_file_integrate": {
		providers.McpClientGeneric: "Use when two files must be merged at a named region boundary without losing section markers.",
		providers.McpClientCursor:  "Use when the Cursor agent must merge two files at a named region boundary without losing section markers.",
		providers.McpClientKiro:    "Use when the Kiro agent must merge two files at a named region boundary without losing section markers.",
		providers.McpClientCopilot: "Use when the Copilot agent must merge two files at a named region boundary without losing section markers.",
		providers.McpClientClaude:  "Use when Claude Code must merge two files at a named region boundary without losing section markers.",
		providers.McpClientCodex:   "Use when Codex must merge two files at a named region boundary without losing section markers.",
	},
	"tool_goal_open": {
		providers.McpClientGeneric: "Call when a new goal must be recorded before any ticket can reference it.",
		providers.McpClientCursor:  "Call when the Cursor agent must record a new goal before any ticket references it.",
		providers.McpClientKiro:    "Call when the Kiro agent must record a new goal before any ticket references it.",
		providers.McpClientCopilot: "Call when the Copilot agent must record a new goal before any ticket references it.",
		providers.McpClientClaude:  "Call when Claude Code must record a new goal before any ticket references it.",
		providers.McpClientCodex:   "Call when Codex must record a new goal before any ticket references it.",
	},
	"tool_goal_close": {
		providers.McpClientGeneric: "Call when a goal is fulfilled and must be closed with an evidence summary.",
		providers.McpClientCursor:  "Call when the Cursor agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientKiro:    "Call when the Kiro agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientCopilot: "Call when the Copilot agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientClaude:  "Call when Claude Code has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientCodex:   "Call when Codex has fulfilled a goal and must close it with an evidence summary.",
	},
	"tool_goal_reopen": {
		providers.McpClientGeneric: "Call when a closed goal must be reopened because its outcome did not hold.",
		providers.McpClientCursor:  "Call when the Cursor agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientKiro:    "Call when the Kiro agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientCopilot: "Call when the Copilot agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientClaude:  "Call when Claude Code must reopen a closed goal because its outcome did not hold.",
		providers.McpClientCodex:   "Call when Codex must reopen a closed goal because its outcome did not hold.",
	},
	"tool_section_extract": {
		providers.McpClientGeneric: "Use when a region must be split out into its own file before shrinking the original module.",
		providers.McpClientCursor:  "Use when the Cursor agent must split a region into its own file before shrinking the original module.",
		providers.McpClientKiro:    "Use when the Kiro agent must split a region into its own file before shrinking the original module.",
		providers.McpClientCopilot: "Use when the Copilot agent must split a region into its own file before shrinking the original module.",
		providers.McpClientClaude:  "Use when Claude Code must split a region into its own file before shrinking the original module.",
		providers.McpClientCodex:   "Use when Codex must split a region into its own file before shrinking the original module.",
	},
	"arg_plan_id": {
		providers.McpClientCursor:  "Set when a `.cursor/plans/*_<id>.plan.md` file exists and must be archived into the ticket on close.",
		providers.McpClientCopilot: "Set when a Copilot project memory file for this id exists and must be archived into the ticket on close.",
		providers.McpClientClaude:  "Set when a `.claude/plan/<id>.md` file exists and must be archived into the ticket on close.",
		providers.McpClientCodex:   "Set when a `.codex/memory/<repo>/<id>.md` file exists and must be archived into the ticket on close.",
	},
	"arg_spec_id": {
		providers.McpClientKiro: "Set when a `.kiro/specs/<id>/` directory exists and must be archived into the ticket on close.",
	},
}

// #endregion 🗣️McpDescriptions

// #region 🦀️McpServerFactory
// wrapMcpToolHandler enforces a wall-clock budget on each MCP tool invocation.
func wrapMcpToolHandler(timeout time.Duration, handler func(context.Context, CallToolRequest) (*CallToolResult, error)) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		if timeout <= 0 {
			return handler(ctx, request)
		}
		toolCtx, cancel := context.WithTimeout(ctx, timeout)
		defer cancel()
		type outcome struct {
			result *CallToolResult
			err    error
		}
		ch := make(chan outcome, 1)
		go func() {
			r, e := handler(toolCtx, request)
			ch <- outcome{r, e}
		}()
		select {
		case o := <-ch:
			return o.result, o.err
		case <-toolCtx.Done():
			return nil, fmt.Errorf("tool call timed out after %s: %w", timeout, toolCtx.Err())
		}
	}
}

// 🦀️CreateMcpServer builds the MCP server for the given IDE kind (stdio).
func CreateMcpServer(kind providers.McpClientKind, toolTimeout time.Duration) *MCPServer {
	if kind == "" {
		kind = providers.McpClientGeneric
	}
	s := NewMCPServer(
		providers.McpServerName(kind),
		"1.0.0",
		WithToolCapabilities(true),
		WithPromptCapabilities(true),
	)
	s.AddPrompt(
		NewPrompt("enhance",
			WithPromptDescription(mcpDesc(kind, "prompt_enhance")),
			WithArgument("prompt", ArgumentDescription("Context the agent must honor while enhancing."), RequiredArgument()),
		),
		handleEnhancePrompt,
	)
	s.AddPrompt(
		NewPrompt("refactor",
			WithPromptDescription(mcpDesc(kind, "prompt_refactor")),
			WithArgument("prompt", ArgumentDescription("Refactor constraints the agent must honor."), RequiredArgument()),
		),
		handleRefactorPrompt,
	)
	s.AddPrompt(
		NewPrompt("test",
			WithPromptDescription(mcpDesc(kind, "prompt_test")),
			WithArgument("prompt", ArgumentDescription("Coverage targets the agent must honor."), RequiredArgument()),
		),
		handleTestPrompt,
	)
	s.AddPrompt(
		NewPrompt("comply",
			WithPromptDescription(mcpDesc(kind, "prompt_comply")),
			WithArgument("prompt", ArgumentDescription("Compliance constraints the agent must honor."), RequiredArgument()),
		),
		handleComplyPrompt,
	)
	s.AddResource(
		NewResource("repo://", mcpDesc(kind, "res_root"), WithMIMEType("text/plain")),
		handleRepoResource,
	)
	s.AddResource(
		NewResource("repo://bundles", mcpDesc(kind, "res_bundles"), WithMIMEType("text/plain")),
		handleBundlesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://bundle/{id}", mcpDesc(kind, "res_bundle_one")),
		handleBundleResource,
	)
	s.AddResource(
		NewResource("repo://folders", mcpDesc(kind, "res_folders"), WithMIMEType("text/plain")),
		handleFoldersResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://folder/{id}", mcpDesc(kind, "res_folder_one")),
		handleFolderResource,
	)
	s.AddResource(
		NewResource("repo://files", mcpDesc(kind, "res_files"), WithMIMEType("text/plain")),
		handleFilesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://file/{id}", mcpDesc(kind, "res_file_one")),
		handleFileResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://sections/{id}", mcpDesc(kind, "res_sections")),
		handleSectionsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://section/{id}", mcpDesc(kind, "res_section_one")),
		handleSectionResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://definitions/{id}", mcpDesc(kind, "res_definitions")),
		handleDefinitionsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://definition/{id}", mcpDesc(kind, "res_definition_one")),
		handleDefinitionResource,
	)
	s.AddResource(
		NewResource("repo://tickets", mcpDesc(kind, "res_tickets"), WithMIMEType("text/plain")),
		handleTicketsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://ticket/{id}", mcpDesc(kind, "res_ticket_one")),
		handleTicketResource,
	)
	s.AddResource(
		NewResource("repo://goals", mcpDesc(kind, "res_goals"), WithMIMEType("text/plain")),
		handleGoalsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://goal/{id}", mcpDesc(kind, "res_goal_one")),
		handleGoalResource,
	)
	s.AddResource(
		NewResource("repo://policies", mcpDesc(kind, "res_policies"), WithMIMEType("text/plain")),
		handlePoliciesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://policy/{id}", mcpDesc(kind, "res_policy_one")),
		handlePolicyResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://statute/{id}", mcpDesc(kind, "res_statute_one")),
		handleStatuteResource,
	)
	s.AddResource(
		NewResource("repo://contributors", mcpDesc(kind, "res_contributors"), WithMIMEType("text/plain")),
		handleContributorsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://contributor/{id}", mcpDesc(kind, "res_contributor_one")),
		handleContributorResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://checkpoint/{id}", mcpDesc(kind, "res_checkpoint_one")),
		handleCheckpointResource,
	)

	openOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_open")),
		WithString("emoji", Required(), Description("Single emoji representing the ticket (e.g. '🎫️', '🐛️', '✨️').")),
		WithString("title", Required(), Description("Short title for the ticket. Shape is not enforced.")),
		WithString("prompt", Required(), Description("Full description of the task.")),
		WithString("goal", Required(), Description("Goal id to associate the ticket with (from repo://goals).")),
		WithString("client", Description("Agent client used for this ticket.")),
		WithString("llm", Description("LLM used for this ticket.")),
		WithString("effort", Description("Reasoning effort for the LLM (low, medium, high, max).")),
		WithString("draft", Description("Draft slug to seed the ticket workspace.")),
		WithString("parent", Description("Parent ticket slug for nested tickets.")),
		WithString("issue", Description("Existing GitHub issue URL to link instead of creating a new one.")),
	}
	switch kind {
	case providers.McpClientCursor, providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		openOpts = append(openOpts, WithString("plan_id", Description(mcpDesc(kind, "arg_plan_id"))))
	case providers.McpClientKiro:
		openOpts = append(openOpts, WithString("spec_id", Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(NewTool("ticket_open", openOpts...), wrapMcpToolHandler(toolTimeout, newTicketOpenHandler(kind)))

	closeOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_close")),
		WithString("summary", Required(), Description("Summary of the work done.")),
		WithArray("files", Description("Files created, updated, or removed during the ticket."), WithStringItems()),
		WithString("path", Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to close the latest open ticket.")),
		WithString("title", Description("Updated title for the ticket.")),
		WithBoolean("no_management", Description("Skip updating the GitHub issue.")),
	}
	s.AddTool(NewTool("ticket_close", closeOpts...), wrapMcpToolHandler(toolTimeout, newTicketCloseHandler(kind)))

	reopenOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_reopen")),
		WithString("path", Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to reopen the latest closed ticket.")),
		WithString("prompt", Description("Updated or additional task description.")),
		WithString("llm", Description("LLM to use for the reopened ticket.")),
		WithString("effort", Description("Reasoning effort for the LLM (low, medium, high, max).")),
		WithString("client", Description("Agent client to use for the reopened ticket.")),
		WithString("title", Description("Updated title for the ticket.")),
		WithString("draft", Description("Draft slug to seed the ticket workspace.")),
		WithBoolean("no_management", Description("Skip updating the GitHub issue.")),
	}
	switch kind {
	case providers.McpClientCursor, providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		reopenOpts = append(reopenOpts, WithString("plan_id", Description(mcpDesc(kind, "arg_plan_id"))))
	case providers.McpClientKiro:
		reopenOpts = append(reopenOpts, WithString("spec_id", Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(NewTool("ticket_reopen", reopenOpts...), wrapMcpToolHandler(toolTimeout, newTicketReopenHandler(kind)))

	s.AddTool(
		NewTool("section_move",
			WithDescription(mcpDesc(kind, "tool_section_move")),
			WithString("file", Required(), Description("Path to the file containing the section.")),
			WithString("old_name", Required(), Description("Current name of the section.")),
			WithString("new_name", Required(), Description("New name for the section.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionMove),
	)
	s.AddTool(
		NewTool("file_integrate",
			WithDescription(mcpDesc(kind, "tool_file_integrate")),
			WithString("source", Required(), Description("Path to the source file.")),
			WithString("target_section", Required(), Description("Name of the section in the target file to integrate into.")),
			WithString("target_file", Required(), Description("Path to the target file.")),
			WithString("target_parent_section", Description("Name of the parent section in the target file.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionIntegrate),
	)
	s.AddTool(
		NewTool("section_extract",
			WithDescription(mcpDesc(kind, "tool_section_extract")),
			WithString("source_file", Required(), Description("Path to the source file.")),
			WithString("source_section", Required(), Description("Name of the section to extract.")),
			WithString("target_file", Required(), Description("Path to the target file where the section will be written.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionExtract),
	)
	s.AddTool(
		NewTool("goal_open",
			WithDescription(mcpDesc(kind, "tool_goal_open")),
			WithString("title", Required(), Description("Goal title.")),
			WithString("prompt", Required(), Description("Goal prompt.")),
			WithString("description", Description("Goal description.")),
			WithString("due_date", Description("YYYY-MM-DD due date.")),
			WithString("llm", Description("Model.")),
			WithString("client", Description("Agent client.")),
			WithString("parent", Description("Parent goal id.")),
			WithString("milestone", Description("Management milestone.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, goalOpen),
	)
	s.AddTool(
		NewTool("goal_close",
			WithDescription(mcpDesc(kind, "tool_goal_close")),
			WithString("id", Required(), Description("Goal id.")),
			WithString("summary", Required(), Description("Completion summary.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, goalClose),
	)
	s.AddTool(
		NewTool("goal_reopen",
			WithDescription(mcpDesc(kind, "tool_goal_reopen")),
			WithString("id", Required(), Description("Goal id.")),
			WithString("prompt", Required(), Description("Additional goal prompt.")),
			WithString("llm", Required(), Description("Model.")),
			WithString("client", Required(), Description("Agent client.")),
			WithString("title", Description("Updated title.")),
			WithString("description", Description("Updated description.")),
			WithString("due_date", Description("YYYY-MM-DD due date.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, goalReopen),
	)
	return s
}

// RunMcpServerFor starts the MCP stdio server for the given kind.
func RunMcpServerFor(kind providers.McpClientKind, toolTimeout time.Duration) error {
	s := CreateMcpServer(kind, toolTimeout)
	return ServeStdio(s)
}

// #endregion 🦀️McpServerFactory

// #region 🪝️TicketMcpHandlers
func newTicketOpenHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		return ticketOpenWithKind(ctx, request, kind)
	}
}

func newTicketCloseHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		_ = kind
		return ticketClose(ctx, request)
	}
}

func newTicketReopenHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		return ticketReopenWithKind(ctx, request, kind)
	}
}

// RunHookFor runs a hook for the IDE kind (client is implied; stdin is hook JSON payload).
func RunHookFor(kind providers.McpClientKind, eventStr string, stdin []byte) error {
	client := providers.HookClientForMcpKind(kind)
	if client == "" {
		return fmt.Errorf("hooks are not available for mcp kind %q", kind)
	}
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	repoRoot := workspace.FindRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)
	var input json.RawMessage
	if len(stdin) > 0 {
		input = json.RawMessage(stdin)
	}
	ctx, cancel := context.WithTimeout(context.Background(), DefaultCommandTimeout)
	defer cancel()
	return hooks.RunHookExecutionCtx(ctx, client, eventStr, "", "", "", "", repoRoot, input, false, os.Stdout, os.Stderr)
}

// RunMCPFor starts the MCP stdio server for the given IDE kind.
func RunMCPFor(kind providers.McpClientKind) error {
	return RunMcpServerFor(kind, DefaultCommandTimeout)
}

// #endregion 🪝️TicketMcpHandlers

// #region 🔌️Mcp Builders

type TextContent struct {
	Type string `json:"type"`
	Text string `json:"text"`
}

type CallToolResult struct {
	Content []TextContent `json:"content"`
	IsError bool          `json:"isError,omitempty"`
}

type CallToolRequest struct {
	Params struct {
		Name      string      `json:"name"`
		Arguments interface{} `json:"arguments"`
	} `json:"params"`
}

type GetPromptRequest struct {
	Params struct {
		Name      string            `json:"name"`
		Arguments map[string]string `json:"arguments"`
	} `json:"params"`
}

type Role string

const RoleUser Role = "user"

type PromptMessage struct {
	Role    Role        `json:"role"`
	Content TextContent `json:"content"`
}

type GetPromptResult struct {
	Description string          `json:"description"`
	Messages    []PromptMessage `json:"messages"`
}

type ReadResourceRequest struct {
	Params struct {
		URI string `json:"uri"`
	} `json:"params"`
}

type ResourceContents interface{ resourceContents() }

type TextResourceContents struct {
	URI      string `json:"uri"`
	MIMEType string `json:"mimeType,omitempty"`
	Text     string `json:"text"`
}

func (TextResourceContents) resourceContents() {}

type InputSchema struct {
	Type       string                 `json:"type"`
	Properties map[string]interface{} `json:"properties"`
	Required   []string               `json:"required,omitempty"`
}

type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description,omitempty"`
	InputSchema InputSchema `json:"inputSchema"`
}

type Prompt struct {
	Name        string              `json:"name"`
	Description string              `json:"description,omitempty"`
	Arguments   map[string]Argument `json:"arguments,omitempty"`
}

type Argument struct {
	Description string `json:"description,omitempty"`
	Required    bool   `json:"required,omitempty"`
}

type Resource struct {
	URI         string `json:"uri"`
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	MIMEType    string `json:"mimeType,omitempty"`
}

type ResourceTemplate = Resource

func NewTextContent(text string) TextContent { return TextContent{Type: "text", Text: text} }

func NewToolResultText(text string) *CallToolResult {
	return &CallToolResult{Content: []TextContent{NewTextContent(text)}}
}

func NewPromptMessage(role Role, content TextContent) PromptMessage {
	return PromptMessage{Role: role, Content: content}
}

func NewGetPromptResult(description string, messages []PromptMessage) *GetPromptResult {
	return &GetPromptResult{Description: description, Messages: messages}
}

type PromptOption func(*Prompt)

type ArgumentOption func(*Argument)

type ResourceOption func(*Resource)

type ToolOption func(*Tool)

type SchemaOption func(map[string]interface{})

type ArgumentDescription string

func RequiredArgument() ArgumentOption { return func(argument *Argument) { argument.Required = true } }

func WithArgument(name string, description ArgumentDescription, options ...ArgumentOption) PromptOption {
	return func(prompt *Prompt) {
		argument := Argument{Description: string(description)}
		for _, option := range options {
			option(&argument)
		}
		if prompt.Arguments == nil {
			prompt.Arguments = map[string]Argument{}
		}
		prompt.Arguments[name] = argument
	}
}

func WithPromptDescription(description string) PromptOption {
	return func(prompt *Prompt) { prompt.Description = description }
}

func NewPrompt(name string, options ...PromptOption) Prompt {
	prompt := Prompt{Name: name, Arguments: map[string]Argument{}}
	for _, option := range options {
		option(&prompt)
	}
	return prompt
}

func WithMIMEType(mimeType string) ResourceOption {
	return func(resource *Resource) { resource.MIMEType = mimeType }
}

func NewResource(uri, description string, options ...ResourceOption) Resource {
	resource := Resource{URI: uri, Name: uri, Description: description}
	for _, option := range options {
		option(&resource)
	}
	return resource
}

func NewResourceTemplate(uri, description string, options ...ResourceOption) ResourceTemplate {
	return NewResource(uri, description, options...)
}

func NewTool(name string, options ...ToolOption) Tool {
	tool := Tool{Name: name, InputSchema: InputSchema{Type: "object", Properties: map[string]interface{}{}}}
	for _, option := range options {
		option(&tool)
	}
	return tool
}

func Description(description string) SchemaOption {
	return func(schema map[string]interface{}) { schema["description"] = description }
}

func Required() SchemaOption {
	return func(schema map[string]interface{}) { schema["required"] = true }
}

func WithStringItems() SchemaOption {
	return func(schema map[string]interface{}) { schema["items"] = map[string]interface{}{"type": "string"} }
}

func WithDescription(description string) ToolOption {
	return func(tool *Tool) { tool.Description = description }
}

func WithString(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "string", options...)
}

func WithBoolean(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "boolean", options...)
}

func WithArray(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "array", options...)
}

func withProperty(name, kind string, options ...SchemaOption) ToolOption {
	return func(tool *Tool) {
		schema := map[string]interface{}{"type": kind}
		for _, option := range options {
			option(schema)
		}
		if required, _ := schema["required"].(bool); required {
			delete(schema, "required")
			tool.InputSchema.Required = append(tool.InputSchema.Required, name)
		}
		tool.InputSchema.Properties[name] = schema
	}
}

// #endregion 🔌️Mcp Builders

// #region 🖥️Mcp Server Adapter

type ServerOption func(*MCPServer)

type ToolHandler func(context.Context, CallToolRequest) (*CallToolResult, error)

type PromptHandler func(context.Context, GetPromptRequest) (*GetPromptResult, error)

type ResourceHandler func(context.Context, ReadResourceRequest) ([]ResourceContents, error)

type ServerTool struct {
	Tool    Tool
	Handler ToolHandler
}

type MCPServer struct {
	Name          string
	Version       string
	tools         map[string]ServerTool
	prompts       map[string]PromptHandler
	promptCards   map[string]Prompt
	resources     map[string]ResourceHandler
	resourceCards map[string]Resource
	templateCards map[string]Resource
}

func WithToolCapabilities(_ bool) ServerOption { return func(*MCPServer) {} }

func WithPromptCapabilities(_ bool) ServerOption { return func(*MCPServer) {} }

func NewMCPServer(name, version string, options ...ServerOption) *MCPServer {
	server := &MCPServer{Name: name, Version: version, tools: map[string]ServerTool{}, prompts: map[string]PromptHandler{}, promptCards: map[string]Prompt{}, resources: map[string]ResourceHandler{}, resourceCards: map[string]Resource{}, templateCards: map[string]Resource{}}
	for _, option := range options {
		option(server)
	}
	return server
}

func (server *MCPServer) AddTool(tool Tool, handler ToolHandler) {
	server.tools[tool.Name] = ServerTool{Tool: tool, Handler: handler}
}

func (server *MCPServer) AddPrompt(prompt Prompt, handler PromptHandler) {
	server.prompts[prompt.Name] = handler
	server.promptCards[prompt.Name] = prompt
}

func (server *MCPServer) AddResource(resource Resource, handler ResourceHandler) {
	server.resources[resource.URI] = handler
	server.resourceCards[resource.URI] = resource
}

func (server *MCPServer) AddResourceTemplate(resource ResourceTemplate, handler ResourceHandler) {
	server.resources[resource.URI] = handler
	server.templateCards[resource.URI] = resource
}

// 📃️ListResources returns every declared concrete resource, ordered by URI so two runs of the same
// server declare the same vocabulary in the same order. A templated URI is not a concrete resource
// and is answered by [MCPServer.ListResourceTemplates] instead.
func (server *MCPServer) ListResources() []Resource {
	return sortedCards(server.resourceCards)
}

// 📃️ListResourceTemplates returns every declared resource template, ordered by URI.
func (server *MCPServer) ListResourceTemplates() []Resource {
	return sortedCards(server.templateCards)
}

// 🔤️sortedCards returns the values of a URI-keyed card table in URI order.
func sortedCards(cards map[string]Resource) []Resource {
	uris := make([]string, 0, len(cards))
	for uri := range cards {
		uris = append(uris, uri)
	}
	sort.Strings(uris)
	ordered := make([]Resource, 0, len(uris))
	for _, uri := range uris {
		ordered = append(ordered, cards[uri])
	}
	return ordered
}

// 📃️ListPrompts returns every declared prompt, ordered by name.
func (server *MCPServer) ListPrompts() []Prompt {
	names := make([]string, 0, len(server.promptCards))
	for name := range server.promptCards {
		names = append(names, name)
	}
	sort.Strings(names)
	prompts := make([]Prompt, 0, len(names))
	for _, name := range names {
		prompts = append(prompts, server.promptCards[name])
	}
	return prompts
}

// 🧰️ListToolCards returns every declared tool, ordered by name.
func (server *MCPServer) ListToolCards() []Tool {
	names := make([]string, 0, len(server.tools))
	for name := range server.tools {
		names = append(names, name)
	}
	sort.Strings(names)
	tools := make([]Tool, 0, len(names))
	for _, name := range names {
		tools = append(tools, server.tools[name].Tool)
	}
	return tools
}

func (server *MCPServer) ListTools() map[string]ServerTool {
	result := make(map[string]ServerTool, len(server.tools))
	for name, tool := range server.tools {
		result[name] = tool
	}
	return result
}

type request struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

type response struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id,omitempty"`
	Result  interface{}     `json:"result,omitempty"`
	Error   *rpcError       `json:"error,omitempty"`
}

type rpcError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// 📡️ServeStdio serves the session on the process standard input and output.
func ServeStdio(server *MCPServer) error {
	return Serve(server, os.Stdin, os.Stdout)
}

// 📡️Serve reads one JSON-RPC message per line and writes one response per request. A notification
// — a message carrying no identifier — is answered with nothing, as the protocol requires.
func Serve(server *MCPServer, in io.Reader, out io.Writer) error {
	scanner := bufio.NewScanner(in)
	scanner.Buffer(make([]byte, 64*1024), 4*1024*1024)
	encoder := json.NewEncoder(out)
	for scanner.Scan() {
		var incoming request
		if err := json.Unmarshal(scanner.Bytes(), &incoming); err != nil {
			if err := encoder.Encode(response{JSONRPC: "2.0", Error: &rpcError{Code: -32700, Message: "parse error"}}); err != nil {
				return err
			}
			continue
		}
		if len(incoming.ID) == 0 {
			continue
		}
		outgoing := response{JSONRPC: "2.0", ID: incoming.ID}
		switch incoming.Method {
		case "initialize":
			outgoing.Result = map[string]interface{}{"protocolVersion": "2025-06-18", "serverInfo": map[string]string{"name": server.Name, "version": server.Version}, "capabilities": map[string]interface{}{"tools": map[string]interface{}{}, "prompts": map[string]interface{}{}, "resources": map[string]interface{}{}}}
		case "tools/list":
			outgoing.Result = map[string]interface{}{"tools": server.ListToolCards()}
		case "prompts/list":
			outgoing.Result = map[string]interface{}{"prompts": server.ListPrompts()}
		case "resources/list":
			outgoing.Result = map[string]interface{}{"resources": server.ListResources()}
		case "resources/templates/list":
			outgoing.Result = map[string]interface{}{"resourceTemplates": server.ListResourceTemplates()}
		case "resources/read":
			var read ReadResourceRequest
			if err := json.Unmarshal(incoming.Params, &read.Params); err != nil {
				outgoing.Error = &rpcError{Code: -32602, Message: "invalid params"}
				break
			}
			handler, ok := server.resources[read.Params.URI]
			if !ok {
				outgoing.Error = &rpcError{Code: -32601, Message: "unknown resource"}
				break
			}
			contents, err := handler(context.Background(), read)
			if err != nil {
				outgoing.Error = &rpcError{Code: -32000, Message: err.Error()}
			} else {
				outgoing.Result = map[string]interface{}{"contents": contents}
			}
		case "tools/call":
			var call CallToolRequest
			if err := json.Unmarshal(incoming.Params, &call.Params); err != nil {
				outgoing.Error = &rpcError{Code: -32602, Message: "invalid params"}
				break
			}
			registered, ok := server.tools[call.Params.Name]
			if !ok {
				outgoing.Error = &rpcError{Code: -32601, Message: "unknown tool"}
				break
			}
			result, err := registered.Handler(context.Background(), call)
			if err != nil {
				outgoing.Error = &rpcError{Code: -32000, Message: err.Error()}
			} else {
				outgoing.Result = result
			}
		default:
			outgoing.Error = &rpcError{Code: -32601, Message: fmt.Sprintf("unknown method %q", incoming.Method)}
		}
		if err := encoder.Encode(outgoing); err != nil {
			return err
		}
	}
	return scanner.Err()
}

// #endregion 🖥️Mcp Server Adapter

// #endregion 🔌️Mcp Surface
