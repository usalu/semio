// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎽️base carries the language plugin interface and the base language: formatting, section and definition parsing and import handling.

// #endregion 🧲️Header

package languages

import (
	fmt "fmt"
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎽️Languages

// 🔌️LanguagePlugin defines the interface contract for language program operations.
type LanguagePlugin interface {
	Name() string
	Extensions() []string
	MatchesExtension(ext string) bool
	SupportsSections() bool
	SupportsDefinitions() bool
	SupportsComments() bool
	SupportsHeaders() bool
	UsesIndentScoping() bool
	CommentPrefix() string
	BlockCommentStart() string
	BlockCommentEnd() string
	ParseSections(content string) []model.Section
	ParseDefinitions(content string, lines []string) []DefinitionRange
	FormatSectionStart(name string) string
	FormatSectionEnd(name string) string
	FormatSectionBoth(name string) string
	FormatHeader(fileId, fileUri, summary, contributors, license, requirements string) string
	PolicySectionStartMatch(line string) (matched bool, name string)
	PolicySectionEndMatch(line string) (matched bool, name string)
	ExtraOrphanDefinitions(lines []string) []DefinitionRange
	ScanComments(ctx CommentPolicy, file, content string, lines []string) []model.Breach
	SkipDirectives() []string
	ExtractImports(content string) ([]string, string)
	FormatImports(imports []string) string
	ExtractPackage(content string) (string, string)
}

// 📖️DefinitionRange holds the data fields for a definition range record.
type DefinitionRange struct {
	Name    string
	Kind    string
	Start   int
	End     int
	Excerpt string
}

// 🗣️BaseLanguage holds the data fields for a base language record.
type BaseLanguage struct {
	name               string
	extensions         []string
	sectionStart       *regexp.Regexp
	sectionEnd         *regexp.Regexp
	definitionRegexp   *regexp.Regexp
	commentPrefix      string
	blockCommentStart  string
	blockCommentEnd    string
	sectionStartFmt    string
	sectionEndFmt      string
	sectionBothFmt     string
	supportsHeaders    bool
	usesIndentScoping  bool
	policySectionStart *regexp.Regexp
	policySectionEnd   *regexp.Regexp
	hasTemplates       bool
	hasRawBackticks    bool
	hasTripleQuotes    bool
	hasVerbatimStrings bool
	hasJSDoc           bool
	skipDirectives     []string
}

// 📥️Name MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) Name() string { return l.name }

// 🔷️Extensions MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) Extensions() []string { return l.extensions }

// 🔧️CommentPrefix MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) CommentPrefix() string { return l.commentPrefix }

// 💬️BlockCommentStart MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) BlockCommentStart() string { return l.blockCommentStart }

// 🔶️BlockCommentEnd MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) BlockCommentEnd() string { return l.blockCommentEnd }

// 🔹️UsesIndentScoping MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) UsesIndentScoping() bool { return l.usesIndentScoping }

// 📑️SupportsSections MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) SupportsSections() bool { return l.sectionStart != nil }

// 🔸️SupportsDefinitions MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) SupportsDefinitions() bool { return l.definitionRegexp != nil }

// 🔺️SupportsComments MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) SupportsComments() bool { return l.commentPrefix != "" }

// 🔻️SupportsHeaders MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) SupportsHeaders() bool { return l.supportsHeaders }

// 🎯️MatchesExtension MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) MatchesExtension(ext string) bool {
	ext = strings.ToLower(ext)
	for _, langExt := range l.extensions {
		if ext == langExt {
			return true
		}
	}
	return false
}

// 🔤️FormatSectionStart MUST produce a well-formed section start string.
// 🖊️FormatSectionStart formats the section start into its string representation.
func (l *BaseLanguage) FormatSectionStart(name string) string {
	if l.sectionStartFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionStartFmt, name)
}

// 📋️FormatSectionEnd MUST produce a well-formed section end string.
// ⏹️FormatSectionEnd formats the section end into its string representation.
func (l *BaseLanguage) FormatSectionEnd(name string) string {
	if l.sectionEndFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionEndFmt, name)
}

// ⬛️FormatSectionBoth MUST produce a well-formed section both string.
// 🔤️FormatSectionBoth formats the section both into its string representation.
func (l *BaseLanguage) FormatSectionBoth(name string) string {
	if l.sectionBothFmt == "" {
		return ""
	}
	if l.sectionEndFmt == "" {
		return fmt.Sprintf(l.sectionBothFmt, name)
	}
	return fmt.Sprintf(l.sectionBothFmt, name, name)
}

// ⬜️FormatHeader MUST produce a well-formed header string.
// 🔢️FormatHeader formats the header into its string representation.
func (l *BaseLanguage) FormatHeader(fileId, fileUri, summary, contributors, license, requirements string) string {
	if !l.supportsHeaders {
		return ""
	}
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

// 📜️PolicySectionStartMatch MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) PolicySectionStartMatch(line string) (bool, string) {
	if l.policySectionStart == nil {
		return false, ""
	}
	match := l.policySectionStart.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		_, name = model.ExtractEntityEmoji(strings.TrimSpace(match[1]))
		name = strings.TrimSpace(name)
	}
	return true, name
}

// 🟥️PolicySectionEndMatch MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) PolicySectionEndMatch(line string) (bool, string) {
	if l.policySectionEnd == nil {
		return false, ""
	}
	match := l.policySectionEnd.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		_, name = model.ExtractEntityEmoji(strings.TrimSpace(match[1]))
		name = strings.TrimSpace(name)
	}
	return true, name
}

// ❌️ParseSections MUST return an error when the input is malformed.
// 📝️ParseSections parses the input and returns the sections result.
func (l *BaseLanguage) ParseSections(content string) []model.Section {
	if l.sectionStart == nil {
		return nil
	}
	lines := strings.Split(content, "\n")

	type sectionPtr struct {
		s        *model.Section
		children []*sectionPtr
	}

	var stack []*sectionPtr
	var roots []*sectionPtr
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := l.sectionStart.FindStringSubmatch(line); match != nil {
			rawName := strings.TrimSpace(match[1])
			emoji, name := model.ExtractEntityEmoji(rawName)
			name = strings.TrimSpace(name)
			if name == "" {
				name = rawName
			}
			s := &model.Section{
				Name:       name,
				Emoji:      emoji,
				StartLine:  lineNum,
				EndLine:    len(lines),
				StartIndex: lineStart,
				EndIndex:   len(content),
				Children:   nil,
			}
			sp := &sectionPtr{s: s}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.children = append(parent.children, sp)
			} else {
				roots = append(roots, sp)
			}
			stack = append(stack, sp)
		} else if l.sectionEnd != nil && l.sectionEnd.MatchString(line) {
			if len(stack) > 0 {
				sp := stack[len(stack)-1]
				sp.s.EndLine = lineNum
				sp.s.EndIndex = charIndex + len(line)
				stack = stack[:len(stack)-1]
			}
		}
		charIndex += len(line) + 1
	}

	var convert func(*sectionPtr) model.Section
	convert = func(sp *sectionPtr) model.Section {
		s := *sp.s
		if len(sp.children) > 0 {
			s.Children = make([]model.Section, len(sp.children))
			for i, child := range sp.children {
				s.Children[i] = convert(child)
			}
		}
		return s
	}

	result := make([]model.Section, len(roots))
	for i, root := range roots {
		result[i] = convert(root)
	}
	return result
}

// 🔬️ParseDefinitions MUST return an error when the input is malformed.
// ▶️ParseDefinitions parses the input and returns the definitions result.
func (l *BaseLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name string
		kind string
		line int
	}
	var defStarts []defStart
	for i, line := range lines {
		matches := l.definitionRegexp.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if len(match) > 1 && match[1] != "" {
				kind := extractDefinitionKeyword(match[0], match[1])
				defStarts = append(defStarts, defStart{name: match[1], kind: kind, line: i + 1})
			}
		}
	}
	var defRanges []DefinitionRange
	for i := 0; i < len(defStarts); i++ {
		start := defStarts[i].line
		end := start
		if l.usesIndentScoping {
			startIndent := len(lines[start-1]) - len(strings.TrimLeft(lines[start-1], " \t"))
			for lineIndex := start; lineIndex < len(lines); lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				currentIndent := len(line) - len(strings.TrimLeft(line, " \t"))
				if currentIndent <= startIndent {
					end = lineIndex
					break
				}
				end = lineIndex + 1
			}
		} else {
			braceDepth := 0
			sawOpen := false
			nextDefStart := len(lines)
			if i+1 < len(defStarts) {
				nextDefStart = defStarts[i+1].line - 1
			}
			for lineIndex := start - 1; lineIndex < len(lines); lineIndex++ {
				if !sawOpen && lineIndex >= nextDefStart {
					break
				}
				line := lines[lineIndex]
				for _, ch := range line {
					if ch == '{' {
						braceDepth++
						sawOpen = true
					} else if ch == '}' {
						if braceDepth > 0 {
							braceDepth--
						}
						if sawOpen && braceDepth == 0 {
							end = lineIndex + 1
							lineIndex = len(lines)
							break
						}
					}
				}
				if sawOpen && braceDepth == 0 && end > start {
					break
				}
			}
			if !sawOpen {
				if i+1 < len(defStarts) {
					end = defStarts[i+1].line - 1
				}
			}
		}
		if end < start {
			end = start
		}
		defRanges = append(defRanges, DefinitionRange{
			Name:    defStarts[i].name,
			Kind:    refineDefinitionKind(defStarts[i].kind, lines[start-1]),
			Start:   start,
			End:     end,
			Excerpt: defStarts[i].name,
		})
	}
	return defRanges
}

// 🧩️arrowFuncPattern holds the data fields for a arrowFuncPattern record.
var arrowFuncPattern = regexp.MustCompile(`=\s*(?:\([^)]*\)|[A-Za-z_][A-Za-z0-9_]*)\s*(?::\s*[^=]+)?\s*=>\s*`)

// 💿️funcExprPattern holds the data fields for a funcExprPattern record.
var funcExprPattern = regexp.MustCompile(`=\s*(?:async\s+)?function\b`)

// 🏛️classExprPattern holds the data fields for a classExprPattern record.
var classExprPattern = regexp.MustCompile(`=\s*class\b`)

// 🏷️refineDefinitionKind holds the data fields for a refineDefinitionKind record.
func refineDefinitionKind(rawKind string, line string) string {
	lower := strings.ToLower(rawKind)
	if lower == "const" || lower == "let" || lower == "var" {
		if arrowFuncPattern.MatchString(line) || funcExprPattern.MatchString(line) || classExprPattern.MatchString(line) {
			return "function"
		}
	}
	return rawKind
}

// 🧲️extractDefinitionKeyword holds the data fields for a extractDefinitionKeyword record.
func extractDefinitionKeyword(fullMatch, name string) string {
	modifiers := map[string]bool{
		"public": true, "private": true, "protected": true, "internal": true,
		"abstract": true, "sealed": true, "virtual": true, "override": true,
		"async": true, "partial": true, "pub": true, "export": true, "static": true,
	}
	keywords := map[string]bool{
		"function": true, "class": true, "interface": true, "type": true, "enum": true,
		"const": true, "let": true, "var": true, "func": true, "fn": true,
		"struct": true, "trait": true, "impl": true, "mod": true, "static": true,
		"def": true, "module": true, "delegate": true, "record": true, "union": true,
		"scalar": true, "query": true, "mutation": true, "subscription": true, "fragment": true,
		"input": true,
	}
	multiWordKeywords := []string{
		"async def", "async function",
		"extend type", "extend interface", "extend enum", "extend union", "extend input",
		"CREATE TABLE", "CREATE VIEW", "CREATE PROCEDURE", "CREATE FUNCTION", "CREATE TRIGGER",
		"CREATE INDEX", "CREATE TYPE", "CREATE SCHEMA", "CREATE DATABASE", "CREATE SEQUENCE",
	}
	lower := strings.ToLower(fullMatch)
	for _, kw := range multiWordKeywords {
		if strings.Contains(lower, strings.ToLower(kw)) {
			return kw
		}
	}
	words := strings.Fields(lower)
	lowerName := strings.ToLower(name)
	var prevWord string
	for _, word := range words {
		clean := strings.TrimRight(word, "(<{[")
		if clean == lowerName {
			break
		}
		prevWord = clean
	}
	if prevWord != "" && keywords[prevWord] {
		return prevWord
	}
	for _, word := range words {
		clean := strings.TrimRight(word, "(<{[")
		if clean == lowerName || modifiers[clean] {
			continue
		}
		if keywords[clean] {
			return clean
		}
	}
	return "definition"
}

// 🟧️ExtraOrphanDefinitions MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	return nil
}

// 🟨️SkipDirectives MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) SkipDirectives() []string {
	builtIn := []string{"TODO", "compose-ignore-"}
	return append(builtIn, l.skipDirectives...)
}

// 💻️ExtractImports MUST return the extracted value without side effects.
// ⬛️ExtractImports extracts the imports from the given input.
func (l *BaseLanguage) ExtractImports(content string) ([]string, string) {
	return []string{}, content
}

// 🟩️FormatImports MUST produce a well-formed imports string.
// 📩️FormatImports formats the imports into its string representation.
func (l *BaseLanguage) FormatImports(imports []string) string {
	return ""
}

// 🟦️ExtractPackage MUST return the extracted value without side effects.
// ⬜️ExtractPackage extracts the package from the given input.
func (l *BaseLanguage) ExtractPackage(content string) (string, string) {
	return "", content
}

// #endregion 🎽️Languages
