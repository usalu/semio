// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// JSON, Markdown, Shell, TOML, YAML, SQL, GraphQL language programs and registry.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"regexp"
	"strings"
)

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
func (l *JSONLanguage) ParseSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

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
func (l *MarkdownLanguage) ParseSections(content string) []Section {
	return ParseMarkdownSectionsInternal(content)
}

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
