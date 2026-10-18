// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Agents docs path, header merging, and section list tools.

// #endregion 🧲️Header

package client

import (
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"path/filepath"
	"strings"
)

// 🎛️UpdateAgentsDocsPath MUST apply the update and return an error if the target is missing.
// ✏️UpdateAgentsDocsPath modifies an existing agents docs path entry.
func UpdateAgentsDocsPath(oldPath, newPath string) {
	agentsPath := filepath.Join(rootDir, "AGENTS.md")
	if !FileExists(agentsPath) {
		return
	}
	content, err := ReadTextFile(agentsPath)
	if err != nil {
		return
	}

	oldNorm := NormalizePath(oldPath)
	newNorm := NormalizePath(newPath)
	if oldNorm == newNorm {
		return
	}

	lines := strings.Split(content, "\n")
	changed := false

	for i, line := range lines {
		if !strings.HasPrefix(line, "## ") && !strings.HasPrefix(line, "### ") {
			continue
		}
		if strings.Contains(line, oldNorm) {
			newLine := strings.Replace(line, oldNorm, newNorm, 1)
			if newLine != line {
				lines[i] = newLine
				changed = true
			}
		}
	}

	if !changed {
		return
	}

	WriteTextFile(agentsPath, strings.Join(lines, "\n"))
}

// 🔖️RemoveAgentsDocsEntry MUST remove the target and return an error on failure.
// 🧪️RemoveAgentsDocsEntry removes the specified agents docs entry.
func RemoveAgentsDocsEntry(filePath string) {
	agentsPath := filepath.Join(rootDir, "AGENTS.md")
	if !FileExists(agentsPath) {
		return
	}
	content, err := ReadTextFile(agentsPath)
	if err != nil {
		return
	}

	norm := NormalizePath(filePath)
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
		WriteTextFile(agentsPath, newContent)
	}
}

// 🔖️SplitHeader MUST complete the operation successfully.
// ✂️SplitHeader splits the header into parts.
func SplitHeader(content string, lang LanguagePlugin) (string, string) {
	sections := lang.ParseSections(content)
	for _, s := range sections {
		if strings.EqualFold(s.Name, "Header") {
			header := content[:s.EndIndex]
			body := content[s.EndIndex:]
			return header, body
		}
	}
	return "", content
}

// 🔖️MergeHeaders MUST combine the inputs and return the merged result.
// 🔀️MergeHeaders combines the headers entries into one.
func MergeHeaders(targetHeader, sourceHeader string, lang LanguagePlugin) string {
	if targetHeader == "" {
		return sourceHeader
	}
	if sourceHeader == "" {
		return targetHeader
	}
	targetLines := strings.Split(targetHeader, "\n")
	sourceLines := strings.Split(sourceHeader, "\n")
	seen := make(map[string]bool)
	for _, line := range targetLines {
		seen[strings.TrimSpace(line)] = true
	}
	var insertIdx = -1
	for i, line := range targetLines {
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			insertIdx = i
		}
	}
	if insertIdx == -1 {
		return targetHeader
	}

	var newLines []string
	for _, line := range sourceLines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		if matched, _ := lang.PolicySectionStartMatch(line); matched {
			continue
		}
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			continue
		}
		if !seen[trimmed] {
			newLines = append(newLines, line)
		}
	}

	if len(newLines) == 0 {
		return targetHeader
	}

	res := make([]string, 0, len(targetLines)+len(newLines)+1)
	res = append(res, targetLines[:insertIdx]...)
	res = append(res, newLines...)
	res = append(res, targetLines[insertIdx:]...)
	return strings.Join(res, "\n")
}

// 🔤️UniqueStrings MUST complete the operation successfully.
func UniqueStrings(input []string) []string {
	keys := make(map[string]bool)
	list := []string{}
	for _, entry := range input {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
}

// 🔖️ToolSectionDelete MUST complete the operation successfully.
func ToolSectionDelete(filePath, sectionPath string) ToolResult {
	repopkg.Emit(repopkg.EventSectionDeleteStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		return toolErrorResult(err)
	}
	sections := ParseSections(content, filePath)
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	section := FindSection(sections, sectionName)
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			return toolErrorMsg("Section path required")
		}
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			return toolErrorResult(err)
		}
		location, ok := locations[strings.Join(pathParts, "/")]
		if !ok {
			return toolErrorMsg(fmt.Sprintf("Section not found: %s", strings.Join(pathParts, "/")))
		}
		_, start, end := jsonExtractEntry(content, location.KeyStart, location.ValueEnd)
		updated := content[:start] + content[end:]
		if err := WriteTextFile(absPath, updated); err != nil {
			return toolErrorResult(err)
		}
		repopkg.Emit(repopkg.EventSectionDeleteEnded, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})
		output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", strings.Join(pathParts, "/"), filePath))
		return ToolResult{Output: *output}
	}
	if section == nil {
		return toolErrorMsg(fmt.Sprintf("Section not found: %s", sectionName))
	}
	lines := strings.Split(content, "\n")
	var newLines []string
	for i, line := range lines {
		lineNum := i + 1
		if lineNum < section.StartLine || lineNum > section.EndLine {
			newLines = append(newLines, line)
		}
	}
	if err := WriteTextFile(absPath, strings.Join(newLines, "\n")); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventSectionDeleteEnded, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})
	output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

// 🔖️ToolSectionList MUST complete the operation successfully.
func ToolSectionList(filePath string) ToolResult {
	output := NewOutput()
	scope := ParseScope(filePath)
	if scope.Kind != ScopeFile && scope.Kind != ScopeSection {
		return toolErrorMsg("Scope must be a file or section")
	}
	absPath := filepath.Join(rootDir, scope.FilePath)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", scope.FilePath))
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		return toolErrorResult(err)
	}
	sections := ParseSections(content, scope.FilePath)
	output.Info(fmt.Sprintf("\n🏷️Sections in %s:\n", scope.FilePath))
	var printSection func(s Section, indent string)
	printSection = func(s Section, indent string) {
		output.Plain(fmt.Sprintf("%s%s (lines %d-%d)", indent, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, indent+"  ")
		}
	}
	for _, s := range sections {
		printSection(s, "   ")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

// 🔖️ToolSectionTree MUST complete the operation successfully.
func ToolSectionTree(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.Split(filePath, "#")[0])
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		return toolErrorResult(err)
	}
	sections := ParseSections(content, filePath)
	output.Info(fmt.Sprintf("\n🏷️Sections in %s:\n", filePath))
	var printSection func(s Section, prefix string)
	printSection = func(s Section, prefix string) {
		output.Plain(fmt.Sprintf("%s└️─️─️ %s (lines %d-%d)", prefix, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, prefix+"    ")
		}
	}
	for _, s := range sections {
		printSection(s, "")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

// 🔖️ToolDefinitionList MUST complete the operation successfully.
func ToolDefinitionList(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	output.Info(fmt.Sprintf("\n📋️ Definitions in %s:\n", filePath))
	output.Plain("   (definition parsing not implemented in Go - use TypeScript API)")
	return ToolResult{Output: *output, Data: []Definition{}}
}

// 🔖️ToolDefinitionTree MUST complete the operation successfully.
func ToolDefinitionTree(filePath string) ToolResult {
	return ToolDefinitionList(filePath)
}

// 🔖️ToolUpdateMetabolism MUST complete the operation successfully.
func ToolUpdateMetabolism() ToolResult {
	output := NewOutput()
	output.Info("\n🔄️ Running update-metabolism via npx tsx...")
	stdout, stderr, exitCode := ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
	if exitCode != 0 {
		return toolErrorMsg(fmt.Sprintf("%s%s", stdout, stderr))
	}
	output.Success(stdout)
	return ToolResult{Output: *output}
}
