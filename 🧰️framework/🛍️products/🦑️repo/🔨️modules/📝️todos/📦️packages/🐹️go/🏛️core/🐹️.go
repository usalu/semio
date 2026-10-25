package todos

import (
	errors "errors"

	model "github.com/usalu/semio/repo/model"
)

// #region 🧲️Todo Contracts

// ✅️Todo is the todo shape of 📐️model, reexported so a client needs one import.
type Todo = model.Todo

// 📍️Location is where a todo sits.
type Location = model.Location

// 🎨️Draft is a named set of files staged before a ticket exists.
type Draft = model.Draft

// 🆕️TodoCreateInput is the input a create takes.
type TodoCreateInput = model.TodoCreateInput

// ♻️TodoChangeInput is the input a change takes.
type TodoChangeInput = model.TodoChangeInput

// ❗️TodoError is everything the todos domain can refuse, as a class rather than a rendered string.
type TodoError struct {
	Kind    string
	Message string
}

// 🔤️Error renders the refusal.
func (err TodoError) Error() string { return err.Message }

// 🏷️Class returns the stable class of the refusal.
func (err TodoError) Class() string { return err.Kind }

// 🏷️ErrorClass returns the stable class of any refusal this domain produced.
func ErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var todoError TodoError
	if errors.As(err, &todoError) {
		return todoError.Kind
	}
	return "port"
}

// 🔍️errTodoNotFound refuses an identifier no todo carries.
func errTodoNotFound() error { return TodoError{Kind: "not-found", Message: "todo not found"} }

// 📍️errNoLocation refuses a todo nothing says the line of.
func errNoLocation() error { return TodoError{Kind: "no-location", Message: "todo has no location"} }

// 🚫️errInvalidParent refuses a parent that is neither a directory nor an existing file.
func errInvalidParent() error {
	return TodoError{Kind: "invalid-parent", Message: "invalid parent id (must be path to folder or file)"}
}

// 📛️errDraftExists refuses an identifier a draft already took.
func errDraftExists(id string) error {
	return TodoError{Kind: "draft-exists", Message: "draft already exists: " + id}
}

// 🔤️errInvalidTitle refuses a draft title that carries no slug.
func errInvalidTitle() error { return TodoError{Kind: "invalid-title", Message: "invalid draft title"} }

// 🗄️errTodoPort refuses when the tree or a port refused.
func errTodoPort(message string) error { return TodoError{Kind: "port", Message: message} }

// #endregion 🧲️Todo Contracts
