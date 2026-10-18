package tickets

import (
	json "encoding/json"
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🐙️Issue Sync

// 🤖️FormatPromptHeading returns the prompt section heading of an issue body or comment.
func FormatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖️ Prompt"
	}
	return "# 🤖️ Prompt\n\n" + body
}

// 🔍️FormatSummaryHeading returns the summary section heading of a closing comment.
func FormatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍️ Summary"
	}
	return "# 🔍️ Summary\n\n" + body
}

// 🐙️EnsureTicketIssue creates, links or reopens the issue of a ticket, returning the warnings.
func EnsureTicketIssue(tracker IssueTracker, ticket *model.Ticket, title, prompt, issue string, milestone *int, reopenIfClosed bool) []string {
	warnings := make([]string, 0)
	issue = strings.TrimSpace(issue)
	if issue != "" {
		ticket.Management = &model.TicketManagementData{Issue: issue}
		return warnings
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		if reopenIfClosed {
			remote, err := tracker.GetIssueDetails(ticket.Management.Issue)
			switch {
			case err != nil:
				warnings = append(warnings, "read github issue: "+err.Error())
			case remote != nil && strings.EqualFold(remote.State, "closed"):
				if err := tracker.ReopenIssue(ticket.Management.Issue); err != nil {
					warnings = append(warnings, "reopen github issue: "+err.Error())
				}
			}
		}
		return warnings
	}
	url, err := tracker.CreateIssue(title, FormatPromptHeading(prompt), milestone)
	switch {
	case err != nil:
		warnings = append(warnings, "Failed to create GitHub issue: "+err.Error())
	case strings.TrimSpace(url) == "":
		warnings = append(warnings, "github issue create returned empty url")
	default:
		ticket.Management = &model.TicketManagementData{Issue: url}
	}
	return warnings
}

// 📪️CloseTicketIssue comments the summary, labels the issue with the touched bundles and closes it.
func CloseTicketIssue(tracker IssueTracker, ticket *model.Ticket, summary string, labels []string, bulk bool) []string {
	warnings := make([]string, 0)
	if ticket.Management == nil || ticket.Management.Issue == "" {
		return warnings
	}
	issueURL := ticket.Management.Issue
	if !bulk {
		if len(labels) > 0 {
			if err := tracker.AddLabels(issueURL, labels); err != nil {
				warnings = append(warnings, "Failed to add labels to GitHub issue: "+err.Error())
			}
		}
		if err := tracker.AddComment(issueURL, FormatSummaryHeading(summary)); err != nil {
			warnings = append(warnings, "Failed to add summary and metrics comment to GitHub issue: "+err.Error())
		}
	}
	if err := tracker.CloseIssue(issueURL); err != nil {
		warnings = append(warnings, "Failed to close GitHub issue: "+err.Error())
	}
	return warnings
}

// 🎯️MilestoneNumberForTitle looks a milestone number up by title, swallowing a lookup failure.
func MilestoneNumberForTitle(tracker IssueTracker, title string) *int {
	milestone, err := tracker.FindMilestoneByTitle(title)
	if err != nil || milestone == nil {
		return nil
	}
	number := milestone.Number
	return &number
}

// 🎞️IssueTrackerScript is what a recorded issue tracker answers.
type IssueTrackerScript struct {
	CreatedIssueURL string   `json:"createdIssueUrl"`
	IssueState      string   `json:"issueState"`
	Milestone       *int     `json:"milestone"`
	MilestoneTitle  string   `json:"milestoneTitle"`
	Failures        []string `json:"failures"`
}

// 🎞️RecordedIssueTracker is a fixture-driven issue tracker that records every interaction.
type RecordedIssueTracker struct {
	script IssueTrackerScript
	calls  []string
}

// 🆕️NewRecordedIssueTracker binds a tracker to a script.
func NewRecordedIssueTracker(script IssueTrackerScript) *RecordedIssueTracker {
	return &RecordedIssueTracker{script: script}
}

// 📥️RecordedIssueTrackerFromJSON reads a script from JSON text.
func RecordedIssueTrackerFromJSON(text string) (*RecordedIssueTracker, error) {
	var script IssueTrackerScript
	if err := json.Unmarshal([]byte(text), &script); err != nil {
		return nil, errInvalid(err.Error())
	}
	return NewRecordedIssueTracker(script), nil
}

// 📼️Calls returns every interaction so far, in order.
func (tracker *RecordedIssueTracker) Calls() []string {
	return append([]string{}, tracker.calls...)
}

// ✍️record appends one interaction.
func (tracker *RecordedIssueTracker) record(call string) { tracker.calls = append(tracker.calls, call) }

// 💣️fails returns the scripted failure of a method.
func (tracker *RecordedIssueTracker) fails(method string) error {
	for _, candidate := range tracker.script.Failures {
		if candidate == method {
			return fmt.Errorf("%s failed", method)
		}
	}
	return nil
}

// 🆕️CreateIssue records the call and answers the scripted URL.
func (tracker *RecordedIssueTracker) CreateIssue(title, body string, milestone *int) (string, error) {
	reference := ""
	if milestone != nil {
		reference = strconv.Itoa(*milestone)
	}
	tracker.record(fmt.Sprintf("create_issue|%s|%s|%s", title, body, reference))
	if err := tracker.fails("create_issue"); err != nil {
		return "", err
	}
	return tracker.script.CreatedIssueURL, nil
}

// 📪️CloseIssue records the call.
func (tracker *RecordedIssueTracker) CloseIssue(issueURL string) error {
	tracker.record("close_issue|" + issueURL)
	return tracker.fails("close_issue")
}

// 🔓️ReopenIssue records the call.
func (tracker *RecordedIssueTracker) ReopenIssue(issueURL string) error {
	tracker.record("reopen_issue|" + issueURL)
	return tracker.fails("reopen_issue")
}

// 🔎️GetIssueDetails records the call and answers the scripted state.
func (tracker *RecordedIssueTracker) GetIssueDetails(issueURL string) (*ManagementIssue, error) {
	tracker.record("get_issue_details|" + issueURL)
	if err := tracker.fails("get_issue_details"); err != nil {
		return nil, err
	}
	if tracker.script.IssueState == "" {
		return nil, nil
	}
	return &ManagementIssue{URL: issueURL, State: tracker.script.IssueState}, nil
}

// 💬️AddComment records the call.
func (tracker *RecordedIssueTracker) AddComment(issueURL, comment string) error {
	tracker.record(fmt.Sprintf("add_comment|%s|%s", issueURL, comment))
	return tracker.fails("add_comment")
}

// 🏷️AddLabels records the call.
func (tracker *RecordedIssueTracker) AddLabels(issueURL string, labels []string) error {
	tracker.record(fmt.Sprintf("add_labels|%s|%s", issueURL, strings.Join(labels, ",")))
	return tracker.fails("add_labels")
}

// 🏷️RemoveLabels records the call.
func (tracker *RecordedIssueTracker) RemoveLabels(issueURL string, labels []string) error {
	tracker.record(fmt.Sprintf("remove_labels|%s|%s", issueURL, strings.Join(labels, ",")))
	return tracker.fails("remove_labels")
}

// 🎯️FindMilestoneByTitle records the call and answers the scripted milestone.
func (tracker *RecordedIssueTracker) FindMilestoneByTitle(title string) (*ManagementMilestone, error) {
	tracker.record("find_milestone_by_title|" + title)
	if err := tracker.fails("find_milestone_by_title"); err != nil {
		return nil, err
	}
	if tracker.script.Milestone == nil {
		return nil, nil
	}
	return &ManagementMilestone{Number: *tracker.script.Milestone, Title: tracker.script.MilestoneTitle}, nil
}

// 🏷️ListRepoLabels records the call.
func (tracker *RecordedIssueTracker) ListRepoLabels() ([]ManagementLabel, error) {
	tracker.record("list_repo_labels")
	if err := tracker.fails("list_repo_labels"); err != nil {
		return nil, err
	}
	return nil, nil
}

// 🚫️NullIssueTracker is the tracker that answers nothing, for a lifecycle run without management.
type NullIssueTracker struct{}

// 🆕️CreateIssue answers no URL.
func (NullIssueTracker) CreateIssue(string, string, *int) (string, error) { return "", nil }

// 📪️CloseIssue does nothing.
func (NullIssueTracker) CloseIssue(string) error { return nil }

// 🔓️ReopenIssue does nothing.
func (NullIssueTracker) ReopenIssue(string) error { return nil }

// 🔎️GetIssueDetails answers no issue.
func (NullIssueTracker) GetIssueDetails(string) (*ManagementIssue, error) { return nil, nil }

// 💬️AddComment does nothing.
func (NullIssueTracker) AddComment(string, string) error { return nil }

// 🏷️AddLabels does nothing.
func (NullIssueTracker) AddLabels(string, []string) error { return nil }

// 🏷️RemoveLabels does nothing.
func (NullIssueTracker) RemoveLabels(string, []string) error { return nil }

// 🎯️FindMilestoneByTitle answers no milestone.
func (NullIssueTracker) FindMilestoneByTitle(string) (*ManagementMilestone, error) { return nil, nil }

// 🏷️ListRepoLabels answers no labels.
func (NullIssueTracker) ListRepoLabels() ([]ManagementLabel, error) { return nil, nil }

// 🐙️IssueSyncOutcome is what one issue synchronisation left behind.
type IssueSyncOutcome struct {
	Issue    string   `json:"issue"`
	Warnings []string `json:"warnings"`
}

// 🐙️SyncOpenIssue synchronises the issue of a ticket that is being opened or reopened.
func SyncOpenIssue(tracker IssueTracker, existingIssue, title, prompt, issue string, milestone *int, reopenIfClosed bool) IssueSyncOutcome {
	ticket := &model.Ticket{Title: title, Status: model.TicketStatusOpen}
	if existingIssue != "" {
		ticket.Management = &model.TicketManagementData{Issue: existingIssue}
	}
	warnings := EnsureTicketIssue(tracker, ticket, title, prompt, issue, milestone, reopenIfClosed)
	link := ""
	if ticket.Management != nil {
		link = ticket.Management.Issue
	}
	return IssueSyncOutcome{Issue: link, Warnings: warnings}
}

// 📪️SyncCloseIssue synchronises the issue of a ticket that is being closed.
func SyncCloseIssue(tracker IssueTracker, issueURL, summary string, labels []string, bulk bool) []string {
	ticket := &model.Ticket{Status: model.TicketStatusClosed}
	if issueURL != "" {
		ticket.Management = &model.TicketManagementData{Issue: issueURL}
	}
	return CloseTicketIssue(tracker, ticket, summary, labels, bulk)
}

// #endregion 🐙️Issue Sync
