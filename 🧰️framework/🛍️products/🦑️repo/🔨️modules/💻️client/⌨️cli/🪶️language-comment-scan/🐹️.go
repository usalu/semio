// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Base language comment scanning and definition helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"regexp"
	"strings"
)

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
func (l *BaseLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Breach {
	if l.commentPrefix == "" {
		return nil
	}
	if DeriveFileKind(file) == FileKindConfig {
		return nil
	}
	var breachs []Breach
	sections := l.ParseSections(content)
	var headerSection *Section
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
						BreachCodeCommentBlock,
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
								BreachCodeCommentJSDoc,
								file, scanState.BlockCommentStartLine, scanState.BlockCommentStartColumn, ""))
						} else {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
								BreachCodeCommentBlock,
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
					fmt.Printf("[TRACE] Ignoring shebang at line 1: %s\n", trimmed)
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
				debugMarker := strings.Contains(line, "[TRACE]")
				if !debugMarker {
					foundInline = true
					if !inlineCommentActive {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
							BreachCodeCommentInline,
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
