// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎄️github carries the GitHub management provider: construction, issue operations, labels, milestones lookups and the gh helpers.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎄️GitHub

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

// #endregion 🎄️GitHub
