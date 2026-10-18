// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Section, integrate, and extract tools.

// #endregion 🧲️Header

package client

import (
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// 🔖️ToolSectionCreate MUST complete the operation successfully.
func ToolSectionCreate(filePath, sectionPath string) ToolResult {
	repopkg.Emit(repopkg.EventSectionCreateStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		return toolErrorResult(err)
	}
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			return toolErrorMsg("Section path required")
		}
		sectionName = pathParts[len(pathParts)-1]
		parentPath := strings.Join(pathParts[:len(pathParts)-1], "/")
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			return toolErrorResult(err)
		}
		targetPath := strings.Join(pathParts, "/")
		if _, exists := locations[targetPath]; exists {
			return toolErrorMsg(fmt.Sprintf("Section already exists: %s", targetPath))
		}
		objectStart, objectEnd, ok := jsonFindObjectRange(content, locations, parentPath)
		if !ok {
			return toolErrorMsg("Parent section is not a JSON object")
		}
		entry := fmt.Sprintf("%s: {}", strconv.Quote(sectionName))
		updated, inserted := jsonInsertEntry(content, objectStart, objectEnd, entry)
		if !inserted {
			return toolErrorMsg("Failed to insert section")
		}
		if err := WriteTextFile(absPath, updated); err != nil {
			return toolErrorResult(err)
		}
		output.Success(fmt.Sprintf("\n🏷️Created section \"%s\" in %s", sectionName, filePath))
		return ToolResult{Output: *output}
	}
	if language == nil || !language.SupportsSections() {
		return toolErrorMsg("Unsupported file type")
	}
	newSection := language.FormatSectionBoth(sectionName)
	if newSection == "" {
		return toolErrorMsg("Cannot create section for this file type")
	}
	if err := WriteTextFile(absPath, content+newSection); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventSectionCreateEnded, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})
	output.Success(fmt.Sprintf("\n🏷️Created section \"%s\" in %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

// 🔖️ToolSectionMove MUST complete the operation successfully.
func ToolSectionMove(filePath, oldPath, newPath string) ToolResult {
	repopkg.Emit(repopkg.EventSectionMoveStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: newPath, OldName: oldPath})
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		return toolErrorResult(err)
	}
	oldParts := strings.Split(oldPath, "#")
	oldName := oldParts[len(oldParts)-1]
	newParts := strings.Split(newPath, "#")
	newName := newParts[len(newParts)-1]
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		oldParts = NormalizeSectionPath(oldPath)
		newParts = NormalizeSectionPath(newPath)
		if len(oldParts) == 0 || len(newParts) == 0 {
			return toolErrorMsg("Section path required")
		}
		oldPathNormalized := strings.Join(oldParts, "/")
		newPathNormalized := strings.Join(newParts, "/")
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			return toolErrorResult(err)
		}
		source, ok := locations[oldPathNormalized]
		if !ok {
			return toolErrorMsg(fmt.Sprintf("Section not found: %s", oldPathNormalized))
		}
		entry, start, end := jsonExtractEntry(content, source.KeyStart, source.ValueEnd)
		updated := content[:start] + content[end:]
		_, updatedLocations, err := ParseJSONSectionsDetailed(updated)
		if err != nil {
			return toolErrorResult(err)
		}
		newName = newParts[len(newParts)-1]
		entry = jsonRenameEntryKey(entry, newName)
		parentPath := strings.Join(newParts[:len(newParts)-1], "/")
		objectStart, objectEnd, ok := jsonFindObjectRange(updated, updatedLocations, parentPath)
		if !ok {
			return toolErrorMsg("Target section is not a JSON object")
		}
		entry = jsonReindentEntry(entry, "")
		finalContent, inserted := jsonInsertEntry(updated, objectStart, objectEnd, entry)
		if !inserted {
			return toolErrorMsg("Failed to move section")
		}
		if err := WriteTextFile(absPath, finalContent); err != nil {
			return toolErrorResult(err)
		}
		output.Success(fmt.Sprintf("\n🏷️Renamed section \"%s\" to \"%s\" in %s", oldPathNormalized, newPathNormalized, filePath))
		return ToolResult{Output: *output}
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
	if err := WriteTextFile(absPath, content); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventSectionMoveEnded, "repo-cli", repopkg.SectionPayload{File: filePath, Name: newPath, OldName: oldPath})
	output.Success(fmt.Sprintf("\n🏷️Renamed section \"%s\" to \"%s\" in %s", oldName, newName, filePath))
	return ToolResult{Output: *output}
}

// 🧬️ToolIntegrate MUST complete the operation successfully.
func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) ToolResult {
	repopkg.Emit(repopkg.EventIntegrateStarting, "repo-cli", repopkg.IntegratePayload{Source: sourcePath, TargetFile: targetFilePath, TargetSection: targetSectionName})
	output := NewOutput()

	absSourcePath := filepath.Join(rootDir, sourcePath)
	if !FileExists(absSourcePath) {
		return toolErrorMsg(fmt.Sprintf("Source file not found: %s", sourcePath))
	}
	sourceContent, err := ReadTextFile(absSourcePath)
	if err != nil {
		return toolErrorResult(err)
	}

	absTargetFilePath := filepath.Join(rootDir, targetFilePath)
	if !FileExists(absTargetFilePath) {
		return toolErrorMsg(fmt.Sprintf("Target file not found: %s", targetFilePath))
	}
	targetContent, err := ReadTextFile(absTargetFilePath)
	if err != nil {
		return toolErrorResult(err)
	}

	targetLanguage := GetLanguage(targetFilePath)
	if targetLanguage == nil || !targetLanguage.SupportsSections() {
		return toolErrorMsg("Target file type does not support sections")
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

	mergedImports := UniqueStrings(append(targetImports, sourceImports...))

	startMarker := targetLanguage.FormatSectionStart(targetSectionName)
	endMarker := targetLanguage.FormatSectionEnd(targetSectionName)

	if !strings.HasSuffix(sourceCode, "\n") && sourceCode != "" {
		sourceCode += "\n"
	}

	sectionContent := "\n" + startMarker + "\n" + sourceCode + endMarker + "\n"

	var updatedBody string
	if targetParentSectionName != "" {

		sections := targetLanguage.ParseSections(targetCode)
		parentSection := FindSection(sections, targetParentSectionName)
		if parentSection == nil {
			return toolErrorMsg(fmt.Sprintf("Parent section not found: %s", targetParentSectionName))
		}

		if parentSection.EndLine == -1 {
			return toolErrorMsg(fmt.Sprintf("Parent section %s is not properly closed", targetParentSectionName))
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

	if err := WriteTextFile(absTargetFilePath, finalContent); err != nil {
		return toolErrorResult(err)
	}

	repopkg.Emit(repopkg.EventIntegrateEnded, "repo-cli", repopkg.IntegratePayload{Source: sourcePath, TargetFile: targetFilePath, TargetSection: targetSectionName})
	output.Success(fmt.Sprintf("\n🧩️Integrated %s into %s section of %s", sourcePath, targetSectionName, targetFilePath))
	return ToolResult{Output: *output}
}

// 🔖️ToolExtract MUST complete the operation successfully.
func ToolExtract(sourceFilePath, sourceSectionName, targetFilePath string) ToolResult {
	repopkg.Emit(repopkg.EventExtractStarting, "repo-cli", repopkg.ExtractPayload{SourceFile: sourceFilePath, SourceSection: sourceSectionName, TargetFile: targetFilePath})
	output := NewOutput()

	absSourcePath := filepath.Join(rootDir, sourceFilePath)
	if !FileExists(absSourcePath) {
		return toolErrorMsg(fmt.Sprintf("Source file not found: %s", sourceFilePath))
	}
	sourceContent, err := ReadTextFile(absSourcePath)
	if err != nil {
		return toolErrorResult(err)
	}

	sourceLanguage := GetLanguage(sourceFilePath)
	if sourceLanguage == nil || !sourceLanguage.SupportsSections() {
		return toolErrorMsg("Source file type does not support sections")
	}

	sections := sourceLanguage.ParseSections(sourceContent)
	section := FindSection(sections, sourceSectionName)
	if section == nil {
		return toolErrorMsg(fmt.Sprintf("Section not found: %s", sourceSectionName))
	}

	lines := strings.Split(sourceContent, "\n")
	if section.StartLine > len(lines) || section.EndLine > len(lines) {
		return toolErrorMsg("Section range invalid")
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

	absTargetFilePath := filepath.Join(rootDir, targetFilePath)
	if err := os.MkdirAll(filepath.Dir(absTargetFilePath), 0755); err != nil {
		return toolErrorResult(err)
	}
	if err := WriteTextFile(absTargetFilePath, targetContent); err != nil {
		return toolErrorResult(err)
	}

	var newSourceLines []string
	if section.StartLine > 0 {
		newSourceLines = append(newSourceLines, lines[:section.StartLine-1]...)
	}
	if section.EndLine < len(lines) {
		newSourceLines = append(newSourceLines, lines[section.EndLine:]...)
	}
	newSourceContent := strings.Join(newSourceLines, "\n")

	if err := WriteTextFile(absSourcePath, newSourceContent); err != nil {

		return toolErrorResult(err)
	}

	repopkg.Emit(repopkg.EventExtractEnded, "repo-cli", repopkg.ExtractPayload{SourceFile: sourceFilePath, SourceSection: sourceSectionName, TargetFile: targetFilePath})
	output.Success(fmt.Sprintf("\n🧩️Extracted %s from %s to %s", sourceSectionName, sourceFilePath, targetFilePath))
	return ToolResult{Output: *output}
}
