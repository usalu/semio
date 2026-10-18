// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📰️todos carries the todo queries and mutations of the repository context.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📰️Todos

// 📨️GetTodos MUST retrieve the requested value or return an error.
// 📖️GetTodos retrieves and returns the todos.
func (c *RepoContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	allTodos := c.scanTodos()
	if filter != nil && filter.Filter != nil {
		search := strings.ToLower(*filter.Filter)
		var match []*model.Todo
		for _, t := range allTodos {
			if strings.Contains(strings.ToLower(t.Name), search) || strings.Contains(strings.ToLower(t.Description), search) {
				match = append(match, t)
			}
		}
		return match, nil
	}
	return allTodos, nil
}

// 🆕️TodoCreate MUST return a non-nil error when the operation fails.
// 🆕️TodoCreate performs the todo create operation on the repo context.
func (c *RepoContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {

	info, err := os.Stat(input.ParentID)
	if err == nil && info.IsDir() {
		todoPath := filepath.Join(input.ParentID, ".todos.md")
		line := fmt.Sprintf("- TODO %s: %s\n", input.Name, input.Description)
		f, err := os.OpenFile(todoPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		todo := &model.Todo{ID: workspace.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
		events.Emit(events.EventTodoCreateEnded, "repo-cli", events.TodoCreatePayload{
			TodoPayload: events.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: contributorspkg.GetGitAuthorAlias()},
		})
		return todo, nil
	}

	if err == nil && !info.IsDir() {
		ext := filepath.Ext(input.ParentID)
		prefix := "//"
		if ext == ".py" || ext == ".sh" || ext == ".yaml" || ext == ".yml" {
			prefix = "#"
		} else if ext == ".sql" || ext == ".lua" {
			prefix = "--"
		}
		line := fmt.Sprintf("\n%s TODO %s: %s\n", prefix, input.Name, input.Description)
		f, err := os.OpenFile(input.ParentID, os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		todo := &model.Todo{ID: workspace.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID, Location: &model.Location{FilePath: input.ParentID}}
		events.Emit(events.EventTodoCreateEnded, "repo-cli", events.TodoCreatePayload{
			TodoPayload: events.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: contributorspkg.GetGitAuthorAlias()},
		})
		return todo, nil
	}
	return nil, fmt.Errorf("invalid parent id (must be path to folder or file)")
}

// ♻️TodoChange MUST return a non-nil error when the operation fails.
// ✏️TodoChange performs the todo change operation on the repo context.
func (c *RepoContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	todos := c.scanTodos()
	var todo *model.Todo
	for _, t := range todos {
		if t.ID == input.ID {
			todo = t
			break
		}
	}
	if todo == nil {
		return nil, fmt.Errorf("todo not found")
	}
	if todo.Location == nil {
		return nil, fmt.Errorf("todo has no location")
	}
	name := todo.Name
	if input.Name != nil {
		name = *input.Name
	}
	description := todo.Description
	if input.Description != nil {
		description = *input.Description
	}
	if strings.HasSuffix(todo.Location.FilePath, ".todos.md") {
		if err := todospkg.ReplaceLineInMarkdown(todo.Location.FilePath, todo.Name, name, description); err != nil {
			return nil, err
		}
	} else if todo.Location.Line > 0 {
		if err := todospkg.ReplaceLineInFile(todo.Location.FilePath, todo.Location.Line, name, description); err != nil {
			return nil, err
		}
	} else {
		return nil, fmt.Errorf("unable to locate todo source line")
	}
	updated := &model.Todo{ID: workspace.Slugify(name), Name: name, Description: description, ParentID: todo.ParentID, Location: todo.Location}
	events.Emit(events.EventTodoChangeEnded, "repo-cli", events.TodoChangePayload{
		TodoPayload: events.TodoPayload{ID: updated.ID, ParentID: todo.ParentID, Name: name, Author: contributorspkg.GetGitAuthorAlias()},
		Name:        input.Name, Description: input.Description,
	})
	return updated, nil
}

// 🗑️TodoDelete MUST return a non-nil error when the operation fails.
// 🗑️TodoDelete performs the todo delete operation on the repo context.
func (c *RepoContext) TodoDelete(id string) (bool, error) {
	todos := c.scanTodos()
	for _, t := range todos {
		if t.ID == id {
			if strings.HasSuffix(t.Location.FilePath, ".todos.md") {
				todospkg.RemoveLineFromMarkdown(t.Location.FilePath, t.Name)
				events.Emit(events.EventTodoDeleteEnded, "repo-cli", events.TodoDeletePayload{
					TodoPayload: events.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: contributorspkg.GetGitAuthorAlias()},
				})
				return true, nil
			}
			if t.Location.Line > 0 {
				todospkg.RemoveLineFromFile(t.Location.FilePath, t.Location.Line)
				events.Emit(events.EventTodoDeleteEnded, "repo-cli", events.TodoDeletePayload{
					TodoPayload: events.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: contributorspkg.GetGitAuthorAlias()},
				})
				return true, nil
			}
		}
	}
	return false, fmt.Errorf("todo not found")
}

// 🎫️TodoToTicket MUST return a non-nil error when the operation fails.
// 🔤️TodoToTicket performs the todo to ticket operation on the repo context.
func (c *RepoContext) TodoToTicket(id string, input model.TicketOpenInput) (*model.Ticket, error) {
	todos := c.scanTodos()
	var todo *model.Todo
	for _, t := range todos {
		if t.ID == id {
			todo = t
			break
		}
	}
	if todo == nil {
		return nil, fmt.Errorf("todo not found")
	}

	if input.Title == "" {
	}
	input.Prompt = fmt.Sprintf("%s\n\n%s", todo.Description, input.Prompt)

	ticket, err := c.TicketOpen(input)
	if err != nil {
		return nil, err
	}
	c.TodoDelete(id)
	return ticket, nil
}

// #endregion 📰️Todos
