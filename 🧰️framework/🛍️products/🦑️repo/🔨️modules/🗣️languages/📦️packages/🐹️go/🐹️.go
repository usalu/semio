// Package languages owns the repo language table and the parsers built on it.
//
// The table itself is the single source of truth shared with the Rust twin
// (semio-framework-repo-languages), which embeds the same file with include_str!. Nothing in this
// package may re-declare a comment prefix, a region marker, a definition keyword or a header format:
// read it from the table.
package languages

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	runtime "runtime"
	sort "sort"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	yaml "github.com/usalu/semio/repo/yaml"
)

// #region 🔖️Table

// 🔤️Token is one token of the declarative pattern language of 🧬️schema/🔣️.json.
type Token struct {
	T    string          `json:"t"`
	V    json.RawMessage `json:"v,omitempty"`
	Of   json.RawMessage `json:"of,omitempty"`
	Name string          `json:"name,omitempty"`
	Sep  string          `json:"sep,omitempty"`
	Stop string          `json:"stop,omitempty"`
	Min  int             `json:"min,omitempty"`
	Max  int             `json:"max,omitempty"`
}

// 🧵️Pattern is an anchored line pattern.
type Pattern struct {
	Ci     bool    `json:"ci,omitempty"`
	Tokens []Token `json:"tokens"`
}

// 🗣️Language is one language of the table.
type Language struct {
	Name                        string             `json:"name"`
	Emoji                       string             `json:"emoji"`
	Extensions                  []string           `json:"extensions"`
	SectionEngine               string             `json:"sectionEngine"`
	SectionStart                *Pattern           `json:"sectionStart,omitempty"`
	SectionEnd                  *Pattern           `json:"sectionEnd,omitempty"`
	PolicySectionStart          *Pattern           `json:"policySectionStart,omitempty"`
	PolicySectionEnd            *Pattern           `json:"policySectionEnd,omitempty"`
	Definition                  *Pattern           `json:"definition,omitempty"`
	DefinitionEngine            string             `json:"definitionEngine,omitempty"`
	OrphanDefinitions           string             `json:"orphanDefinitions,omitempty"`
	AuxPatterns                 map[string]Pattern `json:"auxPatterns,omitempty"`
	CommentPrefix               string             `json:"commentPrefix,omitempty"`
	BlockCommentStart           string             `json:"blockCommentStart,omitempty"`
	BlockCommentEnd             string             `json:"blockCommentEnd,omitempty"`
	SectionStartFormat          string             `json:"sectionStartFormat,omitempty"`
	SectionEndFormat            string             `json:"sectionEndFormat,omitempty"`
	SectionBothFormat           string             `json:"sectionBothFormat,omitempty"`
	SupportsHeaders             bool               `json:"supportsHeaders"`
	UsesIndentScoping           bool               `json:"usesIndentScoping"`
	SupportsDefinitionsOverride *bool              `json:"supportsDefinitionsOverride,omitempty"`
	SupportsCommentsOverride    *bool              `json:"supportsCommentsOverride,omitempty"`
	SkipDirectives              []string           `json:"skipDirectives,omitempty"`
	StringFeatures              []string           `json:"stringFeatures,omitempty"`
	SectionNaming               string             `json:"sectionNaming,omitempty"`
}

// 🔑️DefinitionKeywords is the vocabulary that turns a matched definition line into a raw kind.
type DefinitionKeywords struct {
	Modifiers []string `json:"modifiers"`
	Keywords  []string `json:"keywords"`
	MultiWord []string `json:"multiWord"`
	Fallback  string   `json:"fallback"`
}

// ✨️Refinements are the callable-initialiser promotions applied to a definition line.
type Refinements struct {
	ArrowFunction      Pattern  `json:"arrowFunction"`
	FunctionExpression Pattern  `json:"functionExpression"`
	ClassExpression    Pattern  `json:"classExpression"`
	Promotes           []string `json:"promotes"`
	PromotedTo         string   `json:"promotedTo"`
}

// 💬️ScopeRegionMarker is the comment-prefix-agnostic region marker reader of the scope builder.
type ScopeRegionMarker struct {
	StripPrefixes []string `json:"stripPrefixes"`
	StripSuffixes []string `json:"stripSuffixes"`
	StartKeyword  string   `json:"startKeyword"`
	EndKeyword    string   `json:"endKeyword"`
}

// 🗂️Table is the whole language table.
type Table struct {
	SchemaVersion           uint32               `json:"schemaVersion"`
	Registry                []string             `json:"registry"`
	DefinitionKeywords      DefinitionKeywords   `json:"definitionKeywords"`
	DefinitionKindMap       map[string]string    `json:"definitionKindMap"`
	Refinements             Refinements          `json:"refinements"`
	ScopeRegionMarker       ScopeRegionMarker    `json:"scopeRegionMarker"`
	ScopeDefinitionPatterns map[string][]Pattern `json:"scopeDefinitionPatterns"`
	Languages               []Language           `json:"languages"`
}

// 📄️TableEnvVar overrides the table location, for hosts that relocate the module tree.
const TableEnvVar = "SEMIO_REPO_LANGUAGES_TABLE"

var (
	table     Table
	tablePath string
	tableErr  error
)

func init() {
	tablePath, tableErr = resolveTablePath()
	if tableErr != nil {
		return
	}
	raw, err := os.ReadFile(tablePath)
	if err != nil {
		tableErr = err
		return
	}
	tableErr = json.Unmarshal(raw, &table)
}

// 🧭️resolveTablePath locates 🧬️schema/🔣️languages.json relative to this source file, which keeps the
// table a module-relative asset rather than a copy embedded per package.
func resolveTablePath() (string, error) {
	if override := os.Getenv(TableEnvVar); override != "" {
		return override, nil
	}
	_, self, _, ok := runtime.Caller(0)
	if ok {
		candidate := filepath.Join(filepath.Dir(self), "..", "..", "🧬️schema", "🔣️languages.json")
		if _, err := os.Stat(candidate); err == nil {
			return filepath.Clean(candidate), nil
		}
	}
	dir, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for {
		candidate := filepath.Join(dir, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🗣️languages", "🧬️schema", "🔣️languages.json")
		if _, statErr := os.Stat(candidate); statErr == nil {
			return candidate, nil
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "", os.ErrNotExist
		}
		dir = parent
	}
}

// 🗂️LoadTable returns the parsed language table and the error the loader recorded at init.
func LoadTable() (*Table, error) {
	if tableErr != nil {
		return nil, tableErr
	}
	return &table, nil
}

// 📄️TablePath reports where the table was read from.
func TablePath() string { return tablePath }

// 📖️LanguageByName returns the language registered under name.
func LanguageByName(name string) *Language {
	if tableErr != nil {
		return nil
	}
	for _, registered := range table.Registry {
		if registered != name {
			continue
		}
		for i := range table.Languages {
			if table.Languages[i].Name == name {
				return &table.Languages[i]
			}
		}
	}
	return nil
}

// 🏪️LanguageForPath returns the language claiming a path's extension, honouring registry order.
func LanguageForPath(path string) *Language {
	if tableErr != nil {
		return nil
	}
	ext := lowerASCII(filepath.Ext(path))
	if ext == "" {
		return nil
	}
	for _, name := range table.Registry {
		for i := range table.Languages {
			if table.Languages[i].Name != name {
				continue
			}
			for _, candidate := range table.Languages[i].Extensions {
				if candidate == ext {
					return &table.Languages[i]
				}
			}
		}
	}
	return nil
}

func lowerASCII(s string) string {
	out := []byte(s)
	for i, b := range out {
		if b >= 'A' && b <= 'Z' {
			out[i] = b + 32
		}
	}
	return string(out)
}

// #endregion 🔖️Table

// #region 🗣️Parsing

// 🧭️ Source-parsing primitives moved here from 📡️events, where the wave-1 agents parked them
// until this module existed.

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

// #region 🚚️Split

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

// 🧭️CommentTemplateState tracks nested template literal expression depth during comment scans.
type CommentTemplateState struct {
	ExprDepth int
}

// 🧵️CommentScanState tracks string, template, and block-comment state during comment scans.
type CommentScanState struct {
	InBlockComment          bool
	BlockCommentStartLine   int
	BlockCommentStartIndex  int
	BlockCommentStartColumn int
	BlockCommentIsJsDoc     bool
	BlockCommentHasTodo     bool
	InTodoBlock             bool
	Escaped                 bool
	InSingleQuote           bool
	InDoubleQuote           bool
	InTripleDouble          bool
	InTripleSingle          bool
	InRawBacktick           bool
	InVerbatimString        bool
	Templates               []CommentTemplateState
}

// 🪡️InTemplateRaw reports whether scanning is inside template literal text, not an expression.
func (s *CommentScanState) InTemplateRaw() bool {
	return len(s.Templates) > 0 && s.Templates[len(s.Templates)-1].ExprDepth == 0
}

// 📡️ScanComments MUST operate on the BaseLanguage receiver and return consistent results.
func (l *BaseLanguage) ScanComments(ctx CommentPolicy, file, content string, lines []string) []model.Breach {
	if l.commentPrefix == "" {
		return nil
	}
	if model.DeriveFileKind(file) == model.FileKindConfig {
		return nil
	}
	var breachs []model.Breach
	sections := l.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	prefix := l.commentPrefix
	prefixLen := len(prefix)
	blockStart := l.blockCommentStart
	blockEnd := l.blockCommentEnd
	charLevelBlock := blockStart == "/*" && blockEnd == "*/"
	charIndex := 0
	scanState := CommentScanState{}
	inlineCommentActive := false
	allDirectives := l.SkipDirectives()
	for i, line := range lines {
		lineNum := i + 1

		if i == 0 && strings.HasPrefix(strings.TrimSpace(line), "#!") {
			charIndex += len(line) + 1
			continue
		}

		if headerSection != nil && lineNum >= headerSection.StartLine && lineNum <= headerSection.EndLine {
			charIndex += len(line) + 1
			continue
		}
		if scanState.InBlockComment && !charLevelBlock {
			trimmed := strings.TrimSpace(line)
			if trimmed == blockEnd {
				if !scanState.BlockCommentHasTodo && !ctx.IsSpecBlock(file, scanState.BlockCommentStartLine, lineNum, lines) {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
						model.BreachCodeCommentBlock,
						file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
				}
				scanState.InBlockComment = false
				scanState.BlockCommentHasTodo = false
			} else if strings.Contains(line, "TODO") {
				scanState.BlockCommentHasTodo = true
			}
			charIndex += len(line) + 1
			continue
		}
		if !scanState.InBlockComment && !charLevelBlock && blockStart != "" {
			trimmed := strings.TrimSpace(line)
			if trimmed == blockStart {
				scanState.InBlockComment = true
				scanState.BlockCommentStartLine = lineNum
				scanState.BlockCommentStartColumn = 1
				scanState.BlockCommentHasTodo = strings.Contains(line, "TODO")
				charIndex += len(line) + 1
				continue
			}
		}
		if scanState.InTodoBlock && strings.TrimSpace(line) == "" {
			scanState.InTodoBlock = false
		}
		lineStart := charIndex
		j := 0
		foundInline := false
		for j < len(line) {
			if scanState.InBlockComment {
				if !scanState.BlockCommentHasTodo && strings.Contains(line, "TODO") {
					scanState.BlockCommentHasTodo = true
				}
				if j+1 < len(line) && line[j] == '*' && line[j+1] == '/' {
					if !scanState.BlockCommentHasTodo && !ctx.IsSpecBlock(file, scanState.BlockCommentStartLine, lineNum, lines) {
						if l.hasJSDoc && scanState.BlockCommentIsJsDoc {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("JSDoc comment in %s:%d", file, scanState.BlockCommentStartLine),
								model.BreachCodeCommentJSDoc,
								file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
						} else {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
								model.BreachCodeCommentBlock,
								file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
						}
					}
					scanState.InBlockComment = false
					scanState.BlockCommentHasTodo = false
					j += 2
					continue
				}
				j++
				continue
			}
			if scanState.Escaped {
				scanState.Escaped = false
				j++
				continue
			}
			if line[j] == '\\' && (scanState.InSingleQuote || scanState.InDoubleQuote || (l.hasTemplates && scanState.InTemplateRaw())) {
				scanState.Escaped = true
				j++
				continue
			}
			if l.hasTripleQuotes {
				if scanState.InTripleDouble {
					if j+2 < len(line) && line[j] == '"' && line[j+1] == '"' && line[j+2] == '"' {
						scanState.InTripleDouble = false
						j += 3
						continue
					}
					j++
					continue
				}
				if scanState.InTripleSingle {
					if j+2 < len(line) && line[j] == '\'' && line[j+1] == '\'' && line[j+2] == '\'' {
						scanState.InTripleSingle = false
						j += 3
						continue
					}
					j++
					continue
				}
			}
			if scanState.InSingleQuote {
				if line[j] == '\'' {
					scanState.InSingleQuote = false
				}
				j++
				continue
			}
			if scanState.InDoubleQuote {
				if line[j] == '"' {
					scanState.InDoubleQuote = false
				}
				j++
				continue
			}
			if l.hasTemplates && scanState.InTemplateRaw() {
				if line[j] == '`' {
					scanState.Templates = scanState.Templates[:len(scanState.Templates)-1]
					j++
					continue
				}
				if j+1 < len(line) && line[j] == '$' && line[j+1] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth = 1
					j += 2
					continue
				}
				j++
				continue
			}
			if l.hasTemplates && len(scanState.Templates) > 0 && scanState.Templates[len(scanState.Templates)-1].ExprDepth > 0 {
				if line[j] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth++
					j++
					continue
				}
				if line[j] == '}' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth--
					j++
					continue
				}
			}
			if l.hasRawBackticks && scanState.InRawBacktick {
				if line[j] == '`' {
					scanState.InRawBacktick = false
				}
				j++
				continue
			}
			if l.hasVerbatimStrings && scanState.InVerbatimString {
				if j+1 < len(line) && line[j] == '"' && line[j+1] == '"' {
					j += 2
					continue
				}
				if line[j] == '"' {
					scanState.InVerbatimString = false
				}
				j++
				continue
			}
			if l.hasTripleQuotes {
				if j+2 < len(line) && line[j] == '"' && line[j+1] == '"' && line[j+2] == '"' {
					scanState.InTripleDouble = true
					j += 3
					continue
				}
				if j+2 < len(line) && line[j] == '\'' && line[j+1] == '\'' && line[j+2] == '\'' {
					scanState.InTripleSingle = true
					j += 3
					continue
				}
			}
			if line[j] == '\'' {
				scanState.InSingleQuote = true
				j++
				continue
			}
			if line[j] == '"' {
				if l.hasVerbatimStrings && j > 0 && line[j-1] == '@' {
					scanState.InVerbatimString = true
					j++
					continue
				}
				scanState.InDoubleQuote = true
				j++
				continue
			}
			if l.hasTemplates && line[j] == '`' {
				scanState.Templates = append(scanState.Templates, CommentTemplateState{ExprDepth: 0})
				j++
				continue
			}
			if l.hasRawBackticks && line[j] == '`' {
				scanState.InRawBacktick = true
				j++
				continue
			}
			if charLevelBlock && j+1 < len(line) && line[j] == '/' && line[j+1] == '*' {
				isJsDoc := l.hasJSDoc && j+2 < len(line) && line[j+2] == '*'
				scanState.InBlockComment = true
				scanState.BlockCommentStartLine = lineNum
				scanState.BlockCommentStartIndex = lineStart + j
				scanState.BlockCommentStartColumn = j + 1
				scanState.BlockCommentIsJsDoc = isJsDoc
				scanState.BlockCommentHasTodo = strings.Contains(line[j:], "TODO")
				j += 2
				continue
			}
			if j+prefixLen <= len(line) && line[j:j+prefixLen] == prefix {
				if prefix == "//" && j > 0 && line[j-1] == ':' {
					j += prefixLen
					continue
				}
				if prefix == "//" && j > 0 && line[j-1] == '\\' {
					j += prefixLen
					continue
				}
				if matched, _ := l.PolicySectionStartMatch(line); matched {
					break
				}
				if matched, _ := l.PolicySectionEndMatch(line); matched {
					break
				}
				trimmed := strings.TrimSpace(line)
				commentText := strings.TrimSpace(line[j:])

				if lineNum == 1 && strings.HasPrefix(trimmed, "#!") {
					fmt.Printf("[DEBUG] Ignoring shebang at line 1: %s\n", trimmed)
					break
				}

				shouldSkip := false
				for _, directive := range allDirectives {
					if strings.HasPrefix(trimmed, prefix+" "+directive) || strings.HasPrefix(commentText, prefix+" "+directive) || strings.HasPrefix(commentText, prefix+directive) {
						shouldSkip = true
						if directive == "TODO" {
							scanState.InTodoBlock = true
						}
						break
					}
				}
				if shouldSkip {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if scanState.InTodoBlock && strings.HasPrefix(trimmed, prefix) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsSpecLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsSectionDocLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsDefinitionDocLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				scanState.InTodoBlock = false
				debugMarker := strings.Contains(line, "[DEBUG]")
				if !debugMarker {
					foundInline = true
					if !inlineCommentActive {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
							model.BreachCodeCommentInline,
							file, lineNum, j+1, strings.TrimSpace(line[j:])))
						inlineCommentActive = true
					}
				}
				break
			}
			j++
		}
		if !foundInline {
			scanState.InTodoBlock = false
			inlineCommentActive = false
		}
		charIndex += len(line) + 1
	}
	return breachs
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

// #region 🎶️TypeScript

// 🗣️TypeScriptLanguage holds the data fields for a type script language record.
type TypeScriptLanguage struct {
	BaseLanguage
}

// 🏷️NewTypeScriptLanguage MUST initialize all required fields and return a valid TypeScriptLanguage.
// 🆕️NewTypeScriptLanguage creates and returns a new TypeScriptLanguage instance.
func NewTypeScriptLanguage() *TypeScriptLanguage {
	return &TypeScriptLanguage{
		BaseLanguage: BaseLanguage{
			name:               "typescript",
			extensions:         []string{".ts", ".tsx", ".js", ".jsx"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:export\s+)?(?:(?:async|abstract|declare|default)\s+)*(?:const|let|var|function|class|interface|type|enum)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			blockCommentStart:  "/*",
			blockCommentEnd:    "*/",
			sectionStartFmt:    "// #region 🔖️%s",
			sectionEndFmt:      "// #endregion 🔖️%s",
			sectionBothFmt:     "\n// #region 🔖️%s\n\n// #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// 📥️ScanComments MUST operate on the TypeScriptLanguage receiver and return consistent results.
func (l *TypeScriptLanguage) ScanComments(ctx CommentPolicy, file, content string, lines []string) []model.Breach {
	if model.DeriveFileKind(file) == model.FileKindConfig {
		return nil
	}
	var breachs []model.Breach

	sections := l.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	charIndex := 0
	scanState := CommentScanState{}
	inlineCommentActive := false
	for i, line := range lines {
		lineNum := i + 1

		if headerSection != nil && lineNum >= headerSection.StartLine && lineNum <= headerSection.EndLine {
			charIndex += len(line) + 1
			continue
		}
		if scanState.InTodoBlock && strings.TrimSpace(line) == "" {
			scanState.InTodoBlock = false
		}
		lineStart := charIndex
		foundInline := false
		for j := 0; j < len(line); {
			if scanState.InBlockComment {
				if !scanState.BlockCommentHasTodo && strings.Contains(line, "TODO") {
					scanState.BlockCommentHasTodo = true
				}
				if j+1 < len(line) && line[j] == '*' && line[j+1] == '/' {
					isDefDocstring := false
					if scanState.BlockCommentIsJsDoc {
						restOfLine := strings.TrimSpace(line[j+2:])
						nextDefLine := i + 1
						if restOfLine == "" {
							for k := nextDefLine; k < len(lines); k++ {
								trimmedNext := strings.TrimSpace(lines[k])
								if trimmedNext == "" {
									continue
								}
								nextDefLine = k
								break
							}
						} else {
							nextDefLine = i
						}
						if nextDefLine < len(lines) {
							isDefDocstring = ctx.IsDefinitionDocLine(file, scanState.BlockCommentStartLine)
						}
					}
					if !isDefDocstring && !scanState.BlockCommentHasTodo && !ctx.IsSpecBlock(file, scanState.BlockCommentStartLine, lineNum, lines) {
						if scanState.BlockCommentIsJsDoc {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("JSDoc comment in %s:%d", file, scanState.BlockCommentStartLine),
								model.BreachCodeCommentJSDoc,
								file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
						} else {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
								model.BreachCodeCommentBlock,
								file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
						}
					}
					scanState.InBlockComment = false
					scanState.BlockCommentHasTodo = false
					j += 2
					continue
				}
				j++
				continue
			}
			if scanState.Escaped {
				scanState.Escaped = false
				j++
				continue
			}
			if line[j] == '\\' && (scanState.InSingleQuote || scanState.InDoubleQuote || scanState.InTemplateRaw()) {
				scanState.Escaped = true
				j++
				continue
			}
			if scanState.InSingleQuote {
				if line[j] == '\'' {
					scanState.InSingleQuote = false
				}
				j++
				continue
			}
			if scanState.InDoubleQuote {
				if line[j] == '"' {
					scanState.InDoubleQuote = false
				}
				j++
				continue
			}
			if scanState.InTemplateRaw() {
				if line[j] == '`' {
					scanState.Templates = scanState.Templates[:len(scanState.Templates)-1]
					j++
					continue
				}
				if j+1 < len(line) && line[j] == '$' && line[j+1] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth = 1
					j += 2
					continue
				}
				j++
				continue
			}
			if len(scanState.Templates) > 0 && scanState.Templates[len(scanState.Templates)-1].ExprDepth > 0 {
				if line[j] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth++
					j++
					continue
				}
				if line[j] == '}' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth--
					j++
					continue
				}
			}
			if line[j] == '\'' {
				scanState.InSingleQuote = true
				j++
				continue
			}
			if line[j] == '"' {
				scanState.InDoubleQuote = true
				j++
				continue
			}
			if line[j] == '`' {
				scanState.Templates = append(scanState.Templates, CommentTemplateState{ExprDepth: 0})
				j++
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '*' {
				isJsDoc := j+2 < len(line) && line[j+2] == '*'
				scanState.InBlockComment = true
				scanState.BlockCommentStartLine = lineNum
				scanState.BlockCommentStartIndex = lineStart + j
				scanState.BlockCommentStartColumn = j + 1
				scanState.BlockCommentIsJsDoc = isJsDoc
				scanState.BlockCommentHasTodo = strings.Contains(line[j:], "TODO")
				j += 2
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '/' {

				if j > 0 && line[j-1] == ':' {
					j += 2
					continue
				}
				trimmed := strings.TrimSpace(line)
				if matched, _ := l.PolicySectionStartMatch(trimmed); matched {
					break
				}
				if matched, _ := l.PolicySectionEndMatch(trimmed); matched {
					break
				}
				if strings.HasPrefix(trimmed, "// eslint-") || strings.HasPrefix(trimmed, "// @ts-") || strings.HasPrefix(trimmed, "// noinspection") || strings.HasPrefix(trimmed, "// TODO") || strings.HasPrefix(trimmed, "// compose-ignore-") {
					if strings.HasPrefix(trimmed, "// TODO") {
						scanState.InTodoBlock = true
					}
					foundInline = true
					inlineCommentActive = true
					break
				}
				if scanState.InTodoBlock && strings.HasPrefix(trimmed, "//") {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsSpecLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsSectionDocLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				if ctx.IsDefinitionDocLine(file, lineNum) {
					foundInline = true
					inlineCommentActive = true
					break
				}
				scanState.InTodoBlock = false
				debugMarker := strings.Contains(line, "[DEBUG]")
				if !debugMarker {
					foundInline = true
					if !inlineCommentActive {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
							model.BreachCodeCommentInline,
							file, lineNum, j+1, strings.TrimSpace(line[j:])))
						inlineCommentActive = true
					}
				}
				break
			}
			j++
		}
		if !foundInline {
			scanState.InTodoBlock = false
			inlineCommentActive = false
		}
		charIndex += len(line) + 1
	}
	return breachs
}

// 🧲️ExtractImports MUST return the extracted value without side effects.
// 🧲️ExtractImports extracts the imports from the given input.
func (l *TypeScriptLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") {
			currentImport := line
			if !strings.Contains(line, ";") {
				for j := i + 1; j < len(lines); j++ {
					currentImport += "\n" + lines[j]
					if strings.Contains(lines[j], ";") {
						i = j
						break
					}
				}
			}
			imports = append(imports, currentImport)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

// 🔤️FormatImports MUST produce a well-formed imports string.
// 🖊️FormatImports formats the imports into its string representation.
func (l *TypeScriptLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	return strings.Join(uniqueImports, "\n")
}

// #endregion 🎶️TypeScript

// #region 🎚️Go

// 🗣️GoLanguage holds the data fields for a go language record.
type GoLanguage struct {
	BaseLanguage
}

// 🔷️NewGoLanguage MUST initialize all required fields and return a valid GoLanguage.
// 🆕️NewGoLanguage creates and returns a new GoLanguage instance.
func NewGoLanguage() *GoLanguage {
	return &GoLanguage{
		BaseLanguage: BaseLanguage{
			name:               "go",
			extensions:         []string{".go"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			blockCommentStart:  "/*",
			blockCommentEnd:    "*/",
			sectionStartFmt:    "// #region 🔖️%s",
			sectionEndFmt:      "// #endregion 🔖️%s",
			sectionBothFmt:     "\n// #region 🔖️%s\n\n// #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
			hasRawBackticks:    true,
			skipDirectives:     []string{"nolint"},
		},
	}
}

// 📖️ExtraOrphanDefinitions MUST operate on the GoLanguage receiver and return consistent results.
func (l *GoLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	for i := 0; i < len(lines); i++ {
		trimmed := strings.TrimSpace(lines[i])
		if strings.HasPrefix(trimmed, "package ") {
			name := fmt.Sprintf("package-%d", i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: trimmed})
			break
		}
	}
	for i := 0; i < len(lines); i++ {
		trimmed := strings.TrimSpace(lines[i])
		if strings.HasPrefix(trimmed, "import ") {
			start := i + 1
			end := start
			if strings.HasPrefix(trimmed, "import (") {
				for j := i + 1; j < len(lines); j++ {
					if strings.TrimSpace(lines[j]) == ")" {
						end = j + 1
						i = j
						break
					}
				}
			}
			name := fmt.Sprintf("import-%d", start)
			defs = append(defs, DefinitionRange{Name: name, Start: start, End: end, Excerpt: strings.TrimSpace(lines[start-1])})
		}
	}
	return defs
}

// 🧲️ExtractImports MUST return the extracted value without side effects.
// 🧲️ExtractImports extracts the imports from the given input.
func (l *GoLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	inImportBlock := false
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		if inImportBlock {
			if trimmed == ")" {
				inImportBlock = false
			} else if trimmed != "" {

				imports = append(imports, strings.Trim(trimmed, ","))
			}
			continue
		}

		if strings.HasPrefix(trimmed, "import (") {
			inImportBlock = true
			continue
		}

		if strings.HasPrefix(trimmed, "import ") {

			imports = append(imports, strings.TrimPrefix(trimmed, "import "))
			continue
		}

		bodyLines = append(bodyLines, line)
	}
	return imports, strings.Join(bodyLines, "\n")
}

// 🔤️FormatImports MUST produce a well-formed imports string.
// 📥️FormatImports formats the imports into its string representation.
func (l *GoLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	importBlock := "import (\n"
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	for _, imp := range uniqueImports {
		importBlock += "\t" + imp + "\n"
	}
	importBlock += ")"
	return importBlock
}

// 💻️ExtractPackage MUST return the extracted value without side effects.
// 💾️ExtractPackage extracts the package from the given input.
func (l *GoLanguage) ExtractPackage(content string) (string, string) {
	lines := strings.Split(content, "\n")
	pkg := ""
	var bodyLines []string
	foundPkg := false
	for _, line := range lines {
		if !foundPkg && strings.HasPrefix(strings.TrimSpace(line), "package ") {
			pkg = line
			continue
		}
		bodyLines = append(bodyLines, line)
	}
	return pkg, strings.Join(bodyLines, "\n")
}

// 💿️PythonLanguage holds the data fields for a python language record.
type PythonLanguage struct {
	BaseLanguage
}

// 🔶️NewPythonLanguage MUST initialize all required fields and return a valid PythonLanguage.
// 🔷️NewPythonLanguage creates and returns a new PythonLanguage instance.
func NewPythonLanguage() *PythonLanguage {
	return &PythonLanguage{
		BaseLanguage: BaseLanguage{
			name:               "python",
			extensions:         []string{".py"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*#?region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*#?endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:def|class|async\s+def)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# #region 🔖️%s",
			sectionEndFmt:      "# #endregion 🔖️%s",
			sectionBothFmt:     "\n# #region 🔖️%s\n\n# #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  true,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*#?region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*#?endregion(?:\s+(\S.*?))?\s*$`),
			hasTripleQuotes:    true,
			skipDirectives:     []string{"noqa", "type: ignore", "pylint:", "pragma:"},
		},
	}
}

// 📥️ExtractImports MUST return the extracted value without side effects.
// 📝️ExtractImports extracts the imports from the given input.
func (l *PythonLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") || strings.HasPrefix(trimmed, "from ") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

// 📋️FormatImports MUST produce a well-formed imports string.
// 🖊️FormatImports formats the imports into its string representation.
func (l *PythonLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
}

// #endregion 🎚️Go

// #region 📜️C#

// 🗣️CSharpLanguage holds the data fields for a c sharp language record.
type CSharpLanguage struct {
	BaseLanguage
}

// 🔷️NewCSharpLanguage MUST initialize all required fields and return a valid CSharpLanguage.
// 🆕️NewCSharpLanguage creates and returns a new CSharpLanguage instance.
func NewCSharpLanguage() *CSharpLanguage {
	return &CSharpLanguage{
		BaseLanguage: BaseLanguage{
			name:               "csharp",
			extensions:         []string{".cs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*(?://\s*)?#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*(?://\s*)?#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:(?:public|private|protected|internal|static|partial|abstract|sealed|virtual|override|async)\s+)*(?:class|struct|interface|enum|delegate|record|void|string|int|bool|[A-Z][A-Za-z0-9_<>]*)\s+([A-Z][A-Za-z0-9_]*)\s*[<({]`),
			commentPrefix:      "//",
			blockCommentStart:  "/*",
			blockCommentEnd:    "*/",
			sectionStartFmt:    "// #region 🔖️%s",
			sectionEndFmt:      "// #endregion 🔖️%s",
			sectionBothFmt:     "\n// #region 🔖️%s\n\n// #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*(?://\s*)?#endregion(?:\s+(\S.*?))?\s*$`),
			hasVerbatimStrings: true,
			skipDirectives:     []string{"pragma"},
		},
	}
}

// 🧲️ExtractImports MUST return the extracted value without side effects.
// 🧲️ExtractImports extracts the imports from the given input.
func (l *CSharpLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "using ") && strings.HasSuffix(trimmed, ";") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

// 🔤️FormatImports MUST produce a well-formed imports string.
// 📥️FormatImports formats the imports into its string representation.
func (l *CSharpLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
}

// #endregion 📜️C#

// #region 📭️JSON

// 🗣️JSONLanguage holds the data fields for a j s o n language record.
type JSONLanguage struct {
	BaseLanguage
}

// 📋️NewJSONLanguage MUST initialize all required fields and return a valid JSONLanguage.
// 🆕️NewJSONLanguage creates and returns a new JSONLanguage instance.
func NewJSONLanguage() *JSONLanguage {
	return &JSONLanguage{
		BaseLanguage: BaseLanguage{
			name:              "json",
			extensions:        []string{".json"},
			commentPrefix:     "",
			usesIndentScoping: false,
		},
	}
}

// 📑️SupportsSections MUST operate on the JSONLanguage receiver and return consistent results.
func (l *JSONLanguage) SupportsSections() bool { return true }

// 📖️SupportsDefinitions MUST operate on the JSONLanguage receiver and return consistent results.
func (l *JSONLanguage) SupportsDefinitions() bool { return false }

// 📥️SupportsComments MUST operate on the JSONLanguage receiver and return consistent results.
func (l *JSONLanguage) SupportsComments() bool { return false }

// 🔷️SupportsHeaders MUST operate on the JSONLanguage receiver and return consistent results.
func (l *JSONLanguage) SupportsHeaders() bool { return false }

// ❌️ParseSections MUST return an error when the input is malformed.
// 💾️ParseSections parses the input and returns the sections result.
func (l *JSONLanguage) ParseSections(content string) []model.Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

// #endregion 📭️JSON

// #region 🛒️Markdown

// 🗣️MarkdownLanguage holds the data fields for a markdown language record.
type MarkdownLanguage struct {
	BaseLanguage
}

// 📰️NewMarkdownLanguage MUST initialize all required fields and return a valid MarkdownLanguage.
// 🆕️NewMarkdownLanguage creates and returns a new MarkdownLanguage instance.
func NewMarkdownLanguage() *MarkdownLanguage {
	return &MarkdownLanguage{
		BaseLanguage: BaseLanguage{
			name:              "markdown",
			extensions:        []string{".md", ".mdx"},
			sectionStart:      regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`),
			commentPrefix:     "",
			sectionStartFmt:   "## %s",
			sectionEndFmt:     "",
			sectionBothFmt:    "\n## %s\n\n",
			usesIndentScoping: false,
		},
	}
}

// 📑️SupportsSections MUST operate on the MarkdownLanguage receiver and return consistent results.
func (l *MarkdownLanguage) SupportsSections() bool { return true }

// 📖️SupportsDefinitions MUST operate on the MarkdownLanguage receiver and return consistent results.
func (l *MarkdownLanguage) SupportsDefinitions() bool { return false }

// 📥️SupportsComments MUST operate on the MarkdownLanguage receiver and return consistent results.
func (l *MarkdownLanguage) SupportsComments() bool { return false }

// ❌️ParseSections MUST return an error when the input is malformed.
// 💾️ParseSections parses the input and returns the sections result.
func (l *MarkdownLanguage) ParseSections(content string) []model.Section {
	return ParseMarkdownSectionsInternal(content)
}

// #endregion 🛒️Markdown

// #region ⏳️Rust

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
func (l *RustLanguage) ScanComments(ctx CommentPolicy, file, content string, lines []string) []model.Breach {
	breachs := l.BaseLanguage.ScanComments(ctx, file, content, lines)
	legacyStart := regexp.MustCompile(`(?i)^\s*//\s*#region\b`)
	legacyEnd := regexp.MustCompile(`(?i)^\s*//\s*#endregion\b`)
	for i, line := range lines {
		if legacyStart.MatchString(line) || legacyEnd.MatchString(line) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Legacy region comment in Rust file %s:%d", file, i+1),
				model.BreachCodeRustRegionComment,
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

// #endregion ⏳️Rust

// #region 🪅️Ruby

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

// #endregion 🪅️Ruby

// #region 📊️Shell

// 🗣️ShellLanguage holds the data fields for a shell language record.
type ShellLanguage struct {
	BaseLanguage
}

// 🔷️NewShellLanguage MUST initialize all required fields and return a valid ShellLanguage.
// 🆕️NewShellLanguage creates and returns a new ShellLanguage instance.
func NewShellLanguage() *ShellLanguage {
	return &ShellLanguage{
		BaseLanguage: BaseLanguage{
			name:               "shell",
			extensions:         []string{".sh"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\))?\s*\{`),
			commentPrefix:      "#",
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

// #endregion 📊️Shell

// #region 📢️TOML

// 🗣️TomlLanguage holds the data fields for a toml language record.
type TomlLanguage struct {
	BaseLanguage
}

// 🔷️NewTomlLanguage MUST initialize all required fields and return a valid TomlLanguage.
// 🆕️NewTomlLanguage creates and returns a new TomlLanguage instance.
func NewTomlLanguage() *TomlLanguage {
	return &TomlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "toml",
			extensions:         []string{".toml"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// 📑️SupportsSections MUST operate on the TomlLanguage receiver and return consistent results.
func (l *TomlLanguage) SupportsSections() bool { return true }

// 📖️SupportsDefinitions MUST operate on the TomlLanguage receiver and return consistent results.
func (l *TomlLanguage) SupportsDefinitions() bool { return false }

// 📥️SupportsComments MUST operate on the TomlLanguage receiver and return consistent results.
func (l *TomlLanguage) SupportsComments() bool { return true }

// 🔶️SupportsHeaders MUST operate on the TomlLanguage receiver and return consistent results.
func (l *TomlLanguage) SupportsHeaders() bool { return false }

// #endregion 📢️TOML

// #region 🤸️YAML

// 🗣️YamlLanguage holds the data fields for a yaml language record.
type YamlLanguage struct {
	BaseLanguage
}

// 📃️NewYamlLanguage MUST initialize all required fields and return a valid YamlLanguage.
// 🆕️NewYamlLanguage creates and returns a new YamlLanguage instance.
func NewYamlLanguage() *YamlLanguage {
	return &YamlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "yaml",
			extensions:         []string{".yaml", ".yml"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			usesIndentScoping:  true,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// 📑️SupportsSections MUST operate on the YamlLanguage receiver and return consistent results.
func (l *YamlLanguage) SupportsSections() bool { return true }

// 📖️SupportsDefinitions MUST operate on the YamlLanguage receiver and return consistent results.
func (l *YamlLanguage) SupportsDefinitions() bool { return false }

// 📥️SupportsComments MUST operate on the YamlLanguage receiver and return consistent results.
func (l *YamlLanguage) SupportsComments() bool { return true }

// 🔷️SupportsHeaders MUST operate on the YamlLanguage receiver and return consistent results.
func (l *YamlLanguage) SupportsHeaders() bool { return false }

// #endregion 🤸️YAML

// #region 🕌️SQL

// 🗣️SqlLanguage holds the data fields for a sql language record.
type SqlLanguage struct {
	BaseLanguage
}

// 🔷️NewSqlLanguage MUST initialize all required fields and return a valid SqlLanguage.
// 🆕️NewSqlLanguage creates and returns a new SqlLanguage instance.
func NewSqlLanguage() *SqlLanguage {
	return &SqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "sql",
			extensions:         []string{".sql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*--\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?i)^(?:CREATE\s+(?:OR\s+REPLACE\s+)?(?:TABLE|VIEW|PROCEDURE|FUNCTION|TRIGGER|INDEX|TYPE|SCHEMA|DATABASE|SEQUENCE|MATERIALIZED\s+VIEW))\s+(?:IF\s+NOT\s+EXISTS\s+)?([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "--",
			sectionStartFmt:    "-- #region 🔖️%s",
			sectionEndFmt:      "-- #endregion 🔖️%s",
			sectionBothFmt:     "\n-- #region 🔖️%s\n\n-- #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*--\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion 🕌️SQL

// #region 🎙️GraphQL

// 🕸️GraphqlLanguage holds the data fields for a graphql language record.
type GraphqlLanguage struct {
	BaseLanguage
}

// 🗣️NewGraphqlLanguage MUST initialize all required fields and return a valid GraphqlLanguage.
// 🆕️NewGraphqlLanguage creates and returns a new GraphqlLanguage instance.
func NewGraphqlLanguage() *GraphqlLanguage {
	return &GraphqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "graphql",
			extensions:         []string{".graphql", ".gql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:type|interface|enum|input|union|scalar|query|mutation|subscription|fragment|extend\s+type|extend\s+interface|extend\s+enum|extend\s+union|extend\s+input)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# #region 🔖️%s",
			sectionEndFmt:      "# #endregion 🔖️%s",
			sectionBothFmt:     "\n# #region 🔖️%s\n\n# #endregion 🔖️%s\n",
			supportsHeaders:    true,
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion 🎙️GraphQL

// #region 🎶️TypeScript

// 💿️languageRegistry holds the data fields for a languageRegistry record.
var languageRegistry = []LanguagePlugin{
	NewTypeScriptLanguage(),
	NewGoLanguage(),
	NewPythonLanguage(),
	NewCSharpLanguage(),
	NewMarkdownLanguage(),
	NewRustLanguage(),
	NewRubyLanguage(),
	NewShellLanguage(),
	NewTomlLanguage(),
	NewYamlLanguage(),
	NewSqlLanguage(),
	NewGraphqlLanguage(),
}

// 🏪️GetLanguage MUST return the stored value without modification.
// 📖️GetLanguage returns the language of the value.
func GetLanguage(filePath string) LanguagePlugin {
	ext := strings.ToLower(filepath.Ext(filePath))
	for _, lang := range languageRegistry {
		if lang.MatchesExtension(ext) {
			return lang
		}
	}
	return nil
}

// 🔷️GetLanguageByName MUST return the stored value without modification.
// 🔤️GetLanguageByName returns the language by name of the value.
func GetLanguageByName(name string) LanguagePlugin {
	for _, lang := range languageRegistry {
		if lang.Name() == name {
			return lang
		}
	}
	return nil
}

// #endregion 🎶️TypeScript

// #region 🎽️Languages

// ✍️GitAuthor holds the data fields for a git author record.
type GitAuthor struct {
	Name   string `json:"name,omitempty" yaml:"name,omitempty"`
	Email  string `json:"email,omitempty" yaml:"email,omitempty"`
	GitHub string `json:"github,omitempty" yaml:"github,omitempty"`
}

// 🟪️String MUST return the canonical string value.
// 🟩️String returns the string representation of the GitAuthor.
func (a GitAuthor) String() string {
	if a.Email != "" {
		return fmt.Sprintf("%s <%s>", a.Name, a.Email)
	}
	return a.Name
}

func ParseGitAuthor(s string) GitAuthor {
	res := GitAuthor{}
	if strings.Contains(s, " <") {
		parts := strings.Split(s, " <")
		res.Name = strings.TrimSpace(parts[0])
		res.Email = strings.TrimSuffix(parts[1], ">")
	} else {
		res.Name = s
	}
	return res
}

// 🏪️GetSystem MUST return the stored value without modification.
// 📦️GetSystem returns the system of the value.
func GetSystem() string {
	switch runtime.GOOS {
	case "darwin":
		return "mac"
	case "windows":
		return "windows"
	default:
		return "linux"
	}
}

// 🩷️AnalyzeReport holds the data fields for a analyze report record.
type AnalyzeReport struct {
	Second  string         `json:"second"`
	Status  string         `json:"status"`
	Scope   string         `json:"scope"`
	Summary Summary        `json:"summary"`
	Breachs []model.Breach `json:"breachs"`
}

// 💜️Summary holds the data fields for a summary record.
type Summary struct {
	Total      int            `json:"total"`
	ByPriority map[string]int `json:"byPriority"`
	ByKind     map[string]int `json:"byKind"`
}

// 💙️FileCache holds the data fields for a file cache record.
type FileCache struct {
	FilePath string         `json:"filePath"`
	Hash     string         `json:"hash"`
	Second   string         `json:"second"`
	Breachs  []model.Breach `json:"breachs"`
}

// #endregion 🎽️Languages

// #region 📝️Sections

// 📑️ParseCodeSections MUST return an error when the input is malformed.
// 📑️ParseCodeSections parses the input and returns the code sections result.
func ParseCodeSections(content string, languageName string) []model.Section {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

// 📰️ParseMarkdownSectionsInternal MUST return an error when the input is malformed.
// 📰️ParseMarkdownSectionsInternal parses the input and returns the markdown sections internal result.
func ParseMarkdownSectionsInternal(content string) []model.Section {
	lines := strings.Split(content, "\n")
	var sections []model.Section
	type stackItem struct {
		level   int
		section *model.Section
	}
	var stack []stackItem
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	frontmatterLines := 0
	if strings.HasPrefix(content, "---") {
		endIndex := strings.Index(content[3:], "---")
		if endIndex != -1 {
			frontmatterContent := content[:endIndex+6]
			frontmatterLines = strings.Count(frontmatterContent, "\n")
		}
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(match[2])
			for len(stack) > 0 && stack[len(stack)-1].level >= level {
				popped := stack[len(stack)-1]
				popped.section.EndLine = frontmatterLines + i
				popped.section.EndIndex = lineStart - 1
				stack = stack[:len(stack)-1]
			}
			section := &model.Section{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []model.Section{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.section.Children = append(parent.section.Children, *section)
				section = &parent.section.Children[len(parent.section.Children)-1]
			} else {
				sections = append(sections, *section)
				section = &sections[len(sections)-1]
			}
			stack = append(stack, stackItem{level: level, section: section})
		}
		charIndex += len(line) + 1
	}
	for len(stack) > 0 {
		popped := stack[len(stack)-1]
		popped.section.EndLine = frontmatterLines + len(lines)
		popped.section.EndIndex = len(content)
		stack = stack[:len(stack)-1]
	}
	return sections
}

// 💿️JsonSectionLocation holds the data fields for a json section location record.
type JsonSectionLocation struct {
	Path       string
	KeyStart   int
	KeyEnd     int
	ValueStart int
	ValueEnd   int
	Section    *model.Section
}

// 📋️jsonContext holds the data fields for a jsonContext record.
type jsonContext struct {
	kind      byte
	section   *model.Section
	path      string
	expectKey bool
	location  *JsonSectionLocation
}

// ❌️ParseJSONSectionsDetailed MUST return an error when the input is malformed.
// 💾️ParseJSONSectionsDetailed parses the input and returns the j s o n sections detailed result.
func ParseJSONSectionsDetailed(content string) ([]model.Section, map[string]*JsonSectionLocation, error) {
	var sections []model.Section
	locations := make(map[string]*JsonSectionLocation)
	var stack []jsonContext
	line := 1
	inString := false
	escape := false
	stringStart := 0
	stringBuf := strings.Builder{}
	pendingKey := ""
	pendingKeyStart := 0
	pendingKeyEnd := 0
	pendingKeyLine := 0
	var awaitingValue *JsonSectionLocation
	for i := 0; i < len(content); i++ {
		ch := content[i]
		if ch == '\n' {
			line++
		}
		if inString {
			if escape {
				escape = false
				stringBuf.WriteByte(ch)
			}
			if ch == '\\' {
				escape = true
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '"' {
				inString = false
				value := stringBuf.String()
				stringBuf.Reset()
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' && stack[len(stack)-1].expectKey && awaitingValue == nil {
					pendingKey = value
					pendingKeyStart = stringStart
					pendingKeyEnd = i
					pendingKeyLine = line
					stack[len(stack)-1].expectKey = false
				} else if awaitingValue != nil && awaitingValue.ValueStart == stringStart {
					awaitingValue.ValueEnd = i
					awaitingValue.Section.EndLine = line
					awaitingValue.Section.EndIndex = i + 1
					awaitingValue = nil
				}
				continue
			}
			stringBuf.WriteByte(ch)
			continue
		}
		if ch == '"' {
			if awaitingValue != nil {
				awaitingValue.ValueStart = i
			}
			inString = true
			stringStart = i
			continue
		}
		if len(stack) > 0 && stack[len(stack)-1].kind == '{' && pendingKey != "" && ch == ':' {
			parent := stack[len(stack)-1].section
			path := pendingKey
			if stack[len(stack)-1].path != "" {
				path = stack[len(stack)-1].path + "/" + pendingKey
			}
			section := model.Section{
				Name:       pendingKey,
				StartLine:  pendingKeyLine,
				EndLine:    -1,
				StartIndex: pendingKeyStart,
				EndIndex:   -1,
				Children:   []model.Section{},
			}
			var sectionRef *model.Section
			if parent != nil {
				parent.Children = append(parent.Children, section)
				sectionRef = &parent.Children[len(parent.Children)-1]
			} else {
				sections = append(sections, section)
				sectionRef = &sections[len(sections)-1]
			}
			location := &JsonSectionLocation{
				Path:     path,
				KeyStart: pendingKeyStart,
				KeyEnd:   pendingKeyEnd,
				Section:  sectionRef,
			}
			locations[path] = location
			awaitingValue = location
			pendingKey = ""
			continue
		}
		if awaitingValue != nil {
			if ch == '{' || ch == '[' {
				awaitingValue.ValueStart = i
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   awaitingValue.Section,
					path:      awaitingValue.Path,
					expectKey: ch == '{',
					location:  awaitingValue,
				})
				awaitingValue = nil
				continue
			}
			if ch == '-' || (ch >= '0' && ch <= '9') || ch == 't' || ch == 'f' || ch == 'n' {
				awaitingValue.ValueStart = i
				end := i
				for end < len(content) {
					c := content[end]
					if c == '\n' {
						line++
					}
					if c == ',' || c == '}' || c == ']' || c == ' ' || c == '\t' || c == '\r' || c == '\n' {
						break
					}
					end++
				}
				awaitingValue.ValueEnd = end - 1
				awaitingValue.Section.EndLine = line
				awaitingValue.Section.EndIndex = end
				awaitingValue = nil
				i = end - 1
				continue
			}
		}
		if ch == '{' || ch == '[' {
			if awaitingValue == nil {
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   nil,
					path:      "",
					expectKey: ch == '{',
					location:  nil,
				})
			}
			continue
		}
		if ch == '}' || ch == ']' {
			if len(stack) > 0 {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if top.location != nil {
					top.location.ValueEnd = i
					top.location.Section.EndLine = line
					top.location.Section.EndIndex = i + 1
				}
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
					stack[len(stack)-1].expectKey = true
				}
			}
			continue
		}
		if ch == ',' {
			if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
				stack[len(stack)-1].expectKey = true
			}
			continue
		}
	}
	for _, location := range locations {
		if location.Section.EndIndex == -1 {
			location.Section.EndLine = line
			location.Section.EndIndex = len(content)
		}
	}
	return sections, locations, nil
}

// 🔬️ParseJSONSections MUST return an error when the input is malformed.
// 📝️ParseJSONSections parses the input and returns the j s o n sections result.
func ParseJSONSections(content string) []model.Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

// 📝️ParseSections MUST return an error when the input is malformed.
// 📩️ParseSections parses the input and returns the sections result.
func ParseSections(content string, filePath string) []model.Section {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	return language.ParseSections(content)
}

// 📖️ParseDefinitions MUST return an error when the input is malformed.
// 📖️ParseDefinitions parses the input and returns the definitions result.
func ParseDefinitions(content string, filePath string) []model.Definition {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	if !language.SupportsDefinitions() {
		return nil
	}
	lines := strings.Split(content, "\n")
	ranges := language.ParseDefinitions(content, lines)
	definitions := make([]model.Definition, len(ranges))
	for i, r := range ranges {
		kind := model.DeriveDefinitionKind(r.Kind)
		definitions[i] = model.Definition{
			Name:      r.Name,
			Kind:      kind,
			StartLine: r.Start,
			EndLine:   r.End,
			FilePath:  filePath,
		}
	}
	return definitions
}

// 🎯️HydrateSectionsWithDefinitions MUST attach all matching child elements to their parents.
// 📦️HydrateSectionsWithDefinitions populates the sections with definitions with associated child data.
func HydrateSectionsWithDefinitions(sections []model.Section, definitions []model.Definition) []model.Section {
	if len(sections) == 0 {
		return sections
	}
	newSections := make([]model.Section, len(sections))
	for i := range sections {
		newSections[i] = sections[i]
		start := newSections[i].StartLine
		end := newSections[i].EndLine

		var subset []model.Definition
		for _, def := range definitions {
			if def.StartLine >= start && def.EndLine <= end {
				subset = append(subset, def)
			}
		}

		newSections[i].Children = HydrateSectionsWithDefinitions(newSections[i].Children, subset)

		var myDefs []model.Definition
		for _, def := range subset {
			inChild := false
			for _, child := range newSections[i].Children {
				if def.StartLine >= child.StartLine && def.EndLine <= child.EndLine {
					inChild = true
					break
				}
			}
			if !inChild {
				myDefs = append(myDefs, def)
			}
		}
		newSections[i].Definitions = myDefs
	}
	return newSections
}

// 🛤️NormalizeSectionPath MUST be idempotent for already-normalized values.
// ❓️NormalizeSectionPath normalizes the section path to its canonical form.
func NormalizeSectionPath(sectionPath string) []string {
	cleaned := strings.ReplaceAll(sectionPath, "#", "/")
	raw := strings.FieldsFunc(cleaned, func(r rune) bool { return r == '/' })
	var parts []string
	for _, part := range raw {
		if part != "" {
			parts = append(parts, part)
		}
	}
	return parts
}

// ▶️jsonLineStart holds the data fields for a jsonLineStart record.
func jsonLineStart(content string, index int) int {
	if index <= 0 {
		return 0
	}
	pos := strings.LastIndex(content[:index], "\n")
	if pos == -1 {
		return 0
	}
	return pos + 1
}

// 🔷️jsonLineIndent holds the data fields for a jsonLineIndent record.
func jsonLineIndent(content string, index int) string {
	start := jsonLineStart(content, index)
	end := start
	for end < len(content) && (content[end] == ' ' || content[end] == '\t') {
		end++
	}
	return content[start:end]
}

// 🔶️jsonIsWhitespace holds the data fields for a jsonIsWhitespace record.
func jsonIsWhitespace(ch byte) bool {
	return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

// 🔹️jsonFindMatching holds the data fields for a jsonFindMatching record.
func jsonFindMatching(content string, start int, open byte, close byte) (int, bool) {
	if start < 0 || start >= len(content) || content[start] != open {
		return -1, false
	}
	inString := false
	escape := false
	depth := 0
	for i := start; i < len(content); i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			inString = true
			continue
		}
		if ch == open {
			depth++
			continue
		}
		if ch == close {
			depth--
			if depth == 0 {
				return i, true
			}
		}
	}
	return -1, false
}

// 🌱️jsonFindRootObjectRange holds the data fields for a jsonFindRootObjectRange record.
func jsonFindRootObjectRange(content string) (int, int, bool) {
	for i := 0; i < len(content); i++ {
		if jsonIsWhitespace(content[i]) {
			continue
		}
		if content[i] != '{' {
			return -1, -1, false
		}
		end, ok := jsonFindMatching(content, i, '{', '}')
		return i, end, ok
	}
	return -1, -1, false
}

// 🔸️jsonFindObjectRange holds the data fields for a jsonFindObjectRange record.
func JsonFindObjectRange(content string, locations map[string]*JsonSectionLocation, path string) (int, int, bool) {
	if path == "" {
		return jsonFindRootObjectRange(content)
	}
	location, ok := locations[path]
	if !ok || location.ValueStart < 0 || location.ValueStart >= len(content) {
		return -1, -1, false
	}
	if content[location.ValueStart] != '{' {
		return -1, -1, false
	}
	return location.ValueStart, location.ValueEnd, true
}

// 🔺️jsonObjectHasEntries holds the data fields for a jsonObjectHasEntries record.
func jsonObjectHasEntries(content string, start, end int) bool {
	for i := start + 1; i < end; i++ {
		if !jsonIsWhitespace(content[i]) {
			return true
		}
	}
	return false
}

func jsonFindFirstKeyIndent(content string, start, end int) string {
	depth := 0
	inString := false
	escape := false
	expectKey := true
	for i := start + 1; i < end; i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			if depth == 0 && expectKey {
				return jsonLineIndent(content, i)
			}
			inString = true
			continue
		}
		if ch == '{' || ch == '[' {
			depth++
			expectKey = ch == '{'
			continue
		}
		if ch == '}' || ch == ']' {
			if depth > 0 {
				depth--
			}
			if depth == 0 {
				expectKey = true
			}
			continue
		}
		if ch == ',' && depth == 0 {
			expectKey = true
		}
	}
	return ""
}

// 📌️jsonInsertEntry holds the data fields for a jsonInsertEntry record.
func JsonInsertEntry(content string, objectStart, objectEnd int, entry string) (string, bool) {
	if objectStart < 0 || objectEnd <= objectStart {
		return content, false
	}
	hasEntries := jsonObjectHasEntries(content, objectStart, objectEnd)
	parentIndent := jsonLineIndent(content, objectEnd)
	childIndent := jsonFindFirstKeyIndent(content, objectStart, objectEnd)
	if childIndent == "" {
		childIndent = parentIndent + "  "
	}
	insert := ""
	if hasEntries {
		insert = ",\n" + childIndent + entry + "\n" + parentIndent
	} else {
		insert = "\n" + childIndent + entry + "\n" + parentIndent
	}
	return content[:objectEnd] + insert + content[objectEnd:], true
}

// 🔻️jsonReplaceKey holds the data fields for a jsonReplaceKey record.
func jsonReplaceKey(content string, keyStart, keyEnd int, newName string) string {
	quoted := strconv.Quote(newName)
	return content[:keyStart] + quoted + content[keyEnd+1:]
}

// 🧲️jsonExtractEntry holds the data fields for a jsonExtractEntry record.
func JsonExtractEntry(content string, keyStart int, valueEnd int) (string, int, int) {
	start := jsonLineStart(content, keyStart)
	end := valueEnd + 1
	for end < len(content) && jsonIsWhitespace(content[end]) {
		end++
	}
	if end < len(content) && content[end] == ',' {
		end++
	} else {
		left := start
		for left > 0 && jsonIsWhitespace(content[left-1]) {
			left--
		}
		if left > 0 && content[left-1] == ',' {
			start = left - 1
		}
	}
	entry := strings.TrimSpace(content[start:end])
	entry = strings.TrimSuffix(entry, ",")
	return entry, start, end
}

// 📍️jsonRenameEntryKey holds the data fields for a jsonRenameEntryKey record.
func JsonRenameEntryKey(entry string, newName string) string {
	inString := false
	escape := false
	stringStart := -1
	for i := 0; i < len(entry); i++ {
		ch := entry[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' && stringStart >= 0 {
				quoted := strconv.Quote(newName)
				return entry[:stringStart] + quoted + entry[i+1:]
			}
			continue
		}
		if ch == '"' {
			inString = true
			stringStart = i
		}
	}
	return entry
}

// ⬛️jsonReindentEntry holds the data fields for a jsonReindentEntry record.
func JsonReindentEntry(entry string, indent string) string {
	lines := strings.Split(entry, "\n")
	minIndent := -1
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		leading := 0
		for leading < len(line) && (line[leading] == ' ' || line[leading] == '\t') {
			leading++
		}
		if minIndent == -1 || leading < minIndent {
			minIndent = leading
		}
	}
	if minIndent < 0 {
		return indent + strings.TrimSpace(entry)
	}
	for i, line := range lines {
		if strings.TrimSpace(line) == "" {
			lines[i] = ""
			continue
		}
		if minIndent > 0 && len(line) >= minIndent {
			line = line[minIndent:]
		}
		lines[i] = indent + line
	}
	return strings.Join(lines, "\n")
}

// ⬜️FindSection MUST return nil when no match is found.
// 🔎️FindSection searches for and returns the matching section.
func FindSection(sections []model.Section, name string) *model.Section {
	for i := range sections {
		if sections[i].Name == name {
			return &sections[i]
		}
		if found := FindSection(sections[i].Children, name); found != nil {
			return found
		}
	}
	return nil
}

// #endregion 📝️Sections

// #region 🎙️GraphQL

// #region 🎙️GraphQL
// GraphQL query and mutation string constants.
// 🔖️jsonToYaml holds the data fields for a jsonToYaml record.
func JsonToYaml(jsonStr string) (string, error) {
	var data interface{}
	if err := json.Unmarshal([]byte(jsonStr), &data); err != nil {
		return "", err
	}
	yamlBytes, err := yaml.Marshal(data)
	if err != nil {
		return "", err
	}
	return string(yamlBytes), nil
}

// #endregion 🎙️GraphQL

// #region 🔌️Language Ports

// 🧊️CommentPolicy is the narrow view of the statute engine a language plugin needs while scanning
// comments. It is declared here so 🗣️languages never has to import 📜️statutes; `*PolicyContext`
// satisfies it.
type CommentPolicy interface {
	// 🚨️CreateBreach records one breach at a position.
	CreateBreach(summary string, kind model.Statute, scope string, line int, col int, excerpt string) model.Breach
	// 📏️IsSpecLine reports whether a line is part of a specification block.
	IsSpecLine(filePath string, lineNum int) bool
	// 📐️IsSpecBlock reports whether a line range is one specification block.
	IsSpecBlock(filePath string, startLine, endLine int, lines []string) bool
	// 📝️IsSectionDocLine reports whether a line documents a section.
	IsSectionDocLine(filePath string, lineNum int) bool
	// 📔️IsDefinitionDocLine reports whether a line documents a definition.
	IsDefinitionDocLine(filePath string, lineNum int) bool
}

// #endregion 🔌️Language Ports

// #endregion 🚚️Split
