package tickets

import (
	json "encoding/json"
	errors "errors"
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

// #region 🧹️Purge Generated Output

// 🧹️TicketGeneratedDirectory is the one folder of a ticket whose content a purge may remove.
const TicketGeneratedDirectory = "🗑️generated"

// 📏️OversizedFileBytes is the size above which a generated file is purged.
const OversizedFileBytes int64 = 5 << 20

// 🗃️OversizedFolderBytes is the size above which a generated folder is purged.
const OversizedFolderBytes int64 = 10 << 20

// ⏱️LiveWriterWindow is how recently a generated file may have been written before its ticket is left alone.
const LiveWriterWindow = 10 * time.Minute

// 🎚️PurgeSelection says which part of an eligible generated folder is removed.
type PurgeSelection string

const (
	// 📏️PurgeOversized removes generated files above 5 MiB and generated folders above 10 MiB.
	PurgeOversized PurgeSelection = "oversized"
	// 🧽️PurgeGenerated removes everything below the generated folder.
	PurgeGenerated PurgeSelection = "generated"
)

// 🗑️PurgeRemoval is one planned removal: a path below the tickets folder, its kind and the bytes it frees.
type PurgeRemoval struct {
	Path  string `json:"path"`
	Kind  string `json:"kind"`
	Bytes int64  `json:"bytes"`
}

// ⏭️PurgeSkip is why a whole ticket is left alone.
type PurgeSkip struct {
	Ticket string `json:"ticket"`
	Reason string `json:"reason"`
}

// 🧹️PurgePlan is what a purge removes (applied) or would remove (dry run), sorted by path and ticket.
type PurgePlan struct {
	Removals []PurgeRemoval `json:"removals"`
	Skips    []PurgeSkip    `json:"skips"`
}

// 🔎️TrackedFiles answers which files below a repository-relative folder the git index tracks.
type TrackedFiles func(root, folder string) (map[string]bool, error)

// 🧭️PurgeOptions is the whole input of a purge; without Apply nothing on disk changes.
type PurgeOptions struct {
	Root       string
	TicketsDir string
	Selection  PurgeSelection
	Apply      bool
	Now        time.Time
	Tracked    TrackedFiles
}

// 🌿️GitTrackedFiles lists the index entries below folder with a read-only git query that cannot climb above root.
func GitTrackedFiles(root, folder string) (map[string]bool, error) {
	command := workspace.ToolCommand("git", "-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--", filepath.ToSlash(folder))
	command.Dir = root
	command.Env = gitQueryEnvironment(command.Env, root)
	output, err := command.Output()
	if err != nil {
		return nil, fmt.Errorf("git ls-files in %s: %w", root, err)
	}
	tracked := make(map[string]bool)
	for _, entry := range strings.Split(string(output), "\x00") {
		if entry != "" {
			tracked[entry] = true
		}
	}
	return tracked, nil
}

// 🧼️gitQueryEnvironment is the process environment without any GIT_ redirection, with discovery capped at root.
func gitQueryEnvironment(base []string, root string) []string {
	environment := []string{"GIT_OPTIONAL_LOCKS=0", "GIT_CEILING_DIRECTORIES=" + filepath.Dir(root)}
	if base == nil {
		base = os.Environ()
	}
	for _, entry := range base {
		if !strings.HasPrefix(strings.ToUpper(entry), "GIT_") {
			environment = append(environment, entry)
		}
	}
	return environment
}

// 🔓️ticketManifestIsClosed reports whether the ticket document says exactly status closed.
func ticketManifestIsClosed(ticketDir string) bool {
	data, err := os.ReadFile(filepath.Join(ticketDir, "🎫️ticket.json"))
	if err != nil {
		return false
	}
	var document struct {
		Status string `json:"status"`
	}
	return json.Unmarshal(data, &document) == nil && document.Status == "closed"
}

// 🌳️CollectTicketFolderRoots returns every slug directory under the dated tickets folder.
func CollectTicketFolderRoots(ticketsDir string) ([]string, error) {
	if !workspace.FileExists(ticketsDir) {
		return nil, nil
	}
	levels := []string{ticketsDir}
	for depth := 0; depth < 4; depth++ {
		var next []string
		for _, parent := range levels {
			entries, err := os.ReadDir(parent)
			if err != nil {
				if depth == 0 {
					return nil, err
				}
				continue
			}
			for _, entry := range entries {
				if entry.IsDir() && (depth < 3 || !strings.HasPrefix(entry.Name(), ".")) {
					next = append(next, filepath.Join(parent, entry.Name()))
				}
			}
		}
		levels = next
	}
	sort.Strings(levels)
	return levels, nil
}

// 📸️generatedSnapshot is the shape of a generated folder at one moment: used to refuse an apply that raced a writer.
type generatedSnapshot struct {
	files  int
	bytes  int64
	newest time.Time
}

// 🟰️equal reports whether two snapshots describe the same files.
func (snapshot generatedSnapshot) equal(other generatedSnapshot) bool {
	return snapshot.files == other.files && snapshot.bytes == other.bytes && snapshot.newest.Equal(other.newest)
}

// 🧹️generatedFile is one regular file below a generated folder.
type generatedFile struct {
	path    string
	size    int64
	tracked bool
}

// 🚶️walkGenerated lists the real directories and regular files below root, never following a link.
func walkGenerated(root string, tracked map[string]bool, repoRoot string) (dirs []string, files []generatedFile, snapshot generatedSnapshot, err error) {
	err = filepath.WalkDir(root, func(path string, entry fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if entry.Type()&fs.ModeSymlink != 0 {
			return nil
		}
		if entry.IsDir() {
			dirs = append(dirs, path)
			return nil
		}
		info, statErr := entry.Info()
		if os.IsNotExist(statErr) {
			return nil
		}
		if statErr != nil {
			return statErr
		}
		if !info.Mode().IsRegular() {
			return nil
		}
		relative, relErr := filepath.Rel(repoRoot, path)
		if relErr != nil {
			return relErr
		}
		files = append(files, generatedFile{path: path, size: info.Size(), tracked: tracked[filepath.ToSlash(relative)]})
		snapshot.files++
		snapshot.bytes += info.Size()
		if info.ModTime().After(snapshot.newest) {
			snapshot.newest = info.ModTime()
		}
		return nil
	})
	return dirs, files, snapshot, err
}

// 🧮️planGeneratedOutput decides what to remove below one ticket's generated folder; a non-empty skip leaves the ticket whole.
func planGeneratedOutput(options PurgeOptions, ticketDir, generated string) (removals []PurgeRemoval, skip string, snapshot generatedSnapshot, err error) {
	tracked, err := options.Tracked(options.Root, mustRelative(options.Root, generated))
	if err != nil {
		return nil, "tracked-unknown", snapshot, nil
	}
	dirs, files, snapshot, err := walkGenerated(generated, tracked, options.Root)
	if err != nil {
		return nil, "", snapshot, err
	}
	if snapshot.files > 0 && options.Now.Sub(snapshot.newest) < LiveWriterWindow {
		return nil, "live-writer", snapshot, nil
	}
	everything := options.Selection == PurgeGenerated
	bytesBelow := make(map[string]int64, len(dirs))
	trackedBelow := make(map[string]bool, len(dirs))
	for _, file := range files {
		for parent := filepath.Dir(file.path); ; parent = filepath.Dir(parent) {
			bytesBelow[parent] += file.size
			trackedBelow[parent] = trackedBelow[parent] || file.tracked
			if parent == generated {
				break
			}
		}
	}
	sort.Slice(dirs, func(left, right int) bool {
		leftDepth, rightDepth := strings.Count(dirs[left], string(os.PathSeparator)), strings.Count(dirs[right], string(os.PathSeparator))
		return leftDepth < rightDepth || (leftDepth == rightDepth && dirs[left] < dirs[right])
	})
	var removed []string
	covered := func(path string) bool {
		for _, directory := range removed {
			if path == directory || strings.HasPrefix(path, directory+string(os.PathSeparator)) {
				return true
			}
		}
		return false
	}
	for _, directory := range dirs {
		if (directory == generated && !everything) || covered(directory) || trackedBelow[directory] {
			continue
		}
		if everything || bytesBelow[directory] > OversizedFolderBytes {
			removed = append(removed, directory)
			removals = append(removals, PurgeRemoval{Path: directory, Kind: "directory", Bytes: bytesBelow[directory]})
		}
	}
	for _, file := range files {
		if covered(file.path) || file.tracked {
			continue
		}
		if everything || file.size > OversizedFileBytes {
			removals = append(removals, PurgeRemoval{Path: file.path, Kind: "file", Bytes: file.size})
		}
	}
	return removals, "", snapshot, nil
}

// 🛡️containedIn reports whether path is root itself or lies below it.
func containedIn(root, path string) bool {
	relative, err := filepath.Rel(root, path)
	return err == nil && relative != ".." && !strings.HasPrefix(relative, ".."+string(filepath.Separator))
}

// 🧭️resolvedPath is path with every symbolic link resolved, or path itself where it does not exist.
func resolvedPath(path string) string {
	if resolved, err := filepath.EvalSymlinks(path); err == nil {
		return resolved
	}
	return path
}

// 🧭️mustRelative is filepath.Rel for paths known to share a root.
func mustRelative(root, path string) string {
	relative, err := filepath.Rel(root, path)
	if err != nil {
		return path
	}
	return relative
}

// 🧹️PurgeTicketGeneratedOutput plans, and with options.Apply performs, the removal of generated output of the given ticket folders.
// Only the direct 🗑️generated folder of a CLOSED ticket is eligible; tracked files, links, files outside it and folders with a recent writer are never touched.
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json (PurgePolicy)
func PurgeTicketGeneratedOutput(options PurgeOptions, ticketDirs []string) (PurgePlan, error) {
	if options.Tracked == nil {
		options.Tracked = GitTrackedFiles
	}
	if options.Selection == "" {
		options.Selection = PurgeOversized
	}
	if options.Now.IsZero() {
		options.Now = time.Now()
	}
	options.Root, options.TicketsDir = resolvedPath(options.Root), resolvedPath(options.TicketsDir)
	plan := PurgePlan{Removals: []PurgeRemoval{}, Skips: []PurgeSkip{}}
	var failures []error
	for _, ticketDir := range ticketDirs {
		ticketDir = filepath.Join(resolvedPath(filepath.Dir(filepath.Clean(ticketDir))), filepath.Base(ticketDir))
		ticket := filepath.ToSlash(mustRelative(options.TicketsDir, ticketDir))
		generated := filepath.Join(ticketDir, TicketGeneratedDirectory)
		skip := func(reason string) { plan.Skips = append(plan.Skips, PurgeSkip{Ticket: ticket, Reason: reason}) }
		if folder, err := os.Lstat(ticketDir); err != nil || !folder.IsDir() {
			skip("not-directory")
			continue
		}
		if !ticketManifestIsClosed(ticketDir) {
			skip("not-closed")
			continue
		}
		info, err := os.Lstat(generated)
		if os.IsNotExist(err) {
			continue
		}
		if err != nil {
			failures = append(failures, err)
			continue
		}
		if info.Mode()&fs.ModeSymlink != 0 {
			skip("symlink-root")
			continue
		}
		if !info.IsDir() {
			skip("not-directory")
			continue
		}
		removals, reason, snapshot, err := planGeneratedOutput(options, ticketDir, generated)
		if err != nil {
			failures = append(failures, err)
			continue
		}
		if reason != "" {
			skip(reason)
			continue
		}
		if options.Apply {
			if _, _, current, err := walkGenerated(generated, nil, options.Root); err != nil || !current.equal(snapshot) {
				skip("live-writer")
				continue
			}
			for _, removal := range removals {
				if !containedIn(generated, removal.Path) {
					failures = append(failures, fmt.Errorf("refusing to remove %s outside %s", removal.Path, generated))
					continue
				}
				if err := os.RemoveAll(removal.Path); err != nil {
					failures = append(failures, err)
				}
			}
		}
		for _, removal := range removals {
			removal.Path = filepath.ToSlash(mustRelative(options.TicketsDir, removal.Path))
			plan.Removals = append(plan.Removals, removal)
		}
	}
	sort.Slice(plan.Removals, func(left, right int) bool { return plan.Removals[left].Path < plan.Removals[right].Path })
	sort.Slice(plan.Skips, func(left, right int) bool { return plan.Skips[left].Ticket < plan.Skips[right].Ticket })
	return plan, errors.Join(failures...)
}

// 📣️Report renders the plan as English and German lines for a command line, ending in the totals.
func (plan PurgePlan) Report(applied bool) []string {
	var lines []string
	var freed int64
	for _, removal := range plan.Removals {
		freed += removal.Bytes
		verb := "would remove / würde entfernen"
		if applied {
			verb = "removed / entfernt"
		}
		lines = append(lines, fmt.Sprintf("%s %s (%d bytes)", verb, removal.Path, removal.Bytes))
	}
	for _, skip := range plan.Skips {
		lines = append(lines, fmt.Sprintf("skipped / übersprungen %s: %s", skip.Ticket, skip.Reason))
	}
	if applied {
		return append(lines, fmt.Sprintf("Removed %d paths (%d bytes) / %d Pfade entfernt (%d Bytes)", len(plan.Removals), freed, len(plan.Removals), freed))
	}
	return append(lines, fmt.Sprintf("Dry run: %d paths (%d bytes) would be removed; repeat with --apply / Probelauf: %d Pfade (%d Bytes) würden entfernt; mit --apply wiederholen", len(plan.Removals), freed, len(plan.Removals), freed))
}

// 🧹️PurgeClosedTicketGeneratedOutput purges every closed ticket below options.TicketsDir.
func PurgeClosedTicketGeneratedOutput(options PurgeOptions) (PurgePlan, error) {
	roots, err := CollectTicketFolderRoots(options.TicketsDir)
	if err != nil {
		return PurgePlan{}, err
	}
	return PurgeTicketGeneratedOutput(options, roots)
}

// #endregion 🧹️Purge Generated Output

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
		Checkpoint: workspace.GetGitCheckpoint(workspace.GetRootDir()),
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
	if _, err := PurgeTicketGeneratedOutput(PurgeOptions{Root: workspace.GetRootDir(), TicketsDir: GetTicketsDir(), Selection: PurgeOversized, Apply: true}, []string{ticket.FolderPath}); err != nil {
		workspace.WriteWarningf("Failed to purge oversized ticket generated output: %v", err)
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
