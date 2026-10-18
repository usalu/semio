// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️codebase is the codebase domain of the semio repository tooling, split out of the pre-split godfile. This file carries the codebase context, its path and identifier resolvers and the lookup ports.

// #endregion 🧲️Header

package codebase

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

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

// #endregion 🏩️Codebase
