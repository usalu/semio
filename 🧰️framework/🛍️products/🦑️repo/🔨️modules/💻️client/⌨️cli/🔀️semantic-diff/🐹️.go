// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Semantic change types and ticket diff computation.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"strings"
)

// ♻️SemanticChangeType represents a semantic change type value.
type SemanticChangeType string

const (
	SemanticChangeAdded    SemanticChangeType = "added"
	SemanticChangeDeleted  SemanticChangeType = "deleted"
	SemanticChangeModified SemanticChangeType = "modified"
	SemanticChangeRenamed  SemanticChangeType = "renamed"
)

// 🔖️SemanticChange holds the data fields for a semantic change record.
type SemanticChange struct {
	Kind     string
	Status   SemanticChangeType
	Path     string
	FromPath string
	ToPath   string
	Lines    LineMetrics
}

// 🗃️newTicketDiffSet holds the data fields for a newTicketDiffSet record.
func newTicketDiffSet() TicketDiffSet {
	return TicketDiffSet{
		Deleted:  []TicketFile{},
		Renamed:  []TicketFileRenamed{},
		Modified: []TicketFile{},
		Added:    []TicketFile{},
	}
}

// 🔖️newTicketDiffs holds the data fields for a newTicketDiffs record.
func newTicketDiffs() *TicketDiffs {
	return &TicketDiffs{
		Bundles:     newTicketDiffSet(),
		Folders:     newTicketDiffSet(),
		Files:       newTicketDiffSet(),
		Sections:    newTicketDiffSet(),
		Definitions: newTicketDiffSet(),
	}
}

// ➕️addTicketDiffEntry holds the data fields for a addTicketDiffEntry record.
func addTicketDiffEntry(set *TicketDiffSet, change SemanticChange) {
	lines := &LineMetrics{Added: change.Lines.Added, Removed: change.Lines.Removed}
	switch change.Status {
	case SemanticChangeAdded:
		set.Added = append(set.Added, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeDeleted:
		set.Deleted = append(set.Deleted, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeRenamed:
		set.Renamed = append(set.Renamed, TicketFileRenamed{From: change.FromPath, To: change.ToPath, Lines: lines})
	case SemanticChangeModified:
		set.Modified = append(set.Modified, TicketFile{Path: change.Path, Lines: lines})
	}
}

// 🔖️mergeLineMetrics holds the data fields for a mergeLineMetrics record.
func mergeLineMetrics(target *LineMetrics, add LineMetrics) {
	if target == nil {
		return
	}
	target.Added += add.Added
	target.Removed += add.Removed
}

// 🔖️computeLineMetricsForDiff holds the data fields for a computeLineMetricsForDiff record.
func computeLineMetricsForDiff(diff *DiffLines, baseCheckpoint, filePath string) LineMetrics {
	if diff == nil {
		return LineMetrics{}
	}
	if len(diff.Added) > 0 && len(diff.Removed) == 0 {
		return LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), filePath)), Removed: 0}
	}
	if len(diff.Removed) > 0 && len(diff.Added) == 0 {
		return LineMetrics{Added: 0, Removed: CountLinesAtCheckpoint(baseCheckpoint, filePath)}
	}
	return LineMetrics{Added: len(diff.Added), Removed: len(diff.Removed)}
}

func buildCodebasePathSet(codebase *Codebase) map[string]struct{} {
	result := make(map[string]struct{})
	if codebase == nil {
		return result
	}
	for _, bundle := range codebase.Bundles {
		result[bundle.ID] = struct{}{}
	}
	for _, folder := range codebase.Folders {
		result[folder.Path] = struct{}{}
	}
	for _, file := range codebase.Files {
		result[file.Path] = struct{}{}
	}
	for _, section := range codebase.Sections {
		result[section.Path] = struct{}{}
	}
	for _, def := range codebase.Definitions {
		result[def.Path] = struct{}{}
	}
	return result
}

// 🏗️buildFolderLineTotals holds the data fields for a buildFolderLineTotals record.
func buildFolderLineTotals(files []string, baseCheckpoint string, bundles []Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &CodebaseContext{Bundles: bundles}
	for _, file := range files {
		folderPath := NormalizePath(filepath.Dir(file))
		if folderPath == "." {
			continue
		}
		id := ctx.GetFolderID(folderPath)
		currentTotals[id] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[id] += CountLinesAtCheckpoint(baseCheckpoint, file)
	}
	return currentTotals, baseTotals
}

// 🔖️buildBundleLineTotals holds the data fields for a buildBundleLineTotals record.
func buildBundleLineTotals(files []string, baseCheckpoint string, bundles []Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &CodebaseContext{Bundles: bundles}
	for _, file := range files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		currentTotals[bundleName] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[bundleName] += CountLinesAtCheckpoint(baseCheckpoint, file)
	}
	return currentTotals, baseTotals
}

// 🔖️extractFilePrefix holds the data fields for a extractFilePrefix record.
func extractFilePrefix(path string) string {
	if path == "" {
		return ""
	}
	if idx := strings.Index(path, "#"); idx != -1 {
		return path[:idx]
	}
	if idx := strings.Index(path, "§"); idx != -1 {
		return path[:idx]
	}
	return path
}

// 🔖️reconcileRenamePairs holds the data fields for a reconcileRenamePairs record.
func reconcileRenamePairs(diffSet *TicketDiffSet, matchKey func(path string) string) {
	if diffSet == nil {
		return
	}
	usedAdded := make(map[int]struct{})
	var renamed []TicketFileRenamed
	var remainingDeleted []TicketFile
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
		lines := &LineMetrics{}
		if add.Lines != nil {
			lines.Added = add.Lines.Added
		}
		if del.Lines != nil {
			lines.Removed = del.Lines.Removed
		}
		renamed = append(renamed, TicketFileRenamed{From: del.Path, To: add.Path, Lines: lines})
	}
	var remainingAdded []TicketFile
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
func buildSectionDiffs(baseCodebase, currentCodebase *Codebase, baseCheckpoint string, diffLines map[string]*DiffLines, bundles []Bundle) TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &CodebaseContext{Bundles: bundles}
	currentSectionMap := make(map[string]CodebaseSection)
	baseSectionMap := make(map[string]CodebaseSection)

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
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseSectionMap {
		if _, ok := currentSectionMap[path]; !ok {
			lines := 0
			if baseSectionMap[path].Metrics != nil {
				lines = baseSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCheckpoint(baseCheckpoint, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
		if lang == nil || !lang.SupportsSections() {
			continue
		}
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		addedMap := computeSectionLineMap(currentSections, diff.Added, "")
		removedMap := computeSectionLineMap(baseSections, diff.Removed, "")
		for sectionPath, addedLines := range addedMap {
			removedLines := removedMap[sectionPath]
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for sectionPath, removedLines := range removedMap {
			if _, ok := addedMap[sectionPath]; ok {
				continue
			}
			if len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

// 🔖️buildDefinitionDiffs holds the data fields for a buildDefinitionDiffs record.
func buildDefinitionDiffs(baseCodebase, currentCodebase *Codebase, baseCheckpoint string, diffLines map[string]*DiffLines, bundles []Bundle) TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &CodebaseContext{Bundles: bundles}
	currentDefMap := make(map[string]CodebaseDefinition)
	baseDefMap := make(map[string]CodebaseDefinition)

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
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseDefMap {
		if _, ok := currentDefMap[path]; !ok {
			lines := 0
			if baseDefMap[path].Metrics != nil {
				lines = baseDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCheckpoint(baseCheckpoint, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
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
			addedLines := computeLinesInRange(diff.Added, def.Start, def.End)
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(currentSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for _, def := range baseDefs {
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(baseSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

// 🔖️BuildSemanticDiffs MUST assemble the semantic diffs from the available context data.
// ⚖️BuildSemanticDiffs constructs and returns the semantic diffs structure.
func BuildSemanticDiffs(baseCodebase, currentCodebase *Codebase, baseCheckpoint string, diffLines map[string]*DiffLines, diffStatuses []GitDiffStatus, bundles []Bundle) *TicketDiffs {
	result := newTicketDiffs()
	ctx := &CodebaseContext{Bundles: bundles}

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

	currentBundleMap := make(map[string]CodebaseBundle)
	baseBundleMap := make(map[string]CodebaseBundle)
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
			lines := LineMetrics{Added: currentBundleLines[id], Removed: 0}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeAdded, Path: normalizeBundleLabel(id), Lines: lines})
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
					addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeModified, Path: normalizeBundleLabel(id), Lines: LineMetrics{Added: added, Removed: removed}})
				}
			}
		}
	}
	for id := range baseBundleMap {
		if _, ok := currentBundleMap[id]; !ok {
			lines := LineMetrics{Added: 0, Removed: baseBundleLines[id]}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeDeleted, Path: normalizeBundleLabel(id), Lines: lines})
		}
	}

	currentFolderMap := make(map[string]CodebaseFolder)
	baseFolderMap := make(map[string]CodebaseFolder)
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
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: currentFolderLines[path]}})
		}
	}
	for path := range baseFolderMap {
		if _, ok := currentFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: baseFolderLines[path]}})
		}
	}

	for _, status := range diffStatuses {
		if status.Status == "renamed" {
			fromFolder := NormalizePath(filepath.Dir(status.From))
			toFolder := NormalizePath(filepath.Dir(status.To))
			fromFolderID := ctx.GetFolderID(fromFolder)
			toFolderID := ctx.GetFolderID(toFolder)
			if fromFolderID != toFolderID && fromFolder != "." && toFolder != "." {
				addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeRenamed, FromPath: fromFolderID, ToPath: toFolderID, Lines: LineMetrics{Added: currentFolderLines[toFolderID], Removed: baseFolderLines[fromFolderID]}})
			}
			fromFileID := ctx.GetFileID(status.From)
			toFileID := ctx.GetFileID(status.To)
			addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: SemanticChangeRenamed, FromPath: fromFileID, ToPath: toFileID, Lines: LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), status.To)), Removed: CountLinesAtCheckpoint(baseCheckpoint, status.From)}})
		}
	}

	for filePath, diff := range diffLines {
		fileID := ctx.GetFileID(filePath)
		metrics := computeLineMetricsForDiff(diff, baseCheckpoint, filePath)
		status := SemanticChangeModified
		if len(diff.Added) > 0 && len(diff.Removed) == 0 {
			status = SemanticChangeAdded
		} else if len(diff.Removed) > 0 && len(diff.Added) == 0 {
			status = SemanticChangeDeleted
		}
		addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: status, Path: fileID, Lines: metrics})
	}

	reconcileRenamePairs(&result.Bundles, func(path string) string {
		return path
	})

	result.Sections = buildSectionDiffs(baseCodebase, currentCodebase, baseCheckpoint, diffLines, bundles)
	result.Definitions = buildDefinitionDiffs(baseCodebase, currentCodebase, baseCheckpoint, diffLines, bundles)

	return result
}
