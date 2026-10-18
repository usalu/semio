// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️classify carries the path and language classifiers and the unified line-of-code counters.

// #endregion 🧲️Header

package metrics

import (
	"bytes"
	"encoding/json"
	"strings"
)

// #region 🗂️Classify

// 🧩️extensionOf is the lowercase extension of a path's last segment.
func extensionOf(path string) string {
	base := path
	if idx := strings.LastIndexAny(base, "/\\"); idx >= 0 {
		base = base[idx+1:]
	}
	idx := strings.LastIndex(base, ".")
	if idx < 0 {
		return ""
	}
	return strings.ToLower(base[idx:])
}

// 🏷️ClassifyLocBucket maps a path extension to a `loc` bucket, or "" when it has none.
func ClassifyLocBucket(path string) string {
	switch extensionOf(path) {
	case ".ts", ".tsx", ".cts", ".mts", ".mtsx":
		return "TypeScript"
	case ".go":
		return "Go"
	case ".cs":
		return "C#"
	case ".py":
		return "Python"
	case ".rs":
		return "Rust"
	case ".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme":
		return AggMarkup
	case ".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules":
		return AggData
	default:
		return ""
	}
}

// 🏷️ClassifyLanguageFine maps a path to the fine-grained bucket the TypeScript library reports.
func ClassifyLanguageFine(path string) string {
	rel := NormalizeRepoPath(path)
	base := rel
	if idx := strings.LastIndex(base, "/"); idx >= 0 {
		base = base[idx+1:]
	}
	base = strings.ToLower(base)
	if base == "dockerfile" || strings.HasPrefix(base, "dockerfile.") {
		return "Dockerfile"
	}
	if base == "makefile" || base == "justfile" {
		return "Makefile"
	}
	switch extensionOf(rel) {
	case ".ts", ".tsx", ".cts", ".mts", ".mtsx":
		return "TypeScript"
	case ".js", ".mjs", ".cjs":
		return "JavaScript"
	case ".go":
		return "Go"
	case ".cs":
		return "C#"
	case ".py":
		return "Python"
	case ".rs":
		return "Rust"
	case ".sh", ".bash", ".zsh":
		return "Shell"
	case ".ps1":
		return "PowerShell"
	case ".css", ".scss", ".sass":
		return "CSS"
	case ".sql":
		return "SQL"
	case ".html", ".htm", ".xhtml":
		return "HTML"
	case ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx":
		return "Markdown"
	case ".tex", ".sty", ".cls", ".ltx", ".bib":
		return "TeX"
	case ".json", ".jsonc":
		return "JSON"
	case ".yaml", ".yml":
		return "YAML"
	case ".toml":
		return "TOML"
	case ".csv":
		return "CSV"
	case ".xml":
		return "XML"
	default:
		return ""
	}
}

// 🧾️MakeLangSet is the enabled code-language set, trimmed as the CLI flag delivers it.
func MakeLangSet(languages []string) map[string]bool {
	set := make(map[string]bool, len(languages))
	for _, language := range languages {
		set[strings.TrimSpace(language)] = true
	}
	return set
}

// 🧾️MakeNumstatLangSet is the enabled code languages plus the two aggregate buckets.
func MakeNumstatLangSet(languages []string) map[string]bool {
	set := MakeLangSet(languages)
	set[AggMarkup] = true
	set[AggData] = true
	return set
}

// 🏷️ClassifyForNumstat is the bucket of a path when the active weight set enables it.
func ClassifyForNumstat(path string, weights map[string]bool) string {
	bucket := ClassifyLocBucket(path)
	if bucket == "" || !weights[bucket] {
		return ""
	}
	return bucket
}

// 🫥️PathHasHiddenSegment reports whether any path segment starts with a dot.
func PathHasHiddenSegment(rel string) bool {
	for _, segment := range strings.Split(NormalizeRepoPath(rel), "/") {
		if segment == "" || segment == "." || segment == ".." {
			continue
		}
		if strings.HasPrefix(segment, ".") {
			return true
		}
	}
	return false
}

// 🧬️PathIsRepoMeta reports whether a path is the repository's own meta tree.
func PathIsRepoMeta(rel string) bool {
	normalized := NormalizeRepoPath(rel)
	return normalized == SemioDirName || strings.HasPrefix(normalized, SemioDirName+"/")
}

// 🗂️PathSkippedForLoc reports whether a path must not be counted.
func PathSkippedForLoc(rel string, ignore Ignorer) bool {
	normalized := NormalizeRepoPath(rel)
	if normalized == "" || PathIsRepoMeta(normalized) {
		return true
	}
	if ignore != nil && ignore.MatchesPath(normalized) {
		return true
	}
	return PathHasHiddenSegment(normalized)
}

// 📏️PhysicalLineCount counts newline-terminated lines; a non-empty file has at least one.
func PhysicalLineCount(data []byte) int {
	if len(data) == 0 {
		return 0
	}
	return bytes.Count(data, []byte("\n")) + 1
}

// 🧮️CountJSONKeys recursively counts object keys; arrays contribute only their members.
func CountJSONKeys(value any) int {
	switch typed := value.(type) {
	case map[string]any:
		total := len(typed)
		for _, item := range typed {
			total += CountJSONKeys(item)
		}
		return total
	case []any:
		total := 0
		for _, item := range typed {
			total += CountJSONKeys(item)
		}
		return total
	default:
		return 0
	}
}

// 🧮️JSONKeyCount is the object-key count of a document, 0 when blank or not valid JSON.
func JSONKeyCount(data []byte) int {
	trimmed := bytes.TrimSpace(data)
	if len(trimmed) == 0 {
		return 0
	}
	var value any
	if err := json.Unmarshal(trimmed, &value); err != nil {
		return 0
	}
	return CountJSONKeys(value)
}

// 📏️CountUnifiedLocForFile is the repository-wide unified LOC of one tracked file body.
func CountUnifiedLocForFile(rel string, data []byte) int {
	switch extensionOf(rel) {
	case ".json", ".jsonc":
		if keys := JSONKeyCount(data); keys > 0 {
			return keys
		}
		if len(bytes.TrimSpace(data)) == 0 {
			return 0
		}
		return PhysicalLineCount(data)
	default:
		return PhysicalLineCount(data)
	}
}

// #endregion 🗂️Classify
