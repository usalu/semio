// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛎️mcp_tool_repo carries the MCP tool handlers of the cli for analysis, policy, tickets, drafts, todos, goals, export and contributors.

// #endregion 🧲️Header

package cli

import (
	context "context"
 json "encoding/json"
 sort "sort"
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	graphqlpkg "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
)

// #region 🔌️Mcp Surface

// #region 🪄️Handlers

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
func (repository *mcpRepository) ticketOpenWithKind(ctx context.Context, request CallToolRequest, kind providers.McpClientKind) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request)
 read:=func(name string) string { value,_,_:=getStringArg(args,name);return value }
 noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError};noIssue,_,argumentError:=getBoolArg(args,"no_issue");if argumentError!=nil {return nil,argumentError}
 ticket,err:=repository.repo.TicketOpen(model.TicketOpenInput{Emoji:read("emoji"),Title:read("title"),Prompt:read("prompt"),Goal:read("goal"),Client:providers.ResolveMcpTicketClient(kind,read("client")),LLM:read("llm"),Effort:read("effort"),Draft:read("draft"),Parent:read("parent"),Issue:read("issue"),PlanID:read("plan_id"),SpecID:read("spec_id"),NoIssue:noIssue,NoManagement:noManagement})
 return mcpTicketResult(ticket,err)
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
func (repository *mcpRepository) ticketClose(ctx context.Context,request CallToolRequest) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request);value,_,_:=getStringArg(args,"path")
 year,month,day,slug,err:=repository.resolveTicket(value,model.TicketStatusOpen);if err!=nil { return nil,err }
 summary,_,_:=getStringArg(args,"summary");files,_,_:=getStringSliceArg(args,"files");title,present,_:=getStringArg(args,"title");var titleValue *string;if present { titleValue=&title };noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError}
 ticket,err:=repository.repo.TicketClose(model.TicketCloseInput{Year:year,Month:month,Day:day,Slug:slug,Summary:summary,Files:files,Title:titleValue,NoManagement:noManagement});return mcpTicketResult(ticket,err)
}

// 📬️ticketReopenWithKind handles MCP ticket_reopen for a specific IDE MCP surface.
func (repository *mcpRepository) ticketReopenWithKind(ctx context.Context,request CallToolRequest,kind providers.McpClientKind) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request);read:=func(name string) string { value,_,_:=getStringArg(args,name);return value }
 year,month,day,slug,err:=repository.resolveTicket(read("path"),model.TicketStatusClosed);if err!=nil { return nil,err }
 title,present,_:=getStringArg(args,"title");var titleValue *string;if present { titleValue=&title };noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError}
 ticket,err:=repository.repo.TicketReopen(model.TicketReopenInput{Year:year,Month:month,Day:day,Slug:slug,Prompt:read("prompt"),Client:providers.ResolveMcpTicketClient(kind,read("client")),LLM:read("llm"),Effort:read("effort"),Draft:read("draft"),Title:titleValue,PlanID:read("plan_id"),SpecID:read("spec_id"),NoManagement:noManagement});return mcpTicketResult(ticket,err)
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
func (repository *mcpRepository) goalOpen(ctx context.Context,request CallToolRequest) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request);read:=func(name string) string { value,_,_:=getStringArg(args,name);return value };noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError}
 goal,err:=repository.repo.GoalCreate(model.GoalCreateInput{Title:read("title"),Description:read("description"),Prompt:read("prompt"),DueDate:read("due_date"),LLM:read("llm"),Client:read("client"),Parent:read("parent"),Milestone:read("milestone"),NoManagement:noManagement});return mcpDomainResult(goal,err)
}

// 🔹️goalClose holds the data fields for a goalClose record.
func (repository *mcpRepository) goalClose(ctx context.Context,request CallToolRequest) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request);id,err:=requireStringArg(args,"id");if err!=nil { return nil,err };summary,err:=requireStringArg(args,"summary");if err!=nil { return nil,err };noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError}
 goal,err:=repository.repo.GoalClose(model.GoalCloseInput{ID:id,Summary:summary,NoManagement:noManagement});return mcpDomainResult(goal,err)
}

func (repository *mcpRepository) goalReopen(ctx context.Context,request CallToolRequest) (*CallToolResult,error) {
 if err:=ctx.Err();err!=nil { return nil,err };args:=getArgs(request);read:=func(name string) string { value,_,_:=getStringArg(args,name);return value };optional:=func(name string) *string { value,present,_:=getStringArg(args,name);if !present { return nil };return &value };noManagement,_,argumentError:=getBoolArg(args,"no_management");if argumentError!=nil {return nil,argumentError}
 goal,err:=repository.repo.GoalReopen(model.GoalReopenInput{ID:read("id"),Prompt:read("prompt"),LLM:read("llm"),Client:read("client"),Title:optional("title"),Description:optional("description"),DueDate:optional("due_date"),NoManagement:noManagement});return mcpDomainResult(goal,err)
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

// #endregion 🪄️Handlers

// #endregion 🔌️Mcp Surface

// 🧾️mcpDomainResult renders the actual domain outcome without a human text conversion.
func mcpDomainResult(value interface{},err error) (*CallToolResult,error) { if err!=nil { return nil,err };payload,err:=json.Marshal(value);if err!=nil { return nil,err };return textResult(string(payload)),nil }

// 🎫️resolveTicket chooses only from this server's retained repository service.
func (repository *mcpRepository) resolveTicket(path string,status model.TicketStatus) (int,int,int,string,error) {
 if path!="" { return parseTicketPath(path) }
 tickets,err:=repository.repo.GetTickets(nil,nil,nil,&status);if err!=nil { return 0,0,0,"",err };if len(tickets)==0 { return 0,0,0,"",fmt.Errorf("no %s tickets in this repository",status) }
 sort.Slice(tickets,func(i,j int) bool { a,b:=tickets[i],tickets[j];if a.Year!=b.Year { return a.Year>b.Year };if a.Month!=b.Month { return a.Month>b.Month };if a.Day!=b.Day { return a.Day>b.Day };return a.Slug>b.Slug });selected:=tickets[0];return selected.Year,selected.Month,selected.Day,selected.Slug,nil
}

// 🎫️mcpTicketResult returns the exact rooted ticket identity for subsequent protocol operations.
func mcpTicketResult(ticket *model.Ticket,err error) (*CallToolResult,error) { if err!=nil { return nil,err };if ticket==nil { return nil,fmt.Errorf("ticket outcome required") };return mcpDomainResult(struct { Path string `json:"path"`;Ticket *model.Ticket `json:"ticket"` }{fmt.Sprintf("%02d/%02d/%02d/%s",ticket.Year,ticket.Month,ticket.Day,ticket.Slug),ticket},nil) }
