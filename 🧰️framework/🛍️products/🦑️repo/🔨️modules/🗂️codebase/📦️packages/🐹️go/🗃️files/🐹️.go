// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗃️files carries the codebase folder and file builders.

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

// #region 🗃️Files

// 🔶️BuildCodebaseFolders MUST assemble the codebase folders from the available context data.
// 📦️BuildCodebaseFolders constructs and returns the codebase folders structure.
func BuildCodebaseFolders(ctx *CodebaseContext) []model.CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	breachCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		absPath := filepath.Join(workspace.GetRootDir(), file)
		if content, err := workspace.ReadTextFile(absPath); err == nil {
			lineCounts[folder] += strings.Count(content, "\n") + 1
		}
	}

	for _, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			folder := workspace.NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				breachCounts[folder]++
			}
		}
	}

	var result []model.CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		result = append(result, model.CodebaseFolder{
			ID:       id,
			Path:     folder,
			URI:      ctx.FileURI(folder),
			Name:     filepath.Base(folder),
			ParentID: ctx.ParentFolderID(folder),
			BundleID: ctx.OwningBundleID(folder),
			Metrics: &model.FolderMetricsInternal{
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
func BuildCodebaseFiles(ctx *CodebaseContext) []model.CodebaseFile {
	var result []model.CodebaseFile
	breachsByFile := make(map[string][]model.Breach)

	for _, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			breachsByFile[filePath] = append(breachsByFile[filePath], v)
		}
	}

	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)

		var metrics *model.FileMetricsInternal
		absPath := filepath.Join(workspace.GetRootDir(), file)
		if content, err := workspace.ReadTextFile(absPath); err == nil {
			sections := languages.ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := languages.GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &model.FileMetricsInternal{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}

		var breachs []model.FileBreachRef
		for _, v := range breachsByFile[file] {
			info := v.Kind.Info()
			breachs = append(breachs, model.FileBreachRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}

		result = append(result, model.CodebaseFile{
			ID:       id,
			Path:     file,
			URI:      ctx.FileURI(file),
			FolderID: ctx.ParentFolderID(file),
			BundleID: ctx.OwningBundleID(file),
			Metrics:  metrics,
			Breachs:  breachs,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// 🟧️BuildCodebaseFoldersForFiles MUST assemble the codebase folders for files from the available context data.
// 🟩️BuildCodebaseFoldersForFiles constructs and returns the codebase folders for files structure.
func BuildCodebaseFoldersForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err == nil {
			lineCounts[folder] += workspace.CountLines(content)
		}
	}

	var result []model.CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		parent := filepath.Dir(folder)
		var parentID *string
		if parent != "." && parent != "" {
			parentValue := parent
			parentID = &parentValue
		}
		result = append(result, model.CodebaseFolder{
			ID:       id,
			Path:     folder,
			URI:      ctx.FolderURI(folder),
			Name:     filepath.Base(folder),
			ParentID: parentID,
			Metrics: &model.FolderMetricsInternal{
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
func BuildCodebaseFilesForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseFile {
	var result []model.CodebaseFile
	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)
		var metrics *model.FileMetricsInternal
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err == nil {
			sections := languages.ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := languages.GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &model.FileMetricsInternal{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}
		result = append(result, model.CodebaseFile{
			ID:      id,
			Path:    file,
			URI:     ctx.FileURI(file),
			Metrics: metrics,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

// #endregion 🗃️Files
