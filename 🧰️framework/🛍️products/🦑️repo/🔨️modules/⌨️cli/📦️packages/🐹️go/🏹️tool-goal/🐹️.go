// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏹️tool_goal carries the goal and contributor tool functions of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	sort "sort"
	strings "strings"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	eventspkg "github.com/usalu/semio/repo/events"
	goalspkg "github.com/usalu/semio/repo/goals"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📋️Tickets

// 🔖️ToolGoalCreate MUST complete the operation successfully.
func ToolGoalCreate(title, description, prompt, dueDate, llm, client string, noManagement bool, parent, milestone string) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.GetRootDir())
	goal, err := ctx.GoalCreate(model.GoalCreateInput{
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
		return workspace.ToolErrorResult(err)
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
// Lists goals from `.🦑️repo/🎯️goals` only — avoids BuildMonorepoTree (full repo walk) while
// keeping the same markdown lines as goal nodes in the monorepo tree.
func ToolGoalList() workspace.ToolResult {
	goals, err := goalspkg.ListGoals()
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	output := workspace.NewOutput()
	var sb strings.Builder
	for _, g := range goals {
		goalCreatedAt := ""
		sb.WriteString(model.RenderEntityMarkdown("goal", map[string]interface{}{
			"id":          g.ID,
			"title":       g.Title,
			"status":      g.Status,
			"dueDate":     g.DueDate,
			"createdAt":   goalCreatedAt,
			"description": g.Description,
		}) + "\n")
	}
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: goals}
}

// 🔖️ToolGoalClose MUST complete the operation successfully.
func ToolGoalClose(id, summary string, noManagement bool) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.GetRootDir())
	res, err := ctx.GoalClose(model.GoalCloseInput{
		ID:           id,
		Summary:      summary,
		NoManagement: noManagement,
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
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
func ToolGoalReopen(id, prompt, llm, client, title, description, dueDate string, noManagement bool) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.GetRootDir())
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
	res, err := ctx.GoalReopen(model.GoalReopenInput{
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
		return workspace.ToolErrorResult(err)
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
func ToolContributorAdd(github string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventContributorAddStarting, "repo-cli", eventspkg.ContributorPayload{Github: github})
	contributor, err := contributorspkg.CreateContributor(github)
	if err != nil {
		return workspace.ToolErrorResult(err)
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
func ToolContributorList() workspace.ToolResult {
	contributors := contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(workspace.GetRootDir()))
	var events []Event
	for _, c := range contributors {
		data, _ := json.Marshal(map[string]interface{}{"contributor": c})
		events = append(events, Event{Kind: KindResult, Command: "contributor list", Data: data})
	}
	return toolResultFromEvents(events, contributors)
}

// ➖️ToolContributorRemove MUST complete the operation successfully.
func ToolContributorRemove(github string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventContributorRemoveStarting, "repo-cli", eventspkg.ContributorPayload{Github: github})
	if err := contributorspkg.RemoveContributor(github); err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"contributorRemove": github,
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// #endregion 📋️Tickets
