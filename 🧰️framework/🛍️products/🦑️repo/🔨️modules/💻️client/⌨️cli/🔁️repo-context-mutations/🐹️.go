// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo context ticket, folder, file, section, and contributor mutations.

// #endregion 🧲️Header

package client

import (
	"errors"
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// 🟩applySystemAutofixes holds the data fields for a applySystemAutofixes record.
func applySystemAutofixes(breachs []Breach) (int, error) {
	fixed := 0
	for _, v := range breachs {
		switch v.Kind {
		case BreachFolderIllegalEmpty:
			folderPath := filepath.Join(rootDir, v.Excerpt)
			entries, readErr := os.ReadDir(folderPath)
			if readErr == nil && len(entries) == 0 {
				if err := os.Remove(folderPath); err == nil {
					fixed++
				}
			}
		}
	}
	return fixed, nil
}

// findMatchingSectionStartName locates the section start name for autofix helpers.
func findMatchingSectionStartName(lines []string, endLineIdx int, language LanguagePlugin) string {
	depth := 0
	for i := endLineIdx - 1; i >= 0; i-- {
		if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
			depth++
			continue
		}
		if matched, name := language.PolicySectionStartMatch(lines[i]); matched {
			if depth > 0 {
				depth--
				continue
			}
			return name
		}
	}
	return ""
}

// 📬️TicketOpen MUST return a non-nil error when the operation fails.
// ⚫️TicketOpen performs the ticket open operation on the repo context.
func (c *repoContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	resolvedClient, err := ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := McpKindFromResolvedClient(resolvedClient)
	return OpenTicket(input.Emoji, input.Title, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.NoIssue, input.Goal, input.Parent, input.NoManagement, input.Issue, kind, input.PlanID, input.SpecID)
}

// 🟦️TicketClose MUST return a non-nil error when the operation fails.
// 🩵️TicketClose performs the ticket close operation on the repo context.
func (c *repoContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	if input.All {
		tickets, err := ListTickets(nil, nil, nil)
		if err != nil {
			return nil, err
		}
		var lastTicket *Ticket
		for _, t := range tickets {
			if t.Status == TicketStatusOpen {
				ticket := t
				fmt.Printf("Closing ticket %s...\n", ticket.Slug)
				if err := FinishTicket(&ticket, "Bulk close", []string{}, input.NoManagement, true); err != nil {
					writeWarningf("Failed to close ticket %s: %v", ticket.Slug, err)
					continue
				}
				lastTicket = &ticket
			}
		}

		if !input.NoManagement {
			issueURLs, err := c.managementProvider.ListOpenIssuesWithLabel("ticket")
			if err != nil {
				writeWarningf("Failed to list GitHub issues with 'ticket' label: %v", err)
			} else {
				for _, issueURL := range issueURLs {
					fmt.Printf("Closing GitHub issue %s...\n", issueURL)
					if err := c.managementProvider.CloseIssue(issueURL); err != nil {
						writeWarningf("Failed to close GitHub issue %s: %v", issueURL, err)
					}
				}
			}
		}
		return lastTicket, nil
	}
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
	if err := FinishTicket(ticket, input.Summary, input.Files, input.NoManagement, false); err != nil {
		return nil, err
	}
	return ticket, nil
}

// 🟪️TicketReopen MUST return a non-nil error when the operation fails.
// 🩶️TicketReopen performs the ticket reopen operation on the repo context.
func (c *repoContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
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
	resolvedClient, err := ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := McpKindFromResolvedClient(resolvedClient)
	if err := ReopenTicket(ticket, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.Goal, input.Parent, input.NoManagement, kind, input.PlanID, input.SpecID); err != nil {
		return nil, err
	}
	return ticket, nil
}

// 🟫️FolderCreate MUST return a non-nil error when the operation fails.
// 🩷️FolderCreate performs the folder create operation on the repo context.
func (c *repoContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

// 🚚️FolderMove MUST return a non-nil error when the operation fails.
// 💜️FolderMove performs the folder move operation on the repo context.
func (c *repoContext) FolderMove(src, dst string) (*Folder, error) {
	result := ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &Folder{ID: ctx.GetFolderID(dst), Path: dst, Name: filepath.Base(dst)}, nil
}

// 💠️FolderDelete MUST return a non-nil error when the operation fails.
// 💙️FolderDelete performs the folder delete operation on the repo context.
func (c *repoContext) FolderDelete(path string) error {
	result := ToolFolderDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🔳️FileCreate MUST return a non-nil error when the operation fails.
// 💚️FileCreate performs the file create operation on the repo context.
func (c *repoContext) FileCreate(path string) (*File, error) {
	result := ToolFileCreate(path)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &File{ID: ctx.GetFileID(path), Path: path, Name: filepath.Base(path), Extension: strings.TrimPrefix(filepath.Ext(path), ".")}, nil
}

// 🔲️FileMove MUST return a non-nil error when the operation fails.
// 💛️FileMove performs the file move operation on the repo context.
func (c *repoContext) FileMove(src, dst string) (*File, error) {
	result := ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &File{ID: ctx.GetFileID(dst), Path: dst, Name: filepath.Base(dst), Extension: strings.TrimPrefix(filepath.Ext(dst), ".")}, nil
}

// ▪️FileDelete MUST return a non-nil error when the operation fails.
// 🧡️FileDelete performs the file delete operation on the repo context.
func (c *repoContext) FileDelete(path string) error {
	result := ToolFileDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// ▫️SectionCreate MUST return a non-nil error when the operation fails.
// ❤️SectionCreate performs the section create operation on the repo context.
func (c *repoContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := buildSectionID(fileID, strings.Split(sectionPath, "/"))
	return &Section{ID: id, Name: name, Path: sectionPath, FilePath: file}, nil
}

// ◾SectionMove MUST return a non-nil error when the operation fails.
// 🤍️SectionMove performs the section move operation on the repo context.
func (c *repoContext) SectionMove(file, oldName, newName string) (*Section, error) {
	result := ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := buildSectionID(fileID, strings.Split(newName, "/"))
	return &Section{ID: id, Name: newName, Path: newName, FilePath: file}, nil
}

// ◽SectionDelete MUST return a non-nil error when the operation fails.
// 🖤️SectionDelete performs the section delete operation on the repo context.
func (c *repoContext) SectionDelete(file, name string) error {
	result := ToolSectionDelete(file, name)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🧬️Integrate MUST return a non-nil error when the operation fails.
// 🧬️Integrate performs the integrate operation on the repo context.
func (c *repoContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	s := ""
	if source != nil {
		s = *source
	}
	ts := ""
	if targetSection != nil {
		ts = *targetSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}
	tp := ""
	if targetParent != nil {
		tp = *targetParent
	}
	result := ToolIntegrate(s, ts, tf, tp)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// 🧲️Extract MUST return the extracted component from the input.
// 🤎️Extract extracts the extract from the source.
func (c *repoContext) Extract(sourceFile, sourceSection, targetFile *string) (*File, error) {
	s := ""
	if sourceFile != nil {
		s = *sourceFile
	}
	ss := ""
	if sourceSection != nil {
		ss = *sourceSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}

	result := ToolExtract(s, ss, tf)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// ➕️ContributorAdd MUST return a non-nil error when the operation fails.
// ➕️ContributorAdd performs the contributor add operation on the repo context.
func (c *repoContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	contributor, err := LoadContributor(input.Github)
	if err != nil {
		contributor, err = CreateContributor(input.Github)
		if err != nil {
			return nil, err
		}
	}

	changed := false
	if input.Name != nil && *input.Name != "" && contributor.Name == "" {
		contributor.Name = *input.Name
		changed = true
	}
	for _, n := range input.Names {
		if !slices.Contains(contributor.Names, n) {
			contributor.Names = append(contributor.Names, n)
			changed = true
		}
	}
	if input.Email != nil && *input.Email != "" && contributor.Email == "" {
		contributor.Email = *input.Email
		changed = true
	}
	for _, e := range input.Emails {
		if !slices.Contains(contributor.Emails, e) {
			contributor.Emails = append(contributor.Emails, e)
			changed = true
		}
	}
	if input.Fingerprint != nil && *input.Fingerprint != "" && contributor.Fingerprint == "" {
		contributor.Fingerprint = *input.Fingerprint
		changed = true
	}
	for _, f := range input.Fingerprints {
		if !slices.Contains(contributor.Fingerprints, f) {
			contributor.Fingerprints = append(contributor.Fingerprints, f)
			changed = true
		}
	}

	if changed {
		if err := SaveContributor(*contributor); err != nil {
			return nil, err
		}
		repopkg.Emit(repopkg.EventContributorAddEnded, "repo-cli", repopkg.ContributorPayload{
			Github: contributor.Github, Author: GetGitAuthorAlias(),
		})
	}
	return contributor, nil
}

// ➖️ContributorRemove MUST return a non-nil error when the operation fails.
// 💗️ContributorRemove performs the contributor remove operation on the repo context.
func (c *repoContext) ContributorRemove(github string) error {
	repopkg.Emit(repopkg.EventContributorRemoveEnded, "repo-cli", repopkg.ContributorPayload{
		Github: github, Author: GetGitAuthorAlias(),
	})
	return nil
}

func updateGoalMilestone(goal *Goal, number int) (int, error) {
	repoUrl, err := getGhRepoUrl()
	if err != nil {
		return number, err
	}
	if goal.Management == nil {
		goal.Management = &GoalManagementData{}
	}
	milestoneUrl := fmt.Sprintf("%s/milestone/%d", repoUrl, number)
	if goal.Management.Milestone != milestoneUrl {
		goal.Management.Milestone = milestoneUrl
		if err := SaveGoal(*goal); err != nil {
			return number, err
		}
	}
	return number, nil
}

// ◻ensureGoalMilestone holds the data fields for a ensureGoalMilestone record.
func ensureGoalMilestone(goal *Goal) (*ManagementMilestone, error) {
	if goal == nil {
		return nil, nil
	}
	if strings.TrimSpace(goal.Title) == "" {
		return nil, nil
	}
	if !isRootGoal(goal) {
		return nil, nil
	}
	if goal.Management != nil && goal.Management.Milestone != "" {
		if n, err := parseMilestoneNumber(goal.Management.Milestone); err == nil && n > 0 {
			if milestone, err := GetManagementProvider().GetMilestone(n); err == nil {
				return milestone, nil
			}
		}
	}
	found, err := GetManagementProvider().FindMilestoneByTitle(goal.Title)
	if err != nil {
		return nil, err
	}
	if found != nil {
		if _, err := updateGoalMilestone(goal, found.Number); err != nil {
			return nil, err
		}
		return found, nil
	}
	n, err := GetManagementProvider().CreateMilestone(goal.Title, goal.Description)
	if err != nil {
		return nil, err
	}
	if goal.Dates.Due != "" {
		_ = GetManagementProvider().UpdateMilestone(n, "", "", "", goal.Dates.Due)
	}
	if _, err := updateGoalMilestone(goal, n); err != nil {
		return nil, err
	}
	milestone, err := GetManagementProvider().GetMilestone(n)
	if err != nil {
		return &ManagementMilestone{Number: n, Title: goal.Title, Description: goal.Description}, nil
	}
	return milestone, nil
}
