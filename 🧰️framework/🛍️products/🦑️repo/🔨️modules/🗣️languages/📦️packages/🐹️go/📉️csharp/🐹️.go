// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📜️csharp carries the C# language: imports.

// #endregion 🧲️Header

package languages

import (
	regexp "regexp"
	sort "sort"
	strings "strings"
)

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
