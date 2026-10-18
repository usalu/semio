// Package languages owns the repo language table and the parsers built on it.
//
// The table itself is the single source of truth shared with the Rust twin
// (semio-framework-repo-languages), which embeds the same file with include_str!. Nothing in this
// package may re-declare a comment prefix, a region marker, a definition keyword or a header format:
// read it from the table.
package languages

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	runtime "runtime"

	model "github.com/usalu/semio/repo/model"
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
