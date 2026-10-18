// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Codebase context and file-scoped builders.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"sort"
	"strings"
)

// 💿️CodebaseContext holds the data fields for a codebase context record.
type CodebaseContext struct {
	RootDir  string
	RootURI  string
	Bundles  []Bundle
	Files    []string
	Breachs  []Breach
	Tickets  []Ticket
	Policies []PolicyDef
}

// 📝️NewCodebaseContext MUST initialize all required fields and return a valid CodebaseContext.
// 🆕️NewCodebaseContext creates and returns a new CodebaseContext instance.
func NewCodebaseContext() *CodebaseContext {
	rootURI := "file://" + NormalizePath(rootDir)
	return &CodebaseContext{
		RootDir: rootDir,
		RootURI: rootURI,
	}
}

// ⚙️LoadBundles MUST read from the configured storage path.
// 📖️LoadBundles loads the bundles from storage.
func (ctx *CodebaseContext) LoadBundles() {
	ctx.Bundles = GetTechnologies()
}

// 📄️LoadFiles MUST read from the configured storage path.
// 📄️LoadFiles loads the files from storage.
func (ctx *CodebaseContext) LoadFiles() error {
	files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, ctx.Bundles)
	if err != nil {
		return err
	}
	ctx.Files = files
	return nil
}

// 🛤️LoadBreachs MUST read from the configured storage path.
// ⚠️LoadBreachs loads the breachs from storage.
func (ctx *CodebaseContext) LoadBreachs() error {
	for _, file := range ctx.Files {
		breachs, err := AnalyzeFile(file, ctx.Bundles)
		if err != nil {
			continue
		}
		ctx.Breachs = append(ctx.Breachs, breachs...)
	}
	return nil
}

// 🎫️LoadTickets MUST read from the configured storage path.
// 🔷️LoadTickets loads the tickets from storage.
func (ctx *CodebaseContext) LoadTickets() error {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return err
	}
	ctx.Tickets = tickets
	return nil
}

// 📖️LoadPolicies MUST read from the configured storage path.
// 🔶️LoadPolicies loads the policies from storage.
func (ctx *CodebaseContext) LoadPolicies() {
	ctx.Policies = GetPolicies()
}

// 📦️GetBundleForFile MUST return the stored value without modification.
// 🔤️GetBundleForFile returns the bundle for file of the CodebaseContext.
func (ctx *CodebaseContext) GetBundleForFile(filePath string) string {
	name, _, ok := ctx.GetBundleInfo(filePath)
	if !ok {
		return "repo/repo"
	}
	return name
}

// 🏪️GetBundleInfo MUST return the stored value without modification.
// ℹGetBundleInfo returns the bundle info of the CodebaseContext.
func (ctx *CodebaseContext) GetBundleInfo(path string) (name, root string, ok bool) {
	normalizedPath := NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range ctx.Bundles {
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return normalizeBundleLabel(matchedBundle), matchedRoot, true
	}
	return "", "", false
}

// 🔷️GetFileID MUST return the stored value without modification.
// 🔑️GetFileID returns the file i d of the CodebaseContext.
func (ctx *CodebaseContext) GetFileID(file string) string {
	return buildFileID(file, nil)
}

// 📁️GetFolderID MUST return the stored value without modification.
// 🔹️GetFolderID returns the folder i d of the CodebaseContext.
func (ctx *CodebaseContext) GetFolderID(folder string) string {
	return buildFolderID(folder, nil)
}

// 📥️FileURI MUST operate on the CodebaseContext receiver and return consistent results.
func (ctx *CodebaseContext) FileURI(path string) string {
	return buildFileUriFromPath(NormalizePath(path))
}

// 🔗️FolderURI MUST operate on the CodebaseContext receiver and return consistent results.
func (ctx *CodebaseContext) FolderURI(path string) string {
	return buildFileUriFromPath(NormalizePath(path))
}

// 🟥️BuildCodebaseBundlesForFiles MUST assemble the codebase bundles for files from the available context data.
// 🟨️BuildCodebaseBundlesForFiles constructs and returns the codebase bundles for files structure.
func BuildCodebaseBundlesForFiles(ctx *CodebaseContext, checkpoint string) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})

	for _, bundle := range ctx.Bundles {
		name := normalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
	}

	if _, ok := folderSets["repo/repo"]; !ok {
		folderSets["repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		content, err := ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		lineCounts[bundleName] += CountLines(content)
		sections := ParseSections(content, file)
		sectionCounts[bundleName] += countSections(sections)
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			definitionCounts[bundleName] += len(defs)
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if normalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, CodebaseBundle{
			ID:     name,
			Folder: bundleRoot,
			URI:    ctx.FileURI(bundleRoot),
			Metrics: &BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
			},
		})
	}
	return result
}

// 🟧️BuildCodebaseFoldersForFiles MUST assemble the codebase folders for files from the available context data.
// 🟩️BuildCodebaseFoldersForFiles constructs and returns the codebase folders for files structure.
func BuildCodebaseFoldersForFiles(ctx *CodebaseContext, checkpoint string) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		content, err := ReadTextFileAtCheckpoint(checkpoint, file)
		if err == nil {
			lineCounts[folder] += CountLines(content)
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		parent := filepath.Dir(folder)
		var parentID *string
		if parent != "." && parent != "" {
			parentValue := parent
			parentID = &parentValue
		}
		result = append(result, CodebaseFolder{
			ID:       id,
			Path:     folder,
			URI:      ctx.FolderURI(folder),
			Name:     filepath.Base(folder),
			ParentID: parentID,
			Metrics: &FolderMetricsInternal{
				Files: fileCounts[folder],
				Lines: lineCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟨️BuildCodebaseFilesForFiles MUST assemble the codebase files for files from the available context data.
// 🟪️BuildCodebaseFilesForFiles constructs and returns the codebase files for files structure.
func BuildCodebaseFilesForFiles(ctx *CodebaseContext, checkpoint string) []CodebaseFile {
	var result []CodebaseFile
	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)
		var metrics *FileMetricsInternal
		content, err := ReadTextFileAtCheckpoint(checkpoint, file)
		if err == nil {
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
		result = append(result, CodebaseFile{
			ID:      id,
			Path:    file,
			URI:     ctx.FileURI(file),
			Metrics: metrics,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟩️BuildCodebaseSectionsForFiles MUST assemble the codebase sections for files from the available context data.
// 💠️BuildCodebaseSectionsForFiles constructs and returns the codebase sections for files structure.
func BuildCodebaseSectionsForFiles(ctx *CodebaseContext, checkpoint string) []CodebaseSection {
	var result []CodebaseSection
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)
		fileID := ctx.GetFileID(file)
		addSectionsForContent(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟦️addSectionsForContent holds the data fields for a addSectionsForContent record.
func addSectionsForContent(ctx *CodebaseContext, result *[]CodebaseSection, file, fileID, content string, sections []Section, parentPath string) {
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
			},
		})
		addSectionsForContent(ctx, result, file, fileID, content, section.Children, sectionPath)
	}
}

// 🟪️BuildCodebaseDefinitionsForFiles MUST assemble the codebase definitions for files from the available context data.
// 🔳️BuildCodebaseDefinitionsForFiles constructs and returns the codebase definitions for files structure.
func BuildCodebaseDefinitionsForFiles(ctx *CodebaseContext, checkpoint string) []CodebaseDefinition {
	var result []CodebaseDefinition
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCheckpoint(checkpoint, file)
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
					Lines: def.End - def.Start + 1,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟫️ToolCodebase MUST complete the operation and return consistent results.
func ToolCodebase() ToolResult {
	output := NewOutput()
	ctx := NewCodebaseContext()

	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return toolErrorResult(err)
	}
	if err := ctx.LoadBreachs(); err != nil {
		return toolErrorResult(err)
	}
	if err := ctx.LoadTickets(); err != nil {
		return toolErrorResult(err)
	}
	ctx.LoadPolicies()

	codebase := BuildCodebase(ctx)

	output.Success(fmt.Sprintf("Codebase loaded: %d bundles, %d files, %d breachs",
		len(codebase.Bundles), len(codebase.Files), len(codebase.Breachs)))

	return ToolResult{Output: *output, Data: codebase}
}
