// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Section and definition parsing.

// #endregion 🧲️Header

package client

import (
	"regexp"
	"strings"
)

// 📑️ParseCodeSections MUST return an error when the input is malformed.
// 📑️ParseCodeSections parses the input and returns the code sections result.
func ParseCodeSections(content string, languageName string) []Section {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

// 📰️ParseMarkdownSectionsInternal MUST return an error when the input is malformed.
// 📰️ParseMarkdownSectionsInternal parses the input and returns the markdown sections internal result.
func ParseMarkdownSectionsInternal(content string) []Section {
	lines := strings.Split(content, "\n")
	var sections []Section
	type stackItem struct {
		level   int
		section *Section
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
			section := &Section{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []Section{},
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

// 💿️JsonSectionLocation holds the data fields for a json section location record.
type JsonSectionLocation struct {
	Path       string
	KeyStart   int
	KeyEnd     int
	ValueStart int
	ValueEnd   int
	Section    *Section
}

// 📋️jsonContext holds the data fields for a jsonContext record.
type jsonContext struct {
	kind      byte
	section   *Section
	path      string
	expectKey bool
	location  *JsonSectionLocation
}

// ❌️ParseJSONSectionsDetailed MUST return an error when the input is malformed.
// 💾️ParseJSONSectionsDetailed parses the input and returns the j s o n sections detailed result.
func ParseJSONSectionsDetailed(content string) ([]Section, map[string]*JsonSectionLocation, error) {
	var sections []Section
	locations := make(map[string]*JsonSectionLocation)
	var stack []jsonContext
	line := 1
	inString := false
	escape := false
	stringStart := 0
	stringBuf := strings.Builder{}
	pendingKey := ""
	pendingKeyStart := 0
	pendingKeyEnd := 0
	pendingKeyLine := 0
	var awaitingValue *JsonSectionLocation
	for i := 0; i < len(content); i++ {
		ch := content[i]
		if ch == '\n' {
			line++
		}
		if inString {
			if escape {
				escape = false
				stringBuf.WriteByte(ch)
			}
			if ch == '\\' {
				escape = true
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '"' {
				inString = false
				value := stringBuf.String()
				stringBuf.Reset()
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' && stack[len(stack)-1].expectKey && awaitingValue == nil {
					pendingKey = value
					pendingKeyStart = stringStart
					pendingKeyEnd = i
					pendingKeyLine = line
					stack[len(stack)-1].expectKey = false
				} else if awaitingValue != nil && awaitingValue.ValueStart == stringStart {
					awaitingValue.ValueEnd = i
					awaitingValue.Section.EndLine = line
					awaitingValue.Section.EndIndex = i + 1
					awaitingValue = nil
				}
				continue
			}
			stringBuf.WriteByte(ch)
			continue
		}
		if ch == '"' {
			if awaitingValue != nil {
				awaitingValue.ValueStart = i
			}
			inString = true
			stringStart = i
			continue
		}
		if len(stack) > 0 && stack[len(stack)-1].kind == '{' && pendingKey != "" && ch == ':' {
			parent := stack[len(stack)-1].section
			path := pendingKey
			if stack[len(stack)-1].path != "" {
				path = stack[len(stack)-1].path + "/" + pendingKey
			}
			section := Section{
				Name:       pendingKey,
				StartLine:  pendingKeyLine,
				EndLine:    -1,
				StartIndex: pendingKeyStart,
				EndIndex:   -1,
				Children:   []Section{},
			}
			var sectionRef *Section
			if parent != nil {
				parent.Children = append(parent.Children, section)
				sectionRef = &parent.Children[len(parent.Children)-1]
			} else {
				sections = append(sections, section)
				sectionRef = &sections[len(sections)-1]
			}
			location := &JsonSectionLocation{
				Path:     path,
				KeyStart: pendingKeyStart,
				KeyEnd:   pendingKeyEnd,
				Section:  sectionRef,
			}
			locations[path] = location
			awaitingValue = location
			pendingKey = ""
			continue
		}
		if awaitingValue != nil {
			if ch == '{' || ch == '[' {
				awaitingValue.ValueStart = i
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   awaitingValue.Section,
					path:      awaitingValue.Path,
					expectKey: ch == '{',
					location:  awaitingValue,
				})
				awaitingValue = nil
				continue
			}
			if ch == '-' || (ch >= '0' && ch <= '9') || ch == 't' || ch == 'f' || ch == 'n' {
				awaitingValue.ValueStart = i
				end := i
				for end < len(content) {
					c := content[end]
					if c == '\n' {
						line++
					}
					if c == ',' || c == '}' || c == ']' || c == ' ' || c == '\t' || c == '\r' || c == '\n' {
						break
					}
					end++
				}
				awaitingValue.ValueEnd = end - 1
				awaitingValue.Section.EndLine = line
				awaitingValue.Section.EndIndex = end
				awaitingValue = nil
				i = end - 1
				continue
			}
		}
		if ch == '{' || ch == '[' {
			if awaitingValue == nil {
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   nil,
					path:      "",
					expectKey: ch == '{',
					location:  nil,
				})
			}
			continue
		}
		if ch == '}' || ch == ']' {
			if len(stack) > 0 {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if top.location != nil {
					top.location.ValueEnd = i
					top.location.Section.EndLine = line
					top.location.Section.EndIndex = i + 1
				}
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
					stack[len(stack)-1].expectKey = true
				}
			}
			continue
		}
		if ch == ',' {
			if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
				stack[len(stack)-1].expectKey = true
			}
			continue
		}
	}
	for _, location := range locations {
		if location.Section.EndIndex == -1 {
			location.Section.EndLine = line
			location.Section.EndIndex = len(content)
		}
	}
	return sections, locations, nil
}

// 🔬️ParseJSONSections MUST return an error when the input is malformed.
// 📝️ParseJSONSections parses the input and returns the j s o n sections result.
func ParseJSONSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

// 📝️ParseSections MUST return an error when the input is malformed.
// 📩️ParseSections parses the input and returns the sections result.
func ParseSections(content string, filePath string) []Section {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	return language.ParseSections(content)
}

// 📖️ParseDefinitions MUST return an error when the input is malformed.
// 📖️ParseDefinitions parses the input and returns the definitions result.
func ParseDefinitions(content string, filePath string) []Definition {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	if !language.SupportsDefinitions() {
		return nil
	}
	lines := strings.Split(content, "\n")
	ranges := language.ParseDefinitions(content, lines)
	definitions := make([]Definition, len(ranges))
	for i, r := range ranges {
		kind := DeriveDefinitionKind(r.Kind)
		definitions[i] = Definition{
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
func HydrateSectionsWithDefinitions(sections []Section, definitions []Definition) []Section {
	if len(sections) == 0 {
		return sections
	}
	newSections := make([]Section, len(sections))
	for i := range sections {
		newSections[i] = sections[i]
		start := newSections[i].StartLine
		end := newSections[i].EndLine

		var subset []Definition
		for _, def := range definitions {
			if def.StartLine >= start && def.EndLine <= end {
				subset = append(subset, def)
			}
		}

		newSections[i].Children = HydrateSectionsWithDefinitions(newSections[i].Children, subset)

		var myDefs []Definition
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
