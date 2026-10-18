// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 💬️autofix_comments carries the inline and block comment autofixes.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	strings "strings"
)

// #region 💬️Autofix Comments

// 💬️fixInlineComment removes an inline comment inside a definition.
func (s *autofixState) fixInlineComment(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		startLine := v.Line
		prefix := s.language.CommentPrefix()
		var pendingBlanks []int
		for i := startLine; i <= len(s.lines); i++ {
			line := s.lines[i-1]
			trimmed := strings.TrimSpace(line)
			if trimmed == "" {
				pendingBlanks = append(pendingBlanks, i)
				continue
			}
			if i == startLine && v.Column > 1 {
				if v.Column <= len(line) {
					s.lines[i-1] = strings.TrimRight(line[:v.Column-1], " \t")
					pendingBlanks = nil
					continue
				}
			}
			if !strings.HasPrefix(trimmed, prefix) {
				break
			}
			if matched, _ := s.language.PolicySectionStartMatch(line); matched {
				break
			}
			if matched, _ := s.language.PolicySectionEndMatch(line); matched {
				break
			}
			isSkipDirective := false
			for _, d := range s.language.SkipDirectives() {
				if strings.HasPrefix(trimmed, prefix+" "+d) {
					isSkipDirective = true
					break
				}
			}
			if isSkipDirective {
				break
			}
			if strings.Contains(s.lines[i-1], "[TRACE]") {
				break
			}
			for _, bl := range pendingBlanks {
				s.linesToRemove[bl] = true
			}
			pendingBlanks = nil
			s.linesToRemove[i] = true
		}
		s.fixed++
	}
}

// 🗨️fixBlockComment removes a block comment inside a definition.
func (s *autofixState) fixBlockComment(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		startLine := v.Line
		endPrefix := s.language.BlockCommentEnd()
		startPrefix := s.language.BlockCommentStart()
		for i := startLine; i <= len(s.lines); i++ {
			line := s.lines[i-1]
			if i == startLine && v.Column > 1 {
				idx := strings.Index(line[v.Column-1:], startPrefix)
				if idx >= 0 {
					idx += v.Column - 1
					left := strings.TrimRight(line[:idx], " \t")
					if strings.Contains(line[idx:], endPrefix) {
						endIdx := strings.Index(line[idx:], endPrefix) + idx
						right := ""
						if endIdx+len(endPrefix) < len(line) {
							right = line[endIdx+len(endPrefix):]
						}
						if left != "" && strings.TrimSpace(right) != "" {
							s.lines[i-1] = left + " " + strings.TrimLeft(right, " \t")
						} else {
							s.lines[i-1] = left + right
						}
						if strings.TrimSpace(s.lines[i-1]) == "" {
							s.linesToRemove[i] = true
						}
						break
					}
					if left != "" {
						s.lines[i-1] = left
					} else {
						s.linesToRemove[i] = true
					}
					continue
				}
			}
			if strings.Contains(line, endPrefix) {
				idx := strings.Index(line, endPrefix)
				if idx+len(endPrefix) < len(line) {
					s.lines[i-1] = strings.TrimLeft(line[idx+len(endPrefix):], " \t")
					if strings.TrimSpace(s.lines[i-1]) == "" {
						s.linesToRemove[i] = true
					}
				} else {
					s.linesToRemove[i] = true
				}
				break
			}
			s.linesToRemove[i] = true
		}
		s.fixed++
	}
}

// #endregion 💬️Autofix Comments
