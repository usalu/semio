// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎞️recording_queries carries the read queries of the recording repository context.

// #endregion 🧲️Header

package graphql

import (
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎞️Recording Queries

func (recording *RecordingContext) GetRootDir() string { return recording.records.RootDir }

func (recording *RecordingContext) GetTechnologies() []*model.Technology {
	return recording.records.Technologies
}

func (recording *RecordingContext) GetBundles() []*model.Bundle { return recording.records.Bundles }

func (recording *RecordingContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	checkpoints := recording.records.Checkpoints
	if limit != nil && *limit >= 0 && *limit < len(checkpoints) {
		return checkpoints[:*limit], nil
	}
	return checkpoints, nil
}

func (recording *RecordingContext) GetFolders() []*model.Folder { return recording.records.Folders }

func (recording *RecordingContext) GetFiles() []*model.File { return recording.records.Files }

func (recording *RecordingContext) GetSections() []*model.Section { return recording.records.Sections }

func (recording *RecordingContext) GetDefinitions() []*model.Definition {
	return recording.records.Definitions
}

func (recording *RecordingContext) GetContributors() ([]*model.Contributor, error) {
	return recording.records.Contributors, nil
}

func (recording *RecordingContext) GetGoals() ([]*model.Goal, error) {
	return recording.records.Goals, nil
}

func (recording *RecordingContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	rows := make([]*model.Ticket, 0, len(recording.records.Tickets))
	for _, ticket := range recording.records.Tickets {
		if year != nil && ticket.Year != *year {
			continue
		}
		if month != nil && ticket.Month != *month {
			continue
		}
		if day != nil && ticket.Day != *day {
			continue
		}
		if status != nil && ticket.Status != *status {
			continue
		}
		rows = append(rows, ticket)
	}
	return rows, nil
}

func (recording *RecordingContext) GetPolicies() []*model.Policy { return recording.records.Policies }

func (recording *RecordingContext) GetDrafts() ([]*model.Draft, error) {
	return recording.records.Drafts, nil
}

func (recording *RecordingContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	rows := make([]*model.Todo, 0, len(recording.records.Todos))
	for _, todo := range recording.records.Todos {
		if !matchesFilterInput(todo.Name, filter) {
			continue
		}
		rows = append(rows, todo)
	}
	return rows, nil
}

func (recording *RecordingContext) GetStatutes() []*model.StatuteMeta {
	return recording.records.Statutes
}

func (recording *RecordingContext) GetInteractions() ([]model.InteractionResource, error) {
	return recording.records.Interactions, nil
}

func (recording *RecordingContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	result := recording.records.Analyze
	if result == nil {
		return &model.AnalyzeResult{Breachs: []*model.Breach{}, Metrics: &model.AnalyzeMetrics{ByPriority: &model.PriorityCount{}}}, nil
	}
	if scope == nil {
		return result, nil
	}
	breachs := make([]*model.Breach, 0, len(result.Breachs))
	for _, breach := range result.Breachs {
		if strings.HasPrefix(breach.Scope, *scope) {
			breachs = append(breachs, breach)
		}
	}
	return &model.AnalyzeResult{Breachs: breachs, Metrics: result.Metrics}, nil
}

// #endregion 🎞️Recording Queries
