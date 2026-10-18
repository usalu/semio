// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️policy carries the policy, breach, territory and analysis shapes.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🛡️Policy

// 🔸️BreachPriority represents a breach priority value.
type BreachPriority string

const BreachPriorityHigh BreachPriority = "high"

const BreachPriorityMedium BreachPriority = "medium"

const BreachPriorityLow BreachPriority = "low"

// 🔺️IsValid MUST return true only when the condition is met.
// ⚠️IsValid reports whether the BreachPriority is valid.
func (e BreachPriority) IsValid() bool {
	switch e {
	case BreachPriorityHigh, BreachPriorityMedium, BreachPriorityLow:
		return true
	}
	return false
}

// 🔻️String MUST return the canonical string value.
// 🔷️String returns the string representation of the BreachPriority.
func (e BreachPriority) String() string {
	return string(e)
}

// 🔖️Policy holds the data fields for a policy record.
type Policy struct {
	ID          string         `json:"id"`
	Name        string         `json:"name"`
	Description *string        `json:"description,omitempty"`
	Scopes      []string       `json:"scopes"`
	Groups      []Territory    `json:"groups"`
	Statutes    []*StatuteMeta `json:"statutes"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🍀️IsNode reports whether the Policy is node.
func (p *Policy) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🪴️GetID returns the i d of the Policy.
func (p *Policy) GetID() string {
	slug := strings.TrimPrefix(p.ID, "/")
	return EmojiText(EmojiPolicy) + workspace.Flat(slug)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🌱️GetURI returns the u r i of the Policy.
func (p *Policy) GetURI() string {
	return "repo://policy/" + p.GetID()
}

// 🔖️StatuteMeta holds the data fields for a statute meta record.
type StatuteMeta struct {
	Kind        Statute        `json:"kind"`
	PolicyID    string         `json:"policyId"`
	Priority    BreachPriority `json:"priority"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
	Autofixable bool           `json:"autofixable"`
}

func NormalizeStatuteMeta(meta *StatuteMeta) {
	if meta == nil {
		return
	}
	if meta.Priority == "" {
		meta.Priority = BreachPriorityLow
	}
}

// 🔖️IsNode MUST return true only when the condition is met.
// 📰️IsNode reports whether the StatuteMeta is node.
func (v *StatuteMeta) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌲️GetID returns the i d of the StatuteMeta.
func (v *StatuteMeta) GetID() string {
	return fmt.Sprintf("%s%s", EmojiText(EmojiStatute), workspace.StatutePathToIdValue(string(v.Kind)))
}

// 🔖️GetURI MUST return the stored value without modification.
// 🔖️GetURI returns the u r i of the StatuteMeta.
func (v *StatuteMeta) GetURI() string {
	return "repo://statute/" + v.GetID()
}

// 🔖️AnalyzeResult holds the data fields for a analyze result record.
type AnalyzeResult struct {
	Breachs []*Breach       `json:"breachs"`
	Metrics *AnalyzeMetrics `json:"metrics"`
}

// 🔖️FixResult holds the data fields for a fix result record.
type FixResult struct {
	Fixed     int       `json:"fixed"`
	Remaining int       `json:"remaining"`
	Breachs   []*Breach `json:"breachs"`
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
	// Optional fields from lint script JSON (`.🦑️repo/⚡️cache/breach/*.json`).
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
func (v *Breach) GetID() string { return fmt.Sprintf("%s%s", EmojiText(EmojiBreach), v.ID) }

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

// 🟣️Territory holds the data fields for a statute group record.
type Territory struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	Scopes      []string    `json:"scopes,omitempty"`
	Groups      []Territory `json:"groups,omitempty"`
	Kinds       []Statute   `json:"kinds,omitempty"`
}

// 🟤️AllKinds MUST include all statutes from the group and its children.
// 🟦️AllKinds returns all statutes associated with the group.
func (g *Territory) AllKinds() []Statute {
	var result []Statute
	result = append(result, g.Kinds...)
	for _, child := range g.Groups {
		result = append(result, child.AllKinds()...)
	}
	return result
}

// ⚪️GetID MUST return the stored value without modification.
// 🗺️GetID returns the i d of the Territory.
func (g *Territory) GetID() string {
	return fmt.Sprintf("%s%s", EmojiText(EmojiTerritory), g.Name)
}

// ⚫️GetURI MUST return the stored value without modification.
// ❌️GetURI returns the u r i of the Territory.
func (g *Territory) GetURI() string {
	return "repo://territory/" + g.GetID()
}

// 🔹️BreachFile holds the data fields for a breach file record.
type BreachFile struct {
	ID    string     `json:"id"`
	Path  string     `json:"path"`
	URI   string     `json:"uri"`
	Range *FileRange `json:"range,omitempty"`
}

// 🔸️BreachFolder holds the data fields for a breach folder record.
type BreachFolder struct {
	ID   string `json:"id"`
	Path string `json:"path"`
	URI  string `json:"uri"`
}

// 🔺️CodebaseBreach holds the data fields for a codebase breach record.
type CodebaseBreach struct {
	ID          string         `json:"id"`
	Folders     []BreachFolder `json:"folders,omitempty"`
	Files       []BreachFile   `json:"files,omitempty"`
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
}

// 📜️PolicyBreachRef holds the data fields for a policy breach ref record.
type PolicyBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ◽CodebasePolicy holds the data fields for a codebase policy record.
type CodebasePolicy struct {
	ID      string            `json:"id"`
	Name    string            `json:"name"`
	Scopes  []string          `json:"scopes,omitempty"`
	Breachs []PolicyBreachRef `json:"breachs,omitempty"`
}

// 📜️policyAppliesToScope holds the data fields for a policyAppliesToScope record.
func policyAppliesToScope(policyID string, scope workspace.Scope) bool {
	switch policyID {
	case "code":
		return scope.Kind == workspace.ScopeFile && workspace.IsSourceFile(scope.FilePath)
	case "dev-docs":
		return scope.Kind == workspace.ScopeRepo || scope.Kind == workspace.ScopeFolder || scope.Kind == workspace.ScopeFile
	default:
		return true
	}
}

// #endregion 🛡️Policy
