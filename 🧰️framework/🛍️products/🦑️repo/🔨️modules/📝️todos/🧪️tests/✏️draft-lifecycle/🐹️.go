// 🐹️ Go side of the draft lifecycle case.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	todos "github.com/usalu/semio/repo/todos"
	host "semio.tech/repo/test"
)

// region 🔖️Replay

type draftFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type draftInput struct {
	ID    string      `json:"id"`
	Title string      `json:"title"`
	Files []draftFile `json:"files"`
}

type draftStep struct {
	Op    string     `json:"op"`
	Input draftInput `json:"input"`
}

type draftScript struct {
	Schema string      `json:"schema"`
	Steps  []draftStep `json:"steps"`
}

// endregion 🔖️Replay

// region 🔖️Scenarios

func theDraftScriptProducesOneHistory(ctx *host.Context) (host.Outcome, error) {
	data, err := ctx.FixtureBytes("shared://✏️draft-script.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var script draftScript
	if err := json.Unmarshal(data, &script); err != nil {
		return host.Outcome{}, err
	}
	store := todos.NewMemoryDraftStore()

	outcomes := make([]string, 0, len(script.Steps))
	for _, step := range script.Steps {
		var line string
		switch step.Op {
		case "list":
			ids := make([]string, 0)
			for _, draft := range todos.ListDrafts(store) {
				ids = append(ids, draft.ID)
			}
			line = fmt.Sprintf("list ok %s", strings.Join(ids, ","))
		case "create":
			files := make([]todos.TreeFile, 0, len(step.Input.Files))
			for _, entry := range step.Input.Files {
				files = append(files, todos.TreeFile{Path: entry.Path, Content: entry.Content})
			}
			draft, err := todos.CreateDraft(store, step.Input.Title, files)
			if err != nil {
				line = fmt.Sprintf("create err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("create ok %s %s %s", draft.ID, todos.DraftID(draft), todos.DraftURI(draft))
			}
		case "delete":
			if err := todos.DeleteDraft(store, step.Input.ID); err != nil {
				line = fmt.Sprintf("delete err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("delete ok %t", store.Exists(step.Input.ID))
			}
		default:
			line = fmt.Sprintf("unknown op %s", step.Op)
		}
		outcomes = append(outcomes, line)
	}

	files := make([]string, 0)
	for _, id := range store.IDs() {
		for _, entry := range store.Files(id) {
			files = append(files, fmt.Sprintf("%s/%s=%s", id, entry.Path, entry.Content))
		}
	}
	return host.Outcome{Projection: map[string]any{"outcomes": outcomes, "files": files}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-draft-script-produces-one-history", theDraftScriptProducesOneHistory)
}

// endregion 🔖️Registration
