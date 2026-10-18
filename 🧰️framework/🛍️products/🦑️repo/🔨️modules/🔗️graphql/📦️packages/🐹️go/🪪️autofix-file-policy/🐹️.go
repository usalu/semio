// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪪️autofix_file_policy carries the file level emoji variation and license autofixes.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	strings "strings"
)

// #region 🪪️Autofix File Policy

// 🔣️fixEmojiVariation appends the missing emoji variation selector.
func (s *autofixState) fixEmojiVariation(v model.Breach) {
	if v.Line > 0 && v.Line <= len(s.lines) {
		line := s.lines[v.Line-1]

		line = strings.ReplaceAll(line, "\uFE0E", "\uFE0F")

		textDefaultEmojis := []string{
			"\U0001F3D7",
			"\u2328",
			"\U0001F5B1",
			"\U0001F5C3",
			"\u2699",
			"\u2696",
			"\U0001F3F7",
			"\U0001F6E0",
			"\u2702",
			"\U0001F6E1",
		}
		for _, emoji := range textDefaultEmojis {
			line = strings.ReplaceAll(line, emoji+"\uFE0F", emoji)
			line = strings.ReplaceAll(line, emoji, emoji+"\uFE0F")
		}
		s.lines[v.Line-1] = line
		s.fixed++
	}
}

// 🪪️fixLicense inserts or replaces the license region.
func (s *autofixState) fixLicense(v model.Breach) {
	if s.language != nil {
		sections := s.language.ParseSections(strings.Join(s.lines, "\n"))
		var headerSec *model.Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSec = &sections[i]
				break
			}
		}
		if headerSec != nil {
			var licenseSec *model.Section
			for i := range headerSec.Children {
				if strings.ToLower(headerSec.Children[i].Name) == "license" {
					licenseSec = &headerSec.Children[i]
					break
				}
			}
			prefix := s.language.CommentPrefix()
			licenseText := workspace.AGPLLicenseText()
			var licenseLines []string
			licenseLines = append(licenseLines, "")
			for _, ll := range strings.Split(licenseText, "\n") {
				if ll == "" {
					licenseLines = append(licenseLines, prefix)
				} else {
					licenseLines = append(licenseLines, prefix+" "+ll)
				}
			}
			licenseLines = append(licenseLines, "")
			if licenseSec != nil {
				newLines := make([]string, 0, len(s.lines)+len(licenseLines))
				newLines = append(newLines, s.lines[:licenseSec.StartLine]...)
				newLines = append(newLines, licenseLines...)
				newLines = append(newLines, s.lines[licenseSec.EndLine-1:]...)
				s.lines = newLines
			} else {
				requirementsSec := (*model.Section)(nil)
				for i := range headerSec.Children {
					if strings.ToLower(headerSec.Children[i].Name) == "requirements" {
						requirementsSec = &headerSec.Children[i]
						break
					}
				}
				insertBefore := headerSec.EndLine - 1
				if requirementsSec != nil {
					insertBefore = requirementsSec.StartLine - 1
				}
				regionStart := s.language.FormatSectionStart("License")
				regionEnd := s.language.FormatSectionEnd("License")
				var block []string
				block = append(block, regionStart)
				block = append(block, licenseLines...)
				block = append(block, regionEnd)
				block = append(block, "")
				newLines := make([]string, 0, len(s.lines)+len(block))
				newLines = append(newLines, s.lines[:insertBefore]...)
				newLines = append(newLines, block...)
				newLines = append(newLines, s.lines[insertBefore:]...)
				s.lines = newLines
			}
			s.fixed++
		}
	}
}

// #endregion 🪪️Autofix File Policy
