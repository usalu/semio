// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// GitHub CLI issue, label, and milestone helpers.

// #endregion 🧲️Header

package client

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// 💬️ghAddComment holds the data fields for a ghAddComment record.
func ghAddComment(issueURL, comment string) error {
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
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue comment failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🏷️ghAddLabels holds the data fields for a ghAddLabels record.
func ghAddLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--add-label", label)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 📪️ghCloseIssue holds the data fields for a ghCloseIssue record.
func ghCloseIssue(issueURL string) error {
	args := []string{"issue", "close", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue close failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔓️ghReopenIssue holds the data fields for a ghReopenIssue record.
func ghReopenIssue(issueURL string) error {
	args := []string{"issue", "reopen", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue reopen failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// ⬛️ghUpdateIssueTitle holds the data fields for a ghUpdateIssueTitle record.
func ghUpdateIssueTitle(issueURL, title string) error {
	args := []string{"issue", "edit", issueURL, "--title", title}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// ⬜️ghUpdateIssueBody holds the data fields for a ghUpdateIssueBody record.
func ghUpdateIssueBody(issueURL, body string) error {
	args := []string{"issue", "edit", issueURL, "--body", body}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit body failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🟥️ghIssue holds the data fields for a ghIssue record.
type ghIssue struct {
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

// 🟧️ghMilestone holds the data fields for a ghMilestone record.
type ghMilestone struct {
	Number      int    `json:"number"`
	Title       string `json:"title"`
	Description string `json:"description"`
	URL         string `json:"url"`
	DueOn       string `json:"due_on"`
	State       string `json:"state"`
}

// 🟨️ghLabel holds the data fields for a ghLabel record.
type ghLabel struct {
	Name string `json:"name"`
}

// 🟩️ghGetIssueDetails holds the data fields for a ghGetIssueDetails record.
func ghGetIssueDetails(issueURL string) (*ghIssue, error) {
	args := []string{"issue", "view", issueURL, "--json", "url,state,milestone,labels,title,body"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue view failed: %s", strings.TrimSpace(stderr))
	}
	var issue ghIssue
	if err := json.Unmarshal([]byte(stdout), &issue); err != nil {
		return nil, fmt.Errorf("failed to parse gh issue view output: %w", err)
	}
	return &issue, nil
}

func ghGetMilestone(number int) (*ghMilestone, error) {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number)}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh api milestone get failed: %s", strings.TrimSpace(stderr))
	}
	var milestone ghMilestone
	if err := json.Unmarshal([]byte(stdout), &milestone); err != nil {
		return nil, fmt.Errorf("failed to parse gh milestone output: %w", err)
	}
	return &milestone, nil
}

// 🟦️ghFindMilestoneByTitle holds the data fields for a ghFindMilestoneByTitle record.
func ghFindMilestoneByTitle(title string) (*ghMilestone, error) {
	if strings.TrimSpace(title) == "" {
		return nil, nil
	}
	args := []string{"api", "repos/:owner/:repo/milestones", "-X", "GET", "-F", "state=all", "-F", "per_page=100", "--paginate", "--jq", ".[]"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh api milestone list failed: %s", strings.TrimSpace(stderr))
	}
	scanner := bufio.NewScanner(strings.NewReader(stdout))
	for scanner.Scan() {
		var milestone ghMilestone
		if err := json.Unmarshal(scanner.Bytes(), &milestone); err != nil {
			continue
		}
		if milestone.Title == title {
			return &milestone, nil
		}
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return nil, nil
}

// 🟪️ghUpdateIssueMilestone holds the data fields for a ghUpdateIssueMilestone record.
func ghUpdateIssueMilestone(issueURL, milestoneTitle string) error {
	if strings.TrimSpace(milestoneTitle) == "" {
		return fmt.Errorf("milestone title is required")
	}
	args := []string{"issue", "edit", issueURL, "--milestone", milestoneTitle}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit milestone failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🟫️ghClearIssueMilestone holds the data fields for a ghClearIssueMilestone record.
func ghClearIssueMilestone(issueURL string) error {
	issueNodeID, err := ghGetIssueNodeID(issueURL)
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
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api updateIssue clear milestone failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🚚️ghRemoveLabels holds the data fields for a ghRemoveLabels record.
func ghRemoveLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--remove-label", label)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit remove-label failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 💠️ghListIssuesForLabelSync holds the data fields for a ghListIssuesForLabelSync record.
func ghListIssuesForLabelSync() ([]ghIssue, error) {
	args := []string{"issue", "list", "--state", "all", "--json", "url,labels", "--limit", "1000"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue list for label sync failed: %s", strings.TrimSpace(stderr))
	}
	var issues []ghIssue
	if err := json.Unmarshal([]byte(stdout), &issues); err != nil {
		return nil, fmt.Errorf("failed to parse gh issue list output: %w", err)
	}
	return issues, nil
}

// 🔳️ghListRepoLabels holds the data fields for a ghListRepoLabels record.
func ghListRepoLabels() ([]ghLabel, error) {
	args := []string{"label", "list", "--json", "name", "--limit", "1000"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh label list failed: %s", strings.TrimSpace(stderr))
	}
	var labels []ghLabel
	if err := json.Unmarshal([]byte(stdout), &labels); err != nil {
		return nil, fmt.Errorf("failed to parse gh label list output: %w", err)
	}
	return labels, nil
}

func ghCreateRepoLabel(name string) error {
	if strings.TrimSpace(name) == "" {
		return fmt.Errorf("label name is required")
	}
	args := []string{"label", "create", name, "--color", "1d76db", "--description", "Compose technology or bundle"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh label create failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🗑️ghDeleteRepoLabel holds the data fields for a ghDeleteRepoLabel record.
func ghDeleteRepoLabel(name string) error {
	if strings.TrimSpace(name) == "" {
		return fmt.Errorf("label name is required")
	}
	args := []string{"label", "delete", name, "--yes"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh label delete failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghSyncRepoLabelCatalog(validLabels map[string]bool) error {
	labels, err := ghListRepoLabels()
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
		if err := ghCreateRepoLabel(label); err != nil {
			writeWarningf("Failed to create GitHub label %s: %v", label, err)
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
		if err := ghDeleteRepoLabel(label); err != nil {
			writeWarningf("Failed to delete GitHub label %s: %v", label, err)
		}
	}
	return nil
}

// 🧲️extractMilestoneNumber holds the data fields for a extractMilestoneNumber record.
func extractMilestoneNumber(milestoneURL string) int {
	parts := strings.Split(milestoneURL, "/")
	if len(parts) == 0 {
		return 0
	}
	num, _ := strconv.Atoi(parts[len(parts)-1])
	return num
}

// ⏹️ghListOpenIssuesWithLabel holds the data fields for a ghListOpenIssuesWithLabel record.
func ghListOpenIssuesWithLabel(label string) ([]string, error) {
	args := []string{"issue", "list", "--label", label, "--state", "open", "--json", "url", "--limit", "1000"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
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
