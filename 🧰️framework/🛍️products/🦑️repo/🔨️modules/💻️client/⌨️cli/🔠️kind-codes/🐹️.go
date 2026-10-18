// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Kind code conversions and id mappings.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
)

// 🏺️goalArtifactID holds the data fields for a goalArtifactID record.
func goalArtifactID(rawGoalID string) string {
	parts := strings.Split(rawGoalID, "/")
	result := ""
	for _, p := range parts {
		result += emojiText(EmojiGoal) + Flat(p)
	}
	return result
}

// ⛳️goalPathToComposeID converts a filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI")
// 🖊️to the repo emoji ID format (e.g. "🎯️aioptimizedrepo🎯️repocli").
func goalPathToComposeID(goalPath string) string {
	if goalPath == "" {
		return ""
	}
	return goalArtifactID(goalPath)
}

// 🛤️composeIDToGoalPath converts a repo emoji goal ID (e.g. "🎯️aioptimizedrepo🎯️repocli")
// back to its filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI") by scanning the
// 🔑️goals directory and matching each segment's Flat() value.
func composeIDToGoalPath(composeID string) string {
	if composeID == "" {
		return ""
	}
	goalEmoji := emojiText(EmojiGoal)
	trimmed := strings.ReplaceAll(strings.ReplaceAll(composeID, "\uFE0E", ""), "\uFE0F", "")
	if strings.HasPrefix(trimmed, goalEmoji) {
		trimmed = trimmed[len(goalEmoji):]
	}
	segments := strings.Split(trimmed, goalEmoji)
	if len(segments) == 0 {
		return ""
	}

	dir := GetRepoGoalsDir()
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
			if entry.IsDir() && Flat(entry.Name()) == seg {
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
	prefix := emojiText(EmojiContributor)
	if strings.HasPrefix(identifier, prefix) {
		return identifier
	}
	return prefix + Flat(identifier)
}

// 🐙️composeIDToContributorGithub converts a compose contributor ID back to a contributor alias.
func composeIDToContributorGithub(composeID string) string {
	if composeID == "" || composeID == "unknown" {
		return composeID
	}
	prefix := emojiText(EmojiContributor)
	if !strings.HasPrefix(composeID, prefix) {
		return composeID
	}
	flat := composeID[len(prefix):]
	contributors, err := ListContributors()
	if err == nil {
		for _, c := range contributors {
			if Flat(c.Alias) == flat {
				return c.Alias
			}
		}
	}
	return flat
}

func interactionKindEmoji(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch kind {
	case "edited":
		return emojiText(EmojiInteractionEdited)
	case "finished":
		return emojiText(EmojiInteractionFinished)
	case "restarted":
		return emojiText(EmojiInteractionRestarted)
	case "deleted":
		return emojiText(EmojiInteractionDeleted)
	}
	return emojiText(EmojiInteractionStarted)
}

// 🔶️interactionKindFromEmoji holds the data fields for a interactionKindFromEmoji record.
func interactionKindFromEmoji(emoji string) string {
	switch emoji {
	case EmojiInteractionFinished:
		return "finished"
	case EmojiInteractionRestarted:
		return "restarted"
	case EmojiInteractionDeleted:
		return "deleted"
	case EmojiInteractionEdited:
		return "edited"
	}
	return "started"
}

// 📜️technologyKindCodeFromEmoji holds the data fields for a technologyKindCodeFromEmoji record.
func technologyKindCodeFromEmoji(emoji string) string {
	ne := emojiText(emoji)
	switch ne {
	case emojiText(EmojiTechnologyInfra):
		return "i"
	case emojiText(EmojiTechnologyResearch):
		return "r"
	case emojiText(EmojiTechnologyMono):
		return "m"
	}
	return "u"
}

// 📦️bundleKindCodeFromEmoji holds the data fields for a bundleKindCodeFromEmoji record.
func bundleKindCodeFromEmoji(emoji string) string {
	ne := emojiText(emoji)
	switch ne {
	case emojiText(EmojiBundleRepo):
		return "r"
	case emojiText(EmojiBundleSchema):
		return "s"
	case emojiText(EmojiBundleBinary):
		return "b"
	case emojiText(EmojiBundleUI):
		return "u"
	case emojiText(EmojiBundleExample):
		return "e"
	case emojiText(EmojiBundleSite):
		return "w"
	case emojiText(EmojiBundleAssets):
		return "a"
	}
	return "l"
}

// 🔹️folderKindCodeFromEmoji holds the data fields for a folderKindCodeFromEmoji record.
func folderKindCodeFromEmoji(emoji string) string {
	ne := emojiText(emoji)
	if ne == emojiText(EmojiFolderRequired) {
		return "req"
	}
	return "org"
}

// 📖️definitionKindCodeFromEmoji holds the data fields for a definitionKindCodeFromEmoji record.
func definitionKindCodeFromEmoji(emoji string) string {
	ne := emojiText(emoji)
	switch ne {
	case emojiText(EmojiDefinitionInterface):
		return "f"
	case emojiText(EmojiDefinitionConstant):
		return "c"
	case emojiText(EmojiDefinitionTest):
		return "t"
	}
	return "i"
}

// 🔸️technologyKindCode holds the data fields for a technologyKindCode record.
func technologyKindCode(data map[string]interface{}) string {
	if val, ok := data["kind"].(string); ok {
		switch val {
		case "infrastructure":
			return "i"
		case "research":
			return "r"
		case "mono":
			return "m"
		case "user":
			return "u"
		}
	}
	name, _ := data["name"].(string)
	if strings.Contains(name, "repo") {
		return "i"
	}
	if strings.HasPrefix(name, "coda") {
		return "r"
	}
	return "u"
}

// 🔺️TechnologyKindToCode holds the data fields for a TechnologyKindToCode record.
func TechnologyKindToCode(k TechnologyKind) string {
	switch k {
	case TechnologyKindInfrastructure:
		return "i"
	case TechnologyKindResearch:
		return "r"
	case TechnologyKindMono:
		return "m"
	}
	return "u"
}

// 🔻️bundleKindCode holds the data fields for a bundleKindCode record.
func bundleKindCode(data map[string]interface{}) string {
	bKind, _ := data["kind"].(string)
	switch bKind {
	case "repo":
		return "r"
	case "library":
		return "l"
	case "schema":
		return "s"
	case "binary":
		return "b"
	case "ui":
		return "u"
	case "example":
		return "e"
	case "site":
		return "w"
	case "assets":
		return "a"
	}
	return "l"
}

// ⬛️BundleKindToCode holds the data fields for a BundleKindToCode record.
func BundleKindToCode(k BundleKind) string {
	switch k {
	case BundleKindRepo:
		return "r"
	case BundleKindSchema:
		return "s"
	case BundleKindBinary:
		return "b"
	case BundleKindUI:
		return "u"
	case BundleKindSite:
		return "w"
	case BundleKindAssets:
		return "a"
	}
	return "l"
}

// ⬜️folderKindCode holds the data fields for a folderKindCode record.
func folderKindCode(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	if fkind == "organization" {
		return "org"
	}
	return "req"
}

func FolderKindToCode(k FolderKind) string {
	if k == FolderKindOrganization {
		return "org"
	}
	return "req"
}

// 🟥️definitionKindCode holds the data fields for a definitionKindCode record.
func definitionKindCode(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch DeriveDefinitionKind(kind) {
	case DefinitionKindInterface:
		return "f"
	case DefinitionKindConstant:
		return "c"
	case DefinitionKindTest:
		return "t"
	}
	return "i"
}

// 🟧️interactionKindCode holds the data fields for a interactionKindCode record.
func interactionKindCode(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	return kind
}

// 📑️containsUriSection holds the data fields for a containsUriSection record.
func containsUriSection(uri string) bool {
	return strings.HasPrefix(uri, "repo://section/")
}

// 🔗️containsUriDefinition holds the data fields for a containsUriDefinition record.
func containsUriDefinition(uri string) bool {
	return strings.HasPrefix(uri, "repo://definition/")
}
