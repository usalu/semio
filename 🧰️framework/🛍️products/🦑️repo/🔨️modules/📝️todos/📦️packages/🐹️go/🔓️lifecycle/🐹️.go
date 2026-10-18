package todos

import (
	fmt "fmt"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
)

// #region 🔓️Lifecycle

// 🎫️TicketOpener turns a promoted todo into a ticket and returns the ticket identifier.
type TicketOpener interface {
	// 🆕️Open opens a ticket with a title and a prompt.
	Open(title, prompt string) (string, error)
}

// 🧠️RecordingTicketOpener hands out deterministic identifiers and keeps every request.
type RecordingTicketOpener struct {
	opened []string
}

// 🆕️NewRecordingTicketOpener builds an opener that has opened nothing.
func NewRecordingTicketOpener() *RecordingTicketOpener { return &RecordingTicketOpener{} }

// 📜️Opened returns every title and prompt opened so far, in order.
func (opener *RecordingTicketOpener) Opened() []string {
	return append([]string{}, opener.opened...)
}

// 🆕️Open records the request and hands out the next deterministic identifier.
func (opener *RecordingTicketOpener) Open(title, prompt string) (string, error) {
	opener.opened = append(opener.opened, title+"\t"+prompt)
	return fmt.Sprintf("ticket-%d", len(opener.opened)), nil
}

// 📝️Todos is the todo aggregate bound to its ports.
type Todos struct {
	tree    TodoTree
	emitter events.Emitter
	author  string
}

// 🆕️NewTodos binds the aggregate to a tree, an emitter and an author alias.
func NewTodos(tree TodoTree, emitter events.Emitter, author string) *Todos {
	return &Todos{tree: tree, emitter: emitter, author: author}
}

// 📋️List returns every todo the tree carries.
func (aggregate *Todos) List() []*model.Todo { return ScanTodos(aggregate.tree) }

// 🔍️Find returns the first todo carrying an identifier.
func (aggregate *Todos) Find(id string) (*model.Todo, error) {
	for _, todo := range aggregate.List() {
		if todo.ID == id {
			return todo, nil
		}
	}
	return nil, errTodoNotFound()
}

// 🆕️Create writes a todo: an item appended to the parent directory's own todo markdown, or a
// comment appended to the parent file in that file's comment style.
func (aggregate *Todos) Create(input model.TodoCreateInput) (*model.Todo, error) {
	var todo *model.Todo
	switch {
	case aggregate.tree.IsDir(input.ParentID):
		path := joinTodoPath(input.ParentID, TodoMarkdownName)
		if err := appendToTree(aggregate.tree, path, TodoMarkdownPrefix+input.Name+": "+input.Description+"\n"); err != nil {
			return nil, err
		}
		todo = &model.Todo{ID: identity.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
	case aggregate.tree.Exists(input.ParentID):
		opener := TodoCommentOpener(input.ParentID)
		if err := appendToTree(aggregate.tree, input.ParentID, "\n"+opener+" TODO "+input.Name+": "+input.Description+"\n"); err != nil {
			return nil, err
		}
		todo = &model.Todo{
			ID: identity.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID,
			Location: &model.Location{FilePath: input.ParentID},
		}
	default:
		return nil, errInvalidParent()
	}
	aggregate.emit(events.EventTodoCreateEnded, events.TodoPayload{ID: todo.ID, ParentID: todo.ParentID, Name: todo.Name, Author: aggregate.author})
	return todo, nil
}

// ♻️Change rewrites the source line a todo is, in place.
func (aggregate *Todos) Change(input model.TodoChangeInput) (*model.Todo, error) {
	todo, err := aggregate.Find(input.ID)
	if err != nil {
		return nil, err
	}
	if todo.Location == nil {
		return nil, errNoLocation()
	}
	location := *todo.Location
	name, description := todo.Name, todo.Description
	if input.Name != nil {
		name = *input.Name
	}
	if input.Description != nil {
		description = *input.Description
	}
	document, ok := aggregate.tree.Read(location.FilePath)
	if !ok {
		return nil, errTodoNotFound()
	}
	var rewritten string
	switch {
	case strings.HasSuffix(location.FilePath, TodoMarkdownName):
		rewritten, err = ReplaceInMarkdown(document, todo.Name, name, description)
	case location.Line > 0:
		rewritten, err = ReplaceInFile(document, int64(location.Line), name, description)
	default:
		return nil, errNoLocation()
	}
	if err != nil {
		return nil, err
	}
	if err := aggregate.tree.Write(location.FilePath, rewritten); err != nil {
		return nil, err
	}
	updated := &model.Todo{ID: identity.Slugify(name), Name: name, Description: description, ParentID: todo.ParentID, Location: &location}
	aggregate.emit(events.EventTodoChangeEnded, events.TodoChangePayload{
		TodoPayload: events.TodoPayload{ID: updated.ID, ParentID: todo.ParentID, Author: aggregate.author},
		Name:        input.Name,
		Description: input.Description,
	})
	return updated, nil
}

// 🗑️Delete removes the source line a todo is.
func (aggregate *Todos) Delete(id string) (bool, error) {
	todo, err := aggregate.Find(id)
	if err != nil {
		return false, err
	}
	if todo.Location == nil {
		return false, errNoLocation()
	}
	document, ok := aggregate.tree.Read(todo.Location.FilePath)
	if !ok {
		return false, errTodoNotFound()
	}
	var rewritten string
	switch {
	case strings.HasSuffix(todo.Location.FilePath, TodoMarkdownName):
		rewritten = RemoveFromMarkdown(document, todo.Name)
	case todo.Location.Line > 0:
		rewritten = RemoveFromFile(document, int64(todo.Location.Line))
	default:
		return false, errNoLocation()
	}
	if err := aggregate.tree.Write(todo.Location.FilePath, rewritten); err != nil {
		return false, err
	}
	aggregate.emit(events.EventTodoDeleteEnded, events.TodoPayload{ID: todo.ID, ParentID: todo.ParentID, Name: todo.Name, Author: aggregate.author})
	return true, nil
}

// 🎫️ToTicket promotes a todo into a ticket and removes the todo once the ticket exists.
func (aggregate *Todos) ToTicket(id string, opener TicketOpener, title, prompt string) (string, error) {
	todo, err := aggregate.Find(id)
	if err != nil {
		return "", err
	}
	ticket, err := opener.Open(title, todo.Description+"\n\n"+prompt)
	if err != nil {
		return "", err
	}
	if _, err := aggregate.Delete(id); err != nil {
		return "", err
	}
	return ticket, nil
}

// 📤️emit sends one envelope to the bound emitter.
func (aggregate *Todos) emit(kind events.EventKind, payload interface{}) {
	if aggregate.emitter == nil {
		return
	}
	aggregate.emitter.Emit(kind, "repo-cli", payload)
}

// #endregion 🔓️Lifecycle
