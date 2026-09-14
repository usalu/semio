// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚚️move is the move domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package move

import (
	fmt "fmt"
	io "io"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"
	utf8 "unicode/utf8"

	codebase "github.com/usalu/semio/repo/codebase"
	events "github.com/usalu/semio/repo/events"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 🔤️Rename

// #region 🔤️Rename
// 🔤️applyRenameCasings rewrites UPPER, Title and lower case variants of old→new in the input string.
func ApplyRenameCasings(content, oldToken, newToken string) string {
	oldUpper := strings.ToUpper(oldToken)
	newUpper := strings.ToUpper(newToken)
	oldLower := strings.ToLower(oldToken)
	newLower := strings.ToLower(newToken)
	oldTitle := titleCaseToken(oldLower)
	newTitle := titleCaseToken(newLower)
	out := content
	if oldUpper != oldLower {
		out = strings.ReplaceAll(out, oldUpper, newUpper)
	}
	if oldTitle != oldUpper && oldTitle != oldLower {
		out = strings.ReplaceAll(out, oldTitle, newTitle)
	}
	out = strings.ReplaceAll(out, oldLower, newLower)
	return out
}

// 🅰️titleCaseToken returns s with the first rune upper-cased and the rest lower-cased.
func titleCaseToken(s string) string {
	if s == "" {
		return s
	}
	return strings.ToUpper(s[:1]) + strings.ToLower(s[1:])
}

// #endregion 🔤️Rename

// #region 📋️Tickets

// 🔖️ToolFolderCreate MUST complete the operation successfully.
func ToolFolderCreate(path string) workspace.ToolResult {
	events.Emit(events.EventFolderCreateStarting, "repo-cli", events.FolderPayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, path)
	if workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder already exists: %s", path))
	}
	if err := workspace.EnsureDir(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFolderCreateEnded, "repo-cli", events.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n📁️Created folder: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFolderMove MUST complete the operation successfully.
func ToolFolderMove(source, target string) workspace.ToolResult {
	events.Emit(events.EventFolderMoveStarting, "repo-cli", events.FolderPayload{Path: target, From: source})
	output := workspace.NewOutput()
	absSource := filepath.Join(workspace.RootDir, source)
	absTarget := filepath.Join(workspace.RootDir, target)
	if !workspace.FileExists(absSource) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source folder not found: %s", source))
	}
	if workspace.FileExists(absTarget) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Target folder already exists: %s", target))
	}
	if err := workspace.EnsureDir(filepath.Dir(absTarget)); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return workspace.ToolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	events.Emit(events.EventFolderMoveEnded, "repo-cli", events.FolderPayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📁️Moved folder: %s → %s", source, target))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFolderDelete MUST complete the operation successfully.
func ToolFolderDelete(path string) workspace.ToolResult {
	events.Emit(events.EventFolderDeleteStarting, "repo-cli", events.FolderPayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, path)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	if err := os.RemoveAll(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFolderDeleteEnded, "repo-cli", events.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted folder: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileCreate MUST complete the operation successfully.
func ToolFileCreate(path string) workspace.ToolResult {
	events.Emit(events.EventFileCreateStarting, "repo-cli", events.FilePayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, path)
	if workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File already exists: %s", path))
	}
	language := languages.GetLanguage(path)
	content := GenerateFileHeader(path, language)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFileCreateEnded, "repo-cli", events.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n📄️Created file: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️generateFileHeader holds the data fields for a generateFileHeader record.
func GenerateFileHeader(path string, language languages.LanguagePlugin) string {
	if language == nil || !language.SupportsHeaders() {
		return ""
	}
	gitAuthor := workspace.GetGitAuthor()
	year := strconv.Itoa(time.Now().Year())
	contributors := year + " " + gitAuthor
	return language.FormatHeader(codebase.FileHeaderId(path), codebase.FileHeaderUri(path), "", contributors, workspace.AGPLLicenseText(), "")
}

// 🔖️ToolFileMove MUST complete the operation successfully.
func ToolFileMove(source, target string) workspace.ToolResult {
	events.Emit(events.EventFileMoveStarting, "repo-cli", events.FilePayload{Path: target, From: source})
	output := workspace.NewOutput()
	absSource := filepath.Join(workspace.RootDir, source)
	absTarget := filepath.Join(workspace.RootDir, target)
	if !workspace.FileExists(absSource) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source file not found: %s", source))
	}
	if workspace.FileExists(absTarget) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Target file already exists: %s", target))
	}
	if err := workspace.EnsureDir(filepath.Dir(absTarget)); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return workspace.ToolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	events.Emit(events.EventFileMoveEnded, "repo-cli", events.FilePayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📄️Moved file: %s → %s", source, target))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileDelete MUST complete the operation successfully.
func ToolFileDelete(path string) workspace.ToolResult {
	events.Emit(events.EventFileDeleteStarting, "repo-cli", events.FilePayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, path)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", path))
	}
	if err := os.Remove(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFileDeleteEnded, "repo-cli", events.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted file: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolSectionCreate MUST complete the operation successfully.
func ToolSectionCreate(filePath, sectionPath string) workspace.ToolResult {
	events.Emit(events.EventSectionCreateStarting, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, filePath)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	language := languages.GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := languages.NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			return workspace.ToolErrorMsg("Section path required")
		}
		sectionName = pathParts[len(pathParts)-1]
		parentPath := strings.Join(pathParts[:len(pathParts)-1], "/")
		_, locations, err := languages.ParseJSONSectionsDetailed(content)
		if err != nil {
			return workspace.ToolErrorResult(err)
		}
		targetPath := strings.Join(pathParts, "/")
		if _, exists := locations[targetPath]; exists {
			return workspace.ToolErrorMsg(fmt.Sprintf("Section already exists: %s", targetPath))
		}
		objectStart, objectEnd, ok := languages.JsonFindObjectRange(content, locations, parentPath)
		if !ok {
			return workspace.ToolErrorMsg("Parent section is not a JSON object")
		}
		entry := fmt.Sprintf("%s: {}", strconv.Quote(sectionName))
		updated, inserted := languages.JsonInsertEntry(content, objectStart, objectEnd, entry)
		if !inserted {
			return workspace.ToolErrorMsg("Failed to insert section")
		}
		if err := workspace.WriteTextFile(absPath, updated); err != nil {
			return workspace.ToolErrorResult(err)
		}
		output.Success(fmt.Sprintf("\n🏷️Created section \"%s\" in %s", sectionName, filePath))
		return workspace.ToolResult{Output: *output}
	}
	if language == nil || !language.SupportsSections() {
		return workspace.ToolErrorMsg("Unsupported file type")
	}
	newSection := language.FormatSectionBoth(sectionName)
	if newSection == "" {
		return workspace.ToolErrorMsg("Cannot create section for this file type")
	}
	if err := workspace.WriteTextFile(absPath, content+newSection); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventSectionCreateEnded, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output.Success(fmt.Sprintf("\n🏷️Created section \"%s\" in %s", sectionName, filePath))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolSectionMove MUST complete the operation successfully.
func ToolSectionMove(filePath, oldPath, newPath string) workspace.ToolResult {
	events.Emit(events.EventSectionMoveStarting, "repo-cli", events.SectionPayload{File: filePath, Name: newPath, OldName: oldPath})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, filePath)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	oldParts := strings.Split(oldPath, "#")
	oldName := oldParts[len(oldParts)-1]
	newParts := strings.Split(newPath, "#")
	newName := newParts[len(newParts)-1]
	language := languages.GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		oldParts = languages.NormalizeSectionPath(oldPath)
		newParts = languages.NormalizeSectionPath(newPath)
		if len(oldParts) == 0 || len(newParts) == 0 {
			return workspace.ToolErrorMsg("Section path required")
		}
		oldPathNormalized := strings.Join(oldParts, "/")
		newPathNormalized := strings.Join(newParts, "/")
		_, locations, err := languages.ParseJSONSectionsDetailed(content)
		if err != nil {
			return workspace.ToolErrorResult(err)
		}
		source, ok := locations[oldPathNormalized]
		if !ok {
			return workspace.ToolErrorMsg(fmt.Sprintf("Section not found: %s", oldPathNormalized))
		}
		entry, start, end := languages.JsonExtractEntry(content, source.KeyStart, source.ValueEnd)
		updated := content[:start] + content[end:]
		_, updatedLocations, err := languages.ParseJSONSectionsDetailed(updated)
		if err != nil {
			return workspace.ToolErrorResult(err)
		}
		newName = newParts[len(newParts)-1]
		entry = languages.JsonRenameEntryKey(entry, newName)
		parentPath := strings.Join(newParts[:len(newParts)-1], "/")
		objectStart, objectEnd, ok := languages.JsonFindObjectRange(updated, updatedLocations, parentPath)
		if !ok {
			return workspace.ToolErrorMsg("Target section is not a JSON object")
		}
		entry = languages.JsonReindentEntry(entry, "")
		finalContent, inserted := languages.JsonInsertEntry(updated, objectStart, objectEnd, entry)
		if !inserted {
			return workspace.ToolErrorMsg("Failed to move section")
		}
		if err := workspace.WriteTextFile(absPath, finalContent); err != nil {
			return workspace.ToolErrorResult(err)
		}
		output.Success(fmt.Sprintf("\n🏷️Renamed section \"%s\" to \"%s\" in %s", oldPathNormalized, newPathNormalized, filePath))
		return workspace.ToolResult{Output: *output}
	}
	if language != nil && language.SupportsSections() {
		oldStart := language.FormatSectionStart(oldName)
		newStart := language.FormatSectionStart(newName)
		if oldStart != "" && newStart != "" {
			content = strings.ReplaceAll(content, oldStart, newStart)
		}
		oldEnd := language.FormatSectionEnd(oldName)
		newEnd := language.FormatSectionEnd(newName)
		if oldEnd != "" && newEnd != "" {
			content = strings.ReplaceAll(content, oldEnd, newEnd)
		}
		if language.Name() == "markdown" {
			content = strings.ReplaceAll(content, "# "+oldName, "# "+newName)
		}
	}
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventSectionMoveEnded, "repo-cli", events.SectionPayload{File: filePath, Name: newPath, OldName: oldPath})
	output.Success(fmt.Sprintf("\n🏷️Renamed section \"%s\" to \"%s\" in %s", oldName, newName, filePath))
	return workspace.ToolResult{Output: *output}
}

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

// 🔖️ToolSectionDelete MUST complete the operation successfully.
func ToolSectionDelete(filePath, sectionPath string) workspace.ToolResult {
	events.Emit(events.EventSectionDeleteStarting, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, filePath)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sections := languages.ParseSections(content, filePath)
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	section := languages.FindSection(sections, sectionName)
	language := languages.GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := languages.NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			return workspace.ToolErrorMsg("Section path required")
		}
		_, locations, err := languages.ParseJSONSectionsDetailed(content)
		if err != nil {
			return workspace.ToolErrorResult(err)
		}
		location, ok := locations[strings.Join(pathParts, "/")]
		if !ok {
			return workspace.ToolErrorMsg(fmt.Sprintf("Section not found: %s", strings.Join(pathParts, "/")))
		}
		_, start, end := languages.JsonExtractEntry(content, location.KeyStart, location.ValueEnd)
		updated := content[:start] + content[end:]
		if err := workspace.WriteTextFile(absPath, updated); err != nil {
			return workspace.ToolErrorResult(err)
		}
		events.Emit(events.EventSectionDeleteEnded, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
		output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", strings.Join(pathParts, "/"), filePath))
		return workspace.ToolResult{Output: *output}
	}
	if section == nil {
		return workspace.ToolErrorMsg(fmt.Sprintf("Section not found: %s", sectionName))
	}
	lines := strings.Split(content, "\n")
	var newLines []string
	for i, line := range lines {
		lineNum := i + 1
		if lineNum < section.StartLine || lineNum > section.EndLine {
			newLines = append(newLines, line)
		}
	}
	if err := workspace.WriteTextFile(absPath, strings.Join(newLines, "\n")); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventSectionDeleteEnded, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", sectionName, filePath))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolSectionList MUST complete the operation successfully.
func ToolSectionList(filePath string) workspace.ToolResult {
	output := workspace.NewOutput()
	scope := workspace.ParseScope(filePath)
	if scope.Kind != workspace.ScopeFile && scope.Kind != workspace.ScopeSection {
		return workspace.ToolErrorMsg("Scope must be a file or section")
	}
	absPath := filepath.Join(workspace.RootDir, scope.FilePath)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", scope.FilePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sections := languages.ParseSections(content, scope.FilePath)
	output.Info(fmt.Sprintf("\n🏷️Sections in %s:\n", scope.FilePath))
	var printSection func(s model.Section, indent string)
	printSection = func(s model.Section, indent string) {
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
	return workspace.ToolResult{Output: *output, Data: sections}
}

// 🔖️ToolDefinitionList MUST complete the operation successfully.
func ToolDefinitionList(filePath string) workspace.ToolResult {
	output := workspace.NewOutput()
	scope := workspace.ParseScope(filePath)
	absPath := filepath.Join(workspace.RootDir, scope.FilePath)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", scope.FilePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	definitions := codebase.FileDefinitions(content, scope.FilePath)
	output.Info(fmt.Sprintf("\n📋️ Definitions in %s:\n", scope.FilePath))
	for _, definition := range definitions {
		location := definition.SectionPath
		if location == "" {
			location = "(file)"
		}
		output.Plain(fmt.Sprintf("   %s%s [%s] in %s (lines %d-%d)", definition.Emoji, definition.Name, definition.Kind, location, definition.StartLine, definition.EndLine))
	}
	if len(definitions) == 0 {
		output.Plain("   (no definitions found)")
	}
	return workspace.ToolResult{Output: *output, Data: definitions}
}

// #endregion 📋️Tickets

// #region ⏰️File Utilities

// 🚚️MoveFile MUST return a non-nil error when the operation fails.
func MoveFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	outputFile, err := os.Create(destPath)
	if err != nil {
		inputFile.Close()
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	inputFile.Close()
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}

	err = os.Remove(sourcePath)
	if err != nil {
		return fmt.Errorf("failed removing original file: %v", err)
	}
	return nil
}

// ❌️CopyFile MUST return a non-nil error when the operation fails.
func CopyFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	defer inputFile.Close()
	outputFile, err := os.Create(destPath)
	if err != nil {
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}
	return nil
}

// 🔤️ToolRename rewrites every UPPER/Title/lower variant of oldToken to newToken across non-gitignored file contents and filenames. When scope is non-empty the walk is restricted to rootDir/scope.
func ToolRename(oldToken, newToken, scope string) workspace.ToolResult {
	output := workspace.NewOutput()
	if oldToken == "" || newToken == "" {
		return workspace.ToolErrorMsg("Old and new token must be non-empty")
	}
	if strings.EqualFold(oldToken, newToken) {
		return workspace.ToolErrorMsg("Old and new token are identical")
	}
	walkRoot := workspace.RootDir
	scope = strings.Trim(filepath.ToSlash(scope), "/")
	if scope != "" {
		walkRoot = filepath.Join(workspace.RootDir, filepath.FromSlash(scope))
		info, statErr := os.Stat(walkRoot)
		if statErr != nil || !info.IsDir() {
			return workspace.ToolErrorMsg(fmt.Sprintf("Scope is not a directory: %s", scope))
		}
	}
	var files []string
	var dirs []string
	err := filepath.WalkDir(walkRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		rel, relErr := filepath.Rel(workspace.RootDir, path)
		if relErr != nil {
			return nil
		}
		rel = filepath.ToSlash(rel)
		if rel == "." {
			return nil
		}
		base := filepath.Base(rel)
		if d.IsDir() && (base == ".git" || base == "node_modules") {
			return filepath.SkipDir
		}
		if workspace.IsGitIgnored(rel) {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.IsDir() {
			dirs = append(dirs, rel)
		} else {
			files = append(files, rel)
		}
		return nil
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	filesChanged := 0
	for _, rel := range files {
		abs := filepath.Join(workspace.RootDir, filepath.FromSlash(rel))
		data, readErr := os.ReadFile(abs)
		if readErr != nil {
			continue
		}
		if !utf8.Valid(data) {
			continue
		}
		original := string(data)
		replaced := ApplyRenameCasings(original, oldToken, newToken)
		if replaced == original {
			continue
		}
		info, statErr := os.Stat(abs)
		mode := os.FileMode(0644)
		if statErr == nil {
			mode = info.Mode().Perm()
		}
		if writeErr := os.WriteFile(abs, []byte(replaced), mode); writeErr != nil {
			return workspace.ToolErrorResult(writeErr)
		}
		filesChanged++
	}
	type renameEntry struct {
		rel   string
		isDir bool
	}
	entries := make([]renameEntry, 0, len(files)+len(dirs))
	for _, f := range files {
		entries = append(entries, renameEntry{f, false})
	}
	for _, d := range dirs {
		entries = append(entries, renameEntry{d, true})
	}
	sort.Slice(entries, func(i, j int) bool {
		di := strings.Count(entries[i].rel, "/")
		dj := strings.Count(entries[j].rel, "/")
		if di != dj {
			return di > dj
		}
		return entries[i].rel > entries[j].rel
	})
	filesRenamed := 0
	foldersRenamed := 0
	for _, e := range entries {
		base := filepath.Base(e.rel)
		newBase := ApplyRenameCasings(base, oldToken, newToken)
		if newBase == base {
			continue
		}
		parent := filepath.Dir(e.rel)
		var newRel string
		if parent == "." || parent == "" {
			newRel = newBase
		} else {
			newRel = parent + "/" + newBase
		}
		absSrc := filepath.Join(workspace.RootDir, filepath.FromSlash(e.rel))
		absDst := filepath.Join(workspace.RootDir, filepath.FromSlash(newRel))
		if _, statErr := os.Stat(absDst); statErr == nil {
			return workspace.ToolErrorMsg(fmt.Sprintf("Rename target already exists: %s", newRel))
		}
		if err := workspace.EnsureDir(filepath.Dir(absDst)); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if err := os.Rename(absSrc, absDst); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if e.isDir {
			foldersRenamed++
		} else {
			filesRenamed++
		}
	}
	output.Success(fmt.Sprintf("\n🔤️Renamed %s → %s: %d files edited, %d files renamed, %d folders renamed", oldToken, newToken, filesChanged, filesRenamed, foldersRenamed))
	return workspace.ToolResult{Output: *output, Data: map[string]int{
		"filesChanged":   filesChanged,
		"filesRenamed":   filesRenamed,
		"foldersRenamed": foldersRenamed,
	}}
}

// #endregion ⏰️File Utilities

// #region 🔖️Missing Hook Functions

func CopyDirTree(srcRoot, dstRoot string) error {
	return filepath.WalkDir(srcRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(srcRoot, path)
		if err != nil {
			return err
		}
		if rel == "." {
			return os.MkdirAll(dstRoot, 0o755)
		}
		target := filepath.Join(dstRoot, rel)
		if d.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		return CopyFile(path, target)
	})
}

// #endregion 🔖️Missing Hook Functions

// #endregion 🚚️Split
