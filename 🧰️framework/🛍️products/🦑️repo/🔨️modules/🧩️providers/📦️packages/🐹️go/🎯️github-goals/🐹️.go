// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎯️github_goals carries the GitHub management provider goal operations: milestones, goal issues, sub-issues and parent lookups.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"
	fmt "fmt"
	strconv "strconv"
	strings "strings"
)

// #region 🎯️GitHubGoals

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

// #endregion 🎯️GitHubGoals
