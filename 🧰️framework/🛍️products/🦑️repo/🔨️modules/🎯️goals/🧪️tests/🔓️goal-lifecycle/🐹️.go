// 🐹️ Go side of the goal lifecycle case.
package adapter

import (
	"encoding/json"
	"fmt"

	goals "github.com/usalu/semio/repo/goals"
	host "semio.tech/repo/test"
)

// region 🔖️Replay

type lifecycleStep struct {
	Op    string          `json:"op"`
	Input json.RawMessage `json:"input"`
}

type lifecycleScript struct {
	Schema      string          `json:"schema"`
	Author      string          `json:"author"`
	RepoURL     string          `json:"repoUrl"`
	FirstNumber int64           `json:"firstNumber"`
	Steps       []lifecycleStep `json:"steps"`
}

type stepInput struct {
	ID           string  `json:"id"`
	Title        *string `json:"title"`
	Description  *string `json:"description"`
	Prompt       string  `json:"prompt"`
	DueDate      *string `json:"dueDate"`
	LLM          *string `json:"llm"`
	Effort       *string `json:"effort"`
	Client       string  `json:"client"`
	Parent       *string `json:"parent"`
	Milestone    string  `json:"milestone"`
	Summary      string  `json:"summary"`
	NoManagement bool    `json:"noManagement"`
}

func value(pointer *string) string {
	if pointer == nil {
		return ""
	}
	return *pointer
}

func replay(ctx *host.Context, forceNoManagement bool) ([]string, []string, []string, []string, error) {
	data, err := ctx.FixtureBytes("shared://🔓️lifecycle-script.json")
	if err != nil {
		return nil, nil, nil, nil, err
	}
	var script lifecycleScript
	if err := json.Unmarshal(data, &script); err != nil {
		return nil, nil, nil, nil, err
	}
	store := goals.NewMemoryGoalStore()
	management := goals.NewRecordingManagement(script.RepoURL, script.FirstNumber)
	emitter := goals.NewMemoryEmitter()
	aggregate := goals.NewGoals(store, management, emitter, script.Author)

	outcomes := make([]string, 0, len(script.Steps))
	for _, step := range script.Steps {
		var input stepInput
		if err := json.Unmarshal(step.Input, &input); err != nil {
			return nil, nil, nil, nil, err
		}
		noManagement := forceNoManagement || input.NoManagement
		var line string
		switch step.Op {
		case "create":
			goal, err := aggregate.Create(goals.GoalCreateInput{
				Title: value(input.Title), Description: value(input.Description), Prompt: input.Prompt,
				DueDate: value(input.DueDate), LLM: value(input.LLM), Effort: value(input.Effort),
				Client: input.Client, NoManagement: noManagement, Parent: value(input.Parent), Milestone: input.Milestone,
			})
			if err != nil {
				line = fmt.Sprintf("create err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("create ok %s %s", goal.ID, goal.Status)
			}
		case "change":
			goal, err := aggregate.Change(goals.GoalChangeInput{
				ID: input.ID, Title: input.Title, Description: input.Description, DueDate: input.DueDate,
				LLM: input.LLM, Effort: input.Effort, Parent: input.Parent, NoManagement: noManagement,
			})
			if err != nil {
				line = fmt.Sprintf("change err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("change ok %s %s", goal.ID, goal.Title)
			}
		case "close":
			goal, err := aggregate.Close(goals.GoalCloseInput{ID: input.ID, Summary: input.Summary, NoManagement: noManagement})
			if err != nil {
				line = fmt.Sprintf("close err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("close ok %s %s", goal.ID, goal.Status)
			}
		case "reopen":
			goal, err := aggregate.Reopen(goals.GoalReopenInput{
				ID: input.ID, Prompt: input.Prompt, Client: input.Client, LLM: value(input.LLM), Effort: value(input.Effort),
				Title: input.Title, Description: input.Description, DueDate: input.DueDate, Parent: input.Parent, NoManagement: noManagement,
			})
			if err != nil {
				line = fmt.Sprintf("reopen err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("reopen ok %s %s %s", goal.ID, goal.Status, goal.Effort)
			}
		case "delete":
			removed, err := aggregate.Delete(goals.GoalDeleteInput{ID: input.ID, NoManagement: noManagement})
			if err != nil {
				line = fmt.Sprintf("delete err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("delete ok %t", removed)
			}
		case "read":
			goal, err := aggregate.Read(input.ID)
			if err != nil {
				line = fmt.Sprintf("read err %s", goals.ErrorClass(err))
			} else {
				line = fmt.Sprintf("read ok %s %s %s", goal.ID, goal.Status, goal.Title)
			}
		default:
			line = fmt.Sprintf("unknown op %s", step.Op)
		}
		outcomes = append(outcomes, line)
	}
	documents := make([]string, 0)
	for _, entry := range store.Snapshot() {
		documents = append(documents, fmt.Sprintf("%s=%s", entry.ID, entry.Document))
	}
	return outcomes, documents, management.Calls(), emitter.Envelopes(), nil
}

// endregion 🔖️Replay

// region 🔖️Scenarios

func theScriptProducesOneHistory(ctx *host.Context) (host.Outcome, error) {
	outcomes, documents, calls, envelopes, err := replay(ctx, false)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"outcomes":  outcomes,
		"documents": documents,
		"calls":     calls,
		"envelopes": envelopes,
	}}, nil
}

func managementCanBeSwitchedOff(ctx *host.Context) (host.Outcome, error) {
	outcomes, documents, calls, _, err := replay(ctx, true)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"outcomes": outcomes, "documents": documents, "calls": calls}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-script-produces-one-history", theScriptProducesOneHistory).
		Subject("management-can-be-switched-off", managementCanBeSwitchedOff)
}

// endregion 🔖️Registration
