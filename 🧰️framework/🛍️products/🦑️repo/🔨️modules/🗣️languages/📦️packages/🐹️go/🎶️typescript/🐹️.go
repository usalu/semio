// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎶️typescript carries the TypeScript language: comment scanning and import handling.

// #endregion 🧲️Header

package languages

import (
	fmt "fmt"
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

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
				debugMarker := strings.Contains(line, "[TRACE]")
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
