// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// GitHub and null management providers.

// #endregion 🧲️Header

package client



// 🔌️GitHubManagementProvider holds the data fields for a github management provider record.
type GitHubManagementProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *GitHubManagementProvider) Kind() string { return "github" }

// 🆕️CreateIssue holds the data fields for a CreateIssue record.
func (p *GitHubManagementProvider) CreateIssue(title, body string, milestone *int) (string, error) {
	return ghCreateIssue(title, body, milestone)
}

// 📪️CloseIssue holds the data fields for a CloseIssue record.
func (p *GitHubManagementProvider) CloseIssue(issueURL string) error {
	return ghCloseIssue(issueURL)
}

// 🔓️ReopenIssue holds the data fields for a ReopenIssue record.
func (p *GitHubManagementProvider) ReopenIssue(issueURL string) error {
	return ghReopenIssue(issueURL)
}

// 🗑️DeleteIssue holds the data fields for a DeleteIssue record.
func (p *GitHubManagementProvider) DeleteIssue(issueURLOrNumber string) error {
	return ghDeleteIssue(issueURLOrNumber)
}

// 🔁️UpdateIssueTitle holds the data fields for a UpdateIssueTitle record.
func (p *GitHubManagementProvider) UpdateIssueTitle(issueURL, title string) error {
	return ghUpdateIssueTitle(issueURL, title)
}

// 🔷️UpdateIssueBody holds the data fields for a UpdateIssueBody record.
func (p *GitHubManagementProvider) UpdateIssueBody(issueURL, body string) error {
	return ghUpdateIssueBody(issueURL, body)
}

// 🔶️GetIssueDetails holds the data fields for a GetIssueDetails record.
func (p *GitHubManagementProvider) GetIssueDetails(issueURL string) (*ManagementIssue, error) {
	issue, err := ghGetIssueDetails(issueURL)
	if err != nil {
		return nil, err
	}
	mi := &ManagementIssue{URL: issue.URL, State: issue.State, Title: issue.Title, Body: issue.Body}
	if issue.Milestone != nil {
		mi.Milestone = &struct {
			Number int    `json:"number"`
			Title  string `json:"title"`
		}{Number: issue.Milestone.Number, Title: issue.Milestone.Title}
	}
	for _, l := range issue.Labels {
		mi.Labels = append(mi.Labels, struct {
			Name string `json:"name"`
		}{Name: l.Name})
	}
	return mi, nil
}

// 🌿️GetIssueNodeID holds the data fields for a GetIssueNodeID record.
func (p *GitHubManagementProvider) GetIssueNodeID(issueURL string) (string, error) {
	return ghGetIssueNodeID(issueURL)
}

// 🌐️GetIssueParentURL holds the data fields for a GetIssueParentURL record.
func (p *GitHubManagementProvider) GetIssueParentURL(issueURL string) (string, error) {
	return ghGetIssueParentURL(issueURL)
}

// 💬️AddComment holds the data fields for a AddComment record.
func (p *GitHubManagementProvider) AddComment(issueURL, comment string) error {
	return ghAddComment(issueURL, comment)
}

// 🏷️AddLabels holds the data fields for a AddLabels record.
func (p *GitHubManagementProvider) AddLabels(issueURL string, labels []string) error {
	return ghAddLabels(issueURL, labels)
}

// 🚚️RemoveLabels holds the data fields for a RemoveLabels record.
func (p *GitHubManagementProvider) RemoveLabels(issueURL string, labels []string) error {
	return ghRemoveLabels(issueURL, labels)
}

// ➕️AddIssueToProject holds the data fields for a AddIssueToProject record.
func (p *GitHubManagementProvider) AddIssueToProject(issueURL string) {
	ghAddIssueToProject(issueURL)
}

func (p *GitHubManagementProvider) AssignIssueToCurrentUser(issueURL string) {
	ghAssignIssueToCurrentUser(issueURL)
}

// 🔹️AddSubIssue holds the data fields for a AddSubIssue record.
func (p *GitHubManagementProvider) AddSubIssue(parentIssueURL, childIssueURL string) error {
	return ghAddSubIssue(parentIssueURL, childIssueURL)
}

// 🎯️UpdateIssueMilestone holds the data fields for a UpdateIssueMilestone record.
func (p *GitHubManagementProvider) UpdateIssueMilestone(issueURL, milestoneTitle string) error {
	return ghUpdateIssueMilestone(issueURL, milestoneTitle)
}

func (p *GitHubManagementProvider) ClearIssueMilestone(issueURL string) error {
	return ghClearIssueMilestone(issueURL)
}

// 🔸️CreateMilestone holds the data fields for a CreateMilestone record.
func (p *GitHubManagementProvider) CreateMilestone(title, description string) (int, error) {
	return ghCreateMilestone(title, description)
}

// 🔺️UpdateMilestone holds the data fields for a UpdateMilestone record.
func (p *GitHubManagementProvider) UpdateMilestone(number int, title, description, state, dueOn string) error {
	return ghUpdateMilestone(number, title, description, state, dueOn)
}

// 🔻️DeleteMilestone holds the data fields for a DeleteMilestone record.
func (p *GitHubManagementProvider) DeleteMilestone(number int) error {
	return ghDeleteMilestone(number)
}

// ⬛️GetMilestone holds the data fields for a GetMilestone record.
func (p *GitHubManagementProvider) GetMilestone(number int) (*ManagementMilestone, error) {
	m, err := ghGetMilestone(number)
	if err != nil {
		return nil, err
	}
	return &ManagementMilestone{Number: m.Number, Title: m.Title, Description: m.Description, URL: m.URL, DueOn: m.DueOn, State: m.State}, nil
}

// 📌️GetMilestoneTitle holds the data fields for a GetMilestoneTitle record.
func (p *GitHubManagementProvider) GetMilestoneTitle(number int) (string, error) {
	return ghGetMilestoneTitle(number)
}

func (p *GitHubManagementProvider) FindMilestoneByTitle(title string) (*ManagementMilestone, error) {
	m, err := ghFindMilestoneByTitle(title)
	if err != nil {
		return nil, err
	}
	if m == nil {
		return nil, nil
	}
	return &ManagementMilestone{Number: m.Number, Title: m.Title, Description: m.Description, URL: m.URL, DueOn: m.DueOn, State: m.State}, nil
}

// 📋️ListIssuesForLabelSync holds the data fields for a ListIssuesForLabelSync record.
func (p *GitHubManagementProvider) ListIssuesForLabelSync() ([]ManagementIssue, error) {
	issues, err := ghListIssuesForLabelSync()
	if err != nil {
		return nil, err
	}
	result := make([]ManagementIssue, len(issues))
	for i, issue := range issues {
		result[i] = ManagementIssue{URL: issue.URL, State: issue.State, Title: issue.Title, Body: issue.Body}
		for _, l := range issue.Labels {
			result[i].Labels = append(result[i].Labels, struct {
				Name string `json:"name"`
			}{Name: l.Name})
		}
	}
	return result, nil
}

// 📬️ListOpenIssuesWithLabel holds the data fields for a ListOpenIssuesWithLabel record.
func (p *GitHubManagementProvider) ListOpenIssuesWithLabel(label string) ([]string, error) {
	return ghListOpenIssuesWithLabel(label)
}

// ⬜️ListRepoLabels holds the data fields for a ListRepoLabels record.
func (p *GitHubManagementProvider) ListRepoLabels() ([]ManagementLabel, error) {
	labels, err := ghListRepoLabels()
	if err != nil {
		return nil, err
	}
	result := make([]ManagementLabel, len(labels))
	for i, l := range labels {
		result[i] = ManagementLabel{Name: l.Name}
	}
	return result, nil
}

// 🟥️CreateRepoLabel holds the data fields for a CreateRepoLabel record.
func (p *GitHubManagementProvider) CreateRepoLabel(name string) error {
	return ghCreateRepoLabel(name)
}

// 🟧️DeleteRepoLabel holds the data fields for a DeleteRepoLabel record.
func (p *GitHubManagementProvider) DeleteRepoLabel(name string) error {
	return ghDeleteRepoLabel(name)
}

func (p *GitHubManagementProvider) SyncRepoLabelCatalog(validLabels map[string]bool) error {
	return ghSyncRepoLabelCatalog(validLabels)
}

// ⛳️CreateGoalIssue holds the data fields for a CreateGoalIssue record.
func (p *GitHubManagementProvider) CreateGoalIssue(title, description string, milestone *int) (string, error) {
	return ghCreateGoalIssue(title, description, milestone)
}

// 🟨️UpdateGoalIssue holds the data fields for a UpdateGoalIssue record.
func (p *GitHubManagementProvider) UpdateGoalIssue(issueURL, title, description string) error {
	return ghUpdateGoalIssue(issueURL, title, description)
}

// 👤️GetCurrentUser holds the data fields for a GetCurrentUser record.
func (p *GitHubManagementProvider) GetCurrentUser() string {
	return ghGetCurrentUser()
}

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
