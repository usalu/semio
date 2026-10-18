// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🆔️artifact_id carries the artifact identifier derivation and the compose identifier mappings.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🆔️Artifact Id

// 🏺️goalArtifactID holds the data fields for a goalArtifactID record.
func goalArtifactID(rawGoalID string) string {
	parts := strings.Split(rawGoalID, "/")
	result := ""
	for _, p := range parts {
		result += EmojiText(EmojiGoal) + workspace.Flat(p)
	}
	return result
}

// ⛳️goalPathToComposeID converts a filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI")
// 🖊️to the repo emoji ID format (e.g. "🎯️aioptimizedrepo🎯️repocli").
func GoalPathToComposeID(goalPath string) string {
	if goalPath == "" {
		return ""
	}
	return goalArtifactID(goalPath)
}

// 🛤️composeIDToGoalPath converts a repo emoji goal ID (e.g. "🎯️aioptimizedrepo🎯️repocli")
// back to its filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI") by scanning the
// 🔑️goals directory and matching each segment's Flat() value.
func ComposeIDToGoalPath(composeID string) string {
	if composeID == "" {
		return ""
	}
	goalEmoji := EmojiText(EmojiGoal)
	trimmed := composeID
	if strings.HasPrefix(trimmed, goalEmoji) {
		trimmed = trimmed[len(goalEmoji):]
	}
	segments := strings.Split(trimmed, goalEmoji)
	if len(segments) == 0 {
		return ""
	}

	dir := workspace.GetRepoGoalsDir()
	var pathParts []string
	currentDir := dir

	for _, seg := range segments {
		if seg == "" {
			continue
		}
		entries, err := os.ReadDir(currentDir)
		if err != nil {
			return composeID
		}
		found := false
		for _, entry := range entries {
			if entry.IsDir() && workspace.Flat(entry.Name()) == seg {
				pathParts = append(pathParts, entry.Name())
				currentDir = filepath.Join(currentDir, entry.Name())
				found = true
				break
			}
		}
		if !found {
			return composeID
		}
	}
	return strings.Join(pathParts, "/")
}

// 🤝️contributorGithubToComposeID converts a contributor identifier (alias or github) to the compose contributor ID format.
func contributorGithubToComposeID(identifier string) string {
	if identifier == "" || identifier == "unknown" {
		return identifier
	}
	prefix := EmojiText(EmojiContributor)
	if strings.HasPrefix(identifier, prefix) {
		return identifier
	}
	return prefix + workspace.Flat(identifier)
}

// 🐙️composeIDToContributorGithub converts a compose contributor ID back to a contributor alias.
func composeIDToContributorGithub(composeID string) string {
	if composeID == "" || composeID == "unknown" {
		return composeID
	}
	prefix := EmojiText(EmojiContributor)
	if !strings.HasPrefix(composeID, prefix) {
		return composeID
	}
	flat := composeID[len(prefix):]
	contributors, err := LookupContributors()
	if err == nil {
		for _, c := range contributors {
			if workspace.Flat(c.Alias) == flat {
				return c.Alias
			}
		}
	}
	return flat
}

// 📨️GetArtifactID MUST retrieve the requested value or return an error.
// 🟨️GetArtifactID retrieves and returns the artifact i d.
func GetArtifactID(kind string, data map[string]interface{}) string {
	parentId, _ := data["parentId"].(string)
	switch kind {
	case "root":
		return ""
	case "years":
		return parentId + EmojiText(EmojiYears)
	case "year":
		yy, _ := data["yy"].(string)
		return parentId + EmojiText(EmojiYear) + yy
	case "months":
		return parentId + EmojiText(EmojiMonths)
	case "month":
		mm, _ := data["mm"].(string)
		return parentId + EmojiText(EmojiMonth) + mm
	case "days":
		return parentId + EmojiText(EmojiDays)
	case "day":
		dd, _ := data["dd"].(string)
		return parentId + EmojiText(EmojiDay) + dd
	case "hours":
		return parentId + EmojiText(EmojiHours)
	case "hour":
		hh, _ := data["hh"].(string)
		return parentId + EmojiText(EmojiHour) + hh
	case "minutes":
		return parentId + EmojiText(EmojiMinutes)
	case "minute":
		mm, _ := data["mm"].(string)
		return parentId + EmojiText(EmojiMinute) + mm
	case "seconds":
		return parentId + EmojiText(EmojiSeconds)
	case "second":
		ss, _ := data["ss"].(string)
		return parentId + EmojiText(EmojiSecond) + ss
	case "codebase":
		return parentId + EmojiText(EmojiCodebase)
	case "technologies":
		return parentId + EmojiText(EmojiTechnologies)
	case "technology":
		name, _ := data["name"].(string)
		return technologyKindEmoji(data) + workspace.Flat(name)
	case "bundles":
		return parentId + EmojiText(EmojiBundles)
	case "bundle":
		name, _ := data["name"].(string)
		parts := strings.SplitN(name, "/", 2)
		technologyCode := parts[0]
		bundleCode := technologyCode
		if len(parts) > 1 {
			bundleCode = parts[1]
		}
		techEmoji := resolveTechnologyEmoji(technologyCode)
		return EmojiText(techEmoji) + workspace.Flat(technologyCode) + bundleKindEmoji(data) + workspace.Flat(bundleCode)
	case "folders":
		return parentId + EmojiText(EmojiFolders)
	case "folder":
		path, _ := data["path"].(string)
		name := filepath.Base(path)
		return parentId + folderKindEmoji(data) + workspace.Flat(name)
	case "files":
		return parentId + EmojiText(EmojiFiles)
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
		return parentId + fileKindEmoji(data) + workspace.Flat(name)
	case "sections":
		return parentId + EmojiText(EmojiSections)
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
			return parentId + EmojiText(EmojiSection) + workspace.Flat(sectionName)
		}
		hashIdx := strings.Index(path, "#")
		if hashIdx >= 0 {
			filePath := workspace.NormalizePath(path[:hashIdx])
			sectionPart := path[hashIdx+1:]
			sectionPath := strings.Split(sectionPart, "#")
			fileId := BuildFileID(filePath, nil)
			return BuildSectionID(fileId, sectionPath)
		}
		name, _ := data["name"].(string)
		if name != "" {
			return EmojiText(EmojiSection) + workspace.Flat(name)
		}
		return EmojiText(EmojiSection) + workspace.Flat(path)
	case "definitions":
		return parentId + EmojiText(EmojiDefinitions)
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
			return parentId + k + workspace.Flat(name)
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
			fileId := BuildFileID(filePath, nil)
			sectionId := fileId
			if sectionPath != "" {
				sections := strings.Split(strings.ReplaceAll(sectionPath, "#", "/"), "/")
				sectionId = BuildSectionID(fileId, sections)
			}
			return sectionId + k + workspace.Flat(name)
		}
		return k + workspace.Flat(name)
	case "tickets":
		return parentId + EmojiText(EmojiTickets)
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
		return parentId + EmojiText(EmojiTicket) + workspace.Flat(slug)
	case "goals":
		return parentId + EmojiText(EmojiGoals)
	case "goal":
		id, _ := data["id"].(string)
		name := id
		if idx := strings.LastIndex(id, "/"); idx >= 0 {
			name = id[idx+1:]
		}
		return parentId + EmojiText(EmojiGoal) + workspace.Flat(name)
	case "drafts":
		return parentId + EmojiText(EmojiDrafts)
	case "draft":
		slug, _ := data["slug"].(string)
		if slug == "" {
			slug, _ = data["id"].(string)
		}
		return parentId + EmojiText(EmojiDraft) + workspace.Flat(slug)
	case "todos":
		return parentId + EmojiText(EmojiTodos)
	case "todo":
		slug, _ := data["id"].(string)
		return parentId + EmojiText(EmojiTodo) + workspace.Flat(slug)
	case "policies":
		return parentId + EmojiText(EmojiPolicies)
	case "policy":
		slug, _ := data["id"].(string)
		slug = strings.TrimPrefix(slug, "/")
		return parentId + EmojiText(EmojiPolicy) + workspace.Flat(slug)
	case "statutes":
		return ""
	case "statute":
		id, _ := data["id"].(string)
		return workspace.Flat(id)
	case "contributors":
		return parentId + EmojiText(EmojiContributors)
	case "contributor":
		alias, _ := data["alias"].(string)
		if alias == "" {
			alias, _ = data["github"].(string)
		}
		return EmojiText(EmojiContributor) + workspace.Flat(alias)
	case "checkpoints":
		return parentId + EmojiText(EmojiCheckpoints)
	case "checkpoint":
		sha, _ := data["sha"].(string)
		contributorId, _ := data["contributorId"].(string)
		if contributorId == "" {
			if authorId, ok := data["authorId"].(string); ok && authorId != "" {
				contributorId = EmojiText(EmojiContributor) + workspace.Flat(authorId)
			}
		}
		return contributorId + EmojiText(EmojiCheckpoint) + sha
	case "line":
		lineNum, _ := data["line"].(float64)
		return parentId + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(lineNum))
	case "range":
		startLine, _ := data["startLine"].(float64)
		endLine, _ := data["endLine"].(float64)
		return parentId + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(startLine)) + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(endLine))
	case "breach":
		policyId := parentId
		affected, _ := data["affected"].(string)
		lineId, _ := data["lineId"].(string)
		secondId, _ := data["secondId"].(string)
		return policyId + EmojiText(EmojiBreach) + affected + EmojiText(EmojiBreachScope) + lineId + secondId
	case "breaches":
		return parentId + EmojiText(EmojiBreaches)
	case "interactions":
		return parentId + EmojiText(EmojiInteractions)
	case "interaction":
		k := interactionKindEmoji(data)
		secondId, _ := data["secondId"].(string)
		contributorId, _ := data["contributorId"].(string)
		entityID, _ := data["entityId"].(string)
		return secondId + contributorId + entityID + k
	case "sessions":
		return parentId + EmojiText(EmojiSessions)
	case "session":
		uuid, _ := data["uuid"].(string)
		if uuid == "" {
			uuid, _ = data["id"].(string)
		}
		return parentId + EmojiText(EmojiSession) + workspace.Flat(uuid)
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

// #endregion 🆔️Artifact Id
