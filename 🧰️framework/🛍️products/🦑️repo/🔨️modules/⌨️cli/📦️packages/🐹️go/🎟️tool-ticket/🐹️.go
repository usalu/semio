// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎟️tool_ticket carries the ticket and draft tool functions of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	fmt "fmt"
	strings "strings"
	time "time"

	eventspkg "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📋️Tickets

// 🔖️ToolTicketOpen MUST complete the operation successfully.
func ToolTicketOpen(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind providers.McpClientKind, planID, specID string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketOpenStarting, "repo-cli", eventspkg.TicketOpenPayload{
		Title: title, Prompt: prompt, LLM: llm, Effort: effort, Client: client, Goal: goal, Parent: parent,
	})
	ticket, err := ticketspkg.OpenTicket(emoji, title, prompt, llm, effort, client, draft, noIssue, goal, parent, noManagement, issue, mcpKind, planID, specID)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if ticket == nil {
		output := workspace.NewOutput()
		output.Info("Ticket creation skipped (NOTICKET)")
		return workspace.ToolResult{Output: *output}
	}
	ticketOpenPayload := map[string]interface{}{
		"id":     ticket.GetID(),
		"slug":   ticket.Slug,
		"year":   ticket.Year,
		"month":  ticket.Month,
		"day":    ticket.Day,
		"status": ticket.Status,
		"path":   ticket.FolderPath,
		"uri":    ticket.GetURI(),
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		ticketOpenPayload["github"] = map[string]interface{}{"issue": ticket.Management.Issue}
	}
	data, _ := json.Marshal(map[string]interface{}{"ticketOpen": ticketOpenPayload})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	result := toolResultFromEvents(events, ticket)
	if ticket.Management != nil && ticket.Management.Issue != "" {
		result.Output.Success(fmt.Sprintf("GitHub issue: %s", ticket.Management.Issue))
	}
	return result
}

// 🔖️ToolTicketList MUST complete the operation successfully.
// Always lists from ticket storage without building the full monorepo tree.
func ToolTicketList(year, month, day *int) workspace.ToolResult {
	tickets, err := ticketspkg.ListTickets(year, month, day)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	var sb strings.Builder
	for _, t := range tickets {
		created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+t.Year, t.Month, t.Day)
		if started := t.GetDateStarted(); !started.IsZero() {
			created = started.Format(time.RFC3339)
		}
		data := map[string]interface{}{
			"slug":      t.Slug,
			"year":      t.Year,
			"month":     t.Month,
			"day":       t.Day,
			"title":     t.Title,
			"status":    t.Status,
			"createdAt": created,
		}
		sb.WriteString(model.RenderEntityMarkdown("ticket", data) + "\n")
	}
	output := workspace.NewOutput()
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: tickets}
}

// 🔖️ToolTicketRead MUST complete the operation successfully.
func ToolTicketRead(year, month, day int, slug string) workspace.ToolResult {
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+ticket.Year, ticket.Month, ticket.Day)
	if started := ticket.GetDateStarted(); !started.IsZero() {
		created = started.Format(time.RFC3339)
	}
	dates := map[string]interface{}{"created": created}
	if finished := ticket.GetDateFinished(); finished != nil {
		dates["finished"] = finished.Format(time.RFC3339)
	}
	flat := map[string]interface{}{
		"slug":   ticket.Slug,
		"year":   ticket.Year,
		"month":  ticket.Month,
		"day":    ticket.Day,
		"title":  ticket.Title,
		"status": ticket.Status,
		"date":   dates,
	}
	data, _ := json.Marshal(map[string]interface{}{"ticket": flat})
	events := []Event{{Kind: KindResult, Command: "ticket read", Data: data}}
	return toolResultFromEvents(events, ticket)
}

// 🔖️ToolTicketClose MUST complete the operation successfully.
func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noManagement bool) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketCloseStarting, "repo-cli", eventspkg.TicketClosePayload{
		TicketPayload: eventspkg.TicketPayload{ID: model.FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Summary:       summary, Files: files,
	})
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, title); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := providers.GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.FinishTicket(ticket, summary, files, noManagement, false); err != nil {
		return workspace.ToolErrorResult(err)
	}
	created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+ticket.Year, ticket.Month, ticket.Day)
	if started := ticket.GetDateStarted(); !started.IsZero() {
		created = started.Format(time.RFC3339)
	}
	closeDates := map[string]interface{}{"created": created}
	if finished := ticket.GetDateFinished(); finished != nil {
		closeDates["finished"] = finished.Format(time.RFC3339)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"ticketClose": map[string]interface{}{
			"id":     ticket.GetID(),
			"slug":   ticket.Slug,
			"status": ticket.Status,
			"dates":  closeDates,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, ticket)
}

// 🔖️ToolTicketReopen MUST complete the operation successfully.
func ToolTicketReopen(year, month, day int, slug, prompt, llm, effort, client, draft string, title string, goal string, parent string, noManagement bool, mcpKind providers.McpClientKind, planID, specID string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketReopenStarting, "repo-cli", eventspkg.TicketReopenPayload{
		TicketPayload: eventspkg.TicketPayload{ID: model.FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Prompt:        prompt, LLM: llm, Effort: effort, Client: client,
	})
	output := workspace.NewOutput()
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, title); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := providers.GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.ReopenTicket(ticket, prompt, llm, effort, client, draft, goal, parent, noManagement, mcpKind, planID, specID); err != nil {
		return workspace.ToolErrorResult(err)
	}
	output.Success(fmt.Sprintf("\n🔓️ Ticket reopened: %s", ticket.Slug))
	return workspace.ToolResult{Output: *output, Data: ticket}
}

// 📝️ToolDraftCreate MUST complete the operation successfully.
func ToolDraftCreate(title string, paths []string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventDraftCreateStarting, "repo-cli", eventspkg.DraftPayload{
		Title: title,
	})
	output := workspace.NewOutput()
	files, err := todos.LoadTreeFiles(paths)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	draft, err := todos.CreateDraft(draftStore(), title, files)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventDraftCreateEnded, "repo-cli", eventspkg.DraftPayload{Slug: draft.ID, Title: title})
	output.Success(fmt.Sprintf("\n📝️Created draft: %s", draft.ID))
	return workspace.ToolResult{Output: *output, Data: draft}
}

// 🔖️ToolDraftList MUST complete the operation successfully.
func ToolDraftList() workspace.ToolResult {
	drafts := todos.ListDrafts(draftStore())
	var events []Event
	for _, d := range drafts {
		data, _ := json.Marshal(map[string]interface{}{"draft": d})
		events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
	}
	return toolResultFromEvents(events, drafts)
}

// 🔖️ToolDraftDelete MUST complete the operation successfully.
func ToolDraftDelete(slug string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventDraftDeleteStarting, "repo-cli", eventspkg.DraftPayload{Slug: slug})
	if err := todos.DeleteDraft(draftStore(), slug); err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventDraftDeleteEnded, "repo-cli", eventspkg.DraftPayload{Slug: slug})
	return toolResultFromEvents(nil, nil)
}

// #endregion 📋️Tickets
