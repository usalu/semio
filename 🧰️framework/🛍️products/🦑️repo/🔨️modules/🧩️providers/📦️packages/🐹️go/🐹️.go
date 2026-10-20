// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🗜️Package providers holds the composable provider integrations of the semio repository domain:
// issue/milestone management, version control, sandbox and the editor providers. Every external
// process goes through ProcessRunner, so a provider is exercised without gh or git on the machine.
package providers

import (
	bufio "bufio"
	json "encoding/json"
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🔭️Providers

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

// 📥️ParseManagementIssue reads one `gh issue view --json …` document.
func ParseManagementIssue(source []byte) (ManagementIssue, error) {
	var issue ManagementIssue
	if err := json.Unmarshal(source, &issue); err != nil {
		return ManagementIssue{}, fmt.Errorf("failed to parse gh issue view output: %w", err)
	}
	return issue, nil
}

// 📥️ParseManagementIssues reads a `gh issue list --json …` array.
func ParseManagementIssues(source []byte) ([]ManagementIssue, error) {
	var issues []ManagementIssue
	if err := json.Unmarshal(source, &issues); err != nil {
		return nil, fmt.Errorf("failed to parse gh issue list output: %w", err)
	}
	return issues, nil
}

// 📥️ParseManagementMilestone reads one `gh api …/milestones/N` document.
func ParseManagementMilestone(source []byte) (ManagementMilestone, error) {
	var milestone ManagementMilestone
	if err := json.Unmarshal(source, &milestone); err != nil {
		return ManagementMilestone{}, fmt.Errorf("failed to parse gh milestone output: %w", err)
	}
	return milestone, nil
}

// 🔎️ScanManagementMilestones walks the paginated `--jq .[]` milestone stream line by line and stops
// at the first milestone carrying the wanted title; an unparsable line is skipped, not fatal.
func ScanManagementMilestones(stream string, title string) (*ManagementMilestone, error) {
	scanner := bufio.NewScanner(strings.NewReader(stream))
	for scanner.Scan() {
		var milestone ManagementMilestone
		if err := json.Unmarshal(scanner.Bytes(), &milestone); err != nil {
			continue
		}
		if milestone.Title == title {
			found := milestone
			return &found, nil
		}
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return nil, nil
}

// 📥️ParseManagementLabels reads a `gh label list --json name` array.
func ParseManagementLabels(source []byte) ([]ManagementLabel, error) {
	var labels []ManagementLabel
	if err := json.Unmarshal(source, &labels); err != nil {
		return nil, fmt.Errorf("failed to parse gh label list output: %w", err)
	}
	return labels, nil
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
	ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error)
	FormatHookOutput(hookEventName string, result model.HookResult) string
	NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string
}

// 🐳️DevcontainerSandboxProvider holds the data fields for a devcontainer sandbox provider record.
type DevcontainerSandboxProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *DevcontainerSandboxProvider) Kind() string { return "devcontainer" }

// ⚙️Configure holds the data fields for a Configure record.
func (p *DevcontainerSandboxProvider) Configure(repoRoot string) error { return nil }

// 🔌️AllEditorProviders returns all registered editor providers.
func AllEditorProviders() []EditorProvider {
	return []EditorProvider{
		&CopilotEditorProvider{},
		&CursorEditorProvider{},
		&WindsurfEditorProvider{},
		&ClaudeCodeEditorProvider{},
		&DroidEditorProvider{},
		&CodexEditorProvider{},
		&AntigravityEditorProvider{},
		&KiroEditorProvider{},
	}
}

// 💻️GetEditorProvider returns the editor provider for the given client slug.
func GetEditorProvider(client string) EditorProvider {
	for _, p := range AllEditorProviders() {
		if p.Kind() == client {
			return p
		}
	}
	return nil
}

// 🐙️DefaultManagementProvider returns the default management provider (GitHub).
func DefaultManagementProvider() ManagementProvider {
	return NewGitHubManagementProvider(NewSystemProcessRunner())
}

// 📌️DefaultVersionControlProvider returns the default version control provider (Git).
func DefaultVersionControlProvider() VersionControlProvider {
	return &GitVersionControlProvider{}
}

// 🐳️DefaultSandboxProvider returns the default sandbox provider (Devcontainer).
func DefaultSandboxProvider() SandboxProvider {
	return &DevcontainerSandboxProvider{}
}

// 💿️mgmtProvider holds the data fields for a mgmtProvider record.
var MgmtProvider ManagementProvider = DefaultManagementProvider()

// 🔷️GetManagementProvider holds the data fields for a GetManagementProvider record.
func GetManagementProvider() ManagementProvider { return MgmtProvider }

type McpClientKind string

const McpClientGeneric McpClientKind = "generic"

const McpClientCursor McpClientKind = "cursor"

const McpClientKiro McpClientKind = "kiro"

const McpClientCopilot McpClientKind = "copilot"

const McpClientClaude McpClientKind = "claude"

const McpClientCodex McpClientKind = "codex"

// 📂️ParseMcpClientKind maps a CLI/MCP profile slug to an MCP server kind.
func ParseMcpClientKind(raw string) (McpClientKind, error) {
	switch strings.ToLower(strings.TrimSpace(raw)) {
	case "", "generic", "client":
		return McpClientGeneric, nil
	case "cursor":
		return McpClientCursor, nil
	case "kiro":
		return McpClientKiro, nil
	case "copilot":
		return McpClientCopilot, nil
	case "claude":
		return McpClientClaude, nil
	case "codex":
		return McpClientCodex, nil
	default:
		return "", fmt.Errorf("unknown mcp kind %q (expected client, cursor, copilot, claude, codex, or kiro)", raw)
	}
}

// 🪝️HookClientForMcpKind maps an MCP entry binary to the hook client id used by ResolveHookEvent.
func HookClientForMcpKind(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "cursor-chat"
	case McpClientKiro:
		return "kiro-cli"
	case McpClientCopilot:
		return "copilot-chat"
	case McpClientClaude:
		return "claude-code"
	case McpClientCodex:
		return "codex"
	default:
		return ""
	}
}

// 🧾️resolveMcpTicketClient derives native ticket metadata from the MCP entrypoint.
func ResolveMcpTicketClient(kind McpClientKind, client string) string {
	if strings.TrimSpace(client) != "" {
		return client
	}
	return HookClientForMcpKind(kind)
}

// 📡️McpKindFromResolvedClient maps a validated ticket client slug to an MCP surface kind for plan/spec attachment.
func McpKindFromResolvedClient(client string) McpClientKind {
	switch strings.TrimSpace(client) {
	case "cursor-chat", "cursor":
		return McpClientCursor
	case "kiro-cli":
		return McpClientKiro
	case "copilot-chat":
		return McpClientCopilot
	case "claude-code":
		return McpClientClaude
	case "codex":
		return McpClientCodex
	default:
		return McpClientGeneric
	}
}

// 📜️McpServerName returns the MCP server identifier string for the given kind.
func McpServerName(kind McpClientKind) string {
	switch kind {
	case McpClientCursor, McpClientKiro, McpClientCopilot, McpClientClaude, McpClientCodex, McpClientGeneric, "":
		return "repo"
	default:
		return "repo"
	}
}

// #endregion 🔭️Providers
