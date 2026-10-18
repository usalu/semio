// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Policy context and documentation line scanning.

// #endregion 🧲️Header

package client

import (
	"math/rand"
	"path/filepath"
	"regexp"
	"strings"
)

// 📝PolicyContext holds the data fields for a policy context record.
type PolicyContext struct {
	Scope              Scope
	RootDir            string
	Bundles            []Bundle
	fileCache          map[string]string
	sectionCache       map[string][]Section
	ignoreCache        map[string]map[int][]string
	specLineCache      map[string]map[int]bool
	sectionDocCache    map[string]map[int]bool
	definitionDocCache map[string]map[int]bool
	filesOverride      []string
}

// 🔷NewPolicyContext MUST initialize all required fields and return a valid PolicyContext.
// 🆕NewPolicyContext creates and returns a new PolicyContext instance.
func NewPolicyContext(scope Scope, bundles []Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Bundles:      bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]Section),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

// 📄NewPolicyContextWithFiles MUST initialize all required fields and return a valid PolicyContextWithFiles.
// 📄NewPolicyContextWithFiles creates and returns a new PolicyContextWithFiles instance.
func NewPolicyContextWithFiles(scope Scope, bundles []Bundle, files []string) *PolicyContext {
	ctx := NewPolicyContext(scope, bundles)
	ctx.filesOverride = files
	return ctx
}

// 📥Files MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) Files() ([]string, error) {
	if ctx.filesOverride != nil {
		return ctx.filesOverride, nil
	}
	files, err := ScopeToFiles(ctx.Scope, ctx.Bundles)
	if err != nil {
		return nil, err
	}
	ctx.filesOverride = files
	return files, nil
}

// 🛤️ReadText MUST return the full content from the given path.
// 🔤ReadText reads and returns the text content.
func (ctx *PolicyContext) ReadText(filePath string) string {
	if ctx.fileCache == nil {
		ctx.fileCache = make(map[string]string)
	}
	absPath := filepath.Join(rootDir, filePath)
	if content, ok := ctx.fileCache[absPath]; ok {
		return content
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		ctx.fileCache[absPath] = ""
		return ""
	}
	ctx.fileCache[absPath] = content
	return content
}

// 📑Sections MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) Sections(filePath string) []Section {
	if ctx.sectionCache == nil {
		ctx.sectionCache = make(map[string][]Section)
	}
	if sections, ok := ctx.sectionCache[filePath]; ok {
		return sections
	}
	content := ctx.ReadText(filePath)
	sections := ParseSections(content, filePath)
	ctx.sectionCache[filePath] = sections
	return sections
}

// ❌ParseIgnoreDirectives MUST return an error when the input is malformed.
// 💾ParseIgnoreDirectives parses the input and returns the ignore directives result.
func ParseIgnoreDirectives(content string) map[int][]string {
	result := make(map[int][]string)
	lines := strings.Split(content, "\n")
	ignorePrefix := "// compose-ignore-"
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, ignorePrefix) {
			pattern := strings.TrimPrefix(trimmed, ignorePrefix)

			patterns := strings.Split(pattern, ",")
			for _, p := range patterns {
				p = strings.TrimSpace(p)
				if p != "" {
					result[i+1] = append(result[i+1], p)
				}
			}
		}
	}
	return result
}

// 🔶IgnoreDirectives MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) IgnoreDirectives(filePath string) map[int][]string {
	if ignores, ok := ctx.ignoreCache[filePath]; ok {
		return ignores
	}
	content := ctx.ReadText(filePath)
	ignores := ParseIgnoreDirectives(content)
	ctx.ignoreCache[filePath] = ignores
	return ignores
}

// 🔹️IsIgnored MUST return true only when the condition is met.
// ❓IsIgnored reports whether the PolicyContext is ignored.
func (ctx *PolicyContext) IsIgnored(filePath string, breachLine int, kind Statute) bool {
	ignores := ctx.IgnoreDirectives(filePath)
	kindStr := string(kind)

	for ignoreLine, patterns := range ignores {

		if breachLine > ignoreLine && breachLine <= ignoreLine+100 {
			for _, pattern := range patterns {

				if strings.HasPrefix(kindStr, pattern) {
					return true
				}
			}
		}
	}
	return false
}

// 🆕CreateBreach MUST persist the new entity and return a reference to it.
// ⚠️CreateBreach creates a new breach and persists it.
func (ctx *PolicyContext) CreateBreach(summary string, kind Statute, scope string, line int, col int, excerpt string) Breach {
	return Breach{
		ID:      buildBreachID(scope, line, col),
		Summary: summary,
		Kind:    kind,
		Scope:   scope,
		Line:    line,
		Column:  col,
		Excerpt: excerpt,
	}
}

// 🧲️extractFileFromScope holds the data fields for a extractFileFromScope record.
func extractFileFromScope(scope string) string {

	if idx := strings.Index(scope, "#"); idx != -1 {
		scope = scope[:idx]
	}

	if idx := strings.Index(scope, "::"); idx != -1 {
		scope = scope[:idx]
	}
	return scope
}

// 🧹️FilterIgnored MUST preserve the tree structure while removing non-matching nodes.
// 🔷FilterIgnored filters the ignored based on the given criteria.
func (ctx *PolicyContext) FilterIgnored(breachs []Breach) []Breach {
	var result []Breach
	for _, v := range breachs {
		file := extractFileFromScope(v.Scope)
		if !ctx.IsIgnored(file, v.Line, v.Kind) {
			result = append(result, v)
		}
	}
	return result
}

// 🧩specKeywordPattern holds the data fields for a specKeywordPattern record.
var specKeywordPattern = regexp.MustCompile(`\b(MUST(\s+NOT)?|SHOULD(\s+NOT)?|SHALL(\s+NOT)?|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b`)

// 🔸isSpecText holds the data fields for a isSpecText record.
func isSpecText(text string) bool {
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
	language := GetLanguage(filePath)
	if language == nil {
		ctx.specLineCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionSpecLines func(s Section)
	markSectionSpecLines = func(s Section) {
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
				if isSpecText(commentText) {
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
		if isSpecText(lines[i-1]) {
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
	language := GetLanguage(filePath)
	if language == nil {
		ctx.sectionDocCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionDocLines func(s Section)
	markSectionDocLines = func(s Section) {
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
			if isSpecText(commentText) {
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
	language := GetLanguage(filePath)
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
