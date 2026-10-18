// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📑️autofix_docstring carries the native docstring conversion autofix.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	statutespkg "github.com/usalu/semio/repo/statutes"
	strings "strings"
)

// #region 📑️Autofix Docstring

// 📑️fixNativeDocstring converts a leading comment into the native docstring of the language.
func (s *autofixState) fixNativeDocstring(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		langName := s.language.Name()
		defLineNum := v.Line
		prefix := s.language.CommentPrefix()
		switch langName {
		case "typescript":
			var commentTexts []string
			commentStartIdx := defLineNum - 1
			for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
				line := strings.TrimSpace(s.lines[lineIndex])
				if line == "" {
					break
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				commentStartIdx = lineIndex
				text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				commentTexts = append([]string{text}, commentTexts...)
			}
			if len(commentTexts) > 0 {
				var summaryLines, specLines, todoLines []string
				var identificationLine string
				inTodo := false
				for _, cl := range commentTexts {
					if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
						identificationLine = cl
						inTodo = false
						continue
					}
					if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
						inTodo = true
						todoLines = append(todoLines, cl)
						continue
					}
					if inTodo {
						todoLines = append(todoLines, cl)
						continue
					}
					if statutespkg.IsSpecText(cl) {
						specLines = append(specLines, cl)
					} else {
						summaryLines = append(summaryLines, cl)
					}
				}
				indent := ""
				if defLineNum-1 < len(s.lines) {
					raw := s.lines[defLineNum-1]
					for _, ch := range raw {
						if ch == ' ' || ch == '\t' {
							indent += string(ch)
						} else {
							break
						}
					}
				}
				var jsdocLines []string
				jsdocLines = append(jsdocLines, indent+"/**")
				for _, sl := range summaryLines {
					jsdocLines = append(jsdocLines, indent+" * "+sl)
				}
				if len(specLines) > 0 {
					if len(summaryLines) > 0 {
						jsdocLines = append(jsdocLines, indent+" *")
					}
					for _, sp := range specLines {
						jsdocLines = append(jsdocLines, indent+" * "+sp)
					}
				}
				if len(todoLines) > 0 {
					jsdocLines = append(jsdocLines, indent+" *")
					for _, td := range todoLines {
						jsdocLines = append(jsdocLines, indent+" * "+td)
					}
				}
				if identificationLine != "" {
					jsdocLines = append(jsdocLines, indent+" * "+identificationLine)
				}
				jsdocLines = append(jsdocLines, indent+" **/")
				newLines := make([]string, 0, len(s.lines)-len(commentTexts)+len(jsdocLines))
				newLines = append(newLines, s.lines[:commentStartIdx]...)
				newLines = append(newLines, jsdocLines...)
				newLines = append(newLines, s.lines[defLineNum-1:]...)
				s.lines = newLines
				s.fixed++
			}
		case "csharp", "rust":
			for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
				line := strings.TrimSpace(s.lines[lineIndex])
				if line == "" {
					break
				}
				if strings.HasPrefix(line, "///") {
					break
				}
				if strings.HasPrefix(line, "//") {
					s.lines[lineIndex] = strings.Replace(s.lines[lineIndex], "// ", "/// ", 1)
					s.fixed++
				} else {
					break
				}
			}
		case "python":
			var commentTexts []string
			commentStartIdx := defLineNum - 1
			for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
				line := strings.TrimSpace(s.lines[lineIndex])
				if line == "" {
					break
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				commentStartIdx = lineIndex
				text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				commentTexts = append([]string{text}, commentTexts...)
			}
			if len(commentTexts) > 0 {
				var cSummary, cRequirements, cTodos []string
				var cId string
				inTodo := false
				for _, cl := range commentTexts {
					if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
						cId = cl
						inTodo = false
						continue
					}
					if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
						inTodo = true
						cTodos = append(cTodos, cl)
						continue
					}
					if inTodo {
						cTodos = append(cTodos, cl)
						continue
					}
					if statutespkg.IsSpecText(cl) {
						cRequirements = append(cRequirements, cl)
					} else {
						cSummary = append(cSummary, cl)
					}
				}
				bodyIndent := "    "
				parenDepth := 0
				for _, ch := range s.lines[defLineNum-1] {
					if ch == '(' {
						parenDepth++
					}
					if ch == ')' {
						parenDepth--
					}
				}
				bodyStart := defLineNum
				if parenDepth > 0 {
					for scanIdx := defLineNum; scanIdx < len(s.lines) && scanIdx < defLineNum+15; scanIdx++ {
						for _, ch := range s.lines[scanIdx] {
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
				if bodyStart < len(s.lines) {
					raw := s.lines[bodyStart]
					detected := ""
					for _, ch := range raw {
						if ch == ' ' || ch == '\t' {
							detected += string(ch)
						} else {
							break
						}
					}
					if detected != "" {
						bodyIndent = detected
					}
				}
				existingDocStart := -1
				existingDocEnd := -1
				existingQuote := `"""`
				for bodyIdx := bodyStart; bodyIdx < len(s.lines) && bodyIdx < bodyStart+5; bodyIdx++ {
					trimmed := strings.TrimSpace(s.lines[bodyIdx])
					if trimmed == "" {
						continue
					}
					if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
						existingDocStart = bodyIdx
						if strings.HasPrefix(trimmed, `'''`) {
							existingQuote = `'''`
						}
						afterOpen := strings.TrimPrefix(trimmed, existingQuote)
						closeIdx := strings.Index(afterOpen, existingQuote)
						if closeIdx >= 0 {
							existingDocEnd = bodyIdx
						} else {
							for scanIdx := bodyIdx + 1; scanIdx < len(s.lines); scanIdx++ {
								sline := strings.TrimSpace(s.lines[scanIdx])
								if sline == existingQuote || strings.HasSuffix(sline, existingQuote) {
									existingDocEnd = scanIdx
									break
								}
							}
						}
					}
					break
				}
				var eSummary, eRequirements, eTodos []string
				var eId string
				if existingDocStart >= 0 && existingDocEnd >= 0 {
					for lineIdx := existingDocStart; lineIdx <= existingDocEnd; lineIdx++ {
						trimmed := strings.TrimSpace(s.lines[lineIdx])
						trimmed = strings.TrimPrefix(trimmed, existingQuote)
						trimmed = strings.TrimSuffix(trimmed, existingQuote)
						trimmed = strings.TrimSpace(trimmed)
						if trimmed == "" {
							continue
						}
						if strings.HasPrefix(trimmed, "[") && strings.Contains(trimmed, "](repo://definition/") {
							eId = trimmed
						} else if statutespkg.IsSpecText(trimmed) {
							eRequirements = append(eRequirements, trimmed)
						} else if strings.HasPrefix(trimmed, "TODO:") || strings.HasPrefix(trimmed, "TODO ") {
							eTodos = append(eTodos, trimmed)
						} else {
							eSummary = append(eSummary, trimmed)
						}
					}
				}
				mergedSummary := eSummary
				if len(mergedSummary) == 0 {
					mergedSummary = cSummary
				}
				mergedRequirements := cRequirements
				if len(mergedRequirements) == 0 {
					mergedRequirements = eRequirements
				}
				mergedTodos := cTodos
				if len(mergedTodos) == 0 {
					mergedTodos = eTodos
				}
				mergedId := cId
				if mergedId == "" {
					mergedId = eId
				}
				var docLines []string
				for _, sl := range mergedSummary {
					docLines = append(docLines, sl)
				}
				for _, sp := range mergedRequirements {
					docLines = append(docLines, sp)
				}
				for _, td := range mergedTodos {
					docLines = append(docLines, td)
				}
				if mergedId != "" {
					docLines = append(docLines, mergedId)
				}
				var tripleQuoteLines []string
				if len(docLines) == 1 {
					tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0]+`"""`)
				} else if len(docLines) > 1 {
					tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0])
					for i := 1; i < len(docLines); i++ {
						tripleQuoteLines = append(tripleQuoteLines, bodyIndent+docLines[i])
					}
					tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`)
				}
				if len(tripleQuoteLines) > 0 {
					afterDoc := bodyStart
					if existingDocEnd >= 0 {
						afterDoc = existingDocEnd + 1
					}
					newLines := make([]string, 0, len(s.lines))
					newLines = append(newLines, s.lines[:commentStartIdx]...)
					newLines = append(newLines, s.lines[defLineNum-1:bodyStart]...)
					newLines = append(newLines, tripleQuoteLines...)
					newLines = append(newLines, s.lines[afterDoc:]...)
					s.lines = newLines
					s.fixed++
				}
			}
		}
	}
}

// #endregion 📑️Autofix Docstring
