// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📝️sections carries the section parsing entry points, the definition hydration and the section lookup.

// #endregion 🧲️Header

package languages

import (
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 📝️Sections

// 📑️ParseCodeSections MUST return an error when the input is malformed.
// 📑️ParseCodeSections parses the input and returns the code sections result.
func ParseCodeSections(content string, languageName string) []model.Section {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

// 📰️ParseMarkdownSectionsInternal MUST return an error when the input is malformed.
// 📰️ParseMarkdownSectionsInternal parses the input and returns the markdown sections internal result.
func ParseMarkdownSectionsInternal(content string) []model.Section {
	lines := strings.Split(content, "\n")
	var sections []model.Section
	type stackItem struct {
		level   int
		section *model.Section
	}
	var stack []stackItem
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	frontmatterLines := 0
	if strings.HasPrefix(content, "---") {
		endIndex := strings.Index(content[3:], "---")
		if endIndex != -1 {
			frontmatterContent := content[:endIndex+6]
			frontmatterLines = strings.Count(frontmatterContent, "\n")
		}
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(match[2])
			for len(stack) > 0 && stack[len(stack)-1].level >= level {
				popped := stack[len(stack)-1]
				popped.section.EndLine = frontmatterLines + i
				popped.section.EndIndex = lineStart - 1
				stack = stack[:len(stack)-1]
			}
			section := &model.Section{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []model.Section{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.section.Children = append(parent.section.Children, *section)
				section = &parent.section.Children[len(parent.section.Children)-1]
			} else {
				sections = append(sections, *section)
				section = &sections[len(sections)-1]
			}
			stack = append(stack, stackItem{level: level, section: section})
		}
		charIndex += len(line) + 1
	}
	for len(stack) > 0 {
		popped := stack[len(stack)-1]
		popped.section.EndLine = frontmatterLines + len(lines)
		popped.section.EndIndex = len(content)
		stack = stack[:len(stack)-1]
	}
	return sections
}

// 📝️ParseSections MUST return an error when the input is malformed.
// 📩️ParseSections parses the input and returns the sections result.
func ParseSections(content string, filePath string) []model.Section {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	return language.ParseSections(content)
}

// 📖️ParseDefinitions MUST return an error when the input is malformed.
// 📖️ParseDefinitions parses the input and returns the definitions result.
func ParseDefinitions(content string, filePath string) []model.Definition {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	if !language.SupportsDefinitions() {
		return nil
	}
	lines := strings.Split(content, "\n")
	ranges := language.ParseDefinitions(content, lines)
	definitions := make([]model.Definition, len(ranges))
	for i, r := range ranges {
		kind := model.DeriveDefinitionKind(r.Kind)
		definitions[i] = model.Definition{
			Name:      r.Name,
			Kind:      kind,
			StartLine: r.Start,
			EndLine:   r.End,
			FilePath:  filePath,
		}
	}
	return definitions
}

// 🎯️HydrateSectionsWithDefinitions MUST attach all matching child elements to their parents.
// 📦️HydrateSectionsWithDefinitions populates the sections with definitions with associated child data.
func HydrateSectionsWithDefinitions(sections []model.Section, definitions []model.Definition) []model.Section {
	if len(sections) == 0 {
		return sections
	}
	newSections := make([]model.Section, len(sections))
	for i := range sections {
		newSections[i] = sections[i]
		start := newSections[i].StartLine
		end := newSections[i].EndLine

		var subset []model.Definition
		for _, def := range definitions {
			if def.StartLine >= start && def.EndLine <= end {
				subset = append(subset, def)
			}
		}

		newSections[i].Children = HydrateSectionsWithDefinitions(newSections[i].Children, subset)

		var myDefs []model.Definition
		for _, def := range subset {
			inChild := false
			for _, child := range newSections[i].Children {
				if def.StartLine >= child.StartLine && def.EndLine <= child.EndLine {
					inChild = true
					break
				}
			}
			if !inChild {
				myDefs = append(myDefs, def)
			}
		}
		newSections[i].Definitions = myDefs
	}
	return newSections
}

// 🛤️NormalizeSectionPath MUST be idempotent for already-normalized values.
// ❓️NormalizeSectionPath normalizes the section path to its canonical form.
func NormalizeSectionPath(sectionPath string) []string {
	cleaned := strings.ReplaceAll(sectionPath, "#", "/")
	raw := strings.FieldsFunc(cleaned, func(r rune) bool { return r == '/' })
	var parts []string
	for _, part := range raw {
		if part != "" {
			parts = append(parts, part)
		}
	}
	return parts
}

// ⬜️FindSection MUST return nil when no match is found.
// 🔎️FindSection searches for and returns the matching section.
func FindSection(sections []model.Section, name string) *model.Section {
	for i := range sections {
		if sections[i].Name == name {
			return &sections[i]
		}
		if found := FindSection(sections[i].Children, name); found != nil {
			return found
		}
	}
	return nil
}

// #endregion 📝️Sections
