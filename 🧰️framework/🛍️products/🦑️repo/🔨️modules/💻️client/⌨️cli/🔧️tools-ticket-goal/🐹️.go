// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket, draft, goal, contributor, and technology tools.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"sort"
	"strings"
	"time"
)

// 🔖️ToolTicketOpen MUST complete the operation successfully.
func ToolTicketOpen(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind McpClientKind, planID, specID string) ToolResult {
	repopkg.Emit(repopkg.EventTicketOpenStarting, "repo-cli", repopkg.TicketOpenPayload{
		Title: title, Prompt: prompt, LLM: llm, Effort: effort, Client: client, Goal: goal, Parent: parent,
	})
	ticket, err := OpenTicket(emoji, title, prompt, llm, effort, client, draft, noIssue, goal, parent, noManagement, issue, mcpKind, planID, specID)
	if err != nil {
		return toolErrorResult(err)
	}
	if ticket == nil {
		output := NewOutput()
		output.Info("Ticket creation skipped (NOTICKET)")
		return ToolResult{Output: *output}
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
func ToolTicketList(year, month, day *int) ToolResult {
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		return toolErrorResult(err)
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
		sb.WriteString(renderEntityMarkdown("ticket", data) + "\n")
	}
	output := NewOutput()
	output.Plain(sb.String())
	return ToolResult{Output: *output, Data: tickets}
}

// 🔖️ToolTicketRead MUST complete the operation successfully.
func ToolTicketRead(year, month, day int, slug string) ToolResult {
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		return toolErrorResult(err)
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
func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noManagement bool) ToolResult {
	repopkg.Emit(repopkg.EventTicketCloseStarting, "repo-cli", repopkg.TicketClosePayload{
		TicketPayload: repopkg.TicketPayload{ID: FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Summary:       summary, Files: files,
	})
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		return toolErrorResult(err)
	}
	if title != "" {
		if err := UpdateTicketTitle(ticket, title); err != nil {
			return toolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				writeWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := FinishTicket(ticket, summary, files, noManagement, false); err != nil {
		return toolErrorResult(err)
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
func ToolTicketReopen(year, month, day int, slug, prompt, llm, effort, client, draft string, title string, goal string, parent string, noManagement bool, mcpKind McpClientKind, planID, specID string) ToolResult {
	repopkg.Emit(repopkg.EventTicketReopenStarting, "repo-cli", repopkg.TicketReopenPayload{
		TicketPayload: repopkg.TicketPayload{ID: FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Prompt:        prompt, LLM: llm, Effort: effort, Client: client,
	})
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		return toolErrorResult(err)
	}
	if title != "" {
		if err := UpdateTicketTitle(ticket, title); err != nil {
			return toolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				writeWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ReopenTicket(ticket, prompt, llm, effort, client, draft, goal, parent, noManagement, mcpKind, planID, specID); err != nil {
		return toolErrorResult(err)
	}
	output.Success(fmt.Sprintf("\n🔓️ Ticket reopened: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

// 📝️ToolDraftCreate MUST complete the operation successfully.
func ToolDraftCreate(title string, files []string) ToolResult {
	repopkg.Emit(repopkg.EventDraftCreateStarting, "repo-cli", repopkg.DraftPayload{
		Title: title,
	})
	output := NewOutput()
	draft, err := CreateDraft(title, files)
	if err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventDraftCreateEnded, "repo-cli", repopkg.DraftPayload{Slug: draft.ID, Title: title})
	output.Success(fmt.Sprintf("\n📝️Created draft: %s", draft.ID))
	return ToolResult{Output: *output, Data: draft}
}

// 🔖️ToolDraftList MUST complete the operation successfully.
func ToolDraftList() ToolResult {
	drafts, err := ListDrafts()
	if err != nil {
		return toolErrorResult(err)
	}
	var events []Event
	for _, d := range drafts {
		data, _ := json.Marshal(map[string]interface{}{"draft": d})
		events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
	}
	return toolResultFromEvents(events, drafts)
}

// 🔖️ToolDraftDelete MUST complete the operation successfully.
func ToolDraftDelete(slug string) ToolResult {
	repopkg.Emit(repopkg.EventDraftDeleteStarting, "repo-cli", repopkg.DraftPayload{Slug: slug})
	if err := DeleteDraft(slug); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventDraftDeleteEnded, "repo-cli", repopkg.DraftPayload{Slug: slug})
	return toolResultFromEvents(nil, nil)
}

// 🔖️ToolGoalCreate MUST complete the operation successfully.
func ToolGoalCreate(title, description, prompt, dueDate, llm, client string, noManagement bool, parent, milestone string) ToolResult {
	ctx := NewRepoContext(rootDir)
	goal, err := ctx.GoalCreate(GoalCreateInput{
		Title:        title,
		Description:  description,
		Prompt:       prompt,
		DueDate:      dueDate,
		LLM:          llm,
		Client:       client,
		NoManagement: noManagement,
		Parent:       parent,
		Milestone:    milestone,
	})
	if err != nil {
		return toolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalCreate": map[string]interface{}{
			"id":      goal.ID,
			"title":   goal.Title,
			"status":  goal.Status,
			"prompt":  goal.Prompt,
			"dueDate": goal.Dates.Due,
			"client":  goal.Client,
			"llm":     goal.LLM,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, goal)
}

// 🔖️ToolGoalList MUST complete the operation successfully.
// Lists goals from `.🧬semio/🦑️repo/🎯️goals` only — avoids BuildMonorepoTree (full repo walk) while
// keeping the same markdown lines as goal nodes in the monorepo tree.
func ToolGoalList() ToolResult {
	goals, err := ListGoals()
	if err != nil {
		return toolErrorResult(err)
	}
	sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	output := NewOutput()
	var sb strings.Builder
	for _, g := range goals {
		goalCreatedAt := ""
		sb.WriteString(renderEntityMarkdown("goal", map[string]interface{}{
			"id":          g.ID,
			"title":       g.Title,
			"status":      g.Status,
			"dueDate":     g.DueDate,
			"createdAt":   goalCreatedAt,
			"description": g.Description,
		}) + "\n")
	}
	output.Plain(sb.String())
	return ToolResult{Output: *output, Data: goals}
}

// 🔖️ToolGoalClose MUST complete the operation successfully.
func ToolGoalClose(id, summary string, noManagement bool) ToolResult {
	ctx := NewRepoContext(rootDir)
	res, err := ctx.GoalClose(GoalCloseInput{
		ID:           id,
		Summary:      summary,
		NoManagement: noManagement,
	})
	if err != nil {
		return toolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalClose": map[string]interface{}{
			"id":     res.ID,
			"status": res.Status,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🔖️ToolGoalReopen MUST complete the operation successfully.
func ToolGoalReopen(id, prompt, llm, client, title, description, dueDate string, noManagement bool) ToolResult {
	ctx := NewRepoContext(rootDir)
	var titlePtr, descriptionPtr, dueDatePtr *string
	if title != "" {
		titlePtr = &title
	}
	if description != "" {
		descriptionPtr = &description
	}
	if dueDate != "" {
		dueDatePtr = &dueDate
	}
	res, err := ctx.GoalReopen(GoalReopenInput{
		ID:           id,
		Prompt:       prompt,
		LLM:          llm,
		Client:       client,
		Title:        titlePtr,
		Description:  descriptionPtr,
		DueDate:      dueDatePtr,
		NoManagement: noManagement,
	})
	if err != nil {
		return toolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalReopen": map[string]interface{}{
			"id":     res.ID,
			"status": res.Status,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🤝️ToolContributorAdd MUST complete the operation successfully.
func ToolContributorAdd(github string) ToolResult {
	repopkg.Emit(repopkg.EventContributorAddStarting, "repo-cli", repopkg.ContributorPayload{Github: github})
	contributor, err := CreateContributor(github)
	if err != nil {
		return toolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"contributorAdd": map[string]interface{}{
			"id":     contributor.Github,
			"github": contributor.Github,
			"name":   contributor.Name,
			"emails": contributor.Emails,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, contributor)
}

// 🔖️ToolContributorList MUST complete the operation successfully.
func ToolContributorList() ToolResult {
	contributors, err := ListContributors()
	if err != nil {
		return toolErrorResult(err)
	}
	var events []Event
	for _, c := range contributors {
		data, _ := json.Marshal(map[string]interface{}{"contributor": c})
		events = append(events, Event{Kind: KindResult, Command: "contributor list", Data: data})
	}
	return toolResultFromEvents(events, contributors)
}

// ➖️ToolContributorRemove MUST complete the operation successfully.
func ToolContributorRemove(github string) ToolResult {
	repopkg.Emit(repopkg.EventContributorRemoveStarting, "repo-cli", repopkg.ContributorPayload{Github: github})
	if err := RemoveContributor(github); err != nil {
		return toolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"contributorRemove": github,
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🔖️ToolTechnologyList MUST complete the operation successfully.
func ToolTechnologyList() ToolResult {
	technologies := LoadTechnologies()
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })
	var events []Event
	for _, p := range technologies {
		data, _ := json.Marshal(map[string]interface{}{"technology": p})
		events = append(events, Event{Kind: KindResult, Command: "technology list", Data: data})
	}
	return toolResultFromEvents(events, technologies)
}

// 🔖️ToolBundleList MUST complete the operation successfully.
func ToolBundleList() ToolResult {
	bundles := LoadBundles()
	sort.Slice(bundles, func(i, j int) bool { return bundles[i].Name < bundles[j].Name })
	var events []Event
	for _, b := range bundles {
		data, _ := json.Marshal(map[string]interface{}{"bundle": b})
		events = append(events, Event{Kind: KindResult, Command: "bundle list", Data: data})
	}
	return toolResultFromEvents(events, bundles)
}

// 🔖️ToolTechnologyTree MUST complete the operation successfully.
func ToolTechnologyTree() ToolResult {
	return toolResultFromTreeRender(TreeNodeTechnology)
}
