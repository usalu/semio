// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔄️management carries the management provider synchronization of the repository context.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	sort "sort"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	goalspkg "github.com/usalu/semio/repo/goals"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔄️Management

// ◼SyncManagement MUST return a non-nil error when the operation fails.
// 💖️SyncManagement performs the sync github operation on the repo context.
func (c *RepoContext) SyncManagement() (bool, error) {
	fmt.Println("Syncing local tickets and goals with GitHub...")

	technologies := codebase.LoadTechnologies()
	validLabels := make(map[string]bool)
	for _, p := range technologies {
		technologyLabel := "@" + strings.TrimPrefix(p.Name, "@")
		validLabels[technologyLabel] = true
		for _, b := range p.Bundles {
			validLabels[model.NormalizeBundleLabel(b.Name)] = true
		}
	}
	validLabels["repo"] = true
	if err := c.managementProvider.SyncRepoLabelCatalog(validLabels); err != nil {
		workspace.WriteWarningf("Failed to sync GitHub label catalog: %v", err)
	}

	goals, err := goalspkg.ListGoals()
	if err != nil {
		workspace.WriteWarningf("Failed to list goals: %v", err)
	} else {

		sort.Slice(goals, func(i, j int) bool {
			return goalspkg.GoalDepth(goals[i].ID) < goalspkg.GoalDepth(goals[j].ID)
		})
		for _, goal := range goals {
			if goalspkg.IsRootGoal(goal.ID) {

				if goal.Management != nil && goal.Management.Issue != "" {
					fmt.Printf("Migrating root goal %s: removing issue reference\n", goal.ID)
					goal.Management.Issue = ""
				}
				milestone, err := ensureGoalMilestone(goal)
				if err != nil {
					workspace.WriteWarningf("Failed to ensure milestone for root goal %s: %v", goal.ID, err)
				} else if milestone != nil {
					status := "open"
					if goal.Status == "closed" {
						status = "closed"
					}
					if err := c.managementProvider.UpdateMilestone(milestone.Number, goal.Title, goal.Description, status, goal.Dates.Due); err != nil {
						workspace.WriteWarningf("Failed to update milestone for root goal %s: %v", goal.ID, err)
					}
				}
			} else {

				if goal.Management != nil && goal.Management.Milestone != "" {

					fmt.Printf("Migrating child goal %s: removing milestone\n", goal.ID)
					number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
					if err == nil {
						if err := c.managementProvider.DeleteMilestone(number); err != nil {
							workspace.WriteWarningf("Failed to delete milestone %d for child goal %s: %v", number, goal.ID, err)
						}
					}
					goal.Management.Milestone = ""
				}

				needsSave := false
				if goal.Management == nil || goal.Management.Issue == "" {

					var milestone *int
					if goalspkg.IsFirstGenGoal(goal.ID) {
						milestone, _ = goalspkg.GetRootGoalMilestone(goal.ID)
					}
					issueURL, err := c.managementProvider.CreateGoalIssue(goal.Title, goal.Description, milestone)
					if err != nil {
						workspace.WriteWarningf("Failed to create issue for child goal %s: %v", goal.ID, err)
						continue
					}
					if goal.Management == nil {
						goal.Management = &model.GoalManagementData{}
					}
					goal.Management.Issue = issueURL
					needsSave = true
					fmt.Printf("Created issue for child goal %s: %s\n", goal.ID, issueURL)

					if goal.Status == "closed" {
						_ = c.managementProvider.CloseIssue(issueURL)
					}

					if goalspkg.IsDeeperGoal(goal.ID) {
						parentID := goalspkg.GetParentGoalID(goal.ID)
						parentGoal, parentErr := goalspkg.ReadGoal(parentID)
						if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
							if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, issueURL); err != nil {
								workspace.WriteWarningf("Failed to link sub-issue %s to parent %s: %v", goal.ID, parentID, err)
							} else {
								fmt.Printf("Linked sub-issue %s to parent %s\n", goal.ID, parentID)
							}
						}
					}
				} else {

					remoteIssue, err := c.managementProvider.GetIssueDetails(goal.Management.Issue)
					if err != nil {
						workspace.WriteWarningf("Failed to get issue for goal %s: %v", goal.ID, err)
					} else {
						if goal.Status == "closed" && strings.ToUpper(remoteIssue.State) == "OPEN" {
							_ = c.managementProvider.CloseIssue(goal.Management.Issue)
						} else if goal.Status == "open" && strings.ToUpper(remoteIssue.State) == "CLOSED" {
							_ = c.managementProvider.ReopenIssue(goal.Management.Issue)
						}
						if goalspkg.IsFirstGenGoal(goal.ID) {
							milestone, _ := goalspkg.GetRootGoalMilestone(goal.ID)
							if milestone != nil {
								rootGoal, _ := goalspkg.ReadGoal(goalspkg.GetRootGoalID(goal.ID))
								if rootGoal != nil && (remoteIssue.Milestone == nil || remoteIssue.Milestone.Title != rootGoal.Title) {
									_ = c.managementProvider.UpdateIssueMilestone(goal.Management.Issue, rootGoal.Title)
								}
							}
						} else if goalspkg.IsDeeperGoal(goal.ID) {
							if remoteIssue.Milestone != nil {
								if err := c.managementProvider.ClearIssueMilestone(goal.Management.Issue); err != nil {
									workspace.WriteWarningf("Failed to clear milestone for goal %s: %v", goal.ID, err)
								}
							}
							parentID := goalspkg.GetParentGoalID(goal.ID)
							parentGoal, parentErr := goalspkg.ReadGoal(parentID)
							if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
								parentURL, parentErr := c.managementProvider.GetIssueParentURL(goal.Management.Issue)
								if parentErr != nil {
									workspace.WriteWarningf("Failed to resolve parent for goal %s: %v", goal.ID, parentErr)
								} else if parentURL == "" || parentURL != parentGoal.Management.Issue {
									if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, goal.Management.Issue); err != nil {
										workspace.WriteWarningf("Failed to link sub-issue %s to parent %s: %v", goal.ID, parentID, err)
									} else {
										fmt.Printf("Linked sub-issue %s to parent %s\n", goal.ID, parentID)
									}
								}
							}
						}
						hasGoalLabel := false
						for _, label := range remoteIssue.Labels {
							if label.Name == "goal" {
								hasGoalLabel = true
								break
							}
						}
						if !hasGoalLabel {
							if err := c.managementProvider.AddLabels(goal.Management.Issue, []string{"goal"}); err != nil {
								workspace.WriteWarningf("Failed to add goal label for %s: %v", goal.ID, err)
							}
						}
					}
				}

				if needsSave {
					if err := goalspkg.SaveGoal(*goal); err != nil {
						workspace.WriteWarningf("Failed to save goal %s: %v", goal.ID, err)
					}
				}
			}
		}
	}

	tickets, err := ticketspkg.ListTickets(nil, nil, nil)
	if err != nil {
		return false, err
	}

	for i := range tickets {
		t := &tickets[i]
		if t.Management == nil || t.Management.Issue == "" {
			continue
		}

		issueURL := t.Management.Issue

		remoteIssue, err := c.managementProvider.GetIssueDetails(issueURL)
		if err != nil {
			workspace.WriteWarningf("Failed to get GitHub issue %s: %v", issueURL, err)
			continue
		}

		if t.Status == model.TicketStatusClosed && strings.ToUpper(remoteIssue.State) == "OPEN" {
			fmt.Printf("Closing GitHub issue %s (Ticket is closed locally)\n", issueURL)
			if err := c.managementProvider.CloseIssue(issueURL); err != nil {
				workspace.WriteWarningf("Failed to close GitHub issue %s: %v", issueURL, err)
			}
		}

		if t.Goal != "" {
			rootID := goalspkg.GetRootGoalID(t.Goal)
			rootGoal, err := goalspkg.ReadGoal(rootID)
			if err == nil {
				milestone, err := ensureGoalMilestone(rootGoal)
				if err != nil {
					workspace.WriteWarningf("Failed to resolve goal milestone for issue %s: %v", issueURL, err)
				} else if milestone != nil && milestone.Title != "" {
					if remoteIssue.Milestone == nil || remoteIssue.Milestone.Title != milestone.Title {
						fmt.Printf("Updating milestone for issue %s to %s...\n", issueURL, milestone.Title)
						if err := c.managementProvider.UpdateIssueMilestone(issueURL, milestone.Title); err != nil {
							workspace.WriteWarningf("Failed to update milestone for GitHub issue %s: %v", issueURL, err)
						}
					}
				}
			}
		}

		var labelsToRemove []string
		for _, label := range remoteIssue.Labels {
			if strings.HasPrefix(label.Name, "@") {
				if !validLabels[label.Name] {
					labelsToRemove = append(labelsToRemove, label.Name)
				}
			}
		}
		if len(labelsToRemove) > 0 {
			fmt.Printf("Removing invalid technology labels from issue %s: %v\n", issueURL, labelsToRemove)
			if err := c.managementProvider.RemoveLabels(issueURL, labelsToRemove); err != nil {
				workspace.WriteWarningf("Failed to remove labels from GitHub issue %s: %v", issueURL, err)
			}
		}
	}

	issues, err := c.managementProvider.ListIssuesForLabelSync()
	if err != nil {
		workspace.WriteWarningf("Failed to list GitHub issues for label sync: %v", err)
	} else {
		for _, issue := range issues {
			var labelsToRemove []string
			for _, label := range issue.Labels {
				if strings.HasPrefix(label.Name, "@") && !validLabels[label.Name] {
					labelsToRemove = append(labelsToRemove, label.Name)
				}
			}
			if len(labelsToRemove) == 0 {
				continue
			}
			fmt.Printf("Removing invalid technology labels from issue %s: %v\n", issue.URL, labelsToRemove)
			if err := c.managementProvider.RemoveLabels(issue.URL, labelsToRemove); err != nil {
				workspace.WriteWarningf("Failed to remove labels from GitHub issue %s: %v", issue.URL, err)
			}
		}
	}

	fmt.Println("GitHub sync completed.")
	return true, nil
}

// #endregion 🔄️Management
