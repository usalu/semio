// 🐹️ Go side of the workspace walk case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Fixture

type treeEntry struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type repoTree struct {
	Schema  string      `json:"schema"`
	Entries []treeEntry `json:"entries"`
}

// materializeTree writes the committed tree into a private directory, points the repository root at
// it and drops every cache the global-root Go implementation keeps.
func materializeTree(ctx *host.Context) (string, error) {
	data, err := ctx.FixtureBytes("shared://📡️repo-tree.json")
	if err != nil {
		return "", err
	}
	var tree repoTree
	if err := json.Unmarshal(data, &tree); err != nil {
		return "", err
	}
	root := filepath.Join(ctx.WorkDir, "🌳️tree")
	if err := os.RemoveAll(root); err != nil {
		return "", err
	}
	for _, entry := range tree.Entries {
		path := filepath.Join(root, filepath.FromSlash(entry.Path))
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			return "", err
		}
		if err := os.WriteFile(path, []byte(entry.Content), 0o644); err != nil {
			return "", err
		}
	}
	workspace.SetRootDir(root)
	codebase.InvalidateTechnologyCache()
	return root, nil
}

// endregion 🔖️Fixture

// region 🔖️Scenarios

func walkReportsTheSameConsideredFiles(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	context := codebase.NewCodebaseContext()
	context.LoadBundles()
	if err := context.LoadFiles(); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"files": context.Files}}, nil
}

func walkProjectsTheSameBundleAndFolderAggregates(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	context := codebase.NewCodebaseContext()
	context.LoadBundles()
	if err := context.LoadFiles(); err != nil {
		return host.Outcome{}, err
	}
	bundles := []string{}
	for _, bundle := range codebase.BuildCodebaseBundles(context) {
		bundles = append(bundles, fmt.Sprintf("%s|%s|%d|%d|%d", bundle.ID, bundle.Folder, bundle.Metrics.Folders, bundle.Metrics.Files, bundle.Metrics.Lines))
	}
	folders := []string{}
	for _, folder := range codebase.BuildCodebaseFolders(context) {
		folders = append(folders, fmt.Sprintf("%s|%s|%d|%d", folder.Path, folder.ID, folder.Metrics.Files, folder.Metrics.Lines))
	}
	return host.Outcome{Projection: map[string]any{"bundles": bundles, "folders": folders}}, nil
}

func walkProjectsTheSameFileAndFolderRecords(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	context := codebase.NewCodebaseContext()
	context.LoadBundles()
	if err := context.LoadFiles(); err != nil {
		return host.Outcome{}, err
	}
	folders := []string{}
	for _, folder := range codebase.BuildCodebaseFolders(context) {
		folders = append(folders, fmt.Sprintf("%s|%s|%s|%s|%t", folder.ID, folder.Path, folder.Name, model.DeriveFolderKind(folder.Path), model.IsGeneratedFolder(folder.Path)))
	}
	files := []string{}
	for _, file := range codebase.BuildCodebaseFiles(context) {
		name := filepath.Base(file.Path)
		files = append(files, fmt.Sprintf("%s|%s|%s|%s|%s", file.ID, file.Path, name, filepath.Ext(name), model.DeriveFileKind(name)))
	}
	return host.Outcome{Projection: map[string]any{"folders": folders, "files": files}}, nil
}

func walkProjectsTheSameDefinitions(ctx *host.Context) (host.Outcome, error) {
	root, err := materializeTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	context := codebase.NewCodebaseContext()
	context.LoadBundles()
	if err := context.LoadFiles(); err != nil {
		return host.Outcome{}, err
	}
	definitions := []string{}
	for _, file := range context.Files {
		content, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(file)))
		if err != nil {
			continue
		}
		fileID := context.GetFileID(file)
		for _, definition := range codebase.FileDefinitions(string(content), file) {
			id := codebase.DefinitionID(fileID, definition)
			definitions = append(definitions, fmt.Sprintf("%s|%s|%s|%s|%s|%s|%d|%d", id, definition.Name, definition.Kind, definition.FilePath, definition.SectionPath, definition.Emoji, definition.StartLine, definition.EndLine))
		}
	}
	return host.Outcome{Projection: map[string]any{"definitions": definitions}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("walk-reports-the-same-considered-files", walkReportsTheSameConsideredFiles).
		Subject("walk-projects-the-same-bundle-and-folder-aggregates", walkProjectsTheSameBundleAndFolderAggregates).
		Subject("walk-projects-the-same-file-and-folder-records", walkProjectsTheSameFileAndFolderRecords).
		Subject("walk-projects-the-same-definitions", walkProjectsTheSameDefinitions)
}

// endregion 🔖️Registration
