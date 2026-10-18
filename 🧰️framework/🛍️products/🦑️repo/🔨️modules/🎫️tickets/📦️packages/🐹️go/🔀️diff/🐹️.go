package tickets

import (
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔀️Semantic Diff

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

// #endregion 🔀️Semantic Diff
