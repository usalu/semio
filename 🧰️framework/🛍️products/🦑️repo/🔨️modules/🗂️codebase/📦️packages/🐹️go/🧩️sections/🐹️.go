// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧩️sections carries the codebase section and definition builders and lookups.

// #endregion 🧲️Header

package codebase

import (
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧩️Sections

// 🔸️BuildCodebaseSections MUST assemble the codebase sections from the available context data.
// 🔺️BuildCodebaseSections constructs and returns the codebase sections structure.
func BuildCodebaseSections(ctx *CodebaseContext) []model.CodebaseSection {
	var result []model.CodebaseSection

	for _, file := range ctx.Files {
		absPath := filepath.Join(workspace.RootDir, file)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := languages.ParseSections(content, file)

		fileID := ctx.GetFileID(file)

		addSections(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// ➕️addSections holds the data fields for a addSections record.
func addSections(ctx *CodebaseContext, result *[]model.CodebaseSection, file, fileID, content string, sections []model.Section, parentPath string) {
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		id := fileID + "#" + sectionPath
		sectionContent := ""
		if section.StartIndex < len(content) && section.EndIndex <= len(content) {
			sectionContent = content[section.StartIndex:section.EndIndex]
		}
		defCount := 0
		lang := languages.GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, model.CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &model.SectionMetricsInternal{
				Definitions: defCount,
				Lines:       section.EndLine - section.StartLine + 1,
				Breachs:     0,
			},
		})
		addSections(ctx, result, file, fileID, content, section.Children, sectionPath)
	}
}

// 🔺️BuildCodebaseDefinitions MUST assemble the codebase definitions from the available context data.
// ▶️BuildCodebaseDefinitions constructs and returns the codebase definitions structure.
func BuildCodebaseDefinitions(ctx *CodebaseContext) []model.CodebaseDefinition {
	var result []model.CodebaseDefinition

	for _, file := range ctx.Files {
		absPath := filepath.Join(workspace.RootDir, file)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := languages.ParseSections(content, file)
		fileID := ctx.GetFileID(file)

		for _, def := range defs {
			sectionPath := FindSectionForDefinition(sections, def.Start, def.End, "")
			id := ""
			if sectionPath != "" {
				id = fileID + "#" + sectionPath + "§" + def.Name
			} else {
				id = fileID + "§" + def.Name
			}
			result = append(result, model.CodebaseDefinition{
				ID:   id,
				Path: id,
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &model.DefinitionMetricsInternal{
					Definitions: 0,
					Lines:       def.End - def.Start + 1,
					Breachs:     0,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟩️BuildCodebaseSectionsForFiles MUST assemble the codebase sections for files from the available context data.
// 💠️BuildCodebaseSectionsForFiles constructs and returns the codebase sections for files structure.
func BuildCodebaseSectionsForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseSection {
	var result []model.CodebaseSection
	for _, file := range ctx.Files {
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		sections := languages.ParseSections(content, file)
		fileID := ctx.GetFileID(file)
		addSectionsForContent(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟦️addSectionsForContent holds the data fields for a addSectionsForContent record.
func addSectionsForContent(ctx *CodebaseContext, result *[]model.CodebaseSection, file, fileID, content string, sections []model.Section, parentPath string) {
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		id := fileID + "#" + sectionPath
		sectionContent := ""
		if section.StartIndex < len(content) && section.EndIndex <= len(content) {
			sectionContent = content[section.StartIndex:section.EndIndex]
		}
		defCount := 0
		lang := languages.GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, model.CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &model.SectionMetricsInternal{
				Definitions: defCount,
				Lines:       section.EndLine - section.StartLine + 1,
			},
		})
		addSectionsForContent(ctx, result, file, fileID, content, section.Children, sectionPath)
	}
}

// 🟪️BuildCodebaseDefinitionsForFiles MUST assemble the codebase definitions for files from the available context data.
// 🔳️BuildCodebaseDefinitionsForFiles constructs and returns the codebase definitions for files structure.
func BuildCodebaseDefinitionsForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseDefinition {
	var result []model.CodebaseDefinition
	for _, file := range ctx.Files {
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := languages.ParseSections(content, file)
		fileID := ctx.GetFileID(file)
		for _, def := range defs {
			sectionPath := FindSectionForDefinition(sections, def.Start, def.End, "")
			id := ""
			if sectionPath != "" {
				id = fileID + "#" + sectionPath + "§" + def.Name
			} else {
				id = fileID + "§" + def.Name
			}
			result = append(result, model.CodebaseDefinition{
				ID:   id,
				Path: id,
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &model.DefinitionMetricsInternal{
					Lines: def.End - def.Start + 1,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🗺️computeSectionLineMap holds the data fields for a computeSectionLineMap record.
func ComputeSectionLineMap(sections []model.Section, diffLines []int, parentPath string) map[string][]int {
	result := map[string][]int{}
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		linesInSection := ComputeLinesInRange(diffLines, section.StartLine, section.EndLine)
		childLines := []int{}
		for _, child := range section.Children {
			childLines = append(childLines, ComputeLinesInRange(diffLines, child.StartLine, child.EndLine)...)
		}
		exclusiveLines := workspace.SetDifference(linesInSection, childLines)
		if len(exclusiveLines) > 0 {
			result[sectionPath] = append(result[sectionPath], exclusiveLines...)
		}
		if len(section.Children) > 0 {
			for key, value := range ComputeSectionLineMap(section.Children, diffLines, sectionPath) {
				result[key] = append(result[key], value...)
			}
		}
	}
	return result
}

func ComputeLinesInRange(changedLines []int, startLine, endLine int) []int {
	var result []int
	for _, line := range changedLines {
		if line >= startLine && line <= endLine {
			result = append(result, line)
		}
	}
	return result
}

func FindSectionForDefinition(sections []model.Section, startLine, endLine int, prefix string) string {
	for _, s := range sections {
		if startLine >= s.StartLine && endLine <= s.EndLine {
			p := s.Name
			if prefix != "" {
				p = prefix + "#" + s.Name
			}
			if len(s.Children) > 0 {
				if cp := FindSectionForDefinition(s.Children, startLine, endLine, p); cp != "" {
					return cp
				}
			}
			return p
		}
	}
	return prefix
}

// 🪨️DefinitionID MUST return the same identity for the same definition in the same file.
// 🪨️DefinitionID returns the identity one definition of a file is addressed by, built from the file,
// the section that encloses it, its kind and its name.
func DefinitionID(fileID string, definition model.Definition) string {
	return model.BuildDefinitionID(fileID, languages.NormalizeSectionPath(definition.SectionPath), definition.Name, definition.Kind)
}

// 📖️FileDefinitions MUST answer every definition of one file with every addressable field filled:
// the section that encloses it, its kind emoji and the file it belongs to.
//
// [languages.ParseDefinitions] answers names, kinds and line ranges only; which section a definition
// sits in is a property of the whole file, so it is resolved once here and every caller — the
// `definition list` tool, the GraphQL aggregate, the export snapshot — reads the same record.
func FileDefinitions(content string, filePath string) []model.Definition {
	parsed := languages.ParseDefinitions(content, filePath)
	if len(parsed) == 0 {
		return []model.Definition{}
	}
	sections := languages.ParseSections(content, filePath)
	definitions := make([]model.Definition, 0, len(parsed))
	for _, definition := range parsed {
		definition.FilePath = filePath
		definition.SectionPath = FindSectionForDefinition(sections, definition.StartLine, definition.EndLine, "")
		definition.Emoji = model.DefinitionKindEmoji(definition.Kind)
		definitions = append(definitions, definition)
	}
	return definitions
}

// 🎯️FindSectionBySlug MUST return the matching result or an error if not found.
// 🧩️FindSectionBySlug locates and returns the matching section by slug.
func FindSectionBySlug(sections []model.Section, slug string) *model.Section {
	for i := range sections {
		if workspace.Slugify(sections[i].Name) == slug {
			return &sections[i]
		}
		if found := FindSectionBySlug(sections[i].Children, slug); found != nil {
			return found
		}
	}
	return nil
}

// 🟫️ResolveSectionName MUST return the resolved value or an error if unresolvable.
// 📩️ResolveSectionName resolves and returns the section name.
func ResolveSectionName(filePath string, slug string) string {
	absPath := filepath.Join(workspace.RootDir, filePath)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return model.UnSlugify(slug)
	}
	lang := languages.GetLanguage(filePath)
	if lang == nil || !lang.SupportsSections() {
		return model.UnSlugify(slug)
	}
	sections := lang.ParseSections(content)
	section := FindSectionBySlug(sections, slug)
	if section != nil {
		return section.Name
	}
	return model.UnSlugify(slug)
}

// #endregion 🧩️Sections
