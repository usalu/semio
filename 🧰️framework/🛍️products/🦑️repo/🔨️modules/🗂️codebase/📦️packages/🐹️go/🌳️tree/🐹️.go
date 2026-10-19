// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌳️tree carries the codebase tree, the full build, the snapshot and the folder children queries.

// #endregion 🧲️Header

package codebase

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌳️Tree

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
	ctx := &CodebaseContext{RootDir: workspace.GetRootDir(), RootURI: "file://" + workspace.NormalizePath(workspace.GetRootDir())}
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

// ⬛️GetFolderChildren MUST retrieve the requested value or return an error.
// ⬛️GetFolderChildren retrieves and returns the folder children.
func GetFolderChildren(folderPath string, bundleID *string) ([]*model.Folder, error) {
	absPath := filepath.Join(workspace.GetRootDir(), folderPath)
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
			URI:      fmt.Sprintf("file://%s/%s", workspace.GetRootDir(), c.relPath),
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
	absPath := filepath.Join(workspace.GetRootDir(), folderPath)
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
			URI:       fmt.Sprintf("file://%s/%s", workspace.GetRootDir(), relPath),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
		})
	}
	return files, nil
}

// #endregion 🌳️Tree
