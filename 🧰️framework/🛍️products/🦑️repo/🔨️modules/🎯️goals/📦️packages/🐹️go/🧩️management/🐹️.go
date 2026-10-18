package goals

import (
	fmt "fmt"
)

// #region 🧩️Management

// 🧩️ManagementPort is the provider that owns milestones and issues for the goals of a repository.
type ManagementPort interface {
	// 🏁️CreateMilestone creates a milestone and returns its number.
	CreateMilestone(title, description string) (int64, error)
	// 🔁️UpdateMilestone updates a milestone, leaving blank members untouched.
	UpdateMilestone(number int64, title, description, state, dueOn string) error
	// 🗑️DeleteMilestone deletes a milestone.
	DeleteMilestone(number int64) error
	// 🆕️CreateGoalIssue creates the issue of a goal and returns its URL.
	CreateGoalIssue(title, description string, milestone *int64) (string, error)
	// 🔷️UpdateGoalIssue updates the title and body of a goal issue.
	UpdateGoalIssue(issueURL, title, description string) error
	// 📪️CloseIssue closes an issue.
	CloseIssue(issueURL string) error
	// 🔓️ReopenIssue reopens an issue.
	ReopenIssue(issueURL string) error
	// 🗑️DeleteIssue deletes an issue by its number.
	DeleteIssue(number string) error
	// ➕️AddSubIssue nests a child issue under a parent issue.
	AddSubIssue(parentIssueURL, childIssueURL string) error
	// 🌐️RepoURL returns the repository URL milestone references are built from.
	RepoURL() string
}

// 🚫️NullManagement is the provider that does nothing, for a run with management switched off.
type NullManagement struct{}

// 🏁️CreateMilestone does nothing and hands out the zero number.
func (NullManagement) CreateMilestone(string, string) (int64, error) { return 0, nil }

// 🔁️UpdateMilestone does nothing.
func (NullManagement) UpdateMilestone(int64, string, string, string, string) error { return nil }

// 🗑️DeleteMilestone does nothing.
func (NullManagement) DeleteMilestone(int64) error { return nil }

// 🆕️CreateGoalIssue does nothing and hands out no URL.
func (NullManagement) CreateGoalIssue(string, string, *int64) (string, error) { return "", nil }

// 🔷️UpdateGoalIssue does nothing.
func (NullManagement) UpdateGoalIssue(string, string, string) error { return nil }

// 📪️CloseIssue does nothing.
func (NullManagement) CloseIssue(string) error { return nil }

// 🔓️ReopenIssue does nothing.
func (NullManagement) ReopenIssue(string) error { return nil }

// 🗑️DeleteIssue does nothing.
func (NullManagement) DeleteIssue(string) error { return nil }

// ➕️AddSubIssue does nothing.
func (NullManagement) AddSubIssue(string, string) error { return nil }

// 🌐️RepoURL returns no repository URL.
func (NullManagement) RepoURL() string { return "" }

// 📝️RecordingManagement hands out deterministic numbers and keeps an ordered log of every call.
type RecordingManagement struct {
	repoURL string
	next    int64
	calls   []string
}

// 🆕️NewRecordingManagement builds a provider that numbers milestones and issues from first.
func NewRecordingManagement(repoURL string, first int64) *RecordingManagement {
	return &RecordingManagement{repoURL: repoURL, next: first}
}

// 📜️Calls returns every call made so far, in order.
func (management *RecordingManagement) Calls() []string {
	return append([]string{}, management.calls...)
}

// 🔢️takeNumber hands out the next deterministic number.
func (management *RecordingManagement) takeNumber() int64 {
	number := management.next
	management.next++
	return number
}

// ✍️record appends one call to the log.
func (management *RecordingManagement) record(call string) {
	management.calls = append(management.calls, call)
}

// 🏁️CreateMilestone records the call and hands out the next number.
func (management *RecordingManagement) CreateMilestone(title, description string) (int64, error) {
	number := management.takeNumber()
	management.record(fmt.Sprintf("create-milestone %d %s %s", number, title, description))
	return number, nil
}

// 🔁️UpdateMilestone records the call.
func (management *RecordingManagement) UpdateMilestone(number int64, title, description, state, dueOn string) error {
	management.record(fmt.Sprintf("update-milestone %d %s %s %s %s", number, title, description, state, dueOn))
	return nil
}

// 🗑️DeleteMilestone records the call.
func (management *RecordingManagement) DeleteMilestone(number int64) error {
	management.record(fmt.Sprintf("delete-milestone %d", number))
	return nil
}

// 🆕️CreateGoalIssue records the call and hands out a deterministic issue URL.
func (management *RecordingManagement) CreateGoalIssue(title, description string, milestone *int64) (string, error) {
	number := management.takeNumber()
	reference := "-"
	if milestone != nil {
		reference = fmt.Sprintf("%d", *milestone)
	}
	management.record(fmt.Sprintf("create-goal-issue %d %s %s %s", number, title, description, reference))
	return fmt.Sprintf("%s/issues/%d", management.repoURL, number), nil
}

// 🔷️UpdateGoalIssue records the call.
func (management *RecordingManagement) UpdateGoalIssue(issueURL, title, description string) error {
	management.record(fmt.Sprintf("update-goal-issue %s %s %s", issueURL, title, description))
	return nil
}

// 📪️CloseIssue records the call.
func (management *RecordingManagement) CloseIssue(issueURL string) error {
	management.record(fmt.Sprintf("close-issue %s", issueURL))
	return nil
}

// 🔓️ReopenIssue records the call.
func (management *RecordingManagement) ReopenIssue(issueURL string) error {
	management.record(fmt.Sprintf("reopen-issue %s", issueURL))
	return nil
}

// 🗑️DeleteIssue records the call.
func (management *RecordingManagement) DeleteIssue(number string) error {
	management.record(fmt.Sprintf("delete-issue %s", number))
	return nil
}

// ➕️AddSubIssue records the call.
func (management *RecordingManagement) AddSubIssue(parentIssueURL, childIssueURL string) error {
	management.record(fmt.Sprintf("add-sub-issue %s %s", parentIssueURL, childIssueURL))
	return nil
}

// 🌐️RepoURL returns the repository URL the provider was built with.
func (management *RecordingManagement) RepoURL() string { return management.repoURL }

// #endregion 🧩️Management
