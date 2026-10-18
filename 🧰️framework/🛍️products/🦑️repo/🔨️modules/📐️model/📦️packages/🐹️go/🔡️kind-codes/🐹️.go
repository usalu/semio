// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔡️kind_codes carries the kind to code and emoji to code mappings.

// #endregion 🧲️Header

package model

import (
	strings "strings"
)

// #region 🔡️Kind Codes

func interactionKindEmoji(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch kind {
	case "edited":
		return EmojiText(EmojiInteractionEdited)
	case "finished":
		return EmojiText(EmojiInteractionFinished)
	case "restarted":
		return EmojiText(EmojiInteractionRestarted)
	case "deleted":
		return EmojiText(EmojiInteractionDeleted)
	}
	return EmojiText(EmojiInteractionStarted)
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
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiTechnologyInfra):
		return "i"
	case EmojiText(EmojiTechnologyResearch):
		return "r"
	case EmojiText(EmojiTechnologyMono):
		return "m"
	}
	return "u"
}

// 📦️bundleKindCodeFromEmoji holds the data fields for a bundleKindCodeFromEmoji record.
func bundleKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiBundleRepo):
		return "r"
	case EmojiText(EmojiBundleSchema):
		return "s"
	case EmojiText(EmojiBundleBinary):
		return "b"
	case EmojiText(EmojiBundleUI):
		return "u"
	case EmojiText(EmojiBundleExample):
		return "e"
	case EmojiText(EmojiBundleSite):
		return "w"
	case EmojiText(EmojiBundleAssets):
		return "a"
	}
	return "l"
}

// 🔹️folderKindCodeFromEmoji holds the data fields for a folderKindCodeFromEmoji record.
func folderKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	if ne == EmojiText(EmojiFolderRequired) {
		return "req"
	}
	return "org"
}

// 📖️definitionKindCodeFromEmoji holds the data fields for a definitionKindCodeFromEmoji record.
func definitionKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiDefinitionInterface):
		return "f"
	case EmojiText(EmojiDefinitionConstant):
		return "c"
	case EmojiText(EmojiDefinitionTest):
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

// #endregion 🔡️Kind Codes
