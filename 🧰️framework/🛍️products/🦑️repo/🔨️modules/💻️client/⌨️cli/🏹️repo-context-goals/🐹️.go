// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo context goal and ticket mutations.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

// 🆕️GoalCreate MUST return a non-nil error when the operation fails.
// ◽GoalCreate performs the goal create operation on the repo context.
func (c *repoContext) GoalCreate(input GoalCreateInput) (*Goal, error) {

	if input.Title == "" {
		return nil, fmt.Errorf("missing title")
	}
	if input.Description == "" {
		return nil, fmt.Errorf("missing description")
	}
	if input.Prompt == "" {
		return nil, fmt.Errorf("missing prompt")
	}
	if input.DueDate == "" {
		return nil, fmt.Errorf("missing due date")
	}
	if input.LLM == "" {
		return nil, fmt.Errorf("missing llm")
	}
	if input.Client == "" {
		return nil, fmt.Errorf("missing client")
	}

	llmSlug, err := ResolveAllowedLLM(input.LLM)
	if err != nil {
		return nil, err
	}
	var effortSlug string
	if input.Effort != "" {
		effortSlug, err = ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, err
		}
	}
	uiSlug, err := ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}

	slug := Slugify(input.Title)
	id := slug
	if input.Parent != "" {
		id = input.Parent + "/" + slug
	}

	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(id), "🎯️goal.json")
	if FileExists(path) {
		return nil, fmt.Errorf("goal with id %s already exists", id)
	}

	goal := Goal{
		ID:          id,
		Title:       input.Title,
		Parent:      input.Parent,
		Description: input.Description,
		Prompt:      input.Prompt,
		Status:      "open",
		Dates:       GoalDates{Due: input.DueDate},
		Client:      uiSlug,
		LLM:         llmSlug,
		Effort:      effortSlug,
	}

	if !input.NoManagement {
		if isRootGoal(&goal) {
			if input.Milestone != "" {
				goal.Management = &GoalManagementData{Milestone: input.Milestone}
			} else {
				milestoneNumber, err := c.managementProvider.CreateMilestone(input.Title, input.Description)
				if err != nil {
					return nil, err
				}
				if input.DueDate != "" {
					_ = c.managementProvider.UpdateMilestone(milestoneNumber, "", "", "", input.DueDate)
				}
				repoUrl, _ := getGhRepoUrl()
				goal.Management = &GoalManagementData{
					Milestone: fmt.Sprintf("%s/milestone/%d", repoUrl, milestoneNumber),
				}
			}
		} else if isFirstGenGoal(&goal) {
			milestone, _ := getRootGoalMilestone(id)
			issueURL, err := c.managementProvider.CreateGoalIssue(input.Title, input.Description, milestone)
			if err != nil {
				return nil, err
			}
			goal.Management = &GoalManagementData{Issue: issueURL}
		} else {
			issueURL, err := c.managementProvider.CreateGoalIssue(input.Title, input.Description, nil)
			if err != nil {
				return nil, err
			}
			goal.Management = &GoalManagementData{Issue: issueURL}
			parentID := getParentGoalID(id)
			parentGoal, parentErr := ReadGoal(parentID)
			if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
				if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, issueURL); err != nil {
					writeWarningf("Failed to link sub-issue to parent goal %s: %v", parentID, err)
				}
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	repopkg.Emit(repopkg.EventGoalOpenEnded, "repo-cli", repopkg.GoalOpenPayload{
		GoalPayload: repopkg.GoalPayload{ID: goal.ID},
		Title:       goal.Title, Description: goal.Description, LLM: llmSlug, Effort: effortSlug, Parent: goal.Parent, Author: GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🌐️getGhRepoUrl holds the data fields for a getGhRepoUrl record.
func getGhRepoUrl() (string, error) {
	out, err := exec.Command("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// 🎯️parseMilestoneNumber holds the data fields for a parseMilestoneNumber record.
func parseMilestoneNumber(milestone string) (int, error) {
	if n, err := strconv.Atoi(milestone); err == nil {
		return n, nil
	}
	parts := strings.Split(milestone, "/")
	if len(parts) > 0 {
		if n, err := strconv.Atoi(parts[len(parts)-1]); err == nil {
			return n, nil
		}
	}
	return 0, fmt.Errorf("could not parse milestone number from %s", milestone)
}

// 🔁️UpdateGoalTitle MUST apply the update and return an error if the target is missing.
// ❓️UpdateGoalTitle modifies an existing goal title entry.
func UpdateGoalTitle(goal *Goal, title string) error {
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("goal title is required")
	}
	newSlug := Slugify(title)
	if title == newSlug {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(newSlug) {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}

	var newID string
	if goal.Parent != "" {
		newID = goal.Parent + "/" + newSlug
	} else {
		newID = newSlug
	}

	if newID != goal.ID {
		dir := GetRepoGoalsDir()
		oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
		newPath := filepath.Join(dir, filepath.FromSlash(newID))
		if FileExists(newPath) {
			return fmt.Errorf("goal folder already exists: %s", newPath)
		}
		if err := os.Rename(oldPath, newPath); err != nil {
			return err
		}
		goal.ID = newID
	}
	goal.Title = title
	return nil
}

// ♻️GoalChange MUST return a non-nil error when the operation fails.
// 📐️GoalChange performs the goal change operation on the repo context.
func (c *repoContext) GoalChange(input GoalChangeInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		llmSlug, err := ResolveAllowedLLM(*input.LLM)
		if err != nil {
			return nil, err
		}
		goal.LLM = llmSlug
	}
	if input.Effort != nil {
		effortSlug, err := ResolveAllowedEffort(*input.Effort)
		if err != nil {
			return nil, err
		}
		goal.Effort = effortSlug
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent

		slug := goal.ID
		if idx := strings.LastIndex(goal.ID, "/"); idx != -1 {
			slug = goal.ID[idx+1:]
		}

		var newID string
		if goal.Parent != "" {
			newID = goal.Parent + "/" + slug
		} else {
			newID = slug
		}

		if newID != goal.ID {
			oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
			newPath := filepath.Join(dir, filepath.FromSlash(newID))

			if FileExists(newPath) {
				return nil, fmt.Errorf("goal folder already exists: %s", newPath)
			}

			if err := os.MkdirAll(filepath.Dir(newPath), 0755); err != nil {
				return nil, err
			}

			if err := os.Rename(oldPath, newPath); err != nil {
				return nil, err
			}
			goal.ID = newID
		}
	}

	if goal.Management != nil && !input.NoManagement {
		if isRootGoal(&goal) && goal.Management.Milestone != "" {
			number, err := parseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, goal.Status, goal.Dates.Due); err != nil {
					return nil, err
				}
			}
		} else if !isRootGoal(&goal) && goal.Management.Issue != "" {
			if err := c.managementProvider.UpdateGoalIssue(goal.Management.Issue, goal.Title, goal.Description); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	repopkg.Emit(repopkg.EventGoalChangeEnded, "repo-cli", repopkg.GoalChangePayload{
		GoalPayload: repopkg.GoalPayload{ID: goal.ID},
		Title:       input.Title, Description: input.Description, LLM: input.LLM, Effort: input.Effort, Parent: input.Parent, Author: GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 📪️GoalClose MUST return a non-nil error when the operation fails.
// 🏁️GoalClose performs the goal close operation on the repo context.
func (c *repoContext) GoalClose(input GoalCloseInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if goal.Status == "closed" {
		return nil, fmt.Errorf("goal is already closed")
	}

	goal.Status = "closed"
	goal.Summary = input.Summary

	if goal.Management != nil && !input.NoManagement {
		if isRootGoal(&goal) && goal.Management.Milestone != "" {
			number, err := parseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, "closed", ""); err != nil {
					return nil, err
				}
			}
		} else if !isRootGoal(&goal) && goal.Management.Issue != "" {
			if err := c.managementProvider.CloseIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	repopkg.Emit(repopkg.EventGoalCloseEnded, "repo-cli", repopkg.GoalClosePayload{
		GoalPayload: repopkg.GoalPayload{ID: goal.ID},
		Summary:     goal.Summary, Author: GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🔓️GoalReopen MUST return a non-nil error when the operation fails.
// 🔄️GoalReopen performs the goal reopen operation on the repo context.
func (c *repoContext) GoalReopen(input GoalReopenInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if goal.Status == "open" {
		return nil, fmt.Errorf("goal is already open")
	}

	goal.Status = "open"

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	goal.Prompt = input.Prompt
	goal.LLM = input.LLM
	goal.Client = input.Client
	if input.Effort != "" {
		effortSlug, err := ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, err
		}
		goal.Effort = effortSlug
	}

	if goal.Management != nil && !input.NoManagement {
		if isRootGoal(&goal) && goal.Management.Milestone != "" {
			number, err := parseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, "open", goal.Dates.Due); err != nil {
					return nil, err
				}
			}
		} else if !isRootGoal(&goal) && goal.Management.Issue != "" {
			if err := c.managementProvider.ReopenIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	repopkg.Emit(repopkg.EventGoalReopenEnded, "repo-cli", repopkg.GoalReopenPayload{
		GoalPayload: repopkg.GoalPayload{ID: goal.ID},
		Prompt:      input.Prompt, Client: input.Client, LLM: input.LLM, Effort: goal.Effort, Author: GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🔺️TicketChange MUST return a non-nil error when the operation fails.
// ♻️TicketChange performs the ticket change operation on the repo context.
func (c *repoContext) TicketChange(input TicketChangeInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				writeWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}

	changed := false
	if input.Prompt != nil {
		ticket.Description = *input.Prompt
		if len(ticket.Interactions) > 0 {
			ticket.Interactions[len(ticket.Interactions)-1].Prompt = *input.Prompt
		}
		changed = true
	}

	if len(ticket.Interactions) > 0 {
		if input.LLM != nil {
			llmSlug, err := ResolveAllowedLLM(*input.LLM)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].LLM = llmSlug
			changed = true
		}
		if input.Effort != nil {
			effortSlug, err := ResolveAllowedEffort(*input.Effort)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Effort = effortSlug
			changed = true
		}
		if input.Client != nil {
			uiSlug, err := ResolveAllowedClient(*input.Client)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Client = uiSlug
			changed = true
		}
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
		changed = true
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
		changed = true
	}

	if changed {
		if err := SaveTicket(ticket); err != nil {
			return nil, err
		}
		ticketID := FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		repopkg.Emit(repopkg.EventTicketChangeEnded, "repo-cli", repopkg.TicketChangePayload{
			TicketPayload: repopkg.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
			Title:         input.Title, Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort, Goal: input.Goal, Parent: input.Parent, Author: GetGitAuthorAlias(),
		})
	}

	return ticket, nil
}

// 🗑️GoalDelete MUST return a non-nil error when the operation fails.
// ◼GoalDelete performs the goal delete operation on the repo context.
func (c *repoContext) GoalDelete(input GoalDeleteInput) (bool, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return false, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return false, err
	}

	if !input.NoManagement && goal.Management != nil {
		if isRootGoal(&goal) && goal.Management.Milestone != "" {
			number, err := parseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.DeleteMilestone(number); err != nil {
					return false, err
				}
			}
		} else if !isRootGoal(&goal) && goal.Management.Issue != "" {
			parts := strings.Split(goal.Management.Issue, "/")
			number := parts[len(parts)-1]
			if err := c.managementProvider.DeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	if err := os.RemoveAll(filepath.Dir(path)); err != nil {
		return false, err
	}
	return true, nil
}

// 🔻️TicketDelete MUST return a non-nil error when the operation fails.
// 🔴️TicketDelete performs the ticket delete operation on the repo context.
func (c *repoContext) TicketDelete(input TicketDeleteInput) (bool, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return false, err
	}

	if !input.NoManagement && ticket.Management != nil {
		if ticket.Management.Issue != "" {
			parts := strings.Split(ticket.Management.Issue, "/")
			number := parts[len(parts)-1]
			if err := c.managementProvider.DeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	path := ticket.FolderPath
	if path == "" {
		path = GetTicketPath(input.Year, input.Month, input.Day, input.Slug)
	}

	if err := os.RemoveAll(path); err != nil {
		return false, err
	}

	return true, nil
}
