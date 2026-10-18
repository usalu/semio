// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗒️draft_todo carries the draft and todo shapes and the tool kinds.

// #endregion 🧲️Header

package model

import (
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗒️Draft Todo

// 💿️Draft holds the data fields for a draft record.
type Draft struct {
	ID string `json:"id"`
}

// 🏪️GetID MUST return the stored value without modification.
// 📖️GetID returns the i d of the Draft.
func (d *Draft) GetID() string {
	return EmojiText(EmojiDraft) + workspace.Flat(d.ID)
}

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Draft.
func (d *Draft) GetURI() string {
	return "repo://draft/" + d.GetID()
}

// 🏷️ToolKind represents a categorized tool execution kind.
type ToolKind string

const ToolKindGeneric ToolKind = "generic"

const ToolKindPlan ToolKind = "plan"

const ToolKindCodeSearch ToolKind = "code-search"

const ToolKindCodeEdit ToolKind = "code-edit"

const ToolKindTest ToolKind = "test"

const ToolKindBuild ToolKind = "build"

const ToolKindTerminal ToolKind = "terminal"

// 🆕️TodoCreateInput holds the data fields for a todo create input record.
type TodoCreateInput struct {
	ParentID    string `json:"parentId"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

// ♻️TodoChangeInput holds the data fields for a todo change input record.
type TodoChangeInput struct {
	ID          string  `json:"id"`
	Name        *string `json:"name"`
	Description *string `json:"description"`
}

// ✅️Todo holds the data fields for a todo record.
type Todo struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description,omitempty"`
	ParentID    string    `json:"parentId"`
	Location    *Location `json:"location,omitempty"`
}

// 🌿️IsNode MUST return true only when the condition is met.
// ❓️IsNode reports whether the Todo is node.
func (t *Todo) IsNode() {}

// 🏪️GetID MUST return the stored value without modification.
// 📖️GetID returns the i d of the Todo.
func (t *Todo) GetID() string { return EmojiText(EmojiTodo) + workspace.Flat(t.ID) }

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Todo.
func (t *Todo) GetURI() string {
	return "repo://todo/" + t.GetID()
}

// #endregion 🗒️Draft Todo
