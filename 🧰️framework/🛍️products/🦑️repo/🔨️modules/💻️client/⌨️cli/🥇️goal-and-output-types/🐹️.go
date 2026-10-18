// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Goal, territory, policy definition, report, and output types.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"strings"
)

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

	if alias.Parent != "" && !strings.Contains(alias.Parent, emojiText(EmojiGoal)) {
		alias.Parent = goalPathToComposeID(alias.Parent)
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
	return fmt.Sprintf("%s%s", emojiText(EmojiTerritory), g.Name)
}

// ⚫️GetURI MUST return the stored value without modification.
// ❌️GetURI returns the u r i of the Territory.
func (g *Territory) GetURI() string {
	return "repo://territory/" + g.GetID()
}

// 🩵️PolicyDef holds the data fields for a policy def record.
type PolicyDef struct {
	ID          string         `json:"id"`
	Name        string         `json:"name"`
	Description string         `json:"description"`
	Scopes      []string       `json:"scopes"`
	Priority    BreachPriority `json:"priority"`
	Groups      []Territory    `json:"groups"`
	Run         PolicyFunc     `json:"-"`
}

// 🩶️AllKinds MUST include all statutes from the group and its children.
// 🟪️AllKinds returns all statutes associated with the group.
func (p *PolicyDef) AllKinds() []Statute {
	var result []Statute
	for _, g := range p.Groups {
		result = append(result, g.AllKinds()...)
	}
	return result
}

// 🩷️AnalyzeReport holds the data fields for a analyze report record.
type AnalyzeReport struct {
	Second  string   `json:"second"`
	Status  string   `json:"status"`
	Scope   string   `json:"scope"`
	Summary Summary  `json:"summary"`
	Breachs []Breach `json:"breachs"`
}

// 💜️Summary holds the data fields for a summary record.
type Summary struct {
	Total      int            `json:"total"`
	ByPriority map[string]int `json:"byPriority"`
	ByKind     map[string]int `json:"byKind"`
}

// 💙️FileCache holds the data fields for a file cache record.
type FileCache struct {
	FilePath string   `json:"filePath"`
	Hash     string   `json:"hash"`
	Second   string   `json:"second"`
	Breachs  []Breach `json:"breachs"`
}

// 💚️OutputType represents a output type value.
type OutputType string

const (
	OutputInfo    OutputType = "info"
	OutputSuccess OutputType = "success"
	OutputError   OutputType = "error"
	OutputWarn    OutputType = "warn"
	OutputPlain   OutputType = "plain"
)

// 💛️OutputLine holds the data fields for a output line record.
type OutputLine struct {
	Type OutputType `json:"type"`
	Text string     `json:"text"`
}

// 🧡️CommandOutput holds the data fields for a command output record.
type CommandOutput struct {
	Lines    []OutputLine `json:"lines"`
	ExitCode int          `json:"exitCode"`
}

// ❤️ToolResult holds the data fields for a tool result record.
type ToolResult struct {
	Output CommandOutput `json:"output"`
	Data   interface{}   `json:"data,omitempty"`
	Error  string        `json:"error,omitempty"`
}

// 🤍️ContributorTicket holds the data fields for a contributor ticket record.
type ContributorTicket struct {
	Year     int          `json:"year"`
	Month    int          `json:"month"`
	Day      int          `json:"day"`
	Slug     string       `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string       `json:"filePath,omitempty"`
}

type ContributorCheckpoint struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

// 🖤️ContributorContributionsStorage holds the data fields for a contributor contributions storage record.
type ContributorContributionsStorage struct {
	Bundles     []string                `json:"bundles,omitempty"`
	Folders     []string                `json:"folders,omitempty"`
	Files       []string                `json:"files,omitempty"`
	Regions     []string                `json:"regions,omitempty"`
	Definitions []string                `json:"definitions,omitempty"`
	Tickets     []ContributorTicket     `json:"tickets,omitempty"`
	Checkpoints []ContributorCheckpoint `json:"checkpoints,omitempty"`
	Lines       *LineMetrics            `json:"lines,omitempty"`
}
