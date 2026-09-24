// 🐹️ Go side of the issue synchronisation case. The IssueTracker port is driven by a scripted
// recorder, so nothing spawns a management binary and every interaction is observable in order.
package adapter

import (
	"encoding/json"
	"fmt"

	tickets "github.com/usalu/semio/repo/tickets"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type transcriptVectors struct {
	RepoMetaDir string                     `json:"repoMetaDir"`
	Goal        string                     `json:"goal"`
	IssueURL    string                     `json:"issueUrl"`
	Title       string                     `json:"title"`
	Prompt      string                     `json:"prompt"`
	Summary     string                     `json:"summary"`
	Labels      []string                   `json:"labels"`
	Scripts     map[string]json.RawMessage `json:"scripts"`
}

func loadTranscripts(ctx *host.Context) (transcriptVectors, error) {
	data, err := ctx.FixtureBytes("shared://🐙️issue-sync-transcripts/🐙️transcripts.json")
	if err != nil {
		return transcriptVectors{}, err
	}
	var vectors transcriptVectors
	err = json.Unmarshal(data, &vectors)
	return vectors, err
}

func recorder(vectors transcriptVectors, scenario string) (*tickets.RecordedIssueTracker, error) {
	script, found := vectors.Scripts[scenario]
	if !found {
		return nil, fmt.Errorf("no script for scenario %s", scenario)
	}
	return tickets.RecordedIssueTrackerFromJSON(string(script))
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func aNewTicketCreatesOneIssue(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTranscripts(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	tracker, err := recorder(vectors, "a-new-ticket-creates-one-issue")
	if err != nil {
		return host.Outcome{}, err
	}
	milestone := tickets.MilestoneNumberForTitle(tracker, vectors.Goal)
	outcome := tickets.SyncOpenIssue(tracker, "", vectors.Title, vectors.Prompt, "", milestone, false)
	reference := ""
	if milestone != nil {
		reference = fmt.Sprintf("%d", *milestone)
	}
	return host.Outcome{Projection: map[string]any{
		"calls":     tracker.Calls(),
		"issue":     outcome.Issue,
		"warnings":  outcome.Warnings,
		"milestone": reference,
	}}, nil
}

func anExistingOpenIssueIsLeftAlone(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTranscripts(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	tracker, err := recorder(vectors, "an-existing-open-issue-is-left-alone")
	if err != nil {
		return host.Outcome{}, err
	}
	outcome := tickets.SyncOpenIssue(tracker, vectors.IssueURL, vectors.Title, vectors.Prompt, "", nil, true)
	return host.Outcome{Projection: map[string]any{"calls": tracker.Calls(), "issue": outcome.Issue, "warnings": outcome.Warnings}}, nil
}

func aClosedIssueIsReopened(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTranscripts(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	tracker, err := recorder(vectors, "a-closed-issue-is-reopened")
	if err != nil {
		return host.Outcome{}, err
	}
	outcome := tickets.SyncOpenIssue(tracker, vectors.IssueURL, vectors.Title, vectors.Prompt, "", nil, true)
	return host.Outcome{Projection: map[string]any{"calls": tracker.Calls(), "issue": outcome.Issue, "warnings": outcome.Warnings}}, nil
}

func aCloseCommentsLabelsAndCloses(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTranscripts(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	normal, err := recorder(vectors, "a-close-comments-labels-and-closes")
	if err != nil {
		return host.Outcome{}, err
	}
	bulk, err := recorder(vectors, "a-close-comments-labels-and-closes")
	if err != nil {
		return host.Outcome{}, err
	}
	normalWarnings := tickets.SyncCloseIssue(normal, vectors.IssueURL, vectors.Summary, vectors.Labels, false)
	bulkWarnings := tickets.SyncCloseIssue(bulk, vectors.IssueURL, vectors.Summary, vectors.Labels, true)
	return host.Outcome{Projection: map[string]any{
		"normalCalls":    normal.Calls(),
		"normalWarnings": normalWarnings,
		"bulkCalls":      bulk.Calls(),
		"bulkWarnings":   bulkWarnings,
	}}, nil
}

func everyFailureBecomesAWarning(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTranscripts(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	creating, err := recorder(vectors, "every-failure-becomes-a-warning")
	if err != nil {
		return host.Outcome{}, err
	}
	reopening, err := recorder(vectors, "every-failure-becomes-a-warning")
	if err != nil {
		return host.Outcome{}, err
	}
	closing, err := recorder(vectors, "every-failure-becomes-a-warning")
	if err != nil {
		return host.Outcome{}, err
	}
	created := tickets.SyncOpenIssue(creating, "", vectors.Title, vectors.Prompt, "", nil, false)
	reopened := tickets.SyncOpenIssue(reopening, vectors.IssueURL, vectors.Title, vectors.Prompt, "", nil, true)
	closeWarnings := tickets.SyncCloseIssue(closing, vectors.IssueURL, vectors.Summary, vectors.Labels, false)
	return host.Outcome{Projection: map[string]any{
		"createWarnings": created.Warnings,
		"createIssue":    created.Issue,
		"reopenWarnings": reopened.Warnings,
		"reopenCalls":    reopening.Calls(),
		"closeWarnings":  closeWarnings,
		"closeCalls":     closing.Calls(),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("a-new-ticket-creates-one-issue", aNewTicketCreatesOneIssue).
		Subject("an-existing-open-issue-is-left-alone", anExistingOpenIssueIsLeftAlone).
		Subject("a-closed-issue-is-reopened", aClosedIssueIsReopened).
		Subject("a-close-comments-labels-and-closes", aCloseCommentsLabelsAndCloses).
		Subject("every-failure-becomes-a-warning", everyFailureBecomesAWarning)
}

// endregion 🔖️Registration
