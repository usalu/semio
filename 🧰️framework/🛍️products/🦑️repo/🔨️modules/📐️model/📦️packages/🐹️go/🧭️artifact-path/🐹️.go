// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧭️artifact_path carries the entity kind detection and the identifier to path conversions.

// #endregion 🧲️Header

package model

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧭️Artifact Path

// 🔴️DetectEntityKindFromId determines the entity kind from an emoji-based ID string.
func DetectEntityKindFromId(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(e string) string {
		r := strings.ReplaceAll(e, "\uFE0F", "")
		return strings.ReplaceAll(r, "\uFE0E", "")
	}
	hasPrefix := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.HasPrefix(s, p)
	}
	hasSuffix := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.HasSuffix(s, p)
	}
	contains := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.Contains(s, p)
	}
	// Check interaction suffixes first
	interactionEmojis := []string{EmojiInteractionStarted, EmojiInteractionEdited, EmojiInteractionFinished, EmojiInteractionRestarted, EmojiInteractionDeleted}
	for _, ie := range interactionEmojis {
		if hasSuffix(normalized, ie) {
			return "interaction"
		}
	}
	// Check definition emojis (must check before section since definitions contain sections)
	defEmojis := []string{EmojiDefinitionImpl, EmojiDefinitionInterface, EmojiDefinitionConstant, EmojiDefinitionTest}
	for _, de := range defEmojis {
		if contains(normalized, de) {
			return "definition"
		}
	}
	// Check section emoji
	if contains(normalized, EmojiSection) {
		return "section"
	}
	// Check contributor before file emojis (contributor emoji 🧑️‍💻️ contains 💻️ which matches EmojiFileCode)
	if contains(normalized, EmojiContributor) {
		return "contributor"
	}
	// Check file emojis
	fileEmojis := []string{EmojiFileCode, EmojiFileLab, EmojiFileScript, EmojiFileDocs, EmojiFileConfig, EmojiFileResource, EmojiFileLicense}
	for _, fe := range fileEmojis {
		if contains(normalized, fe) {
			return "file"
		}
	}
	// Check folder emojis
	folderEmojis := []string{EmojiFolderOrg, EmojiFolderRequired}
	for _, fe := range folderEmojis {
		if contains(normalized, fe) {
			return "folder"
		}
	}
	// Check technology/bundle emojis
	technologyEmojis := []string{EmojiTechnologyUser, EmojiTechnologyInfra, EmojiTechnologyResearch, EmojiTechnologyMono}
	bundleEmojis := []string{EmojiBundleLibrary, EmojiBundleSchema, EmojiBundleBinary, EmojiBundleUI, EmojiBundleExample, EmojiBundleSite, EmojiBundleAssets, EmojiBundleRepo}
	for _, pe := range technologyEmojis {
		if hasPrefix(normalized, pe) {
			for _, be := range bundleEmojis {
				if contains(normalized, be) {
					return "bundle"
				}
			}
			return "technology"
		}
	}

	// singularOrPlural is a helper for entity kinds where singular and plural emojis are the same.
	// If the ID has text after the emoji prefix → singular kind, otherwise → plural kind.
	singularOrPlural := func(emoji, singular, plural string) string {
		e := norm(emoji)
		if e != "" && strings.HasPrefix(normalized, e) {
			if len(normalized) > len(e) {
				return singular
			}
			return plural
		}
		return ""
	}

	// Collection emojis where singular/plural differ
	if hasPrefix(normalized, EmojiYears) && norm(EmojiYears) != norm(EmojiYear) {
		return "years"
	}
	if hasPrefix(normalized, EmojiYear) {
		return "year"
	}
	if hasPrefix(normalized, EmojiMonths) && norm(EmojiMonths) != norm(EmojiMonth) {
		return "months"
	}
	if hasPrefix(normalized, EmojiMonth) {
		return "month"
	}
	if hasPrefix(normalized, EmojiDays) && norm(EmojiDays) != norm(EmojiDay) {
		return "days"
	}
	if hasPrefix(normalized, EmojiDay) {
		return "day"
	}
	if hasPrefix(normalized, EmojiHours) && norm(EmojiHours) != norm(EmojiHour) {
		return "hours"
	}
	if hasPrefix(normalized, EmojiHour) {
		return "hour"
	}
	if hasPrefix(normalized, EmojiMinutes) && norm(EmojiMinutes) != norm(EmojiMinute) {
		return "minutes"
	}
	if hasPrefix(normalized, EmojiMinute) {
		return "minute"
	}
	if hasPrefix(normalized, EmojiSeconds) && norm(EmojiSeconds) != norm(EmojiSecond) {
		return "seconds"
	}
	if hasPrefix(normalized, EmojiSecond) {
		return "second"
	}
	if hasPrefix(normalized, EmojiTechnologies) {
		return "technologies"
	}
	if hasPrefix(normalized, EmojiBundles) {
		return "bundles"
	}
	if hasPrefix(normalized, EmojiFolders) {
		return "folders"
	}
	if hasPrefix(normalized, EmojiFiles) {
		return "files"
	}
	if hasPrefix(normalized, EmojiSections) {
		return "sections"
	}
	if hasPrefix(normalized, EmojiDefinitions) {
		return "definitions"
	}
	if hasPrefix(normalized, EmojiCodebase) {
		return "codebase"
	}
	// Entity kinds where singular and plural emojis are the same
	if k := singularOrPlural(EmojiTicket, "ticket", "tickets"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiGoal, "goal", "goals"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiDraft, "draft", "drafts"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiTodo, "todo", "todos"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiPolicy, "policy", "policies"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiBreach, "breach", "breaches"); k != "" {
		return k
	}
	// contributor already handled above via contains
	if hasPrefix(normalized, EmojiContributors) {
		return "contributors"
	}
	if k := singularOrPlural(EmojiCheckpoint, "checkpoint", "checkpoints"); k != "" {
		return k
	}
	if hasPrefix(normalized, EmojiInteractions) {
		return "interactions"
	}
	if k := singularOrPlural(EmojiSession, "session", "sessions"); k != "" {
		return k
	}
	return ""
}

// 🟡️IdToPath converts an emoji-based artifact ID back to a filesystem path.
// It walks all tracked bundles and files, computes their IDs, and matches.
// ◽For file/folder/section/definition IDs, it returns the file path.
func IdToPath(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	if normalized == "" {
		return ""
	}
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	// Strip section and definition suffixes to get to the file-level ID.
	fileId := normalized
	sectionEmojis := []string{EmojiSection}
	definitionEmojis := []string{EmojiDefinitionImpl, EmojiDefinitionInterface, EmojiDefinitionConstant, EmojiDefinitionTest}
	// Find the first section or definition emoji to truncate to file ID.
	firstSectionIdx := -1
	for _, se := range sectionEmojis {
		nse := norm(se)
		idx := strings.Index(fileId, nse)
		if idx >= 0 && (firstSectionIdx < 0 || idx < firstSectionIdx) {
			firstSectionIdx = idx
		}
	}
	if firstSectionIdx < 0 {
		for _, de := range definitionEmojis {
			nde := norm(de)
			idx := strings.Index(fileId, nde)
			if idx >= 0 && (firstSectionIdx < 0 || idx < firstSectionIdx) {
				firstSectionIdx = idx
			}
		}
	}
	if firstSectionIdx > 0 {
		fileId = fileId[:firstSectionIdx]
	}
	// Now fileId should be a bundle+folder+file ID like "🧰️repo⌨️cli💻️main"
	// Walk all bundles and their files to find a match.
	bundles := LookupBundles()
	for _, b := range bundles {
		bundleId := norm(b.GetID())
		if !strings.HasPrefix(fileId, bundleId) {
			continue
		}
		bundleRoot := filepath.Join(workspace.GetRootDir(), workspace.NormalizePath(b.Root))
		// Walk files in this bundle
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil || info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
				return nil
			}
			return nil
		})
		// Instead of walking, compute ID for the remainder after bundle ID.
		// The remainder encodes folder + file emojis.
		rest := fileId[len(bundleId):]
		if rest == "" {
			// This is a bundle-level ID, return bundle root
			return workspace.NormalizePath(b.Root)
		}
		// 📂️Walk the bundle directory and match file IDs
		var matchedPath string
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
				return nil
			}
			relPath, _ := filepath.Rel(workspace.GetRootDir(), path)
			relPath = workspace.NormalizePath(relPath)
			computedId := norm(BuildFileID(relPath, nil))
			if computedId == fileId {
				matchedPath = relPath
				return filepath.SkipAll
			}
			return nil
		})
		if matchedPath != "" {
			return matchedPath
		}
	}
	// Fallback: try to match folder IDs
	for _, b := range bundles {
		bundleId := norm(b.GetID())
		if !strings.HasPrefix(fileId, bundleId) {
			continue
		}
		bundleRoot := filepath.Join(workspace.GetRootDir(), workspace.NormalizePath(b.Root))
		var matchedPath string
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if !info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) {
				return filepath.SkipDir
			}
			relPath, _ := filepath.Rel(workspace.GetRootDir(), path)
			relPath = workspace.NormalizePath(relPath)
			computedId := norm(BuildFolderID(relPath, nil))
			if computedId == fileId {
				matchedPath = relPath
				return filepath.SkipAll
			}
			return nil
		})
		if matchedPath != "" {
			return matchedPath
		}
	}
	return ""
}

// 🟢️IdToSectionPath extracts the section path parts from an emoji-based ID.
// 🔵️Returns the section names as a slice.
func IdToSectionPath(id string) []string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	sectionEmoji := norm(EmojiSection)
	definitionEmojis := []string{
		norm(EmojiDefinitionImpl),
		norm(EmojiDefinitionInterface),
		norm(EmojiDefinitionConstant),
		norm(EmojiDefinitionTest),
	}
	// Strip definition suffix if present
	forSections := normalized
	for _, de := range definitionEmojis {
		idx := strings.LastIndex(forSections, de)
		if idx >= 0 {
			forSections = forSections[:idx]
			break
		}
	}
	// 🟣️Find all section emoji occurrences
	var sections []string
	remaining := forSections
	for {
		idx := strings.Index(remaining, sectionEmoji)
		if idx < 0 {
			break
		}
		remaining = remaining[idx+len(sectionEmoji):]
		// Find the end of this section name (next section emoji or end of string)
		nextIdx := strings.Index(remaining, sectionEmoji)
		if nextIdx < 0 {
			sections = append(sections, remaining)
			break
		}
		sections = append(sections, remaining[:nextIdx])
		remaining = remaining[nextIdx:]
	}
	return sections
}

// 🟤️IdToDefinitionName extracts the definition name from an emoji-based ID.
func IdToDefinitionName(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	definitionEmojis := []string{
		norm(EmojiDefinitionImpl),
		norm(EmojiDefinitionInterface),
		norm(EmojiDefinitionConstant),
		norm(EmojiDefinitionTest),
	}
	for _, de := range definitionEmojis {
		idx := strings.LastIndex(normalized, de)
		if idx >= 0 {
			return normalized[idx+len(de):]
		}
	}
	return ""
}

// #endregion 🧭️Artifact Path
