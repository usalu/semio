// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Folder and file tools with header generation.

// #endregion 🧲️Header

package client

import (
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

// 🔖️ToolFolderCreate MUST complete the operation successfully.
func ToolFolderCreate(path string) ToolResult {
	repopkg.Emit(repopkg.EventFolderCreateStarting, "repo-cli", repopkg.FolderPayload{Path: path})
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("Folder already exists: %s", path))
	}
	if err := EnsureDir(absPath); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventFolderCreateEnded, "repo-cli", repopkg.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n📁️Created folder: %s", path))
	return ToolResult{Output: *output}
}

// 🔖️ToolFolderMove MUST complete the operation successfully.
func ToolFolderMove(source, target string) ToolResult {
	repopkg.Emit(repopkg.EventFolderMoveStarting, "repo-cli", repopkg.FolderPayload{Path: target, From: source})
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		return toolErrorMsg(fmt.Sprintf("Source folder not found: %s", source))
	}
	if FileExists(absTarget) {
		return toolErrorMsg(fmt.Sprintf("Target folder already exists: %s", target))
	}
	if refusal := refuseSealedMove(source); refusal != nil {
		return *refusal
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		return toolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return toolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	repopkg.Emit(repopkg.EventFolderMoveEnded, "repo-cli", repopkg.FolderPayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📁️Moved folder: %s → %s", source, target))
	return ToolResult{Output: *output}
}

// 🔖️ToolFolderDelete MUST complete the operation successfully.
func ToolFolderDelete(path string) ToolResult {
	repopkg.Emit(repopkg.EventFolderDeleteStarting, "repo-cli", repopkg.FolderPayload{Path: path})
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	if err := os.RemoveAll(absPath); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventFolderDeleteEnded, "repo-cli", repopkg.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted folder: %s", path))
	return ToolResult{Output: *output}
}

// 🔖️ToolFolderList MUST complete the operation successfully.
func ToolFolderList(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	folders, err := ListDirEntries(absPath, true)
	if err != nil {
		return toolErrorResult(err)
	}
	var relPaths []string
	for _, f := range folders {
		relPaths = append(relPaths, NormalizePath(filepath.Join(path, f)))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []string
	for _, f := range folders {
		relPath := NormalizePath(filepath.Join(path, f))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, f)
		}
	}
	output.Info(fmt.Sprintf("\n📁️Found %d folders in %s:\n", len(filtered), path))
	for _, f := range filtered {
		output.Plain(fmt.Sprintf("   %s/", f))
	}
	return ToolResult{Output: *output, Data: filtered}
}

// 🔖️ToolFolderTree MUST complete the operation successfully.
func ToolFolderTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📁️Folder tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}

// 🔖️printTree holds the data fields for a printTree record.
func printTree(output *CommandOutput, dir, prefix string) {
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
		relPaths = append(relPaths, GetRelativePath(filepath.Join(dir, e.Name())))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []os.DirEntry
	for _, e := range items {
		relPath := GetRelativePath(filepath.Join(dir, e.Name()))
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
			printTree(output, filepath.Join(dir, e.Name()), newPrefix)
		}
	}
}

// 🔖️ToolFileCreate MUST complete the operation successfully.
func ToolFileCreate(path string) ToolResult {
	repopkg.Emit(repopkg.EventFileCreateStarting, "repo-cli", repopkg.FilePayload{Path: path})
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File already exists: %s", path))
	}
	language := GetLanguage(path)
	content := generateFileHeader(path, language)
	if err := WriteTextFile(absPath, content); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventFileCreateEnded, "repo-cli", repopkg.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n📄️Created file: %s", path))
	return ToolResult{Output: *output}
}

// 🔖️FileHeaderId MUST complete the operation successfully.
func FileHeaderId(path string) string {
	kind := DeriveFileKind(filepath.Base(path))
	if kind == FileKindCode {
		absPath := filepath.Join(rootDir, path)
		if content, err := os.ReadFile(absPath); err == nil {
			text := string(content)
			if strings.HasPrefix(text, "#!") || strings.HasPrefix(text, "\xEF\xBB\xBF#!") {
				kind = FileKindScript
			}
		}
	}
	normalized := NormalizePath(path)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		parentID = resolveParentIDFromPath(dir)
	}
	data := map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID}
	return GetArtifactID("file", data)
}

// 🗃️AGPLLicenseText MUST complete the operation successfully.
func AGPLLicenseText() string {
	return `This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.`
}

// 🔗️FileHeaderUri MUST return the repo URI for a file path.
// ◾FileHeaderUri returns the artifact URI for the given file path.
func FileHeaderUri(path string) string {
	data := map[string]interface{}{"path": path}
	return GetArtifactURI("file", data)
}

// 🏺️SectionHeaderId MUST return the section artifact ID for a file path and section path.
// ◼SectionHeaderId returns the section identification string.
func SectionHeaderId(filePath string, sectionPath string) string {
	normalizedFilePath := NormalizePath(filePath)
	fileID := buildFileID(normalizedFilePath, nil)
	path := strings.ReplaceAll(sectionPath, "/", "#")
	parts := strings.Split(path, "#")
	return buildSectionID(fileID, parts)
}

// 🔖️SectionHeaderUri MUST return the repo URI for a section.
// 🔵️SectionHeaderUri returns the section artifact URI.
func SectionHeaderUri(filePath string, sectionPath string) string {
	path := filePath + "#" + sectionPath
	data := map[string]interface{}{"path": path}
	return GetArtifactURI("section", data)
}

// 🔖️DefinitionHeaderId MUST return the definition artifact ID for a file path, section path, and definition name.
// 🟡️DefinitionHeaderId returns the definition identification string.
func DefinitionHeaderId(filePath string, sectionPath string, name string, kind DefinitionKind) string {
	normalizedFilePath := NormalizePath(filePath)
	fileID := buildFileID(normalizedFilePath, nil)
	normalizedSectionPath := strings.ReplaceAll(sectionPath, "#", "/")
	var sectionParts []string
	if normalizedSectionPath != "" {
		sectionParts = strings.Split(normalizedSectionPath, "/")
	}
	return buildDefinitionID(fileID, sectionParts, name, kind)
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
	return GetArtifactURI("definition", data)
}

// 🔖️generateFileHeader holds the data fields for a generateFileHeader record.
func generateFileHeader(path string, language LanguagePlugin) string {
	if language == nil || !language.SupportsHeaders() {
		return ""
	}
	gitAuthor := GetGitAuthor()
	year := strconv.Itoa(time.Now().Year())
	contributors := year + " " + gitAuthor
	return language.FormatHeader(FileHeaderId(path), FileHeaderUri(path), "", contributors, AGPLLicenseText(), "")
}

// 🔖️ToolFileMove MUST complete the operation successfully.
func ToolFileMove(source, target string) ToolResult {
	repopkg.Emit(repopkg.EventFileMoveStarting, "repo-cli", repopkg.FilePayload{Path: target, From: source})
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		return toolErrorMsg(fmt.Sprintf("Source file not found: %s", source))
	}
	if FileExists(absTarget) {
		return toolErrorMsg(fmt.Sprintf("Target file already exists: %s", target))
	}
	if refusal := refuseSealedMove(source); refusal != nil {
		return *refusal
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		return toolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return toolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	repopkg.Emit(repopkg.EventFileMoveEnded, "repo-cli", repopkg.FilePayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📄️Moved file: %s → %s", source, target))
	return ToolResult{Output: *output}
}

// 🔖️ToolFileDelete MUST complete the operation successfully.
func ToolFileDelete(path string) ToolResult {
	repopkg.Emit(repopkg.EventFileDeleteStarting, "repo-cli", repopkg.FilePayload{Path: path})
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("File not found: %s", path))
	}
	if err := os.Remove(absPath); err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventFileDeleteEnded, "repo-cli", repopkg.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted file: %s", path))
	return ToolResult{Output: *output}
}

// 🔖️ToolFileList MUST complete the operation successfully.
func ToolFileList(scopeRaw string) ToolResult {
	output := NewOutput()
	scope := ParseScope(scopeRaw)
	bundles := GetTechnologies()
	files, err := ScopeToFiles(scope, bundles)
	if err != nil {
		return toolErrorResult(err)
	}
	output.Info(fmt.Sprintf("\n📄️Found %d files in scope \"%s\":\n", len(files), scopeRaw))
	for i, f := range files {
		if i >= 50 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(files)-50))
			break
		}
		output.Plain(fmt.Sprintf("   %s", f))
	}
	return ToolResult{Output: *output, Data: files}
}

// 🔖️ToolFileTree MUST complete the operation successfully.
func ToolFileTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		return toolErrorMsg(fmt.Sprintf("Path not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📄️File tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}
