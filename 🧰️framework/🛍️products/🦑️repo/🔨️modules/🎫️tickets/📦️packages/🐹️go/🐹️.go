// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️tickets is the tickets domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package tickets

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	goals "github.com/usalu/semio/repo/goals"
	identity "github.com/usalu/semio/repo/identity"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 💡️GraphQL Types

// 🧲️ticketIdToParts extracts year, month, day, slug from a ticket emoji ID.
// 🧩️It walks all tickets to find the one whose ID matches.
func TicketIdToParts(emojiId string) (year, month, day int, slug string) {
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	normalizedId := norm(emojiId)
	// Strip goal prefix to get ticket-only part
	ticketEmoji := norm(model.EmojiText(model.EmojiTicket))
	lastTicketIdx := strings.LastIndex(normalizedId, ticketEmoji)
	if lastTicketIdx < 0 {
		return 0, 0, 0, ""
	}
	ticketSlugFlat := normalizedId[lastTicketIdx+len(ticketEmoji):]
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return 0, 0, 0, ""
	}
	for _, t := range tickets {
		if workspace.Flat(t.Slug) == ticketSlugFlat {
			return t.Year, t.Month, t.Day, t.Slug
		}
	}
	return 0, 0, 0, ""
}

// 🗃️newTicketDiffSet holds the data fields for a newTicketDiffSet record.
func newTicketDiffSet() model.TicketDiffSet {
	return model.TicketDiffSet{
		Deleted:  []model.TicketFile{},
		Renamed:  []model.TicketFileRenamed{},
		Modified: []model.TicketFile{},
		Added:    []model.TicketFile{},
	}
}

// 🔖️newTicketDiffs holds the data fields for a newTicketDiffs record.
func newTicketDiffs() *model.TicketDiffs {
	return &model.TicketDiffs{
		Bundles:     newTicketDiffSet(),
		Folders:     newTicketDiffSet(),
		Files:       newTicketDiffSet(),
		Sections:    newTicketDiffSet(),
		Definitions: newTicketDiffSet(),
	}
}

// ➕️addTicketDiffEntry holds the data fields for a addTicketDiffEntry record.
func addTicketDiffEntry(set *model.TicketDiffSet, change model.SemanticChange) {
	lines := &model.LineMetrics{Added: change.Lines.Added, Removed: change.Lines.Removed}
	switch change.Status {
	case model.SemanticChangeAdded:
		set.Added = append(set.Added, model.TicketFile{Path: change.Path, Lines: lines})
	case model.SemanticChangeDeleted:
		set.Deleted = append(set.Deleted, model.TicketFile{Path: change.Path, Lines: lines})
	case model.SemanticChangeRenamed:
		set.Renamed = append(set.Renamed, model.TicketFileRenamed{From: change.FromPath, To: change.ToPath, Lines: lines})
	case model.SemanticChangeModified:
		set.Modified = append(set.Modified, model.TicketFile{Path: change.Path, Lines: lines})
	}
}

// 🔖️computeLineMetricsForDiff holds the data fields for a computeLineMetricsForDiff record.
func computeLineMetricsForDiff(diff *model.DiffLines, baseCheckpoint, filePath string) model.LineMetrics {
	if diff == nil {
		return model.LineMetrics{}
	}
	if len(diff.Added) > 0 && len(diff.Removed) == 0 {
		return model.LineMetrics{Added: workspace.CountLinesInFile(filepath.Join(workspace.GetRootDir(), filePath)), Removed: 0}
	}
	if len(diff.Removed) > 0 && len(diff.Added) == 0 {
		return model.LineMetrics{Added: 0, Removed: workspace.CountLinesAtCheckpoint(baseCheckpoint, filePath)}
	}
	return model.LineMetrics{Added: len(diff.Added), Removed: len(diff.Removed)}
}

// 🏗️buildFolderLineTotals holds the data fields for a buildFolderLineTotals record.
func buildFolderLineTotals(files []string, baseCheckpoint string, bundles []model.Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &codebase.CodebaseContext{Bundles: bundles}
	for _, file := range files {
		folderPath := workspace.NormalizePath(filepath.Dir(file))
		if folderPath == "." {
			continue
		}
		id := ctx.GetFolderID(folderPath)
		currentTotals[id] += workspace.CountLinesInFile(filepath.Join(workspace.GetRootDir(), file))
		baseTotals[id] += workspace.CountLinesAtCheckpoint(baseCheckpoint, file)
	}
	return currentTotals, baseTotals
}

// 🔖️buildBundleLineTotals holds the data fields for a buildBundleLineTotals record.
func buildBundleLineTotals(files []string, baseCheckpoint string, bundles []model.Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &codebase.CodebaseContext{Bundles: bundles}
	for _, file := range files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		currentTotals[bundleName] += workspace.CountLinesInFile(filepath.Join(workspace.GetRootDir(), file))
		baseTotals[bundleName] += workspace.CountLinesAtCheckpoint(baseCheckpoint, file)
	}
	return currentTotals, baseTotals
}

// 🔖️reconcileRenamePairs holds the data fields for a reconcileRenamePairs record.
func reconcileRenamePairs(diffSet *model.TicketDiffSet, matchKey func(path string) string) {
	if diffSet == nil {
		return
	}
	usedAdded := make(map[int]struct{})
	var renamed []model.TicketFileRenamed
	var remainingDeleted []model.TicketFile
	for _, del := range diffSet.Deleted {
		matchIndex := -1
		key := matchKey(del.Path)
		for i, add := range diffSet.Added {
			if _, ok := usedAdded[i]; ok {
				continue
			}
			if key != "" && matchKey(add.Path) != key {
				continue
			}
			removedLines := 0
			if del.Lines != nil {
				removedLines = del.Lines.Removed
			}
			addedLines := 0
			if add.Lines != nil {
				addedLines = add.Lines.Added
			}
			if removedLines > 0 && addedLines > 0 && removedLines != addedLines {
				continue
			}
			matchIndex = i
			break
		}
		if matchIndex == -1 {
			remainingDeleted = append(remainingDeleted, del)
			continue
		}
		add := diffSet.Added[matchIndex]
		usedAdded[matchIndex] = struct{}{}
		lines := &model.LineMetrics{}
		if add.Lines != nil {
			lines.Added = add.Lines.Added
		}
		if del.Lines != nil {
			lines.Removed = del.Lines.Removed
		}
		renamed = append(renamed, model.TicketFileRenamed{From: del.Path, To: add.Path, Lines: lines})
	}
	var remainingAdded []model.TicketFile
	for i, add := range diffSet.Added {
		if _, ok := usedAdded[i]; ok {
			continue
		}
		remainingAdded = append(remainingAdded, add)
	}
	diffSet.Added = remainingAdded
	diffSet.Deleted = remainingDeleted
	diffSet.Renamed = append(diffSet.Renamed, renamed...)
}

// 🔖️buildSectionDiffs holds the data fields for a buildSectionDiffs record.
func buildSectionDiffs(baseCodebase, currentCodebase *model.Codebase, baseCheckpoint string, diffLines map[string]*model.DiffLines, bundles []model.Bundle) model.TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &codebase.CodebaseContext{Bundles: bundles}
	currentSectionMap := make(map[string]model.CodebaseSection)
	baseSectionMap := make(map[string]model.CodebaseSection)

	for _, section := range currentCodebase.Sections {
		currentSectionMap[section.Path] = section
	}
	for _, section := range baseCodebase.Sections {
		baseSectionMap[section.Path] = section
	}

	for path := range currentSectionMap {
		if _, ok := baseSectionMap[path]; !ok {
			lines := 0
			if currentSectionMap[path].Metrics != nil {
				lines = currentSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "section", Status: model.SemanticChangeAdded, Path: path, Lines: model.LineMetrics{Added: lines}})
		}
	}
	for path := range baseSectionMap {
		if _, ok := currentSectionMap[path]; !ok {
			lines := 0
			if baseSectionMap[path].Metrics != nil {
				lines = baseSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "section", Status: model.SemanticChangeDeleted, Path: path, Lines: model.LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := workspace.ReadTextFile(filepath.Join(workspace.GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := workspace.ReadTextFileAtCheckpoint(baseCheckpoint, filePath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(filePath)
		if lang == nil || !lang.SupportsSections() {
			continue
		}
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		addedMap := codebase.ComputeSectionLineMap(currentSections, diff.Added, "")
		removedMap := codebase.ComputeSectionLineMap(baseSections, diff.Removed, "")
		for sectionPath, addedLines := range addedMap {
			removedLines := removedMap[sectionPath]
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "section", Status: model.SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: model.LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for sectionPath, removedLines := range removedMap {
			if _, ok := addedMap[sectionPath]; ok {
				continue
			}
			if len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "section", Status: model.SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: model.LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return model.ExtractFilePrefix(path)
	})

	return result
}

// 🔖️buildDefinitionDiffs holds the data fields for a buildDefinitionDiffs record.
func buildDefinitionDiffs(baseCodebase, currentCodebase *model.Codebase, baseCheckpoint string, diffLines map[string]*model.DiffLines, bundles []model.Bundle) model.TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &codebase.CodebaseContext{Bundles: bundles}
	currentDefMap := make(map[string]model.CodebaseDefinition)
	baseDefMap := make(map[string]model.CodebaseDefinition)

	for _, def := range currentCodebase.Definitions {
		currentDefMap[def.Path] = def
	}
	for _, def := range baseCodebase.Definitions {
		baseDefMap[def.Path] = def
	}

	for path := range currentDefMap {
		if _, ok := baseDefMap[path]; !ok {
			lines := 0
			if currentDefMap[path].Metrics != nil {
				lines = currentDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "definition", Status: model.SemanticChangeAdded, Path: path, Lines: model.LineMetrics{Added: lines}})
		}
	}
	for path := range baseDefMap {
		if _, ok := currentDefMap[path]; !ok {
			lines := 0
			if baseDefMap[path].Metrics != nil {
				lines = baseDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "definition", Status: model.SemanticChangeDeleted, Path: path, Lines: model.LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := workspace.ReadTextFile(filepath.Join(workspace.GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := workspace.ReadTextFileAtCheckpoint(baseCheckpoint, filePath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(filePath)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		currentLines := strings.Split(content, "\n")
		baseLines := strings.Split(baseContent, "\n")
		currentDefs := lang.ParseDefinitions(content, currentLines)
		baseDefs := lang.ParseDefinitions(baseContent, baseLines)
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		for _, def := range currentDefs {
			addedLines := codebase.ComputeLinesInRange(diff.Added, def.Start, def.End)
			removedLines := codebase.ComputeLinesInRange(diff.Removed, def.Start, def.End)
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			sectionPath := codebase.FindSectionForDefinition(currentSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "definition", Status: model.SemanticChangeModified, Path: defPath, Lines: model.LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for _, def := range baseDefs {
			removedLines := codebase.ComputeLinesInRange(diff.Removed, def.Start, def.End)
			if len(removedLines) == 0 {
				continue
			}
			sectionPath := codebase.FindSectionForDefinition(baseSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, model.SemanticChange{Kind: "definition", Status: model.SemanticChangeModified, Path: defPath, Lines: model.LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return model.ExtractFilePrefix(path)
	})

	return result
}

// 🔖️BuildSemanticDiffs MUST assemble the semantic diffs from the available context data.
// ⚖️BuildSemanticDiffs constructs and returns the semantic diffs structure.
func BuildSemanticDiffs(baseCodebase, currentCodebase *model.Codebase, baseCheckpoint string, diffLines map[string]*model.DiffLines, diffStatuses []providers.GitDiffStatus, bundles []model.Bundle) *model.TicketDiffs {
	result := newTicketDiffs()
	ctx := &codebase.CodebaseContext{Bundles: bundles}

	currentFilesSet := make(map[string]struct{})
	baseFilesSet := make(map[string]struct{})
	for filePath := range diffLines {
		currentFilesSet[filePath] = struct{}{}
		baseFilesSet[filePath] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.From != "" {
			baseFilesSet[status.From] = struct{}{}
		}
		if status.To != "" {
			currentFilesSet[status.To] = struct{}{}
		}
	}
	var currentFiles []string
	for f := range currentFilesSet {
		currentFiles = append(currentFiles, f)
	}
	var baseFiles []string
	for f := range baseFilesSet {
		baseFiles = append(baseFiles, f)
	}

	currentFolderLines, _ := buildFolderLineTotals(currentFiles, baseCheckpoint, bundles)
	_, baseFolderLines := buildFolderLineTotals(baseFiles, baseCheckpoint, bundles)
	currentBundleLines, _ := buildBundleLineTotals(currentFiles, baseCheckpoint, bundles)
	_, baseBundleLines := buildBundleLineTotals(baseFiles, baseCheckpoint, bundles)

	currentBundleMap := make(map[string]model.CodebaseBundle)
	baseBundleMap := make(map[string]model.CodebaseBundle)
	if currentCodebase != nil {
		for _, bundle := range currentCodebase.Bundles {
			currentBundleMap[bundle.ID] = bundle
		}
	}
	if baseCodebase != nil {
		for _, bundle := range baseCodebase.Bundles {
			baseBundleMap[bundle.ID] = bundle
		}
	}
	for id, bundle := range currentBundleMap {
		if _, ok := baseBundleMap[id]; !ok {
			lines := model.LineMetrics{Added: currentBundleLines[id], Removed: 0}
			addTicketDiffEntry(&result.Bundles, model.SemanticChange{Kind: "bundle", Status: model.SemanticChangeAdded, Path: model.NormalizeBundleLabel(id), Lines: lines})
		} else if bundle.Metrics != nil {
			baseBundle := baseBundleMap[id]
			if baseBundle.Metrics != nil {
				added := currentBundleLines[id] - baseBundleLines[id]
				removed := 0
				if added < 0 {
					removed = -added
					added = 0
				}
				if added > 0 || removed > 0 {
					addTicketDiffEntry(&result.Bundles, model.SemanticChange{Kind: "bundle", Status: model.SemanticChangeModified, Path: model.NormalizeBundleLabel(id), Lines: model.LineMetrics{Added: added, Removed: removed}})
				}
			}
		}
	}
	for id := range baseBundleMap {
		if _, ok := currentBundleMap[id]; !ok {
			lines := model.LineMetrics{Added: 0, Removed: baseBundleLines[id]}
			addTicketDiffEntry(&result.Bundles, model.SemanticChange{Kind: "bundle", Status: model.SemanticChangeDeleted, Path: model.NormalizeBundleLabel(id), Lines: lines})
		}
	}

	currentFolderMap := make(map[string]model.CodebaseFolder)
	baseFolderMap := make(map[string]model.CodebaseFolder)
	if currentCodebase != nil {
		for _, folder := range currentCodebase.Folders {
			currentFolderMap[folder.Path] = folder
		}
	}
	if baseCodebase != nil {
		for _, folder := range baseCodebase.Folders {
			baseFolderMap[folder.Path] = folder
		}
	}
	for path := range currentFolderMap {
		if _, ok := baseFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, model.SemanticChange{Kind: "folder", Status: model.SemanticChangeAdded, Path: path, Lines: model.LineMetrics{Added: currentFolderLines[path]}})
		}
	}
	for path := range baseFolderMap {
		if _, ok := currentFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, model.SemanticChange{Kind: "folder", Status: model.SemanticChangeDeleted, Path: path, Lines: model.LineMetrics{Removed: baseFolderLines[path]}})
		}
	}

	for _, status := range diffStatuses {
		if status.Status == "renamed" {
			fromFolder := workspace.NormalizePath(filepath.Dir(status.From))
			toFolder := workspace.NormalizePath(filepath.Dir(status.To))
			fromFolderID := ctx.GetFolderID(fromFolder)
			toFolderID := ctx.GetFolderID(toFolder)
			if fromFolderID != toFolderID && fromFolder != "." && toFolder != "." {
				addTicketDiffEntry(&result.Folders, model.SemanticChange{Kind: "folder", Status: model.SemanticChangeRenamed, FromPath: fromFolderID, ToPath: toFolderID, Lines: model.LineMetrics{Added: currentFolderLines[toFolderID], Removed: baseFolderLines[fromFolderID]}})
			}
			fromFileID := ctx.GetFileID(status.From)
			toFileID := ctx.GetFileID(status.To)
			addTicketDiffEntry(&result.Files, model.SemanticChange{Kind: "file", Status: model.SemanticChangeRenamed, FromPath: fromFileID, ToPath: toFileID, Lines: model.LineMetrics{Added: workspace.CountLinesInFile(filepath.Join(workspace.GetRootDir(), status.To)), Removed: workspace.CountLinesAtCheckpoint(baseCheckpoint, status.From)}})
		}
	}

	for filePath, diff := range diffLines {
		fileID := ctx.GetFileID(filePath)
		metrics := computeLineMetricsForDiff(diff, baseCheckpoint, filePath)
		status := model.SemanticChangeModified
		if len(diff.Added) > 0 && len(diff.Removed) == 0 {
			status = model.SemanticChangeAdded
		} else if len(diff.Removed) > 0 && len(diff.Added) == 0 {
			status = model.SemanticChangeDeleted
		}
		addTicketDiffEntry(&result.Files, model.SemanticChange{Kind: "file", Status: status, Path: fileID, Lines: metrics})
	}

	reconcileRenamePairs(&result.Bundles, func(path string) string {
		return path
	})

	result.Sections = buildSectionDiffs(baseCodebase, currentCodebase, baseCheckpoint, diffLines, bundles)
	result.Definitions = buildDefinitionDiffs(baseCodebase, currentCodebase, baseCheckpoint, diffLines, bundles)

	return result
}

// #endregion 💡️GraphQL Types

// #region 🎽️Languages

// ⛳️ListInteractions aggregates interactions from all tickets and goals.
func ListInteractions() ([]model.InteractionResource, error) {
	var result []model.InteractionResource
	tickets, err := ListTickets(nil, nil, nil)
	if err == nil {
		for _, t := range tickets {
			tid := t.GetID()
			for _, ix := range t.Interactions {
				result = append(result, model.InteractionResource{
					Interaction: ix,
					SourceKind:  "ticket",
					SourceID:    tid,
					GoalID:      t.Goal,
					TicketID:    tid,
				})
			}
		}
	}
	return result, nil
}

// ▪️StreamInteractions streams interactions from all tickets and goals.
func StreamInteractions(ctx context.Context, out chan<- model.InteractionResource) {
	defer close(out)
	tickets, _ := ListTickets(nil, nil, nil)
	for _, t := range tickets {
		tid := t.GetID()
		for _, ix := range t.Interactions {
			select {
			case <-ctx.Done():
				return
			case out <- model.InteractionResource{
				Interaction: ix,
				SourceKind:  "ticket",
				SourceID:    tid,
				GoalID:      t.Goal,
				TicketID:    tid,
			}:
			}
		}
	}
}

// #endregion 🎽️Languages

// #region 📋️Tickets

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
	ctx := model.LookupRepoContext(workspace.RootDir)
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
		files, err := model.ScopeToFiles(workspace.Scope{Kind: workspace.ScopeRepo}, codebase.GetTechnologies())
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
	files = model.FilterConsideredFiles(files)
	files = model.FilterGitIgnored(files)
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

// #endregion 📋️Tickets

// #region 🎗️Ticket File Resolution

// #region 🎗️Ticket File Resolution
// Ticket file input normalization for close operations.
// Requirements: Accept repo-relative paths, absolute paths, repo file URIs, and file artifact IDs.
// Docs: Used by ticket close to map file identifiers to repo paths.
// 📝️normalizeTicketFileInput holds the data fields for a normalizeTicketFileInput record.
func normalizeTicketFileInput(filePath string) string {
	normalized := strings.TrimSpace(filePath)
	if normalized == "" {
		return ""
	}
	if strings.HasPrefix(normalized, "repo://") {
		id := model.UriToId(normalized)
		if id != "" {
			path := model.IdToPath(id)
			if path != "" {
				return workspace.NormalizeRepoPath(path)
			}
		}
	}
	if strings.HasPrefix(normalized, "file://") {
		path := strings.TrimPrefix(normalized, "file://")
		return workspace.NormalizeRepoPath(path)
	}
	if filepath.IsAbs(normalized) || strings.ContainsAny(normalized, `/\`) || filepath.Ext(normalized) != "" {
		return workspace.NormalizeRepoPath(normalized)
	}
	uri := model.IdToUri(normalized)
	if uri != "" && strings.HasPrefix(uri, "repo://") {
		id := model.UriToId(uri)
		if id != "" {
			path := model.IdToPath(id)
			if path != "" {
				return workspace.NormalizeRepoPath(path)
			}
		}
	}
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err == nil {
		for _, file := range ctx.Files {
			if codebase.FileHeaderId(file) == normalized {
				return workspace.NormalizeRepoPath(file)
			}
		}
	}
	return workspace.NormalizeRepoPath(normalized)
}

// 💿️normalizeTicketFileInputs holds the data fields for a normalizeTicketFileInputs record.
func normalizeTicketFileInputs(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	seen := make(map[string]struct{}, len(files))
	for _, filePath := range files {
		normalized := normalizeTicketFileInput(filePath)
		if normalized == "" {
			continue
		}
		if _, ok := seen[normalized]; ok {
			continue
		}
		seen[normalized] = struct{}{}
		filtered = append(filtered, normalized)
	}
	return filtered
}

// #endregion 🎗️Ticket File Resolution

// #region 📋️Tickets

// 🧲️extractMilestoneNumber holds the data fields for a extractMilestoneNumber record.
func extractMilestoneNumber(milestoneURL string) int {
	parts := strings.Split(milestoneURL, "/")
	if len(parts) == 0 {
		return 0
	}
	num, _ := strconv.Atoi(parts[len(parts)-1])
	return num
}

// 🔲️SaveTicket MUST persist the ticket atomically to the data store.
// 💾️SaveTicket persists ticket to the data store.
func SaveTicket(ticket *model.Ticket) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	jsonBytes, err := json.MarshalIndent(ticket, "", "  ")
	if err != nil {
		return err
	}
	if err := workspace.WriteTextFile(ticket.JsonPath, string(jsonBytes)); err != nil {
		return err
	}
	// Sync ticket state to server (fire-and-forget, don't block on failure)
	action := "open"
	if ticket.Status == model.TicketStatusClosed {
		action = "close"
	}
	go syncTicketToServer(ticket, action)
	return nil
}

// 📖️ReadTicket MUST return the ticket content or an error if unavailable.
// 🔸️ReadTicket reads and returns ticket from the source.
func ReadTicket(year, month, day int, slug string) (*model.Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	if !workspace.FileExists(jsonPath) {
		return nil, fmt.Errorf("ticket not found: %s", jsonPath)
	}
	raw, err := workspace.ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var ticket model.Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		return nil, err
	}

	ticket.Year = year
	ticket.Month = month
	ticket.Day = day
	ticket.Slug = slug
	ticket.FolderPath = folderPath
	ticket.JsonPath = jsonPath
	ticket.ImportantPath = GetImportantFilePath(year, month, day, slug)

	return &ticket, nil
}

// ▪️ListTickets MUST return all available tickets entries.
// 📋️ListTickets returns a list of tickets entries.
func ListTickets(year, month, day *int) ([]model.Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !workspace.FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []model.Ticket
	var years []string
	if year != nil {
		years = []string{model.FormatYearDir(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	for _, y := range years {
		yearPath := filepath.Join(ticketsDir, y)
		if !workspace.FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{model.FormatMonthDir(*month)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !workspace.FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{model.FormatDayDir(*day)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !workspace.FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := workspace.ParseDatedDir(y, model.EmojiYear)
				monthInt, monthErr := workspace.ParseDatedDir(m, model.EmojiMonth)
				dayInt, dayErr := workspace.ParseDatedDir(d, model.EmojiDay)
				if yearErr != nil || monthErr != nil || dayErr != nil {
					continue
				}
				filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					if !dEntry.IsDir() {
						return nil
					}
					name := dEntry.Name()
					if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
						return filepath.SkipDir
					}
					if path == dayPath {
						return nil
					}
					ticketFilePath := filepath.Join(path, "🎫️ticket.json")
					if !workspace.FileExists(ticketFilePath) {
						return nil
					}
					rel, relErr := filepath.Rel(dayPath, path)
					if relErr != nil {
						return filepath.SkipDir
					}
					slug := filepath.ToSlash(rel)
					ticket, readErr := ReadTicket(yearInt, monthInt, dayInt, slug)
					if readErr == nil {
						tickets = append(tickets, *ticket)
					}
					return filepath.SkipDir
				})
			}
		}
	}
	return tickets, nil
}

// 📍️StreamTickets MUST invoke the callback for each matching tickets entry.
// ⚡️StreamTickets streams tickets entries through the callback.
func StreamTickets(ctx context.Context, year, month, day *int, out chan<- model.Ticket, opts ...model.StreamOptions) error {
	defer close(out)

	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	ticketsDir := GetTicketsDir()
	if !workspace.FileExists(ticketsDir) {
		return nil
	}
	var years []string
	if year != nil {
		years = []string{model.FormatYearDir(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}

	for _, y := range years {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		yearPath := filepath.Join(ticketsDir, y)
		if !workspace.FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{model.FormatMonthDir(*month)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !workspace.FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{model.FormatDayDir(*day)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !workspace.FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := workspace.ParseDatedDir(y, model.EmojiYear)
				monthInt, monthErr := workspace.ParseDatedDir(m, model.EmojiMonth)
				dayInt, dayErr := workspace.ParseDatedDir(d, model.EmojiDay)
				if yearErr != nil || monthErr != nil || dayErr != nil {
					continue
				}
				walkErr := filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					select {
					case <-ctx.Done():
						return ctx.Err()
					default:
					}
					if !dEntry.IsDir() {
						return nil
					}
					name := dEntry.Name()
					if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
						return filepath.SkipDir
					}
					if path == dayPath {
						return nil
					}
					ticketFilePath := filepath.Join(path, "🎫️ticket.json")
					if !workspace.FileExists(ticketFilePath) {
						return nil
					}
					rel, relErr := filepath.Rel(dayPath, path)
					if relErr != nil {
						return filepath.SkipDir
					}
					slug := filepath.ToSlash(rel)
					ticket, readErr := ReadTicket(yearInt, monthInt, dayInt, slug)
					if readErr != nil {
						return filepath.SkipDir
					}
					if !model.MatchesFilter(ticket.GetID(), options) && !model.MatchesFilter(ticket.Slug, options) && !model.MatchesFilter(ticket.Title, options) {
						return filepath.SkipDir
					}
					if !model.MatchesQuery(ticket.GetID()+" "+ticket.Slug+" "+ticket.Title+" "+ticket.Description+" "+string(ticket.Status), options) {
						return filepath.SkipDir
					}
					if !ticketMatchesKinds(ticket, options) {
						return filepath.SkipDir
					}
					out <- *ticket
					return filepath.SkipDir
				})
				if walkErr != nil {
					return walkErr
				}
			}
		}
	}
	return nil
}

// ▫️ticketMatchesKinds holds the data fields for a ticketMatchesKinds record.
func ticketMatchesKinds(t *model.Ticket, opts model.StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}
	allFiles := map[string]bool{}
	for _, f := range t.GetInteractionFiles() {
		allFiles[f.Path] = true
	}

	if len(allFiles) == 0 {

		if len(opts.IncludeKinds) > 0 {
			return false
		}

		return true
	}

	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	for f := range allFiles {
		kind := model.DeriveFileKind(filepath.Base(f))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}
	}

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

// 🔵️StreamBundles MUST invoke the callback for each matching bundles entry.
// 🎯️StreamBundles streams bundles entries through the callback.
func StreamBundles(ctx context.Context, out chan<- model.Bundle, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	bundles := codebase.LoadBundles()
	for _, b := range bundles {
		if !model.MatchesFilter(b.Name, options) {
			continue
		}
		if !model.MatchesQuery(b.Name+" "+string(b.Kind), options) {
			continue
		}

		if !shouldIncludeBundleKind(b.Kind, options) {
			continue
		}

		if !bundleMatchesKinds(b, options) {
			continue
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- b
		}
	}
	return nil
}

// 🔴️bundleMatchesKinds holds the data fields for a bundleMatchesKinds record.
func bundleMatchesKinds(b model.Bundle, opts model.StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}

	bundleRoot := filepath.Join(workspace.RootDir, b.Root)
	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() {
			return nil
		}
		if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
			return nil
		}

		kind := model.DeriveFileKind(filepath.Base(path))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}

		if hasIncluded && (len(opts.ExcludeKinds) == 0 || hasExcluded) {
			return filepath.SkipAll
		}
		return nil
	})

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

// 🟡️StreamTechnologies MUST invoke the callback for each matching technologies entry.
// 🔻️StreamTechnologies streams technologies entries through the callback.
func StreamTechnologies(ctx context.Context, out chan<- model.Technology, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	technologies := codebase.LoadTechnologies()
	for _, p := range technologies {
		if !model.MatchesFilter(p.Name, options) {
			continue
		}
		if !model.MatchesQuery(p.Name+" "+string(p.Kind), options) {
			continue
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- p
		}
	}
	return nil
}

// 🔖️queryableItem holds the data fields for a queryableItem record.
type queryableItem struct {
	text string
	idx  int
}

// ⚪️searchFilterItems returns the matching item positions.
func searchFilterItems(items []queryableItem, query string) map[int]bool {
	result := make(map[int]bool)
	if query == "" {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	mapping := search.NewIndexMapping()
	index, err := search.NewMemOnly(mapping)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	defer index.Close()
	for _, item := range items {
		doc := map[string]interface{}{"text": item.text}
		index.Index(fmt.Sprintf("%d", item.idx), doc)
	}
	mq := search.NewMatchQuery(query)
	mq.SetFuzziness(2)
	searchRequest := search.NewSearchRequest(mq)
	searchRequest.Size = len(items)
	results, err := index.Search(searchRequest)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	for _, hit := range results.Hits {
		idx, err := strconv.Atoi(hit.ID)
		if err == nil {
			result[idx] = true
		}
	}
	return result
}

// ⚫️shouldIncludeKind holds the data fields for a shouldIncludeKind record.
func shouldIncludeKind(kind string, opts model.StreamOptions) bool {
	if len(opts.IncludeKinds) > 0 {
		found := false
		for _, k := range opts.IncludeKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩵️shouldIncludeBundleKind holds the data fields for a shouldIncludeBundleKind record.
func shouldIncludeBundleKind(kind model.BundleKind, opts model.StreamOptions) bool {
	if len(opts.IncludeBundleKinds) > 0 {
		found := false
		for _, k := range opts.IncludeBundleKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeBundleKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 📁️shouldIncludeFolderKind holds the data fields for a shouldIncludeFolderKind record.
func shouldIncludeFolderKind(kind model.FolderKind, opts model.StreamOptions) bool {
	if len(opts.IncludeFolderKinds) > 0 {
		found := false
		for _, k := range opts.IncludeFolderKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeFolderKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩶️shouldIncludeDefinitionKind holds the data fields for a shouldIncludeDefinitionKind record.
func shouldIncludeDefinitionKind(kind model.DefinitionKind, opts model.StreamOptions) bool {
	if len(opts.IncludeDefinitionKinds) > 0 {
		found := false
		for _, k := range opts.IncludeDefinitionKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeDefinitionKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩷️StreamFolders MUST invoke the callback for each matching folders entry.
// 📄️StreamFolders streams folders entries through the callback.
func StreamFolders(ctx context.Context, scope string, out chan<- model.Folder, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	root := workspace.RootDir
	if bundleName, found := strings.CutPrefix(scope, "compose/"); found {
		bundles := codebase.GetTechnologies()
		for _, b := range bundles {
			if b.Name == bundleName || model.NormalizeBundleLabel(b.Name) == bundleName {
				root = filepath.Join(workspace.RootDir, b.Root)
				break
			}
		}
	} else if scope != "" && scope != "compose" {
		if filepath.IsAbs(scope) {
			root = scope
		} else {
			root = filepath.Join(workspace.RootDir, scope)
		}
	}

	var folders []string
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() {
			return nil
		}
		rel, _ := filepath.Rel(root, path)
		if rel == "." {
			return nil
		}
		if workspace.IsRepoExcludedPath(path) {
			return filepath.SkipDir
		}
		if workspace.IsGitIgnored(path) && !options.ShowIgnored {
			return filepath.SkipDir
		}
		folders = append(folders, path)
		return nil
	})
	if err != nil {
		return err
	}

	for _, folderPath := range folders {
		ignored := workspace.IsGitIgnored(folderPath)
		relPath, _ := filepath.Rel(workspace.RootDir, folderPath)
		generated := model.IsGenerated(folderPath) || model.IsGeneratedFolder(relPath)
		folderKind := model.DeriveFolderKind(relPath)

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeFolderKind(folderKind, options) {
			continue
		}

		if !model.MatchesFilter(filepath.Base(folderPath), options) {
			continue
		}
		if !model.MatchesQuery(folderPath, options) {
			continue
		}

		var bundleID *string
		if b := model.GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}

		parentPath := filepath.Dir(relPath)
		var parentID *string
		if parentPath != "." {
			id := model.BuildFolderID(parentPath, bundleID)
			parentID = &id
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- model.Folder{
				ID:        model.BuildFolderID(relPath, bundleID),
				Path:      relPath,
				URI:       model.BuildFolderUriFromPath(relPath),
				Name:      filepath.Base(relPath),
				ParentID:  parentID,
				BundleID:  bundleID,
				Kind:      folderKind,
				Ignored:   ignored,
				Generated: generated,
			}
		}
	}
	return nil
}

// 💜️StreamFiles MUST invoke the callback for each matching files entry.
// 🔲️StreamFiles streams files entries through the callback.
func StreamFiles(ctx context.Context, scope string, out chan<- model.File, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	root := workspace.RootDir
	if scope != "" && scope != "compose" {
		bundles := codebase.GetTechnologies()
		matched := false
		for _, b := range bundles {
			if b.Name == scope || b.Root == scope || model.NormalizeBundleLabel(b.Name) == scope {
				root = filepath.Join(workspace.RootDir, b.Root)
				matched = true
				break
			}
		}
		if !matched {
			parts := strings.SplitN(scope, "/", 3)
			if len(parts) == 3 {
				bundleScope := parts[0] + "/" + parts[1]
				subPath := parts[2]
				for _, b := range bundles {
					if b.Name == bundleScope || b.Root == bundleScope || model.NormalizeBundleLabel(b.Name) == bundleScope {
						root = filepath.Join(workspace.RootDir, b.Root, subPath)
						matched = true
						break
					}
				}
			}
		}
		if !matched {
			if filepath.IsAbs(scope) {
				root = scope
			} else {
				root = filepath.Join(workspace.RootDir, scope)
			}
		}
	}

	if info, err := os.Stat(root); err == nil && !info.IsDir() {
		relPath, _ := filepath.Rel(workspace.RootDir, root)
		var bundleID *string
		if b := model.GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}
		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := model.BuildFolderID(folderPath, bundleID)
			folderID = &id
		}
		out <- model.File{
			ID:        model.BuildFileID(relPath, bundleID),
			Path:      relPath,
			URI:       model.BuildFileUriFromPath(relPath),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
			Kind:      model.DeriveFileKind(filepath.Base(relPath)),
			Ignored:   workspace.IsGitIgnored(root),
			Generated: model.IsGenerated(root),
		}
		return nil
	}

	var files []string
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			rel, _ := filepath.Rel(root, path)
			if rel != "." {
				if workspace.IsRepoExcludedPath(path) {
					return filepath.SkipDir
				}
				if workspace.IsGitIgnored(path) && !options.ShowIgnored {
					return filepath.SkipDir
				}
			}
			return nil
		}
		if workspace.IsRepoExcludedPath(path) {
			return nil
		}
		if workspace.IsGitIgnored(path) && !options.ShowIgnored {
			return nil
		}
		files = append(files, path)
		return nil
	})
	if err != nil {
		return err
	}

	for _, filePath := range files {
		ignored := workspace.IsGitIgnored(filePath)
		generated := model.IsGenerated(filePath)
		kind := model.DeriveFileKind(filepath.Base(filePath))

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeKind(kind, options) {
			continue
		}

		name := filepath.Base(filePath)
		if !model.MatchesFilter(name, options) {
			continue
		}
		if !model.MatchesQuery(filePath, options) {
			continue
		}

		relPath, _ := filepath.Rel(workspace.RootDir, filePath)
		var bundleID *string
		if b := model.GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}

		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := model.BuildFolderID(folderPath, bundleID)
			folderID = &id
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- model.File{
				ID:        model.BuildFileID(relPath, bundleID),
				Path:      relPath,
				URI:       model.BuildFileUriFromPath(relPath),
				Name:      filepath.Base(relPath),
				Extension: filepath.Ext(relPath),
				FolderID:  folderID,
				BundleID:  bundleID,
				Kind:      kind,
				Ignored:   ignored,
				Generated: generated,
			}
		}
	}
	return nil
}

// 💙️flattenSections holds the data fields for a flattenSections record.
func flattenSections(sections []model.Section) []model.Section {
	return flattenSectionsWithPrefix(sections, "")
}

// 🔧️flattenSectionsWithPrefix holds the data fields for a flattenSectionsWithPrefix record.
func flattenSectionsWithPrefix(sections []model.Section, prefix string) []model.Section {
	var result []model.Section
	for _, s := range sections {
		children := s.Children
		s.Children = nil
		sPrefix := s.Name
		if s.Emoji != "" {
			sPrefix = s.Emoji + s.Name
		}
		if prefix != "" {
			s.Path = prefix + "#" + sPrefix
		} else {
			s.Path = sPrefix
		}
		result = append(result, s)
		result = append(result, flattenSectionsWithPrefix(children, s.Path)...)
	}
	return result
}

// 💚️hydrateSectionMetadata holds the data fields for a hydrateSectionMetadata record.
func hydrateSectionMetadata(s *model.Section, filePath string, prefix string) {
	s.FilePath = filePath
	s.Path = prefix
	for i := range s.Children {
		childEmoji := s.Children[i].Emoji
		childPrefix := s.Children[i].Name
		if childEmoji != "" {
			childPrefix = childEmoji + childPrefix
		}
		childPath := prefix + "#" + childPrefix
		hydrateSectionMetadata(&s.Children[i], filePath, childPath)
	}
}

// 💛️StreamSections MUST invoke the callback for each matching sections entry.
// ▪️StreamSections streams sections entries through the callback.
func StreamSections(ctx context.Context, scope string, out chan<- model.Section, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan model.File)
	go func() {
		StreamFiles(ctx, scope, fileChan, filesOpts)
	}()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(workspace.RootDir, fullPath)
		}
		content, err := workspace.ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		sections := languages.ParseSections(content, f.Path)
		if options.Filter != "" || options.Regex {
			flatSections := flattenSections(sections)
			for _, s := range flatSections {
				s.FilePath = f.Path
				if !model.MatchesFilter(s.Name, options) {
					continue
				}
				if !model.MatchesQuery(s.Name+" "+s.FilePath, options) {
					continue
				}

				select {
				case <-ctx.Done():
					return ctx.Err()
				default:
					out <- s
				}
			}
		} else {
			for i := range sections {
				rootPrefix := sections[i].Name
				if sections[i].Emoji != "" {
					rootPrefix = sections[i].Emoji + rootPrefix
				}
				hydrateSectionMetadata(&sections[i], f.Path, rootPrefix)
				select {
				case <-ctx.Done():
					return ctx.Err()
				default:
					out <- sections[i]
				}
			}
		}
	}
	return nil
}

// 🧡️StreamDefinitions MUST invoke the callback for each matching definitions entry.
// ▶️StreamDefinitions streams definitions entries through the callback.
func StreamDefinitions(ctx context.Context, scope string, out chan<- model.Definition, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan model.File)
	go func() {
		StreamFiles(ctx, scope, fileChan, filesOpts)
	}()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(workspace.RootDir, fullPath)
		}
		content, err := workspace.ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		for _, d := range defs {
			rawKind := d.Kind
			if rawKind == "" {
				rawKind = "definition"
			}
			kind := model.DeriveDefinitionKind(rawKind)

			if !shouldIncludeDefinitionKind(kind, options) {
				continue
			}

			def := model.Definition{
				Name:      d.Name,
				Kind:      kind,
				FilePath:  f.Path,
				StartLine: d.Start,
				EndLine:   d.End,
			}

			if !model.MatchesFilter(def.Name, options) {
				continue
			}
			if !model.MatchesQuery(def.Name+" "+string(def.Kind)+" "+def.FilePath, options) {
				continue
			}

			select {
			case <-ctx.Done():
				return ctx.Err()
			default:
				out <- def
			}
		}
	}
	return nil
}

// ❤️ResolveBundleForPath MUST return the resolved value or an error if unresolvable.
// 📩️ResolveBundleForPath resolves and returns the bundle for path.
func ResolveBundleForPath(filePath string, bundles []model.Bundle) string {
	var bestMatch string
	var maxLen int
	for _, b := range bundles {
		if b.Name == "" {
			continue
		}
		if strings.HasPrefix(filePath, b.Root+"/") || filePath == b.Root {
			if len(b.Root) > maxLen {
				maxLen = len(b.Root)
				bestMatch = model.NormalizeBundleLabel(b.Name)
			}
		}
	}
	return bestMatch
}

// 🤍️formatLineMetrics holds the data fields for a formatLineMetrics record.
func formatLineMetrics(metrics *model.LineMetrics) string {
	if metrics == nil {
		return ""
	}
	var parts []string
	if metrics.Removed > 0 {
		parts = append(parts, fmt.Sprintf("-%d", metrics.Removed))
	}
	if metrics.Added > 0 {
		parts = append(parts, fmt.Sprintf("+%d", metrics.Added))
	}
	if len(parts) == 0 {
		return ""
	}
	return " " + strings.Join(parts, " ")
}

// 🖤️formatPathWithBundle holds the data fields for a formatPathWithBundle record.
func formatPathWithBundle(path string, bundles []model.Bundle) string {
	bundleName := ResolveBundleForPath(path, bundles)
	if bundleName == "" {
		return path
	}
	bundleLabel := model.NormalizeBundleLabel(bundleName)
	root := ""
	for _, bundle := range bundles {
		if model.NormalizeBundleLabel(bundle.Name) == bundleLabel || bundle.Name == bundleName {
			root = bundle.Root
			break
		}
	}
	if root != "" {
		relative := strings.TrimPrefix(path, root+"/")
		if relative != path {
			if relative == "" {
				return bundleLabel
			}
			return bundleLabel + "/" + relative
		}
	}
	return bundleLabel + "/" + path
}

// 🤎️formatSemanticPath holds the data fields for a formatSemanticPath record.
func formatSemanticPath(path string, bundles []model.Bundle) string {
	filePath := model.ExtractFilePrefix(path)
	remainder := path[len(filePath):]
	return formatPathWithBundle(filePath, bundles) + remainder
}

// 💗️formatDeletedPath holds the data fields for a formatDeletedPath record.
func formatDeletedPath(path string, bundles []model.Bundle) string {
	filePath := model.ExtractFilePrefix(path)
	remainder := path[len(filePath):]
	base := formatPathWithBundle(filePath, bundles)
	if remainder == "" {
		return "<del>" + base + "</del>"
	}
	return base + "<del>" + remainder + "</del>"
}

// 💖️commonPrefixLength holds the data fields for a commonPrefixLength record.
func commonPrefixLength(a, b string) int {
	limit := len(a)
	if len(b) < limit {
		limit = len(b)
	}
	idx := 0
	for idx < limit {
		if a[idx] != b[idx] {
			break
		}
		idx++
	}
	return idx
}

// 💝️commonSuffixLength holds the data fields for a commonSuffixLength record.
func commonSuffixLength(a, b string, prefix int) int {
	max := len(a) - prefix
	if len(b)-prefix < max {
		max = len(b) - prefix
	}
	idx := 0
	for idx < max {
		if a[len(a)-1-idx] != b[len(b)-1-idx] {
			break
		}
		idx++
	}
	return idx
}

// 💘️formatRenameDelta holds the data fields for a formatRenameDelta record.
func formatRenameDelta(from, to string) string {
	if from == to {
		return from
	}
	prefix := commonPrefixLength(from, to)
	suffix := commonSuffixLength(from, to, prefix)
	fromMiddle := from[prefix : len(from)-suffix]
	toMiddle := to[prefix : len(to)-suffix]
	return from[:prefix] + "<del>" + fromMiddle + "</del>" + toMiddle + from[len(from)-suffix:]
}

func formatRenamePath(from, to string, bundles []model.Bundle) string {
	fromFormatted := formatSemanticPath(from, bundles)
	toFormatted := formatSemanticPath(to, bundles)
	return formatRenameDelta(fromFormatted, toFormatted)
}

// ➡️appendDiffLines holds the data fields for a appendDiffLines record.
func appendDiffLines(lines *[]string, diffSet model.TicketDiffSet, iconAdded, iconChanged, iconRemoved, iconRenamed string, bundles []model.Bundle, formatter func(string) string, renameFormatter func(string, string) string) {
	if len(diffSet.Added) > 0 {
		sort.Slice(diffSet.Added, func(i, j int) bool { return diffSet.Added[i].Path < diffSet.Added[j].Path })
		for _, entry := range diffSet.Added {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconAdded, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Modified) > 0 {
		sort.Slice(diffSet.Modified, func(i, j int) bool { return diffSet.Modified[i].Path < diffSet.Modified[j].Path })
		for _, entry := range diffSet.Modified {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconChanged, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Renamed) > 0 {
		sort.Slice(diffSet.Renamed, func(i, j int) bool { return diffSet.Renamed[i].To < diffSet.Renamed[j].To })
		for _, entry := range diffSet.Renamed {
			path := renameFormatter(entry.From, entry.To)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRenamed, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Deleted) > 0 {
		sort.Slice(diffSet.Deleted, func(i, j int) bool { return diffSet.Deleted[i].Path < diffSet.Deleted[j].Path })
		for _, entry := range diffSet.Deleted {
			path := formatDeletedPath(entry.Path, bundles)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRemoved, path, formatLineMetrics(entry.Lines)))
		}
	}
}

// 💕️generateMetricsComment holds the data fields for a generateMetricsComment record.
func generateMetricsComment(diffs *model.TicketDiffs, bundles []model.Bundle) string {
	if diffs == nil {
		return ""
	}
	var lines []string
	appendDiffLines(&lines, diffs.Bundles, "📦️", "📦️", "📦️", "📦️", bundles, func(path string) string {
		return path
	}, func(from, to string) string {
		return formatRenameDelta(from, to)
	})
	appendDiffLines(&lines, diffs.Folders, "📂️", "📁️", "📁️", "📁️", bundles, func(path string) string {
		return formatPathWithBundle(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Files, "📄️", "📝️", "📄️", "📄️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Sections, "📑️", "🔖️", "🔖️", "🔖️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Definitions, "🏷️", "🏷️", "🏷️", "🏷️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	if len(lines) == 0 {
		return ""
	}
	return strings.Join(lines, "\n")
}

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

// 🔖️printTree holds the data fields for a printTree record.
func PrintTree(output *workspace.CommandOutput, dir, prefix string) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return
	}
	var items []os.DirEntry
	for _, e := range entries {
		if !strings.HasPrefix(e.Name(), ".") {
			items = append(items, e)
		}
	}
	var relPaths []string
	for _, e := range items {
		relPaths = append(relPaths, workspace.GetRelativePath(filepath.Join(dir, e.Name())))
	}
	ignored := workspace.GetGitIgnoredSet(relPaths)
	var filtered []os.DirEntry
	for _, e := range items {
		relPath := workspace.GetRelativePath(filepath.Join(dir, e.Name()))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, e)
		}
	}
	for i, e := range filtered {
		isLast := i == len(filtered)-1
		connector := "├️─️─️ "
		if isLast {
			connector = "└️─️─️ "
		}
		suffix := ""
		if e.IsDir() {
			suffix = "/"
		}
		output.Plain(fmt.Sprintf("%s%s%s%s", prefix, connector, e.Name(), suffix))
		if e.IsDir() {
			newPrefix := prefix + "│️   "
			if isLast {
				newPrefix = prefix + "    "
			}
			PrintTree(output, filepath.Join(dir, e.Name()), newPrefix)
		}
	}
}

// 🏺️SectionHeaderId MUST return the section artifact ID for a file path and section path.
// ◼SectionHeaderId returns the section identification string.
func SectionHeaderId(filePath string, sectionPath string) string {
	normalizedFilePath := workspace.NormalizePath(filePath)
	fileID := model.BuildFileID(normalizedFilePath, nil)
	path := strings.ReplaceAll(sectionPath, "/", "#")
	parts := strings.Split(path, "#")
	return model.BuildSectionID(fileID, parts)
}

// 🔖️SectionHeaderUri MUST return the repo URI for a section.
// 🔵️SectionHeaderUri returns the section artifact URI.
func SectionHeaderUri(filePath string, sectionPath string) string {
	path := filePath + "#" + sectionPath
	data := map[string]interface{}{"path": path}
	return model.GetArtifactURI("section", data)
}

// 🔖️DefinitionHeaderId MUST return the definition artifact ID for a file path, section path, and definition name.
// 🟡️DefinitionHeaderId returns the definition identification string.
func DefinitionHeaderId(filePath string, sectionPath string, name string, kind model.DefinitionKind) string {
	normalizedFilePath := workspace.NormalizePath(filePath)
	fileID := model.BuildFileID(normalizedFilePath, nil)
	normalizedSectionPath := strings.ReplaceAll(sectionPath, "#", "/")
	var sectionParts []string
	if normalizedSectionPath != "" {
		sectionParts = strings.Split(normalizedSectionPath, "/")
	}
	return model.BuildDefinitionID(fileID, sectionParts, name, kind)
}

// 🔖️DefinitionHeaderUri MUST return the repo URI for a definition.
// 🩷️DefinitionHeaderUri returns the definition artifact URI.
func DefinitionHeaderUri(filePath string, sectionPath string, name string) string {
	val := filePath
	if sectionPath != "" {
		val += "#" + sectionPath
	}
	val += "§" + name
	data := map[string]interface{}{"id": val}
	return model.GetArtifactURI("definition", data)
}

// 🔖️RemoveAgentsDocsEntry MUST remove the target and return an error on failure.
// 🧪️RemoveAgentsDocsEntry removes the specified agents docs entry.
func RemoveAgentsDocsEntry(filePath string) {
	agentsPath := filepath.Join(workspace.RootDir, "AGENTS.md")
	if !workspace.FileExists(agentsPath) {
		return
	}
	content, err := workspace.ReadTextFile(agentsPath)
	if err != nil {
		return
	}

	norm := workspace.NormalizePath(filePath)
	lines := strings.Split(content, "\n")
	var newLines []string
	skip := false

	for _, line := range lines {
		if strings.HasPrefix(line, "#") {
			if skip {
				skip = false
			}
			if strings.HasPrefix(line, "## ") {
				header := line[3:]
				cleanHeader := strings.ReplaceAll(header, "\uFE0E", "")
				cleanHeader = strings.ReplaceAll(cleanHeader, "\uFE0F", "")
				runes := []rune(cleanHeader)
				if len(runes) > 1 {
					pathPart := strings.TrimSuffix(string(runes[1:]), "/")
					if pathPart == norm {
						skip = true
						continue
					}
				}
			}
		}
		if !skip {
			newLines = append(newLines, line)
		}
	}

	newContent := strings.Join(newLines, "\n")
	if newContent != content {
		workspace.WriteTextFile(agentsPath, newContent)
	}
}

// #endregion 📋️Tickets

// #region 🧬️Missing Utilities

// 🎫️ComputeTicketFiles MUST return the computed result deterministically.
// ComputeTicketFiles computes and returns the ticket files.
// 💾️Uses unstaged diffs only (index vs working tree) for complete, current working state.
func ComputeTicketFiles(ticket *model.Ticket, files []string) (*model.TicketDiffs, error) {
	files = normalizeTicketFileInputs(files)
	if ticket != nil && ticket.FolderPath != "" {
		files = FilterTicketWorkspaceFiles(workspace.NormalizeRepoPath(ticket.FolderPath), files)
	}
	files = model.FilterConsideredFiles(files)
	files = model.FilterGitIgnored(files)
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}
	baseRef := providers.GitIndexRef
	diffStatuses, err := providers.GetGitDiffStatus(baseRef, "", files)
	if err != nil {
		return nil, err
	}
	diffLines, err := providers.GetGitDiffLines(baseRef, "", files)
	if err != nil {
		return nil, err
	}

	currentFiles := make(map[string]struct{})
	baseFiles := make(map[string]struct{})
	for _, file := range files {
		currentFiles[file] = struct{}{}
		baseFiles[file] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.To != "" {
			currentFiles[status.To] = struct{}{}
		}
		if status.From != "" {
			baseFiles[status.From] = struct{}{}
		}
	}
	var currentFileList []string
	for file := range currentFiles {
		currentFileList = append(currentFileList, file)
	}
	var baseFileList []string
	for file := range baseFiles {
		baseFileList = append(baseFileList, file)
	}

	bundles := codebase.GetTechnologies()
	baseCodebase, err := codebase.BuildCodebaseSnapshot(baseFileList, bundles, baseRef)
	if err != nil {
		return nil, err
	}
	currentCodebase, err := codebase.BuildCodebaseSnapshot(currentFileList, bundles, "")
	if err != nil {
		return nil, err
	}
	result := BuildSemanticDiffs(baseCodebase, currentCodebase, baseRef, diffLines, diffStatuses, bundles)
	return result, nil
}

// #endregion 🧬️Missing Utilities

// #region ⚡️Server Client

// ⚙️syncTicketToServer sends ticket state to the server. No-operation if server is not configured.
func syncTicketToServer(ticket *model.Ticket, action string) {
	addr := events.GetServerAddr()
	if addr == "" {
		return
	}
	ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	payload := map[string]interface{}{
		"action":    action,
		"ticket_id": ticketID,
		"title":     ticket.Title,
		"prompt":    ticket.Description,
		"summary":   ticket.Summary,
		"goal":      ticket.Goal,
		"parent":    ticket.Parent,
		"author":    contributors.GetGitAuthorAlias(),
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		payload["github_issue"] = ticket.Management.Issue
	}
	for _, s := range ticket.Sessions {
		if s != "" {
			payload["session_id"] = s
			break
		}
	}
	resp, err := events.ServerRequest("POST", "/api/v1/tickets", payload)
	if err != nil {
		return
	}
	resp.Body.Close()
}

// #endregion ⚡️Server Client

// #region 📦️MoveTicketPlan

func moveTicketPlanIntoFolder(ticket *model.Ticket) error {
	if ticket == nil || ticket.Plan == nil || strings.TrimSpace(ticket.Plan.Source) == "" {
		return nil
	}
	src := filepath.Clean(ticket.Plan.Source)
	if ticket.FolderPath == "" {
		return fmt.Errorf("ticket folder path is empty")
	}
	destName := filepath.Base(src)
	dest := filepath.Join(ticket.FolderPath, destName)
	if _, err := os.Stat(src); os.IsNotExist(err) {
		if _, err2 := os.Stat(dest); err2 == nil {
			ticket.Plan.Local = destName
			ticket.Plan.Source = ""
			return nil
		}
		return fmt.Errorf("plan source %q missing and destination %q not found", src, dest)
	}
	st, err := os.Stat(src)
	if err != nil {
		return fmt.Errorf("plan source missing: %w", err)
	}
	if st.IsDir() {
		if err := os.Rename(src, dest); err != nil {
			if err := move.CopyDirTree(src, dest); err != nil {
				return fmt.Errorf("copy spec directory: %w", err)
			}
			if err := os.RemoveAll(src); err != nil {
				return fmt.Errorf("remove spec source after copy: %w", err)
			}
		}
	} else {
		if err := move.MoveFile(src, dest); err != nil {
			return fmt.Errorf("move plan file: %w", err)
		}
	}
	ticket.Plan.Local = destName
	ticket.Plan.Source = ""
	return nil
}

// #endregion 📦️MoveTicketPlan

// #region 📝️TicketPlanComment

// #region 📝️TicketPlanComment
// 📝️stripPlanFrontmatter removes YAML frontmatter from plan markdown.
func stripPlanFrontmatter(content string) string {
	if !strings.HasPrefix(content, "---") {
		return strings.TrimSpace(content)
	}
	end := strings.Index(content[3:], "\n---")
	if end < 0 {
		return strings.TrimSpace(content)
	}
	rest := content[3+end+4:]
	if strings.HasPrefix(rest, "\r\n") {
		rest = rest[2:]
	} else if strings.HasPrefix(rest, "\n") {
		rest = rest[1:]
	}
	return strings.TrimSpace(rest)
}

// 📝️formatPlanFileSection formats one plan file as a collapsible GitHub markdown section.
func formatPlanFileSection(path string) (string, error) {
	raw, err := workspace.ReadTextFile(path)
	if err != nil {
		return "", err
	}
	body := stripPlanFrontmatter(raw)
	if strings.TrimSpace(body) == "" {
		return "", nil
	}
	name := filepath.Base(path)
	return fmt.Sprintf("<details>\n<summary>%s</summary>\n\n%s\n\n</details>", name, body), nil
}

// 📝️formatPlanComment builds a GitHub issue comment body from a bound plan or spec.
func formatPlanComment(plan *model.TicketPlan, src string) (string, error) {
	if plan == nil || strings.TrimSpace(src) == "" {
		return "", nil
	}
	st, err := os.Stat(src)
	if err != nil {
		return "", err
	}
	var sections []string
	if st.IsDir() {
		matches, gerr := filepath.Glob(filepath.Join(src, "*.md"))
		if gerr != nil {
			return "", gerr
		}
		sort.Strings(matches)
		for _, p := range matches {
			sec, serr := formatPlanFileSection(p)
			if serr != nil {
				return "", serr
			}
			if sec != "" {
				sections = append(sections, sec)
			}
		}
	} else {
		sec, serr := formatPlanFileSection(src)
		if serr != nil {
			return "", serr
		}
		if sec != "" {
			sections = append(sections, sec)
		}
	}
	if len(sections) == 0 {
		return "", nil
	}
	return "# 📋️ Plan\n\n" + strings.Join(sections, "\n\n"), nil
}

// 📝️postTicketPlanComment posts the bound plan markdown to the ticket GitHub issue.
func postTicketPlanComment(ticket *model.Ticket, noManagement bool) {
	if noManagement || ticket == nil || ticket.Plan == nil {
		return
	}
	src := strings.TrimSpace(ticket.Plan.Source)
	if src == "" {
		return
	}
	if ticket.Management == nil || strings.TrimSpace(ticket.Management.Issue) == "" {
		return
	}
	body, err := formatPlanComment(ticket.Plan, src)
	if err != nil {
		workspace.WriteWarningf("Failed to format plan comment: %v", err)
		return
	}
	if strings.TrimSpace(body) == "" {
		return
	}
	issueURL := ticket.Management.Issue
	if err := providers.GetManagementProvider().AddComment(issueURL, body); err != nil {
		workspace.WriteWarningf("Failed to add plan comment to GitHub issue: %v", err)
	}
}

// #endregion 📝️TicketPlanComment

// #region 🔌️Ticket Ports

// 🔌️init wires the ticket read 🗂️codebase declares but cannot implement.
func init() {
	codebase.LookupCodebaseTickets = func() ([]model.Ticket, error) { return ListTickets(nil, nil, nil) }
}

// #endregion 🔌️Ticket Ports

// #endregion 🚚️Split

// #region 🚪️Ports

// #region 🔁️Reexports

// 🐙️ManagementIssue is the issue shape of 🧩️providers, reexported so a client needs one import.
type ManagementIssue = providers.ManagementIssue

// 🎯️ManagementMilestone is the milestone shape of 🧩️providers.
type ManagementMilestone = providers.ManagementMilestone

// 🏷️ManagementLabel is the label shape of 🧩️providers.
type ManagementLabel = providers.ManagementLabel

// 🪪️McpClientKind is the MCP surface a plan or spec id is resolved against.
type McpClientKind = providers.McpClientKind

// 🐙️IssueTracker is everything the ticket domain asks of an issue tracker.
type IssueTracker interface {
	// 🆕️CreateIssue opens an issue and returns its URL.
	CreateIssue(title, body string, milestone *int) (string, error)
	// 📪️CloseIssue closes an issue.
	CloseIssue(issueURL string) error
	// 🔓️ReopenIssue reopens an issue.
	ReopenIssue(issueURL string) error
	// 🔎️GetIssueDetails reads an issue, or nothing when it cannot be read.
	GetIssueDetails(issueURL string) (*ManagementIssue, error)
	// 💬️AddComment comments on an issue.
	AddComment(issueURL, comment string) error
	// 🏷️AddLabels labels an issue.
	AddLabels(issueURL string, labels []string) error
	// 🏷️RemoveLabels unlabels an issue.
	RemoveLabels(issueURL string, labels []string) error
	// 🎯️FindMilestoneByTitle looks a milestone up by title.
	FindMilestoneByTitle(title string) (*ManagementMilestone, error)
	// 🏷️ListRepoLabels lists the labels of the repository.
	ListRepoLabels() ([]ManagementLabel, error)
}

// #endregion 🔁️Reexports

// #region ❌️Errors

// ⚠️TicketError is every ticket failure, carrying a class for machine consumers and a message.
type TicketError struct {
	Class   string `json:"class"`
	Message string `json:"message"`
}

// 🔤️Error renders the failure message.
func (err TicketError) Error() string { return err.Message }

// 🏷️TicketErrorClass returns the class of any failure this domain produced.
func TicketErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var ticketError TicketError
	if errors.As(err, &ticketError) {
		return ticketError.Class
	}
	return "store"
}

// 🚫️errInvalid refuses an input the caller may correct.
func errInvalid(message string) error { return TicketError{Class: "invalid", Message: message} }

// 🔍️errTicketNotFound refuses a ticket, document or node that is not there.
func errTicketNotFound(message string) error {
	return TicketError{Class: "not-found", Message: message}
}

// 💥️errStore refuses a store operation that failed.
func errStore(message string) error { return TicketError{Class: "store", Message: message} }

// 🔀️errConflict refuses a state that forbids the requested transition.
func errConflict(message string) error { return TicketError{Class: "conflict", Message: message} }

// #endregion ❌️Errors

// #region 🪪️Ids

// 🎆️EmojiYear is the year directory prefix.
const EmojiYear = "🎆️"

// 🌙️EmojiMonth is the month directory prefix.
const EmojiMonth = "🌙️"

// ☀️EmojiDay is the day directory prefix.
const EmojiDay = "☀️"

// 🎫️TicketsDirName is the tickets collection directory, under the repository meta directory.
const TicketsDirName = "🎫️tickets"

// 🎫️TicketDocumentName is the persisted ticket document.
const TicketDocumentName = "🎫️ticket.json"

// 📌️ImportantDirName is the directory holding the important document of a ticket.
const ImportantDirName = "📌️important"

// 📝️ImportantDocumentName is the important document itself.
const ImportantDocumentName = "📝️.md"

// 🚷️SkippedWalkDirs are the directory names a ticket walk never descends into.
var SkippedWalkDirs = []string{"node_modules", "dist", "build", "target", "__pycache__"}

// 🔢️PadNumber left-pads a number with zeroes to a width.
func PadNumber(value int, width int) string {
	negative := value < 0
	magnitude := value
	if negative {
		magnitude = -value
	}
	digits := strconv.Itoa(magnitude)
	padding := width - len(digits)
	if negative {
		padding--
	}
	var builder strings.Builder
	if negative {
		builder.WriteByte('-')
	}
	for index := 0; index < padding; index++ {
		builder.WriteByte('0')
	}
	builder.WriteString(digits)
	return builder.String()
}

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string { return EmojiYear + PadNumber(year, 2) }

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string { return EmojiMonth + PadNumber(month, 2) }

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string { return EmojiDay + PadNumber(day, 2) }

// 🧭️ParseDatedDirSegment parses one canonical emoji-prefixed date directory segment.
func ParseDatedDirSegment(segment, prefix string) (int, error) {
	if !strings.HasPrefix(segment, prefix) {
		return 0, errInvalid(fmt.Sprintf("date directory %q is missing prefix %q", segment, prefix))
	}
	value, err := strconv.Atoi(segment[len(prefix):])
	if err != nil {
		return 0, errInvalid(fmt.Sprintf("invalid date directory %q", segment))
	}
	return value, nil
}

// 🎫️TicketID is the dated identity of a ticket, in both of its spellings.
type TicketID struct {
	Year  int    `json:"year"`
	Month int    `json:"month"`
	Day   int    `json:"day"`
	Slug  string `json:"slug"`
}

// 🆕️NewTicketID builds an identity from its parts.
func NewTicketID(year, month, day int, slug string) TicketID {
	return TicketID{Year: year, Month: month, Day: day, Slug: slug}
}

// 🪪️ID returns the logical identity a dev types and a URI carries.
func (id TicketID) ID() string {
	return fmt.Sprintf("%s/%s/%s/%s", PadNumber(id.Year, 2), PadNumber(id.Month, 2), PadNumber(id.Day, 2), id.Slug)
}

// 🗂️RelPath returns the relative folder path under the tickets directory.
func (id TicketID) RelPath() string {
	return fmt.Sprintf("%s/%s/%s/%s", FormatYearDir(id.Year), FormatMonthDir(id.Month), FormatDayDir(id.Day), id.Slug)
}

// 🌐️URI returns the artifact URI of the ticket.
func (id TicketID) URI() string { return "repo://ticket/" + id.RelPath() }

// 👪️ParentSlug returns the slug of the parent ticket when this one is nested.
func (id TicketID) ParentSlug() (string, bool) {
	index := strings.LastIndex(id.Slug, "/")
	if index < 0 {
		return "", false
	}
	return id.Slug[:index], true
}

// 🧭️ParseTicketID reads either spelling back. A slug may itself carry a separator.
func ParseTicketID(value string) (TicketID, error) {
	trimmed := strings.Trim(strings.TrimSpace(value), "/")
	segments := strings.Split(trimmed, "/")
	if len(segments) < 4 {
		return TicketID{}, errInvalid(fmt.Sprintf("ticket id %q must be YY/MM/DD/SLUG", value))
	}
	read := func(segment, prefix string) (int, error) {
		if strings.HasPrefix(segment, prefix) {
			return ParseDatedDirSegment(segment, prefix)
		}
		parsed, err := strconv.Atoi(segment)
		if err != nil {
			return 0, errInvalid(fmt.Sprintf("invalid date segment %q", segment))
		}
		return parsed, nil
	}
	year, err := read(segments[0], EmojiYear)
	if err != nil {
		return TicketID{}, err
	}
	month, err := read(segments[1], EmojiMonth)
	if err != nil {
		return TicketID{}, err
	}
	day, err := read(segments[2], EmojiDay)
	if err != nil {
		return TicketID{}, err
	}
	slug := strings.Join(segments[3:], "/")
	if slug == "" {
		return TicketID{}, errInvalid(fmt.Sprintf("ticket id %q carries no slug", value))
	}
	return TicketID{Year: year, Month: month, Day: day, Slug: slug}, nil
}

// 🗺️TicketLayout derives every path the ticket domain owns from one repository meta directory.
type TicketLayout struct {
	repoMetaDir string
}

// 🆕️NewTicketLayout anchors the layout at a repository meta directory.
func NewTicketLayout(repoMetaDir string) TicketLayout {
	return TicketLayout{repoMetaDir: NormalizeSeparators(repoMetaDir)}
}

// 🏠️RepoMetaDir returns the meta directory the layout is anchored at.
func (layout TicketLayout) RepoMetaDir() string { return layout.repoMetaDir }

// 🎫️TicketsDir returns the tickets collection directory.
func (layout TicketLayout) TicketsDir() string { return JoinPath(layout.repoMetaDir, TicketsDirName) }

// 🎆️YearDir returns the directory of one year.
func (layout TicketLayout) YearDir(year int) string {
	return JoinPath(layout.TicketsDir(), FormatYearDir(year))
}

// 🌙️MonthDir returns the directory of one month.
func (layout TicketLayout) MonthDir(year, month int) string {
	return JoinPath(layout.YearDir(year), FormatMonthDir(month))
}

// ☀️DayDir returns the directory of one day.
func (layout TicketLayout) DayDir(year, month, day int) string {
	return JoinPath(layout.MonthDir(year, month), FormatDayDir(day))
}

// 🗂️TicketDir returns the folder of one ticket.
func (layout TicketLayout) TicketDir(id TicketID) string {
	return JoinPath(layout.TicketsDir(), id.RelPath())
}

// 🎫️DocumentPath returns the persisted document of one ticket.
func (layout TicketLayout) DocumentPath(id TicketID) string {
	return JoinPath(layout.TicketDir(id), TicketDocumentName)
}

// 📌️ImportantDir returns the important directory of one ticket.
func (layout TicketLayout) ImportantDir(id TicketID) string {
	return JoinPath(layout.TicketDir(id), ImportantDirName)
}

// 📝️ImportantPath returns the important document of one ticket.
func (layout TicketLayout) ImportantPath(id TicketID) string {
	return JoinPath(layout.ImportantDir(id), ImportantDocumentName)
}

// ➗️NormalizeSeparators replaces every backslash with a forward slash and collapses repeats.
func NormalizeSeparators(path string) string {
	replaced := strings.ReplaceAll(path, "\\", "/")
	var builder strings.Builder
	previousSeparator := false
	for _, character := range replaced {
		if character == '/' {
			if previousSeparator {
				continue
			}
			previousSeparator = true
		} else {
			previousSeparator = false
		}
		builder.WriteRune(character)
	}
	normalized := builder.String()
	if len(normalized) > 1 && strings.HasSuffix(normalized, "/") {
		normalized = normalized[:len(normalized)-1]
	}
	return normalized
}

// 🔗️JoinPath joins two path fragments with a single forward slash.
func JoinPath(base, child string) string {
	base = NormalizeSeparators(base)
	child = NormalizeSeparators(child)
	if base == "" {
		return child
	}
	if child == "" {
		return base
	}
	return strings.TrimRight(base, "/") + "/" + strings.TrimLeft(child, "/")
}

// 📛️BaseName returns the last segment of a path.
func BaseName(path string) string {
	normalized := NormalizeSeparators(path)
	if index := strings.LastIndex(normalized, "/"); index >= 0 {
		return normalized[index+1:]
	}
	return normalized
}

// 🗂️DirName returns everything before the last segment of a path.
func DirName(path string) string {
	normalized := NormalizeSeparators(path)
	index := strings.LastIndex(normalized, "/")
	if index < 0 {
		return "."
	}
	if index == 0 {
		return "/"
	}
	return normalized[:index]
}

// 🔤️TicketSlugFromTitle returns the ticket slug of a title.
func TicketSlugFromTitle(title string) (string, error) {
	trimmed := strings.TrimSpace(title)
	if trimmed == "" {
		return "", errInvalid("ticket title is required")
	}
	slug := identity.Slugify(trimmed)
	if slug == "" {
		return "", errInvalid("ticket title must contain at least one alphanumeric character")
	}
	return slug, nil
}

// 🎫️ValidateTicketEmojiTitle validates the emoji and title pair and returns the derived slug.
func ValidateTicketEmojiTitle(emoji, title string) (string, error) {
	emoji = strings.TrimSpace(emoji)
	if emoji == "" {
		return "", errInvalid("ticket emoji is required")
	}
	extracted, remaining := identity.ExtractEntityEmoji(emoji)
	if extracted == "" || strings.TrimSpace(remaining) != "" {
		return "", errInvalid("ticket emoji must be a single emoji character")
	}
	return TicketSlugFromTitle(title)
}

// #endregion 🪪️Ids

// #region 🔤️GoJson

// 🔤️GoJSONString encodes one string the way Go's encoding/json does, HTML escaping included.
func GoJSONString(value string) string {
	var builder strings.Builder
	builder.WriteByte('"')
	for _, character := range value {
		switch {
		case character == '"':
			builder.WriteString("\\\"")
		case character == '\\':
			builder.WriteString("\\\\")
		case character == '\n':
			builder.WriteString("\\n")
		case character == '\r':
			builder.WriteString("\\r")
		case character == '\t':
			builder.WriteString("\\t")
		case character == '<':
			builder.WriteString("\\u003c")
		case character == '>':
			builder.WriteString("\\u003e")
		case character == '&':
			builder.WriteString("\\u0026")
		case character == ' ':
			builder.WriteString("\\u2028")
		case character == ' ':
			builder.WriteString("\\u2029")
		case character < 0x20:
			builder.WriteString(fmt.Sprintf("\\u%04x", character))
		default:
			builder.WriteRune(character)
		}
	}
	builder.WriteByte('"')
	return builder.String()
}

// 🧱️goMember is one member of a Go-shaped JSON object, in declaration order.
type goMember struct {
	name    string
	text    string
	nested  []goMember
	strings []string
	kind    int
}

// 🖨️renderGoMembers renders members the way MarshalIndent renders a struct.
func renderGoMembers(members []goMember, depth int) string {
	if len(members) == 0 {
		return "{}"
	}
	indent := strings.Repeat("  ", depth+1)
	closing := strings.Repeat("  ", depth)
	lines := make([]string, 0, len(members))
	for _, member := range members {
		switch member.kind {
		case 0:
			lines = append(lines, indent+GoJSONString(member.name)+": "+GoJSONString(member.text))
		case 1:
			lines = append(lines, indent+GoJSONString(member.name)+": "+renderGoMembers(member.nested, depth+1))
		default:
			if len(member.strings) == 0 {
				lines = append(lines, indent+GoJSONString(member.name)+": []")
				continue
			}
			elementIndent := strings.Repeat("  ", depth+2)
			elements := make([]string, 0, len(member.strings))
			for _, value := range member.strings {
				elements = append(elements, elementIndent+GoJSONString(value))
			}
			lines = append(lines, indent+GoJSONString(member.name)+": [\n"+strings.Join(elements, ",\n")+"\n"+indent+"]")
		}
	}
	return "{\n" + strings.Join(lines, ",\n") + "\n" + closing + "}"
}

// #endregion 🔤️GoJson

// #region 📄️Codec

// 🔤️textMember returns a text member, or the empty string when it is absent or not a string.
func textMember(members map[string]interface{}, name string) string {
	if value, ok := members[name].(string); ok {
		return value
	}
	return ""
}

// 🧺️objectsOf returns every object element of an array member.
func objectsOf(value interface{}) []map[string]interface{} {
	entries, ok := value.([]interface{})
	if !ok {
		return nil
	}
	found := make([]map[string]interface{}, 0, len(entries))
	for _, entry := range entries {
		if members, ok := entry.(map[string]interface{}); ok {
			found = append(found, members)
		}
	}
	return found
}

// 🟫️decodeInteraction decodes one interaction leniently, every absent member taking its zero value.
func decodeInteraction(members map[string]interface{}) model.Interaction {
	files := make([]model.InteractionFile, 0)
	for _, file := range objectsOf(members["files"]) {
		files = append(files, model.InteractionFile{Path: textMember(file, "path"), ID: textMember(file, "id"), URI: textMember(file, "uri")})
	}
	if len(files) == 0 {
		files = nil
	}
	return model.Interaction{
		Kind:       textMember(members, "kind"),
		Date:       textMember(members, "date"),
		Author:     textMember(members, "author"),
		System:     textMember(members, "system"),
		Client:     textMember(members, "client"),
		Checkpoint: textMember(members, "checkpoint"),
		Prompt:     textMember(members, "prompt"),
		Summary:    textMember(members, "summary"),
		LLM:        textMember(members, "llm"),
		Effort:     textMember(members, "effort"),
		Files:      files,
	}
}

// 🔳️decodeAgent decodes one agent record leniently, plan included only when it decodes cleanly.
func decodeAgent(members map[string]interface{}) model.TicketAgent {
	agent := model.TicketAgent{
		Session:     textMember(members, "session"),
		Contributor: textMember(members, "contributor"),
		System:      textMember(members, "system"),
		Client:      textMember(members, "client"),
		LLM:         textMember(members, "llm"),
		Effort:      textMember(members, "effort"),
		Transcript:  textMember(members, "transcript"),
	}
	if raw, found := members["plan"]; found {
		if data, err := json.Marshal(raw); err == nil {
			var plan model.TicketAgentPlan
			if err := json.Unmarshal(data, &plan); err == nil {
				agent.Plan = &plan
			}
		}
	}
	return agent
}

// 📖️DecodeTicketDocument decodes one ticket document. The status member is mandatory and closed.
func DecodeTicketDocument(text string) (*model.Ticket, error) {
	var value interface{}
	if err := json.Unmarshal([]byte(text), &value); err != nil {
		return nil, errInvalid(err.Error())
	}
	members, ok := value.(map[string]interface{})
	if !ok {
		return nil, errInvalid("ticket document must be a JSON object")
	}
	status, ok := members["status"].(string)
	if !ok || (status != string(model.TicketStatusOpen) && status != string(model.TicketStatusClosed)) {
		return nil, errInvalid("ticket status must be explicitly \"open\" or \"closed\"")
	}
	var management *model.TicketManagementData
	if nested, ok := members["github"].(map[string]interface{}); ok {
		management = &model.TicketManagementData{Issue: textMember(nested, "issue")}
	}
	var plan *model.TicketPlan
	if nested, ok := members["plan"].(map[string]interface{}); ok {
		plan = &model.TicketPlan{
			Client: textMember(nested, "client"),
			ID:     textMember(nested, "id"),
			Source: textMember(nested, "source"),
			Local:  textMember(nested, "local"),
		}
	}
	interactions := make([]model.Interaction, 0)
	for _, entry := range objectsOf(members["interactions"]) {
		interactions = append(interactions, decodeInteraction(entry))
	}
	agents := make([]model.TicketAgent, 0)
	for _, entry := range objectsOf(members["agents"]) {
		agents = append(agents, decodeAgent(entry))
	}

	ticket := &model.Ticket{
		Title:        textMember(members, "title"),
		Emoji:        textMember(members, "emoji"),
		Status:       model.TicketStatus(status),
		Description:  textMember(members, "description"),
		Summary:      textMember(members, "summary"),
		Management:   management,
		Goal:         textMember(members, "goal"),
		Parent:       textMember(members, "parent"),
		Plan:         plan,
		Interactions: interactions,
	}
	if ticket.Description == "" {
		ticket.Description = textMember(members, "prompt")
	}
	if entries, ok := members["sessions"].([]interface{}); ok {
		allStrings := true
		for _, entry := range entries {
			if _, ok := entry.(string); !ok {
				allStrings = false
				break
			}
		}
		if allStrings {
			for _, entry := range entries {
				AppendSessionID(ticket, entry.(string))
			}
		} else {
			legacy := make([]model.TicketAgent, 0)
			for _, entry := range objectsOf(members["sessions"]) {
				legacy = append(legacy, decodeAgent(entry))
			}
			for _, agent := range legacy {
				AppendSessionID(ticket, agent.Session)
			}
			if len(agents) == 0 {
				agents = legacy
			}
		}
	}
	ticket.Agents = agents
	if ticket.Summary == "" {
		for index := len(ticket.Interactions) - 1; index >= 0; index-- {
			interaction := ticket.Interactions[index]
			if IsTicketInteractionKind(interaction.Kind, "ticket.close") && interaction.Summary != "" {
				ticket.Summary = interaction.Summary
				break
			}
		}
	}
	for _, agent := range ticket.Agents {
		AppendSessionID(ticket, agent.Session)
	}
	return ticket, nil
}

// 🖨️EncodeTicketDocument encodes one ticket document, member order and omission rules included.
func EncodeTicketDocument(ticket *model.Ticket) string {
	members := make([]goMember, 0, 9)
	members = append(members, goMember{name: "title", text: ticket.Title})
	if ticket.Emoji != "" {
		members = append(members, goMember{name: "emoji", text: ticket.Emoji})
	}
	members = append(members, goMember{name: "status", text: string(ticket.Status)})
	if ticket.Description != "" {
		members = append(members, goMember{name: "description", text: ticket.Description})
	}
	if ticket.Summary != "" {
		members = append(members, goMember{name: "summary", text: ticket.Summary})
	}
	if ticket.Management != nil {
		nested := make([]goMember, 0, 1)
		if ticket.Management.Issue != "" {
			nested = append(nested, goMember{name: "issue", text: ticket.Management.Issue})
		}
		members = append(members, goMember{name: "github", nested: nested, kind: 1})
	}
	if ticket.Goal != "" {
		members = append(members, goMember{name: "goal", text: ticket.Goal})
	}
	if ticket.Plan != nil {
		nested := make([]goMember, 0, 4)
		if ticket.Plan.Client != "" {
			nested = append(nested, goMember{name: "client", text: ticket.Plan.Client})
		}
		if ticket.Plan.ID != "" {
			nested = append(nested, goMember{name: "id", text: ticket.Plan.ID})
		}
		if ticket.Plan.Source != "" {
			nested = append(nested, goMember{name: "source", text: ticket.Plan.Source})
		}
		if ticket.Plan.Local != "" {
			nested = append(nested, goMember{name: "local", text: ticket.Plan.Local})
		}
		members = append(members, goMember{name: "plan", nested: nested, kind: 1})
	}
	if len(ticket.Sessions) > 0 {
		members = append(members, goMember{name: "sessions", strings: ticket.Sessions, kind: 2})
	}
	return renderGoMembers(members, 0)
}

// ➕️AppendSessionID appends a session id once, ignoring blanks and duplicates.
func AppendSessionID(ticket *model.Ticket, sessionID string) {
	model.AppendTicketSessionID(ticket, sessionID)
}

// 🔍️IsTicketInteractionKind reports whether a kind names an operation, with or without its suffix.
func IsTicketInteractionKind(kind, expected string) bool {
	kind = strings.TrimSpace(kind)
	expected = strings.TrimSpace(expected)
	if kind == expected {
		return true
	}
	return strings.HasSuffix(kind, ".ended") && strings.TrimSuffix(kind, ".ended") == expected
}

// #endregion 📄️Codec

// #region 🗄️Store

// 🧱️NodeKind states what a node in the store is.
type NodeKind string

// 📄️NodeFile is a regular file.
const NodeFile NodeKind = "file"

// 🗂️NodeDirectory is a directory.
const NodeDirectory NodeKind = "directory"

// 🔗️NodeSymlink is a symbolic link.
const NodeSymlink NodeKind = "symlink"

// 📏️NodeMetadata is what the store knows about one node.
type NodeMetadata struct {
	Kind NodeKind `json:"kind"`
	Size int64    `json:"size"`
	Mode uint32   `json:"mode"`
}

// 📇️DirectoryEntry is one child of a directory.
type DirectoryEntry struct {
	Name string   `json:"name"`
	Kind NodeKind `json:"kind"`
}

// 🗄️TicketStore is every filesystem touch the ticket domain performs.
type TicketStore interface {
	// 🔍️Metadata returns what is at a path, without following a symlink.
	Metadata(path string) (NodeMetadata, bool)
	// 📖️Read reads a document as text.
	Read(path string) (string, error)
	// ✍️Write writes a document, creating parent directories.
	Write(path string, content string) error
	// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
	CreateExclusive(path string, mode uint32) error
	// 🗂️CreateDirectory creates a directory and every missing ancestor.
	CreateDirectory(path string) error
	// 🗑️RemoveFile removes one file.
	RemoveFile(path string) error
	// 🗑️RemoveDirectory removes one directory, which must be empty.
	RemoveDirectory(path string) error
	// 🧨️RemoveTree removes a whole subtree.
	RemoveTree(path string) error
	// 🚚️Rename moves a node.
	Rename(from, to string) error
	// 📇️Entries lists the children of a directory, sorted by name.
	Entries(path string) ([]DirectoryEntry, error)
}

// ✔️StoreExists reports whether anything is at a path.
func StoreExists(store TicketStore, path string) bool {
	_, found := store.Metadata(path)
	return found
}

// 🗂️StoreIsDirectory reports whether a directory is at a path.
func StoreIsDirectory(store TicketStore, path string) bool {
	metadata, found := store.Metadata(path)
	return found && metadata.Kind == NodeDirectory
}

// 📄️StoreIsFile reports whether a regular file is at a path.
func StoreIsFile(store TicketStore, path string) bool {
	metadata, found := store.Metadata(path)
	return found && metadata.Kind == NodeFile
}

// 🧠️memoryNode is one node of the in-memory store.
type memoryNode struct {
	kind    NodeKind
	content string
	mode    uint32
}

// 🧠️MemoryTicketStore is a whole ticket tree in memory, with failure injection.
type MemoryTicketStore struct {
	nodes         map[string]memoryNode
	writeFailures map[string]string
}

// 🆕️NewMemoryTicketStore builds an empty store.
func NewMemoryTicketStore() *MemoryTicketStore {
	return &MemoryTicketStore{nodes: map[string]memoryNode{}, writeFailures: map[string]string{}}
}

// 💣️FailWrite makes every following write to a path fail with a message.
func (store *MemoryTicketStore) FailWrite(path, message string) {
	store.writeFailures[NormalizeSeparators(path)] = message
}

// 🧹️ClearFailure lifts an injected failure.
func (store *MemoryTicketStore) ClearFailure(path string) {
	delete(store.writeFailures, NormalizeSeparators(path))
}

// 📸️Snapshot returns every file path with its content, in path order.
func (store *MemoryTicketStore) Snapshot() []TicketFile {
	snapshot := make([]TicketFile, 0, len(store.nodes))
	for _, path := range store.Paths() {
		if store.nodes[path].kind == NodeFile {
			snapshot = append(snapshot, TicketFile{Path: path, Content: store.nodes[path].content})
		}
	}
	return snapshot
}

// 📄️TicketFile is one path and content pair of a store snapshot.
type TicketFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 📸️Paths returns every path in the store, files and directories alike, in ascending order.
func (store *MemoryTicketStore) Paths() []string {
	paths := make([]string, 0, len(store.nodes))
	for path := range store.nodes {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	return paths
}

// 🌱️SeedFile seeds one file, creating its ancestors.
func (store *MemoryTicketStore) SeedFile(path, content string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: content, mode: 0o644}
}

// 🌱️SeedSizedFile seeds one file of a size.
func (store *MemoryTicketStore) SeedSizedFile(path string, size int64) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: strings.Repeat("\x00", int(size)), mode: 0o644}
}

// 🌱️SeedDirectory seeds one directory.
func (store *MemoryTicketStore) SeedDirectory(path string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeDirectory, mode: 0o755}
}

// 🔗️SeedSymlink seeds a symlink, which every transaction guard must refuse.
func (store *MemoryTicketStore) SeedSymlink(path string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeSymlink, mode: 0o777}
}

// 🗂️ensureAncestors materialises every missing ancestor directory of a path.
func (store *MemoryTicketStore) ensureAncestors(path string) {
	current := DirName(path)
	pending := make([]string, 0)
	for current != "." && current != "/" && current != "" {
		if _, found := store.nodes[current]; found {
			break
		}
		pending = append(pending, current)
		parent := DirName(current)
		if parent == current {
			break
		}
		current = parent
	}
	for index := len(pending) - 1; index >= 0; index-- {
		store.nodes[pending[index]] = memoryNode{kind: NodeDirectory, mode: 0o755}
	}
}

// 💣️injected returns the injected failure of a path.
func (store *MemoryTicketStore) injected(path string) (string, bool) {
	message, found := store.writeFailures[path]
	return message, found
}

// 🔍️Metadata returns what is at a path.
func (store *MemoryTicketStore) Metadata(path string) (NodeMetadata, bool) {
	path = NormalizeSeparators(path)
	node, found := store.nodes[path]
	if !found {
		return NodeMetadata{}, false
	}
	return NodeMetadata{Kind: node.kind, Size: int64(len(node.content)), Mode: node.mode}, true
}

// 📖️Read reads a document as text.
func (store *MemoryTicketStore) Read(path string) (string, error) {
	path = NormalizeSeparators(path)
	node, found := store.nodes[path]
	if !found {
		return "", errTicketNotFound(path + " does not exist")
	}
	if node.kind != NodeFile {
		return "", errStore(path + " is not a regular file")
	}
	return node.content, nil
}

// ✍️Write writes a document, creating parent directories.
func (store *MemoryTicketStore) Write(path string, content string) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: content, mode: 0o644}
	return nil
}

// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
func (store *MemoryTicketStore) CreateExclusive(path string, mode uint32) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	if _, found := store.nodes[path]; found {
		return errConflict(path + " already exists")
	}
	store.nodes[path] = memoryNode{kind: NodeFile, mode: mode}
	return nil
}

// 🗂️CreateDirectory creates a directory and every missing ancestor.
func (store *MemoryTicketStore) CreateDirectory(path string) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	if existing, found := store.nodes[path]; found {
		if existing.kind == NodeDirectory {
			return nil
		}
		return errConflict(path + " is not a directory")
	}
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeDirectory, mode: 0o755}
	return nil
}

// 🗑️RemoveFile removes one file.
func (store *MemoryTicketStore) RemoveFile(path string) error {
	path = NormalizeSeparators(path)
	if _, found := store.nodes[path]; !found {
		return errTicketNotFound(path + " does not exist")
	}
	delete(store.nodes, path)
	return nil
}

// 🗑️RemoveDirectory removes one directory, which must be empty.
func (store *MemoryTicketStore) RemoveDirectory(path string) error {
	path = NormalizeSeparators(path)
	prefix := path + "/"
	for candidate := range store.nodes {
		if strings.HasPrefix(candidate, prefix) {
			return errConflict(path + " is not empty")
		}
	}
	if _, found := store.nodes[path]; !found {
		return errTicketNotFound(path + " does not exist")
	}
	delete(store.nodes, path)
	return nil
}

// 🧨️RemoveTree removes a whole subtree.
func (store *MemoryTicketStore) RemoveTree(path string) error {
	path = NormalizeSeparators(path)
	prefix := path + "/"
	for candidate := range store.nodes {
		if candidate == path || strings.HasPrefix(candidate, prefix) {
			delete(store.nodes, candidate)
		}
	}
	return nil
}

// 🚚️Rename moves a node and everything below it.
func (store *MemoryTicketStore) Rename(from, to string) error {
	from = NormalizeSeparators(from)
	to = NormalizeSeparators(to)
	if message, found := store.injected(to); found {
		return errStore(message)
	}
	if _, found := store.nodes[from]; !found {
		return errTicketNotFound(from + " does not exist")
	}
	if _, found := store.nodes[to]; found {
		return errConflict(to + " already exists")
	}
	prefix := from + "/"
	moved := make([]string, 0)
	for candidate := range store.nodes {
		if candidate == from || strings.HasPrefix(candidate, prefix) {
			moved = append(moved, candidate)
		}
	}
	sort.Strings(moved)
	store.ensureAncestors(to)
	for _, candidate := range moved {
		node := store.nodes[candidate]
		delete(store.nodes, candidate)
		target := to
		if candidate != from {
			target = to + "/" + candidate[len(prefix):]
		}
		store.nodes[target] = node
	}
	return nil
}

// 📇️Entries lists the children of a directory, sorted by name.
func (store *MemoryTicketStore) Entries(path string) ([]DirectoryEntry, error) {
	path = NormalizeSeparators(path)
	if !StoreIsDirectory(store, path) {
		return nil, errTicketNotFound(path + " is not a directory")
	}
	prefix := path + "/"
	listed := make([]DirectoryEntry, 0)
	for candidate, node := range store.nodes {
		if !strings.HasPrefix(candidate, prefix) {
			continue
		}
		rest := candidate[len(prefix):]
		if strings.Contains(rest, "/") {
			continue
		}
		listed = append(listed, DirectoryEntry{Name: rest, Kind: node.kind})
	}
	sort.SliceStable(listed, func(left, right int) bool { return listed[left].Name < listed[right].Name })
	return listed, nil
}

// 💽️FileTicketStore is the real filesystem.
type FileTicketStore struct{}

// 🆕️NewFileTicketStore builds the store rooted at the machine's filesystem.
func NewFileTicketStore() *FileTicketStore { return &FileTicketStore{} }

// 🔍️Metadata returns what is at a path, without following a symlink.
func (store *FileTicketStore) Metadata(path string) (NodeMetadata, bool) {
	info, err := os.Lstat(path)
	if err != nil {
		return NodeMetadata{}, false
	}
	kind := NodeFile
	switch {
	case info.Mode()&os.ModeSymlink != 0:
		kind = NodeSymlink
	case info.IsDir():
		kind = NodeDirectory
	}
	return NodeMetadata{Kind: kind, Size: info.Size(), Mode: uint32(info.Mode().Perm())}, true
}

// 📖️Read reads a document as text.
func (store *FileTicketStore) Read(path string) (string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return "", errStore(fmt.Sprintf("read %s: %v", path, err))
	}
	return string(data), nil
}

// ✍️Write writes a document, creating parent directories.
func (store *FileTicketStore) Write(path string, content string) error {
	parent := DirName(path)
	if parent != "." && parent != "/" {
		if err := os.MkdirAll(parent, 0o755); err != nil {
			return errStore(fmt.Sprintf("create %s: %v", parent, err))
		}
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		return errStore(fmt.Sprintf("write %s: %v", path, err))
	}
	return nil
}

// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
func (store *FileTicketStore) CreateExclusive(path string, mode uint32) error {
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, os.FileMode(mode))
	if err != nil {
		return errStore(fmt.Sprintf("create %s: %v", path, err))
	}
	return file.Close()
}

// 🗂️CreateDirectory creates a directory and every missing ancestor.
func (store *FileTicketStore) CreateDirectory(path string) error {
	if err := os.MkdirAll(path, 0o755); err != nil {
		return errStore(fmt.Sprintf("create %s: %v", path, err))
	}
	return nil
}

// 🗑️RemoveFile removes one file.
func (store *FileTicketStore) RemoveFile(path string) error {
	if err := os.Remove(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🗑️RemoveDirectory removes one directory, which must be empty.
func (store *FileTicketStore) RemoveDirectory(path string) error {
	if err := os.Remove(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🧨️RemoveTree removes a whole subtree.
func (store *FileTicketStore) RemoveTree(path string) error {
	if err := os.RemoveAll(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🚚️Rename moves a node.
func (store *FileTicketStore) Rename(from, to string) error {
	if err := os.Rename(from, to); err != nil {
		return errStore(fmt.Sprintf("rename %s -> %s: %v", from, to, err))
	}
	return nil
}

// 📇️Entries lists the children of a directory, sorted by name.
func (store *FileTicketStore) Entries(path string) ([]DirectoryEntry, error) {
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, errStore(fmt.Sprintf("read directory %s: %v", path, err))
	}
	listed := make([]DirectoryEntry, 0, len(entries))
	for _, entry := range entries {
		kind := NodeFile
		switch {
		case entry.Type()&os.ModeSymlink != 0:
			kind = NodeSymlink
		case entry.IsDir():
			kind = NodeDirectory
		}
		listed = append(listed, DirectoryEntry{Name: entry.Name(), Kind: kind})
	}
	sort.SliceStable(listed, func(left, right int) bool { return listed[left].Name < listed[right].Name })
	return listed, nil
}

// #endregion 🗄️Store

// #region 💾️Important

// 📓️JournalStep is one recorded step of an important-document transaction.
type JournalStep struct {
	Op      string `json:"op"`
	Path    string `json:"path"`
	Outcome string `json:"outcome"`
}

// 📓️TransactionJournal is the ordered record of every step a transaction attempted.
type TransactionJournal struct {
	Steps []JournalStep `json:"steps"`
}

// ➕️Record records one step.
func (journal *TransactionJournal) Record(op, path, outcome string) {
	journal.Steps = append(journal.Steps, JournalStep{Op: op, Path: path, Outcome: outcome})
}

// 🔤️Trace returns the step sequence as op and outcome pairs.
func (journal TransactionJournal) Trace() []string {
	trace := make([]string, 0, len(journal.Steps))
	for _, step := range journal.Steps {
		trace = append(trace, step.Op+":"+step.Outcome)
	}
	return trace
}

// 🈳️IsEmpty reports whether nothing was recorded.
func (journal TransactionJournal) IsEmpty() bool { return len(journal.Steps) == 0 }

// 🖼️ImportantPreimage is the exact empty-document bundle a close must find and may restore.
type ImportantPreimage struct {
	Path string `json:"path"`
	Dir  string `json:"dir"`
	Mode uint32 `json:"mode"`
}

// ✨️ImportantCreation is what a reopen created, so a failed save removes exactly that.
type ImportantCreation struct {
	Path        string `json:"path"`
	Dir         string `json:"dir"`
	FileCreated bool   `json:"fileCreated"`
	DirCreated  bool   `json:"dirCreated"`
}

// 🔎️InspectImportantDocument requires the canonical important document to be an exact bundle.
func InspectImportantDocument(store TicketStore, path string, journal *TransactionJournal) (ImportantPreimage, error) {
	dir := DirName(path)
	metadata, found := store.Metadata(path)
	if !found {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errTicketNotFound(fmt.Sprintf("cannot finish ticket: required important document %s is unavailable", path))
	}
	if metadata.Kind != NodeFile {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important document %s is not a regular file", path))
	}
	content, err := store.Read(path)
	if err != nil {
		return ImportantPreimage{}, err
	}
	if content != "" {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important document %s is not empty", path))
	}
	entries, err := store.Entries(dir)
	if err != nil {
		return ImportantPreimage{}, err
	}
	expected := BaseName(path)
	if len(entries) != 1 || entries[0].Name != expected {
		journal.Record("inspect", dir, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important directory %s must contain only %s", dir, expected))
	}
	journal.Record("inspect", path, "ok")
	return ImportantPreimage{Path: path, Dir: dir, Mode: metadata.Mode}, nil
}

// 🔄️RestoreImportantDocument restores an empty document without overwriting an existing node.
func RestoreImportantDocument(store TicketStore, preimage ImportantPreimage, journal *TransactionJournal) error {
	if err := store.CreateDirectory(preimage.Dir); err != nil {
		return err
	}
	if err := store.CreateExclusive(preimage.Path, preimage.Mode&0o777); err != nil {
		return err
	}
	journal.Record("restore", preimage.Path, "ok")
	return nil
}

// 🗑️RemoveImportantDocument removes the exact bundle, restoring it when the directory will not go.
func RemoveImportantDocument(store TicketStore, preimage ImportantPreimage, journal *TransactionJournal) error {
	if err := store.RemoveFile(preimage.Path); err != nil {
		return err
	}
	journal.Record("remove-file", preimage.Path, "ok")
	if err := store.RemoveDirectory(preimage.Dir); err != nil {
		journal.Record("remove-dir", preimage.Dir, "failed")
		if restore := RestoreImportantDocument(store, preimage, journal); restore != nil {
			return errStore(fmt.Sprintf("remove important directory: %v; restore important document: %v", err, restore))
		}
		return errStore(fmt.Sprintf("remove important directory: %v", err))
	}
	journal.Record("remove-dir", preimage.Dir, "ok")
	return nil
}

// ✨️EnsureImportantDocument preserves an existing regular document or creates an empty one.
func EnsureImportantDocument(store TicketStore, path string, journal *TransactionJournal) (ImportantCreation, error) {
	dir := DirName(path)
	creation := ImportantCreation{Path: path, Dir: dir}
	if metadata, found := store.Metadata(path); found {
		if metadata.Kind != NodeFile {
			journal.Record("ensure", path, "failed")
			return creation, errInvalid(fmt.Sprintf("important document %s is not a regular file", path))
		}
		if _, err := store.Read(path); err != nil {
			return creation, err
		}
		journal.Record("ensure", path, "preserved")
		return creation, nil
	}
	metadata, found := store.Metadata(dir)
	switch {
	case found && metadata.Kind == NodeDirectory:
	case found:
		journal.Record("ensure-dir", dir, "failed")
		return creation, errInvalid(fmt.Sprintf("important directory %s is not a physical directory", dir))
	default:
		if err := store.CreateDirectory(dir); err != nil {
			return creation, err
		}
		creation.DirCreated = true
		journal.Record("ensure-dir", dir, "created")
	}
	if err := store.CreateExclusive(path, 0o644); err != nil {
		if creation.DirCreated {
			_ = store.RemoveDirectory(dir)
		}
		journal.Record("ensure", path, "failed")
		return creation, err
	}
	creation.FileCreated = true
	journal.Record("ensure", path, "created")
	return creation, nil
}

// ↩️RollbackImportantCreation removes only the nodes the current reopen created.
func RollbackImportantCreation(store TicketStore, creation ImportantCreation, journal *TransactionJournal) error {
	if creation.FileCreated && StoreExists(store, creation.Path) {
		if err := store.RemoveFile(creation.Path); err != nil {
			return err
		}
		journal.Record("rollback-file", creation.Path, "ok")
	}
	if creation.DirCreated && StoreExists(store, creation.Dir) {
		if err := store.RemoveDirectory(creation.Dir); err != nil {
			return err
		}
		journal.Record("rollback-dir", creation.Dir, "ok")
	}
	return nil
}

// #endregion 💾️Important

// #region 🗺️PlanSource

// 🗺️PlanSource is where a plan or spec id resolves to.
type PlanSource struct {
	Path        string `json:"path"`
	IsDirectory bool   `json:"isDirectory"`
}

// 🏠️PlanRoots are the two roots plan resolution reads.
type PlanRoots struct {
	RepoRoot string `json:"repoRoot"`
	HomeDir  string `json:"homeDir"`
}

// 🏷️PlanClientTag returns the value stored for the plan attachment of a client.
func PlanClientTag(kind providers.McpClientKind) string {
	switch kind {
	case providers.McpClientCursor:
		return "cursor"
	case providers.McpClientKiro:
		return "kiro"
	case providers.McpClientCopilot:
		return "copilot"
	case providers.McpClientClaude:
		return "claude"
	case providers.McpClientCodex:
		return "codex"
	}
	return "generic"
}

// 🏠️HostPlanRoots are the plan roots of the repository and the user the process runs as.
func HostPlanRoots() PlanRoots {
	home, _ := os.UserHomeDir()
	return PlanRoots{RepoRoot: workspace.RootDir, HomeDir: home}
}

// 🗺️ResolvePlanSource resolves a plan or spec id to a store path, per IDE.
func ResolvePlanSource(store TicketStore, roots PlanRoots, kind providers.McpClientKind, id string) (PlanSource, error) {
	id = strings.TrimSpace(id)
	if id == "" {
		return PlanSource{}, errInvalid("plan or spec id is empty")
	}
	root := strings.TrimSpace(roots.RepoRoot)
	if root == "" {
		return PlanSource{}, errInvalid("repository root is not set")
	}
	switch kind {
	case providers.McpClientCursor:
		directory := JoinPath(root, ".cursor/plans")
		suffix := "_" + id + ".plan.md"
		entries, _ := store.Entries(directory)
		matches := make([]string, 0)
		for _, entry := range entries {
			if entry.Kind == NodeFile && strings.HasSuffix(entry.Name, suffix) {
				matches = append(matches, JoinPath(directory, entry.Name))
			}
		}
		switch len(matches) {
		case 0:
			return PlanSource{}, errTicketNotFound(fmt.Sprintf("no Cursor plan matches id %q (glob %s/*%s)", id, directory, suffix))
		case 1:
			return PlanSource{Path: matches[0]}, nil
		}
		return PlanSource{}, errConflict(fmt.Sprintf("ambiguous Cursor plan id %q: %d matches", id, len(matches)))
	case providers.McpClientKiro:
		directory := JoinPath(root, ".kiro/specs/"+id)
		metadata, found := store.Metadata(directory)
		switch {
		case found && metadata.Kind == NodeDirectory:
			return PlanSource{Path: directory, IsDirectory: true}, nil
		case found:
			return PlanSource{}, errInvalid(fmt.Sprintf("Kiro spec %q is not a directory", id))
		}
		return PlanSource{}, errTicketNotFound(fmt.Sprintf("Kiro spec %q: no such directory", id))
	case providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		home := strings.TrimSpace(roots.HomeDir)
		if home == "" {
			return PlanSource{}, errInvalid("home directory is not set")
		}
		repoBase := BaseName(root)
		path := JoinPath(home, ".codex/memory/"+repoBase+"/"+id+".md")
		switch kind {
		case providers.McpClientCopilot:
			path = JoinPath(home, ".copilot/projects/"+repoBase+"/memory/"+id+".md")
		case providers.McpClientClaude:
			path = JoinPath(home, ".claude/plans/"+id+".md")
		}
		metadata, found := store.Metadata(path)
		switch {
		case found && metadata.Kind == NodeDirectory:
			return PlanSource{}, errInvalid("expected file at " + path)
		case found:
			return PlanSource{Path: path}, nil
		}
		return PlanSource{}, errTicketNotFound(fmt.Sprintf("plan file for id %q: no such file", id))
	}
	return PlanSource{}, errInvalid(fmt.Sprintf("plan or spec attachment is not supported for mcp kind %q", string(kind)))
}

// 📎️ApplyTicketPlanFromIDs resolves a plan or spec id and attaches it to a ticket before it is saved.
func ApplyTicketPlanFromIDs(store TicketStore, roots PlanRoots, ticket *model.Ticket, kind providers.McpClientKind, planID, specID string) error {
	planID = strings.TrimSpace(planID)
	specID = strings.TrimSpace(specID)
	if planID == "" && specID == "" {
		return nil
	}
	if planID != "" && specID != "" {
		return errInvalid("pass only one of plan_id or spec_id")
	}
	id := planID
	if kind == providers.McpClientKiro {
		if planID != "" {
			return errInvalid("use spec_id for Kiro, not plan_id")
		}
		id = specID
	} else if specID != "" {
		return errInvalid("use plan_id for this client, not spec_id")
	}
	source, err := ResolvePlanSource(store, roots, kind, id)
	if err != nil {
		return err
	}
	ticket.Plan = &model.TicketPlan{Client: PlanClientTag(kind), ID: id, Source: source.Path}
	return nil
}

// 📦️MovePlanIntoFolderIn moves an attached plan or spec into the ticket folder as it closes.
func MovePlanIntoFolderIn(store TicketStore, ticket *model.Ticket) error {
	if ticket.Plan == nil || strings.TrimSpace(ticket.Plan.Source) == "" {
		return nil
	}
	if ticket.FolderPath == "" {
		return errInvalid("ticket folder path is empty")
	}
	source := NormalizeSeparators(ticket.Plan.Source)
	destinationName := BaseName(source)
	destination := JoinPath(ticket.FolderPath, destinationName)
	if !StoreExists(store, source) {
		if StoreExists(store, destination) {
			ticket.Plan.Local = destinationName
			ticket.Plan.Source = ""
			return nil
		}
		return errTicketNotFound(fmt.Sprintf("plan source %q missing and destination %q not found", source, destination))
	}
	if err := store.Rename(source, destination); err != nil {
		return err
	}
	ticket.Plan.Local = destinationName
	ticket.Plan.Source = ""
	return nil
}

// 📝️StripPlanFrontmatter strips YAML frontmatter from plan markdown.
func StripPlanFrontmatter(content string) string {
	if !strings.HasPrefix(content, "---") {
		return strings.TrimSpace(content)
	}
	end := strings.Index(content[3:], "\n---")
	if end < 0 {
		return strings.TrimSpace(content)
	}
	rest := content[3+end+4:]
	switch {
	case strings.HasPrefix(rest, "\r\n"):
		rest = rest[2:]
	case strings.HasPrefix(rest, "\n"):
		rest = rest[1:]
	}
	return strings.TrimSpace(rest)
}

// 📝️FormatPlanFileSection formats one plan file as a collapsible markdown section.
func FormatPlanFileSection(name, raw string) string {
	body := StripPlanFrontmatter(raw)
	if strings.TrimSpace(body) == "" {
		return ""
	}
	return "<details>\n<summary>" + name + "</summary>\n\n" + body + "\n\n</details>"
}

// #endregion 🗺️PlanSource

// #region 📐️FileScope

// 📏️DiffLines is how many lines one file gained and lost.
type DiffLines struct {
	Added   int `json:"added"`
	Removed int `json:"removed"`
}

// 🔢️GitIndexRef is the reference a ticket diff is taken against.
const GitIndexRef = ":0"

// 🔧️VersionControl is everything the ticket domain asks of version control.
type VersionControl interface {
	// 🙈️IsIgnored reports whether a path is ignored.
	IsIgnored(path string) bool
	// 📇️NameStatus returns the name-status output of a diff.
	NameStatus(ref, against string, files []string) (string, error)
	// 📄️UnifiedDiff returns the unified diff output.
	UnifiedDiff(ref, against string, files []string) (string, error)
}

// 📼️RecordedVersionControl answers from a frozen recording.
type RecordedVersionControl struct {
	Ignored        []string `json:"ignored"`
	NameStatusOut  string   `json:"nameStatus"`
	UnifiedDiffOut string   `json:"unifiedDiff"`
}

// 🙈️IsIgnored reports whether a path was recorded as ignored.
func (control RecordedVersionControl) IsIgnored(path string) bool {
	for _, candidate := range control.Ignored {
		if candidate == path {
			return true
		}
	}
	return false
}

// 📇️NameStatus returns the recorded name-status output.
func (control RecordedVersionControl) NameStatus(string, string, []string) (string, error) {
	return control.NameStatusOut, nil
}

// 📄️UnifiedDiff returns the recorded unified diff output.
func (control RecordedVersionControl) UnifiedDiff(string, string, []string) (string, error) {
	return control.UnifiedDiffOut, nil
}

// 🧹️IsRepoExcludedPath reports whether a path is excluded from every ticket file scope.
func IsRepoExcludedPath(path string) bool {
	normalized := strings.TrimPrefix(NormalizeSeparators(strings.TrimSpace(path)), "./")
	if normalized == "" {
		return false
	}
	under := func(root string) bool {
		return normalized == root || strings.HasPrefix(normalized, root+"/")
	}
	if under(".🧬semio") || under("assets/repo") || strings.Contains(normalized, "/asset/repo/") {
		return true
	}
	if under("node_modules") || strings.Contains(normalized, "/node_modules/") {
		return true
	}
	if under(".git") || strings.Contains(normalized, "/.git/") {
		return true
	}
	for _, segment := range []string{"/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"} {
		if strings.Contains(normalized, segment) {
			return true
		}
	}
	return strings.HasSuffix(BaseName(normalized), ".Designer.cs") || strings.Contains(normalized, "/codegen/")
}

// 📝️NormalizeTicketFileInput normalises one file identifier into a repository-relative path.
func NormalizeTicketFileInput(filePath string) string {
	normalized := strings.TrimSpace(filePath)
	if normalized == "" {
		return ""
	}
	if rest, found := strings.CutPrefix(normalized, "file://"); found {
		return strings.TrimPrefix(NormalizeSeparators(rest), "./")
	}
	return strings.TrimPrefix(NormalizeSeparators(normalized), "./")
}

// 💿️NormalizeTicketFileInputs normalises and de-duplicates file identifiers, preserving order.
func NormalizeTicketFileInputs(files []string) []string {
	seen := map[string]bool{}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizeTicketFileInput(filePath)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		filtered = append(filtered, normalized)
	}
	return filtered
}

// 🗺️FilterTicketWorkspaceFiles drops the files that live inside a ticket's own folder.
func FilterTicketWorkspaceFiles(folderPath string, files []string) []string {
	relative := strings.TrimPrefix(NormalizeSeparators(folderPath), "./")
	if relative == "" {
		return files
	}
	kept := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := strings.TrimPrefix(NormalizeSeparators(filePath), "./")
		if normalized == relative || strings.HasPrefix(normalized, relative+"/") {
			continue
		}
		kept = append(kept, filePath)
	}
	return kept
}

// 📐️TicketFileScope is the file scope a ticket closes with.
type TicketFileScope struct {
	Files     []string             `json:"files"`
	Excluded  []string             `json:"excluded"`
	DiffLines map[string]DiffLines `json:"diffLines"`
	Statuses  []string             `json:"statuses"`
}

// 🎫️ComputeTicketFileScope computes the file scope of a ticket close through version control.
func ComputeTicketFileScope(control VersionControl, folderPath string, files []string) (TicketFileScope, error) {
	normalized := NormalizeTicketFileInputs(files)
	workspaceFiles := FilterTicketWorkspaceFiles(folderPath, normalized)
	kept := make([]string, 0, len(workspaceFiles))
	excluded := make([]string, 0)
	for _, filePath := range workspaceFiles {
		if IsRepoExcludedPath(filePath) || control.IsIgnored(filePath) {
			excluded = append(excluded, filePath)
			continue
		}
		kept = append(kept, filePath)
	}
	if len(kept) == 0 {
		return TicketFileScope{}, errInvalid("at least one file is required")
	}
	nameStatus, err := control.NameStatus(GitIndexRef, "", kept)
	if err != nil {
		return TicketFileScope{}, err
	}
	statuses := make([]string, 0)
	for _, line := range strings.Split(nameStatus, "\n") {
		if strings.TrimSpace(line) != "" {
			statuses = append(statuses, line)
		}
	}
	diff, err := control.UnifiedDiff(GitIndexRef, "", kept)
	if err != nil {
		return TicketFileScope{}, err
	}
	return TicketFileScope{Files: kept, Excluded: excluded, DiffLines: ParseDiffLines(diff), Statuses: statuses}, nil
}

// 📏️ParseDiffLines counts the added and removed lines of a unified diff, per file.
func ParseDiffLines(stdout string) map[string]DiffLines {
	counted := map[string]DiffLines{}
	current := ""
	for _, line := range strings.Split(stdout, "\n") {
		switch {
		case strings.HasPrefix(line, "+++ b/"):
			current = strings.TrimPrefix(line, "+++ b/")
			if _, found := counted[current]; !found {
				counted[current] = DiffLines{}
			}
		case strings.HasPrefix(line, "--- a/") || strings.HasPrefix(line, "diff --git ") || strings.HasPrefix(line, "@@"):
		case current == "":
		case strings.HasPrefix(line, "+"):
			entry := counted[current]
			entry.Added++
			counted[current] = entry
		case strings.HasPrefix(line, "-"):
			entry := counted[current]
			entry.Removed++
			counted[current] = entry
		}
	}
	return counted
}

// 📪️CanCloseTicket reports whether a ticket may be closed, with the reasons it may not.
func CanCloseTicket(ticket *model.Ticket) (bool, []string) {
	if ticket == nil {
		return false, []string{"Ticket data is nil"}
	}
	return true, nil
}

// #endregion 📐️FileScope

// #region 🐙️IssueSync

// 🤖️FormatPromptHeading returns the prompt section heading of an issue body or comment.
func FormatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖️ Prompt"
	}
	return "# 🤖️ Prompt\n\n" + body
}

// 🔍️FormatSummaryHeading returns the summary section heading of a closing comment.
func FormatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍️ Summary"
	}
	return "# 🔍️ Summary\n\n" + body
}

// 🐙️EnsureTicketIssue creates, links or reopens the issue of a ticket, returning the warnings.
func EnsureTicketIssue(tracker IssueTracker, ticket *model.Ticket, title, prompt, issue string, milestone *int, reopenIfClosed bool) []string {
	warnings := make([]string, 0)
	issue = strings.TrimSpace(issue)
	if issue != "" {
		ticket.Management = &model.TicketManagementData{Issue: issue}
		return warnings
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		if reopenIfClosed {
			remote, err := tracker.GetIssueDetails(ticket.Management.Issue)
			switch {
			case err != nil:
				warnings = append(warnings, "read github issue: "+err.Error())
			case remote != nil && strings.EqualFold(remote.State, "closed"):
				if err := tracker.ReopenIssue(ticket.Management.Issue); err != nil {
					warnings = append(warnings, "reopen github issue: "+err.Error())
				}
			}
		}
		return warnings
	}
	url, err := tracker.CreateIssue(title, FormatPromptHeading(prompt), milestone)
	switch {
	case err != nil:
		warnings = append(warnings, "Failed to create GitHub issue: "+err.Error())
	case strings.TrimSpace(url) == "":
		warnings = append(warnings, "github issue create returned empty url")
	default:
		ticket.Management = &model.TicketManagementData{Issue: url}
	}
	return warnings
}

// 📪️CloseTicketIssue comments the summary, labels the issue with the touched bundles and closes it.
func CloseTicketIssue(tracker IssueTracker, ticket *model.Ticket, summary string, labels []string, bulk bool) []string {
	warnings := make([]string, 0)
	if ticket.Management == nil || ticket.Management.Issue == "" {
		return warnings
	}
	issueURL := ticket.Management.Issue
	if !bulk {
		if len(labels) > 0 {
			if err := tracker.AddLabels(issueURL, labels); err != nil {
				warnings = append(warnings, "Failed to add labels to GitHub issue: "+err.Error())
			}
		}
		if err := tracker.AddComment(issueURL, FormatSummaryHeading(summary)); err != nil {
			warnings = append(warnings, "Failed to add summary and metrics comment to GitHub issue: "+err.Error())
		}
	}
	if err := tracker.CloseIssue(issueURL); err != nil {
		warnings = append(warnings, "Failed to close GitHub issue: "+err.Error())
	}
	return warnings
}

// 🎯️MilestoneNumberForTitle looks a milestone number up by title, swallowing a lookup failure.
func MilestoneNumberForTitle(tracker IssueTracker, title string) *int {
	milestone, err := tracker.FindMilestoneByTitle(title)
	if err != nil || milestone == nil {
		return nil
	}
	number := milestone.Number
	return &number
}

// 🎞️IssueTrackerScript is what a recorded issue tracker answers.
type IssueTrackerScript struct {
	CreatedIssueURL string   `json:"createdIssueUrl"`
	IssueState      string   `json:"issueState"`
	Milestone       *int     `json:"milestone"`
	MilestoneTitle  string   `json:"milestoneTitle"`
	Failures        []string `json:"failures"`
}

// 🎞️RecordedIssueTracker is a fixture-driven issue tracker that records every interaction.
type RecordedIssueTracker struct {
	script IssueTrackerScript
	calls  []string
}

// 🆕️NewRecordedIssueTracker binds a tracker to a script.
func NewRecordedIssueTracker(script IssueTrackerScript) *RecordedIssueTracker {
	return &RecordedIssueTracker{script: script}
}

// 📥️RecordedIssueTrackerFromJSON reads a script from JSON text.
func RecordedIssueTrackerFromJSON(text string) (*RecordedIssueTracker, error) {
	var script IssueTrackerScript
	if err := json.Unmarshal([]byte(text), &script); err != nil {
		return nil, errInvalid(err.Error())
	}
	return NewRecordedIssueTracker(script), nil
}

// 📼️Calls returns every interaction so far, in order.
func (tracker *RecordedIssueTracker) Calls() []string {
	return append([]string{}, tracker.calls...)
}

// ✍️record appends one interaction.
func (tracker *RecordedIssueTracker) record(call string) { tracker.calls = append(tracker.calls, call) }

// 💣️fails returns the scripted failure of a method.
func (tracker *RecordedIssueTracker) fails(method string) error {
	for _, candidate := range tracker.script.Failures {
		if candidate == method {
			return fmt.Errorf("%s failed", method)
		}
	}
	return nil
}

// 🆕️CreateIssue records the call and answers the scripted URL.
func (tracker *RecordedIssueTracker) CreateIssue(title, body string, milestone *int) (string, error) {
	reference := ""
	if milestone != nil {
		reference = strconv.Itoa(*milestone)
	}
	tracker.record(fmt.Sprintf("create_issue|%s|%s|%s", title, body, reference))
	if err := tracker.fails("create_issue"); err != nil {
		return "", err
	}
	return tracker.script.CreatedIssueURL, nil
}

// 📪️CloseIssue records the call.
func (tracker *RecordedIssueTracker) CloseIssue(issueURL string) error {
	tracker.record("close_issue|" + issueURL)
	return tracker.fails("close_issue")
}

// 🔓️ReopenIssue records the call.
func (tracker *RecordedIssueTracker) ReopenIssue(issueURL string) error {
	tracker.record("reopen_issue|" + issueURL)
	return tracker.fails("reopen_issue")
}

// 🔎️GetIssueDetails records the call and answers the scripted state.
func (tracker *RecordedIssueTracker) GetIssueDetails(issueURL string) (*ManagementIssue, error) {
	tracker.record("get_issue_details|" + issueURL)
	if err := tracker.fails("get_issue_details"); err != nil {
		return nil, err
	}
	if tracker.script.IssueState == "" {
		return nil, nil
	}
	return &ManagementIssue{URL: issueURL, State: tracker.script.IssueState}, nil
}

// 💬️AddComment records the call.
func (tracker *RecordedIssueTracker) AddComment(issueURL, comment string) error {
	tracker.record(fmt.Sprintf("add_comment|%s|%s", issueURL, comment))
	return tracker.fails("add_comment")
}

// 🏷️AddLabels records the call.
func (tracker *RecordedIssueTracker) AddLabels(issueURL string, labels []string) error {
	tracker.record(fmt.Sprintf("add_labels|%s|%s", issueURL, strings.Join(labels, ",")))
	return tracker.fails("add_labels")
}

// 🏷️RemoveLabels records the call.
func (tracker *RecordedIssueTracker) RemoveLabels(issueURL string, labels []string) error {
	tracker.record(fmt.Sprintf("remove_labels|%s|%s", issueURL, strings.Join(labels, ",")))
	return tracker.fails("remove_labels")
}

// 🎯️FindMilestoneByTitle records the call and answers the scripted milestone.
func (tracker *RecordedIssueTracker) FindMilestoneByTitle(title string) (*ManagementMilestone, error) {
	tracker.record("find_milestone_by_title|" + title)
	if err := tracker.fails("find_milestone_by_title"); err != nil {
		return nil, err
	}
	if tracker.script.Milestone == nil {
		return nil, nil
	}
	return &ManagementMilestone{Number: *tracker.script.Milestone, Title: tracker.script.MilestoneTitle}, nil
}

// 🏷️ListRepoLabels records the call.
func (tracker *RecordedIssueTracker) ListRepoLabels() ([]ManagementLabel, error) {
	tracker.record("list_repo_labels")
	if err := tracker.fails("list_repo_labels"); err != nil {
		return nil, err
	}
	return nil, nil
}

// 🚫️NullIssueTracker is the tracker that answers nothing, for a lifecycle run without management.
type NullIssueTracker struct{}

// 🆕️CreateIssue answers no URL.
func (NullIssueTracker) CreateIssue(string, string, *int) (string, error) { return "", nil }

// 📪️CloseIssue does nothing.
func (NullIssueTracker) CloseIssue(string) error { return nil }

// 🔓️ReopenIssue does nothing.
func (NullIssueTracker) ReopenIssue(string) error { return nil }

// 🔎️GetIssueDetails answers no issue.
func (NullIssueTracker) GetIssueDetails(string) (*ManagementIssue, error) { return nil, nil }

// 💬️AddComment does nothing.
func (NullIssueTracker) AddComment(string, string) error { return nil }

// 🏷️AddLabels does nothing.
func (NullIssueTracker) AddLabels(string, []string) error { return nil }

// 🏷️RemoveLabels does nothing.
func (NullIssueTracker) RemoveLabels(string, []string) error { return nil }

// 🎯️FindMilestoneByTitle answers no milestone.
func (NullIssueTracker) FindMilestoneByTitle(string) (*ManagementMilestone, error) { return nil, nil }

// 🏷️ListRepoLabels answers no labels.
func (NullIssueTracker) ListRepoLabels() ([]ManagementLabel, error) { return nil, nil }

// 🐙️IssueSyncOutcome is what one issue synchronisation left behind.
type IssueSyncOutcome struct {
	Issue    string   `json:"issue"`
	Warnings []string `json:"warnings"`
}

// 🐙️SyncOpenIssue synchronises the issue of a ticket that is being opened or reopened.
func SyncOpenIssue(tracker IssueTracker, existingIssue, title, prompt, issue string, milestone *int, reopenIfClosed bool) IssueSyncOutcome {
	ticket := &model.Ticket{Title: title, Status: model.TicketStatusOpen}
	if existingIssue != "" {
		ticket.Management = &model.TicketManagementData{Issue: existingIssue}
	}
	warnings := EnsureTicketIssue(tracker, ticket, title, prompt, issue, milestone, reopenIfClosed)
	link := ""
	if ticket.Management != nil {
		link = ticket.Management.Issue
	}
	return IssueSyncOutcome{Issue: link, Warnings: warnings}
}

// 📪️SyncCloseIssue synchronises the issue of a ticket that is being closed.
func SyncCloseIssue(tracker IssueTracker, issueURL, summary string, labels []string, bulk bool) []string {
	ticket := &model.Ticket{Status: model.TicketStatusClosed}
	if issueURL != "" {
		ticket.Management = &model.TicketManagementData{Issue: issueURL}
	}
	return CloseTicketIssue(tracker, ticket, summary, labels, bulk)
}

// #endregion 🐙️IssueSync

// #region ⏰️Ports

// ⏰️TicketClock is the clock the lifecycle reads, so a case pins every timestamp.
type TicketClock interface {
	// 📅️Today returns the two-digit year, month and day the folder scheme uses.
	Today() (int, int, int)
	// 🕰️Stamp returns the interaction timestamp.
	Stamp() string
}

// 📌️FixedTicketClock is a clock frozen at one instant.
type FixedTicketClock struct {
	Year      int
	Month     int
	Day       int
	Timestamp string
}

// 🆕️NewFixedTicketClock freezes a clock at a date and a timestamp.
func NewFixedTicketClock(year, month, day int, timestamp string) FixedTicketClock {
	return FixedTicketClock{Year: year, Month: month, Day: day, Timestamp: timestamp}
}

// 📅️Today returns the frozen date.
func (clock FixedTicketClock) Today() (int, int, int) { return clock.Year, clock.Month, clock.Day }

// 🕰️Stamp returns the frozen timestamp.
func (clock FixedTicketClock) Stamp() string { return clock.Timestamp }

// 🕰️SystemTicketClock is the clock of the machine this runs on.
type SystemTicketClock struct{}

// 📅️Today returns today as a two-digit year, month and day.
func (SystemTicketClock) Today() (int, int, int) {
	now := time.Now()
	return now.Year() % 100, int(now.Month()), now.Day()
}

// 🕰️Stamp returns the current timestamp.
func (SystemTicketClock) Stamp() string { return time.Now().Format("2006-01-02 15:04:05") }

// 📡️RecordedTicketEvent is one emitted event: the kind, the source and the payload as JSON text.
type RecordedTicketEvent struct {
	Kind    string `json:"kind"`
	Source  string `json:"source"`
	Payload string `json:"payload"`
}

// 📡️EventSink is where the lifecycle sends its events.
type EventSink interface {
	// 📤️Emit emits one event, its payload already encoded.
	Emit(kind, source, payload string)
}

// 📼️RecordingEventSink keeps every event in emission order.
type RecordingEventSink struct {
	events []RecordedTicketEvent
}

// 🆕️NewRecordingEventSink builds an empty sink.
func NewRecordingEventSink() *RecordingEventSink { return &RecordingEventSink{} }

// 📼️Recorded returns everything emitted so far, in order.
func (sink *RecordingEventSink) Recorded() []RecordedTicketEvent {
	return append([]RecordedTicketEvent{}, sink.events...)
}

// 📤️Emit records one event.
func (sink *RecordingEventSink) Emit(kind, source, payload string) {
	sink.events = append(sink.events, RecordedTicketEvent{Kind: kind, Source: source, Payload: payload})
}

// 🌐️CoordinatorEventSink posts to the repository coordinator through 📡️events.
type CoordinatorEventSink struct{}

// 📤️Emit posts one event to the coordinator.
func (CoordinatorEventSink) Emit(kind, source, payload string) {
	var value interface{}
	if err := json.Unmarshal([]byte(payload), &value); err != nil {
		return
	}
	events.Emit(events.EventKind(kind), source, value)
}

// 🏷️TicketEventSource is the event source every ticket operation carries.
const TicketEventSource = "repo-cli"

// #endregion ⏰️Ports

// #region 🔓️Lifecycle

// 📬️TicketOpenRequest is everything an open request carries.
type TicketOpenRequest struct {
	Emoji        string `json:"emoji"`
	Title        string `json:"title"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort"`
	Client       string `json:"client"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent"`
	NoIssue      bool   `json:"noIssue"`
	NoManagement bool   `json:"noManagement"`
	Issue        string `json:"issue"`
	Session      string `json:"session"`
	PlanID       string `json:"planId"`
	SpecID       string `json:"specId"`
}

// 📪️TicketCloseRequest is everything a close request carries.
type TicketCloseRequest struct {
	ID           string   `json:"id"`
	Summary      string   `json:"summary"`
	Files        []string `json:"files"`
	NoManagement bool     `json:"noManagement"`
	Bulk         bool     `json:"bulk"`
}

// 🔓️TicketReopenRequest is everything a reopen request carries.
type TicketReopenRequest struct {
	ID           string `json:"id"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort"`
	Client       string `json:"client"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent"`
	NoManagement bool   `json:"noManagement"`
	Session      string `json:"session"`
	PlanID       string `json:"planId"`
	SpecID       string `json:"specId"`
}

// ♻️TicketChangeRequest is everything a change request carries.
type TicketChangeRequest struct {
	ID           string  `json:"id"`
	Title        *string `json:"title"`
	Description  *string `json:"description"`
	Goal         *string `json:"goal"`
	Parent       *string `json:"parent"`
	NoManagement bool    `json:"noManagement"`
}

// 🎫️TicketOutcome is the outcome of one lifecycle operation.
type TicketOutcome struct {
	ID       string             `json:"id"`
	RelPath  string             `json:"relPath"`
	Status   string             `json:"status"`
	Document string             `json:"document"`
	Journal  TransactionJournal `json:"journal"`
	Warnings []string           `json:"warnings"`
}

// 🧹️PurgeReport is what an artifact purge removed.
type PurgeReport struct {
	RemovedDirectories []string `json:"removedDirectories"`
	RemovedFiles       []string `json:"removedFiles"`
}

// 🧹️OversizedFileBytes is the size above which a file is purged from a closed ticket folder.
const OversizedFileBytes int64 = 5 << 20

// 🧹️OversizedFolderBytes is the size above which a subfolder is purged.
const OversizedFolderBytes int64 = 10 << 20

// 🎫️TicketService is the ticket domain bound to one store, tracker, clock and event sink.
type TicketService struct {
	Layout    TicketLayout
	Store     TicketStore
	Tracker   IssueTracker
	Clock     TicketClock
	Events    EventSink
	Roots     PlanRoots
	McpClient providers.McpClientKind
}

// 🆕️NewTicketService binds the domain to its ports.
func NewTicketService(layout TicketLayout, store TicketStore, tracker IssueTracker, clock TicketClock, sink EventSink) *TicketService {
	return &TicketService{Layout: layout, Store: store, Tracker: tracker, Clock: clock, Events: sink, McpClient: providers.McpClientGeneric}
}

// 🗺️WithRoots sets the roots plan resolution reads.
func (service *TicketService) WithRoots(roots PlanRoots) *TicketService {
	service.Roots = roots
	return service
}

// 🪪️WithMcpClient sets the MCP surface a plan or spec id is resolved against.
func (service *TicketService) WithMcpClient(kind providers.McpClientKind) *TicketService {
	service.McpClient = kind
	return service
}

// 📖️Read reads one ticket, filling in every derived path.
func (service *TicketService) Read(id TicketID) (*model.Ticket, error) {
	documentPath := service.Layout.DocumentPath(id)
	if !StoreExists(service.Store, documentPath) {
		return nil, errTicketNotFound("ticket not found: " + documentPath)
	}
	document, err := service.Store.Read(documentPath)
	if err != nil {
		return nil, err
	}
	ticket, err := DecodeTicketDocument(document)
	if err != nil {
		return nil, err
	}
	service.hydrate(ticket, id)
	return ticket, nil
}

// 💾️Save writes one ticket back.
func (service *TicketService) Save(ticket *model.Ticket) error {
	if ticket.JsonPath == "" {
		return errInvalid("ticket json path is empty")
	}
	return service.Store.Write(ticket.JsonPath, EncodeTicketDocument(ticket))
}

// ▪️List returns every ticket, optionally narrowed to a year, a month and a day.
func (service *TicketService) List(year, month, day *int) ([]*model.Ticket, error) {
	ticketsDir := service.Layout.TicketsDir()
	if !StoreExists(service.Store, ticketsDir) {
		return nil, nil
	}
	found := make([]*model.Ticket, 0)
	for _, yearValue := range service.datedChildren(ticketsDir, EmojiYear, year) {
		yearPath := JoinPath(ticketsDir, FormatYearDir(yearValue))
		for _, monthValue := range service.datedChildren(yearPath, EmojiMonth, month) {
			monthPath := JoinPath(yearPath, FormatMonthDir(monthValue))
			for _, dayValue := range service.datedChildren(monthPath, EmojiDay, day) {
				dayPath := JoinPath(monthPath, FormatDayDir(dayValue))
				for _, slug := range service.slugsUnder(dayPath, "") {
					if ticket, err := service.Read(NewTicketID(yearValue, monthValue, dayValue, slug)); err == nil {
						found = append(found, ticket)
					}
				}
			}
		}
	}
	return found, nil
}

// 🔎️FindBySlug returns the most recently created ticket whose slug matches.
func (service *TicketService) FindBySlug(slug string) (*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	for index := len(tickets) - 1; index >= 0; index-- {
		if tickets[index].Slug == slug || BaseName(tickets[index].Slug) == slug {
			return tickets[index], nil
		}
	}
	return nil, errTicketNotFound("ticket not found: " + slug)
}

// 🕰️Latest returns the newest ticket by date, then by slug.
func (service *TicketService) Latest() (*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, errTicketNotFound("no tickets found")
	}
	sort.SliceStable(tickets, func(left, right int) bool {
		first, second := tickets[left], tickets[right]
		if first.Year != second.Year {
			return first.Year > second.Year
		}
		if first.Month != second.Month {
			return first.Month > second.Month
		}
		if first.Day != second.Day {
			return first.Day > second.Day
		}
		return first.Slug > second.Slug
	})
	return tickets[0], nil
}

// 📬️Open opens a ticket: validates, materialises the folder, syncs the issue and emits.
func (service *TicketService) Open(request TicketOpenRequest) (TicketOutcome, error) {
	if strings.TrimSpace(request.Goal) == "" {
		return TicketOutcome{}, errInvalid("ticket goal is required")
	}
	slug, err := ValidateTicketEmojiTitle(request.Emoji, request.Title)
	if err != nil {
		return TicketOutcome{}, err
	}
	llm, err := resolveOptionalSlug(request.LLM, model.ResolveAllowedLLM)
	if err != nil {
		return TicketOutcome{}, err
	}
	effort, err := resolveOptionalSlug(request.Effort, model.ResolveAllowedEffort)
	if err != nil {
		return TicketOutcome{}, err
	}
	client, err := model.ResolveAllowedClient(request.Client)
	if err != nil {
		return TicketOutcome{}, errInvalid(err.Error())
	}
	year, month, day := service.Clock.Today()
	if strings.TrimSpace(request.Parent) != "" {
		parent, err := service.FindBySlug(strings.TrimSpace(request.Parent))
		if err != nil {
			return TicketOutcome{}, err
		}
		year, month, day = parent.Year, parent.Month, parent.Day
		slug = parent.Slug + "/" + slug
	}
	id := NewTicketID(year, month, day, slug)
	ticketDir := service.Layout.TicketDir(id)
	if StoreExists(service.Store, ticketDir) {
		return TicketOutcome{}, errConflict("ticket folder already exists: " + ticketDir)
	}
	if err := service.Store.CreateDirectory(ticketDir); err != nil {
		return TicketOutcome{}, err
	}
	journal := TransactionJournal{}
	if err := service.Store.Write(service.Layout.ImportantPath(id), ""); err != nil {
		return TicketOutcome{}, err
	}
	journal.Record("create-important", service.Layout.ImportantPath(id), "ok")

	ticket := &model.Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        id.Slug,
		Title:       strings.TrimSpace(request.Title),
		Emoji:       strings.TrimSpace(request.Emoji),
		Status:      model.TicketStatusOpen,
		Description: request.Prompt,
		Goal:        request.Goal,
		Parent:      request.Parent,
		Interactions: []model.Interaction{{
			Kind:   "ticket.open",
			Date:   service.Clock.Stamp(),
			Client: client,
			Prompt: request.Prompt,
			LLM:    llm,
			Effort: effort,
		}},
		FolderPath:    ticketDir,
		JsonPath:      service.Layout.DocumentPath(id),
		ImportantPath: service.Layout.ImportantPath(id),
	}
	AppendSessionID(ticket, request.Session)
	if err := ApplyTicketPlanFromIDs(service.Store, service.Roots, ticket, service.McpClient, request.PlanID, request.SpecID); err != nil {
		return TicketOutcome{}, err
	}
	warnings := make([]string, 0)
	if !request.NoIssue && !request.NoManagement {
		milestone := MilestoneNumberForTitle(service.Tracker, request.Goal)
		warnings = append(warnings, EnsureTicketIssue(service.Tracker, ticket, ticket.Title, request.Prompt, request.Issue, milestone, false)...)
	}
	if err := service.Save(ticket); err != nil {
		return TicketOutcome{}, err
	}
	service.emitTicketEvent(string(events.EventTicketOpenEnded), id, map[string]interface{}{
		"title": ticket.Title, "prompt": request.Prompt, "llm": llm, "effort": effort,
		"client": client, "goal": request.Goal, "parent": request.Parent,
	})
	return service.outcome(id, ticket, journal, warnings), nil
}

// 📪️Close closes a ticket: consumes the important document, syncs and emits.
func (service *TicketService) Close(request TicketCloseRequest) (TicketOutcome, error) {
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	if ticket.Status != model.TicketStatusOpen {
		return TicketOutcome{}, errConflict("ticket is not open")
	}
	files := NormalizeTicketFileInputs(request.Files)
	summary := request.Summary
	if request.Bulk {
		if summary == "" {
			summary = "Bulk close"
		}
	} else {
		if summary == "" {
			return TicketOutcome{}, errInvalid("summary is required to finish a ticket")
		}
		if len(files) == 0 {
			return TicketOutcome{}, errInvalid("at least one file is required to finish a ticket")
		}
	}
	journal := TransactionJournal{}
	preimage, err := InspectImportantDocument(service.Store, ticket.ImportantPath, &journal)
	if err != nil {
		return TicketOutcome{}, err
	}
	if err := MovePlanIntoFolderIn(service.Store, ticket); err != nil {
		return TicketOutcome{}, err
	}
	warnings := make([]string, 0)
	if !request.NoManagement {
		warnings = append(warnings, CloseTicketIssue(service.Tracker, ticket, summary, nil, request.Bulk)...)
	}
	if err := RemoveImportantDocument(service.Store, preimage, &journal); err != nil {
		return TicketOutcome{}, err
	}
	lastClient := ""
	if len(ticket.Interactions) > 0 {
		lastClient = ticket.Interactions[len(ticket.Interactions)-1].Client
	}
	interactionFiles := make([]model.InteractionFile, 0, len(files))
	for _, filePath := range files {
		interactionFiles = append(interactionFiles, model.InteractionFile{Path: filePath})
	}
	ticket.Summary = summary
	ticket.Status = model.TicketStatusClosed
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.close", Date: service.Clock.Stamp(), Client: lastClient, Summary: summary, Files: interactionFiles,
	})
	if err := service.Save(ticket); err != nil {
		journal.Record("save", ticket.JsonPath, "failed")
		if restore := RestoreImportantDocument(service.Store, preimage, &journal); restore != nil {
			return TicketOutcome{}, errStore(fmt.Sprintf("save ticket: %v; restore important document: %v", err, restore))
		}
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
	service.emitTicketEvent(string(events.EventTicketCloseEnded), id, map[string]interface{}{"summary": summary, "files": files})
	return service.outcome(id, ticket, journal, warnings), nil
}

// 🔓️Reopen reopens a closed ticket, creating the important document transactionally.
func (service *TicketService) Reopen(request TicketReopenRequest) (TicketOutcome, error) {
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	if ticket.Status != model.TicketStatusClosed {
		return TicketOutcome{}, errConflict("ticket is already open")
	}
	if strings.TrimSpace(request.Client) == "" {
		return TicketOutcome{}, errInvalid("client is required")
	}
	llm, err := resolveOptionalSlug(request.LLM, model.ResolveAllowedLLM)
	if err != nil {
		return TicketOutcome{}, err
	}
	effort, err := resolveOptionalSlug(request.Effort, model.ResolveAllowedEffort)
	if err != nil {
		return TicketOutcome{}, err
	}
	client, err := model.ResolveAllowedClient(request.Client)
	if err != nil {
		return TicketOutcome{}, errInvalid(err.Error())
	}
	if strings.TrimSpace(request.Goal) != "" {
		ticket.Goal = strings.TrimSpace(request.Goal)
	}
	if strings.TrimSpace(request.Parent) != "" {
		ticket.Parent = strings.TrimSpace(request.Parent)
	}
	if err := ApplyTicketPlanFromIDs(service.Store, service.Roots, ticket, service.McpClient, request.PlanID, request.SpecID); err != nil {
		return TicketOutcome{}, err
	}
	journal := TransactionJournal{}
	creation, err := EnsureImportantDocument(service.Store, ticket.ImportantPath, &journal)
	if err != nil {
		return TicketOutcome{}, err
	}
	previousTitle := ticket.Title
	ticket.Status = model.TicketStatusOpen
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.reopen", Date: service.Clock.Stamp(), Client: client, Prompt: request.Prompt, LLM: llm, Effort: effort,
	})
	AppendSessionID(ticket, request.Session)

	warnings := make([]string, 0)
	if !request.NoManagement {
		warnings = append(warnings, EnsureTicketIssue(service.Tracker, ticket, previousTitle, request.Prompt, "", nil, true)...)
		if ticket.Management != nil && ticket.Management.Issue != "" {
			if err := service.Tracker.AddComment(ticket.Management.Issue, FormatPromptHeading(request.Prompt)); err != nil {
				warnings = append(warnings, "Failed to add prompt comment to GitHub issue: "+err.Error())
			}
		}
	}
	if err := service.Save(ticket); err != nil {
		journal.Record("save", ticket.JsonPath, "failed")
		if rollback := RollbackImportantCreation(service.Store, creation, &journal); rollback != nil {
			return TicketOutcome{}, errStore(fmt.Sprintf("save ticket: %v; rollback important document: %v", err, rollback))
		}
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
	service.emitTicketEvent(string(events.EventTicketReopenEnded), id, map[string]interface{}{
		"prompt": request.Prompt, "llm": llm, "effort": effort, "client": client,
	})
	return service.outcome(id, ticket, journal, warnings), nil
}

// ♻️Change changes a ticket in place, renaming its folder when the title changes its slug.
func (service *TicketService) Change(request TicketChangeRequest) (TicketOutcome, error) {
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	journal := TransactionJournal{}
	nextID := id
	if request.Title != nil {
		title := strings.TrimSpace(*request.Title)
		if title == "" {
			return TicketOutcome{}, errInvalid("ticket title is required")
		}
		slug, err := TicketSlugFromTitle(title)
		if err != nil {
			return TicketOutcome{}, err
		}
		if parent, nested := id.ParentSlug(); nested {
			slug = parent + "/" + slug
		}
		nextID = NewTicketID(id.Year, id.Month, id.Day, slug)
		if nextID.Slug != id.Slug {
			target := service.Layout.TicketDir(nextID)
			if StoreExists(service.Store, target) {
				return TicketOutcome{}, errConflict("ticket folder already exists: " + target)
			}
			if err := service.Store.Rename(service.Layout.TicketDir(id), target); err != nil {
				return TicketOutcome{}, err
			}
			journal.Record("rename", target, "ok")
		}
		ticket.Title = title
	}
	if request.Description != nil {
		ticket.Description = *request.Description
	}
	if request.Goal != nil {
		ticket.Goal = *request.Goal
	}
	if request.Parent != nil {
		ticket.Parent = *request.Parent
	}
	service.hydrate(ticket, nextID)
	if err := service.Save(ticket); err != nil {
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
	service.emitTicketEvent(string(events.EventTicketChangeEnded), nextID, map[string]interface{}{"title": ticket.Title, "goal": ticket.Goal})
	return service.outcome(nextID, ticket, journal, make([]string, 0)), nil
}

// 🧹️PurgeArtifacts deletes oversized artifacts from one ticket folder, deepest folder first.
func (service *TicketService) PurgeArtifacts(id TicketID) (PurgeReport, error) {
	root := service.Layout.TicketDir(id)
	report := PurgeReport{RemovedDirectories: make([]string, 0), RemovedFiles: make([]string, 0)}
	if !StoreIsDirectory(service.Store, root) {
		return report, nil
	}
	directories := make([]string, 0)
	files := make([]sizedPath, 0)
	service.collectTree(root, &directories, &files)
	directorySizes := map[string]int64{}
	for _, file := range files {
		parent := DirName(file.path)
		for strings.HasPrefix(parent, root) {
			directorySizes[parent] += file.size
			if parent == root {
				break
			}
			parent = DirName(parent)
		}
	}
	sort.SliceStable(directories, func(left, right int) bool {
		return strings.Count(directories[left], "/") > strings.Count(directories[right], "/")
	})
	deleted := make([]string, 0)
	for _, directory := range directories {
		skip := false
		for _, removed := range deleted {
			if strings.HasPrefix(directory, removed+"/") {
				skip = true
				break
			}
		}
		if skip || directorySizes[directory] <= OversizedFolderBytes {
			continue
		}
		if err := service.Store.RemoveTree(directory); err != nil {
			return report, err
		}
		report.RemovedDirectories = append(report.RemovedDirectories, directory)
		deleted = append(deleted, directory)
	}
	for _, file := range files {
		skip := false
		for _, removed := range deleted {
			if file.path == removed || strings.HasPrefix(file.path, removed+"/") {
				skip = true
				break
			}
		}
		if skip || BaseName(file.path) == TicketDocumentName || file.size <= OversizedFileBytes {
			continue
		}
		if err := service.Store.RemoveFile(file.path); err != nil {
			return report, err
		}
		report.RemovedFiles = append(report.RemovedFiles, file.path)
	}
	sort.Strings(report.RemovedDirectories)
	sort.Strings(report.RemovedFiles)
	return report, nil
}

// 📏️sizedPath is one collected file with its size.
type sizedPath struct {
	path string
	size int64
}

// 🚶️collectTree collects every directory and file below a root.
func (service *TicketService) collectTree(root string, directories *[]string, files *[]sizedPath) {
	entries, err := service.Store.Entries(root)
	if err != nil {
		return
	}
	for _, entry := range entries {
		path := JoinPath(root, entry.Name)
		switch entry.Kind {
		case NodeDirectory:
			*directories = append(*directories, path)
			service.collectTree(path, directories, files)
		case NodeFile:
			size := int64(0)
			if metadata, found := service.Store.Metadata(path); found {
				size = metadata.Size
			}
			*files = append(*files, sizedPath{path: path, size: size})
		}
	}
}

// 💧️hydrate fills in every derived path of a ticket.
func (service *TicketService) hydrate(ticket *model.Ticket, id TicketID) {
	ticket.Year = id.Year
	ticket.Month = id.Month
	ticket.Day = id.Day
	ticket.Slug = id.Slug
	ticket.FolderPath = service.Layout.TicketDir(id)
	ticket.JsonPath = service.Layout.DocumentPath(id)
	ticket.ImportantPath = service.Layout.ImportantPath(id)
}

// 🎫️outcome assembles the outcome of one lifecycle operation.
func (service *TicketService) outcome(id TicketID, ticket *model.Ticket, journal TransactionJournal, warnings []string) TicketOutcome {
	return TicketOutcome{
		ID:       id.ID(),
		RelPath:  id.RelPath(),
		Status:   string(ticket.Status),
		Document: EncodeTicketDocument(ticket),
		Journal:  journal,
		Warnings: warnings,
	}
}

// 📤️emitTicketEvent emits one lifecycle event with its identity members merged in.
func (service *TicketService) emitTicketEvent(kind string, id TicketID, extra map[string]interface{}) {
	if service.Events == nil {
		return
	}
	payload := map[string]interface{}{"id": id.RelPath(), "year": id.Year, "month": id.Month, "day": id.Day, "slug": id.Slug}
	for name, value := range extra {
		payload[name] = value
	}
	service.Events.Emit(kind, TicketEventSource, canonicalTicketPayload(payload))
}

// 🎆️datedChildren returns the date directories under a path, ascending.
func (service *TicketService) datedChildren(path, prefix string, pinned *int) []int {
	if pinned != nil {
		candidate := JoinPath(path, prefix+PadNumber(*pinned, 2))
		if StoreExists(service.Store, candidate) {
			return []int{*pinned}
		}
		return nil
	}
	entries, err := service.Store.Entries(path)
	if err != nil {
		return nil
	}
	values := make([]int, 0, len(entries))
	for _, entry := range entries {
		if entry.Kind != NodeDirectory {
			continue
		}
		if value, err := ParseDatedDirSegment(entry.Name, prefix); err == nil {
			values = append(values, value)
		}
	}
	sort.Ints(values)
	return values
}

// 🔤️slugsUnder returns every ticket slug under a day directory.
func (service *TicketService) slugsUnder(dayPath, prefix string) []string {
	root := dayPath
	if prefix != "" {
		root = JoinPath(dayPath, prefix)
	}
	entries, err := service.Store.Entries(root)
	if err != nil {
		return nil
	}
	slugs := make([]string, 0)
	for _, entry := range entries {
		if entry.Kind != NodeDirectory || strings.HasPrefix(entry.Name, ".") || containsTicketString(SkippedWalkDirs, entry.Name) {
			continue
		}
		slug := entry.Name
		if prefix != "" {
			slug = prefix + "/" + entry.Name
		}
		if StoreExists(service.Store, JoinPath(JoinPath(dayPath, slug), TicketDocumentName)) {
			slugs = append(slugs, slug)
			continue
		}
		slugs = append(slugs, service.slugsUnder(dayPath, slug)...)
	}
	return slugs
}

// 🔎️containsTicketString reports whether a slice carries a value.
func containsTicketString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🔤️resolveOptionalSlug resolves a slug when it is given, and accepts a blank one.
func resolveOptionalSlug(value string, resolve func(string) (string, error)) (string, error) {
	if strings.TrimSpace(value) == "" {
		return "", nil
	}
	resolved, err := resolve(value)
	if err != nil {
		return "", errInvalid(err.Error())
	}
	return resolved, nil
}

// 🧮️canonicalTicketPayload renders a payload as compact JSON with members in ascending key order.
func canonicalTicketPayload(payload interface{}) string {
	data, err := json.Marshal(payload)
	if err != nil {
		return ""
	}
	var generic interface{}
	if err := json.Unmarshal(data, &generic); err != nil {
		return ""
	}
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(generic); err != nil {
		return ""
	}
	return strings.TrimRight(buffer.String(), "\n")
}

// 📥️ParseOpenRequest reads an open request from JSON text.
func ParseOpenRequest(text string) (TicketOpenRequest, error) {
	var request TicketOpenRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketOpenRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseCloseRequest reads a close request from JSON text.
func ParseCloseRequest(text string) (TicketCloseRequest, error) {
	var request TicketCloseRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketCloseRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseReopenRequest reads a reopen request from JSON text.
func ParseReopenRequest(text string) (TicketReopenRequest, error) {
	var request TicketReopenRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketReopenRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseChangeRequest reads a change request from JSON text.
func ParseChangeRequest(text string) (TicketChangeRequest, error) {
	var request TicketChangeRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketChangeRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// #endregion 🔓️Lifecycle

// #region 🔎️Search

// 🔍️TicketQuery is what narrows a ticket listing.
type TicketQuery struct {
	Filter         string   `json:"filter"`
	Query          string   `json:"query"`
	MatchCase      bool     `json:"matchCase"`
	MatchWholeWord bool     `json:"matchWholeWord"`
	Status         *string  `json:"status"`
	IncludeYears   []int    `json:"includeYears"`
	ExcludeYears   []int    `json:"excludeYears"`
	Slugs          []string `json:"slugs"`
}

// ✔️Matches reports whether a ticket survives the query.
func (query TicketQuery) Matches(ticket *model.Ticket) bool {
	if query.Status != nil && string(ticket.Status) != *query.Status {
		return false
	}
	if len(query.IncludeYears) > 0 && !containsTicketInt(query.IncludeYears, ticket.Year) {
		return false
	}
	if containsTicketInt(query.ExcludeYears, ticket.Year) {
		return false
	}
	if len(query.Slugs) > 0 && !containsTicketString(query.Slugs, ticket.Slug) {
		return false
	}
	identifier := NewTicketID(ticket.Year, ticket.Month, ticket.Day, ticket.Slug).ID()
	if query.Filter != "" {
		matched := false
		for _, candidate := range []string{identifier, ticket.Slug, ticket.Title} {
			if query.matchesFilter(candidate) {
				matched = true
				break
			}
		}
		if !matched {
			return false
		}
	}
	if query.Query != "" {
		haystack := strings.ToLower(fmt.Sprintf("%s %s %s %s %s", identifier, ticket.Slug, ticket.Title, ticket.Description, ticket.Status))
		if !strings.Contains(haystack, strings.ToLower(query.Query)) {
			return false
		}
	}
	return true
}

// 🔤️matchesFilter reports whether one candidate satisfies the filter.
func (query TicketQuery) matchesFilter(name string) bool {
	target, pattern := name, query.Filter
	if !query.MatchCase {
		target, pattern = strings.ToLower(name), strings.ToLower(query.Filter)
	}
	if query.MatchWholeWord {
		for _, word := range strings.FieldsFunc(target, func(character rune) bool {
			return !((character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z') || (character >= '0' && character <= '9') || character == '_')
		}) {
			if word == pattern {
				return true
			}
		}
		return false
	}
	return strings.Contains(target, pattern)
}

// 🔎️containsTicketInt reports whether a slice carries a value.
func containsTicketInt(values []int, value int) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🔍️Search returns every ticket a query keeps, in listing order.
func (service *TicketService) Search(query TicketQuery) ([]*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	kept := make([]*model.Ticket, 0, len(tickets))
	for _, ticket := range tickets {
		if query.Matches(ticket) {
			kept = append(kept, ticket)
		}
	}
	return kept, nil
}

// #endregion 🔎️Search

// #endregion 🚪️Ports
