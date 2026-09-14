// 🐹️ Go side of the GitHub management transcript case. The provider reaches the machine only through
// ProcessRunner, so the recorded transcript replaces gh entirely: no binary, no network, no
// repository — exactly the seam the Rust adapter exercises.
package adapter

import (
	"encoding/json"
	"fmt"

	providers "github.com/usalu/semio/repo/providers"
	host "semio.tech/repo/test"
)

// region 🔖️Support

const transcripts = "local://🎞️gh-transcripts.json"

func provider(ctx *host.Context) (*providers.GitHubManagementProvider, error) {
	raw, err := ctx.FixtureBytes(transcripts)
	if err != nil {
		return nil, err
	}
	var byScenario map[string]json.RawMessage
	if err := json.Unmarshal(raw, &byScenario); err != nil {
		return nil, fmt.Errorf("transcript fixture is not valid JSON: %w", err)
	}
	entry, found := byScenario[ctx.Scenario.ID]
	if !found {
		return nil, fmt.Errorf("transcript fixture carries no entry for scenario %s", ctx.Scenario.ID)
	}
	transcript, err := providers.ParseProcessTranscript(entry)
	if err != nil {
		return nil, err
	}
	return providers.NewGitHubManagementProvider(providers.NewRecordedProcessRunner(transcript)), nil
}

func argv(subject *providers.GitHubManagementProvider) []any {
	calls := []any{}
	for _, call := range subject.Issued() {
		parts := []any{}
		for _, part := range call {
			parts = append(parts, part)
		}
		calls = append(calls, parts)
	}
	return calls
}

func labelNames(labels []providers.ManagementLabel) []any {
	names := []any{}
	for _, label := range labels {
		names = append(names, label.Name)
	}
	return names
}

// endregion 🔖️Support

// region 🔖️Scenarios

func issueViewIsParsed(ctx *host.Context) (host.Outcome, error) {
	subject, err := provider(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	issue, err := subject.GetIssueDetails("https://github.com/usalu/semio/issues/412")
	if err != nil {
		return host.Outcome{}, err
	}
	if issue == nil {
		return host.Outcome{}, fmt.Errorf("no issue parsed")
	}
	var milestone any
	if issue.Milestone != nil {
		milestone = map[string]any{"number": issue.Milestone.Number, "title": issue.Milestone.Title}
	}
	names := []any{}
	for _, label := range issue.Labels {
		names = append(names, label.Name)
	}
	return host.Outcome{Projection: map[string]any{
		"issue": map[string]any{
			"url":       issue.URL,
			"state":     issue.State,
			"title":     issue.Title,
			"body":      issue.Body,
			"milestone": milestone,
			"labels":    names,
		},
		"argv": argv(subject),
	}}, nil
}

func milestoneListIsScannedForATitle(ctx *host.Context) (host.Outcome, error) {
	subject, err := provider(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	milestone, err := subject.FindMilestoneByTitle("26/09")
	if err != nil {
		return host.Outcome{}, err
	}
	if milestone == nil {
		return host.Outcome{}, fmt.Errorf("no milestone matched")
	}
	return host.Outcome{Projection: map[string]any{
		"milestone": map[string]any{
			"number":      milestone.Number,
			"title":       milestone.Title,
			"description": milestone.Description,
			"url":         milestone.URL,
			"dueOn":       milestone.DueOn,
			"state":       milestone.State,
		},
		"argv": argv(subject),
	}}, nil
}

func labelCatalogIsListed(ctx *host.Context) (host.Outcome, error) {
	subject, err := provider(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	labels, err := subject.ListRepoLabels()
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"labels": labelNames(labels), "argv": argv(subject)}}, nil
}

func createIssueResolvesTheMilestoneTitleFirst(ctx *host.Context) (host.Outcome, error) {
	subject, err := provider(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	milestone := 17
	url, err := subject.CreateIssue("Split the godfile", "One package per domain.", &milestone)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"url": url, "argv": argv(subject)}}, nil
}

func aFailingCallCarriesTheTrimmedStderr(ctx *host.Context) (host.Outcome, error) {
	subject, err := provider(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	failed := false
	message := ""
	if _, callErr := subject.GetIssueDetails("https://github.com/usalu/semio/issues/999"); callErr != nil {
		failed = true
		message = callErr.Error()
	}
	return host.Outcome{Projection: map[string]any{"failed": failed, "message": message, "argv": argv(subject)}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("issue-view-is-parsed", issueViewIsParsed).
		Subject("milestone-list-is-scanned-for-a-title", milestoneListIsScannedForATitle).
		Subject("label-catalog-is-listed", labelCatalogIsListed).
		Subject("create-issue-resolves-the-milestone-title-first", createIssueResolvesTheMilestoneTitleFirst).
		Subject("a-failing-call-carries-the-trimmed-stderr", aFailingCallCarriesTheTrimmedStderr)
}

// endregion 🔖️Registration
