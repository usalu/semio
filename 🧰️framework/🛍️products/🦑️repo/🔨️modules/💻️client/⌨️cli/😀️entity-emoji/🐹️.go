// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Entity emoji helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
)

// 💿️SemanticId holds the data fields for a semantic id record.
type SemanticId struct {
	Emoji string
	Value string
}

// 😀️emojiText holds the data fields for a emojiText record.
func emojiText(emoji string) string {
	stripped := strings.ReplaceAll(emoji, "\uFE0E", "")
	textDefaultEmojis := []string{
		"\U0001F3D7", "\u2328", "\U0001F5B1", "\U0001F5C3",
		"\u2699", "\u2696", "\U0001F3F7", "\U0001F6E0",
		"\u2702", "\U0001F6E1", "\U0001F5D1",
		"\u2600", "\u23F1", "\u270F", "\U0001F46E",
		"\u2B05", "\u2B06", "\u2B07",
	}
	base := strings.ReplaceAll(stripped, "\uFE0F", "")
	for _, td := range textDefaultEmojis {
		if strings.Contains(base, td) {
			return strings.ReplaceAll(base, td, td+"\uFE0F")
		}
	}
	return base
}

// 🧲️extractEntityEmoji extracts the leading emoji and remaining text from a string.
// 🧲️Returns (emoji, remainingText). If no emoji is found, returns ("", original).
func extractEntityEmoji(s string) (string, string) {
	if s == "" {
		return "", ""
	}
	runes := []rune(s)
	if len(runes) == 0 {
		return "", ""
	}
	i := 0
	r := runes[i]
	if strings.ContainsRune("0123456789#*", r) {
		end := 1
		if end < len(runes) && runes[end] == 0xFE0F {
			end++
		}
		if end < len(runes) && runes[end] == 0x20E3 {
			return string(runes[:end+1]), string(runes[end+1:])
		}
		return "", s
	}
	// Check if the first rune is an emoji
	if !isEmojiRune(r) {
		return "", s
	}
	i++
	if r >= 0x1F1E6 && r <= 0x1F1FF && i < len(runes) && runes[i] >= 0x1F1E6 && runes[i] <= 0x1F1FF {
		i++
	}
	// Consume variation selectors and ZWJ sequences
	for i < len(runes) {
		r = runes[i]
		if r == 0xFE0F || r == 0xFE0E { // variation selectors
			i++
		} else if r == 0x20E3 { // combining enclosing keycap
			i++
		} else if r == 0x200D { // ZWJ
			i++
			if i < len(runes) {
				i++ // consume the next emoji after ZWJ
				// Continue consuming variation selectors
				for i < len(runes) && (runes[i] == 0xFE0F || runes[i] == 0xFE0E) {
					i++
				}
			}
		} else if r >= 0x1F3FB && r <= 0x1F3FF { // skin tone modifiers
			i++
		} else {
			break
		}
	}
	emoji := string(runes[:i])
	remaining := string(runes[i:])
	return emoji, remaining
}

// 🔷️isEmojiRune returns true if the rune is likely an emoji base character.
func isEmojiRune(r rune) bool {
	if r >= 0x1F1E6 && r <= 0x1F1FF {
		return true
	}
	// Common emoji ranges
	if r >= 0x1F600 && r <= 0x1F64F {
		return true
	} // Emoticons
	if r >= 0x1F300 && r <= 0x1F5FF {
		return true
	} // Misc Symbols and Pictographs
	if r >= 0x1F680 && r <= 0x1F6FF {
		return true
	} // Transport and Map
	if r >= 0x1F700 && r <= 0x1F77F {
		return true
	} // Alchemical Symbols
	if r >= 0x1F780 && r <= 0x1F7FF {
		return true
	} // Geometric Shapes Extended
	if r >= 0x1F800 && r <= 0x1F8FF {
		return true
	} // Supplemental Arrows-C
	if r >= 0x1F900 && r <= 0x1F9FF {
		return true
	} // Supplemental Symbols and Pictographs
	if r >= 0x1FA00 && r <= 0x1FA6F {
		return true
	} // Chess Symbols
	if r >= 0x1FA70 && r <= 0x1FAFF {
		return true
	} // Symbols and Pictographs Extended-A
	if r >= 0x2600 && r <= 0x26FF {
		return true
	} // Misc Symbols
	if r >= 0x2700 && r <= 0x27BF {
		return true
	} // Dingbats
	if r >= 0x2300 && r <= 0x23FF {
		return true
	} // Misc Technical
	if r >= 0x2B50 && r <= 0x2B55 {
		return true
	} // Stars
	if r >= 0x2B05 && r <= 0x2B07 {
		return true
	}
	if r >= 0x200D && r <= 0x200D {
		return true
	} // ZWJ
	if r >= 0xFE00 && r <= 0xFE0F {
		return true
	} // Variation selectors
	if r == 0x2139 || r == 0x2194 || r == 0x2195 {
		return true
	}
	if r >= 0x2196 && r <= 0x2199 {
		return true
	}
	if r >= 0x21A9 && r <= 0x21AA {
		return true
	}
	if r >= 0x231A && r <= 0x231B {
		return true
	}
	if r >= 0x25AA && r <= 0x25AB {
		return true
	}
	if r == 0x25B6 || r == 0x25C0 || r == 0x25FB || r == 0x25FC || r == 0x25FD || r == 0x25FE {
		return true
	}
	if r >= 0x2614 && r <= 0x2615 {
		return true
	}
	if r >= 0x2648 && r <= 0x2653 {
		return true
	}
	if r == 0x267F || r == 0x2693 || r == 0x26A1 {
		return true
	}
	if r >= 0x26AA && r <= 0x26AB {
		return true
	}
	if r >= 0x26BD && r <= 0x26BE {
		return true
	}
	if r >= 0x26C4 && r <= 0x26C5 {
		return true
	}
	if r == 0x26CE || r == 0x26D4 {
		return true
	}
	if r == 0x26EA || r == 0x26F2 || r == 0x26F3 || r == 0x26F5 || r == 0x26FA || r == 0x26FD {
		return true
	}
	if r == 0x2702 || r == 0x2705 {
		return true
	}
	if r >= 0x2708 && r <= 0x270D {
		return true
	}
	if r == 0x270F {
		return true
	}
	if r == 0x2712 || r == 0x2714 || r == 0x2716 || r == 0x271D || r == 0x2721 {
		return true
	}
	if r == 0x2728 {
		return true
	}
	if r >= 0x2733 && r <= 0x2734 {
		return true
	}
	if r == 0x2744 || r == 0x2747 || r == 0x274C || r == 0x274E {
		return true
	}
	if r >= 0x2753 && r <= 0x2755 {
		return true
	}
	if r == 0x2757 {
		return true
	}
	if r >= 0x2763 && r <= 0x2764 {
		return true
	}
	if r >= 0x2795 && r <= 0x2797 {
		return true
	}
	if r == 0x27A1 || r == 0x27B0 || r == 0x27BF {
		return true
	}
	if r >= 0x2934 && r <= 0x2935 {
		return true
	}
	if r >= 0x3030 && r <= 0x3030 {
		return true
	}
	if r == 0x303D || r == 0x3297 || r == 0x3299 {
		return true
	}
	if r == 0x00A9 || r == 0x00AE {
		return true
	} // © ®
	return false
}

// 🛠️resolveTechnologyEmoji looks up the emoji for a technology by name.
// 📺️Falls back to kind-derived emoji.
func resolveTechnologyEmoji(techName string) string {
	techs := LoadTechnologies()
	for _, t := range techs {
		if t.Name == techName {
			if t.Emoji != "" {
				return t.Emoji
			}
			break
		}
	}
	kind := DeriveTechnologyKind(techName)
	return string(kind)
}

// 🔤️String MUST return a non-empty string representation.
// 🔤️String returns the string representation of the semantic id.
func (s SemanticId) String() string {
	if s.Value == "" {
		return emojiText(s.Emoji)
	}
	return fmt.Sprintf("%s%s", emojiText(s.Emoji), s.Value)
}

// 🏷️technologyKindEmoji holds the data fields for a technologyKindEmoji record.
func technologyKindEmoji(data map[string]interface{}) string {
	if emoji, ok := data["emoji"].(string); ok && emoji != "" {
		return emojiText(emoji)
	}
	if val, ok := data["kind"].(string); ok {
		switch val {
		case "infrastructure":
			return emojiText(EmojiTechnologyInfra)
		case "research":
			return emojiText(EmojiTechnologyResearch)
		case "mono":
			return emojiText(EmojiTechnologyMono)
		case "user":
			return emojiText(EmojiTechnologyUser)
		}
	}
	name, _ := data["name"].(string)
	if strings.Contains(name, "repo") {
		return emojiText(EmojiTechnologyInfra)
	}
	if strings.HasPrefix(name, "coda") {
		return emojiText(EmojiTechnologyResearch)
	}
	return emojiText(EmojiTechnologyUser)
}

func bundleKindEmoji(data map[string]interface{}) string {
	if emoji, ok := data["emoji"].(string); ok && emoji != "" {
		return emojiText(emoji)
	}
	bKind, _ := data["kind"].(string)
	switch bKind {
	case "schema":
		return emojiText(EmojiBundleSchema)
	case "binary":
		return emojiText(EmojiBundleBinary)
	case "ui":
		return emojiText(EmojiBundleUI)
	case "site":
		return emojiText(EmojiBundleSite)
	case "assets":
		return emojiText(EmojiBundleAssets)
	case "library":
		return emojiText(EmojiBundleLibrary)
	case "example":
		return emojiText(EmojiBundleExample)
	}
	return emojiText(EmojiBundleLibrary)
}

// 📄️fileKindEmoji holds the data fields for a fileKindEmoji record.
func fileKindEmoji(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	switch fkind {
	case "code":
		return emojiText(EmojiFileCode)
	case "lab":
		return emojiText(EmojiFileLab)
	case "script":
		return emojiText(EmojiFileScript)
	case "docs":
		return emojiText(EmojiFileDocs)
	case "config":
		return emojiText(EmojiFileConfig)
	case "resource":
		return emojiText(EmojiFileResource)
	case "template":
		return emojiText(EmojiFileTemplate)
	case "license":
		return emojiText(EmojiFileLicense)
	}
	return ""
}

// 📁️folderKindEmoji holds the data fields for a folderKindEmoji record.
func folderKindEmoji(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	if fkind == "organization" {
		return emojiText(EmojiFolderOrg)
	} else if fkind == "root" {
		return emojiText(EmojiFolderRoot)
	}
	return emojiText(EmojiFolderRequired)
}

func definitionKindEmoji(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch DeriveDefinitionKind(kind) {
	case DefinitionKindInterface:
		return emojiText(EmojiDefinitionInterface)
	case DefinitionKindConstant:
		return emojiText(EmojiDefinitionConstant)
	case DefinitionKindTest:
		return emojiText(EmojiDefinitionTest)
	}
	return emojiText(EmojiDefinitionImpl)
}
