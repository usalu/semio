// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📝️autofix_summary carries the definition summary autofix.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	strings "strings"
)

// #region 📝️Autofix Summary

// 📝️fixMissingSummary inserts a missing definition summary into the native docstring.
func (s *autofixState) fixMissingSummary(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		defName := ""
		if idx := strings.Index(v.Scope, "::"); idx >= 0 {
			defName = v.Scope[idx+2:]
		}
		if defName != "" {
			langName := s.language.Name()
			prefix := s.language.CommentPrefix()
			summaryText := defName + " holds the data fields for a " + defName + " record."
			defLine := s.lines[v.Line-1]
			trimmedDef := strings.TrimSpace(defLine)
			noPub := strings.TrimPrefix(trimmedDef, "pub ")
			if strings.HasPrefix(noPub, "(") {
				if cidx := strings.Index(noPub, ") "); cidx >= 0 {
					noPub = strings.TrimSpace(noPub[cidx+2:])
				}
			}
			noExport := strings.TrimPrefix(trimmedDef, "export ")
			noExport = strings.TrimLeft(noExport, "async abstract declare default ")
			if strings.HasPrefix(noPub, "fn ") || strings.HasPrefix(noExport, "function ") || strings.HasPrefix(trimmedDef, "def ") || strings.HasPrefix(trimmedDef, "async def ") || strings.HasPrefix(trimmedDef, "func ") {
				summaryText = defName + " performs the " + defName + " operation."
			}
			if langName == "python" {
				parenDepth := 0
				for _, ch := range defLine {
					if ch == '(' {
						parenDepth++
					}
					if ch == ')' {
						parenDepth--
					}
				}
				bodyStart := v.Line
				if parenDepth > 0 {
					for scanIdx := v.Line; scanIdx < len(s.lines) && scanIdx < v.Line+15; scanIdx++ {
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
				docstringFound := false
				for bodyIdx := bodyStart; bodyIdx < len(s.lines) && bodyIdx < bodyStart+5; bodyIdx++ {
					trimmed := strings.TrimSpace(s.lines[bodyIdx])
					if trimmed == "" {
						continue
					}
					if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
						docstringFound = true
						quote := `"""`
						if strings.HasPrefix(trimmed, `'''`) {
							quote = `'''`
						}
						afterOpen := strings.TrimPrefix(trimmed, quote)
						closeIdx := strings.Index(afterOpen, quote)
						bodyIndent := ""
						for _, ch := range s.lines[bodyIdx] {
							if ch == ' ' || ch == '\t' {
								bodyIndent += string(ch)
							} else {
								break
							}
						}
						if closeIdx >= 0 {
							existingContent := strings.TrimSpace(afterOpen[:closeIdx])
							if existingContent == "" || strings.HasPrefix(existingContent, "[") {
								if existingContent != "" {
									s.lines[bodyIdx] = bodyIndent + quote + summaryText
									newLines := make([]string, 0, len(s.lines)+2)
									newLines = append(newLines, s.lines[:bodyIdx+1]...)
									newLines = append(newLines, bodyIndent+existingContent)
									newLines = append(newLines, bodyIndent+quote)
									newLines = append(newLines, s.lines[bodyIdx+1:]...)
									s.lines = newLines
								} else {
									s.lines[bodyIdx] = bodyIndent + quote + summaryText + quote
								}
							}
						} else {
							firstContent := strings.TrimSpace(afterOpen)
							if firstContent == "" || strings.HasPrefix(firstContent, "[") {
								s.lines[bodyIdx] = bodyIndent + quote + summaryText
								if firstContent != "" {
									newLines := make([]string, 0, len(s.lines)+1)
									newLines = append(newLines, s.lines[:bodyIdx+1]...)
									newLines = append(newLines, bodyIndent+firstContent)
									newLines = append(newLines, s.lines[bodyIdx+1:]...)
									s.lines = newLines
								}
							}
						}
						s.fixed++
					}
					break
				}
				if !docstringFound {
					bodyIndent := "    "
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
					newLines := make([]string, 0, len(s.lines)+1)
					newLines = append(newLines, s.lines[:bodyStart]...)
					newLines = append(newLines, bodyIndent+`"""`+summaryText+`"""`)
					newLines = append(newLines, s.lines[bodyStart:]...)
					s.lines = newLines
					s.fixed++
				}
			} else if langName == "typescript" {
				prevIdx := v.Line - 2
				if prevIdx >= 0 {
					prevLine := strings.TrimSpace(s.lines[prevIdx])
					if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
						indent := ""
						for _, ch := range s.lines[v.Line-1] {
							if ch == ' ' || ch == '\t' {
								indent += string(ch)
							} else {
								break
							}
						}
						for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
							sline := strings.TrimSpace(s.lines[scanIdx])
							if strings.HasPrefix(sline, "/**") {
								openContent := strings.TrimPrefix(sline, "/**")
								openContent = strings.TrimSpace(openContent)
								if openContent == "" || openContent == "**/" || openContent == "*/" {
									s.lines[scanIdx] = indent + "/** " + summaryText
								}
								s.fixed++
								break
							}
						}
						return
					}
				}
				newLine := prefix + " " + summaryText
				insertAt := v.Line - 1
				for insertAt > 0 {
					prev := strings.TrimSpace(s.lines[insertAt-1])
					if prev == "" || !strings.HasPrefix(prev, prefix) {
						break
					}
					insertAt--
				}
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:insertAt]...)
				newLines = append(newLines, newLine)
				newLines = append(newLines, s.lines[insertAt:]...)
				s.lines = newLines
				s.fixed++
			} else if langName == "csharp" || langName == "rust" {
				prevIdx := v.Line - 2
				if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(s.lines[prevIdx]), "///") {
					hasSummaryTag := false
					docStartIdx := prevIdx
					for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
						sline := strings.TrimSpace(s.lines[scanIdx])
						if !strings.HasPrefix(sline, "///") {
							break
						}
						docStartIdx = scanIdx
						if strings.Contains(sline, "<summary>") {
							hasSummaryTag = true
							summaryContent := strings.TrimPrefix(sline, "///")
							summaryContent = strings.TrimSpace(summaryContent)
							summaryContent = strings.TrimPrefix(summaryContent, "<summary>")
							summaryContent = strings.TrimSuffix(summaryContent, "</summary>")
							summaryContent = strings.TrimSpace(summaryContent)
							if summaryContent == "" {
								s.lines[scanIdx] = "/// <summary>" + summaryText + "</summary>"
								s.fixed++
							}
							break
						}
					}
					if !hasSummaryTag {
						summaryLine := "/// <summary>" + summaryText + "</summary>"
						newLines := make([]string, 0, len(s.lines)+1)
						newLines = append(newLines, s.lines[:docStartIdx]...)
						newLines = append(newLines, summaryLine)
						newLines = append(newLines, s.lines[docStartIdx:]...)
						s.lines = newLines
						s.fixed++
					}
					return
				}
				summaryLine := "/// <summary>" + summaryText + "</summary>"
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:v.Line-1]...)
				newLines = append(newLines, summaryLine)
				newLines = append(newLines, s.lines[v.Line-1:]...)
				s.lines = newLines
				s.fixed++
			} else {
				newLine := prefix + " " + summaryText
				insertAt := v.Line - 1
				for insertAt > 0 {
					prev := strings.TrimSpace(s.lines[insertAt-1])
					if prev == "" || !strings.HasPrefix(prev, prefix) {
						break
					}
					insertAt--
				}
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:insertAt]...)
				newLines = append(newLines, newLine)
				newLines = append(newLines, s.lines[insertAt:]...)
				s.lines = newLines
				s.fixed++
			}
		}
	}
}

// #endregion 📝️Autofix Summary
