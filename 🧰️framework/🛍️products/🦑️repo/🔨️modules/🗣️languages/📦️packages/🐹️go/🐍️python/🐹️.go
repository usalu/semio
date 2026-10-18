// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🐍️python carries the Python language: imports.

// #endregion 🧲️Header

package languages

import (
	regexp "regexp"
	sort "sort"
	strings "strings"
)

// #region 🎚️Go

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
