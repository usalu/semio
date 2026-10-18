// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Codebase bundle, folder, file, section, and definition builders.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"sort"
	"strings"
)

// 🏗️BuildCodebaseBundles MUST assemble the codebase bundles from the available context data.
// 🧱️BuildCodebaseBundles constructs and returns the codebase bundles structure.
func BuildCodebaseBundles(ctx *CodebaseContext) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})
	contributorSets := make(map[string]map[string]struct{})
	ticketSets := make(map[string]map[string]struct{})
	breachCounts := make(map[string]int)

	for _, bundle := range ctx.Bundles {
		name := normalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
		contributorSets[name] = make(map[string]struct{})
		ticketSets[name] = make(map[string]struct{})
	}

	if _, ok := folderSets["repo/repo"]; !ok {
		folderSets["repo/repo"] = make(map[string]struct{})
		contributorSets["repo/repo"] = make(map[string]struct{})
		ticketSets["repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[bundleName] += strings.Count(content, "\n") + 1
			sections := ParseSections(content, file)
			sectionCounts[bundleName] += countSections(sections)
			lang := GetLanguage(file)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				definitionCounts[bundleName] += len(defs)
			}
			headerSection := FindSection(sections, "Header")
			if headerSection != nil {
				headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
				for _, line := range strings.Split(headerContent, "\n") {
					if name, email, ok := ParseContributorIdentity(line); ok {
						_ = name
						contributorSets[bundleName][email] = struct{}{}
					}
				}
			}
		}
	}

	for _, v := range ctx.Breachs {
		bundleName := ctx.GetBundleForFile(v.Scope)
		if bundleName != "" {
			breachCounts[bundleName]++
		}
	}

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		interactionFiles := ticket.GetInteractionFiles()
		if len(interactionFiles) > 0 {
			for _, entry := range interactionFiles {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		var contributors []string
		for c := range contributorSets[name] {
			contributors = append(contributors, c)
		}
		sort.Strings(contributors)

		var tickets []string
		for t := range ticketSets[name] {
			tickets = append(tickets, t)
		}
		sort.Strings(tickets)

		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if normalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, CodebaseBundle{
			ID:           name,
			Folder:       bundleRoot,
			URI:          ctx.FileURI(bundleRoot),
			Contributors: contributors,
			Tickets:      tickets,
			Metrics: &BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
				Breachs:     breachCounts[name],
			},
		})
	}
	return result
}

// 📑️countSections holds the data fields for a countSections record.
func countSections(sections []Section) int {
	count := len(sections)
	for _, s := range sections {
		count += countSections(s.Children)
	}
	return count
}

// 🔶️BuildCodebaseFolders MUST assemble the codebase folders from the available context data.
// 📦️BuildCodebaseFolders constructs and returns the codebase folders structure.
func BuildCodebaseFolders(ctx *CodebaseContext) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	breachCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[folder] += strings.Count(content, "\n") + 1
		}
	}

	for _, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				breachCounts[folder]++
			}
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		result = append(result, CodebaseFolder{
			ID:   id,
			Path: folder,
			URI:  ctx.FileURI(folder),
			Metrics: &FolderMetricsInternal{
				Files:   fileCounts[folder],
				Lines:   lineCounts[folder],
				Breachs: breachCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🧲️extractFilePath holds the data fields for a extractFilePath record.
func extractFilePath(scope string) string {
	scope = strings.Split(scope, "#")[0]
	scope = strings.Split(scope, "§")[0]
	return scope
}

// 🔹️BuildCodebaseFiles MUST assemble the codebase files from the available context data.
// 🔸️BuildCodebaseFiles constructs and returns the codebase files structure.
func BuildCodebaseFiles(ctx *CodebaseContext) []CodebaseFile {
	var result []CodebaseFile
	breachsByFile := make(map[string][]Breach)

	for _, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			breachsByFile[filePath] = append(breachsByFile[filePath], v)
		}
	}

	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)

		var metrics *FileMetricsInternal
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			sections := ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &FileMetricsInternal{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}

		var breachs []FileBreachRef
		for _, v := range breachsByFile[file] {
			info := v.Kind.Info()
			breachs = append(breachs, FileBreachRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}

		result = append(result, CodebaseFile{
			ID:      id,
			Path:    file,
			URI:     ctx.FileURI(file),
			Metrics: metrics,
			Breachs: breachs,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🔸️BuildCodebaseSections MUST assemble the codebase sections from the available context data.
// 🔺️BuildCodebaseSections constructs and returns the codebase sections structure.
func BuildCodebaseSections(ctx *CodebaseContext) []CodebaseSection {
	var result []CodebaseSection

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)

		fileID := ctx.GetFileID(file)

		addSections(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// ➕️addSections holds the data fields for a addSections record.
func addSections(ctx *CodebaseContext, result *[]CodebaseSection, file, fileID, content string, sections []Section, parentPath string) {
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
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &SectionMetricsInternal{
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
func BuildCodebaseDefinitions(ctx *CodebaseContext) []CodebaseDefinition {
	var result []CodebaseDefinition

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := ParseSections(content, file)
		fileID := ctx.GetFileID(file)

		for _, def := range defs {
			sectionPath := findSectionForDefinition(sections, def.Start, def.End, "")
			id := ""
			if sectionPath != "" {
				id = fileID + "#" + sectionPath + "§" + def.Name
			} else {
				id = fileID + "§" + def.Name
			}
			result = append(result, CodebaseDefinition{
				ID:   id,
				Path: id,
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &DefinitionMetricsInternal{
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
