// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧷️artifact_uri carries the uri builders and the uri to identifier conversions.

// #endregion 🧲️Header

package model

import (
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧷️Artifact Uri

// 🏗️buildFolderUriFromPath holds the data fields for a buildFolderUriFromPath record.
func BuildFolderUriFromPath(path string) string {
	return GetArtifactURI("folder", map[string]interface{}{"path": path})
}

// ◾extractPathFromFolderUri extracts the filesystem path from a folder URI.
// 🟩️With the new repo://folder/{{id}} format, it resolves the emoji ID back to a path.
func extractPathFromFolderUri(uri string) string {
	id := UriToId("repo://" + uri)
	if id == "" {
		id = UriToId(uri)
	}
	if id == "" {
		return ""
	}
	return IdToPath(id)
}

// ◽extractPathFromFileUri extracts the filesystem path from a file URI.
// 🟫️With the new repo://file/{{id}} format, it resolves the emoji ID back to a path.
func extractPathFromFileUri(uri string) string {
	id := UriToId("repo://" + uri)
	if id == "" {
		id = UriToId(uri)
	}
	if id == "" {
		return ""
	}
	return IdToPath(id)
}

// ◻buildFileUriFromPath builds a file URI from a filesystem path.
func BuildFileUriFromPath(path string) string {
	id := BuildFileID(path, nil)
	if id == "" {
		return GetArtifactURI("file", map[string]interface{}{"path": path})
	}
	return "repo://file/" + id
}

// 📋️buildSectionUriFromPath builds a section URI from a path#section format.
func buildSectionUriFromPath(path string) string {
	return GetArtifactURI("section", map[string]interface{}{"path": path})
}

// ◼buildDefinitionUriFromIdValue builds a definition URI from an id value.
func buildDefinitionUriFromIdValue(id string, dkc string) string {
	return GetArtifactURI("definition", map[string]interface{}{"id": id})
}

// 🔵️IdToUri converts an emoji-based ID to a repo:// URI.
// ▫️New format: repo://{{entity-kind}}/{{id}}
func IdToUri(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	normalized = strings.TrimSpace(normalized)
	if normalized == "" {
		return ""
	}
	kind := DetectEntityKindFromId(normalized)
	if kind == "" {
		return ""
	}
	return "repo://" + kind + "/" + id
}

// 🟠️UriToId converts a repo:// URI to an emoji-based ID.
// ◾New format: repo://{{entity-kind}}/{{id}} → {{id}}
func UriToId(uri string) string {
	if !strings.HasPrefix(uri, "repo://") {
		return ""
	}
	p := strings.TrimPrefix(uri, "repo://")
	if p == "" || p == "root" {
		return ""
	}
	// Find the first / to split kind from id
	slashIdx := strings.Index(p, "/")
	if slashIdx < 0 {
		// No id part, just the kind — return empty for entity kinds
		return ""
	}
	id := p[slashIdx+1:]
	return id
}

// 🩶️uriSubPathToId holds the data fields for a uriSubPathToId record.
func uriSubPathToId(parentId string, subPath string) string {
	if subPath == "fds" {
		return EmojiText(EmojiFolders)
	}
	if strings.HasPrefix(subPath, "fd/") {
		fdRest := strings.TrimPrefix(subPath, "fd/")
		fdParts := strings.SplitN(fdRest, "/", 3)
		if len(fdParts) < 2 {
			return ""
		}
		fEmoji := folderEmojiFromCode(fdParts[0])
		fName := fdParts[1]
		folderId := parentId
		_ = fEmoji
		_ = fName
		if len(fdParts) == 2 {
			return EmojiText(fEmoji) + workspace.Flat(fName)
		}
		return uriSubPathToId(folderId, fdParts[2])
	}
	if subPath == "fis" {
		return EmojiText(EmojiFiles)
	}
	if strings.HasPrefix(subPath, "f/") {
		fRest := strings.TrimPrefix(subPath, "f/")
		fParts := strings.SplitN(fRest, "/", 2)
		fileName := workspace.PathFromUriPath(fParts[0])
		ext := filepath.Ext(fileName)
		if ext != "" {
			fileName = strings.TrimSuffix(fileName, ext)
		}
		fileId := EmojiText(EmojiFileCode) + workspace.Flat(fileName)
		if len(fParts) == 1 {
			return fileId
		}
		return uriSubPathToIdFromFile(fileId, fParts[1])
	}
	if subPath == "ss" {
		return EmojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		sectionParts := splitSectionUri(sRest)
		result := parentId
		for _, sp := range sectionParts {
			result += EmojiText(EmojiSection) + workspace.PathFromUriPath(sp)
		}
		return result
	}
	if subPath == "ds" {
		return EmojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return parentId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	return ""
}

// 🩷️uriSubPathToIdFromFile holds the data fields for a uriSubPathToIdFromFile record.
func uriSubPathToIdFromFile(fileId string, subPath string) string {
	if subPath == "ss" {
		return EmojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		return parseSectionAndRest(fileId, sRest)
	}
	if subPath == "ds" {
		return EmojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return fileId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	return ""
}

func parseSectionAndRest(parentId string, sRest string) string {
	parts := strings.SplitN(sRest, "/s/", 2)
	sectionName := workspace.PathFromUriPath(parts[0])
	if dIdx := strings.Index(parts[0], "/d/"); dIdx >= 0 {
		sectionName = workspace.PathFromUriPath(parts[0][:dIdx])
		sectionId := parentId + EmojiText(EmojiSection) + sectionName
		dRest := parts[0][dIdx+3:]
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return sectionId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	sectionId := parentId + EmojiText(EmojiSection) + sectionName
	if len(parts) == 1 {
		return sectionId
	}
	return parseSectionAndRest(sectionId, parts[1])
}

// 💜️splitSectionUri holds the data fields for a splitSectionUri record.
func splitSectionUri(path string) []string {
	return strings.Split(path, "/s/")
}

func parseGoalUri(rest string) string {
	parts := strings.SplitN(rest, "/", 2)
	goalName := parts[0]
	result := EmojiText(EmojiGoal) + workspace.Flat(goalName)
	if len(parts) == 1 {
		return result
	}
	sub := parts[1]
	if strings.HasPrefix(sub, "g/") {
		return result + parseGoalUri(strings.TrimPrefix(sub, "g/"))
	}
	if strings.HasPrefix(sub, "tk/") {
		return EmojiText(EmojiTicket) + workspace.Flat(strings.TrimPrefix(sub, "tk/"))
	}
	if sub == "tks" {
		return EmojiText(EmojiTickets)
	}
	if sub == "gs" {
		return EmojiText(EmojiGoals)
	}
	return result
}

// #endregion 🧷️Artifact Uri
