// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Go, Python, and C# language programs.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"regexp"
	"sort"
	"strings"
)

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
