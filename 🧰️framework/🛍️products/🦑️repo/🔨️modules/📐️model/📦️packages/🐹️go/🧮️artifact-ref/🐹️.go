// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧮️artifact_ref carries the artifact reference parsing and the uri path conversions.

// #endregion 🧲️Header

package model

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧮️Artifact Ref

// 📑️containsUriSection holds the data fields for a containsUriSection record.
func containsUriSection(uri string) bool {
	return strings.HasPrefix(uri, "repo://section/")
}

// 🔗️containsUriDefinition holds the data fields for a containsUriDefinition record.
func containsUriDefinition(uri string) bool {
	return strings.HasPrefix(uri, "repo://definition/")
}

// 🟨️extractFileAndSectionsFromUri extracts the file path and section names from a URI.
// 🆕️Supports new format repo://section/{{id}} where ID encodes file+sections as emojis.
func extractFileAndSectionsFromUri(uri string) (filePath string, sectionSlugs []string) {
	// New format: repo://section/{{id}}
	if strings.HasPrefix(uri, "repo://section/") || strings.HasPrefix(uri, "repo://file/") {
		id := UriToId(uri)
		if id == "" {
			return "", nil
		}
		filePath = IdToPath(id)
		sectionSlugs = IdToSectionPath(id)
		return filePath, sectionSlugs
	}
	// Try the raw string as an ID (for cases where "repo://" was already stripped)
	prefixStripped := strings.TrimPrefix(uri, "repo://")
	if strings.HasPrefix(prefixStripped, "section/") || strings.HasPrefix(prefixStripped, "file/") {
		fullUri := "repo://" + prefixStripped
		id := UriToId(fullUri)
		if id != "" {
			return IdToPath(id), IdToSectionPath(id)
		}
	}
	// Legacy fallback: parse old /f/ and /s/ format
	p := prefixStripped
	var sectionParts []string
	remaining := p
	for {
		idx := strings.Index(remaining, "/s/")
		if idx < 0 {
			break
		}
		after := remaining[idx+3:]
		nextS := strings.Index(after, "/s/")
		nextD := strings.Index(after, "/d/")
		endIdx := len(after)
		if nextS >= 0 && nextS < endIdx {
			endIdx = nextS
		}
		if nextD >= 0 && nextD < endIdx {
			endIdx = nextD
		}
		sectionParts = append(sectionParts, workspace.PathFromUriPath(after[:endIdx]))
		remaining = after[endIdx:]
	}
	fIdx := strings.Index(p, "/f/")
	if fIdx >= 0 {
		rest := p[fIdx+3:]
		nextSlash := strings.Index(rest, "/")
		if nextSlash < 0 {
			filePath = workspace.PathFromUriPath(rest)
		} else {
			filePath = workspace.PathFromUriPath(rest[:nextSlash])
		}
	}
	return filePath, sectionParts
}

// 🟩️extractFileAndDefinitionFromUri extracts the file path and definition name from a URI.
// ▶️Supports new format repo://definition/{{id}} where ID encodes file+sections+definition as emojis.
func extractFileAndDefinitionFromUri(uri string) (filePath string, defName string) {
	// New format: repo://definition/{{id}}
	if strings.HasPrefix(uri, "repo://definition/") {
		id := UriToId(uri)
		if id == "" {
			return "", ""
		}
		filePath = IdToPath(id)
		defName = IdToDefinitionName(id)
		return filePath, defName
	}
	// Try with the raw string (for cases where "repo://" was already stripped)
	prefixStripped := strings.TrimPrefix(uri, "repo://")
	if strings.HasPrefix(prefixStripped, "definition/") {
		fullUri := "repo://" + prefixStripped
		id := UriToId(fullUri)
		if id != "" {
			return IdToPath(id), IdToDefinitionName(id)
		}
	}
	// Legacy fallback: parse old /f/ and /d/ format
	p := prefixStripped
	fIdx := strings.Index(p, "/f/")
	if fIdx >= 0 {
		rest := p[fIdx+3:]
		nextSlash := strings.Index(rest, "/")
		if nextSlash < 0 {
			filePath = workspace.PathFromUriPath(rest)
		} else {
			filePath = workspace.PathFromUriPath(rest[:nextSlash])
		}
	}
	dIdx := strings.LastIndex(p, "/d/")
	if dIdx >= 0 {
		rest := p[dIdx+3:]
		dParts := strings.SplitN(rest, "/", 2)
		if len(dParts) >= 2 {
			defName = workspace.PathFromUriPath(dParts[1])
		}
	}
	return filePath, defName
}

// 🟦️ArtifactRef holds the data fields for a artifact ref record.
type ArtifactRef struct {
	Kind         string
	Path         string
	SectionParts []string
}

// ❌️ParseArtifactRef MUST return the parsed result or an error for invalid input.
// 💾️ParseArtifactRef parses and returns the artifact ref from the input.
func ParseArtifactRef(ref string) ArtifactRef {
	clean := strings.ReplaceAll(ref, "\uFE0E", "")
	clean = strings.ReplaceAll(clean, "\uFE0F", "")
	if uri := IdToUri(clean); uri != "" && strings.HasPrefix(uri, "repo://") {
		if containsUriSection(uri) && !containsUriDefinition(uri) {
			filePath, sectionSlugs := extractFileAndSectionsFromUri(uri)
			return ArtifactRef{Kind: "section", Path: workspace.NormalizePath(filePath), SectionParts: sectionSlugs}
		}
		if containsUriDefinition(uri) {
			filePath, defName := extractFileAndDefinitionFromUri(uri)
			if defName != "" {
				return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(filePath + "§" + defName)}
			}
			return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(filePath)}
		}
	}

	sectionEmojis := []string{"🔖️"}
	for _, e := range sectionEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			parts := strings.SplitN(rest, "#", 2)
			filePath := workspace.NormalizePath(parts[0])
			var sectionParts []string
			if len(parts) > 1 {
				sectionParts = strings.Split(parts[1], "#")
			}
			return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
		}
	}

	defEmojis := []string{"🏷️", "✂️", "🪨️", "🛠️"}
	for _, e := range defEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(rest)}
		}
	}

	folderEmojis := []string{"📁️", "🗃️", "📂️"}
	for _, e := range folderEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "folder", Path: workspace.NormalizePath(strings.TrimSuffix(rest, "/"))}
		}
	}

	fileEmojis := []string{"💻️", "🥼️", "📜️", "📃️", "⚙️", "💾️", "⚖️", "📄️"}
	for _, e := range fileEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "file", Path: workspace.NormalizePath(rest)}
		}
	}

	if strings.Contains(ref, "#") {
		parts := strings.SplitN(ref, "#", 2)
		filePath := workspace.NormalizePath(parts[0])
		var sectionParts []string
		if len(parts) > 1 {
			sectionParts = strings.Split(parts[1], "#")
		}
		return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
	}
	if strings.HasSuffix(ref, "/") {
		return ArtifactRef{Kind: "folder", Path: workspace.NormalizePath(strings.TrimSuffix(ref, "/"))}
	}
	normalized := workspace.NormalizePath(ref)
	absPath := filepath.Join(workspace.GetRootDir(), normalized)
	if info, err := os.Stat(absPath); err == nil && info.IsDir() {
		return ArtifactRef{Kind: "folder", Path: normalized}
	}
	return ArtifactRef{Kind: "file", Path: normalized}
}

// 🟪️UnSlugify MUST complete the operation successfully.
func UnSlugify(slug string) string {
	parts := strings.Split(slug, "-")
	for i, p := range parts {
		if len(p) > 0 {
			parts[i] = strings.ToUpper(p[:1]) + strings.ToLower(p[1:])
		}
	}
	return strings.Join(parts, " ")
}

// 💠️SectionIdValueToUriPath MUST complete the operation successfully.
func SectionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	if hashIdx < 0 {
		return workspace.PathToUriPath(value)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	sectionParts := strings.Split(rest, "#")
	result := workspace.PathToUriPath(filePath)
	for _, p := range sectionParts {
		result += "/" + workspace.PathToUriPath(p)
	}
	return result
}

// 🔳️DefinitionIdValueToUriPath MUST complete the operation successfully.
func DefinitionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	paragraphIdx := strings.Index(value, "§")
	if hashIdx < 0 && paragraphIdx < 0 {
		return workspace.PathToUriPath(value)
	}
	if hashIdx < 0 && paragraphIdx >= 0 {
		filePath := value[:paragraphIdx]
		defName := value[paragraphIdx+len("§"):]
		return workspace.PathToUriPath(filePath) + "/" + workspace.PathToUriPath(defName)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	parts := strings.Split(rest, "§")
	result := workspace.PathToUriPath(filePath)
	for _, p := range parts {
		subParts := strings.Split(p, "#")
		for _, sp := range subParts {
			result += "/" + workspace.PathToUriPath(sp)
		}
	}
	return result
}

// 🔬️ParseSectionUriPath MUST return the parsed result or an error for invalid input.
// 📝️ParseSectionUriPath parses and returns the section uri path from the input.
func ParseSectionUriPath(uriPath string) (filePath string, sectionSlugs []string) {
	parts := strings.Split(uriPath, "/")
	fileEnd := -1
	for i, p := range parts {
		if strings.Contains(p, ".") {
			fileEnd = i
		}
	}
	if fileEnd < 0 {
		return uriPath, nil
	}
	filePath = strings.Join(parts[:fileEnd+1], "/")
	if fileEnd+1 < len(parts) {
		sectionSlugs = parts[fileEnd+1:]
	}
	return
}

// 🔲️StatuteIdToUriPath MUST complete the operation successfully.
func StatuteIdToUriPath(id string) string {
	parts := strings.Split(id, "/")
	for i, p := range parts {
		parts[i] = workspace.PathToUriPath(p)
	}
	return strings.Join(parts, "/")
}

// ▪️StatuteUriPathToId MUST complete the operation successfully.
func StatuteUriPathToId(uriPath string) string {
	parts := strings.Split(uriPath, "/")
	for i, p := range parts {
		parts[i] = workspace.PathFromUriPath(p)
	}
	return strings.Join(parts, "/")
}

// #endregion 🧮️Artifact Ref
