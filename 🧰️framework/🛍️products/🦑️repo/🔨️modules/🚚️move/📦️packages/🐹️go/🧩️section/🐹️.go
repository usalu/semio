// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧩️section carries the section and definition tool functions of the move domain.

// #endregion 🧲️Header

package move

import (
	fmt "fmt"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	events "github.com/usalu/semio/repo/events"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📋️Tickets

// 🔖️ToolSectionCreate MUST complete the operation successfully.
func ToolSectionCreate(filePath, sectionPath string) workspace.ToolResult {
	events.Emit(events.EventSectionCreateStarting, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), filePath)
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
	absPath := filepath.Join(workspace.GetRootDir(), filePath)
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

// 🔖️ToolSectionDelete MUST complete the operation successfully.
func ToolSectionDelete(filePath, sectionPath string) workspace.ToolResult {
	events.Emit(events.EventSectionDeleteStarting, "repo-cli", events.SectionPayload{File: filePath, Name: sectionPath})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), filePath)
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
	absPath := filepath.Join(workspace.GetRootDir(), scope.FilePath)
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
	absPath := filepath.Join(workspace.GetRootDir(), scope.FilePath)
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
