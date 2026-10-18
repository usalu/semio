package statutes

import (
	rand "math/rand"
	regexp "regexp"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
)

// #region 📖️Spec Lines

// 🧩specKeywordPattern holds the data fields for a specKeywordPattern record.
var specKeywordPattern = regexp.MustCompile(`\b(MUST(\s+NOT)?|SHOULD(\s+NOT)?|SHALL(\s+NOT)?|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b`)

// 🔸isSpecText holds the data fields for a isSpecText record.
func IsSpecText(text string) bool {
	return specKeywordPattern.MatchString(text)
}

// 📐specImplSyntaxBacktick holds the data fields for a specImplSyntaxBacktick record.
var specImplSyntaxBacktick = regexp.MustCompile("`[^`]+`")

// ⚡specImplSyntaxFuncCall holds the data fields for a specImplSyntaxFuncCall record.
var specImplSyntaxFuncCall = regexp.MustCompile(`[A-Za-z_]\w*\.\w+\(|[A-Z]\w+\(`)

func hasImplementationSyntax(text string) (bool, string) {
	if specImplSyntaxBacktick.MatchString(text) {
		return true, "backtick-wrapped code"
	}
	if specImplSyntaxFuncCall.MatchString(text) {
		match := specImplSyntaxFuncCall.FindString(text)
		if match != "MUST(" && match != "SHOULD(" && match != "SHALL(" && match != "MAY(" {
			return true, "function/method call: " + match
		}
	}
	return false, ""
}

// 🔺️SpecLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) SpecLines(filePath string) map[int]bool {
	if ctx.specLineCache == nil {
		ctx.specLineCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.specLineCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.specLineCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.specLineCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionSpecLines func(s model.Section)
	markSectionSpecLines = func(s model.Section) {
		if strings.ToLower(s.Name) == "header" {
			for _, child := range s.Children {
				markSectionSpecLines(child)
			}
			return
		}
		inLeadBlock := true
		for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
			line := strings.TrimSpace(lines[i-1])
			if line == "" {
				continue
			}
			isComment := strings.HasPrefix(line, prefix)
			if isComment && inLeadBlock {
				commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				if IsSpecText(commentText) {
					result[i] = true
				}
			} else if !isComment {
				inLeadBlock = false
			}
			if matched, _ := language.PolicySectionStartMatch(lines[i-1]); matched {
				break
			}
		}
		for _, child := range s.Children {
			markSectionSpecLines(child)
		}
	}
	for _, s := range sections {
		markSectionSpecLines(s)
	}
	ctx.specLineCache[filePath] = result
	return result
}

// 🔻️IsSpecLine MUST return true only when the condition is met.
// 🐙IsSpecLine reports whether the PolicyContext is spec line.
func (ctx *PolicyContext) IsSpecLine(filePath string, lineNum int) bool {
	return ctx.SpecLines(filePath)[lineNum]
}

// ⬛IsSpecBlock MUST return true only when the condition is met.
// 🔒IsSpecBlock reports whether the PolicyContext is spec block.
func (ctx *PolicyContext) IsSpecBlock(filePath string, startLine, endLine int, lines []string) bool {
	for i := startLine; i <= endLine && i <= len(lines); i++ {
		if IsSpecText(lines[i-1]) {
			return true
		}
	}
	return false
}

// ⬜SectionDocLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) SectionDocLines(filePath string) map[int]bool {
	if ctx.sectionDocCache == nil {
		ctx.sectionDocCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.sectionDocCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.sectionDocCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.sectionDocCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionDocLines func(s model.Section)
	markSectionDocLines = func(s model.Section) {
		if strings.ToLower(s.Name) == "header" {
			for _, child := range s.Children {
				markSectionDocLines(child)
			}
			return
		}
		var blockLines []int
		foundFirstComment := false
		for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
			line := strings.TrimSpace(lines[i-1])
			if line == "" {
				if !foundFirstComment {
					continue
				}
				break
			}
			if !strings.HasPrefix(line, prefix) {
				break
			}
			foundFirstComment = true
			blockLines = append(blockLines, i)
			commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
			if IsSpecText(commentText) {
			}
		}
		for _, ln := range blockLines {
			result[ln] = true
		}
		for _, child := range s.Children {
			markSectionDocLines(child)
		}
	}
	for _, s := range sections {
		markSectionDocLines(s)
	}
	if language.SupportsDefinitions() {
		parsedDefs := language.ParseDefinitions(content, lines)
		for _, def := range parsedDefs {
			for lineIndex := def.Start - 2; lineIndex >= 0; lineIndex-- {
				line := strings.TrimSpace(lines[lineIndex])
				if line == "" {
					break
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				result[lineIndex+1] = true
			}
		}
	}
	ctx.sectionDocCache[filePath] = result
	return result
}

// 🟥IsSectionDocLine MUST return true only when the condition is met.
// 🔹️IsSectionDocLine reports whether the PolicyContext is section doc line.
func (ctx *PolicyContext) IsSectionDocLine(filePath string, lineNum int) bool {
	return ctx.SectionDocLines(filePath)[lineNum]
}

// 📖DefinitionDocLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) DefinitionDocLines(filePath string) map[int]bool {
	if ctx.definitionDocCache == nil {
		ctx.definitionDocCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.definitionDocCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	if !language.SupportsDefinitions() {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	prefix := language.CommentPrefix()
	defs := language.ParseDefinitions(content, lines)
	extras := language.ExtraOrphanDefinitions(lines)
	allDefs := append(defs, extras...)
	for _, d := range allDefs {
		for lineIndex := d.Start - 2; lineIndex >= 0; lineIndex-- {
			line := strings.TrimSpace(lines[lineIndex])
			if line == "" {
				break
			}
			if strings.HasPrefix(line, prefix) {
				result[lineIndex+1] = true
				continue
			}
			if language.Name() == "typescript" {
				if strings.HasPrefix(line, "/*") && strings.HasSuffix(line, "*/") && !strings.HasPrefix(line, "/**") {
					break
				}
				if strings.HasSuffix(line, "**/") || strings.HasSuffix(line, "*/") || strings.HasPrefix(line, "* ") || line == "*" || strings.HasPrefix(line, "/**") {
					result[lineIndex+1] = true
					if strings.HasPrefix(line, "/**") {
						break
					}
					continue
				}
			}
			if (language.Name() == "csharp" || language.Name() == "rust") && strings.HasPrefix(line, "///") {
				result[lineIndex+1] = true
				continue
			}
			break
		}
		if language.Name() == "python" {
			parenDepth := 0
			for _, ch := range lines[d.Start-1] {
				if ch == '(' {
					parenDepth++
				}
				if ch == ')' {
					parenDepth--
				}
			}
			bodyStart := d.Start
			if parenDepth > 0 {
				for scanIdx := d.Start; scanIdx < len(lines) && scanIdx < d.Start+15; scanIdx++ {
					for _, ch := range lines[scanIdx] {
						if ch == '(' {
							parenDepth++
						}
						if ch == ')' {
							parenDepth--
						}
					}
					if parenDepth <= 0 {
						bodyStart = scanIdx + 1
						break
					}
				}
			}
			for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
				trimmed := strings.TrimSpace(lines[bodyIdx])
				if trimmed == "" {
					continue
				}
				if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
					quote := `"""`
					if strings.HasPrefix(trimmed, `'''`) {
						quote = `'''`
					}
					result[bodyIdx+1] = true
					afterOpen := strings.TrimPrefix(trimmed, quote)
					if strings.Index(afterOpen, quote) < 0 {
						for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
							sline := strings.TrimSpace(lines[scanIdx])
							result[scanIdx+1] = true
							if sline == quote || strings.HasSuffix(sline, quote) {
								break
							}
						}
					}
				}
				break
			}
		}
	}
	ctx.definitionDocCache[filePath] = result
	return result
}

// 🟧IsDefinitionDocLine MUST return true only when the condition is met.
// ▶️IsDefinitionDocLine reports whether the PolicyContext is definition doc line.
func (ctx *PolicyContext) IsDefinitionDocLine(filePath string, lineNum int) bool {
	return ctx.DefinitionDocLines(filePath)[lineNum]
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

// #endregion 📖️Spec Lines
