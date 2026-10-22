package tickets

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🖨️Print

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
	ignored := workspace.GetGitIgnoredSet(workspace.GetRootDir(), relPaths)
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
	agentsPath := filepath.Join(workspace.GetRootDir(), "AGENTS.md")
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

// #endregion 🖨️Print
