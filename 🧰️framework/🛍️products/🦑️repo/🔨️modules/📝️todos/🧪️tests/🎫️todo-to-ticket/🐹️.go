// 🐹️ Go side of the todo lifecycle case.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	todos "github.com/usalu/semio/repo/todos"
	host "semio.tech/repo/test"
)

// region 🔖️Replay

type lifecycleInput struct {
	ID          string  `json:"id"`
	ParentID    string  `json:"parentId"`
	Name        *string `json:"name"`
	Description *string `json:"description"`
	Title       string  `json:"title"`
	Prompt      string  `json:"prompt"`
}

type lifecycleStep struct {
	Op    string         `json:"op"`
	Input lifecycleInput `json:"input"`
}

type lifecycleFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type lifecycleScript struct {
	Schema string          `json:"schema"`
	Author string          `json:"author"`
	Files  []lifecycleFile `json:"files"`
	Steps  []lifecycleStep `json:"steps"`
}

func text(pointer *string) string {
	if pointer == nil {
		return ""
	}
	return *pointer
}

// endregion 🔖️Replay

// region 🔖️Scenarios

func theScriptProducesOneHistory(ctx *host.Context) (host.Outcome, error) {
	data, err := ctx.FixtureBytes("shared://🔓️lifecycle-script.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var script lifecycleScript
	if err := json.Unmarshal(data, &script); err != nil {
		return host.Outcome{}, err
	}
	seeded := make([]todos.TreeFile, 0, len(script.Files))
	for _, entry := range script.Files {
		seeded = append(seeded, todos.TreeFile{Path: entry.Path, Content: entry.Content})
	}
	tree := todos.NewMemoryTodoTree(seeded)
	emitter := todos.NewMemoryEmitter()
	opener := todos.NewRecordingTicketOpener()
	aggregate := todos.NewTodos(tree, emitter, script.Author)

	outcomes := make([]string, 0, len(script.Steps))
	for _, step := range script.Steps {
		var line string
		switch step.Op {
		case "list":
			ids := make([]string, 0)
			for _, todo := range aggregate.List() {
				ids = append(ids, todo.ID)
			}
			line = fmt.Sprintf("list ok %s", strings.Join(ids, ","))
		case "find":
			todo, err := aggregate.Find(step.Input.ID)
			if err != nil {
				line = fmt.Sprintf("find err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("find ok %s %s %s", todo.ID, todo.Name, todo.Description)
			}
		case "create":
			todo, err := aggregate.Create(todos.TodoCreateInput{ParentID: step.Input.ParentID, Name: text(step.Input.Name), Description: text(step.Input.Description)})
			if err != nil {
				line = fmt.Sprintf("create err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("create ok %s %s", todo.ID, todo.ParentID)
			}
		case "change":
			todo, err := aggregate.Change(todos.TodoChangeInput{ID: step.Input.ID, Name: step.Input.Name, Description: step.Input.Description})
			if err != nil {
				line = fmt.Sprintf("change err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("change ok %s %s %s", todo.ID, todo.Name, todo.Description)
			}
		case "delete":
			removed, err := aggregate.Delete(step.Input.ID)
			if err != nil {
				line = fmt.Sprintf("delete err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("delete ok %t", removed)
			}
		case "to-ticket":
			ticket, err := aggregate.ToTicket(step.Input.ID, opener, step.Input.Title, step.Input.Prompt)
			if err != nil {
				line = fmt.Sprintf("to-ticket err %s", todos.ErrorClass(err))
			} else {
				line = fmt.Sprintf("to-ticket ok %s", ticket)
			}
		default:
			line = fmt.Sprintf("unknown op %s", step.Op)
		}
		outcomes = append(outcomes, line)
	}

	files := make([]string, 0)
	for _, entry := range tree.Snapshot() {
		files = append(files, fmt.Sprintf("%s=%s", entry.Path, entry.Content))
	}
	return host.Outcome{Projection: map[string]any{
		"outcomes":  outcomes,
		"files":     files,
		"tickets":   opener.Opened(),
		"envelopes": emitter.Envelopes(),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-script-produces-one-history", theScriptProducesOneHistory)
}

// endregion 🔖️Registration
