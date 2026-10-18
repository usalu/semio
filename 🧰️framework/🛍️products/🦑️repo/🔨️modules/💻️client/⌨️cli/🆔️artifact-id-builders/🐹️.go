// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact id builders and git diff helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"strings"
)

// 📦️GetBundleByPath MUST retrieve the requested value or return an error.
// 🔻️GetBundleByPath retrieves and returns the bundle by path.
func GetBundleByPath(path string) *Bundle {
	bundles := GetTechnologies()
	normalizedPath := NormalizePath(path)
	var bestMatch *Bundle
	var matchedLen int
	for i := range bundles {
		bundle := &bundles[i]
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				bestMatch = bundle
				matchedLen = len(root)
			}
		}
	}
	return bestMatch
}

// 🔷️findBundleInfo holds the data fields for a findBundleInfo record.
func findBundleInfo(path string) (name, root string, ok bool) {
	bundles := GetTechnologies()
	normalizedPath := NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range bundles {
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return matchedBundle, matchedRoot, true
	}
	return "", "", false
}

// 🛤️resolveParentIDFromPath holds the data fields for a resolveParentIDFromPath record.
func resolveParentIDFromPath(dirPath string) string {
	normalized := NormalizePath(dirPath)
	if normalized == "." || normalized == "" {
		bundle := GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	bundle := GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if strings.HasPrefix(normalized, bundleRoot+"/") {
			relPath := normalized[len(bundleRoot)+1:]
			parts := strings.Split(relPath, "/")
			parentID := bundle.GetID()
			currentPath := bundleRoot
			for _, part := range parts {
				currentPath += "/" + part
				kind := DeriveFolderKind(currentPath)
				if kind == FolderKindOrganization {
					parentID += emojiText(EmojiFolderOrg) + Flat(part)
				} else {
					parentID += emojiText(EmojiFolderRequired) + Flat(part)
				}
			}
			return parentID
		}
	}
	parts := strings.Split(normalized, "/")
	parentID := ""
	currentPath := ""
	for _, part := range parts {
		if currentPath == "" {
			currentPath = part
		} else {
			currentPath += "/" + part
		}
		kind := DeriveFolderKind(currentPath)
		if kind == FolderKindOrganization {
			parentID += emojiText(EmojiFolderOrg) + Flat(part)
		} else {
			parentID += emojiText(EmojiFolderRequired) + Flat(part)
		}
	}
	return parentID
}

// 📁️buildFolderID holds the data fields for a buildFolderID record.
func buildFolderID(path string, bundleID *string) string {
	normalized := NormalizePath(path)
	if normalized == "." || normalized == "" {
		bundle := GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	dir := filepath.Dir(normalized)
	name := filepath.Base(normalized)
	parentID := ""
	bundle := GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if dir == bundleRoot {
			parentID = bundle.GetID()
		} else if strings.HasPrefix(dir, bundleRoot+"/") {
			parentID = resolveParentIDFromPath(dir)
		} else {
			parentID = resolveParentIDFromPath(dir)
		}
	} else if dir != "." && dir != "" {
		parentID = resolveParentIDFromPath(dir)
	} else {
		repoBundle := GetBundleByPath(".")
		if repoBundle != nil {
			parentID = repoBundle.GetID()
		}
	}
	kind := DeriveFolderKind(normalized)
	if kind == FolderKindRoot {
		if bundle != nil {
			return bundle.GetID()
		}
	}
	if kind == FolderKindOrganization {
		return parentID + emojiText(EmojiFolderOrg) + Flat(name)
	}
	return parentID + emojiText(EmojiFolderRequired) + Flat(name)
}

// 📄️buildFileID holds the data fields for a buildFileID record.
func buildFileID(path string, bundleID *string) string {
	normalized := NormalizePath(path)
	name := filepath.Base(normalized)
	kind := DeriveFileKind(name)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		bundle := GetBundleByPath(normalized)
		if bundle != nil && NormalizePath(bundle.Root) == NormalizePath(dir) {
			parentID = bundle.GetID()
		} else {
			parentID = resolveParentIDFromPath(dir)
		}
	} else {
		bundle := GetBundleByPath(normalized)
		if bundle != nil {
			parentID = bundle.GetID()
		}
	}
	return GetArtifactID("file", map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID})
}

// 📑️buildSectionID holds the data fields for a buildSectionID record.
func buildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 || (len(sectionPath) == 1 && sectionPath[0] == "") {
		return fileID
	}
	var segments []string
	for _, s := range sectionPath {
		if s != "" {
			segments = append(segments, s)
		}
	}
	if len(segments) == 0 {
		return fileID
	}
	result := fileID
	for _, segment := range segments {
		emoji, name := extractEntityEmoji(segment)
		if emoji == "" {
			emoji = EmojiSection
		}
		result += emojiText(emoji) + Flat(name)
	}
	return result
}

// 🧪️isTestFunctionName holds the data fields for a isTestFunctionName record.
func isTestFunctionName(name string) bool {

	for _, prefix := range []string{"Test", "Benchmark", "Fuzz"} {
		if strings.HasPrefix(name, prefix) && len(name) > len(prefix) && name[len(prefix)] >= 'A' && name[len(prefix)] <= 'Z' {
			return true
		}
	}

	if strings.HasPrefix(name, "test_") {
		return true
	}
	return false
}

// 📖️buildDefinitionID holds the data fields for a buildDefinitionID record.
func buildDefinitionID(fileID string, sectionPath []string, name string, kind DefinitionKind) string {
	effectiveKind := kind
	if kind == DefinitionKindImplementation && strings.Contains(fileID, emojiText(EmojiFileLab)) && isTestFunctionName(name) {
		effectiveKind = DefinitionKindTest
	}
	data := map[string]interface{}{"kind": string(effectiveKind)}
	return buildSectionID(fileID, sectionPath) + definitionKindEmoji(data) + Flat(name)
}

// 🔶️GuessSectionName MUST complete the operation successfully.
func GuessSectionName(filePath string) string {
	base := filepath.Base(filePath)
	ext := filepath.Ext(base)
	name := strings.TrimSuffix(base, ext)
	name = strings.ReplaceAll(name, "-", " ")
	name = strings.ReplaceAll(name, "_", " ")
	name = strings.ReplaceAll(name, ".", " ")
	words := strings.Fields(name)
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}

// 🐙️GetGitDiffSectionLineMetrics MUST retrieve the requested value or return an error.
// ⚖️GetGitDiffSectionLineMetrics retrieves and returns the git diff section line metrics.
func GetGitDiffSectionLineMetrics(baseCheckpoint, endCheckpoint, filePath string) map[string]LineMetrics {
	return nil
}

// 🔖️FlattenSections MUST return a single-level collection with all nested items.
// 🔢️FlattenSections flattens the nested sections into a single level.
func FlattenSections(sections []Section) []Section {
	var result []Section
	var flatten func(secs []Section)
	flatten = func(secs []Section) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

// 🗺️computeSectionLineMap holds the data fields for a computeSectionLineMap record.
func computeSectionLineMap(sections []Section, diffLines []int, parentPath string) map[string][]int {
	result := map[string][]int{}
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		linesInSection := computeLinesInRange(diffLines, section.StartLine, section.EndLine)
		childLines := []int{}
		for _, child := range section.Children {
			childLines = append(childLines, computeLinesInRange(diffLines, child.StartLine, child.EndLine)...)
		}
		exclusiveLines := setDifference(linesInSection, childLines)
		if len(exclusiveLines) > 0 {
			result[sectionPath] = append(result[sectionPath], exclusiveLines...)
		}
		if len(section.Children) > 0 {
			for key, value := range computeSectionLineMap(section.Children, diffLines, sectionPath) {
				result[key] = append(result[key], value...)
			}
		}
	}
	return result
}

func computeAffectedSections(filePath string, sections []Section, defs []DefinitionRange, addedLineMap map[string][]int, removedLineMap map[string][]int, parentPath string) []TicketSection {
	var result []TicketSection
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		exclusiveAddedLines := addedLineMap[sectionPath]
		exclusiveRemovedLines := removedLineMap[sectionPath]

		if len(exclusiveAddedLines) > 0 || len(exclusiveRemovedLines) > 0 {
			var affectedDefs []string
			for _, def := range defs {
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						defAddedLines := computeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, TicketSection{
				Name:        sectionPath,
				Range:       &Range{Start: section.StartLine, End: section.EndLine},
				Definitions: uniqueStrings(affectedDefs),
				Lines:       &LineMetrics{Added: len(exclusiveAddedLines), Removed: len(exclusiveRemovedLines)},
			})
		}

		if len(section.Children) > 0 {
			childResults := computeAffectedSections(filePath, section.Children, defs, addedLineMap, removedLineMap, sectionPath)
			result = append(result, childResults...)
		}
	}
	return result
}

// 🗃️setDifference holds the data fields for a setDifference record.
func setDifference(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var diff []int
	for _, x := range a {
		if !m[x] {
			diff = append(diff, x)
		}
	}
	return diff
}

// 🔹️setIntersection holds the data fields for a setIntersection record.
func setIntersection(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var intersection []int
	for _, x := range a {
		if m[x] {
			intersection = append(intersection, x)
		}
	}
	return intersection
}

// 🔤️uniqueStrings holds the data fields for a uniqueStrings record.
func uniqueStrings(strs []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	for _, s := range strs {
		if !seen[s] {
			seen[s] = true
			result = append(result, s)
		}
	}
	return result
}

func computeLinesInRange(changedLines []int, startLine, endLine int) []int {
	var result []int
	for _, line := range changedLines {
		if line >= startLine && line <= endLine {
			result = append(result, line)
		}
	}
	return result
}

// 🔸️findSectionForLine holds the data fields for a findSectionForLine record.
func findSectionForLine(sections []Section, line int) string {
	for _, section := range sections {
		if line >= section.StartLine && line <= section.EndLine {
			if len(section.Children) > 0 {
				childSection := findSectionForLine(section.Children, line)
				if childSection != "" {
					return section.Name + "/" + childSection
				}
			}
			return section.Name
		}
	}
	return ""
}

// 🧱️BuildGitDiffArgs MUST construct and return the fully initialized result.
// BuildGitDiffArgs constructs and returns the git diff args.
// 🌳️GitIndexRef as baseCheckpoint yields unstaged-only diff (index vs working tree) with no tree-ish.
func BuildGitDiffArgs(flag, baseCheckpoint, headCheckpoint string, paths []string) []string {
	if baseCheckpoint == GitIndexRef {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M"}
		}
		return append([]string{"diff", flag, "-M", "--"}, paths...)
	}
	if headCheckpoint == "" {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M", baseCheckpoint}
		}
		return append([]string{"diff", flag, "-M", baseCheckpoint, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "-M", baseCheckpoint, headCheckpoint}
	}
	return append([]string{"diff", flag, "-M", baseCheckpoint, headCheckpoint, "--"}, paths...)
}

// 🔺️GitDiffStatus holds the data fields for a git diff status record.
type GitDiffStatus struct {
	Status string
	From   string
	To     string
}

// 🔻️GetGitDiffStatus MUST retrieve the requested value or return an error.
// 🐙️GetGitDiffStatus retrieves and returns the git diff status.
func GetGitDiffStatus(baseCheckpoint, headCheckpoint string, paths []string) ([]GitDiffStatus, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := []string{"diff", "--name-status", "-M"}
	if baseCheckpoint != GitIndexRef {
		args = append(args, baseCheckpoint)
		if headCheckpoint != "" {
			args = append(args, headCheckpoint)
		}
	}
	if len(paths) > 0 {
		args = append(args, "--")
		args = append(args, paths...)
	}
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff status failed: %s", strings.TrimSpace(stderr))
	}
	var results []GitDiffStatus
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		status := strings.TrimSpace(parts[0])
		if strings.HasPrefix(status, "R") && len(parts) >= 3 {
			results = append(results, GitDiffStatus{Status: "renamed", From: parts[1], To: parts[2]})
			continue
		}
		file := parts[1]
		switch status {
		case "A":
			results = append(results, GitDiffStatus{Status: "added", To: file})
		case "D":
			results = append(results, GitDiffStatus{Status: "deleted", From: file})
		case "M":
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		default:
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		}
	}
	return results, nil
}
