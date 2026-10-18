// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧱️autofix_sections carries the section region autofixes.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	strings "strings"
)

// #region 🧱️Autofix Sections

// 🕳️fixEmptySection marks an empty section and its surrounding blank line for removal.
func (s *autofixState) fixEmptySection(v model.Breach) {
	sectionStartLine := 0
	sectionEndLine := 0
	for i := v.Line - 1; i >= 0; i-- {
		if s.language != nil {
			if matched, _ := s.language.PolicySectionStartMatch(s.lines[i]); matched {
				sectionStartLine = i + 1
				break
			}
		}
	}
	for i := v.Line - 1; i < len(s.lines); i++ {
		if s.language != nil {
			if matched, _ := s.language.PolicySectionEndMatch(s.lines[i]); matched {
				sectionEndLine = i + 1
				break
			}
		}
	}
	if sectionStartLine > 0 && sectionEndLine > 0 {
		for i := sectionStartLine; i <= sectionEndLine; i++ {
			s.linesToRemove[i] = true
		}
		hasPrecedingBlank := sectionStartLine > 1 && strings.TrimSpace(s.lines[sectionStartLine-2]) == ""
		hasFollowingBlank := sectionEndLine < len(s.lines) && strings.TrimSpace(s.lines[sectionEndLine]) == ""
		if hasPrecedingBlank {
			s.linesToRemove[sectionStartLine-1] = true
		}
		if hasFollowingBlank && !hasPrecedingBlank {
			s.linesToRemove[sectionEndLine+1] = true
		}
		s.fixed++
	}
}

// 📏️fixNewlineAfterRegion marks the blank line after a region start for removal.
func (s *autofixState) fixNewlineAfterRegion(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) {
		if strings.TrimSpace(s.lines[v.Line-1]) == "" {
			s.linesToRemove[v.Line] = true
			s.fixed++
		}
	}
}

// 🏷️fixMissingSectionEndName names a section end after its matching start.
func (s *autofixState) fixMissingSectionEndName(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		line := s.lines[v.Line-1]
		if matched, _ := s.language.PolicySectionEndMatch(line); matched {
			startName := findMatchingSectionStartName(s.lines, v.Line-1, s.language)
			if startName != "" {
				s.lines[v.Line-1] = s.language.FormatSectionEnd(startName)
				s.fixed++
			}
		}
	}
}

// 🔀️fixSectionNameMismatch renames a section end to match its start.
func (s *autofixState) fixSectionNameMismatch(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		startName := findMatchingSectionStartName(s.lines, v.Line-1, s.language)
		if startName != "" {
			s.lines[v.Line-1] = s.language.FormatSectionEnd(startName)
			s.fixed++
		}
	}
}

// 🗒️fixMissingSectionSummary inserts a missing section summary comment.
func (s *autofixState) fixMissingSectionSummary(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) && s.language != nil {
		sectionName := ""
		if idx := strings.Index(v.Scope, "#"); idx >= 0 {
			sectionName = v.Scope[idx+1:]
		}
		if sectionName != "" {
			prefix := s.language.CommentPrefix()
			summaryLine := prefix + " " + sectionName + " MUST provide the " + strings.ToLower(sectionName) + " functionality."
			insertAt := v.Line
			for i := v.Line; i < len(s.lines); i++ {
				line := strings.TrimSpace(s.lines[i])
				if line == "" {
					continue
				}
				if strings.HasPrefix(line, prefix) {
					commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
					if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://section/") {
						insertAt = i + 1
						break
					}
				}
				break
			}
			newLines := make([]string, 0, len(s.lines)+1)
			newLines = append(newLines, s.lines[:insertAt]...)
			newLines = append(newLines, summaryLine)
			newLines = append(newLines, s.lines[insertAt:]...)
			s.lines = newLines
			s.fixed++
		}
	}
}

// #endregion 🧱️Autofix Sections
