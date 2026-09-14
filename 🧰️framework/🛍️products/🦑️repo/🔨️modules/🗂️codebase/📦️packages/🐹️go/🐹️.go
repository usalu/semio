// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️codebase is the codebase domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package codebase

import (
	json "encoding/json"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strings "strings"
	sync "sync"
	time "time"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
	yaml "github.com/usalu/semio/repo/yaml"
)

// #region 🚚️Split

// #region 🏩️Codebase

// 💿️CodebaseContext holds the data fields for a codebase context record.
type CodebaseContext struct {
	RootDir  string
	RootURI  string
	Bundles  []model.Bundle
	Files    []string
	Breachs  []model.Breach
	Tickets  []model.Ticket
	Policies []statutes.PolicyDef
}

// 📝️NewCodebaseContext MUST initialize all required fields and return a valid CodebaseContext.
// 🆕️NewCodebaseContext creates and returns a new CodebaseContext instance.
func NewCodebaseContext() *CodebaseContext {
	rootURI := "file://" + workspace.NormalizePath(workspace.RootDir)
	return &CodebaseContext{
		RootDir: workspace.RootDir,
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
	files, err := model.ScopeToFiles(workspace.Scope{Kind: workspace.ScopeRepo}, ctx.Bundles)
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
		breachs, err := statutes.AnalyzeFile(file, ctx.Bundles)
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
	tickets, err := LookupCodebaseTickets()
	if err != nil {
		return err
	}
	ctx.Tickets = tickets
	return nil
}

// 📖️LoadPolicies MUST read from the configured storage path.
// 🔶️LoadPolicies loads the policies from storage.
func (ctx *CodebaseContext) LoadPolicies() {
	ctx.Policies = statutes.GetPolicies()
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
	normalizedPath := workspace.NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range ctx.Bundles {
		root := workspace.NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return model.NormalizeBundleLabel(matchedBundle), matchedRoot, true
	}
	return "", "", false
}

// 🔷️GetFileID MUST return the stored value without modification.
// 🔑️GetFileID returns the file i d of the CodebaseContext.
func (ctx *CodebaseContext) GetFileID(file string) string {
	return model.BuildFileID(file, nil)
}

// 📁️GetFolderID MUST return the stored value without modification.
// 🔹️GetFolderID returns the folder i d of the CodebaseContext.
func (ctx *CodebaseContext) GetFolderID(folder string) string {
	return model.BuildFolderID(folder, nil)
}

// 📁️ParentFolderID MUST return the artifact id of the folder holding the path, nil at the root.
func (ctx *CodebaseContext) ParentFolderID(path string) *string {
	parent := workspace.NormalizePath(filepath.Dir(path))
	if parent == "." || parent == "" {
		return nil
	}
	id := ctx.GetFolderID(parent)
	return &id
}

// 📦️OwningBundleID MUST return the artifact id of the deepest bundle owning the path, nil for none.
func (ctx *CodebaseContext) OwningBundleID(path string) *string {
	bundle := model.GetBundleByPath(workspace.NormalizePath(path))
	if bundle == nil {
		return nil
	}
	id := bundle.GetID()
	return &id
}

// 📥️FileURI MUST operate on the CodebaseContext receiver and return consistent results.
func (ctx *CodebaseContext) FileURI(path string) string {
	return model.BuildFileUriFromPath(workspace.NormalizePath(path))
}

// 🔗️FolderURI MUST operate on the CodebaseContext receiver and return consistent results.
func (ctx *CodebaseContext) FolderURI(path string) string {
	return model.BuildFileUriFromPath(workspace.NormalizePath(path))
}

// 🏗️BuildCodebaseBundles MUST assemble the codebase bundles from the available context data.
// 🧱️BuildCodebaseBundles constructs and returns the codebase bundles structure.
func BuildCodebaseBundles(ctx *CodebaseContext) []model.CodebaseBundle {
	var result []model.CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})
	contributorSets := make(map[string]map[string]struct{})
	ticketSets := make(map[string]map[string]struct{})
	breachCounts := make(map[string]int)

	for _, bundle := range ctx.Bundles {
		name := model.NormalizeBundleLabel(bundle.Name)
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
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		absPath := filepath.Join(workspace.RootDir, file)
		if content, err := workspace.ReadTextFile(absPath); err == nil {
			lineCounts[bundleName] += strings.Count(content, "\n") + 1
			sections := languages.ParseSections(content, file)
			sectionCounts[bundleName] += countSections(sections)
			lang := languages.GetLanguage(file)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				definitionCounts[bundleName] += len(defs)
			}
			headerSection := languages.FindSection(sections, "Header")
			if headerSection != nil {
				headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
				for _, line := range strings.Split(headerContent, "\n") {
					if name, email, ok := model.ParseContributorIdentity(line); ok {
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
			if model.NormalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, model.CodebaseBundle{
			ID:           name,
			Folder:       bundleRoot,
			URI:          ctx.FileURI(bundleRoot),
			Contributors: contributors,
			Tickets:      tickets,
			Metrics: &model.BundleMetricsInternal{
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
func countSections(sections []model.Section) int {
	count := len(sections)
	for _, s := range sections {
		count += countSections(s.Children)
	}
	return count
}

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
		absPath := filepath.Join(workspace.RootDir, file)
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
		absPath := filepath.Join(workspace.RootDir, file)
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

// 🗃️BuildCodebaseTickets MUST assemble the codebase tickets from the available context data.
// 🗺️BuildCodebaseTickets constructs and returns the codebase tickets structure.
func BuildCodebaseTickets(ctx *CodebaseContext) []model.CodebaseTicket {
	var result []model.CodebaseTicket

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		ticketPath := ticket.FolderPath
		if ticketPath == "" {
			ticketPath = ".🧬semio/🦑️repo/🎫️tickets/" + model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		}

		bundleFiles := make(map[string]int)
		var paths []string
		for _, f := range ticket.GetInteractionFiles() {
			paths = append(paths, f.Path)
		}
		for _, path := range paths {
			bundleName := ctx.GetBundleForFile(path)
			if bundleName != "" {
				bundleFiles[bundleName]++
			}
		}
		var bundleContribs []model.TicketBundleContribInfo
		for bundleName, fileCount := range bundleFiles {
			bundleContribs = append(bundleContribs, model.TicketBundleContribInfo{
				ID: bundleName,
				Metrics: &model.CountMetrics{
					Added: fileCount,
				},
			})
		}

		llm := ticket.GetLLM()

		var finishedStr string
		if f := ticket.GetDateFinished(); f != nil {
			finishedStr = f.Format(time.RFC3339)
		}

		result = append(result, model.CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &model.TicketDateInfo{
				Created:  ticket.GetDateStarted().Format(time.RFC3339),
				Finished: finishedStr,
			},
			Checkpoint: ticket.GetCheckpoint(),
			Year:       fmt.Sprintf("%04d", ticket.Year),
			Month:      fmt.Sprintf("%02d", ticket.Month),
			Day:        fmt.Sprintf("%02d", ticket.Day),
			Slug:       ticket.Slug,
			Prompt:     ticket.GetPrompt(),
			LLM:        llm,
			Author:     ticket.GetAuthor(),
			Status:     ticket.GetStatus(),
			Bundles:    bundleContribs,
		})
	}
	return result
}

// 🔻️BuildCodebasePolicies MUST assemble the codebase policies from the available context data.
// ⬛️BuildCodebasePolicies constructs and returns the codebase policies structure.
func BuildCodebasePolicies(ctx *CodebaseContext) []model.CodebasePolicy {
	var result []model.CodebasePolicy
	breachsByPolicy := make(map[string][]model.Breach)

	for _, v := range ctx.Breachs {
		parts := strings.Split(string(v.Kind), "/")
		if len(parts) > 0 {
			policyID := parts[0]
			breachsByPolicy[policyID] = append(breachsByPolicy[policyID], v)
		}
	}

	for _, policy := range ctx.Policies {
		var breachs []model.PolicyBreachRef
		for _, v := range breachsByPolicy[policy.ID] {
			info := v.Kind.Info()
			breachs = append(breachs, model.PolicyBreachRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}
		result = append(result, model.CodebasePolicy{
			ID:      policy.ID,
			Name:    policy.Name,
			Scopes:  policy.Scopes,
			Breachs: breachs,
		})
	}
	return result
}

// ⬛️BuildCodebaseBreachs MUST assemble the codebase breachs from the available context data.
// ⬜️BuildCodebaseBreachs constructs and returns the codebase breachs structure.
func BuildCodebaseBreachs(ctx *CodebaseContext) []model.CodebaseBreach {
	var result []model.CodebaseBreach

	for i, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		bundleName := ctx.GetBundleForFile(filePath)
		info := v.Kind.Info()

		breachID := fmt.Sprintf("%s#|%s|%s#%d", v.Kind, bundleName, filePath, i)

		var folders []model.BreachFolder
		if filePath != "" {
			folder := workspace.NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				folderID := folder
				if bundleName != "" {
					folderID = bundleName + "/" + folder
				}
				folders = append(folders, model.BreachFolder{
					ID:   folderID,
					Path: folder,
					URI:  ctx.FolderURI(folder),
				})
			}
		}

		var files []model.BreachFile
		if filePath != "" {
			fileID := filePath
			if bundleName != "" {
				fileID = bundleName + "/" + filepath.Base(filePath)
			}
			files = append(files, model.BreachFile{
				ID:   fileID,
				Path: filePath,
				URI:  ctx.FileURI(filePath),
				Range: &model.FileRange{
					Start: model.RangePosition{Line: v.Line, Column: v.Column},
					End:   model.RangePosition{Line: v.Line, Column: v.Column},
				},
			})
		}

		result = append(result, model.CodebaseBreach{
			ID:          breachID,
			Folders:     folders,
			Files:       files,
			Kind:        v.Kind,
			Priority:    info.Priority,
			Autofixable: info.Autofixable,
			Reason:      info.Reason,
			Solution:    info.Solution,
		})
	}
	return result
}

// 🌳️BuildCodebaseTree MUST assemble the codebase tree from the available context data.
// 🌳️BuildCodebaseTree constructs and returns the codebase tree structure.
func BuildCodebaseTree(ctx *CodebaseContext, bundles []model.CodebaseBundle, files []model.CodebaseFile, sections []model.CodebaseSection, definitions []model.CodebaseDefinition) map[string]*model.CbTreeNode {
	tree := make(map[string]*model.CbTreeNode)
	tree["compose"] = &model.CbTreeNode{Kind: model.CbTreeNodeRepo, Children: make(map[string]*model.CbTreeNode)}
	root := tree["compose"]

	for _, bundle := range bundles {
		root.Children[bundle.ID] = &model.CbTreeNode{Kind: model.CbTreeNodeBundle, Children: make(map[string]*model.CbTreeNode)}
	}

	folderNodes := make(map[string]*model.CbTreeNode)
	for _, file := range files {
		bundleName := ctx.GetBundleForFile(file.Path)
		var parent *model.CbTreeNode
		if bundleName != "" {
			parent = root.Children[bundleName]
		} else {
			parent = root
		}
		folder := workspace.NormalizePath(filepath.Dir(file.Path))
		if folder != "." {
			parts := strings.Split(folder, "/")
			for i, part := range parts {
				folderPath := strings.Join(parts[:i+1], "/")
				if _, ok := folderNodes[folderPath]; !ok {
					folderNode := &model.CbTreeNode{Kind: model.CbTreeNodeFolder, Children: make(map[string]*model.CbTreeNode)}
					if i == 0 {
						parent.Children[part] = folderNode
					} else {
						parentPath := strings.Join(parts[:i], "/")
						folderNodes[parentPath].Children[part] = folderNode
					}
					folderNodes[folderPath] = folderNode
				}
			}
			fileNode := &model.CbTreeNode{Kind: model.CbTreeNodeFile, Children: make(map[string]*model.CbTreeNode)}
			folderNodes[folder].Children[file.ID] = fileNode
		} else {
			fileNode := &model.CbTreeNode{Kind: model.CbTreeNodeFile, Children: make(map[string]*model.CbTreeNode)}
			parent.Children[file.ID] = fileNode
		}
	}

	return tree
}

// ⬜️BuildCodebase MUST assemble the codebase from the available context data.
// 🟥️BuildCodebase constructs and returns the codebase structure.
func BuildCodebase(ctx *CodebaseContext) *model.Codebase {
	bundles := BuildCodebaseBundles(ctx)
	folders := BuildCodebaseFolders(ctx)
	files := BuildCodebaseFiles(ctx)
	sections := BuildCodebaseSections(ctx)
	definitions := BuildCodebaseDefinitions(ctx)
	contributors := LookupCodebaseContributors(ctx)
	tickets := BuildCodebaseTickets(ctx)
	policies := BuildCodebasePolicies(ctx)
	breachs := BuildCodebaseBreachs(ctx)
	tree := BuildCodebaseTree(ctx, bundles, files, sections, definitions)

	return &model.Codebase{
		Bundles:      bundles,
		Folders:      folders,
		Files:        files,
		Sections:     sections,
		Definitions:  definitions,
		Contributors: contributors,
		Tickets:      tickets,
		Policies:     policies,
		Breachs:      breachs,
		Tree:         tree,
	}
}

// 📸️BuildCodebaseSnapshot MUST assemble the codebase snapshot from the available context data.
// 🟧️BuildCodebaseSnapshot constructs and returns the codebase snapshot structure.
func BuildCodebaseSnapshot(files []string, bundles []model.Bundle, checkpoint string) (*model.Codebase, error) {
	ctx := &CodebaseContext{RootDir: workspace.RootDir, RootURI: "file://" + workspace.NormalizePath(workspace.RootDir)}
	ctx.Bundles = bundles
	ctx.Files = files
	ctx.Policies = statutes.GetPolicies()
	codebase := &model.Codebase{}
	codebase.Bundles = BuildCodebaseBundlesForFiles(ctx, checkpoint)
	codebase.Folders = BuildCodebaseFoldersForFiles(ctx, checkpoint)
	codebase.Files = BuildCodebaseFilesForFiles(ctx, checkpoint)
	codebase.Sections = BuildCodebaseSectionsForFiles(ctx, checkpoint)
	codebase.Definitions = BuildCodebaseDefinitionsForFiles(ctx, checkpoint)
	codebase.Tree = BuildCodebaseTree(ctx, codebase.Bundles, codebase.Files, codebase.Sections, codebase.Definitions)
	return codebase, nil
}

// 🟥️BuildCodebaseBundlesForFiles MUST assemble the codebase bundles for files from the available context data.
// 🟨️BuildCodebaseBundlesForFiles constructs and returns the codebase bundles for files structure.
func BuildCodebaseBundlesForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseBundle {
	var result []model.CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})

	for _, bundle := range ctx.Bundles {
		name := model.NormalizeBundleLabel(bundle.Name)
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
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		lineCounts[bundleName] += workspace.CountLines(content)
		sections := languages.ParseSections(content, file)
		sectionCounts[bundleName] += countSections(sections)
		lang := languages.GetLanguage(file)
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
			if model.NormalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, model.CodebaseBundle{
			ID:     name,
			Folder: bundleRoot,
			URI:    ctx.FileURI(bundleRoot),
			Metrics: &model.BundleMetricsInternal{
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

// #endregion 🏩️Codebase

// #region 📋️Tickets

var technologyCache []model.Technology

var technologyCacheLoaded bool

var technologyCacheMutex sync.Mutex

// 📜️InvalidateTechnologyCache MUST clear the cached state to force a reload.
// ✔️InvalidateTechnologyCache invalidates the cached technology cache.
func InvalidateTechnologyCache() {
	technologyCacheMutex.Lock()
	defer technologyCacheMutex.Unlock()
	technologyCacheLoaded = false
	technologyCache = nil
}

// ◾LoadTechnologies MUST return all matching technologies from the data source.
// 📺️LoadTechnologies loads and returns technologies from the data source.
func LoadTechnologies() []model.Technology {
	technologyCacheMutex.Lock()
	defer technologyCacheMutex.Unlock()
	if technologyCacheLoaded {
		return technologyCache
	}
	technologyCache = loadTechnologiesInternal()
	technologyCacheLoaded = true
	return technologyCache
}

// ◽isTechnologyDir holds the data fields for a isTechnologyDir record.
func isTechnologyDir(name string) bool {
	readmePath := filepath.Join(workspace.RootDir, name, "README.md")
	content, err := workspace.ReadTextFile(readmePath)
	if err != nil || !strings.HasPrefix(content, "---") {
		return false
	}
	if strings.HasPrefix(name, ".") {
		return false
	}
	if name == "node_modules" || name == "target" || name == "temp" {
		return false
	}
	return true
}

// ◻loadTechnologiesInternal holds the data fields for a loadTechnologiesInternal record.
func loadTechnologiesInternal() []model.Technology {
	var technologies []model.Technology
	technologiesDir := workspace.RootDir
	entries, err := os.ReadDir(technologiesDir)
	if err != nil {
		return nil
	}

	for _, d := range entries {
		if !d.IsDir() {
			continue
		}
		name := d.Name()
		if !isTechnologyDir(name) {
			continue
		}

		rawName := strings.TrimPrefix(name, "@")

		technologyName := rawName
		technologyKind := model.DeriveTechnologyKind(rawName)
		readmePath := filepath.Join(workspace.RootDir, name, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil && strings.HasPrefix(content, "---") {
			endIdx := strings.Index(content[3:], "---")
			if endIdx > 0 {
				fmContent := content[3 : 3+endIdx]
				var fm struct {
					Name string `yaml:"name"`
					Kind string `yaml:"kind"`
				}
				if yaml.Unmarshal([]byte(fmContent), &fm) == nil {
					if fm.Name != "" {
						technologyName = fm.Name
					}
					if fm.Kind != "" {
						switch strings.ToLower(fm.Kind) {
						case "user":
							technologyKind = model.TechnologyKindUser
						case "infrastructure":
							technologyKind = model.TechnologyKindInfrastructure
						case "research":
							technologyKind = model.TechnologyKindResearch
						}
					}
				}
			}
		}

		technology := model.Technology{
			Name:    technologyName,
			Root:    name,
			Kind:    technologyKind,
			Bundles: []model.Bundle{},
		}

		// Read technology emoji from AGENTS.md frontmatter
		agentsPath := filepath.Join(workspace.RootDir, name, "AGENTS.md")
		if content, err := workspace.ReadTextFile(agentsPath); err == nil && strings.HasPrefix(content, "---") {
			endIdx := strings.Index(content[3:], "---")
			if endIdx > 0 {
				fmContent := content[3 : 3+endIdx]
				var agentsFm struct {
					Emoji string `yaml:"emoji"`
				}
				if yaml.Unmarshal([]byte(fmContent), &agentsFm) == nil && agentsFm.Emoji != "" {
					technology.Emoji = agentsFm.Emoji
				}
			}
		}

		technologyPath := filepath.Join(technologiesDir, name)
		subEntries, _ := os.ReadDir(technologyPath)
		for _, sub := range subEntries {
			if !sub.IsDir() {
				continue
			}
			if strings.HasPrefix(sub.Name(), ".") {
				continue
			}
			if sub.Name() == "node_modules" {
				continue
			}
			bunName := sub.Name()
			if bunName == "sites" {
				siteEntries, _ := os.ReadDir(filepath.Join(technologyPath, bunName))
				for _, site := range siteEntries {
					if !site.IsDir() {
						continue
					}
					if strings.HasPrefix(site.Name(), ".") || site.Name() == "node_modules" {
						continue
					}
					siteName := site.Name()
					fullBundleName := technologyName + "/" + siteName
					bundlePath := filepath.ToSlash(filepath.Join(name, bunName, siteName))
					bundle := model.Bundle{
						Name:           fullBundleName,
						Root:           bundlePath,
						TechnologyName: technologyName,
						Kind:           model.BundleKindSite,
					}
					// Read site bundle emoji from AGENTS.md frontmatter
					siteAgentsPath := filepath.Join(workspace.RootDir, bundlePath, "AGENTS.md")
					if content, err := workspace.ReadTextFile(siteAgentsPath); err == nil && strings.HasPrefix(content, "---") {
						endIdx := strings.Index(content[3:], "---")
						if endIdx > 0 {
							fmContent := content[3 : 3+endIdx]
							var siteFm struct {
								Bundle struct {
									Emoji string `yaml:"emoji"`
								} `yaml:"bundle"`
							}
							if yaml.Unmarshal([]byte(fmContent), &siteFm) == nil && siteFm.Bundle.Emoji != "" {
								bundle.Emoji = siteFm.Bundle.Emoji
							}
						}
					}
					bundle.Packages = loadPackages(filepath.Join(workspace.RootDir, bundlePath))
					configPath := filepath.Join(technologyPath, bunName, siteName, "project.json")
					if !workspace.FileExists(configPath) {
						configPath = filepath.Join(technologyPath, bunName, siteName, "package.json")
					}
					if workspace.FileExists(configPath) {
						content, err := workspace.ReadTextFile(configPath)
						if err == nil {
							var meta struct {
								SourceRoot string   `json:"sourceRoot"`
								Tags       []string `json:"tags"`
							}
							if json.Unmarshal([]byte(content), &meta) == nil {
								bundle.SourceRoot = meta.SourceRoot
								bundle.Tags = meta.Tags
							}
						}
					}
					technology.Bundles = append(technology.Bundles, bundle)
				}
				continue
			}
			fullBundleName := technologyName + "/" + bunName

			bundlePath := filepath.ToSlash(filepath.Join(name, bunName))
			kind := model.DeriveBundleKind(fullBundleName, bundlePath)

			bundle := model.Bundle{
				Name:           fullBundleName,
				Root:           bundlePath,
				TechnologyName: technologyName,
				Kind:           kind,
			}

			// Read bundle emoji from AGENTS.md frontmatter
			bundleAgentsPath := filepath.Join(workspace.RootDir, bundlePath, "AGENTS.md")
			if content, err := workspace.ReadTextFile(bundleAgentsPath); err == nil && strings.HasPrefix(content, "---") {
				endIdx := strings.Index(content[3:], "---")
				if endIdx > 0 {
					fmContent := content[3 : 3+endIdx]
					var bundleFm struct {
						Bundle struct {
							Emoji string `yaml:"emoji"`
						} `yaml:"bundle"`
					}
					if yaml.Unmarshal([]byte(fmContent), &bundleFm) == nil && bundleFm.Bundle.Emoji != "" {
						bundle.Emoji = bundleFm.Bundle.Emoji
					}
				}
			}

			bundle.Packages = loadPackages(filepath.Join(workspace.RootDir, bundlePath))

			configPath := filepath.Join(technologyPath, bunName, "project.json")
			if !workspace.FileExists(configPath) {
				configPath = filepath.Join(technologyPath, bunName, "package.json")
			}
			if workspace.FileExists(configPath) {
				content, err := workspace.ReadTextFile(configPath)
				if err == nil {
					var meta struct {
						SourceRoot string   `json:"sourceRoot"`
						Tags       []string `json:"tags"`
					}
					if json.Unmarshal([]byte(content), &meta) == nil {
						bundle.SourceRoot = meta.SourceRoot
						bundle.Tags = meta.Tags
					}
				}
			}
			technology.Bundles = append(technology.Bundles, bundle)
		}
		technologies = append(technologies, technology)
	}

	return technologies
}

// 📦️LoadBundles MUST return all matching bundles from the data source.
// 🗄️LoadBundles loads and returns bundles from the data source.
func LoadBundles() []model.Bundle {
	var bundles []model.Bundle
	technologies := LoadTechnologies()
	for _, p := range technologies {
		bundles = append(bundles, p.Bundles...)
	}
	return bundles
}

// 📨️GetTechnologies MUST retrieve the requested value or return an error.
// 🔺️GetTechnologies retrieves and returns the technologies.
func GetTechnologies() []model.Bundle {
	return LoadBundles()
}

// 🟠️loadPackages holds the data fields for a loadPackages record.
func loadPackages(bundleRoot string) []model.Package {
	var packages []model.Package
	filepath.WalkDir(bundleRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}

		if d.IsDir() {
			name := d.Name()
			if name == "node_modules" || name == ".git" || name == "bin" || name == "obj" || name == "target" || name == "dist" || name == "build" || name == "__pycache__" || strings.HasPrefix(name, ".") {
				return fs.SkipDir
			}
			return nil
		}

		rel, _ := filepath.Rel(bundleRoot, path)

		name := ""
		version := ""
		kind := ""

		filename := d.Name()
		switch filename {
		case "package.json":
			kind = "npm"
			if content, err := os.ReadFile(path); err == nil {
				var meta struct {
					Name    string `json:"name"`
					Version string `json:"version"`
				}
				json.Unmarshal(content, &meta)
				name = meta.Name
				version = meta.Version
			}
		case "go.mod":
			kind = "go"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				if len(lines) > 0 && strings.HasPrefix(lines[0], "module ") {
					name = strings.TrimSpace(strings.TrimPrefix(lines[0], "module "))
				}
			}
		case "Cargo.toml":
			kind = "cargo"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		case "pyproject.toml":
			kind = "pip"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		}

		if strings.HasSuffix(filename, ".csproj") {
			kind = "nuget"
			name = strings.TrimSuffix(filename, ".csproj")
		}

		if kind != "" && name != "" {
			packages = append(packages, model.Package{
				Name:    name,
				Version: version,
				Path:    rel,
				Kind:    kind,
			})
		}
		return nil
	})
	return packages
}

// 🔖️FileHeaderId MUST complete the operation successfully.
func FileHeaderId(path string) string {
	kind := model.DeriveFileKind(filepath.Base(path))
	if kind == model.FileKindCode {
		absPath := filepath.Join(workspace.RootDir, path)
		if content, err := os.ReadFile(absPath); err == nil {
			text := string(content)
			if strings.HasPrefix(text, "#!") || strings.HasPrefix(text, "\xEF\xBB\xBF#!") {
				kind = model.FileKindScript
			}
		}
	}
	normalized := workspace.NormalizePath(path)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		parentID = model.ResolveParentIDFromPath(dir)
	}
	data := map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID}
	return model.GetArtifactID("file", data)
}

// 🔗️FileHeaderUri MUST return the repo URI for a file path.
// ◾FileHeaderUri returns the artifact URI for the given file path.
func FileHeaderUri(path string) string {
	data := map[string]interface{}{"path": path}
	return model.GetArtifactURI("file", data)
}

// #endregion 📋️Tickets

// #region 🧬️Missing Utilities

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

// ⬛️GetFolderChildren MUST retrieve the requested value or return an error.
// ⬛️GetFolderChildren retrieves and returns the folder children.
func GetFolderChildren(folderPath string, bundleID *string) ([]*model.Folder, error) {
	absPath := filepath.Join(workspace.RootDir, folderPath)
	entries, err := os.ReadDir(absPath)
	if err != nil {
		return []*model.Folder{}, nil
	}
	type candidate struct {
		name    string
		relPath string
	}
	var candidates []candidate
	var relPaths []string
	for _, entry := range entries {
		if entry.IsDir() {
			if strings.HasPrefix(entry.Name(), ".") && entry.Name() != ".🧬semio" {
				continue
			}
			if entry.Name() == "node_modules" || entry.Name() == "bin" || entry.Name() == "obj" {
				continue
			}
			relPath := workspace.NormalizePath(filepath.Join(folderPath, entry.Name()))
			candidates = append(candidates, candidate{name: entry.Name(), relPath: relPath})
			relPaths = append(relPaths, relPath)
		}
	}
	ignored := workspace.GetGitIgnoredSet(relPaths)
	var children []*model.Folder
	for _, c := range candidates {
		if ignored[c.relPath] || ignored[c.relPath+"/"] {
			continue
		}
		child := &model.Folder{
			ID:       model.BuildFolderID(c.relPath, bundleID),
			Path:     c.relPath,
			URI:      fmt.Sprintf("file://%s/%s", workspace.RootDir, c.relPath),
			Name:     c.name,
			BundleID: bundleID,
		}
		children = append(children, child)
	}
	return children, nil
}

// ⬜️GetFolderFiles MUST retrieve the requested value or return an error.
// ⬜️GetFolderFiles retrieves and returns the folder files.
func GetFolderFiles(folderPath string, bundleID *string) ([]*model.File, error) {
	absPath := filepath.Join(workspace.RootDir, folderPath)
	entries, err := os.ReadDir(absPath)
	if err != nil {
		return []*model.File{}, nil
	}
	var filePaths []string
	for _, entry := range entries {
		if !entry.IsDir() {
			if strings.HasPrefix(entry.Name(), ".") {
				continue
			}
			relPath := filepath.Join(folderPath, entry.Name())
			filePaths = append(filePaths, relPath)
		}
	}
	filePaths = model.FilterConsideredFiles(filePaths)
	filePaths = model.FilterGitIgnored(filePaths)
	var files []*model.File
	var folderID *string
	if folderPath != "." {
		id := model.BuildFolderID(folderPath, bundleID)
		folderID = &id
	}
	for _, relPath := range filePaths {
		files = append(files, &model.File{
			ID:        model.BuildFileID(relPath, bundleID),
			Path:      relPath,
			URI:       fmt.Sprintf("file://%s/%s", workspace.RootDir, relPath),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
		})
	}
	return files, nil
}

// #endregion 🧬️Missing Utilities

// #region 🔊️Cli

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

// #endregion 🔊️Cli

// #region 🦀️Hooks

func CollectTestDefinitionsFromContent(content string, normalized string) []model.Definition {
	definitions := languages.ParseDefinitions(content, normalized)
	var tests []model.Definition
	seen := map[string]bool{}
	for i := range definitions {
		if definitions[i].Kind == "" && statutes.IsTestOrBenchmarkFile(normalized) {
			definitions[i].Kind = model.DefinitionKindTest
		}
		if definitions[i].Kind != model.DefinitionKindTest {
			continue
		}
		id := definitions[i].GetID()
		if id == "" || seen[id] {
			continue
		}
		seen[id] = true
		tests = append(tests, definitions[i])
	}
	if len(tests) > 0 {
		return tests
	}
	return fallbackTestDefinitionsFromContent(content, normalized)
}

func fallbackTestDefinitionsFromContent(content string, normalized string) []model.Definition {
	patterns := []*regexp.Regexp{
		regexp.MustCompile(`(?m)^[\t ]*func[\t ]+(Test[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*def[\t ]+(test_[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*(?:it|test)\s*\(?\s*['"]([^'"]+)['"]`),
		regexp.MustCompile(`(?m)^[\t ]*public[\t ]+function[\t ]+(test[A-Za-z0-9_]+)\s*\(`),
	}
	var tests []model.Definition
	seen := map[string]bool{}
	for _, pattern := range patterns {
		matches := pattern.FindAllStringSubmatch(content, -1)
		for _, match := range matches {
			if len(match) < 2 {
				continue
			}
			name := strings.TrimSpace(match[1])
			if name == "" {
				continue
			}
			definition := model.Definition{Name: name, Kind: model.DefinitionKindTest, FilePath: normalized}
			id := definition.GetID()
			if id == "" || seen[id] {
				continue
			}
			seen[id] = true
			tests = append(tests, definition)
		}
	}
	return tests
}

func TestSelectorMatchesDefinitionName(selector string, definitionName string) bool {
	selector = strings.TrimSpace(selector)
	selector = strings.Trim(selector, `"'`)
	if selector == "" || definitionName == "" {
		return false
	}
	selectorLower := strings.ToLower(selector)
	definitionLower := strings.ToLower(definitionName)
	if selectorLower == definitionLower || strings.Contains(definitionLower, selectorLower) {
		return true
	}
	pattern, err := regexp.Compile(selector)
	if err == nil && pattern.MatchString(definitionName) {
		return true
	}
	quoted, quoteErr := regexp.Compile(regexp.QuoteMeta(selector))
	return quoteErr == nil && quoted.MatchString(definitionName)
}

// #endregion 🦀️Hooks

// #region 🧱️Artifact ID

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

// #endregion 🧱️Artifact ID

// #region 💾️Missing Utility Functions

// 🛤️resolvePathToFileID resolves a file path to a file ID.
func ResolvePathToFileID(path string) string {
	normalized := workspace.NormalizeHookPath(path)
	if normalized == "" {
		return ""
	}
	return model.BuildFileID(normalized, nil)
}

func ResolveTestNamesToDefinitionIDs(testFiles []string, testNames []string) map[string]string {
	result := make(map[string]string)
	if len(testFiles) == 0 || len(testNames) == 0 {
		return result
	}
	for _, file := range testFiles {
		normalized := workspace.NormalizeHookPath(file)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(workspace.GetRootDir(), normalized)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		definitions := CollectTestDefinitionsFromContent(string(content), normalized)
		for i := range definitions {
			definitionID := definitions[i].GetID()
			if definitionID == "" {
				continue
			}
			for _, name := range testNames {
				if _, ok := result[name]; ok {
					continue
				}
				if TestSelectorMatchesDefinitionName(name, definitions[i].Name) {
					result[name] = definitionID
				}
			}
		}
		if len(result) == len(testNames) {
			return result
		}
	}
	return result
}

// #endregion 💾️Missing Utility Functions

// #region 🔌️Codebase Ports

// 🎫️LookupCodebaseTickets answers every ticket for a codebase snapshot. 🎫️tickets installs it.
var LookupCodebaseTickets = func() ([]model.Ticket, error) { return nil, nil }

// 🧑️LookupCodebaseContributors builds the contributor half of a codebase snapshot.
var LookupCodebaseContributors = func(ctx *CodebaseContext) []model.CodebaseContributor { return nil }

// 🔌️init wires the walks 📐️model and 🏠️workspace declare but cannot implement.
func init() {
	model.LookupBundles = GetTechnologies
	model.LookupTechnologies = LoadTechnologies
	workspace.WatchRootDir(InvalidateTechnologyCache)
}

// #endregion 🔌️Codebase Ports

// #endregion 🚚️Split
