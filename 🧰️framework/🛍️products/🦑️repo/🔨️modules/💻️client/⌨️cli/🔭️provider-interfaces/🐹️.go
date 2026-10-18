// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Composable provider interface contracts.

// #endregion 🧲️Header

package client



// 🔌️VersionControlProvider defines the interface for version control operations (Git, ...).
type VersionControlProvider interface {
	Kind() string
	RepoURL() (string, error)

	Checkpoint(repoRoot string, description string) (id string, err error)

	CurrentCheckpoint(repoRoot string) (string, error)

	Checkin(repoRoot string, contributor string) error

	Checkout(repoRoot string, contributor string, description string) (id string, err error)

	CurrentBranch(repoRoot string) (string, error)

	StagedFiles(repoRoot string) ([]string, error)

	StageAll(repoRoot string) error
}

// 💿️ManagementIssue holds the data fields for a management issue record.
type ManagementIssue struct {
	URL       string `json:"url"`
	State     string `json:"state"`
	Title     string `json:"title"`
	Body      string `json:"body"`
	Milestone *struct {
		Number int    `json:"number"`
		Title  string `json:"title"`
	} `json:"milestone"`
	Labels []struct {
		Name string `json:"name"`
	} `json:"labels"`
}

// 🎯️ManagementMilestone holds the data fields for a management milestone record.
type ManagementMilestone struct {
	Number      int    `json:"number"`
	Title       string `json:"title"`
	Description string `json:"description"`
	URL         string `json:"url"`
	DueOn       string `json:"due_on"`
	State       string `json:"state"`
}

// 🏷️ManagementLabel holds the data fields for a management label record.
type ManagementLabel struct {
	Name string `json:"name"`
}

// 🐙️ManagementProvider defines the interface for issue/milestone management operations (GitHub, Jira, Trello, Linear, ...).
type ManagementProvider interface {
	Kind() string
	CreateIssue(title, body string, milestone *int) (string, error)
	CloseIssue(issueURL string) error
	ReopenIssue(issueURL string) error
	DeleteIssue(issueURLOrNumber string) error
	UpdateIssueTitle(issueURL, title string) error
	UpdateIssueBody(issueURL, body string) error
	GetIssueDetails(issueURL string) (*ManagementIssue, error)
	GetIssueNodeID(issueURL string) (string, error)
	GetIssueParentURL(issueURL string) (string, error)
	AddComment(issueURL, comment string) error
	AddLabels(issueURL string, labels []string) error
	RemoveLabels(issueURL string, labels []string) error
	AddIssueToProject(issueURL string)
	AssignIssueToCurrentUser(issueURL string)
	AddSubIssue(parentIssueURL, childIssueURL string) error
	UpdateIssueMilestone(issueURL, milestoneTitle string) error
	ClearIssueMilestone(issueURL string) error
	CreateMilestone(title, description string) (int, error)
	UpdateMilestone(number int, title, description, state, dueOn string) error
	DeleteMilestone(number int) error
	GetMilestone(number int) (*ManagementMilestone, error)
	GetMilestoneTitle(number int) (string, error)
	FindMilestoneByTitle(title string) (*ManagementMilestone, error)
	ListIssuesForLabelSync() ([]ManagementIssue, error)
	ListOpenIssuesWithLabel(label string) ([]string, error)
	ListRepoLabels() ([]ManagementLabel, error)
	CreateRepoLabel(name string) error
	DeleteRepoLabel(name string) error
	SyncRepoLabelCatalog(validLabels map[string]bool) error
	CreateGoalIssue(title, description string, milestone *int) (string, error)
	UpdateGoalIssue(issueURL, title, description string) error
	GetCurrentUser() string
}

// 🐳️SandboxProvider defines the interface for sandbox/container operations (Devcontainer, Podman, ...).
type SandboxProvider interface {
	Kind() string
}

// 📝️EditorProvider defines the interface for editor/agent operations (VSCode/Copilot, Cursor, Windsurf, Claude Code, Codex, Droid, Antigravity, ...).
type EditorProvider interface {
	Kind() string
	ResolveNativeEvent(nativeEvent string, toolKind ToolKind) (HookEvent, string, error)
	FormatHookOutput(hookEventName string, result HookResult) string
	NativeEventFromHookEvent(event HookEvent, parentInfo string) string
}
