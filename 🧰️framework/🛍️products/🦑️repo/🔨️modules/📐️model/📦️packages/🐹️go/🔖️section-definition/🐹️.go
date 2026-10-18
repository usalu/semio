// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔖️section_definition carries the section and definition shapes with the definition kinds.

// #endregion 🧲️Header

package model

import (
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔖️Section Definition

// 📖️DefinitionKind represents a definition kind value.
type DefinitionKind string

const DefinitionKindImplementation DefinitionKind = "implementation"

const DefinitionKindInterface DefinitionKind = "interface"

const DefinitionKindConstant DefinitionKind = "constant"

const DefinitionKindTest DefinitionKind = "test"

// 🔷️IsValid MUST return true only when the condition is met.
// ▶️IsValid reports whether the DefinitionKind is valid.
func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindImplementation, DefinitionKindInterface, DefinitionKindConstant, DefinitionKindTest:
		return true
	}
	return false
}

// 🔤️String MUST return the canonical string value.
// 🔤️String returns the string representation of the DefinitionKind.
func (e DefinitionKind) String() string {
	return string(e)
}

// 📝️DeriveDefinitionKind MUST return a valid value for any recognized input.
// 💾️DeriveDefinitionKind infers and returns the definition kind from the given input.
func DeriveDefinitionKind(rawKind string) DefinitionKind {
	switch strings.ToLower(rawKind) {
	case "interface", "type", "trait", "abstract",
		"extend type", "extend interface", "extend enum", "extend union", "extend input",
		"input", "union", "scalar",
		"delegate", "record":
		return DefinitionKindInterface
	case "const", "constant", "enum", "var", "let", "static":
		return DefinitionKindConstant
	case "test":
		return DefinitionKindTest
	default:
		return DefinitionKindImplementation
	}
}

// 💗️Section holds the data fields for a section record.
type Section struct {
	ID          string       `json:"id,omitempty"`
	Name        string       `json:"name"`
	Path        string       `json:"path,omitempty"`
	FilePath    string       `json:"filePath,omitempty"`
	Emoji       string       `json:"emoji"`
	StartLine   int          `json:"startLine"`
	EndLine     int          `json:"endLine"`
	StartIndex  int          `json:"startIndex"`
	EndIndex    int          `json:"endIndex"`
	Children    []Section    `json:"children,omitempty"`
	Definitions []Definition `json:"definitions,omitempty"`
}

// 💖️IsNode MUST return true only when the condition is met.
// 🧡️IsNode reports whether the Section is node.
func (s *Section) IsNode() {}

// 💝️GetID MUST return the stored value without modification.
// ❤️GetID returns the i d of the Section.
func (s *Section) GetID() string {
	if s.FilePath != "" {
		fileID := BuildFileID(s.FilePath, nil)
		if s.Path != "" {
			return BuildSectionID(fileID, strings.Split(strings.ReplaceAll(s.Path, "#", "/"), "/"))
		}
		segment := s.Name
		if s.Emoji != "" {
			segment = s.Emoji + s.Name
		}
		return BuildSectionID(fileID, []string{segment})
	}
	emoji := s.Emoji
	if emoji == "" {
		emoji = EmojiSection
	}
	return EmojiText(emoji) + workspace.Flat(s.Name)
}

// 💘️GetURI MUST return the stored value without modification.
// 🤍️GetURI returns the u r i of the Section.
func (s *Section) GetURI() string {
	return "repo://section/" + s.GetID()
}

// 💕️Definition holds the data fields for a definition record.
type Definition struct {
	ID          string         `json:"id,omitempty"`
	Name        string         `json:"name"`
	Kind        DefinitionKind `json:"kind"`
	FilePath    string         `json:"filePath,omitempty"`
	SectionPath string         `json:"sectionPath,omitempty"`
	Emoji       string         `json:"emoji"`
	StartLine   int            `json:"startLine"`
	EndLine     int            `json:"endLine"`
	StartIndex  int            `json:"startIndex"`
	EndIndex    int            `json:"endIndex"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🖤️IsNode reports whether the Definition is node.
func (d *Definition) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🤎️GetID returns the i d of the Definition.
func (d *Definition) GetID() string {
	if d.FilePath != "" {
		fileID := BuildFileID(d.FilePath, nil)
		var sectionParts []string
		if d.SectionPath != "" {
			sectionParts = strings.Split(strings.ReplaceAll(d.SectionPath, "#", "/"), "/")
		}
		return BuildDefinitionID(fileID, sectionParts, d.Name, d.Kind)
	}
	data := map[string]interface{}{"kind": string(d.Kind)}
	return definitionKindEmoji(data) + workspace.Flat(d.Name)
}

// 🔖️GetURI MUST return the stored value without modification.
// 💖️GetURI returns the u r i of the Definition.
func (d *Definition) GetURI() string {
	return "repo://definition/" + d.GetID()
}

// #endregion 🔖️Section Definition
