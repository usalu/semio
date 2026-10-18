// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket opening and creation with GitHub issue helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// 📬️OpenTicket MUST complete the operation and return consistent results.
func OpenTicket(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind McpClientKind, planID, specID string) (*Ticket, error) {
	if prompt == "" {
		prompt = title
	}
	if shouldSkipTicket(prompt) {
		return nil, nil
	}
	if shouldContinueTicket(prompt) {
		latest, err := LatestTicket()
		if err != nil {
			return nil, err
		}
		if latest.GetStatus() == TicketStatusClosed {
			return latest, ReopenTicket(latest, prompt, llm, effort, client, draft, goal, parent, noManagement, mcpKind, planID, specID)
		}
		return latest, nil
	}
	return CreateTicket(emoji, title, prompt, llm, effort, client, draft, noIssue, goal, parent, noManagement, issue, mcpKind, planID, specID)
}

// ⛳️OpenGoal MUST complete the operation and return consistent results.
func OpenGoal(title, description, prompt, dueDate, client, llm, effort string, noManagement bool) (*Goal, error) {
	ctx := NewRepoContext(rootDir)
	input := GoalCreateInput{
		Title:        title,
		Description:  description,
		Prompt:       prompt,
		DueDate:      dueDate,
		Client:       client,
		LLM:          llm,
		Effort:       effort,
		NoManagement: noManagement,
	}
	return ctx.GoalCreate(input)
}

// 🎫️validateTicketEmojiTitle MUST validate the emoji and title and return the derived slug.
// 🎫️Requires a non-empty emoji that extractEntityEmoji recognizes and a non-empty title.
// 🎫️Does NOT enforce any shape on the title string.
func validateTicketEmojiTitle(emoji, title string) (string, error) {
	emoji = strings.TrimSpace(emoji)
	if emoji == "" {
		return "", fmt.Errorf("ticket emoji is required")
	}
	extracted, remaining := extractEntityEmoji(emoji)
	if extracted == "" || strings.TrimSpace(remaining) != "" {
		return "", fmt.Errorf("ticket emoji must be a single emoji character")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return "", fmt.Errorf("ticket title is required")
	}
	slug := Slugify(title)
	if slug == "" {
		return "", fmt.Errorf("ticket title must contain at least one alphanumeric character")
	}
	return slug, nil
}

// 🔁️UpdateTicketTitle MUST complete the operation and return consistent results.
// 🔁️Only validates that the title is non-empty; does NOT enforce any shape on the title string.
func UpdateTicketTitle(ticket *Ticket, title string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("ticket title is required")
	}
	slug := Slugify(title)
	if slug == "" {
		return fmt.Errorf("ticket title must contain at least one alphanumeric character")
	}

	parentDir := filepath.Dir(ticket.Slug)
	if parentDir != "." {
		slug = filepath.ToSlash(filepath.Join(parentDir, slug))
	}

	newFolderPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, slug)
	if slug != ticket.Slug {
		if _, err := os.Lstat(newFolderPath); err == nil {
			return fmt.Errorf("ticket folder already exists: %s", newFolderPath)
		} else if !os.IsNotExist(err) {
			return err
		}
		if err := EnsureDir(filepath.Dir(newFolderPath)); err != nil {
			return err
		}
		if err := os.Rename(ticket.FolderPath, newFolderPath); err != nil {
			return err
		}
	}
	ticket.Title = title
	ticket.Slug = slug
	ticket.FolderPath = newFolderPath
	ticket.JsonPath = GetTicketJsonPath(ticket.Year, ticket.Month, ticket.Day, slug)
	ticket.ImportantPath = GetImportantFilePath(ticket.Year, ticket.Month, ticket.Day, slug)
	return nil
}

// 🔗️linkTicketIssueToGoalIssue links a ticket issue under the nearest ancestor goal issue.
func linkTicketIssueToGoalIssue(issueURL, goalRef string) {
	goalPath := goalIDForFilesystem(goalRef)
	for goalPath != "" {
		goal, err := ReadGoal(goalPath)
		if err != nil {
			return
		}
		if goal.Management != nil && goal.Management.Issue != "" {
			if err := GetManagementProvider().AddSubIssue(goal.Management.Issue, issueURL); err != nil {
				writeWarningf("Failed to link ticket issue to goal issue %s: %v", goal.Management.Issue, err)
			}
			return
		}
		goalPath = goal.Parent
	}
}

// 🐙️ensureTicketGitHubIssue creates, links, or reopens the GitHub issue for a ticket.
func ensureTicketGitHubIssue(ticket *Ticket, title, prompt, goalRef, issue string, noManagement, reopenIfClosed bool) error {
	if noManagement {
		return nil
	}
	provider := GetManagementProvider()
	if issue != "" {
		ticket.Management = &TicketManagementData{Issue: issue}
		provider.AddIssueToProject(issue)
		linkTicketIssueToGoalIssue(issue, goalRef)
		return nil
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		issueURL := ticket.Management.Issue
		provider.AddIssueToProject(issueURL)
		if reopenIfClosed {
			if remote, err := provider.GetIssueDetails(issueURL); err == nil && strings.EqualFold(remote.State, "CLOSED") {
				if err := provider.ReopenIssue(issueURL); err != nil {
					return fmt.Errorf("reopen github issue: %w", err)
				}
			}
		}
		linkTicketIssueToGoalIssue(issueURL, goalRef)
		return nil
	}
	issueBody := formatPromptHeading(prompt)
	var milestone *int
	if goalRef != "" {
		milestone, _ = getRootGoalMilestone(goalRef)
	}
	issueURL, err := provider.CreateIssue(title, issueBody, milestone)
	if err != nil {
		return err
	}
	if issueURL == "" {
		return fmt.Errorf("github issue create returned empty url")
	}
	ticket.Management = &TicketManagementData{Issue: issueURL}
	linkTicketIssueToGoalIssue(issueURL, goalRef)
	return nil
}

// 🆕️CreateTicket MUST persist the new entity and return a reference to it.
// 🆕️CreateTicket creates a new ticket and persists it.
func CreateTicket(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind McpClientKind, planID, specID string) (*Ticket, error) {
	if goal == "" {
		return nil, fmt.Errorf("ticket goal is required")
	}
	slug, err := validateTicketEmojiTitle(emoji, title)
	if err != nil {
		return nil, err
	}
	emoji = strings.TrimSpace(emoji)
	title = strings.TrimSpace(title)

	now := time.Now()
	year, month, day := FormatDate(now)

	if parent != "" {
		pTicket, err := FindTicketBySlug(parent)
		if err != nil {
			return nil, fmt.Errorf("failed to find parent ticket '%s': %w", parent, err)
		}
		year = pTicket.Year
		month = pTicket.Month
		day = pTicket.Day
		slug = filepath.ToSlash(filepath.Join(pTicket.Slug, slug))
	}

	var llmSlug string
	if llm != "" {
		llmSlug, err = ResolveAllowedLLM(llm)
		if err != nil {
			return nil, err
		}
	}
	var effortSlug string
	if effort != "" {
		effortSlug, err = ResolveAllowedEffort(effort)
		if err != nil {
			return nil, err
		}
	}
	uiSlug, err := ResolveAllowedClient(client)
	if err != nil {
		return nil, err
	}

	ticketDir := GetTicketPath(year, month, day, slug)
	if _, err := os.Lstat(ticketDir); err == nil {
		return nil, fmt.Errorf("ticket folder already exists: %s", ticketDir)
	} else if !os.IsNotExist(err) {
		return nil, err
	}
	if err := EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	jsonPath := GetTicketJsonPath(year, month, day, slug)

	importantFilePath := GetImportantFilePath(year, month, day, slug)

	if draft != "" {
		draftPath := filepath.Join(GetDraftsPath(), draft)
		if IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticketDir, entry.Name())
					if err := MoveFile(src, dst); err != nil {
						writeWarningf("Failed to move draft file %s: %v", entry.Name(), err)
						continue
					}
				}
			}
			os.RemoveAll(draftPath)
		}
	}

	if err := WriteTextFile(importantFilePath, ""); err != nil {
		return nil, fmt.Errorf("failed to write important file: %w", err)
	}

	ticket := &Ticket{
		Year:          year,
		Month:         month,
		Day:           day,
		Slug:          slug,
		Title:         title,
		Emoji:         emoji,
		Status:        TicketStatusOpen,
		Description:   prompt,
		Goal:          goal,
		Parent:        parent,
		FolderPath:    ticketDir,
		JsonPath:      jsonPath,
		ImportantPath: importantFilePath,
		Interactions: []Interaction{{
			Kind:   "ticket.open",
			Prompt: prompt,
			LLM:    llmSlug,
			Effort: effortSlug,
			System: GetSystem(),
			Client: uiSlug,
			Date:   now.Format("2006-01-02 15:04:05"),
		}},
	}
	appendTicketSessionID(ticket, currentTicketSessionID())

	if err := ApplyTicketPlanFromIDs(ticket, mcpKind, planID, specID); err != nil {
		return nil, err
	}

	if !noIssue {
		if err := ensureTicketGitHubIssue(ticket, title, prompt, goal, issue, noManagement, false); err != nil {
			writeWarningf("Failed to create GitHub issue: %v", err)
		}
	}

	postTicketPlanComment(ticket, noManagement)

	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	ticketID := FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	repopkg.Emit(repopkg.EventTicketOpenEnded, "repo-cli", repopkg.TicketOpenPayload{
		TicketPayload: repopkg.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Title:         ticket.Title, Prompt: ticket.Description, LLM: llmSlug, Effort: effortSlug, Client: uiSlug, Author: GetGitAuthorAlias(), Goal: goal, Parent: parent,
	})
	return ticket, nil
}

// 📌️ghGetMilestoneTitle holds the data fields for a ghGetMilestoneTitle record.
func ghGetMilestoneTitle(number int) (string, error) {
	stdout, stderr, exitCode := ExecCommand("gh", []string{"api", fmt.Sprintf("repos/{owner}/{repo}/milestones/%d", number), "--jq", ".title"}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh api milestone failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 🔗️ghExtractIssueURL parses a GitHub issue URL from `gh issue create` output.
func ghExtractIssueURL(output string) string {
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
func ghCreateIssue(title, body string, milestone *int) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", body, "--label", "ticket"}
	if milestone != nil {
		milestoneTitle, err := ghGetMilestoneTitle(*milestone)
		if err != nil {
			writeWarningf("could not resolve milestone %d, creating issue without milestone: %v", *milestone, err)
		} else {
			args = append(args, "--milestone", milestoneTitle)
		}
	}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := ghExtractIssueURL(stdout)
	if issueURL == "" {
		issueURL = ghExtractIssueURL(stderr)
	}
	if issueURL == "" {
		return "", fmt.Errorf("gh issue create succeeded but no issue url in output: stdout=%q stderr=%q", strings.TrimSpace(stdout), strings.TrimSpace(stderr))
	}
	ghAddIssueToProject(issueURL)
	ghAssignIssueToCurrentUser(issueURL)
	return issueURL, nil
}

// 🛠️buildTechnologyLinkArgs holds the data fields for a buildTechnologyLinkArgs record.
func buildTechnologyLinkArgs(issueURL string) []string {
	return []string{"project", "item-add", "2", "--owner", "usalu", "--url", issueURL}
}

// ➕️ghAddIssueToProject holds the data fields for a ghAddIssueToProject record.
func ghAddIssueToProject(issueURL string) {
	if issueURL == "" {
		return
	}
	ExecCommand("gh", buildTechnologyLinkArgs(issueURL), "")
}

// 👤️ghGetCurrentUser holds the data fields for a ghGetCurrentUser record.
func ghGetCurrentUser() string {
	stdout, _, exitCode := ExecCommand("gh", []string{"api", "user", "--jq", ".login"}, "")
	if exitCode != 0 {
		return ""
	}
	return strings.TrimSpace(stdout)
}

// 🔺️ghAssignIssueToCurrentUser holds the data fields for a ghAssignIssueToCurrentUser record.
func ghAssignIssueToCurrentUser(issueURL string) {
	if issueURL == "" {
		return
	}
	user := ghGetCurrentUser()
	if user == "" {
		return
	}
	ExecCommand("gh", []string{"issue", "edit", issueURL, "--add-assignee", user}, "")
}
