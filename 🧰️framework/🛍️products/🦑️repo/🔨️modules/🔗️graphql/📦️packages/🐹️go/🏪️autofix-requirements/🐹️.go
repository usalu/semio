// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📋️autofix_requirements carries the definition requirements autofix.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	strings "strings"
)

// #region 📋️Autofix Requirements

// 📋️fixMissingRequirements inserts the missing requirements into the native docstring.
func (s *autofixState) fixMissingRequirements(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		defName := ""
		if idx := strings.Index(v.Scope, "::"); idx >= 0 {
			defName = v.Scope[idx+2:]
		}
		if defName != "" {
			langName := s.language.Name()
			prefix := s.language.CommentPrefix()
			specText := v.Excerpt
			if langName == "python" {
				parenDepth := 0
				for _, ch := range s.lines[v.Line-1] {
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
				for bodyIdx := bodyStart; bodyIdx < len(s.lines) && bodyIdx < bodyStart+5; bodyIdx++ {
					trimmed := strings.TrimSpace(s.lines[bodyIdx])
					if trimmed == "" {
						continue
					}
					if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
						quote := `"""`
						if strings.HasPrefix(trimmed, `'''`) {
							quote = `'''`
						}
						bodyIndent := ""
						for _, ch := range s.lines[bodyIdx] {
							if ch == ' ' || ch == '\t' {
								bodyIndent += string(ch)
							} else {
								break
							}
						}
						afterOpen := strings.TrimPrefix(trimmed, quote)
						closeIdx := strings.Index(afterOpen, quote)
						if closeIdx >= 0 {
							existingContent := strings.TrimSpace(afterOpen[:closeIdx])
							s.lines[bodyIdx] = bodyIndent + quote + existingContent
							newLines := make([]string, 0, len(s.lines)+2)
							newLines = append(newLines, s.lines[:bodyIdx+1]...)
							newLines = append(newLines, bodyIndent+specText)
							newLines = append(newLines, bodyIndent+quote)
							newLines = append(newLines, s.lines[bodyIdx+1:]...)
							s.lines = newLines
						} else {
							for scanIdx := bodyIdx + 1; scanIdx < len(s.lines); scanIdx++ {
								sline := strings.TrimSpace(s.lines[scanIdx])
								if sline == quote || strings.HasSuffix(sline, quote) {
									insertIdx := scanIdx
									for backIdx := scanIdx - 1; backIdx > bodyIdx; backIdx-- {
										bline := strings.TrimSpace(s.lines[backIdx])
										if strings.HasPrefix(bline, "[") && strings.Contains(bline, "](repo://definition/") {
											insertIdx = backIdx
											break
										}
									}
									newLines := make([]string, 0, len(s.lines)+1)
									newLines = append(newLines, s.lines[:insertIdx]...)
									newLines = append(newLines, bodyIndent+specText)
									newLines = append(newLines, s.lines[insertIdx:]...)
									s.lines = newLines
									break
								}
							}
						}
						s.fixed++
					}
					break
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
								newLines := make([]string, 0, len(s.lines)+1)
								newLines = append(newLines, s.lines[:scanIdx+1]...)
								newLines = append(newLines, indent+" * "+specText)
								newLines = append(newLines, s.lines[scanIdx+1:]...)
								s.lines = newLines
								s.fixed++
								break
							}
						}
						return
					}
				}
				newLine := prefix + " " + specText
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:v.Line-1]...)
				newLines = append(newLines, newLine)
				newLines = append(newLines, s.lines[v.Line-1:]...)
				s.lines = newLines
				s.fixed++
			} else if langName == "csharp" || langName == "rust" {
				prevIdx := v.Line - 2
				if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(s.lines[prevIdx]), "///") {
					hasRemarks := false
					remarksEnd := -1
					for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
						sline := strings.TrimSpace(s.lines[scanIdx])
						if !strings.HasPrefix(sline, "///") {
							break
						}
						if strings.Contains(sline, "</remarks>") {
							remarksEnd = scanIdx
						}
						if strings.Contains(sline, "<remarks>") {
							hasRemarks = true
							break
						}
					}
					if hasRemarks && remarksEnd >= 0 {
						newLines := make([]string, 0, len(s.lines)+1)
						newLines = append(newLines, s.lines[:remarksEnd]...)
						newLines = append(newLines, "/// "+specText)
						newLines = append(newLines, s.lines[remarksEnd:]...)
						s.lines = newLines
						s.fixed++
					} else {
						newLines := make([]string, 0, len(s.lines)+3)
						newLines = append(newLines, s.lines[:v.Line-1]...)
						newLines = append(newLines, "/// <remarks>")
						newLines = append(newLines, "/// "+specText)
						newLines = append(newLines, "/// </remarks>")
						newLines = append(newLines, s.lines[v.Line-1:]...)
						s.lines = newLines
						s.fixed++
					}
					return
				}
				newLine := "/// " + specText
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:v.Line-1]...)
				newLines = append(newLines, newLine)
				newLines = append(newLines, s.lines[v.Line-1:]...)
				s.lines = newLines
				s.fixed++
			} else {
				newLine := prefix + " " + specText
				newLines := make([]string, 0, len(s.lines)+1)
				newLines = append(newLines, s.lines[:v.Line-1]...)
				newLines = append(newLines, newLine)
				newLines = append(newLines, s.lines[v.Line-1:]...)
				s.lines = newLines
				s.fixed++
			}
		}
	}
}

// #endregion 📋️Autofix Requirements
