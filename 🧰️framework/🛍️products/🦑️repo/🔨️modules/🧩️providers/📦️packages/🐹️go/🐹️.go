// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Package providers holds the composable provider integrations of the semio repository domain:
// issue/milestone management, version control, sandbox and the editor providers. Every external
// process goes through ProcessRunner, so a provider is exercised without gh or git on the machine.
package providers

import (
	bufio "bufio"
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	regexp "regexp"
	strconv "strconv"
	strings "strings"
	time "time"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏃️ProcessRunner

// 📨️ProcessRequest is one external process invocation; Argv[0] is the program.
type ProcessRequest struct {
	Argv  []string `json:"argv"`
	Cwd   string   `json:"cwd,omitempty"`
	Stdin string   `json:"stdin,omitempty"`
}

// 📬️ProcessOutcome is what a process left behind.
type ProcessOutcome struct {
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr"`
	Status int    `json:"status"`
}

// 🎞️ProcessExchange is one recorded exchange of a process transcript fixture.
type ProcessExchange struct {
	Argv   []string `json:"argv"`
	Stdin  string   `json:"stdin,omitempty"`
	Stdout string   `json:"stdout"`
	Stderr string   `json:"stderr"`
	Status int      `json:"status"`
}

// 🎬️ProcessTranscript is the ordered set of exchanges a recorded runner replays.
type ProcessTranscript struct {
	Exchanges []ProcessExchange `json:"exchanges"`
}

// 🏃️ProcessRunner is the single seam through which a provider touches the operating system.
type ProcessRunner interface {
	Run(request ProcessRequest) ProcessOutcome
	Issued() [][]string
}

// 💻️SystemProcessRunner runs the real thing through os/exec.
type SystemProcessRunner struct {
	issued [][]string
}

// 🆕️NewSystemProcessRunner returns a runner with an empty log.
func NewSystemProcessRunner() *SystemProcessRunner { return &SystemProcessRunner{} }

// ▶️Run executes the request and reports what the process left behind.
func (r *SystemProcessRunner) Run(request ProcessRequest) ProcessOutcome {
	r.issued = append(r.issued, append([]string(nil), request.Argv...))
	if len(request.Argv) == 0 {
		return ProcessOutcome{Stderr: "empty argv", Status: 127}
	}
	cmd := exec.Command(request.Argv[0], request.Argv[1:]...)
	if request.Cwd != "" {
		cmd.Dir = request.Cwd
	}
	if request.Stdin != "" {
		cmd.Stdin = strings.NewReader(request.Stdin)
	}
	var stdout, stderr strings.Builder
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	status := 0
	if err != nil {
		var exitErr *exec.ExitError
		if ok := asExitError(err, &exitErr); ok {
			status = exitErr.ExitCode()
		} else {
			return ProcessOutcome{Stdout: stdout.String(), Stderr: err.Error(), Status: 127}
		}
	}
	return ProcessOutcome{Stdout: stdout.String(), Stderr: stderr.String(), Status: status}
}

// 📜️Issued lists every argv this runner issued, in order.
func (r *SystemProcessRunner) Issued() [][]string { return r.issued }

func asExitError(err error, target **exec.ExitError) bool {
	exitErr, ok := err.(*exec.ExitError)
	if ok {
		*target = exitErr
	}
	return ok
}

// 🎞️RecordedProcessRunner replays a recorded transcript: the first not-yet-consumed exchange whose
// argv matches wins, so a provider's optional calls may be omitted from the fixture.
type RecordedProcessRunner struct {
	transcript ProcessTranscript
	consumed   []bool
	issued     [][]string
}

// 🆕️NewRecordedProcessRunner wraps a transcript.
func NewRecordedProcessRunner(transcript ProcessTranscript) *RecordedProcessRunner {
	return &RecordedProcessRunner{transcript: transcript, consumed: make([]bool, len(transcript.Exchanges))}
}

// ▶️Run replays the matching exchange, or reports exit 127 when the transcript does not carry it.
func (r *RecordedProcessRunner) Run(request ProcessRequest) ProcessOutcome {
	r.issued = append(r.issued, append([]string(nil), request.Argv...))
	for index, exchange := range r.transcript.Exchanges {
		if r.consumed[index] || !equalArgv(exchange.Argv, request.Argv) {
			continue
		}
		r.consumed[index] = true
		return ProcessOutcome{Stdout: exchange.Stdout, Stderr: exchange.Stderr, Status: exchange.Status}
	}
	return ProcessOutcome{Stderr: fmt.Sprintf("no recorded exchange for %q", request.Argv), Status: 127}
}

// 📜️Issued lists every argv this runner issued, in order.
func (r *RecordedProcessRunner) Issued() [][]string { return r.issued }

func equalArgv(left, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

// 📥️ParseProcessTranscript parses a transcript fixture into the shape a recorded runner replays.
func ParseProcessTranscript(source []byte) (ProcessTranscript, error) {
	var transcript ProcessTranscript
	if err := json.Unmarshal(source, &transcript); err != nil {
		return ProcessTranscript{}, fmt.Errorf("failed to parse process transcript: %w", err)
	}
	return transcript, nil
}

// #endregion 🏃️ProcessRunner

// #region 🚚️Split

// #region 🔭️Provider Interfaces

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

// #endregion 🔭️Provider Interfaces

// #region 🎄️GitHub Management Provider

// 🔌️GitHubManagementProvider talks to the GitHub CLI through a ProcessRunner, so every call it
// makes is a recordable (argv, cwd) → outcome exchange and no scenario needs the gh binary.
type GitHubManagementProvider struct {
	runner ProcessRunner
}

// 🆕️NewGitHubManagementProvider binds a provider to the runner every gh invocation goes through.
func NewGitHubManagementProvider(runner ProcessRunner) *GitHubManagementProvider {
	if runner == nil {
		runner = NewSystemProcessRunner()
	}
	return &GitHubManagementProvider{runner: runner}
}

// 📜️Issued lists every argv this provider issued through its runner, in order.
func (p *GitHubManagementProvider) Issued() [][]string {
	if p.runner == nil {
		return nil
	}
	return p.runner.Issued()
}

// 🐙️gh issues one gh invocation and reports stdout, stderr and the exit status.
func (p *GitHubManagementProvider) gh(args []string) (string, string, int) {
	if p.runner == nil {
		p.runner = NewSystemProcessRunner()
	}
	outcome := p.runner.Run(ProcessRequest{Argv: append([]string{"gh"}, args...)})
	return outcome.Stdout, outcome.Stderr, outcome.Status
}

// 💿️Kind names the management provider.
func (p *GitHubManagementProvider) Kind() string { return "github" }

// 💻️NullManagementProvider is a no-operation implementation of ManagementProvider.
type NullManagementProvider struct{}

func (p *NullManagementProvider) Kind() string { return "none" }

// ⚙️Configure holds the data fields for a Configure record.
func (p *NullManagementProvider) Configure(repoRoot string) error { return nil }

// 🟩️CreateIssue holds the data fields for a CreateIssue record.
func (p *NullManagementProvider) CreateIssue(title, body string, milestone *int) (string, error) {
	return "", nil
}

// 🟦️CloseIssue holds the data fields for a CloseIssue record.
func (p *NullManagementProvider) CloseIssue(issueURL string) error { return nil }

func (p *NullManagementProvider) ReopenIssue(issueURL string) error { return nil }

// 🟪️DeleteIssue holds the data fields for a DeleteIssue record.
func (p *NullManagementProvider) DeleteIssue(issueURLOrNumber string) error { return nil }

// 🟫️UpdateIssueTitle holds the data fields for a UpdateIssueTitle record.
func (p *NullManagementProvider) UpdateIssueTitle(issueURL, title string) error { return nil }

func (p *NullManagementProvider) UpdateIssueBody(issueURL, body string) error { return nil }

func (p *NullManagementProvider) GetIssueDetails(issueURL string) (*ManagementIssue, error) {
	return nil, nil
}

// 💠️GetIssueNodeID holds the data fields for a GetIssueNodeID record.
func (p *NullManagementProvider) GetIssueNodeID(issueURL string) (string, error) { return "", nil }

// 🔳️GetIssueParentURL holds the data fields for a GetIssueParentURL record.
func (p *NullManagementProvider) GetIssueParentURL(issueURL string) (string, error) { return "", nil }

// 🔲️AddComment holds the data fields for a AddComment record.
func (p *NullManagementProvider) AddComment(issueURL, comment string) error { return nil }

func (p *NullManagementProvider) AddLabels(issueURL string, labels []string) error { return nil }

// ➖️RemoveLabels holds the data fields for a RemoveLabels record.
func (p *NullManagementProvider) RemoveLabels(issueURL string, labels []string) error { return nil }

// ▪️AddIssueToProject holds the data fields for a AddIssueToProject record.
func (p *NullManagementProvider) AddIssueToProject(issueURL string) {}

func (p *NullManagementProvider) AssignIssueToCurrentUser(issueURL string) {}

// ▫️AddSubIssue holds the data fields for a AddSubIssue record.
func (p *NullManagementProvider) AddSubIssue(parentIssueURL, childIssueURL string) error {
	return nil
}

// ◾UpdateIssueMilestone holds the data fields for a UpdateIssueMilestone record.
func (p *NullManagementProvider) UpdateIssueMilestone(issueURL, milestoneTitle string) error {
	return nil
}

// ◽ClearIssueMilestone holds the data fields for a ClearIssueMilestone record.
func (p *NullManagementProvider) ClearIssueMilestone(issueURL string) error { return nil }

// ◻CreateMilestone holds the data fields for a CreateMilestone record.
func (p *NullManagementProvider) CreateMilestone(title, description string) (int, error) {
	return 0, nil
}

// ◼UpdateMilestone holds the data fields for a UpdateMilestone record.
func (p *NullManagementProvider) UpdateMilestone(number int, title, description, state, dueOn string) error {
	return nil
}

// 🔵️DeleteMilestone holds the data fields for a DeleteMilestone record.
func (p *NullManagementProvider) DeleteMilestone(number int) error { return nil }

// 🔴️GetMilestone holds the data fields for a GetMilestone record.
func (p *NullManagementProvider) GetMilestone(number int) (*ManagementMilestone, error) {
	return nil, nil
}

// 🟠️GetMilestoneTitle holds the data fields for a GetMilestoneTitle record.
func (p *NullManagementProvider) GetMilestoneTitle(number int) (string, error) { return "", nil }

func (p *NullManagementProvider) FindMilestoneByTitle(title string) (*ManagementMilestone, error) {
	return nil, nil
}

// 🟡️ListIssuesForLabelSync holds the data fields for a ListIssuesForLabelSync record.
func (p *NullManagementProvider) ListIssuesForLabelSync() ([]ManagementIssue, error) {
	return nil, nil
}

// ⏹️ListOpenIssuesWithLabel holds the data fields for a ListOpenIssuesWithLabel record.
func (p *NullManagementProvider) ListOpenIssuesWithLabel(label string) ([]string, error) {
	return nil, nil
}

func (p *NullManagementProvider) ListRepoLabels() ([]ManagementLabel, error) { return nil, nil }

// 🟢️CreateRepoLabel holds the data fields for a CreateRepoLabel record.
func (p *NullManagementProvider) CreateRepoLabel(name string) error { return nil }

// 🟣️DeleteRepoLabel holds the data fields for a DeleteRepoLabel record.
func (p *NullManagementProvider) DeleteRepoLabel(name string) error { return nil }

// 📜️SyncRepoLabelCatalog holds the data fields for a SyncRepoLabelCatalog record.
func (p *NullManagementProvider) SyncRepoLabelCatalog(validLabels map[string]bool) error {
	return nil
}

func (p *NullManagementProvider) CreateGoalIssue(title, description string, milestone *int) (string, error) {
	return "", nil
}

// 🟤️UpdateGoalIssue holds the data fields for a UpdateGoalIssue record.
func (p *NullManagementProvider) UpdateGoalIssue(issueURL, title, description string) error {
	return nil
}

// ⚪️GetCurrentUser holds the data fields for a GetCurrentUser record.
func (p *NullManagementProvider) GetCurrentUser() string { return "" }

// #endregion 🎄️GitHub Management Provider

// #region 🔒️Git Version Control Provider

// 🔌️GitVersionControlProvider holds the data fields for a git version control provider record.
type GitVersionControlProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *GitVersionControlProvider) Kind() string { return "git" }

// 🌐️RepoURL holds the data fields for a RepoURL record.
func (p *GitVersionControlProvider) RepoURL() (string, error) {
	out, err := exec.Command("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// ⚙️Configure holds the data fields for a Configure record.
func (p *GitVersionControlProvider) Configure(repoRoot string) error { return nil }

// 💾️Checkpoint holds the data fields for a Checkpoint record.
func (p *GitVersionControlProvider) Checkpoint(repoRoot string, description string) (string, error) {
	if _, _, exitCode := workspace.ExecCommand("git", []string{"diff", "--cached", "--quiet"}, repoRoot); exitCode == 0 {
		if err := p.StageAll(repoRoot); err != nil {
			return "", fmt.Errorf("stage all failed: %w", err)
		}
	}
	args := []string{"commit", "-m", description}
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit failed (exit %d): %s", exitCode, strings.TrimSpace(stderr))
	}
	_ = stdout
	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", fmt.Errorf("failed to get checkpoint sha after checkpoint: %w", err)
	}
	return sha, nil
}

func (p *GitVersionControlProvider) CurrentCheckpoint(repoRoot string) (string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"rev-parse", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// ✔️Checkin holds the data fields for a Checkin record.
func (p *GitVersionControlProvider) Checkin(repoRoot string, contributor string) error {
	contributorBranch := contributor + "/latest"

	_, stderr, exitCode := workspace.ExecCommand("git", []string{"fetch", "origin", "main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fetch origin main failed: %s", strings.TrimSpace(stderr))
	}

	currentBranch, err := p.CurrentBranch(repoRoot)
	if err != nil {
		return err
	}

	if currentBranch != contributorBranch {
		_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", contributorBranch}, repoRoot)
		if exitCode != 0 {

			_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", "-c", contributorBranch}, repoRoot)
			if exitCode != 0 {
				return fmt.Errorf("git switch to %s failed: %s", contributorBranch, strings.TrimSpace(stderr))
			}
		}
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"merge", "--ff-only", "origin/main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fast-forward merge to main failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔷️Checkout holds the data fields for a Checkout record.
func (p *GitVersionControlProvider) Checkout(repoRoot string, contributor string, description string) (string, error) {
	contributorBranch := contributor + "/latest"
	now := time.Now().UTC()
	archiveBranch := fmt.Sprintf("%s/%04d/%02d/%02d", contributor, now.Year(), now.Month(), now.Day())

	_, stderr, exitCode := workspace.ExecCommand("git", []string{"branch", archiveBranch, contributorBranch}, repoRoot)
	if exitCode != 0 {

		for i := 2; i <= 99; i++ {
			candidate := fmt.Sprintf("%s-%d", archiveBranch, i)
			_, _, ec := workspace.ExecCommand("git", []string{"branch", candidate, contributorBranch}, repoRoot)
			if ec == 0 {
				archiveBranch = candidate
				break
			}
		}
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", "main"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git switch to main failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"merge", "--squash", contributorBranch}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git squash merge failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"commit", "-m", description}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit squash merge failed: %s", strings.TrimSpace(stderr))
	}

	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", err
	}
	return sha, nil
}

// 🌿️CurrentBranch holds the data fields for a CurrentBranch record.
func (p *GitVersionControlProvider) CurrentBranch(repoRoot string) (string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"rev-parse", "--abbrev-ref", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse --abbrev-ref HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 📄️StagedFiles holds the data fields for a StagedFiles record.
func (p *GitVersionControlProvider) StagedFiles(repoRoot string) ([]string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"diff", "--cached", "--name-only"}, repoRoot)
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff --cached --name-only failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.ReplaceAll(stdout, "\r\n", "\n"), "\n") {
		if line != "" {
			files = append(files, line)
		}
	}
	return files, nil
}

// 🏷️StageAll holds the data fields for a StageAll record.
func (p *GitVersionControlProvider) StageAll(repoRoot string) error {
	_, stderr, exitCode := workspace.ExecCommand("git", []string{"add", "-A"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git add -A failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// #endregion 🔒️Git Version Control Provider

// #region ⛑️Devcontainer Sandbox Provider

// 🐳️DevcontainerSandboxProvider holds the data fields for a devcontainer sandbox provider record.
type DevcontainerSandboxProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *DevcontainerSandboxProvider) Kind() string { return "devcontainer" }

// ⚙️Configure holds the data fields for a Configure record.
func (p *DevcontainerSandboxProvider) Configure(repoRoot string) error { return nil }

// #endregion ⛑️Devcontainer Sandbox Provider

// #region 🎆️Editor Providers

// 🔌️CopilotEditorProvider holds the data fields for a copilot editor provider record.
type CopilotEditorProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *CopilotEditorProvider) Kind() string { return "copilot-chat" }

// 📡️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CopilotEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCopilotEvent(nativeEvent, toolKind)
}

// 📋️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CopilotEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	return FormatVSCodeHookOutput(hookEventName, result)
}

// 🔷️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CopilotEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return vsCodeEventFromHookEvent(event, parentInfo)
}

// 📝️CursorEditorProvider holds the data fields for a cursor editor provider record.
type CursorEditorProvider struct{}

// 🏷️Kind holds the data fields for a Kind record.
func (p *CursorEditorProvider) Kind() string { return "cursor-chat" }

// 🔸️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CursorEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCursorEvent(nativeEvent, toolKind)
}

// 🔺️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CursorEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🔻️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CursorEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💻️WindsurfEditorProvider holds the data fields for a windsurf editor provider record.
type WindsurfEditorProvider struct{}

// 🟥️Kind holds the data fields for a Kind record.
func (p *WindsurfEditorProvider) Kind() string { return "windsurf-chat" }

// 🟨️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *WindsurfEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveWindsurfEvent(nativeEvent, toolKind)
}

// 🟩️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *WindsurfEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟦️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *WindsurfEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💠️ClaudeCodeEditorProvider holds the data fields for a claude code editor provider record.
type ClaudeCodeEditorProvider struct{}

// 🔳️Kind holds the data fields for a Kind record.
func (p *ClaudeCodeEditorProvider) Kind() string { return "claude-code" }

// ▪️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *ClaudeCodeEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// ▫️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *ClaudeCodeEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// ◾NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *ClaudeCodeEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ◼DroidEditorProvider holds the data fields for a droid editor provider record.
type DroidEditorProvider struct{}

// 🔵️Kind holds the data fields for a Kind record.
func (p *DroidEditorProvider) Kind() string { return "droid" }

// 🟠️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *DroidEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🟡️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *DroidEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟢️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *DroidEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ⚪️CodexEditorProvider holds the data fields for a codex editor provider record.
type CodexEditorProvider struct{}

// ⚫️Kind holds the data fields for a Kind record.
func (p *CodexEditorProvider) Kind() string { return "codex" }

// 🩶️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CodexEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🩷️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CodexEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 💜️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CodexEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💛️AntigravityEditorProvider holds the data fields for an antigravity editor provider record.
type AntigravityEditorProvider struct{}

// 🧡️Kind holds the data fields for a Kind record.
func (p *AntigravityEditorProvider) Kind() string { return "antigravity-chat" }

// 🤍️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *AntigravityEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🖤️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *AntigravityEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🤎️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *AntigravityEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💝️KiroEditorProvider holds the data fields for a kiro editor provider record.
type KiroEditorProvider struct{}

func (p *KiroEditorProvider) Kind() string { return "kiro-cli" }

func (p *KiroEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveKiroEvent(nativeEvent, toolKind)
}

func (p *KiroEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

func (p *KiroEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// #endregion 🎆️Editor Providers

// #region 🎖️Provider Registry

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

// #endregion 🎖️Provider Registry

// #region 📋️Tickets

// 📌️ghGetMilestoneTitle holds the data fields for a ghGetMilestoneTitle record.
func (p *GitHubManagementProvider) GetMilestoneTitle(number int) (string, error) {
	stdout, stderr, exitCode := p.gh([]string{"api", fmt.Sprintf("repos/{owner}/{repo}/milestones/%d", number), "--jq", ".title"})
	if exitCode != 0 {
		return "", fmt.Errorf("gh api milestone failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 🔗️ghExtractIssueURL parses a GitHub issue URL from `gh issue create` output.
func ExtractIssueURL(output string) string {
	output = strings.TrimSpace(output)
	if output == "" {
		return ""
	}
	if fields := strings.Fields(output); len(fields) > 0 && strings.HasPrefix(fields[0], "https://") {
		return fields[0]
	}
	const prefix = "https://github.com/"
	if idx := strings.Index(output, prefix); idx >= 0 {
		rest := output[idx:]
		if end := strings.IndexAny(rest, " \t\r\n"); end > 0 {
			rest = rest[:end]
		}
		if strings.Contains(rest, "/issues/") {
			return rest
		}
	}
	return ""
}

// 🔸️ghCreateIssue holds the data fields for a ghCreateIssue record.
func (p *GitHubManagementProvider) CreateIssue(title, body string, milestone *int) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", body, "--label", "ticket"}
	if milestone != nil {
		milestoneTitle, err := p.GetMilestoneTitle(*milestone)
		if err != nil {
			workspace.WriteWarningf("could not resolve milestone %d, creating issue without milestone: %v", *milestone, err)
		} else {
			args = append(args, "--milestone", milestoneTitle)
		}
	}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := ExtractIssueURL(stdout)
	if issueURL == "" {
		issueURL = ExtractIssueURL(stderr)
	}
	if issueURL == "" {
		return "", fmt.Errorf("gh issue create succeeded but no issue url in output: stdout=%q stderr=%q", strings.TrimSpace(stdout), strings.TrimSpace(stderr))
	}
	p.AddIssueToProject(issueURL)
	p.AssignIssueToCurrentUser(issueURL)
	return issueURL, nil
}

// 🛠️buildTechnologyLinkArgs holds the data fields for a buildTechnologyLinkArgs record.
func buildTechnologyLinkArgs(issueURL string) []string {
	return []string{"project", "item-add", "2", "--owner", "usalu", "--url", issueURL}
}

// ➕️ghAddIssueToProject holds the data fields for a ghAddIssueToProject record.
func (p *GitHubManagementProvider) AddIssueToProject(issueURL string) {
	if issueURL == "" {
		return
	}
	p.gh(buildTechnologyLinkArgs(issueURL))
}

// 👤️ghGetCurrentUser holds the data fields for a ghGetCurrentUser record.
func (p *GitHubManagementProvider) GetCurrentUser() string {
	stdout, _, exitCode := p.gh([]string{"api", "user", "--jq", ".login"})
	if exitCode != 0 {
		return ""
	}
	return strings.TrimSpace(stdout)
}

// 🔺️ghAssignIssueToCurrentUser holds the data fields for a ghAssignIssueToCurrentUser record.
func (p *GitHubManagementProvider) AssignIssueToCurrentUser(issueURL string) {
	if issueURL == "" {
		return
	}
	user := p.GetCurrentUser()
	if user == "" {
		return
	}
	p.gh([]string{"issue", "edit", issueURL, "--add-assignee", user})
}

// 💬️ghAddComment holds the data fields for a ghAddComment record.
func (p *GitHubManagementProvider) AddComment(issueURL, comment string) error {
	tmp, err := os.CreateTemp("", "gh-comment-*.md")
	if err != nil {
		return fmt.Errorf("create comment temp file: %w", err)
	}
	tmpPath := tmp.Name()
	defer os.Remove(tmpPath)
	if _, err := tmp.WriteString(comment); err != nil {
		tmp.Close()
		return fmt.Errorf("write comment temp file: %w", err)
	}
	if err := tmp.Close(); err != nil {
		return fmt.Errorf("close comment temp file: %w", err)
	}
	args := []string{"issue", "comment", issueURL, "--body-file", tmpPath}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue comment failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🏷️ghAddLabels holds the data fields for a ghAddLabels record.
func (p *GitHubManagementProvider) AddLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--add-label", label)
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 📪️ghCloseIssue holds the data fields for a ghCloseIssue record.
func (p *GitHubManagementProvider) CloseIssue(issueURL string) error {
	args := []string{"issue", "close", issueURL}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue close failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔓️ghReopenIssue holds the data fields for a ghReopenIssue record.
func (p *GitHubManagementProvider) ReopenIssue(issueURL string) error {
	args := []string{"issue", "reopen", issueURL}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue reopen failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// ⬛️ghUpdateIssueTitle holds the data fields for a ghUpdateIssueTitle record.
func (p *GitHubManagementProvider) UpdateIssueTitle(issueURL, title string) error {
	args := []string{"issue", "edit", issueURL, "--title", title}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// ⬜️ghUpdateIssueBody holds the data fields for a ghUpdateIssueBody record.
func (p *GitHubManagementProvider) UpdateIssueBody(issueURL, body string) error {
	args := []string{"issue", "edit", issueURL, "--body", body}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit body failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔶️GetIssueDetails parses `gh issue view` into a ManagementIssue.
func (p *GitHubManagementProvider) GetIssueDetails(issueURL string) (*ManagementIssue, error) {
	args := []string{"issue", "view", issueURL, "--json", "url,state,milestone,labels,title,body"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue view failed: %s", strings.TrimSpace(stderr))
	}
	issue, err := ParseManagementIssue([]byte(stdout))
	if err != nil {
		return nil, err
	}
	return &issue, nil
}

func (p *GitHubManagementProvider) GetMilestone(number int) (*ManagementMilestone, error) {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number)}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh api milestone get failed: %s", strings.TrimSpace(stderr))
	}
	milestone, err := ParseManagementMilestone([]byte(stdout))
	if err != nil {
		return nil, err
	}
	return &milestone, nil
}

// 🟦️ghFindMilestoneByTitle holds the data fields for a ghFindMilestoneByTitle record.
func (p *GitHubManagementProvider) FindMilestoneByTitle(title string) (*ManagementMilestone, error) {
	if strings.TrimSpace(title) == "" {
		return nil, nil
	}
	args := []string{"api", "repos/:owner/:repo/milestones", "-X", "GET", "-F", "state=all", "-F", "per_page=100", "--paginate", "--jq", ".[]"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh api milestone list failed: %s", strings.TrimSpace(stderr))
	}
	return ScanManagementMilestones(stdout, title)
}

// 🟪️ghUpdateIssueMilestone holds the data fields for a ghUpdateIssueMilestone record.
func (p *GitHubManagementProvider) UpdateIssueMilestone(issueURL, milestoneTitle string) error {
	if strings.TrimSpace(milestoneTitle) == "" {
		return fmt.Errorf("milestone title is required")
	}
	args := []string{"issue", "edit", issueURL, "--milestone", milestoneTitle}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit milestone failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🟫️ghClearIssueMilestone holds the data fields for a ghClearIssueMilestone record.
func (p *GitHubManagementProvider) ClearIssueMilestone(issueURL string) error {
	issueNodeID, err := p.GetIssueNodeID(issueURL)
	if err != nil {
		return err
	}
	query := `mutation($issueId: ID!) {
		updateIssue(input: { id: $issueId, milestoneId: null }) {
			issue { id }
		}
	}`
	args := []string{"api", "graphql",
		"-f", fmt.Sprintf("issueId=%s", issueNodeID),
		"-f", fmt.Sprintf("query=%s", query),
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh api updateIssue clear milestone failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🚚️ghRemoveLabels holds the data fields for a ghRemoveLabels record.
func (p *GitHubManagementProvider) RemoveLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--remove-label", label)
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit remove-label failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 💠️ghListIssuesForLabelSync holds the data fields for a ghListIssuesForLabelSync record.
func (p *GitHubManagementProvider) ListIssuesForLabelSync() ([]ManagementIssue, error) {
	args := []string{"issue", "list", "--state", "all", "--json", "url,labels", "--limit", "1000"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue list for label sync failed: %s", strings.TrimSpace(stderr))
	}
	return ParseManagementIssues([]byte(stdout))
}

// 🔳️ghListRepoLabels holds the data fields for a ghListRepoLabels record.
func (p *GitHubManagementProvider) ListRepoLabels() ([]ManagementLabel, error) {
	args := []string{"label", "list", "--json", "name", "--limit", "1000"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh label list failed: %s", strings.TrimSpace(stderr))
	}
	return ParseManagementLabels([]byte(stdout))
}

func (p *GitHubManagementProvider) CreateRepoLabel(name string) error {
	if strings.TrimSpace(name) == "" {
		return fmt.Errorf("label name is required")
	}
	args := []string{"label", "create", name, "--color", "1d76db", "--description", "Compose technology or bundle"}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh label create failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🗑️ghDeleteRepoLabel holds the data fields for a ghDeleteRepoLabel record.
func (p *GitHubManagementProvider) DeleteRepoLabel(name string) error {
	if strings.TrimSpace(name) == "" {
		return fmt.Errorf("label name is required")
	}
	args := []string{"label", "delete", name, "--yes"}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh label delete failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func (p *GitHubManagementProvider) SyncRepoLabelCatalog(validLabels map[string]bool) error {
	labels, err := p.ListRepoLabels()
	if err != nil {
		return err
	}
	existing := make(map[string]bool, len(labels))
	for _, label := range labels {
		existing[label.Name] = true
	}

	for label := range validLabels {
		if !strings.HasPrefix(label, "@") {
			continue
		}
		if existing[label] {
			continue
		}
		fmt.Printf("Creating missing GitHub label %s...\n", label)
		if err := p.CreateRepoLabel(label); err != nil {
			workspace.WriteWarningf("Failed to create GitHub label %s: %v", label, err)
		}
	}

	for label := range existing {
		if !strings.HasPrefix(label, "@") {
			continue
		}
		if validLabels[label] {
			continue
		}
		fmt.Printf("Deleting invalid GitHub label %s...\n", label)
		if err := p.DeleteRepoLabel(label); err != nil {
			workspace.WriteWarningf("Failed to delete GitHub label %s: %v", label, err)
		}
	}
	return nil
}

// ⏹️ghListOpenIssuesWithLabel holds the data fields for a ghListOpenIssuesWithLabel record.
func (p *GitHubManagementProvider) ListOpenIssuesWithLabel(label string) ([]string, error) {
	args := []string{"issue", "list", "--label", label, "--state", "open", "--json", "url", "--limit", "1000"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue list failed: %s", strings.TrimSpace(stderr))
	}
	var issues []struct {
		URL string `json:"url"`
	}
	if err := json.Unmarshal([]byte(stdout), &issues); err != nil {
		return nil, fmt.Errorf("failed to parse gh issue list output: %w", err)
	}
	urls := make([]string, len(issues))
	for i, issue := range issues {
		urls[i] = issue.URL
	}
	return urls, nil
}

// 🔸️GhCreateIssue opens a ticket issue through the default management provider.
func GhCreateIssue(title, body string, milestone *int) (string, error) {
	return NewGitHubManagementProvider(nil).CreateIssue(title, body, milestone)
}

// 📪️GhCloseIssue closes an issue through the default management provider.
func GhCloseIssue(issueURL string) error {
	return NewGitHubManagementProvider(nil).CloseIssue(issueURL)
}

// 🌿️GhGetIssueNodeID reads an issue node id through the default management provider.
func GhGetIssueNodeID(issueURL string) (string, error) {
	return NewGitHubManagementProvider(nil).GetIssueNodeID(issueURL)
}

// #endregion 📋️Tickets

// #region 🧬️Missing Utilities

// 🌳️GitIndexRef is the git ref for the staging index. Used for unstaged-only diffs (index vs working tree).
// ⏹️Requirements: ticket close and interaction finish use only unstaged diffs; git diff runs without tree-ish for index vs working tree.
const GitIndexRef = ":0"

// 📨️GetGitDiffLines MUST retrieve the requested value or return an error.
// GetGitDiffLines retrieves and returns the git diff lines.
// ✔️For unstaged-only diffs use baseCheckpoint GitIndexRef (index vs working tree).
func GetGitDiffLines(baseCheckpoint, headCheckpoint string, paths []string) (map[string]*model.DiffLines, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := BuildGitDiffArgs("-U0", baseCheckpoint, headCheckpoint, paths)
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff failed: %s", strings.TrimSpace(stderr))
	}
	result := make(map[string]*model.DiffLines)
	var currentFile string
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Fields(line)
			if len(parts) >= 4 {
				newPath := strings.TrimPrefix(parts[3], "b/")
				currentFile = newPath
				if result[currentFile] == nil {
					result[currentFile] = &model.DiffLines{Added: []int{}, Removed: []int{}}
				}
			}
		} else if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &model.DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

				newStart, _ := strconv.Atoi(match[3])
				newCount := 1
				if match[4] != "" {
					newCount, _ = strconv.Atoi(match[4])
				}
				for i := 0; i < newCount; i++ {
					result[currentFile].Added = append(result[currentFile].Added, newStart+i)
				}
			}
		}
	}
	return result, nil
}

// 🧱️BuildGitDiffArgs MUST construct and return the fully initialized result.
// BuildGitDiffArgs constructs and returns the git diff args.
// 🌳️GitIndexRef as baseCheckpoint yields unstaged-only diff (index vs working tree) with no tree-ish.
func BuildGitDiffArgs(flag, baseCheckpoint, headCheckpoint string, paths []string) []string {
	if baseCheckpoint == GitIndexRef {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M"}
		}
		return append([]string{"diff", flag, "-M", "--"}, paths...)
	}
	if headCheckpoint == "" {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M", baseCheckpoint}
		}
		return append([]string{"diff", flag, "-M", baseCheckpoint, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "-M", baseCheckpoint, headCheckpoint}
	}
	return append([]string{"diff", flag, "-M", baseCheckpoint, headCheckpoint, "--"}, paths...)
}

// 🔺️GitDiffStatus holds the data fields for a git diff status record.
type GitDiffStatus struct {
	Status string
	From   string
	To     string
}

// 🔻️GetGitDiffStatus MUST retrieve the requested value or return an error.
// 🐙️GetGitDiffStatus retrieves and returns the git diff status.
func GetGitDiffStatus(baseCheckpoint, headCheckpoint string, paths []string) ([]GitDiffStatus, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := []string{"diff", "--name-status", "-M"}
	if baseCheckpoint != GitIndexRef {
		args = append(args, baseCheckpoint)
		if headCheckpoint != "" {
			args = append(args, headCheckpoint)
		}
	}
	if len(paths) > 0 {
		args = append(args, "--")
		args = append(args, paths...)
	}
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff status failed: %s", strings.TrimSpace(stderr))
	}
	var results []GitDiffStatus
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		status := strings.TrimSpace(parts[0])
		if strings.HasPrefix(status, "R") && len(parts) >= 3 {
			results = append(results, GitDiffStatus{Status: "renamed", From: parts[1], To: parts[2]})
			continue
		}
		file := parts[1]
		switch status {
		case "A":
			results = append(results, GitDiffStatus{Status: "added", To: file})
		case "D":
			results = append(results, GitDiffStatus{Status: "deleted", From: file})
		case "M":
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		default:
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		}
	}
	return results, nil
}

// #endregion 🧬️Missing Utilities

// #region ❄️Goals

// 💿️ghCreateMilestone holds the data fields for a ghCreateMilestone record.
func (p *GitHubManagementProvider) CreateMilestone(title, description string) (int, error) {
	args := []string{"api", "repos/:owner/:repo/milestones", "-f", fmt.Sprintf("title=%s", title), "-f", fmt.Sprintf("description=%s", description), "--jq", ".number"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return 0, fmt.Errorf("gh api milestone create failed: %s", stderr)
	}
	num, _ := strconv.Atoi(strings.TrimSpace(stdout))
	return num, nil
}

// 🔁️ghUpdateMilestone holds the data fields for a ghUpdateMilestone record.
func (p *GitHubManagementProvider) UpdateMilestone(number int, title, description, state, dueOn string) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "PATCH"}
	if title != "" {
		args = append(args, "-f", fmt.Sprintf("title=%s", title))
	}
	if description != "" {
		args = append(args, "-f", fmt.Sprintf("description=%s", description))
	}
	if state != "" {
		args = append(args, "-f", fmt.Sprintf("state=%s", state))
	}
	if dueOn != "" {

		if !strings.Contains(dueOn, "T") {
			dueOn = dueOn + "T00:00:00Z"
		}
		args = append(args, "-f", fmt.Sprintf("due_on=%s", dueOn))
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone update failed: %s", stderr)
	}
	return nil
}

// 🗑️ghDeleteMilestone holds the data fields for a ghDeleteMilestone record.
func (p *GitHubManagementProvider) DeleteMilestone(number int) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "DELETE"}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone delete failed: %s", stderr)
	}
	return nil
}

// 🆕️ghCreateGoalIssue holds the data fields for a ghCreateGoalIssue record.
func (p *GitHubManagementProvider) CreateGoalIssue(title, description string, milestone *int) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", description, "--label", "goal"}
	if milestone != nil {
		milestoneTitle, err := p.GetMilestoneTitle(*milestone)
		if err != nil {
			return "", fmt.Errorf("could not resolve milestone %d: %w", *milestone, err)
		}
		args = append(args, "--milestone", milestoneTitle)
	}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := strings.TrimSpace(stdout)
	if issueURL != "" {
		p.AddIssueToProject(issueURL)
	}
	return issueURL, nil
}

// 🔷️ghUpdateGoalIssue holds the data fields for a ghUpdateGoalIssue record.
func (p *GitHubManagementProvider) UpdateGoalIssue(issueURL, title, description string) error {
	args := []string{"issue", "edit", issueURL}
	if title != "" {
		args = append(args, "--title", title)
	}
	if description != "" {
		args = append(args, "--body", description)
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func (p *GitHubManagementProvider) GetIssueNodeID(issueURL string) (string, error) {
	args := []string{"issue", "view", issueURL, "--json", "id", "--jq", ".id"}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue view failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 🌐️ghGetIssueParentURL holds the data fields for a ghGetIssueParentURL record.
func (p *GitHubManagementProvider) GetIssueParentURL(issueURL string) (string, error) {
	query := `query($url: URI!) {
		resource(url: $url) {
			... on Issue {
				parent {
					url
				}
			}
		}
	}`
	args := []string{"api", "graphql",
		"-f", fmt.Sprintf("url=%s", issueURL),
		"-f", fmt.Sprintf("query=%s", query),
	}
	stdout, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return "", fmt.Errorf("gh api issue parent query failed: %s", strings.TrimSpace(stderr))
	}
	var response struct {
		Data struct {
			Resource *struct {
				Parent *struct {
					URL string `json:"url"`
				} `json:"parent"`
			} `json:"resource"`
		} `json:"data"`
	}
	if err := json.Unmarshal([]byte(stdout), &response); err != nil {
		return "", fmt.Errorf("failed to parse issue parent response: %w", err)
	}
	if response.Data.Resource == nil || response.Data.Resource.Parent == nil {
		return "", nil
	}
	return response.Data.Resource.Parent.URL, nil
}

// ➕️ghAddSubIssue holds the data fields for a ghAddSubIssue record.
func (p *GitHubManagementProvider) AddSubIssue(parentIssueURL, childIssueURL string) error {
	parentNodeID, err := p.GetIssueNodeID(parentIssueURL)
	if err != nil {
		return fmt.Errorf("failed to get parent node ID: %w", err)
	}
	childNodeID, err := p.GetIssueNodeID(childIssueURL)
	if err != nil {
		return fmt.Errorf("failed to get child node ID: %w", err)
	}
	query := `mutation($parentId: ID!, $childId: ID!) {
		addSubIssue(input: { issueId: $parentId, subIssueId: $childId }) {
			subIssue { url }
		}
	}`
	args := []string{"api", "graphql",
		"-H", "GraphQL-Features:sub_issues",
		"-f", fmt.Sprintf("parentId=%s", parentNodeID),
		"-f", fmt.Sprintf("childId=%s", childNodeID),
		"-f", fmt.Sprintf("query=%s", query),
	}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh api addSubIssue failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// #endregion ❄️Goals

// #region ⚙️Types

// #endregion ❄️Goals
// 📦️ghDeleteIssue holds the data fields for a ghDeleteIssue record.
func (p *GitHubManagementProvider) DeleteIssue(issueURLOrNumber string) error {
	args := []string{"issue", "delete", issueURLOrNumber, "--yes"}
	_, stderr, exitCode := p.gh(args)
	if exitCode != 0 {
		return fmt.Errorf("gh issue delete failed: %s", stderr)
	}
	return nil
}

// #endregion ⚙️Types

// #region 🔭️Missing Hook Functions

// 📡️resolveCopilotEvent maps a VS Code / Copilot Chat native event to a neutral HookEvent.
func resolveCopilotEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "SessionStart":
		return model.HookAgentStarted, "", nil
	case "Stop":
		return model.HookAgentEnded, "", nil
	case "SubagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "SubagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "UserPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "PreCompact":
		return model.HookAgentCompacting, "", nil
	case "PreToolUse":
		return resolvePreToolUse(kind), "", nil
	case "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return model.HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return model.HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return model.HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return model.HookAgentEnded, "", nil
	case "afterAgentThought":
		return model.HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for copilot-chat", nativeEvent)
	}
}

// 🗺️resolveCursorEvent maps a Cursor native event to a neutral HookEvent.
func resolveCursorEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "sessionStart":
		return model.HookAgentStarted, "", nil
	case "sessionEnd":
		return model.HookAgentEnded, "", nil
	case "subagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	case "userPromptSubmit", "beforeSubmitPrompt":
		return model.HookAgentPromptSubmitting, "", nil
	case "preCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "beforeMCPExecution":
		return model.HookAgentToolStarting, "", nil
	case "afterMCPExecution":
		return model.HookAgentToolEnded, "", nil
	case "beforeReadFile", "beforeTabFileRead":
		return model.HookAgentToolSearchStarting, "", nil
	case "afterFileEdit", "afterTabFileEdit":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "beforeShellExecution":
		return resolveShellPreToolUse(kind), "", nil
	case "afterShellExecution":
		return resolveShellPostToolUse(kind), "", nil
	case "afterAgentResponse":
		return model.HookAgentEnded, "", nil
	case "afterAgentThought":
		return model.HookAgentThinkingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for cursor-chat", nativeEvent)
	}
}

// 🔷️resolveWindsurfEvent maps a Windsurf native event to a neutral HookEvent.
func resolveWindsurfEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "pre_user_prompt":
		return model.HookAgentPromptSubmitting, "", nil
	case "post_cascade_response":
		return model.HookAgentEnded, "", nil
	case "post_setup_worktree":
		return model.HookAgentStarted, "", nil
	case "subagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	case "preCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse", "pre_mcp_tool_use":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "post_mcp_tool_use":
		return resolvePostToolUse(kind), "", nil
	case "pre_read_code":
		return model.HookAgentToolSearchStarting, "", nil
	case "post_read_code":
		return model.HookAgentToolSearchEnded, "", nil
	case "pre_write_code":
		return model.HookAgentToolCodeEditStarting, "", nil
	case "post_write_code":
		return model.HookAgentToolCodeEditEnded, "", nil
	case "pre_run_command":
		return resolveShellPreToolUse(kind), "", nil
	case "post_run_command":
		return resolveShellPostToolUse(kind), "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for windsurf-chat", nativeEvent)
	}
}

// 🔶️resolveClaudeCompatibleEvent maps a Claude-compatible native event to a neutral HookEvent.
func ResolveClaudeCompatibleEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "start", "SessionStart":
		return model.HookAgentStarted, "", nil
	case "stop", "Stop", "SessionEnd":
		return model.HookAgentEnded, "", nil
	case "subagentStart", "SubagentStart":
		return model.HookAgentStarted, "subagent", nil
	case "subagentStop", "SubagentStop":
		return model.HookAgentEnded, "subagent", nil
	case "userPromptSubmit", "UserPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "preCompact", "PreCompact":
		return model.HookAgentCompacting, "", nil
	case "preToolUse", "PreToolUse", "PermissionRequest", "TeammateIdle", "Notification":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse", "postToolUseFailure", "PostToolUse", "PostToolUseFailure":
		return resolvePostToolUse(kind), "", nil
	case "TaskCompleted":
		return model.HookAgentToolPlanUpdatingEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for claude-compatible", nativeEvent)
	}
}

// 🔹️resolveKiroEvent maps a Kiro native event to a neutral HookEvent.
func resolveKiroEvent(nativeEvent string, kind model.ToolKind) (model.HookEvent, string, error) {
	switch nativeEvent {
	case "agentSpawn":
		return model.HookAgentStarted, "", nil
	case "userPromptSubmit":
		return model.HookAgentPromptSubmitting, "", nil
	case "preToolUse":
		return resolvePreToolUse(kind), "", nil
	case "postToolUse":
		return resolvePostToolUse(kind), "", nil
	case "stop":
		return model.HookAgentEnded, "", nil
	default:
		return "", "", fmt.Errorf("unknown native event %q for kiro-cli", nativeEvent)
	}
}

// 💻️formatVSCodeHookOutput formats a hook output for VS Code.
func FormatVSCodeHookOutput(hookEventName string, result model.HookResult) string {
	payload := map[string]interface{}{}
	out, _ := json.Marshal(result)
	_ = json.Unmarshal(out, &payload)
	hookSpecificOutput := map[string]interface{}{}
	if hookEventName != "" {
		hookSpecificOutput["hookEventName"] = hookEventName
	}
	if hookEventName == "PreToolUse" {
		if result.IsAllowed() {
			hookSpecificOutput["permissionDecision"] = "allow"
		} else {
			hookSpecificOutput["permissionDecision"] = "deny"
			if reason := strings.TrimSpace(result.GetMessage()); reason != "" {
				hookSpecificOutput["permissionDecisionReason"] = reason
			}
		}
	} else if message := strings.TrimSpace(result.GetMessage()); message != "" {
		hookSpecificOutput["additionalContext"] = message
	}
	if len(hookSpecificOutput) > 0 {
		payload["hookSpecificOutput"] = hookSpecificOutput
	}
	finalOut, _ := json.Marshal(payload)
	return string(finalOut)
}

// 🔤️vsCodeEventFromHookEvent converts a HookEvent to a VS Code event string.
func vsCodeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	switch event {
	case model.HookAgentStarted:
		if parentInfo == "subagent" {
			return "SubagentStart"
		}
		return "SessionStart"
	case model.HookAgentEnded:
		if parentInfo == "subagent" {
			return "SubagentStop"
		}
		return "Stop"
	case model.HookAgentPromptSubmitting:
		return "UserPromptSubmit"
	case model.HookAgentCompacting:
		return "PreCompact"
	case model.HookAgentToolStarting,
		model.HookAgentToolPlanUpdatingStarting,
		model.HookAgentToolSearchStarting,
		model.HookAgentToolCodeEditStarting,
		model.HookAgentToolTestStarting,
		model.HookAgentToolBuildStarting,
		model.HookAgentToolTerminalStarting:
		return "PreToolUse"
	case model.HookAgentToolEnded,
		model.HookAgentToolPlanUpdatingEnded,
		model.HookAgentToolSearchEnded,
		model.HookAgentToolCodeEditEnded,
		model.HookAgentToolTestEnded,
		model.HookAgentToolBuildEnded,
		model.HookAgentToolTerminalEnded:
		return "PostToolUse"
	default:
		return ""
	}
}

// 🏷️resolvePreToolUse resolves the pre-tool-use event based on tool kind.
func resolvePreToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindPlan:
		return model.HookAgentToolPlanUpdatingStarting
	case model.ToolKindCodeSearch:
		return model.HookAgentToolSearchStarting
	case model.ToolKindCodeEdit:
		return model.HookAgentToolCodeEditStarting
	case model.ToolKindTest:
		return model.HookAgentToolTestStarting
	case model.ToolKindBuild:
		return model.HookAgentToolBuildStarting
	case model.ToolKindTerminal:
		return model.HookAgentToolTerminalStarting
	default:
		return model.HookAgentToolStarting
	}
}

// 🔸️resolvePostToolUse resolves the post-tool-use event based on tool kind.
func resolvePostToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindPlan:
		return model.HookAgentToolPlanUpdatingEnded
	case model.ToolKindCodeSearch:
		return model.HookAgentToolSearchEnded
	case model.ToolKindCodeEdit:
		return model.HookAgentToolCodeEditEnded
	case model.ToolKindTest:
		return model.HookAgentToolTestEnded
	case model.ToolKindBuild:
		return model.HookAgentToolBuildEnded
	case model.ToolKindTerminal:
		return model.HookAgentToolTerminalEnded
	default:
		return model.HookAgentToolEnded
	}
}

func resolveShellPreToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindGeneric:
		return model.HookAgentToolTerminalStarting
	default:
		return resolvePreToolUse(kind)
	}
}

func resolveShellPostToolUse(kind model.ToolKind) model.HookEvent {
	switch kind {
	case model.ToolKindGeneric:
		return model.HookAgentToolTerminalEnded
	default:
		return resolvePostToolUse(kind)
	}
}

// #endregion 🔭️Missing Hook Functions

// #region 🪪️McpClientKind

type McpClientKind string

const McpClientGeneric McpClientKind = "generic"

const McpClientCursor McpClientKind = "cursor"

const McpClientKiro McpClientKind = "kiro"

const McpClientCopilot McpClientKind = "copilot"

const McpClientClaude McpClientKind = "claude"

const McpClientCodex McpClientKind = "codex"

// ParseMcpClientKind maps a CLI/MCP profile slug to an MCP server kind.
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

// HookClientForMcpKind maps an MCP entry binary to the hook client id used by ResolveHookEvent.
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

// McpKindFromResolvedClient maps a validated ticket client slug to an MCP surface kind for plan/spec attachment.
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

// McpServerName returns the MCP server identifier string for the given kind.
func McpServerName(kind McpClientKind) string {
	switch kind {
	case McpClientCursor, McpClientKiro, McpClientCopilot, McpClientClaude, McpClientCodex, McpClientGeneric, "":
		return "repo"
	default:
		return "repo"
	}
}

// #endregion 🪪️McpClientKind

// #endregion 🚚️Split
