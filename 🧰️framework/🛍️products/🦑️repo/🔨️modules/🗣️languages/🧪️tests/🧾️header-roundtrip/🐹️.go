// 🐹️ Go side of the file header case. The Go implementation has no header reader yet, so the
// read-back half is composed here from the production section parser plus the language's own comment
// prefix — exactly the decomposition the Rust `parse_header` performs internally.
package adapter

import (
	"strings"

	languages "github.com/usalu/semio/repo/languages"
	host "semio.tech/repo/test"
)

// #region 🔖️Projection

var registry = []string{"typescript", "go", "python", "csharp", "markdown", "rust", "ruby", "shell", "toml", "yaml", "sql", "graphql"}

const (
	fileID       = "💻️test/file"
	fileURI      = "repo://file/💻️test"
	summary      = "A test file"
	contributors = "2025 Test User <test@test.com>"
	license      = "AGPL license text here"
	requirements = "Some requirements"
)

// 🧾️blocks splits a formatted header body into blank-line-separated blocks of prefix-stripped lines.
func blocks(text, prefix string, startLine, endLine int) [][]string {
	lines := strings.Split(text, "\n")
	var out [][]string
	var current []string
	for i := startLine; i < endLine-1 && i < len(lines); i++ {
		line := lines[i]
		if strings.TrimSpace(line) == "" {
			if len(current) > 0 {
				out = append(out, current)
				current = nil
			}
			continue
		}
		stripped := strings.TrimPrefix(line, prefix)
		current = append(current, strings.TrimPrefix(stripped, " "))
	}
	if len(current) > 0 {
		out = append(out, current)
	}
	return out
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

func headerPerLanguage(_ *host.Context) (host.Outcome, error) {
	projection := map[string]any{}
	for _, name := range registry {
		lang := languages.GetLanguageByName(name)
		if lang == nil {
			projection[name] = ""
			continue
		}
		projection[name] = lang.FormatHeader(fileID, fileURI, summary, contributors, license, requirements)
	}
	return host.Outcome{Projection: projection}, nil
}

func headerRegionIsParseable(_ *host.Context) (host.Outcome, error) {
	projection := map[string]any{}
	for _, name := range registry {
		lang := languages.GetLanguageByName(name)
		if lang == nil || !lang.SupportsHeaders() {
			continue
		}
		text := lang.FormatHeader(fileID, fileURI, summary, contributors, license, requirements)
		sections := lang.ParseSections(text)
		entry := map[string]any{
			"sectionCount":     len(sections),
			"sectionName":      "",
			"sectionStartLine": 0,
			"sectionEndLine":   0,
			"fileId":           "",
			"fileUri":          "",
			"contributors":     "",
			"license":          "",
		}
		if len(sections) > 0 {
			entry["sectionName"] = sections[0].Name
			entry["sectionStartLine"] = sections[0].StartLine
			entry["sectionEndLine"] = sections[0].EndLine
			body := blocks(text, lang.CommentPrefix(), sections[0].StartLine, sections[0].EndLine)
			if len(body) > 0 && len(body[0]) > 0 {
				identity := body[0][0]
				if open := strings.Index(identity, "["); open >= 0 {
					if close := strings.Index(identity, "]("); close > open {
						if end := strings.LastIndex(identity, ")"); end > close {
							entry["fileId"] = identity[open+1 : close]
							entry["fileUri"] = identity[close+2 : end]
						}
					}
				}
				entry["contributors"] = strings.Join(body[0][1:], "\n")
			}
			if len(body) > 1 {
				entry["license"] = strings.Join(body[1], "\n")
			}
		}
		projection[name] = entry
	}
	return host.Outcome{Projection: projection}, nil
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("header-per-language", headerPerLanguage).
		Subject("header-region-is-parseable", headerRegionIsParseable)
}

// #endregion 🔖️Registration
