// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket finish and reopen.

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

// 🔖️FinishTicket MUST return a non-nil error when the operation fails.
func FinishTicket(ticket *Ticket, summary string, files []string, noManagement bool, bulk bool) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	if ticket.Status != TicketStatusOpen {
		return fmt.Errorf("ticket is not open")
	}
	files = normalizeTicketFileInputs(files)
	if !bulk {
		if summary == "" {
			return fmt.Errorf("summary is required to finish a ticket")
		}
		if len(files) == 0 {
			return fmt.Errorf("at least one file is required to finish a ticket")
		}
	} else {
		if summary == "" {
			summary = "Bulk close"
		}
	}

	importantPreimage, err := inspectTicketImportantDocument(ticket)
	if err != nil {
		return err
	}

	var tickFilesResult *TicketDiffs
	if (!bulk || len(files) > 0) && !noManagement {
		tickFilesResult, err = ComputeTicketFiles(ticket, files)
		if err != nil {
			return err
		}
	}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		return err
	}

	if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
		issueURL := ticket.Management.Issue

		if !bulk {

			bundles := GetTechnologies()
			labels := make(map[string]struct{})
			if tickFilesResult != nil {
				addLabel := func(path string) {
					if path == "" {
						return
					}
					labels[path] = struct{}{}
				}
				bundleDiffs := tickFilesResult.Bundles
				for _, entry := range bundleDiffs.Added {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Modified {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Deleted {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Renamed {
					addLabel(entry.From)
					addLabel(entry.To)
				}
			}
			var labelList []string
			for l := range labels {
				labelList = append(labelList, l)
			}
			if len(labelList) > 0 {
				if err := GetManagementProvider().AddLabels(issueURL, labelList); err != nil {
					writeWarningf("Failed to add labels to GitHub issue: %v", err)
				}
			}

			comment := formatSummaryHeading(summary)
			metricsComment := generateMetricsComment(tickFilesResult, bundles)
			if metricsComment != "" {
				comment += "\n\n#✍️Changes\n\n" + metricsComment
			}

			if err := GetManagementProvider().AddComment(issueURL, comment); err != nil {
				writeWarningf("Failed to add summary and metrics comment to GitHub issue: %v", err)
			}
		}

		if err := GetManagementProvider().CloseIssue(issueURL); err != nil {
			writeWarningf("Failed to close GitHub issue: %v", err)
		}
	}

	now := time.Now()
	nowStr := now.Format("2006-01-02 15:04:05")
	closeClient := ""
	if len(ticket.Interactions) > 0 {
		closeClient = ticket.Interactions[len(ticket.Interactions)-1].Client
	}
	var interactionFiles []InteractionFile
	seenInteractionFiles := make(map[string]struct{})
	addInteractionFile := func(path string) {
		if _, ok := seenInteractionFiles[path]; ok {
			return
		}
		seenInteractionFiles[path] = struct{}{}
		interactionFiles = append(interactionFiles, InteractionFile{Path: path})
	}
	if tickFilesResult != nil {
		for _, f := range tickFilesResult.Files.Added {
			addInteractionFile(f.Path)
		}
		for _, f := range tickFilesResult.Files.Modified {
			addInteractionFile(f.Path)
		}
		for _, f := range tickFilesResult.Files.Deleted {
			addInteractionFile(f.Path)
		}
		for _, f := range tickFilesResult.Files.Renamed {
			addInteractionFile(f.To)
		}
	} else {
		for _, path := range files {
			addInteractionFile(path)
		}
	}
	closeInteraction := Interaction{
		Kind:       "ticket.close",
		Author:     GetGitAuthorAlias(),
		System:     GetSystem(),
		Client:     closeClient,
		Checkpoint: GetGitCheckpoint(),
		Date:       nowStr,
		Summary:    summary,
		Files:      interactionFiles,
	}
	if _, err := inspectTicketImportantDocument(ticket); err != nil {
		return err
	}
	if err := removeTicketImportantDocument(importantPreimage); err != nil {
		return fmt.Errorf("remove important document: %w", err)
	}
	previousSummary := ticket.Summary
	previousStatus := ticket.Status
	previousInteractions := append([]Interaction(nil), ticket.Interactions...)
	ticket.Summary = summary
	ticket.Status = TicketStatusClosed
	ticket.Interactions = append(ticket.Interactions, closeInteraction)
	if err := SaveTicket(ticket); err != nil {
		ticket.Summary = previousSummary
		ticket.Status = previousStatus
		ticket.Interactions = previousInteractions
		if restoreErr := restoreTicketImportantDocument(importantPreimage); restoreErr != nil {
			return fmt.Errorf("save ticket: %w; restore important document: %v", err, restoreErr)
		}
		return err
	}
	if err := purgeOversizedTicketArtifacts(ticket.FolderPath); err != nil {
		writeWarningf("Failed to purge oversized ticket artifacts: %v", err)
	}
	ticketID := FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	fileList := files
	if len(fileList) == 0 {
		for _, f := range interactionFiles {
			fileList = append(fileList, f.Path)
		}
	}
	repopkg.Emit(repopkg.EventTicketCloseEnded, "repo-cli", repopkg.TicketClosePayload{
		TicketPayload: repopkg.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Summary:       summary, Files: fileList, Author: GetGitAuthorAlias(),
	})
	return nil
}

// 🔖️ReopenTicket MUST return a non-nil error when the operation fails.
func ReopenTicket(ticket *Ticket, prompt, llm, effort, client, draft string, goal string, parent string, noManagement bool, mcpKind McpClientKind, planID, specID string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	if ticket.Status != TicketStatusClosed {
		return fmt.Errorf("ticket is already open")
	}
	nextGoal := ticket.Goal
	if goal != "" {
		nextGoal = goal
	}
	nextParent := ticket.Parent
	if parent != "" {
		nextParent = parent
	}
	var llmSlug string
	var effortSlug string
	var uiSlug string
	var err error
	if llm != "" {
		llmSlug, err = ResolveAllowedLLM(llm)
		if err != nil {
			return err
		}
	}
	if effort != "" {
		effortSlug, err = ResolveAllowedEffort(effort)
		if err != nil {
			return err
		}
	}
	if client != "" {
		uiSlug, err = ResolveAllowedClient(client)
		if err != nil {
			return err
		}
	} else {
		return fmt.Errorf("client is required")
	}
	if err := ApplyTicketPlanFromIDs(ticket, mcpKind, planID, specID); err != nil {
		return err
	}

	interaction := Interaction{
		Kind:   "ticket.reopen",
		Prompt: prompt,
		LLM:    llmSlug,
		Effort: effortSlug,
		System: GetSystem(),
		Client: uiSlug,
		Date:   time.Now().Format("2006-01-02 15:04:05"),
	}

	if draft != "" {
		draftPath := filepath.Join(GetDraftsPath(), draft)
		if IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticket.FolderPath, entry.Name())

					if FileExists(dst) {
						ext := filepath.Ext(entry.Name())
						name := strings.TrimSuffix(entry.Name(), ext)
						for i := 2; ; i++ {
							newDst := filepath.Join(ticket.FolderPath, fmt.Sprintf("%s_%d%s", name, i, ext))
							if !FileExists(newDst) {
								dst = newDst
								break
							}
						}
					}
					if err := MoveFile(src, dst); err != nil {
						writeWarningf("Failed to move draft file %s: %v", entry.Name(), err)
						continue
					}
				}
			}
			os.RemoveAll(draftPath)
		}
	}

	importantCreation, err := ensureTicketImportantDocument(ticket)
	if err != nil {
		return err
	}
	previousGoal := ticket.Goal
	previousParent := ticket.Parent
	previousStatus := ticket.Status
	previousInteractions := append([]Interaction(nil), ticket.Interactions...)
	previousSessions := append([]string(nil), ticket.Sessions...)
	ticket.Goal = nextGoal
	ticket.Parent = nextParent
	ticket.Interactions = append(ticket.Interactions, interaction)
	appendTicketSessionID(ticket, currentTicketSessionID())
	ticket.Status = TicketStatusOpen

	if err := ensureTicketGitHubIssue(ticket, ticket.Title, prompt, ticket.Goal, "", noManagement, true); err != nil {
		writeWarningf("Failed to ensure GitHub issue: %v", err)
	}
	if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
		issueURL := ticket.Management.Issue
		comment := formatPromptHeading(prompt)
		if err := GetManagementProvider().AddComment(issueURL, comment); err != nil {
			writeWarningf("Failed to add prompt comment to GitHub issue: %v", err)
		}
	}

	postTicketPlanComment(ticket, noManagement)

	if err := SaveTicket(ticket); err != nil {
		ticket.Goal = previousGoal
		ticket.Parent = previousParent
		ticket.Status = previousStatus
		ticket.Interactions = previousInteractions
		ticket.Sessions = previousSessions
		if rollbackErr := rollbackTicketImportantCreation(importantCreation); rollbackErr != nil {
			return fmt.Errorf("save ticket: %w; rollback important document: %v", err, rollbackErr)
		}
		return err
	}
	ticketID := FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	repopkg.Emit(repopkg.EventTicketReopenEnded, "repo-cli", repopkg.TicketReopenPayload{
		TicketPayload: repopkg.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Prompt:        prompt, LLM: llmSlug, Effort: effortSlug, Client: uiSlug, Author: GetGitAuthorAlias(),
	})
	return nil
}
