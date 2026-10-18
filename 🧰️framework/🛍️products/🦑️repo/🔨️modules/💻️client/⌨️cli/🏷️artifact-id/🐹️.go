// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact id and URI computation.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"strings"
)

// 📨️GetArtifactID MUST retrieve the requested value or return an error.
// 🟨️GetArtifactID retrieves and returns the artifact i d.
func GetArtifactID(kind string, data map[string]interface{}) string {
	parentId, _ := data["parentId"].(string)
	switch kind {
	case "root":
		return ""
	case "years":
		return parentId + emojiText(EmojiYears)
	case "year":
		yy, _ := data["yy"].(string)
		return parentId + emojiText(EmojiYear) + yy
	case "months":
		return parentId + emojiText(EmojiMonths)
	case "month":
		mm, _ := data["mm"].(string)
		return parentId + emojiText(EmojiMonth) + mm
	case "days":
		return parentId + emojiText(EmojiDays)
	case "day":
		dd, _ := data["dd"].(string)
		return parentId + emojiText(EmojiDay) + dd
	case "hours":
		return parentId + emojiText(EmojiHours)
	case "hour":
		hh, _ := data["hh"].(string)
		return parentId + emojiText(EmojiHour) + hh
	case "minutes":
		return parentId + emojiText(EmojiMinutes)
	case "minute":
		mm, _ := data["mm"].(string)
		return parentId + emojiText(EmojiMinute) + mm
	case "seconds":
		return parentId + emojiText(EmojiSeconds)
	case "second":
		ss, _ := data["ss"].(string)
		return parentId + emojiText(EmojiSecond) + ss
	case "codebase":
		return parentId + emojiText(EmojiCodebase)
	case "technologies":
		return parentId + emojiText(EmojiTechnologies)
	case "technology":
		name, _ := data["name"].(string)
		return technologyKindEmoji(data) + Flat(name)
	case "bundles":
		return parentId + emojiText(EmojiBundles)
	case "bundle":
		name, _ := data["name"].(string)
		parts := strings.SplitN(name, "/", 2)
		technologyCode := parts[0]
		bundleCode := technologyCode
		if len(parts) > 1 {
			bundleCode = parts[1]
		}
		techEmoji := resolveTechnologyEmoji(technologyCode)
		return emojiText(techEmoji) + Flat(technologyCode) + bundleKindEmoji(data) + Flat(bundleCode)
	case "folders":
		return parentId + emojiText(EmojiFolders)
	case "folder":
		path, _ := data["path"].(string)
		name := filepath.Base(path)
		return parentId + folderKindEmoji(data) + Flat(name)
	case "files":
		return parentId + emojiText(EmojiFiles)
	case "file":
		path, _ := data["path"].(string)
		if path == "" {
			path, _ = data["id"].(string)
		}
		name := filepath.Base(path)
		ext := filepath.Ext(name)
		if ext != "" {
			name = strings.TrimSuffix(name, ext)
		}
		return parentId + fileKindEmoji(data) + Flat(name)
	case "sections":
		return parentId + emojiText(EmojiSections)
	case "section":
		path, _ := data["path"].(string)
		if path == "" {
			filePath, _ := data["filePath"].(string)
			name, _ := data["name"].(string)
			if filePath != "" && name != "" {
				path = filePath + "#" + name
			} else if name != "" {
				path = name
			}
		}
		if parentId != "" {
			hashIdx := strings.LastIndex(path, "#")
			sectionName := path
			if hashIdx >= 0 {
				sectionName = path[hashIdx+1:]
			}
			return parentId + emojiText(EmojiSection) + Flat(sectionName)
		}
		hashIdx := strings.Index(path, "#")
		if hashIdx >= 0 {
			filePath := NormalizePath(path[:hashIdx])
			sectionPart := path[hashIdx+1:]
			sectionPath := strings.Split(sectionPart, "#")
			fileId := buildFileID(filePath, nil)
			return buildSectionID(fileId, sectionPath)
		}
		name, _ := data["name"].(string)
		if name != "" {
			return emojiText(EmojiSection) + Flat(name)
		}
		return emojiText(EmojiSection) + Flat(path)
	case "definitions":
		return parentId + emojiText(EmojiDefinitions)
	case "definition":
		k := definitionKindEmoji(data)
		name, _ := data["name"].(string)
		if name == "" {
			id, _ := data["id"].(string)
			if id != "" {
				paragraphIdx := strings.LastIndex(id, "§")
				if paragraphIdx >= 0 {
					name = id[paragraphIdx+len("§"):]
				} else {
					hashIdx := strings.LastIndex(id, "#")
					if hashIdx >= 0 {
						name = id[hashIdx+1:]
					} else {
						name = id
					}
				}
			}
		}
		if parentId != "" {
			return parentId + k + Flat(name)
		}
		sectionPath, _ := data["sectionPath"].(string)
		filePath, _ := data["filePath"].(string)
		if filePath == "" {
			id, _ := data["id"].(string)
			if id != "" {
				if paragraphIdx := strings.LastIndex(id, "§"); paragraphIdx >= 0 {
					name = id[paragraphIdx+len("§"):]
					if hashIdx := strings.Index(id[:paragraphIdx], "#"); hashIdx >= 0 {
						filePath = id[:hashIdx]
						sectionPath = strings.ReplaceAll(id[hashIdx+1:paragraphIdx], "#", "/")
					} else {
						filePath = id[:paragraphIdx]
					}
				} else if hashIdx := strings.LastIndex(id, "#"); hashIdx >= 0 {
					name = id[hashIdx+1:]
					filePath = id[:hashIdx]
				}
			}
		}
		if filePath != "" {
			fileId := buildFileID(filePath, nil)
			sectionId := fileId
			if sectionPath != "" {
				sections := strings.Split(strings.ReplaceAll(sectionPath, "#", "/"), "/")
				sectionId = buildSectionID(fileId, sections)
			}
			return sectionId + k + Flat(name)
		}
		return k + Flat(name)
	case "tickets":
		return parentId + emojiText(EmojiTickets)
	case "ticket":
		if parentId == "" {
			if goalId, ok := data["goalId"].(string); ok && goalId != "" {
				parentId = goalArtifactID(goalId)
			}
		}
		slug, _ := data["slug"].(string)
		title, _ := data["title"].(string)
		if slug == "" && title != "" {
			slug = title
		}
		if slug == "" {
			if id, ok := data["id"].(string); ok && id != "" {
				return id
			}
		}
		return parentId + emojiText(EmojiTicket) + Flat(slug)
	case "goals":
		return parentId + emojiText(EmojiGoals)
	case "goal":
		id, _ := data["id"].(string)
		name := id
		if idx := strings.LastIndex(id, "/"); idx >= 0 {
			name = id[idx+1:]
		}
		return parentId + emojiText(EmojiGoal) + Flat(name)
	case "drafts":
		return parentId + emojiText(EmojiDrafts)
	case "draft":
		slug, _ := data["slug"].(string)
		if slug == "" {
			slug, _ = data["id"].(string)
		}
		return parentId + emojiText(EmojiDraft) + Flat(slug)
	case "todos":
		return parentId + emojiText(EmojiTodos)
	case "todo":
		slug, _ := data["id"].(string)
		return parentId + emojiText(EmojiTodo) + Flat(slug)
	case "policies":
		return parentId + emojiText(EmojiPolicies)
	case "policy":
		slug, _ := data["id"].(string)
		slug = strings.TrimPrefix(slug, "/")
		return parentId + emojiText(EmojiPolicy) + Flat(slug)
	case "statutes":
		return ""
	case "statute":
		id, _ := data["id"].(string)
		return Flat(id)
	case "contributors":
		return parentId + emojiText(EmojiContributors)
	case "contributor":
		alias, _ := data["alias"].(string)
		if alias == "" {
			alias, _ = data["github"].(string)
		}
		return emojiText(EmojiContributor) + Flat(alias)
	case "checkpoints":
		return parentId + emojiText(EmojiCheckpoints)
	case "checkpoint":
		sha, _ := data["sha"].(string)
		contributorId, _ := data["contributorId"].(string)
		if contributorId == "" {
			if authorId, ok := data["authorId"].(string); ok && authorId != "" {
				contributorId = emojiText(EmojiContributor) + Flat(authorId)
			}
		}
		return contributorId + emojiText(EmojiCheckpoint) + sha
	case "line":
		lineNum, _ := data["line"].(float64)
		return parentId + emojiText(EmojiLine) + fmt.Sprintf("%d", int(lineNum))
	case "range":
		startLine, _ := data["startLine"].(float64)
		endLine, _ := data["endLine"].(float64)
		return parentId + emojiText(EmojiLine) + fmt.Sprintf("%d", int(startLine)) + emojiText(EmojiLine) + fmt.Sprintf("%d", int(endLine))
	case "breach":
		policyId := parentId
		affected, _ := data["affected"].(string)
		lineId, _ := data["lineId"].(string)
		secondId, _ := data["secondId"].(string)
		return policyId + emojiText(EmojiBreach) + affected + emojiText(EmojiBreachScope) + lineId + secondId
	case "breaches":
		return parentId + emojiText(EmojiBreaches)
	case "interactions":
		return parentId + emojiText(EmojiInteractions)
	case "interaction":
		k := interactionKindEmoji(data)
		secondId, _ := data["secondId"].(string)
		contributorId, _ := data["contributorId"].(string)
		entityID, _ := data["entityId"].(string)
		return secondId + contributorId + entityID + k
	case "sessions":
		return parentId + emojiText(EmojiSessions)
	case "session":
		uuid, _ := data["uuid"].(string)
		if uuid == "" {
			uuid, _ = data["id"].(string)
		}
		return parentId + emojiText(EmojiSession) + Flat(uuid)
	}
	return ""
}

// ▫️GetArtifactURI MUST retrieve the requested value or return an error.
// 🌐️GetArtifactURI retrieves and returns the artifact URI using the format repo://{{entity-kind}}/{{id}}.
func GetArtifactURI(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	if id == "" {
		return "repo://" + kind
	}
	return "repo://" + kind + "/" + id
}

// 🏗️buildFolderUriFromPath holds the data fields for a buildFolderUriFromPath record.
func buildFolderUriFromPath(path string) string {
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
func buildFileUriFromPath(path string) string {
	id := buildFileID(path, nil)
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
