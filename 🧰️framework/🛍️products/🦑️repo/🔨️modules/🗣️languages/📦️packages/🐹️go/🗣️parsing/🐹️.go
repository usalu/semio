// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗣️parsing carries the parsed section and definition types and the parsers of region markers, headings, definitions and scopes.

// #endregion 🧲️Header

package languages

import (
	fmt "fmt"
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"
)

// #region 🗣️Parsing

// 📑️ParsedSection represents a parsed source code section from region markers or markdown headings.
type ParsedSection struct {
	Name      string
	Path      string
	StartLine int
	EndLine   int
}

// 📖️ParsedDefinition represents a parsed source code definition from regex patterns.
type ParsedDefinition struct {
	Name      string
	StartLine int
	EndLine   int
}

// 🎭️IsEmojiRune returns true if the rune is likely an emoji base character.
func IsEmojiRune(r rune) bool {
	if r >= 0x1F600 && r <= 0x1F64F {
		return true
	}
	if r >= 0x1F300 && r <= 0x1F5FF {
		return true
	}
	if r >= 0x1F680 && r <= 0x1F6FF {
		return true
	}
	if r >= 0x1F700 && r <= 0x1F77F {
		return true
	}
	if r >= 0x1F780 && r <= 0x1F7FF {
		return true
	}
	if r >= 0x1F800 && r <= 0x1F8FF {
		return true
	}
	if r >= 0x1F900 && r <= 0x1F9FF {
		return true
	}
	if r >= 0x1FA00 && r <= 0x1FA6F {
		return true
	}
	if r >= 0x1FA70 && r <= 0x1FAFF {
		return true
	}
	if r >= 0x2600 && r <= 0x26FF {
		return true
	}
	if r >= 0x2700 && r <= 0x27BF {
		return true
	}
	if r >= 0x2300 && r <= 0x23FF {
		return true
	}
	if r >= 0x2B50 && r <= 0x2B55 {
		return true
	}
	if r >= 0x200D && r <= 0x200D {
		return true
	}
	if r >= 0xFE00 && r <= 0xFE0F {
		return true
	}
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
	if r >= 0x25B6 && r <= 0x25C0 {
		return true
	}
	if r >= 0x25FB && r <= 0x25FE {
		return true
	}
	if r >= 0x2614 && r <= 0x2615 {
		return true
	}
	if r >= 0x2648 && r <= 0x2653 {
		return true
	}
	if r >= 0x267F && r <= 0x267F {
		return true
	}
	if r >= 0x2693 && r <= 0x2693 {
		return true
	}
	if r >= 0x26A1 && r <= 0x26A1 {
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
	if r >= 0x26CE && r <= 0x26CF {
		return true
	}
	if r >= 0x26D4 && r <= 0x26D4 {
		return true
	}
	if r >= 0x26EA && r <= 0x26EA {
		return true
	}
	if r >= 0x26F2 && r <= 0x26F3 {
		return true
	}
	if r >= 0x26F5 && r <= 0x26F5 {
		return true
	}
	if r >= 0x26FA && r <= 0x26FA {
		return true
	}
	if r >= 0x26FD && r <= 0x26FD {
		return true
	}
	if r == 0x203C || r == 0x2049 {
		return true
	}
	if r == 0x20E3 {
		return true
	}
	if r == 0x00A9 || r == 0x00AE {
		return true
	}
	if r == 0x2122 {
		return true
	}
	return false
}

// 🧲️ExtractEntityEmoji extracts the leading emoji and remaining text from a string.
func ExtractEntityEmoji(s string) (string, string) {
	if s == "" {
		return "", ""
	}
	runes := []rune(s)
	if len(runes) == 0 {
		return "", ""
	}
	i := 0
	r := runes[i]
	if !IsEmojiRune(r) {
		return "", s
	}
	i++
	for i < len(runes) {
		r = runes[i]
		if r == 0xFE0F || r == 0xFE0E {
			i++
		} else if r == 0x20E3 {
			i++
		} else if r == 0x200D {
			i++
			if i < len(runes) {
				i++
				for i < len(runes) && (runes[i] == 0xFE0F || runes[i] == 0xFE0E) {
					i++
				}
			}
		} else if r >= 0x1F3FB && r <= 0x1F3FF {
			i++
		} else {
			break
		}
	}
	emoji := string(runes[:i])
	remaining := string(runes[i:])
	return emoji, remaining
}

// 💬️ParseRegionMarker detects region start/end markers in a line, stripping common comment prefixes.
// 💬️Supports any emoji prefix (e.g. #region 📋️EventKind, #region 🔖️Legacy).
func ParseRegionMarker(line string) (string, bool, bool) {
	trimmed := strings.TrimSpace(line)
	trimmed = strings.TrimPrefix(trimmed, "//")
	trimmed = strings.TrimPrefix(trimmed, "#")
	trimmed = strings.TrimPrefix(trimmed, "--")
	trimmed = strings.TrimPrefix(trimmed, "/*")
	trimmed = strings.TrimSuffix(trimmed, "*/")
	trimmed = strings.TrimSpace(trimmed)
	if strings.HasPrefix(trimmed, "#region ") {
		content := strings.TrimSpace(strings.TrimPrefix(trimmed, "#region "))
		emoji, name := ExtractEntityEmoji(content)
		if emoji != "" {
			return name, true, false
		}
	}
	if strings.HasPrefix(trimmed, "#endregion ") {
		content := strings.TrimSpace(strings.TrimPrefix(trimmed, "#endregion "))
		emoji, name := ExtractEntityEmoji(content)
		if emoji != "" {
			return name, true, true
		}
	}
	return "", false, false
}

// 🔬️ParseMarkdownHeading parses a markdown heading line into level and title.
func ParseMarkdownHeading(line string) (int, string) {
	trimmed := strings.TrimSpace(line)
	if !strings.HasPrefix(trimmed, "#") {
		return 0, ""
	}
	level := 0
	for level < len(trimmed) && trimmed[level] == '#' {
		level++
	}
	if level == 0 || level > 6 {
		return 0, ""
	}
	name := strings.TrimSpace(trimmed[level:])
	if name == "" {
		return 0, ""
	}
	return level, name
}

// 🗣️DefinitionPatterns returns language-specific regex patterns for extracting definitions by file extension.
func DefinitionPatterns(ext string) []*regexp.Regexp {
	switch ext {
	case ".go":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*func\s+(?:\([^\)]*\)\s*)?([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*type\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*var\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*const\s+([A-Za-z0-9_]+)`),
		}
	case ".ts", ".tsx", ".js", ".jsx":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?class\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?interface\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?type\s+([A-Za-z0-9_]+)`),
		}
	case ".py":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*def\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*class\s+([A-Za-z0-9_]+)`),
		}
	case ".cs":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:class|struct|interface|enum|record)\s+([A-Za-z0-9_]+)`),
		}
	case ".rs":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?struct\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?enum\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?trait\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*impl\s+([A-Za-z0-9_]+)`),
		}
	case ".rb":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*def\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*class\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*module\s+([A-Za-z0-9_]+)`),
		}
	case ".md", ".mdx":
		return []*regexp.Regexp{}
	default:
		return []*regexp.Regexp{}
	}
}

// 🌳️ParseSectionsFromLines extracts sections from source lines using region markers and markdown headings.
func ParseSectionsFromLines(lines []string, ext string) []ParsedSection {
	var sections []ParsedSection
	type sectionFrame struct {
		Name      string
		StartLine int
		Level     int
		Path      string
	}
	var stack []sectionFrame
	for index, line := range lines {
		lineNumber := index + 1
		if name, ok, isEnd := ParseRegionMarker(line); ok {
			if isEnd {
				if len(stack) > 0 {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					sections = append(sections, ParsedSection{
						Name:      frame.Name,
						Path:      frame.Path,
						StartLine: frame.StartLine,
						EndLine:   lineNumber - 1,
					})
				}
			} else {
				path := name
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + name
				}
				stack = append(stack, sectionFrame{Name: name, StartLine: lineNumber, Level: 0, Path: path})
			}
			continue
		}
		if ext == ".md" || ext == ".mdx" {
			if level, title := ParseMarkdownHeading(line); level > 0 {
				for len(stack) > 0 && stack[len(stack)-1].Level >= level {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					sections = append(sections, ParsedSection{
						Name:      frame.Name,
						Path:      frame.Path,
						StartLine: frame.StartLine,
						EndLine:   lineNumber - 1,
					})
				}
				path := title
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + title
				}
				stack = append(stack, sectionFrame{Name: title, StartLine: lineNumber, Level: level, Path: path})
			}
		}
	}
	for _, frame := range stack {
		sections = append(sections, ParsedSection{
			Name:      frame.Name,
			Path:      frame.Path,
			StartLine: frame.StartLine,
			EndLine:   len(lines),
		})
	}
	return sections
}

// 🧩️ParseDefinitionsFromLines extracts definitions from source lines using the given regex patterns.
func ParseDefinitionsFromLines(lines []string, patterns []*regexp.Regexp) []ParsedDefinition {
	var defs []ParsedDefinition
	for index, line := range lines {
		lineNumber := index + 1
		for _, pattern := range patterns {
			matches := pattern.FindStringSubmatch(line)
			if len(matches) > 1 {
				defs = append(defs, ParsedDefinition{
					Name:      matches[len(matches)-1],
					StartLine: lineNumber,
					EndLine:   lineNumber,
				})
				break
			}
		}
	}
	return defs
}

// 🔭️BuildScopeID generates a deterministic scope ID from kind, file path, section path, and definition name.
func BuildScopeID(kind string, filePath string, sectionPath string, definition string) string {
	if kind == "file" {
		return fmt.Sprintf("file:%s", filePath)
	}
	if kind == "section" {
		return fmt.Sprintf("section:%s#%s", filePath, sectionPath)
	}
	if sectionPath != "" {
		return fmt.Sprintf("def:%s#%s::%s", filePath, sectionPath, definition)
	}
	return fmt.Sprintf("def:%s#%s", filePath, definition)
}

// 📍️ScopeEntry holds kind, id, file, section, and definition for a parsed scope.
type ScopeEntry struct {
	Kind        string
	ID          string
	FilePath    string
	SectionPath string
	Definition  string
	StartLine   int
	EndLine     int
}

// 🏗️BuildScopesForFile parses file content into scope entries for file, sections, and definitions.
func BuildScopesForFile(path string, content string) []ScopeEntry {
	lines := strings.Split(content, "\n")
	ext := strings.ToLower(filepath.Ext(path))

	var entries []ScopeEntry

	fileEntry := ScopeEntry{
		Kind:      "file",
		ID:        BuildScopeID("file", path, "", ""),
		FilePath:  path,
		StartLine: 1,
		EndLine:   len(lines),
	}
	entries = append(entries, fileEntry)

	sections := ParseSectionsFromLines(lines, ext)
	for _, s := range sections {
		entry := ScopeEntry{
			Kind:        "section",
			ID:          BuildScopeID("section", path, s.Path, ""),
			FilePath:    path,
			SectionPath: s.Path,
			StartLine:   s.StartLine,
			EndLine:     s.EndLine,
		}
		entries = append(entries, entry)
	}

	sectionByLine := map[int]string{}
	for _, s := range sections {
		for line := s.StartLine; line <= s.EndLine; line++ {
			sectionByLine[line] = s.Path
		}
	}

	patterns := DefinitionPatterns(ext)
	defs := ParseDefinitionsFromLines(lines, patterns)
	for _, d := range defs {
		sp := sectionByLine[d.StartLine]
		entry := ScopeEntry{
			Kind:        "definition",
			ID:          BuildScopeID("definition", path, sp, d.Name),
			FilePath:    path,
			SectionPath: sp,
			Definition:  d.Name,
			StartLine:   d.StartLine,
			EndLine:     d.EndLine,
		}
		entries = append(entries, entry)
	}

	return entries
}

// #endregion 🗣️Parsing
