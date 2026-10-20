// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Scope, todo, breach, and ticket metric types.

// #endregion 🧲️Header

package client

import (
	"fmt"
)

// 🏷️ToolKind represents a categorized tool execution kind.
type ToolKind string

const (
	ToolKindGeneric    ToolKind = "generic"
	ToolKindPlan       ToolKind = "plan"
	ToolKindCodeSearch ToolKind = "code-search"
	ToolKindCodeEdit   ToolKind = "code-edit"
	ToolKindTest       ToolKind = "test"
	ToolKindBuild      ToolKind = "build"
	ToolKindTerminal   ToolKind = "terminal"
)

// 🔭️ScopeKind represents a scope kind value.
type ScopeKind string

const (
	ScopeRepo       ScopeKind = "repo"
	ScopeTechnology ScopeKind = "bundle"
	ScopeFolder     ScopeKind = "folder"
	ScopeFile       ScopeKind = "file"
	ScopeSection    ScopeKind = "section"
	ScopeDefinition ScopeKind = "definition"
)

// 💿️Scope holds the data fields for a scope record.
type Scope struct {
	Raw            string    `json:"raw"`
	Kind           ScopeKind `json:"kind"`
	TechnologyName string    `json:"technologyName,omitempty"`
	FilePath       string    `json:"filePath,omitempty"`
	SectionPath    []string  `json:"sectionPath,omitempty"`
	DefinitionName string    `json:"definitionName,omitempty"`
}

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
func (t *Todo) GetID() string { return emojiText(EmojiTodo) + Flat(t.ID) }

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Todo.
func (t *Todo) GetURI() string {
	return "repo://todo/" + t.GetID()
}

// 🔷️Location holds the data fields for a location record.
type Location struct {
	FilePath string `json:"filePath"`
	Line     int    `json:"line"`
	Column   int    `json:"column"`
}

// 🔶️Breach holds the data fields for a breach record.
type Breach struct {
	ID      string  `json:"id"`
	Summary string  `json:"summary"`
	Kind    Statute `json:"kind"`
	Scope   string  `json:"scope"`
	Line    int     `json:"line,omitempty"`
	Column  int     `json:"column,omitempty"`
	Excerpt string  `json:"excerpt,omitempty"`
	// 🧹️Optional fields from lint script JSON (`.🧬semio/🦑️repo/⚡️cache/breach/*.json`).
	LintPriority    BreachPriority `json:"priority,omitempty"`
	LintAutofixable *bool          `json:"autofixable,omitempty"`
	Reason          string         `json:"reason,omitempty"`
	Solution        string         `json:"solution,omitempty"`
}

// 🔹️IsNode MUST return true only when the condition is met.
// ⚠️IsNode reports whether the Breach is node.
func (v *Breach) IsNode() {}

// 🔸️GetID MUST return the stored value without modification.
// 🔑️GetID returns the i d of the Breach.
func (v *Breach) GetID() string { return fmt.Sprintf("%s%s", emojiText(EmojiBreach), v.ID) }

// 🔺️GetURI MUST return the stored value without modification.
// 🔹️GetURI returns the u r i of the Breach.
func (v *Breach) GetURI() string {
	return "repo://breach/" + v.GetID()
}

// 🔻️Priority MUST derive the value from the statute metadata.
// 📰️Priority returns the priority of the breach from its kind metadata.
func (v *Breach) Priority() BreachPriority {
	if v.LintPriority != "" {
		return v.LintPriority
	}
	return v.Kind.Info().Priority
}

// 🔧️Autofixable MUST return true only for statutes that support auto-fix.
// 📜️Autofixable reports whether the statute supports automatic fixing.
func (v *Breach) Autofixable() bool {
	if v.LintAutofixable != nil {
		return *v.LintAutofixable
	}
	return v.Kind.Info().Autofixable
}

// 🎫️TicketFileMetrics holds the data fields for a ticket file metrics record.
type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

// 📦️TicketBundleMetrics holds the data fields for a ticket bundle metrics record.
type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

// ⬛️TicketBundles represents a ticket bundles value.
type TicketBundles map[string]TicketBundleMetrics
