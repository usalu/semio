package statutes

import (
	fmt "fmt"
	filepath "path/filepath"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
)

// #region 🧩️Section Policy

// 🧪️isTestOrBenchmarkFile reports whether a path conventionally contains tests or benchmarks.
func IsTestOrBenchmarkFile(file string) bool {
	if model.DeriveFileKind(filepath.Base(file)) == model.FileKindLab {
		return true
	}
	normalized := strings.ToLower(filepath.ToSlash(file))
	if strings.Contains(normalized, "/tests/") || strings.Contains(normalized, ".tests/") ||
		strings.Contains(normalized, "/test/") || strings.Contains(normalized, ".test/") ||
		strings.Contains(normalized, "/benchmark/") || strings.Contains(normalized, ".benchmark/") {
		return true
	}
	name := filepath.Base(normalized)
	return strings.HasSuffix(name, "_test.go") ||
		strings.HasSuffix(name, ".test.ts") ||
		strings.HasSuffix(name, ".test.tsx") ||
		strings.HasSuffix(name, ".test.js") ||
		strings.HasSuffix(name, ".test.jsx") ||
		strings.HasSuffix(name, ".spec.ts") ||
		strings.HasSuffix(name, ".spec.tsx") ||
		strings.HasSuffix(name, ".spec.js") ||
		strings.HasSuffix(name, ".spec.jsx") ||
		strings.HasPrefix(name, "test_") ||
		strings.Contains(name, "benchmark")
}

// 📤isExportedDefinition holds the data fields for a isExportedDefinition record.
func isExportedDefinition(name string, line string, langName string) bool {
	return true
}

// 🧿definitionModifierWords are the words a TypeScript definition head may carry before `function` or `class`.
var definitionModifierWords = []string{"async ", "abstract ", "declare ", "default "}

// ✂️stripDefinitionModifierWords strips leading modifier words as whole space-separated prefixes, never as a character cutset.
func stripDefinitionModifierWords(line string) string {
	for stripped := true; stripped; {
		stripped = false
		for _, word := range definitionModifierWords {
			if strings.HasPrefix(line, word) {
				line = line[len(word):]
				stripped = true
			}
		}
	}
	return line
}

// 🟦requiresDefinitionRequirements holds the data fields for a requiresDefinitionRequirements record.
func requiresDefinitionRequirements(line string, langName string) bool {
	trimmed := strings.TrimSpace(line)
	switch langName {
	case "typescript":
		noExport := strings.TrimPrefix(trimmed, "export ")
		noExport = stripDefinitionModifierWords(noExport)
		return strings.HasPrefix(noExport, "function ") || strings.HasPrefix(noExport, "class ")
	case "go":
		return strings.HasPrefix(trimmed, "func ")
	case "python":
		return strings.HasPrefix(trimmed, "def ") || strings.HasPrefix(trimmed, "class ") || strings.HasPrefix(trimmed, "async def ")
	case "csharp":
		return !strings.Contains(trimmed, " interface ") && !strings.Contains(trimmed, " enum ")
	case "rust":
		noPrefix := strings.TrimPrefix(trimmed, "pub ")
		if strings.HasPrefix(noPrefix, "(") {
			idx := strings.Index(noPrefix, ") ")
			if idx >= 0 {
				noPrefix = strings.TrimSpace(noPrefix[idx+2:])
			}
		}
		return strings.HasPrefix(noPrefix, "fn ") || strings.HasPrefix(noPrefix, "struct ") || strings.HasPrefix(noPrefix, "impl ") || strings.HasPrefix(noPrefix, "trait ")
	}
	return true
}

// 🟪sectionPolicy holds the data fields for a sectionPolicy record.
func sectionPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsSections() {
			continue
		}
		lines := strings.Split(content, "\n")
		type stackItem struct {
			name string
			line int
		}
		var stack []stackItem
		for i, line := range lines {
			lineNum := i + 1
			line = strings.TrimSuffix(line, "\r")
			if matched, name := language.PolicySectionStartMatch(line); matched {
				if name == "" {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						model.BreachCodeSectionMissingStartName,
						file, lineNum, 0, strings.TrimSpace(line)))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if matched, endName := language.PolicySectionEndMatch(line); matched {
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								model.BreachCodeSectionMissingEndName,
								file, lineNum, 0, strings.TrimSpace(line)))
						} else if endName != open.name {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								model.BreachCodeSectionNameMismatch,
								file, lineNum, 0, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName)))
						}
					}
				}
			}
		}
		sections := ctx.Sections(file)
		commentPrefix := language.CommentPrefix()
		var checkSection func(s model.Section, parentPath string)
		checkSection = func(s model.Section, parentPath string) {
			sectionPath := s.Name
			if parentPath != "" {
				sectionPath = parentPath + "#" + s.Name
			}
			sectionContent := content[s.StartIndex:s.EndIndex]
			sectionLines := strings.Split(sectionContent, "\n")
			nonEmpty := 0
			for _, line := range sectionLines[1 : len(sectionLines)-1] {
				trimmed := strings.TrimSpace(line)
				if trimmed != "" && !strings.HasPrefix(trimmed, "//") && !strings.HasPrefix(trimmed, "#") {
					nonEmpty++
				}
			}
			isExempt := s.Name == "Header"
			if nonEmpty == 0 && len(s.Children) == 0 && !isExempt {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Empty section \"%s\" in %s", s.Name, file),
					model.BreachCodeSectionEmpty,
					fmt.Sprintf("%s#%s", file, sectionPath), s.StartLine, 0, ""))
			}
			if s.StartLine < len(lines) && strings.TrimSpace(lines[s.StartLine]) == "" {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Blank line after region start marker in section \"%s\" in %s:%d", s.Name, file, s.StartLine+1),
					model.BreachCodeSectionWrongFormatNewlineAfterRegion,
					fmt.Sprintf("%s#%s", file, sectionPath), s.StartLine+1, 0, ""))
			}
			if !isExempt && s.Name != "" && !IsTestOrBenchmarkFile(file) {
				hasSummary := false
				for i := 1; i < len(sectionLines)-1; i++ {
					line := strings.TrimSpace(sectionLines[i])
					if line == "" {
						continue
					}
					if matched, _ := language.PolicySectionStartMatch(line); matched {
						break
					}
					if matched, _ := language.PolicySectionEndMatch(line); matched {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText == "" {
						continue
					}
					hasSummary = true
				}
				if !hasSummary {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Section \"%s\" is missing a summary comment in %s", s.Name, file),
						model.BreachCodeSectionMissingSummary,
						fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, 0, ""))
				}
			}
			for _, child := range s.Children {
				checkSection(child, sectionPath)
			}
		}
		for _, s := range sections {
			checkSection(s, "")
		}
		covered := make([]bool, len(lines))
		var markCovered func(s model.Section)
		markCovered = func(s model.Section) {
			start := s.StartLine
			if start < 1 {
				start = 1
			}
			end := s.EndLine
			if end < start {
				end = start
			}
			if end > len(lines) {
				end = len(lines)
			}
			for lineIndex := start; lineIndex <= end; lineIndex++ {
				covered[lineIndex-1] = true
			}
			for _, child := range s.Children {
				markCovered(child)
			}
		}
		for _, s := range sections {
			markCovered(s)
		}
		type lineRange struct {
			start int
			end   int
		}
		type defRange struct {
			name  string
			kind  string
			start int
			end   int
		}
		type orphanRangeInfo struct {
			start          int
			end            int
			firstLine      string
			isCommentBlock bool
		}
		orphanLines := make([]bool, len(lines))
		for i, line := range lines {
			if covered[i] {
				continue
			}
			line = strings.TrimSuffix(line, "\r")
			if strings.TrimSpace(line) == "" {
				continue
			}
			if i == 0 && strings.HasPrefix(strings.TrimSpace(line), "#!") {
				continue
			}
			if startMatched, _ := language.PolicySectionStartMatch(line); startMatched {
				continue
			}
			if endMatched, _ := language.PolicySectionEndMatch(line); endMatched {
				continue
			}
			orphanLines[i] = true
		}
		var orphanRanges []lineRange
		inOrphan := false
		startLine := 0
		for i := 0; i < len(orphanLines); i++ {
			if orphanLines[i] {
				if !inOrphan {
					inOrphan = true
					startLine = i + 1
				}
			} else if inOrphan {
				orphanRanges = append(orphanRanges, lineRange{start: startLine, end: i})
				inOrphan = false
			}
		}
		if inOrphan {
			orphanRanges = append(orphanRanges, lineRange{start: startLine, end: len(lines)})
		}
		var defRanges []defRange
		var realDefRanges []defRange
		defExcerpts := make(map[string]string)
		if language.SupportsDefinitions() {
			parsedDefs := language.ParseDefinitions(content, lines)
			for _, def := range parsedDefs {
				defRanges = append(defRanges, defRange{name: def.Name, kind: def.Kind, start: def.Start, end: def.End})
				realDefRanges = append(realDefRanges, defRange{name: def.Name, kind: def.Kind, start: def.Start, end: def.End})
				defExcerpts[def.Name] = def.Excerpt
			}
		}
		extraDefs := language.ExtraOrphanDefinitions(lines)
		for _, def := range extraDefs {
			defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
			defExcerpts[def.Name] = def.Excerpt
		}
		var orphanInfos []orphanRangeInfo
		for _, orphanRange := range orphanRanges {
			firstLine := ""
			isCommentBlock := true
			for lineIndex := orphanRange.start; lineIndex <= orphanRange.end; lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex-1], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				if firstLine == "" {
					firstLine = strings.TrimSpace(line)
				}
				if !strings.HasPrefix(strings.TrimSpace(line), commentPrefix) {
					isCommentBlock = false
				}
			}
			orphanInfos = append(orphanInfos, orphanRangeInfo{
				start:          orphanRange.start,
				end:            orphanRange.end,
				firstLine:      firstLine,
				isCommentBlock: isCommentBlock,
			})
			if isCommentBlock {
				name := fmt.Sprintf("comment-block-%d", orphanRange.start)
				defRanges = append(defRanges, defRange{name: name, start: orphanRange.start, end: orphanRange.end})
				defExcerpts[name] = firstLine
			}
		}
		reportedDefs := make(map[string]bool)
		skipOrphans := IsTestOrBenchmarkFile(file)
		for _, orphanRange := range orphanInfos {
			if skipOrphans {
				continue
			}
			matched := false
			for _, defRange := range defRanges {
				if orphanRange.start <= defRange.end && orphanRange.end >= defRange.start {
					if !reportedDefs[defRange.name] {
						reportedDefs[defRange.name] = true
						excerpt := defRange.name
						if value, ok := defExcerpts[defRange.name]; ok && value != "" {
							excerpt = value
						}
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Orphan definition outside sections at %s:%d", file, defRange.start),
							model.BreachCodeSectionOrphanDefinition,
							fmt.Sprintf("%s::%s", file, defRange.name),
							defRange.start, 0,
							excerpt))
					}
					matched = true
				}
			}
			if matched {
				continue
			}
			name := fmt.Sprintf("orphan-block-%d", orphanRange.start)
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Orphan definition outside sections at %s:%d", file, orphanRange.start),
				model.BreachCodeSectionOrphanDefinition,
				fmt.Sprintf("%s::%s", file, name),
				orphanRange.start, 0,
				orphanRange.firstLine))
		}
		var findSectionPathForLine func(secs []model.Section, lineNum int) string
		findSectionPathForLine = func(secs []model.Section, lineNum int) string {
			for _, s := range secs {
				if lineNum < s.StartLine || lineNum > s.EndLine {
					continue
				}
				childPath := findSectionPathForLine(s.Children, lineNum)
				if childPath != "" {
					return s.Name + "#" + childPath
				}
				return s.Name
			}
			return ""
		}
		for _, def := range realDefRanges {
			isTestFile := IsTestOrBenchmarkFile(file)
			if isTestFile {
				break
			}
			defLine := ""
			if def.start-1 >= 0 && def.start-1 < len(lines) {
				defLine = lines[def.start-1]
			}
			if !isExportedDefinition(def.name, defLine, language.Name()) {
				continue
			}
			hasSummary := false
			hasRequirements := false
			isNativeDocstring := false
			langName := language.Name()
			if langName == "typescript" {
				prevIdx := def.start - 2
				if prevIdx >= 0 {
					prevLine := strings.TrimSpace(lines[prevIdx])
					if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
						isNativeDocstring = true
						for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
							sline := strings.TrimSpace(lines[scanIdx])
							isOpenLine := strings.HasPrefix(sline, "/**")
							content := sline
							if isOpenLine {
								content = strings.TrimPrefix(content, "/**")
							} else if strings.HasPrefix(content, "* ") {
								content = content[2:]
							} else if content == "*" || content == "*/" || content == "**/" {
								if isOpenLine {
									break
								}
								continue
							}
							content = strings.TrimSpace(content)
							if strings.HasSuffix(content, "**/") || strings.HasSuffix(content, "*/") {
								content = strings.TrimSpace(strings.TrimSuffix(strings.TrimSuffix(content, "**/"), "*/"))
							}
							if content == "" {
								if isOpenLine {
									break
								}
								continue
							}
							if strings.HasPrefix(content, "* ") {
								content = strings.TrimSpace(content[2:])
							}
							if IsSpecText(content) {
								hasRequirements = true
							} else {
								hasSummary = true
							}
							if isOpenLine {
								break
							}
						}
					}
				}
			}
			if langName == "csharp" || langName == "rust" {
				prevIdx := def.start - 2
				if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
					isNativeDocstring = true
					for lineIndex := prevIdx; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if !strings.HasPrefix(line, "///") {
							break
						}
						commentText := strings.TrimSpace(strings.TrimPrefix(line, "///"))
						if commentText == "" {
							continue
						}
						commentText = strings.TrimPrefix(commentText, "<summary>")
						commentText = strings.TrimSuffix(commentText, "</summary>")
						commentText = strings.TrimSpace(commentText)
						if commentText == "<remarks>" || commentText == "</remarks>" || commentText == "" {
							continue
						}
						if IsSpecText(commentText) {
							hasRequirements = true
						} else {
							hasSummary = true
						}
					}
				}
			}
			if !isNativeDocstring && langName == "python" {
				defLineRaw := lines[def.start-1]
				parenDepth := 0
				for _, ch := range defLineRaw {
					if ch == '(' {
						parenDepth++
					}
					if ch == ')' {
						parenDepth--
					}
				}
				bodyStart := def.start
				if parenDepth > 0 {
					for scanIdx := def.start; scanIdx < len(lines) && scanIdx < def.start+15; scanIdx++ {
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
						isNativeDocstring = true
						quote := `"""`
						if strings.HasPrefix(trimmed, `'''`) {
							quote = `'''`
						}
						afterOpen := strings.TrimPrefix(trimmed, quote)
						closeIdx := strings.Index(afterOpen, quote)
						if closeIdx >= 0 {
							content := strings.TrimSpace(afterOpen[:closeIdx])
							if content != "" {
								if IsSpecText(content) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
						} else {
							firstContent := strings.TrimSpace(afterOpen)
							if firstContent != "" {
								if IsSpecText(firstContent) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
							for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
								sline := strings.TrimSpace(lines[scanIdx])
								if sline == quote {
									break
								}
								if strings.HasSuffix(sline, quote) {
									content := strings.TrimSpace(strings.TrimSuffix(sline, quote))
									if content != "" {
										if IsSpecText(content) {
											hasRequirements = true
										} else {
											hasSummary = true
										}
									}
									break
								}
								if sline == "" {
									continue
								}
								if IsSpecText(sline) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
						}
					}
					break
				}
			}
			if langName == "python" && isNativeDocstring {
				for lineIndex := def.start - 2; lineIndex >= 0; lineIndex-- {
					line := strings.TrimSpace(lines[lineIndex])
					if line == "" {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText != "" {
						isNativeDocstring = false
						break
					}
				}
			}
			if !isNativeDocstring {
				if langName == "go" {
					isNativeDocstring = true
				}
				for lineIndex := def.start - 2; lineIndex >= 0; lineIndex-- {
					line := strings.TrimSpace(lines[lineIndex])
					if line == "" {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText == "" {
						continue
					}
					if IsSpecText(commentText) {
						hasRequirements = true
					} else {
						hasSummary = true
					}
				}
			}
			if !isNativeDocstring && (hasSummary || hasRequirements) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is not using native docstring format in %s:%d", def.name, file, def.start),
					model.BreachCodeDefNotNativeDocstring,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
			if !hasSummary {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is missing a summary comment in %s:%d", def.name, file, def.start),
					model.BreachCodeDefMissingSummary,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
			if !hasRequirements && requiresDefinitionRequirements(defLine, language.Name()) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is missing spec comments in %s:%d", def.name, file, def.start),
					model.BreachCodeDefMissingRequirements,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

// #endregion 🧩️Section Policy
