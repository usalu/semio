// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Interaction models, listing, and ticket agent types.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// 📄️InteractionFile holds a file reference with path, id and uri.
type InteractionFile struct {
	Path string `json:"path" yaml:"path"`
	ID   string `json:"id,omitempty" yaml:"id,omitempty"`
	URI  string `json:"uri,omitempty" yaml:"uri,omitempty"`
}

// 🟫️Interaction holds the data fields for a interaction record.
type Interaction struct {
	Kind       string            `json:"kind" yaml:"kind"`
	Date       string            `json:"date" yaml:"date"`
	Author     string            `json:"author" yaml:"author"`
	System     string            `json:"system" yaml:"system"`
	Client     string            `json:"client" yaml:"client"`
	Checkpoint string            `json:"checkpoint" yaml:"checkpoint"`
	Prompt     string            `json:"prompt,omitempty" yaml:"prompt,omitempty"`
	Summary    string            `json:"summary,omitempty" yaml:"summary,omitempty"`
	LLM        string            `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort     string            `json:"effort,omitempty" yaml:"effort,omitempty"`
	Files      []InteractionFile `json:"files,omitempty" yaml:"files,omitempty"`
}

// 💾️CheckpointDiffRename holds a from/to pair for renamed entities.
type CheckpointDiffRename struct {
	From string `json:"from" yaml:"from"`
	To   string `json:"to" yaml:"to"`
}

// 🆕️CheckpointDiffStats holds deleted/renamed/modified/created lists for a diff category.
type CheckpointDiffStats struct {
	Deleted  []string               `json:"deleted,omitempty" yaml:"deleted,omitempty"`
	Renamed  []CheckpointDiffRename `json:"renamed,omitempty" yaml:"renamed,omitempty"`
	Modified []string               `json:"modified,omitempty" yaml:"modified,omitempty"`
	Created  []string               `json:"created,omitempty" yaml:"created,omitempty"`
}

// 📁️CheckpointDiff holds diff stats for technologies, bundles, folders, files, sections, and definitions.
type CheckpointDiff struct {
	Technologies CheckpointDiffStats `json:"technologies,omitempty" yaml:"technologies,omitempty"`
	Bundles      CheckpointDiffStats `json:"bundles,omitempty" yaml:"bundles,omitempty"`
	Folders      CheckpointDiffStats `json:"folders,omitempty" yaml:"folders,omitempty"`
	Files        CheckpointDiffStats `json:"files,omitempty" yaml:"files,omitempty"`
	Sections     CheckpointDiffStats `json:"sections,omitempty" yaml:"sections,omitempty"`
	Definitions  CheckpointDiffStats `json:"definitions,omitempty" yaml:"definitions,omitempty"`
}

// 🎫️TicketAgentPlanStep holds a single step in an agent plan with optional timestamps.
type TicketAgentPlanStep struct {
	ID          string `json:"id" yaml:"id"`
	Name        string `json:"name" yaml:"name"`
	Description string `json:"description,omitempty" yaml:"description,omitempty"`
	Status      string `json:"status,omitempty" yaml:"status,omitempty"`
	Ideated     string `json:"ideated,omitempty" yaml:"ideated,omitempty"`
	Started     string `json:"started,omitempty" yaml:"started,omitempty"`
	Completed   string `json:"completed,omitempty" yaml:"completed,omitempty"`
	Abandoned   string `json:"abandoned,omitempty" yaml:"abandoned,omitempty"`
}

// 💠️TicketAgentPlan holds the plan steps for an agent.
type TicketAgentPlan struct {
	Steps []TicketAgentPlanStep `json:"steps,omitempty" yaml:"steps,omitempty"`
}

// 🔳️TicketAgent holds an agent record with contributor and plan.
type TicketAgent struct {
	Session     string           `json:"session" yaml:"session"`
	Contributor string           `json:"contributor,omitempty" yaml:"contributor,omitempty"`
	System      string           `json:"system,omitempty" yaml:"system,omitempty"`
	Client      string           `json:"client,omitempty" yaml:"client,omitempty"`
	LLM         string           `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort      string           `json:"effort,omitempty" yaml:"effort,omitempty"`
	Transcript  string           `json:"transcript,omitempty" yaml:"transcript,omitempty"`
	Plan        *TicketAgentPlan `json:"plan,omitempty" yaml:"plan,omitempty"`
}

// 🔲️UnmarshalJSON MUST handle both legacy and current JSON layouts.
func (i *Interaction) UnmarshalJSON(data []byte) error {
	type Alias Interaction
	raw := struct {
		Alias
		RawAuthor json.RawMessage `json:"author"`
		RawSystem json.RawMessage `json:"system"`
		RawDates  json.RawMessage `json:"dates"`
	}{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	*i = Interaction(raw.Alias)
	if i.Date == "" && len(raw.RawDates) > 0 {
		var legacy struct {
			Created string `json:"created"`
		}
		if err := json.Unmarshal(raw.RawDates, &legacy); err == nil && legacy.Created != "" {
			i.Date = legacy.Created
		}
	}
	if len(raw.RawAuthor) > 0 {
		if raw.RawAuthor[0] == '"' {
			json.Unmarshal(raw.RawAuthor, &i.Author)
		} else if raw.RawAuthor[0] == '{' {
			var obj struct {
				Name   string `json:"name"`
				Email  string `json:"email"`
				Github string `json:"github"`
			}
			if err := json.Unmarshal(raw.RawAuthor, &obj); err == nil {
				if obj.Name != "" && obj.Email != "" {
					i.Author = fmt.Sprintf("%s <%s>", obj.Name, obj.Email)
				} else if obj.Name != "" {
					i.Author = obj.Name
				} else if obj.Email != "" {
					i.Author = obj.Email
				}
			}
		}
	}
	if len(raw.RawSystem) > 0 {
		if raw.RawSystem[0] == '"' {
			json.Unmarshal(raw.RawSystem, &i.System)
		} else if raw.RawSystem[0] == '{' {
			var obj struct {
				Version string `json:"version"`
				Client  string `json:"client"`
			}
			if err := json.Unmarshal(raw.RawSystem, &obj); err == nil {
				i.System = obj.Version
				if obj.Client != "" && i.Client == "" {
					i.Client = obj.Client
				}
			}
		}
	}
	return nil
}

// 🔐️resolveAuthorToAlias holds the data fields for a resolveAuthorToAlias record.
func resolveAuthorToAlias(name, email string) string {
	contributors, err := ListContributors()
	if err == nil {
		for _, c := range contributors {
			if email != "" {
				for _, e := range c.Emails {
					if strings.EqualFold(e, email) {
						return c.Alias
					}
				}
			}
			if name != "" {
				if strings.EqualFold(c.Name, name) {
					return c.Alias
				}
				for _, n := range c.Names {
					if strings.EqualFold(n, name) {
						return c.Alias
					}
				}
			}
		}
	}
	if name != "" && email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	if name != "" {
		return name
	}
	if email != "" {
		return email
	}
	return ""
}

// 🎁️InteractionResource holds a flat interaction enriched with its source context.
type InteractionResource struct {
	Interaction
	SourceKind string `json:"sourceKind"`
	SourceID   string `json:"sourceId"`
	GoalID     string `json:"goalId,omitempty"`
	TicketID   string `json:"ticketId,omitempty"`
}

// ⛳️ListInteractions aggregates interactions from all tickets and goals.
func ListInteractions() ([]InteractionResource, error) {
	var result []InteractionResource
	tickets, err := ListTickets(nil, nil, nil)
	if err == nil {
		for _, t := range tickets {
			tid := t.GetID()
			for _, ix := range t.Interactions {
				result = append(result, InteractionResource{
					Interaction: ix,
					SourceKind:  "ticket",
					SourceID:    tid,
					GoalID:      t.Goal,
					TicketID:    tid,
				})
			}
		}
	}
	return result, nil
}

// ▪️StreamInteractions streams interactions from all tickets and goals.
func StreamInteractions(ctx context.Context, out chan<- InteractionResource) {
	defer close(out)
	tickets, _ := ListTickets(nil, nil, nil)
	for _, t := range tickets {
		tid := t.GetID()
		for _, ix := range t.Interactions {
			select {
			case <-ctx.Done():
				return
			case out <- InteractionResource{
				Interaction: ix,
				SourceKind:  "ticket",
				SourceID:    tid,
				GoalID:      t.Goal,
				TicketID:    tid,
			}:
			}
		}
	}
}

// ▫️TicketSection holds the data fields for a ticket section record.
type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// ◾TicketFile holds the data fields for a ticket file record.
type TicketFile struct {
	Path  string       `json:"path"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🐙️TicketManagementData holds the data fields for a ticket github data record.
type TicketManagementData struct {
	Issue string `json:"issue,omitempty"`
}

// ◽TicketFileRenamed holds the data fields for a ticket file renamed record.
type TicketFileRenamed struct {
	From  string       `json:"from"`
	To    string       `json:"to"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🗃️TicketDiffSet holds the data fields for a ticket diff set record.
type TicketDiffSet struct {
	Deleted  []TicketFile        `json:"deleted"`
	Renamed  []TicketFileRenamed `json:"renamed"`
	Modified []TicketFile        `json:"modified"`
	Added    []TicketFile        `json:"added"`
}

// ◻TicketDiffs holds the data fields for a ticket diffs record.
type TicketDiffs struct {
	Bundles     TicketDiffSet `json:"bundles"`
	Folders     TicketDiffSet `json:"folders"`
	Files       TicketDiffSet `json:"files"`
	Sections    TicketDiffSet `json:"sections"`
	Definitions TicketDiffSet `json:"definitions"`
}

// ◼TicketData holds the data fields for a ticket data record.
type TicketData struct {
	Title      string                `json:"title"`
	Status     TicketStatus          `json:"status"`
	Summary    string                `json:"summary,omitempty"`
	Management *TicketManagementData `json:"github,omitempty"`
	Goal       string                `json:"goal,omitempty"`
	Parent     string                `json:"parent,omitempty"`
}
