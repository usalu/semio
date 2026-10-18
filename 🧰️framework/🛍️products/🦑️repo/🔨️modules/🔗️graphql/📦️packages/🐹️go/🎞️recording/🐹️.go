// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎞️recording carries the recording repository context, its state and snapshot.

// #endregion 🧲️Header

package graphql

import (
	json "encoding/json"
	strings "strings"
	unicode "unicode"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎞️Recording Context

/** 🗄️ The mutable record set a [RecordingContext] answers from. */
type recordingRecords struct {
	RootDir      string                      `json:"rootDir"`
	Now          string                      `json:"now"`
	Technologies []*model.Technology         `json:"technologies"`
	Bundles      []*model.Bundle             `json:"bundles"`
	Checkpoints  []*model.Checkpoint         `json:"checkpoints"`
	Folders      []*model.Folder             `json:"folders"`
	Files        []*model.File               `json:"files"`
	Sections     []*model.Section            `json:"sections"`
	Definitions  []*model.Definition         `json:"definitions"`
	Contributors []*model.Contributor        `json:"contributors"`
	Policies     []*model.Policy             `json:"policies"`
	Drafts       []*model.Draft              `json:"drafts"`
	Todos        []*model.Todo               `json:"todos"`
	Statutes     []*model.StatuteMeta        `json:"statutes"`
	Interactions []model.InteractionResource `json:"-"`
	Analyze      *model.AnalyzeResult        `json:"analyze"`
	Goals        []*model.Goal               `json:"-"`
	Tickets      []*model.Ticket             `json:"-"`
	Events       []map[string]interface{}    `json:"-"`
}

/** 🎫️ The ticket fields the wire shape carries but the domain type derives from its path. */
type recordedTicketIdentity struct {
	Year          int                 `json:"year"`
	Month         int                 `json:"month"`
	Day           int                 `json:"day"`
	Slug          string              `json:"slug"`
	Parent        string              `json:"parent"`
	FolderPath    string              `json:"folderPath"`
	JsonPath      string              `json:"jsonPath"`
	ImportantPath string              `json:"importantPath"`
	Interactions  []model.Interaction `json:"interactions"`
	Agents        []model.TicketAgent `json:"agents"`
}

/** 🎯️ The goal fields the wire shape carries but the domain type derives from its path. */
type recordedGoalIdentity struct {
	ID   string `json:"id"`
	Path string `json:"path"`
}

/** 🩻️ A fixture-backed context that answers every read from a record set and records every write.
 *
 * It is the executor's language-agnostic test double: the same record file drives every subject and
 * the oracle, so what two implementations disagree about is the executor, never the filesystem. */
type RecordingContext struct {
	records *recordingRecords
}

/** 📥️ Builds a context from one record document's bytes. */
func NewRecordingContext(source []byte) (*RecordingContext, error) {
	records := &recordingRecords{Now: "2026-01-01 00:00:00"}
	if err := json.Unmarshal(source, records); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var identities struct {
		Tickets []recordedTicketIdentity `json:"tickets"`
		Goals   []recordedGoalIdentity   `json:"goals"`
	}
	if err := json.Unmarshal(source, &identities); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var shapes struct {
		Tickets      []*plainTicket      `json:"tickets"`
		Goals        []*plainGoal        `json:"goals"`
		Interactions []model.Interaction `json:"interactions"`
	}
	if err := json.Unmarshal(source, &shapes); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var resources struct {
		Interactions []struct {
			SourceKind string `json:"sourceKind"`
			SourceID   string `json:"sourceId"`
			GoalID     string `json:"goalId"`
			TicketID   string `json:"ticketId"`
		} `json:"interactions"`
	}
	if err := json.Unmarshal(source, &resources); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	for index, interaction := range shapes.Interactions {
		resource := model.InteractionResource{Interaction: interaction}
		if index < len(resources.Interactions) {
			extra := resources.Interactions[index]
			resource.SourceKind, resource.SourceID = extra.SourceKind, extra.SourceID
			resource.GoalID, resource.TicketID = extra.GoalID, extra.TicketID
		}
		records.Interactions = append(records.Interactions, resource)
	}
	for index, ticket := range shapes.Tickets {
		if index >= len(identities.Tickets) {
			break
		}
		identity := identities.Tickets[index]
		ticket.Year, ticket.Month, ticket.Day, ticket.Slug = identity.Year, identity.Month, identity.Day, identity.Slug
		ticket.Parent, ticket.FolderPath, ticket.JsonPath, ticket.ImportantPath = identity.Parent, identity.FolderPath, identity.JsonPath, identity.ImportantPath
		ticket.Interactions, ticket.Agents = identity.Interactions, identity.Agents
	}
	for index, goal := range shapes.Goals {
		if index >= len(identities.Goals) {
			break
		}
		goal.ID, goal.Path = identities.Goals[index].ID, identities.Goals[index].Path
	}
	for _, ticket := range shapes.Tickets {
		records.Tickets = append(records.Tickets, (*model.Ticket)(ticket))
	}
	for _, goal := range shapes.Goals {
		records.Goals = append(records.Goals, (*model.Goal)(goal))
	}
	if records.Now == "" {
		records.Now = "2026-01-01 00:00:00"
	}
	return &RecordingContext{records: records}, nil
}

/** 📥️ Builds a context from one record document's text. */
func NewRecordingContextFromText(source string) (*RecordingContext, error) {
	return NewRecordingContext([]byte(source))
}

/** 📜️ The events every mutation appended, in the order they happened. */
func (recording *RecordingContext) Events() []interface{} {
	rows := make([]interface{}, 0, len(recording.records.Events))
	for _, event := range recording.records.Events {
		rows = append(rows, event)
	}
	return rows
}

/** 🎯️ Every goal record, read off its declared fields alone. */
func plainGoals(goals []*model.Goal) []interface{} {
	rows := make([]interface{}, 0, len(goals))
	for _, goal := range goals {
		rows = append(rows, goalBody(goal))
	}
	return rows
}

/** 🗄️ The mutable record set as it stands now, for a mutation case to assert on. */
func (recording *RecordingContext) Snapshot() interface{} {
	tickets := make([]interface{}, 0, len(recording.records.Tickets))
	for _, ticket := range recording.records.Tickets {
		row := ticketBody(ticket)
		row["year"], row["month"], row["day"], row["slug"] = ticket.Year, ticket.Month, ticket.Day, ticket.Slug
		row["interactions"] = toJSON(ticket.Interactions)
		tickets = append(tickets, row)
	}
	return map[string]interface{}{
		"goals":        plainGoals(recording.records.Goals),
		"tickets":      tickets,
		"todos":        toJSON(recording.records.Todos),
		"drafts":       toJSON(recording.records.Drafts),
		"contributors": toJSON(recording.records.Contributors),
		"folders":      toJSON(recording.records.Folders),
		"files":        toJSON(recording.records.Files),
		"sections":     toJSON(recording.records.Sections),
	}
}

/** 📝️ Appends one event. */
func (recording *RecordingContext) record(kind string, payload interface{}) {
	recording.records.Events = append(recording.records.Events, map[string]interface{}{"kind": kind, "payload": toJSON(payload)})
}

/** 🔤️ The identifier a created aggregate derives from its title. */
func deriveRecordID(title string) string {
	parts := strings.FieldsFunc(strings.ToLower(title), func(character rune) bool {
		return !unicode.IsLetter(character) && !unicode.IsDigit(character)
	})
	return strings.Join(parts, "-")
}

// #endregion 🎞️Recording Context
