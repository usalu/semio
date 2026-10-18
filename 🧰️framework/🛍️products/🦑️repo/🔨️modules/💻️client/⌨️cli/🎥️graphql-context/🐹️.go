// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo context port and reads.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"strings"
)

// 🔌️RepoContext defines the interface for repo context operations.
type RepoContext interface {
	GetRootDir() string
	GetBundles() []*Bundle
	GetTechnologies() []*Technology
	GetCheckpoints(limit *int) ([]*Checkpoint, error)
	GetFolders() []*Folder
	GetFiles() []*File
	GetSections() []*Section
	GetDefinitions() []*Definition
	GetContributors() ([]*Contributor, error)
	GetGoals() ([]*Goal, error)
	GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error)
	GetPolicies() []*Policy
	GetDrafts() ([]*Draft, error)
	GetTodos(filter *FilterInput) ([]*Todo, error)
	GetStatutes() []*StatuteMeta
	Analyze(scope *string) (*AnalyzeResult, error)
	Fix(scope *string) (*FixResult, error)
	GoalCreate(input GoalCreateInput) (*Goal, error)
	GoalChange(input GoalChangeInput) (*Goal, error)
	GoalClose(input GoalCloseInput) (*Goal, error)
	GoalReopen(input GoalReopenInput) (*Goal, error)
	GoalDelete(input GoalDeleteInput) (bool, error)
	TodoCreate(input TodoCreateInput) (*Todo, error)
	TodoChange(input TodoChangeInput) (*Todo, error)
	TodoDelete(id string) (bool, error)
	DraftCreate(input DraftCreateInput) (*Draft, error)
	DraftDelete(id string) (bool, error)
	TicketOpen(input TicketOpenInput) (*Ticket, error)
	TicketClose(input TicketCloseInput) (*Ticket, error)
	TicketReopen(input TicketReopenInput) (*Ticket, error)
	TicketChange(input TicketChangeInput) (*Ticket, error)
	TicketDelete(input TicketDeleteInput) (bool, error)
	FolderCreate(path string) (*Folder, error)
	FolderMove(src, dst string) (*Folder, error)
	FolderDelete(path string) error
	FileCreate(path string) (*File, error)
	FileMove(src, dst string) (*File, error)
	FileDelete(path string) error
	SectionCreate(file, name string, parent *string) (*Section, error)
	SectionMove(file, oldName, newName string) (*Section, error)
	SectionDelete(file, name string) error
	Integrate(source, targetSection, targetFile, targetParent *string) (*File, error)
	Extract(sourceFile, sourceSection, targetFile *string) (*File, error)
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
	SyncManagement() (bool, error)
}

// 📝️defaultContext holds the data fields for a defaultContext record.
type defaultContext struct {
	rootDir string
}

// 🔷️NewDefaultContext MUST initialize all required fields and return a valid default context.
// 🟧️NewDefaultContext creates and returns a new default context instance.
func NewDefaultContext(rootDir string) RepoContext {
	return &defaultContext{rootDir: rootDir}
}

// 💿️repoContext holds the data fields for a repoContext record.
type repoContext struct {
	rootDir            string
	bundles            []Bundle
	managementProvider ManagementProvider
}

// 🔶️NewRepoContext MUST initialize all required fields and return a valid repo context.
// 🐙️NewRepoContext creates and returns a new repo context instance.
func NewRepoContext(rootDir string) RepoContext {
	resolvedRoot := rootDir
	if strings.TrimSpace(resolvedRoot) == "" {
		resolvedRoot = findRepoRoot("")
	}
	SetRootDir(resolvedRoot)
	ctx := &repoContext{rootDir: resolvedRoot, managementProvider: DefaultManagementProvider()}
	ctx.bundles = LoadBundles()
	return ctx
}

// 🔹️ManagementProv holds the data fields for a ManagementProv record.
func (c *repoContext) ManagementProv() ManagementProvider { return c.managementProvider }

// 📨️GetRootDir MUST retrieve the requested value or return an error.
// 🟨️GetRootDir retrieves and returns the root dir.
func (c *repoContext) GetRootDir() string { return c.rootDir }

// ❌️GetFileID MUST retrieve the requested value or return an error.
// 🔑️GetFileID retrieves and returns the file i d.
func (c *repoContext) GetFileID(path string) string {
	ctx := &CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFileID(path)
}

// 📁️GetFolderID MUST retrieve the requested value or return an error.
// 🟦️GetFolderID retrieves and returns the folder i d.
func (c *repoContext) GetFolderID(path string) string {
	ctx := &CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFolderID(path)
}

// 📦️GetBundles MUST retrieve the requested value or return an error.
// 🟪️GetBundles retrieves and returns the bundles.
func (c *repoContext) GetBundles() []*Bundle {
	result := make([]*Bundle, len(c.bundles))
	for i := range c.bundles {
		result[i] = &c.bundles[i]
	}
	return result
}

// 📜️GetTechnologies MUST retrieve the requested value or return an error.
// 📺️GetTechnologies retrieves and returns the technologies.
func (c *repoContext) GetTechnologies() []*Technology {
	technologies := LoadTechnologies()
	res := make([]*Technology, len(technologies))
	for i := range technologies {
		res[i] = &technologies[i]
	}
	return res
}

// 💾️GetCheckpoints MUST retrieve the requested value or return an error.
// ✔️GetCheckpoints retrieves and returns the checkpoints.
func (c *repoContext) GetCheckpoints(limit *int) ([]*Checkpoint, error) {
	checkpoints := LoadCheckpoints(limit)
	res := make([]*Checkpoint, len(checkpoints))
	for i := range checkpoints {
		res[i] = &checkpoints[i]
	}
	return res, nil
}

// 🔸️GetFolders MUST retrieve the requested value or return an error.
// 🟫️GetFolders retrieves and returns the folders.
func (c *repoContext) GetFolders() []*Folder {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Folder{}
	}
	folders := BuildCodebaseFolders(ctx)
	results := make([]*Folder, 0, len(folders))
	for _, entry := range folders {
		parent := filepath.Dir(entry.Path)
		var parentID *string
		if parent != "." {
			pid := ctx.GetFolderID(parent)
			parentID = &pid
		}
		bundleID := ctx.GetBundleForFile(entry.Path)
		results = append(results, &Folder{
			ID:       entry.ID,
			Path:     entry.Path,
			URI:      entry.URI,
			Name:     entry.Name,
			ParentID: parentID,
			BundleID: &bundleID,
		})
	}
	return results
}

// 📄️GetFiles MUST retrieve the requested value or return an error.
// 💠️GetFiles retrieves and returns the files.
func (c *repoContext) GetFiles() []*File {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*File{}
	}
	files := BuildCodebaseFiles(ctx)
	results := make([]*File, 0, len(files))
	for _, entry := range files {
		bundleID := ctx.GetBundleForFile(entry.Path)
		folder := filepath.Dir(entry.Path)
		var folderID *string
		if folder != "." {
			fid := ctx.GetFolderID(folder)
			folderID = &fid
		}
		ext := strings.TrimPrefix(filepath.Ext(entry.Path), ".")
		results = append(results, &File{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      filepath.Base(entry.Path),
			Extension: ext,
			FolderID:  folderID,
			BundleID:  &bundleID,
		})
	}
	return results
}

// 📖️GetDefinitions MUST retrieve the requested value or return an error.
// 🔳️GetDefinitions retrieves and returns the definitions.
func (c *repoContext) GetDefinitions() []*Definition {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Definition{}
	}
	var results []*Definition
	for _, file := range ctx.Files {
		tool := ToolDefinitionList(file)
		definitions, ok := tool.Data.([]Definition)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range definitions {
			def := definitions[i]

			var sectionSegments []string
			if def.SectionPath != "" {
				sectionSegments = strings.Split(def.SectionPath, "/")
			}
			id := buildDefinitionID(fileID, sectionSegments, def.Name, def.Kind)
			results = append(results, &Definition{
				Name:        def.Name,
				Kind:        def.Kind,
				FilePath:    def.FilePath,
				SectionPath: def.SectionPath,
				StartLine:   def.StartLine,
				EndLine:     def.EndLine,
				ID:          id,
			})
		}
	}
	return results
}

// 📑️GetSections MUST retrieve the requested value or return an error.
// 🔲️GetSections retrieves and returns the sections.
func (c *repoContext) GetSections() []*Section {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Section{}
	}
	var results []*Section
	for _, file := range ctx.Files {
		tool := ToolSectionList(file)
		sections, ok := tool.Data.([]Section)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range sections {
			sec := sections[i]

			id := buildSectionID(fileID, strings.Split(sec.Path, "/"))
			results = append(results, &Section{
				Name:       sec.Name,
				Path:       sec.Path,
				FilePath:   sec.FilePath,
				StartLine:  sec.StartLine,
				EndLine:    sec.EndLine,
				StartIndex: sec.StartIndex,
				EndIndex:   sec.EndIndex,
				ID:         id,
			})
		}
	}
	return results
}

// 🤝️GetContributors MUST retrieve the requested value or return an error.
// ▪️GetContributors retrieves and returns the contributors.
func (c *repoContext) GetContributors() ([]*Contributor, error) {
	contributors, err := ListContributors()
	if err != nil {
		return nil, err
	}
	result := make([]*Contributor, len(contributors))
	for i := range contributors {
		result[i] = &contributors[i]
	}
	return result, nil
}

// 🎫️GetTickets MUST retrieve the requested value or return an error.
// ▫️GetTickets retrieves and returns the tickets.
func (c *repoContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*Ticket
	for i := range tickets {
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, &tickets[i])
		}
	}
	return result, nil
}

// ⛳️GetGoals MUST retrieve the requested value or return an error.
// ◾GetGoals retrieves and returns the goals.
func (c *repoContext) GetGoals() ([]*Goal, error) {
	return ListGoals()
}
