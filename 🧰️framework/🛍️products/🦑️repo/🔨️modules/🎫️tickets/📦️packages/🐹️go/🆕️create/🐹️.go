package tickets

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	goals "github.com/usalu/semio/repo/goals"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🆕️Create

// 🎫️GetTicketsDir MUST return the stored value without modification.
// 📖️GetTicketsDir returns the tickets dir of the value.
func GetTicketsDir() string {
	return filepath.Join(workspace.GetRepoMetaDir(), "🎫️tickets")
}

// 🏪️GetTicketPath MUST return the stored value without modification.
// 📦️GetTicketPath returns the ticket path of the value.
func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), model.FormatYearDir(year), model.FormatMonthDir(month), model.FormatDayDir(day), slug)
}

// 📥️GetImportantFilePath MUST return the stored value without modification.
// 📥️GetImportantFilePath returns the important file path of the value.
func GetImportantFilePath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "📌️important", "📝️.md")
}

type ticketImportantPreimage struct {
	path string
	dir  string
	mode os.FileMode
}

type ticketImportantCreation struct {
	path        string
	dir         string
	fileCreated bool
	dirCreated  bool
}

// 🧷️canonicalTicketImportantPath returns and stores the ticket-owned important document path.
func canonicalTicketImportantPath(ticket *model.Ticket) (string, error) {
	if ticket == nil {
		return "", fmt.Errorf("ticket is nil")
	}
	if ticket.Slug == "" {
		return "", fmt.Errorf("ticket slug is required")
	}
	ticket.ImportantPath = GetImportantFilePath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	return ticket.ImportantPath, nil
}

// 🔎️inspectTicketImportantDocument requires the canonical document to be an exact empty regular-file bundle.
func inspectTicketImportantDocument(ticket *model.Ticket) (ticketImportantPreimage, error) {
	path, err := canonicalTicketImportantPath(ticket)
	if err != nil {
		return ticketImportantPreimage{}, err
	}
	info, err := os.Lstat(path)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: required important document %s is unavailable: %w", path, err)
	}
	if !info.Mode().IsRegular() {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important document %s is not a regular file", path)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: read important document %s: %w", path, err)
	}
	if len(data) != 0 {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important document %s is not empty", path)
	}
	dir := filepath.Dir(path)
	entries, err := os.ReadDir(dir)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: read important directory %s: %w", dir, err)
	}
	if len(entries) != 1 || entries[0].Name() != filepath.Base(path) {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important directory %s must contain only %s", dir, filepath.Base(path))
	}
	return ticketImportantPreimage{path: path, dir: dir, mode: info.Mode()}, nil
}

// 🔄️restoreTicketImportantDocument restores an empty document without overwriting an existing node.
func restoreTicketImportantDocument(preimage ticketImportantPreimage) error {
	if err := os.MkdirAll(preimage.dir, 0755); err != nil {
		return err
	}
	file, err := os.OpenFile(preimage.path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, preimage.mode.Perm())
	if err != nil {
		return err
	}
	return file.Close()
}

// 🗑️removeTicketImportantDocument removes the exact empty document bundle and restores it on partial failure.
func removeTicketImportantDocument(preimage ticketImportantPreimage) error {
	if err := os.Remove(preimage.path); err != nil {
		return err
	}
	if err := os.Remove(preimage.dir); err != nil {
		if restoreErr := restoreTicketImportantDocument(preimage); restoreErr != nil {
			return fmt.Errorf("remove important directory: %w; restore important document: %v", err, restoreErr)
		}
		return fmt.Errorf("remove important directory: %w", err)
	}
	return nil
}

// ✨️ensureTicketImportantDocument preserves an existing regular document or exclusively creates an empty one.
func ensureTicketImportantDocument(ticket *model.Ticket) (ticketImportantCreation, error) {
	path, err := canonicalTicketImportantPath(ticket)
	if err != nil {
		return ticketImportantCreation{}, err
	}
	creation := ticketImportantCreation{path: path, dir: filepath.Dir(path)}
	info, err := os.Lstat(path)
	if err == nil {
		if !info.Mode().IsRegular() {
			return creation, fmt.Errorf("important document %s is not a regular file", path)
		}
		if _, err := os.ReadFile(path); err != nil {
			return creation, fmt.Errorf("read important document %s: %w", path, err)
		}
		return creation, nil
	}
	if !os.IsNotExist(err) {
		return creation, err
	}
	dirInfo, dirErr := os.Lstat(creation.dir)
	if dirErr == nil {
		if !dirInfo.IsDir() || dirInfo.Mode()&os.ModeSymlink != 0 {
			return creation, fmt.Errorf("important directory %s is not a physical directory", creation.dir)
		}
	} else if os.IsNotExist(dirErr) {
		if err := os.MkdirAll(creation.dir, 0755); err != nil {
			return creation, err
		}
		creation.dirCreated = true
	} else {
		return creation, dirErr
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		if creation.dirCreated {
			_ = os.Remove(creation.dir)
		}
		return ticketImportantCreation{}, err
	}
	if err := file.Close(); err != nil {
		_ = os.Remove(path)
		if creation.dirCreated {
			_ = os.Remove(creation.dir)
		}
		return ticketImportantCreation{}, err
	}
	creation.fileCreated = true
	return creation, nil
}

// ↩️rollbackTicketImportantCreation removes only nodes created by the current reopen operation.
func rollbackTicketImportantCreation(creation ticketImportantCreation) error {
	if creation.fileCreated {
		if err := os.Remove(creation.path); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	if creation.dirCreated {
		if err := os.Remove(creation.dir); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	return nil
}

// 🛤️GetTicketJsonPath MUST return the stored value without modification.
// ⬛️GetTicketJsonPath returns the ticket json path of the value.
func GetTicketJsonPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "🎫️ticket.json")
}

// 💿️normalizeTicketKeyword holds the data fields for a normalizeTicketKeyword record.
func normalizeTicketKeyword(value string) string {
	return strings.ToUpper(strings.TrimSpace(value))
}

// 🔷️hasTicketKeyword holds the data fields for a hasTicketKeyword record.
func hasTicketKeyword(text, keyword string) bool {
	return strings.Contains(strings.ToUpper(text), keyword)
}

// 🎯️FindTicketBySlug MUST return nil when no match is found.
// 🔎️FindTicketBySlug searches for and returns the matching ticket by slug.
func FindTicketBySlug(slug string) (*model.Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	for i := len(tickets) - 1; i >= 0; i-- {
		t := tickets[i]
		if t.Slug == slug || filepath.Base(t.Slug) == slug {
			return &t, nil
		}
	}
	return nil, fmt.Errorf("ticket not found: %s", slug)
}

// 🧪️LatestTicket MUST complete the operation and return consistent results.
func LatestTicket() (*model.Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, fmt.Errorf("no tickets found")
	}
	sort.Slice(tickets, func(i, j int) bool {
		if tickets[i].Year != tickets[j].Year {
			return tickets[i].Year > tickets[j].Year
		}
		if tickets[i].Month != tickets[j].Month {
			return tickets[i].Month > tickets[j].Month
		}
		if tickets[i].Day != tickets[j].Day {
			return tickets[i].Day > tickets[j].Day
		}
		return tickets[i].GetDateStarted().After(tickets[j].GetDateStarted())
	})
	return &tickets[0], nil
}

// 🔶️shouldContinueTicket holds the data fields for a shouldContinueTicket record.
func shouldContinueTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "CONTINUE")
}

// 🔹️shouldSkipTicket holds the data fields for a shouldSkipTicket record.
func shouldSkipTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "NOTICKET")
}

// 📬️OpenTicket MUST complete the operation and return consistent results.
func OpenTicket(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind providers.McpClientKind, planID, specID string) (*model.Ticket, error) {
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
		if latest.GetStatus() == model.TicketStatusClosed {
			return latest, ReopenTicket(latest, prompt, llm, effort, client, draft, goal, parent, noManagement, mcpKind, planID, specID)
		}
		return latest, nil
	}
	return CreateTicket(emoji, title, prompt, llm, effort, client, draft, noIssue, goal, parent, noManagement, issue, mcpKind, planID, specID)
}

// ⛳️OpenGoal MUST complete the operation and return consistent results.
func OpenGoal(title, description, prompt, dueDate, client, llm, effort string, noManagement bool) (*model.Goal, error) {
	ctx := model.LookupRepoContext(workspace.GetRootDir())
	input := model.GoalCreateInput{
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
	extracted, remaining := model.ExtractEntityEmoji(emoji)
	if extracted == "" || strings.TrimSpace(remaining) != "" {
		return "", fmt.Errorf("ticket emoji must be a single emoji character")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return "", fmt.Errorf("ticket title is required")
	}
	slug := workspace.Slugify(title)
	if slug == "" {
		return "", fmt.Errorf("ticket title must contain at least one alphanumeric character")
	}
	return slug, nil
}

// 🔁️UpdateTicketTitle MUST complete the operation and return consistent results.
// 🔁️Only validates that the title is non-empty; does NOT enforce any shape on the title string.
func UpdateTicketTitle(ticket *model.Ticket, title string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("ticket title is required")
	}
	slug := workspace.Slugify(title)
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
		if err := workspace.EnsureDir(filepath.Dir(newFolderPath)); err != nil {
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
	goalPath := goals.GoalIDForFilesystem(goalRef)
	for goalPath != "" {
		goal, err := goals.ReadGoal(goalPath)
		if err != nil {
			return
		}
		if goal.Management != nil && goal.Management.Issue != "" {
			if err := providers.GetManagementProvider().AddSubIssue(goal.Management.Issue, issueURL); err != nil {
				workspace.WriteWarningf("Failed to link ticket issue to goal issue %s: %v", goal.Management.Issue, err)
			}
			return
		}
		goalPath = goal.Parent
	}
}

// 🐙️ensureTicketGitHubIssue creates, links, or reopens the GitHub issue for a ticket.
func ensureTicketGitHubIssue(ticket *model.Ticket, title, prompt, goalRef, issue string, noManagement, reopenIfClosed bool) error {
	if noManagement {
		return nil
	}
	provider := providers.GetManagementProvider()
	if issue != "" {
		ticket.Management = &model.TicketManagementData{Issue: issue}
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
		milestone, _ = goals.GetRootGoalMilestone(goalRef)
	}
	issueURL, err := provider.CreateIssue(title, issueBody, milestone)
	if err != nil {
		return err
	}
	if issueURL == "" {
		return fmt.Errorf("github issue create returned empty url")
	}
	ticket.Management = &model.TicketManagementData{Issue: issueURL}
	linkTicketIssueToGoalIssue(issueURL, goalRef)
	return nil
}

// 🆕️CreateTicket MUST persist the new entity and return a reference to it.
// 🆕️CreateTicket creates a new ticket and persists it.
func CreateTicket(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind providers.McpClientKind, planID, specID string) (*model.Ticket, error) {
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
	year, month, day := workspace.FormatDate(now)

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
		llmSlug, err = model.ResolveAllowedLLM(llm)
		if err != nil {
			return nil, err
		}
	}
	var effortSlug string
	if effort != "" {
		effortSlug, err = model.ResolveAllowedEffort(effort)
		if err != nil {
			return nil, err
		}
	}
	uiSlug, err := model.ResolveAllowedClient(client)
	if err != nil {
		return nil, err
	}

	ticketDir := GetTicketPath(year, month, day, slug)
	if _, err := os.Lstat(ticketDir); err == nil {
		return nil, fmt.Errorf("ticket folder already exists: %s", ticketDir)
	} else if !os.IsNotExist(err) {
		return nil, err
	}
	if err := workspace.EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	jsonPath := GetTicketJsonPath(year, month, day, slug)

	importantFilePath := GetImportantFilePath(year, month, day, slug)

	if draft != "" {
		draftPath := filepath.Join(workspace.GetDraftsPath(), draft)
		if workspace.IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticketDir, entry.Name())
					if err := move.MoveFile(src, dst); err != nil {
						workspace.WriteWarningf("Failed to move draft file %s: %v", entry.Name(), err)
						continue
					}
				}
			}
			os.RemoveAll(draftPath)
		}
	}

	if err := workspace.WriteTextFile(importantFilePath, ""); err != nil {
		return nil, fmt.Errorf("failed to write important file: %w", err)
	}

	ticket := &model.Ticket{
		Year:          year,
		Month:         month,
		Day:           day,
		Slug:          slug,
		Title:         title,
		Emoji:         emoji,
		Status:        model.TicketStatusOpen,
		Description:   prompt,
		Goal:          goal,
		Parent:        parent,
		FolderPath:    ticketDir,
		JsonPath:      jsonPath,
		ImportantPath: importantFilePath,
		Interactions: []model.Interaction{{
			Kind:   "ticket.open",
			Prompt: prompt,
			LLM:    llmSlug,
			Effort: effortSlug,
			System: languages.GetSystem(),
			Client: uiSlug,
			Date:   now.Format("2006-01-02 15:04:05"),
		}},
	}
	model.AppendTicketSessionID(ticket, contributors.CurrentTicketSessionID())

	if err := ApplyTicketPlanFromIDs(NewFileTicketStore(), HostPlanRoots(), ticket, mcpKind, planID, specID); err != nil {
		return nil, err
	}

	if !noIssue {
		if err := ensureTicketGitHubIssue(ticket, title, prompt, goal, issue, noManagement, false); err != nil {
			workspace.WriteWarningf("Failed to create GitHub issue: %v", err)
		}
	}

	postTicketPlanComment(ticket, noManagement)

	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	events.Emit(events.EventTicketOpenEnded, "repo-cli", events.TicketOpenPayload{
		TicketPayload: events.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Title:         ticket.Title, Prompt: ticket.Description, LLM: llmSlug, Effort: effortSlug, Client: uiSlug, Author: contributors.GetGitAuthorAlias(), Goal: goal, Parent: parent,
	})
	return ticket, nil
}

// ✔️ListFilesAtCheckpoint MUST return all available files at checkpoint entries.
// 🔢️ListFilesAtCheckpoint returns a list of files at checkpoint entries.
func ListFilesAtCheckpoint(checkpoint string) ([]string, error) {
	if checkpoint == "" {
		files, err := model.ScopeToFiles(workspace.GetRootDir(), workspace.Scope{Kind: workspace.ScopeRepo}, codebase.GetTechnologies())
		if err != nil {
			return nil, err
		}
		return files, nil
	}
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"ls-tree", "-r", "--name-only", checkpoint}, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git ls-tree failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		files = append(files, strings.TrimSpace(line))
	}
	files = model.FilterConsideredFiles(workspace.GetRootDir(), files)
	files = model.FilterGitIgnored(workspace.GetRootDir(), files)
	return files, nil
}

// 📰️formatPromptHeading holds the data fields for a formatPromptHeading record.
func formatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖️ Prompt"
	}
	return "# 🤖️ Prompt\n\n" + body
}

// 📋️formatSummaryHeading holds the data fields for a formatSummaryHeading record.
func formatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍️ Summary"
	}
	return "# 🔍️ Summary\n\n" + body
}

// 📑️replaceSectionContent holds the data fields for a replaceSectionContent record.
func replaceSectionContent(content, sectionHeading, newContent string) string {
	idx := strings.Index(content, sectionHeading)
	if idx == -1 {
		return content
	}
	before := content[:idx]
	after := content[idx+len(sectionHeading):]

	nextHeading := strings.Index(after, "\n## ")
	if nextHeading == -1 {

		return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + "\n"
	}

	rest := after[nextHeading:]
	return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + rest
}

// #endregion 🆕️Create
