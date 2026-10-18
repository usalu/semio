// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🥅️goal carries the goal shape with its accessors and JSON codec.

// #endregion 🧲️Header

package model

import (
	json "encoding/json"
	strings "strings"
)

// #region 🥅️Goal

// 💿️GoalNode holds the data fields for a goal node record.
type GoalNode struct {
	ID, Title, Status  string
	DueDate, CreatedAt string
	Description        string
	Children           []*GoalNode
	Tickets            []*TicketNode
}

// 🔵️Goal holds the data fields for a goal record.
type Goal struct {
	Title       string              `json:"title"`
	Description string              `json:"description"`
	Prompt      string              `json:"prompt"`
	Status      string              `json:"status"`
	Summary     string              `json:"summary,omitempty"`
	DueDate     string              `json:"dueDate,omitempty"`
	Dates       GoalDates           `json:"dates"`
	Client      string              `json:"client"`
	LLM         string              `json:"llm"`
	Effort      string              `json:"effort,omitempty"`
	Parent      string              `json:"parent,omitempty"`
	Management  *GoalManagementData `json:"github,omitempty"`

	ID   string `json:"-"`
	Path string `json:"-"`
}

// 😀️MarshalJSON converts internal filesystem paths to repo emoji IDs for Goal serialization.
func (g Goal) MarshalJSON() ([]byte, error) {
	type GoalAlias Goal
	alias := GoalAlias(g)

	if alias.Parent != "" && !strings.Contains(alias.Parent, EmojiText(EmojiGoal)) {
		alias.Parent = GoalPathToComposeID(alias.Parent)
	}
	return json.Marshal(alias)
}

// 🌿️IsNode MUST return true only when the condition is met.
// 🔖️IsNode reports whether the Goal is node.
func (g *Goal) IsNode() {}

// 🔴️GetID MUST return the stored value without modification.
// 🔑️GetID returns the i d of the Goal.
func (g *Goal) GetID() string { return goalArtifactID(g.ID) }

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Goal.
func (g *Goal) GetURI() string {
	return GetArtifactURI("goal", map[string]interface{}{"id": g.ID})
}

// 🟠️GoalDates holds the data fields for a goal dates record.
type GoalDates struct {
	Due string `json:"due,omitempty"`
}

// 🟡️GoalManagementData holds the data fields for a goal github data record.
type GoalManagementData struct {
	Milestone string `json:"milestone,omitempty"`
	Issue     string `json:"issue,omitempty"`
}

// #endregion 🥅️Goal
