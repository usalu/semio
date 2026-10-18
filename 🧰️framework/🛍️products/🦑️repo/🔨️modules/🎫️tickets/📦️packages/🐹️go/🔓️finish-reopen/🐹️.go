package tickets

import (
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔓️Finish Reopen

const ticketOversizedFileBytes = 5 << 20

const ticketOversizedFolderBytes = 10 << 20

// 🧹️purgeOversizedTicketArtifacts deletes files above 5 MiB and subfolders above 10 MiB inside a closed ticket folder.
func PurgeOversizedTicketArtifacts(ticketDir string) error {
	ticketDir = strings.TrimSpace(ticketDir)
	if ticketDir == "" {
		return nil
	}
	absDir, err := filepath.Abs(ticketDir)
	if err != nil {
		return err
	}
	info, err := os.Stat(absDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return err
	}
	if !info.IsDir() {
		return fmt.Errorf("ticket path is not a directory: %s", absDir)
	}

	type fileEntry struct {
		path string
		size int64
	}
	dirSizes := make(map[string]int64)
	var dirs []string
	var files []fileEntry

	walkErr := filepath.WalkDir(absDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			workspace.WriteWarningf("Failed to walk ticket folder %s: %v", path, err)
			return nil
		}
		if !ticketPathContained(absDir, path) {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.Type()&fs.ModeSymlink != 0 {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.IsDir() {
			if path != absDir {
				dirs = append(dirs, path)
			}
			return nil
		}
		fi, err := os.Lstat(path)
		if err != nil {
			workspace.WriteWarningf("Failed to stat ticket artifact %s: %v", path, err)
			return nil
		}
		if fi.Mode()&fs.ModeSymlink != 0 {
			return nil
		}
		size := fi.Size()
		files = append(files, fileEntry{path: path, size: size})
		for parent := filepath.Dir(path); ticketPathContained(absDir, parent); parent = filepath.Dir(parent) {
			dirSizes[parent] += size
			if parent == absDir {
				break
			}
		}
		return nil
	})
	if walkErr != nil {
		return walkErr
	}

	sort.Slice(dirs, func(i, j int) bool {
		return strings.Count(dirs[i], string(os.PathSeparator)) > strings.Count(dirs[j], string(os.PathSeparator))
	})

	deleted := make(map[string]bool)
	for _, dir := range dirs {
		if ticketArtifactUnderDeleted(dir, deleted) {
			continue
		}
		if dirSizes[dir] <= ticketOversizedFolderBytes {
			continue
		}
		if err := os.RemoveAll(dir); err != nil {
			workspace.WriteWarningf("Failed to delete oversized ticket folder %s: %v", dir, err)
			continue
		}
		deleted[dir] = true
	}

	ticketJSONBase := "🎫️ticket.json"
	for _, file := range files {
		if ticketArtifactUnderDeleted(file.path, deleted) {
			continue
		}
		if filepath.Base(file.path) == ticketJSONBase {
			continue
		}
		if file.size <= ticketOversizedFileBytes {
			continue
		}
		if err := os.Remove(file.path); err != nil {
			workspace.WriteWarningf("Failed to delete oversized ticket file %s: %v", file.path, err)
		}
	}
	return nil
}

// 🧹️collectTicketFolderRoots returns every slug directory under the dated ticket tree.
func collectTicketFolderRoots() ([]string, error) {
	ticketsDir := GetTicketsDir()
	if !workspace.FileExists(ticketsDir) {
		return nil, nil
	}
	var roots []string
	yearEntries, err := os.ReadDir(ticketsDir)
	if err != nil {
		return nil, err
	}
	for _, yearEntry := range yearEntries {
		if !yearEntry.IsDir() {
			continue
		}
		yearPath := filepath.Join(ticketsDir, yearEntry.Name())
		monthEntries, err := os.ReadDir(yearPath)
		if err != nil {
			continue
		}
		for _, monthEntry := range monthEntries {
			if !monthEntry.IsDir() {
				continue
			}
			monthPath := filepath.Join(yearPath, monthEntry.Name())
			dayEntries, err := os.ReadDir(monthPath)
			if err != nil {
				continue
			}
			for _, dayEntry := range dayEntries {
				if !dayEntry.IsDir() {
					continue
				}
				dayPath := filepath.Join(monthPath, dayEntry.Name())
				slugEntries, err := os.ReadDir(dayPath)
				if err != nil {
					continue
				}
				for _, slugEntry := range slugEntries {
					if !slugEntry.IsDir() {
						continue
					}
					if strings.HasPrefix(slugEntry.Name(), ".") {
						continue
					}
					roots = append(roots, filepath.Join(dayPath, slugEntry.Name()))
				}
			}
		}
	}
	return roots, nil
}

// 🧹️PurgeAllOversizedTicketArtifacts deletes oversized artifacts in every ticket folder.
func PurgeAllOversizedTicketArtifacts() (int, error) {
	roots, err := collectTicketFolderRoots()
	if err != nil {
		return 0, err
	}
	purged := 0
	for _, root := range roots {
		if err := PurgeOversizedTicketArtifacts(root); err != nil {
			return purged, err
		}
		purged++
	}
	return purged, nil
}

// 🛡️ticketPathContained reports whether path stays inside the ticket root directory.
func ticketPathContained(root, path string) bool {
	rel, err := filepath.Rel(root, path)
	if err != nil {
		return false
	}
	return rel != ".." && !strings.HasPrefix(rel, ".."+string(filepath.Separator))
}

// 🗑️ticketArtifactUnderDeleted reports whether path lies inside a deleted ticket subtree.
func ticketArtifactUnderDeleted(path string, deleted map[string]bool) bool {
	for dir := range deleted {
		if path == dir || strings.HasPrefix(path, dir+string(os.PathSeparator)) {
			return true
		}
	}
	return false
}

// 🔖️FinishTicket MUST return a non-nil error when the operation fails.
func FinishTicket(ticket *model.Ticket, summary string, files []string, noManagement bool, bulk bool) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	if ticket.Status != model.TicketStatusOpen {
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

	var tickFilesResult *model.TicketDiffs
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

			bundles := codebase.GetTechnologies()
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
				if err := providers.GetManagementProvider().AddLabels(issueURL, labelList); err != nil {
					workspace.WriteWarningf("Failed to add labels to GitHub issue: %v", err)
				}
			}

			comment := formatSummaryHeading(summary)
			metricsComment := generateMetricsComment(tickFilesResult, bundles)
			if metricsComment != "" {
				comment += "\n\n#✍️Changes\n\n" + metricsComment
			}

			if err := providers.GetManagementProvider().AddComment(issueURL, comment); err != nil {
				workspace.WriteWarningf("Failed to add summary and metrics comment to GitHub issue: %v", err)
			}
		}

		if err := providers.GetManagementProvider().CloseIssue(issueURL); err != nil {
			workspace.WriteWarningf("Failed to close GitHub issue: %v", err)
		}
	}

	now := time.Now()
	nowStr := now.Format("2006-01-02 15:04:05")
	closeClient := ""
	if len(ticket.Interactions) > 0 {
		closeClient = ticket.Interactions[len(ticket.Interactions)-1].Client
	}
	var interactionFiles []model.InteractionFile
	seenInteractionFiles := make(map[string]struct{})
	addInteractionFile := func(path string) {
		if _, ok := seenInteractionFiles[path]; ok {
			return
		}
		seenInteractionFiles[path] = struct{}{}
		interactionFiles = append(interactionFiles, model.InteractionFile{Path: path})
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
	closeInteraction := model.Interaction{
		Kind:       "ticket.close",
		Author:     contributors.GetGitAuthorAlias(),
		System:     languages.GetSystem(),
		Client:     closeClient,
		Checkpoint: workspace.GetGitCheckpoint(),
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
	previousInteractions := append([]model.Interaction(nil), ticket.Interactions...)
	ticket.Summary = summary
	ticket.Status = model.TicketStatusClosed
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
	if err := PurgeOversizedTicketArtifacts(ticket.FolderPath); err != nil {
		workspace.WriteWarningf("Failed to purge oversized ticket artifacts: %v", err)
	}
	ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	fileList := files
	if len(fileList) == 0 {
		for _, f := range interactionFiles {
			fileList = append(fileList, f.Path)
		}
	}
	events.Emit(events.EventTicketCloseEnded, "repo-cli", events.TicketClosePayload{
		TicketPayload: events.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Summary:       summary, Files: fileList, Author: contributors.GetGitAuthorAlias(),
	})
	return nil
}

// 🔖️ReopenTicket MUST return a non-nil error when the operation fails.
func ReopenTicket(ticket *model.Ticket, prompt, llm, effort, client, draft string, goal string, parent string, noManagement bool, mcpKind providers.McpClientKind, planID, specID string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	if ticket.Status != model.TicketStatusClosed {
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
		llmSlug, err = model.ResolveAllowedLLM(llm)
		if err != nil {
			return err
		}
	}
	if effort != "" {
		effortSlug, err = model.ResolveAllowedEffort(effort)
		if err != nil {
			return err
		}
	}
	if client != "" {
		uiSlug, err = model.ResolveAllowedClient(client)
		if err != nil {
			return err
		}
	} else {
		return fmt.Errorf("client is required")
	}
	if err := ApplyTicketPlanFromIDs(NewFileTicketStore(), HostPlanRoots(), ticket, mcpKind, planID, specID); err != nil {
		return err
	}

	interaction := model.Interaction{
		Kind:   "ticket.reopen",
		Prompt: prompt,
		LLM:    llmSlug,
		Effort: effortSlug,
		System: languages.GetSystem(),
		Client: uiSlug,
		Date:   time.Now().Format("2006-01-02 15:04:05"),
	}

	if draft != "" {
		draftPath := filepath.Join(workspace.GetDraftsPath(), draft)
		if workspace.IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticket.FolderPath, entry.Name())

					if workspace.FileExists(dst) {
						ext := filepath.Ext(entry.Name())
						name := strings.TrimSuffix(entry.Name(), ext)
						for i := 2; ; i++ {
							newDst := filepath.Join(ticket.FolderPath, fmt.Sprintf("%s_%d%s", name, i, ext))
							if !workspace.FileExists(newDst) {
								dst = newDst
								break
							}
						}
					}
					if err := move.MoveFile(src, dst); err != nil {
						workspace.WriteWarningf("Failed to move draft file %s: %v", entry.Name(), err)
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
	previousInteractions := append([]model.Interaction(nil), ticket.Interactions...)
	previousSessions := append([]string(nil), ticket.Sessions...)
	ticket.Goal = nextGoal
	ticket.Parent = nextParent
	ticket.Interactions = append(ticket.Interactions, interaction)
	model.AppendTicketSessionID(ticket, contributors.CurrentTicketSessionID())
	ticket.Status = model.TicketStatusOpen

	if err := ensureTicketGitHubIssue(ticket, ticket.Title, prompt, ticket.Goal, "", noManagement, true); err != nil {
		workspace.WriteWarningf("Failed to ensure GitHub issue: %v", err)
	}
	if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
		issueURL := ticket.Management.Issue
		comment := formatPromptHeading(prompt)
		if err := providers.GetManagementProvider().AddComment(issueURL, comment); err != nil {
			workspace.WriteWarningf("Failed to add prompt comment to GitHub issue: %v", err)
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
	ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	events.Emit(events.EventTicketReopenEnded, "repo-cli", events.TicketReopenPayload{
		TicketPayload: events.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
		Prompt:        prompt, LLM: llmSlug, Effort: effortSlug, Client: uiSlug, Author: contributors.GetGitAuthorAlias(),
	})
	return nil
}

// #endregion 🔓️Finish Reopen
