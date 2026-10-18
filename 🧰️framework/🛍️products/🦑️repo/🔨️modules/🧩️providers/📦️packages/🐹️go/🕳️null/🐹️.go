// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🕳️null carries the no-operation management provider.

// #endregion 🧲️Header

package providers

// #region 🕳️NullManagement

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

// #endregion 🕳️NullManagement
