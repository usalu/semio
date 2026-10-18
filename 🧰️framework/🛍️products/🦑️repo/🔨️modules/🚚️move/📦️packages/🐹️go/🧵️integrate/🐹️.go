// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧵️integrate carries the integrate and extract tools of the move domain with the header split and merge helpers.

// #endregion 🧲️Header

package move

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	languages "github.com/usalu/semio/repo/languages"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📋️Tickets

// 🧬️ToolIntegrate MUST complete the operation successfully.
func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) workspace.ToolResult {
	events.Emit(events.EventIntegrateStarting, "repo-cli", events.IntegratePayload{Source: sourcePath, TargetFile: targetFilePath, TargetSection: targetSectionName})
	output := workspace.NewOutput()

	absSourcePath := filepath.Join(workspace.RootDir, sourcePath)
	if !workspace.FileExists(absSourcePath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source file not found: %s", sourcePath))
	}
	sourceContent, err := workspace.ReadTextFile(absSourcePath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}

	absTargetFilePath := filepath.Join(workspace.RootDir, targetFilePath)
	if !workspace.FileExists(absTargetFilePath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Target file not found: %s", targetFilePath))
	}
	targetContent, err := workspace.ReadTextFile(absTargetFilePath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}

	targetLanguage := languages.GetLanguage(targetFilePath)
	if targetLanguage == nil || !targetLanguage.SupportsSections() {
		return workspace.ToolErrorMsg("Target file type does not support sections")
	}

	output.Info("Splitting headers...")

	sourceHeader, sourceBody := SplitHeader(sourceContent, targetLanguage)
	targetHeader, targetBody := SplitHeader(targetContent, targetLanguage)

	output.Info(fmt.Sprintf("Source Header len: %d, Source Body len: %d", len(sourceHeader), len(sourceBody)))

	_, sourceBodyNoPkg := targetLanguage.ExtractPackage(sourceBody)
	targetPkg, targetBodyNoPkg := targetLanguage.ExtractPackage(targetBody)

	sourceImports, sourceCode := targetLanguage.ExtractImports(sourceBodyNoPkg)
	targetImports, targetCode := targetLanguage.ExtractImports(targetBodyNoPkg)

	mergedHeader := MergeHeaders(targetHeader, sourceHeader, targetLanguage)

	mergedImports := workspace.UniqueStrings(append(targetImports, sourceImports...))

	startMarker := targetLanguage.FormatSectionStart(targetSectionName)
	endMarker := targetLanguage.FormatSectionEnd(targetSectionName)

	if !strings.HasSuffix(sourceCode, "\n") && sourceCode != "" {
		sourceCode += "\n"
	}

	sectionContent := "\n" + startMarker + "\n" + sourceCode + endMarker + "\n"

	var updatedBody string
	if targetParentSectionName != "" {

		sections := targetLanguage.ParseSections(targetCode)
		parentSection := languages.FindSection(sections, targetParentSectionName)
		if parentSection == nil {
			return workspace.ToolErrorMsg(fmt.Sprintf("Parent section not found: %s", targetParentSectionName))
		}

		if parentSection.EndLine == -1 {
			return workspace.ToolErrorMsg(fmt.Sprintf("Parent section %s is not properly closed", targetParentSectionName))
		}

		lines := strings.Split(targetCode, "\n")
		newLines := make([]string, 0, len(lines)+strings.Count(sectionContent, "\n"))
		newLines = append(newLines, lines[:parentSection.EndLine-1]...)
		newLines = append(newLines, strings.Split(strings.Trim(sectionContent, "\n"), "\n")...)
		newLines = append(newLines, lines[parentSection.EndLine-1:]...)
		updatedBody = strings.Join(newLines, "\n")
	} else {

		updatedBody = targetCode
		if !strings.HasSuffix(updatedBody, "\n") && updatedBody != "" {
			updatedBody += "\n"
		}
		updatedBody += sectionContent
	}

	finalContent := mergedHeader
	if finalContent != "" {
		if !strings.HasSuffix(finalContent, "\n") {
			finalContent += "\n"
		}
		finalContent += "\n"
	}

	if targetPkg != "" {
		finalContent += targetPkg + "\n\n"
	}

	formattedImports := targetLanguage.FormatImports(mergedImports)
	if formattedImports != "" {
		finalContent += formattedImports + "\n\n"
	}

	finalContent += updatedBody

	if err := workspace.WriteTextFile(absTargetFilePath, finalContent); err != nil {
		return workspace.ToolErrorResult(err)
	}

	events.Emit(events.EventIntegrateEnded, "repo-cli", events.IntegratePayload{Source: sourcePath, TargetFile: targetFilePath, TargetSection: targetSectionName})
	output.Success(fmt.Sprintf("\n🧩️Integrated %s into %s section of %s", sourcePath, targetSectionName, targetFilePath))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolExtract MUST complete the operation successfully.
func ToolExtract(sourceFilePath, sourceSectionName, targetFilePath string) workspace.ToolResult {
	events.Emit(events.EventExtractStarting, "repo-cli", events.ExtractPayload{SourceFile: sourceFilePath, SourceSection: sourceSectionName, TargetFile: targetFilePath})
	output := workspace.NewOutput()

	absSourcePath := filepath.Join(workspace.RootDir, sourceFilePath)
	if !workspace.FileExists(absSourcePath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source file not found: %s", sourceFilePath))
	}
	sourceContent, err := workspace.ReadTextFile(absSourcePath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}

	sourceLanguage := languages.GetLanguage(sourceFilePath)
	if sourceLanguage == nil || !sourceLanguage.SupportsSections() {
		return workspace.ToolErrorMsg("Source file type does not support sections")
	}

	sections := sourceLanguage.ParseSections(sourceContent)
	section := languages.FindSection(sections, sourceSectionName)
	if section == nil {
		return workspace.ToolErrorMsg(fmt.Sprintf("Section not found: %s", sourceSectionName))
	}

	lines := strings.Split(sourceContent, "\n")
	if section.StartLine > len(lines) || section.EndLine > len(lines) {
		return workspace.ToolErrorMsg("Section range invalid")
	}

	var extractedLines []string
	if section.EndLine > section.StartLine {
		extractedLines = lines[section.StartLine : section.EndLine-1]
	}
	extractedBody := strings.Join(extractedLines, "\n")

	header, sourceBody := SplitHeader(sourceContent, sourceLanguage)
	_, sourceBodyNoPkg := sourceLanguage.ExtractPackage(sourceBody)
	imports, _ := sourceLanguage.ExtractImports(sourceBodyNoPkg)

	targetContent := ""
	if header != "" {
		targetContent += header + "\n\n"
	}

	pkgDecl, _ := sourceLanguage.ExtractPackage(sourceBody)
	if pkgDecl != "" {
		targetContent += pkgDecl + "\n\n"
	} else if len(imports) > 0 {

	}

	if len(imports) > 0 {
		formattedImports := sourceLanguage.FormatImports(imports)
		if formattedImports != "" {
			targetContent += formattedImports + "\n\n"
		}
	}

	targetContent += extractedBody
	if !strings.HasSuffix(targetContent, "\n") {
		targetContent += "\n"
	}

	absTargetFilePath := filepath.Join(workspace.RootDir, targetFilePath)
	if err := os.MkdirAll(filepath.Dir(absTargetFilePath), 0755); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := workspace.WriteTextFile(absTargetFilePath, targetContent); err != nil {
		return workspace.ToolErrorResult(err)
	}

	var newSourceLines []string
	if section.StartLine > 0 {
		newSourceLines = append(newSourceLines, lines[:section.StartLine-1]...)
	}
	if section.EndLine < len(lines) {
		newSourceLines = append(newSourceLines, lines[section.EndLine:]...)
	}
	newSourceContent := strings.Join(newSourceLines, "\n")

	if err := workspace.WriteTextFile(absSourcePath, newSourceContent); err != nil {

		return workspace.ToolErrorResult(err)
	}

	events.Emit(events.EventExtractEnded, "repo-cli", events.ExtractPayload{SourceFile: sourceFilePath, SourceSection: sourceSectionName, TargetFile: targetFilePath})
	output.Success(fmt.Sprintf("\n🧩️Extracted %s from %s to %s", sourceSectionName, sourceFilePath, targetFilePath))
	return workspace.ToolResult{Output: *output}
}

// 🎛️UpdateAgentsDocsPath MUST apply the update and return an error if the target is missing.
// ✏️UpdateAgentsDocsPath modifies an existing agents docs path entry.
func UpdateAgentsDocsPath(oldPath, newPath string) {
	agentsPath := filepath.Join(workspace.RootDir, "AGENTS.md")
	if !workspace.FileExists(agentsPath) {
		return
	}
	content, err := workspace.ReadTextFile(agentsPath)
	if err != nil {
		return
	}

	oldNorm := workspace.NormalizePath(oldPath)
	newNorm := workspace.NormalizePath(newPath)
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

	workspace.WriteTextFile(agentsPath, strings.Join(lines, "\n"))
}

// 🔖️SplitHeader MUST complete the operation successfully.
// ✂️SplitHeader splits the header into parts.
func SplitHeader(content string, lang languages.LanguagePlugin) (string, string) {
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
func MergeHeaders(targetHeader, sourceHeader string, lang languages.LanguagePlugin) string {
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

// #endregion 📋️Tickets
