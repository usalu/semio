// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Rust and Ruby language programs.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"regexp"
	"sort"
	"strings"
)

// 🗣️RustLanguage holds the data fields for a rust language record.
type RustLanguage struct {
	BaseLanguage
}

// 📑️NewRustLanguage MUST initialize all required fields and return a valid RustLanguage.
// NewRustLanguage creates and returns a new RustLanguage instance.
// ✔️RustSectionNameToModName MUST convert a display section name to a valid Rust mod identifier.
func RustSectionNameToModName(name string) string {
	var buf strings.Builder
	runes := []rune(name)
	for i, r := range runes {
		if r >= 'A' && r <= 'Z' {
			if i > 0 {
				prev := runes[i-1]
				if (prev >= 'a' && prev <= 'z') || (prev >= '0' && prev <= '9') {
					buf.WriteRune('_')
				}
			}
			buf.WriteRune(r - 'A' + 'a')
		} else if r >= 'a' && r <= 'z' {
			buf.WriteRune(r)
		} else if r >= '0' && r <= '9' {
			buf.WriteRune(r)
		} else {
			if buf.Len() > 0 {
				last := []rune(buf.String())
				if last[len(last)-1] != '_' {
					buf.WriteRune('_')
				}
			}
		}
	}
	result := strings.Trim(buf.String(), "_")
	if result == "" {
		return "section"
	}
	return result
}

func NewRustLanguage() *RustLanguage {
	return &RustLanguage{
		BaseLanguage: BaseLanguage{
			name:               "rust",
			extensions:         []string{".rs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*(?:pub(?:\(crate\))?\s+)?mod\s+\w+\s*\{\s*//\s*(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*\}\s*//\s*(.+?)\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:pub\s+)?(?:fn|struct|enum|trait|impl|type|const|static)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			blockCommentStart:  "/*",
			blockCommentEnd:    "*/",
			sectionStartFmt:    "",
			sectionEndFmt:      "",
			sectionBothFmt:     "",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*(?:pub(?:\(crate\))?\s+)?mod\s+\w+\s*\{\s*//\s*(\S.*?)\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*\}\s*//\s*(\S.*?)\s*$`),
		},
	}
}

// 🔤️FormatSectionStart MUST produce a well-formed Rust mod section start string.
func (l *RustLanguage) FormatSectionStart(name string) string {
	return fmt.Sprintf("mod %s { // 🔖️%s", RustSectionNameToModName(name), name)
}

// 📋️FormatSectionEnd MUST produce a well-formed Rust mod section end string.
func (l *RustLanguage) FormatSectionEnd(name string) string {
	return fmt.Sprintf("} // 🔖️%s", name)
}

// 🔷️FormatSectionBoth MUST produce a well-formed Rust mod section both string.
func (l *RustLanguage) FormatSectionBoth(name string) string {
	modName := RustSectionNameToModName(name)
	return fmt.Sprintf("\nmod %s { // 🔖️%s\n\n} // 🔖️%s\n", modName, name, name)
}

// 🔶️FormatHeader MUST produce a well-formed Rust mod header string.
func (l *RustLanguage) FormatHeader(fileId, fileUri, summary, contributors, license, requirements string) string {
	cp := l.commentPrefix
	var b strings.Builder
	b.WriteString(l.FormatSectionStart("Header"))
	b.WriteString("\n")
	b.WriteString(cp + " [" + fileId + "](" + fileUri + ")\n")
	for _, line := range strings.Split(contributors, "\n") {
		if strings.TrimSpace(line) != "" {
			b.WriteString(cp + " " + line + "\n")
		}
	}
	b.WriteString("\n")
	for _, line := range strings.Split(license, "\n") {
		if line == "" {
			b.WriteString(cp + "\n")
		} else {
			b.WriteString(cp + " " + line + "\n")
		}
	}
	b.WriteString("\n")
	if summary != "" {
		for _, line := range strings.Split(summary, "\n") {
			if line == "" {
				b.WriteString(cp + "\n")
			} else {
				b.WriteString(cp + " " + line + "\n")
			}
		}
		b.WriteString("\n")
	}
	if requirements != "" {
		for _, line := range strings.Split(requirements, "\n") {
			if line == "" {
				b.WriteString(cp + "\n")
			} else {
				b.WriteString(cp + " " + line + "\n")
			}
		}
		b.WriteString("\n")
	}
	b.WriteString(l.FormatSectionEnd("Header"))
	b.WriteString("\n")
	return b.String()
}

// 💬️ScanComments MUST detect legacy region comments in Rust files.
func (l *RustLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Breach {
	breachs := l.BaseLanguage.ScanComments(ctx, file, content, lines)
	legacyStart := regexp.MustCompile(`(?i)^\s*//\s*#region\b`)
	legacyEnd := regexp.MustCompile(`(?i)^\s*//\s*#endregion\b`)
	for i, line := range lines {
		if legacyStart.MatchString(line) || legacyEnd.MatchString(line) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Legacy region comment in Rust file %s:%d", file, i+1),
				BreachCodeRustRegionComment,
				file, i+1, 0, strings.TrimSpace(line)))
		}
	}
	return breachs
}

// 📖️ExtraOrphanDefinitions MUST operate on the RustLanguage receiver and return consistent results.
func (l *RustLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	modRegexp := regexp.MustCompile(`^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;`)
	for i, line := range lines {
		if match := modRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("mod-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}

// 🗣️RubyLanguage holds the data fields for a ruby language record.
type RubyLanguage struct {
	BaseLanguage
}

// 🔷️NewRubyLanguage MUST initialize all required fields and return a valid RubyLanguage.
// 🆕️NewRubyLanguage creates and returns a new RubyLanguage instance.
func NewRubyLanguage() *RubyLanguage {
	return &RubyLanguage{
		BaseLanguage: BaseLanguage{
			name:               "ruby",
			extensions:         []string{".rb", ".rake", ".gemspec"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:def|class|module)\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "#",
			blockCommentStart:  "=begin",
			blockCommentEnd:    "=end",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// 📖️ParseDefinitions MUST return an error when the input is malformed.
// 📖️ParseDefinitions parses the input and returns the definitions result.
func (l *RubyLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name  string
		line  int
		depth int
	}
	var defStack []defStart
	var defRanges []DefinitionRange
	endRegexp := regexp.MustCompile(`^\s*end\s*$`)
	blockStartRegexp := regexp.MustCompile(`(?:^|\s)(?:if|unless|case|while|until|for|begin|do)\b`)
	depth := 0
	for i, line := range lines {
		lineNum := i + 1
		trimmed := strings.TrimSpace(line)
		if matches := l.definitionRegexp.FindAllStringSubmatch(line, -1); matches != nil {
			for _, match := range matches {
				if len(match) > 1 && match[1] != "" {
					defStack = append(defStack, defStart{name: match[1], line: lineNum, depth: depth})
					depth++
				}
			}
		} else if blockStartRegexp.MatchString(line) && !strings.Contains(line, " do ") {
			depth++
		}
		if endRegexp.MatchString(trimmed) {
			if depth > 0 {
				depth--
			}
			for len(defStack) > 0 && defStack[len(defStack)-1].depth == depth {
				def := defStack[len(defStack)-1]
				defStack = defStack[:len(defStack)-1]
				defRanges = append(defRanges, DefinitionRange{
					Name:    def.name,
					Start:   def.line,
					End:     lineNum,
					Excerpt: def.name,
				})
			}
		}
	}
	for _, def := range defStack {
		defRanges = append(defRanges, DefinitionRange{
			Name:    def.name,
			Start:   def.line,
			End:     len(lines),
			Excerpt: def.name,
		})
	}
	sort.Slice(defRanges, func(i, j int) bool {
		return defRanges[i].Start < defRanges[j].Start
	})
	return defRanges
}

// 📥️ExtraOrphanDefinitions MUST operate on the RubyLanguage receiver and return consistent results.
func (l *RubyLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	moduleRegexp := regexp.MustCompile(`^\s*module\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`)
	for i, line := range lines {
		if match := moduleRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("module-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}
