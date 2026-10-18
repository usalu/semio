// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📖️extract carries the markdown, header and docstring extraction used to generate technology documents.

// #endregion 🧲️Header

package testrunner

import (
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🕸️Test Command

// 😀️headingEmojiRe holds the data fields for a headingEmojiRe record.
var headingEmojiRe = regexp.MustCompile(`^[\x{1F000}-\x{1FFFF}\x{2600}-\x{27FF}\x{FE00}-\x{FE0F}]+\s*`)

// 📑️ExtractMarkdownSection holds the data fields for a ExtractMarkdownSection record.
func ExtractMarkdownSection(content string, sectionName string) string {
	lines := strings.Split(content, "\n")
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	inSection := false
	sectionLevel := 0
	var result []string
	for _, line := range lines {
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(headingEmojiRe.ReplaceAllString(match[2], ""))
			if !inSection && name == sectionName && level == 1 {
				inSection = true
				sectionLevel = level
				continue
			}
			if inSection && level <= sectionLevel {
				break
			}
		}
		if inSection {
			result = append(result, line)
		}
	}
	text := strings.TrimSpace(strings.Join(result, "\n"))
	return text
}

// 🗃️isLicenseText holds the data fields for a isLicenseText record.
func isLicenseText(text string) bool {
	lower := strings.ToLower(text)
	return strings.Contains(lower, "gnu") ||
		strings.Contains(lower, "license") ||
		strings.Contains(lower, "free software") ||
		strings.Contains(lower, "warranty") ||
		strings.Contains(lower, "redistribute") ||
		strings.Contains(lower, "copyright")
}

// 🟨️isHeaderMetaLine holds the data fields for a isHeaderMetaLine record.
func isHeaderMetaLine(text string) bool {
	if strings.HasPrefix(text, "[") && strings.Contains(text, "](") {
		return true
	}
	if strings.HasPrefix(text, "#region") || strings.HasPrefix(text, "#endregion") ||
		strings.HasPrefix(text, "region ") || text == "region" ||
		strings.HasPrefix(text, "endregion") {
		return true
	}
	if len(text) > 0 && text[0] >= '0' && text[0] <= '9' {
		return true
	}
	if strings.HasPrefix(text, "💻️") || strings.HasPrefix(text, "🧰️") || strings.HasPrefix(text, "🔬️") {
		return true
	}
	return false
}

func ExtractFileHeaderSummary(filePath string) string {
	absPath := filepath.Join(workspace.RootDir, filePath)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return ""
	}
	lang := languages.GetLanguage(filePath)
	if lang == nil || !lang.SupportsHeaders() {
		return ""
	}
	prefix := lang.CommentPrefix()
	sections := lang.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	if headerSection == nil {
		return ""
	}
	lines := strings.Split(content, "\n")
	type commentBlock struct {
		lines []string
	}
	var blocks []commentBlock
	var currentBlock []string
	inBlock := false
	for i := headerSection.StartLine; i < headerSection.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			if inBlock {
				blocks = append(blocks, commentBlock{lines: currentBlock})
				currentBlock = nil
				inBlock = false
			}
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			continue
		}
		inBlock = true
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		currentBlock = append(currentBlock, commentText)
	}
	if len(currentBlock) > 0 {
		blocks = append(blocks, commentBlock{lines: currentBlock})
	}
	for _, block := range blocks {
		allMeta := true
		anyLicense := false
		hasSpec := false
		for _, l := range block.lines {
			if !isHeaderMetaLine(l) {
				allMeta = false
			}
			if isLicenseText(l) {
				anyLicense = true
			}
			if statutes.IsSpecText(l) {
				hasSpec = true
			}
		}
		if allMeta || anyLicense || hasSpec {
			continue
		}
		return strings.TrimSpace(strings.Join(block.lines, "\n"))
	}
	return ""
}

// 📄️ExtractFileHeaderRequirements holds the data fields for a ExtractFileHeaderRequirements record.
func ExtractFileHeaderRequirements(filePath string) string {
	absPath := filepath.Join(workspace.RootDir, filePath)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return ""
	}
	lang := languages.GetLanguage(filePath)
	if lang == nil || !lang.SupportsHeaders() {
		return ""
	}
	prefix := lang.CommentPrefix()
	sections := lang.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	if headerSection == nil {
		return ""
	}
	lines := strings.Split(content, "\n")
	type commentBlock struct {
		lines []string
	}
	var blocks []commentBlock
	var currentBlock []string
	inBlock := false
	for i := headerSection.StartLine; i < headerSection.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			if inBlock {
				blocks = append(blocks, commentBlock{lines: currentBlock})
				currentBlock = nil
				inBlock = false
			}
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			continue
		}
		inBlock = true
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		currentBlock = append(currentBlock, commentText)
	}
	if len(currentBlock) > 0 {
		blocks = append(blocks, commentBlock{lines: currentBlock})
	}
	var requirementsLines []string
	for _, block := range blocks {
		hasSpec := false
		for _, l := range block.lines {
			if statutes.IsSpecText(l) {
				hasSpec = true
				break
			}
		}
		if hasSpec {
			requirementsLines = append(requirementsLines, block.lines...)
		}
	}
	return strings.TrimSpace(strings.Join(requirementsLines, "\n"))
}

func ExtractSectionLeadComments(content string, section model.Section, prefix string) (requirements string, summary string) {
	lines := strings.Split(content, "\n")
	lowerName := strings.ToLower(section.Name)
	if lowerName == "header" || lowerName == "license" {
		return "", ""
	}
	var specLines []string
	var summaryLines []string
	for i := section.StartLine; i < section.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			break
		}
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		if isHeaderMetaLine(commentText) {
			continue
		}
		if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://") {
			continue
		}
		if isLicenseText(commentText) {
			continue
		}
		if statutes.IsSpecText(commentText) {
			specLines = append(specLines, commentText)
		} else {
			summaryLines = append(summaryLines, commentText)
		}
	}
	return strings.TrimSpace(strings.Join(specLines, "\n")), strings.TrimSpace(strings.Join(summaryLines, "\n"))
}

// 📖️ExtractDefinitionDocstring holds the data fields for a ExtractDefinitionDocstring record.
func ExtractDefinitionDocstring(content string, def model.Definition, prefix string) (requirements string, summary string) {
	lines := strings.Split(content, "\n")
	for i := def.StartLine - 2; i >= 0; i-- {
		line := strings.TrimSpace(lines[i])
		if line == "" {
			break
		}
		if !strings.HasPrefix(line, prefix) {
			break
		}
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://") {
			continue
		}
		if statutes.IsSpecText(commentText) {
			requirements = commentText + "\n" + requirements
		} else {
			summary = commentText + "\n" + summary
		}
	}
	return strings.TrimSpace(requirements), strings.TrimSpace(summary)
}

// #endregion 🕸️Test Command
