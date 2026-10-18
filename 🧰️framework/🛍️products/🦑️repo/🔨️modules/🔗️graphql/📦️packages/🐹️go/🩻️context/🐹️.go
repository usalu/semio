// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩻️context carries the filesystem backed repository context and its read queries.

// #endregion 🧲️Header

package graphql

import (
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	goalspkg "github.com/usalu/semio/repo/goals"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🩻️Repo Context

// 💿️repoContext holds the data fields for a repoContext record.
type RepoContext struct {
	rootDir            string
	bundles            []model.Bundle
	managementProvider providers.ManagementProvider
}

// 🔶️NewRepoContext MUST initialize all required fields and return a valid repo context.
// 🐙️NewRepoContext creates and returns a new repo context instance.
func NewRepoContext(rootDir string) model.RepoContext {
	resolvedRoot := rootDir
	if strings.TrimSpace(resolvedRoot) == "" {
		resolvedRoot = workspace.FindRepoRoot("")
	}
	workspace.SetRootDir(resolvedRoot)
	ctx := &RepoContext{rootDir: resolvedRoot, managementProvider: providers.DefaultManagementProvider()}
	ctx.bundles = codebase.LoadBundles()
	return ctx
}

// 🔹️ManagementProv holds the data fields for a ManagementProv record.
func (c *RepoContext) ManagementProv() providers.ManagementProvider { return c.managementProvider }

// 📨️GetRootDir MUST retrieve the requested value or return an error.
// 🟨️GetRootDir retrieves and returns the root dir.
func (c *RepoContext) GetRootDir() string { return c.rootDir }

// ❌️GetFileID MUST retrieve the requested value or return an error.
// 🔑️GetFileID retrieves and returns the file i d.
func (c *RepoContext) GetFileID(path string) string {
	ctx := &codebase.CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFileID(path)
}

// 📁️GetFolderID MUST retrieve the requested value or return an error.
// 🟦️GetFolderID retrieves and returns the folder i d.
func (c *RepoContext) GetFolderID(path string) string {
	ctx := &codebase.CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFolderID(path)
}

// 📦️GetBundles MUST retrieve the requested value or return an error.
// 🟪️GetBundles retrieves and returns the bundles.
func (c *RepoContext) GetBundles() []*model.Bundle {
	result := make([]*model.Bundle, len(c.bundles))
	for i := range c.bundles {
		result[i] = &c.bundles[i]
	}
	return result
}

// 📜️GetTechnologies MUST retrieve the requested value or return an error.
// 📺️GetTechnologies retrieves and returns the technologies.
func (c *RepoContext) GetTechnologies() []*model.Technology {
	technologies := codebase.LoadTechnologies()
	res := make([]*model.Technology, len(technologies))
	for i := range technologies {
		res[i] = &technologies[i]
	}
	return res
}

// 💾️GetCheckpoints MUST retrieve the requested value or return an error.
// ✔️GetCheckpoints retrieves and returns the checkpoints.
func (c *RepoContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	checkpoints := contributorspkg.LoadCheckpoints(limit)
	res := make([]*model.Checkpoint, len(checkpoints))
	for i := range checkpoints {
		res[i] = &checkpoints[i]
	}
	return res, nil
}

// 🔸️GetFolders MUST retrieve the requested value or return an error.
// 🟫️GetFolders retrieves and returns the folders.
func (c *RepoContext) GetFolders() []*model.Folder {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Folder{}
	}
	folders := codebase.BuildCodebaseFolders(ctx)
	results := make([]*model.Folder, 0, len(folders))
	for _, entry := range folders {
		results = append(results, &model.Folder{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      entry.Name,
			ParentID:  entry.ParentID,
			BundleID:  entry.BundleID,
			Kind:      model.DeriveFolderKind(entry.Path),
			Generated: model.IsGeneratedFolder(entry.Path),
		})
	}
	return results
}

// 📄️GetFiles MUST retrieve the requested value or return an error.
// 💠️GetFiles retrieves and returns the files.
func (c *RepoContext) GetFiles() []*model.File {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.File{}
	}
	files := codebase.BuildCodebaseFiles(ctx)
	results := make([]*model.File, 0, len(files))
	for _, entry := range files {
		name := filepath.Base(entry.Path)
		results = append(results, &model.File{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      name,
			Extension: filepath.Ext(entry.Path),
			FolderID:  entry.FolderID,
			BundleID:  entry.BundleID,
			Kind:      model.DeriveFileKind(name),
		})
	}
	return results
}

// 📖️GetDefinitions MUST retrieve the requested value or return an error.
// 🔳️GetDefinitions retrieves and returns the definitions.
func (c *RepoContext) GetDefinitions() []*model.Definition {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Definition{}
	}
	var results []*model.Definition
	for _, file := range ctx.Files {
		tool := move.ToolDefinitionList(file)
		definitions, ok := tool.Data.([]model.Definition)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range definitions {
			def := definitions[i]

			def.ID = codebase.DefinitionID(fileID, def)
			results = append(results, &def)
		}
	}
	return results
}

// 📑️GetSections MUST retrieve the requested value or return an error.
// 🔲️GetSections retrieves and returns the sections.
func (c *RepoContext) GetSections() []*model.Section {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Section{}
	}
	var results []*model.Section
	for _, file := range ctx.Files {
		tool := move.ToolSectionList(file)
		sections, ok := tool.Data.([]model.Section)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range sections {
			sec := sections[i]

			sec.FilePath = file
			sec.ID = model.BuildSectionID(fileID, languages.NormalizeSectionPath(sec.Path))
			results = append(results, &sec)
		}
	}
	return results
}

// 🤝️GetContributors MUST retrieve the requested value or return an error.
// ▪️GetContributors retrieves and returns the contributors.
func (c *RepoContext) GetContributors() ([]*model.Contributor, error) {
	contributors := contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(c.rootDir))
	result := make([]*model.Contributor, len(contributors))
	for i := range contributors {
		result[i] = &contributors[i]
	}
	return result, nil
}

// 🎫️GetTickets MUST retrieve the requested value or return an error.
// ▫️GetTickets retrieves and returns the tickets.
func (c *RepoContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	tickets, err := ticketspkg.ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*model.Ticket
	for i := range tickets {
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, &tickets[i])
		}
	}
	return result, nil
}

// ⛳️GetGoals MUST retrieve the requested value or return an error.
// ◾GetGoals retrieves and returns the goals.
func (c *RepoContext) GetGoals() ([]*model.Goal, error) {
	return goalspkg.ListGoals()
}

// #endregion 🩻️Repo Context
