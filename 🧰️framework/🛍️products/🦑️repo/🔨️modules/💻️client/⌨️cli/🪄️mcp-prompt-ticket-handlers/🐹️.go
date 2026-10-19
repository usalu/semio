// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP prompt and ticket handlers.

// #endregion 🧲️Header

package client

import (
	"context"
	"fmt"
	mcp "github.com/usalu/semio/repo/client/internal/mcp"
	"strconv"
	"strings"
)

// 💿️handleEnhancePrompt holds the data fields for a handleEnhancePrompt record.
func handleEnhancePrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("enhance", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Enhance the implementation by adding more features and enhance the existing tests to cover the new features.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func handleRefactorPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("refactor", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Refactor the implementation and dont stop until all tests pass.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

// 🧪️handleTestPrompt holds the data fields for a handleTestPrompt record.
func handleTestPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("test", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Extend the current tests by testing more features.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

// 🔷️handleComplyPrompt holds the data fields for a handleComplyPrompt record.
func handleComplyPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("comply", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Get the implementation to comply the a set of tests. Dont remove any functionality from the tests.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func analyze(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
func policyCheck(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
func ticketOpenWithKind(ctx context.Context, request mcp.CallToolRequest, kind McpClientKind) (*mcp.CallToolResult, error) {
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
	client = resolveMcpTicketClient(kind, client)

	result := ToolTicketOpen(emoji, title, prompt, llm, effort, client, draft, false, goal, parent, false, issue, kind, planID, specID)
	return toolResultToMCP(result)
}

func ticketRead(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
	yearPart := strings.TrimPrefix(parts[0], EmojiYear)
	monthPart := strings.TrimPrefix(parts[1], EmojiMonth)
	dayPart := strings.TrimPrefix(parts[2], EmojiDay)
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
	ticket, err := latestOpenTicket()
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
	ticket, err := LatestTicket()
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and failed to find latest ticket: %w", err)
	}
	if ticket == nil {
		return 0, 0, 0, "", fmt.Errorf("no path provided and no tickets found")
	}
	return ticket.Year, ticket.Month, ticket.Day, ticket.Slug, nil
}

// 🔶️ticketClose holds the data fields for a ticketClose record.
func ticketClose(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
func ticketReopenWithKind(ctx context.Context, request mcp.CallToolRequest, kind McpClientKind) (*mcp.CallToolResult, error) {
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
	client = resolveMcpTicketClient(kind, client)

	year, month, day, slug, err := resolveTicketForReopen(path)
	if err != nil {
		return nil, err
	}

	result := ToolTicketReopen(year, month, day, slug, prompt, llm, effort, client, draft, title, "", "", noManagement, kind, planID, specID)
	return toolResultToMCP(result)
}

// 🗑️draftDelete holds the data fields for a draftDelete record.
func draftDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	result := ToolDraftDelete(slug)
	return toolResultToMCP(result)
}

func todoCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
	payload, err := gql(query, variables)
	if err != nil {
		return nil, err
	}
	return textResult(payload), nil
}

func todoDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	id := path
	if strings.HasPrefix(path, "repo://") {
		id = UriToId(path)
	}
	query := `mutation TodoDelete($id: ID!) { todoDelete(id: $id) }`
	payload, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	return textResult(payload), nil
}

// ⛳️goalOpen holds the data fields for a goalOpen record.
func goalOpen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
func goalClose(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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

func goalReopen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
