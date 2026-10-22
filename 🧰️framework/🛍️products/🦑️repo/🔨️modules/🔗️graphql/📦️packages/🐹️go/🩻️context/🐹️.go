// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩻️context carries the filesystem backed repository context and its read queries.

// #endregion 🧲️Header

package graphql

import (
 json "encoding/json"
 fmt "fmt"
 os "os"
 strings "strings"
	filepath "path/filepath"

	codebase "github.com/usalu/semio/repo/codebase"
 events "github.com/usalu/semio/repo/events"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	goalspkg "github.com/usalu/semio/repo/goals"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🩻️Repo Context

// 💿️repoContext holds the data fields for a repoContext record.
type RepoContext struct {
	rootDir            string
	codebase           *codebase.CodebaseContext
	bundles            []model.Bundle
	managementProvider providers.ManagementProvider
	tickets *ticketspkg.TicketService
 ticketSession string
 repositoryURL string
	goals *goalspkg.Goals
}

// 🔶️NewRepoContext MUST initialize all required fields and return a valid repo context.
// 🐙️NewRepoContext creates and returns a new repo context instance.
func NewRepoContext(rootDir string) model.RepoContext {
 snapshot := codebase.NewCodebaseContext(rootDir)
 snapshot.LoadBundles()
 ctx := &RepoContext{rootDir:rootDir, bundles:snapshot.Bundles, codebase:snapshot, managementProvider:providers.DefaultManagementProvider(rootDir)}
 ctx.bindServices()
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
	technologies := c.codebase.Technologies
	res := make([]*model.Technology, len(technologies))
	for i := range technologies {
		res[i] = &technologies[i]
	}
	return res
}

// 💾️GetCheckpoints MUST retrieve the requested value or return an error.
// ✔️GetCheckpoints retrieves and returns the checkpoints.
func (c *RepoContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	checkpoints := contributorspkg.ListCheckpoints(contributorspkg.NewFsCheckpointSource(c.rootDir),limit)
	res := make([]*model.Checkpoint, len(checkpoints))
	for i := range checkpoints {
		res[i] = &checkpoints[i]
	}
	return res, nil
}

// 🔸️GetFolders MUST retrieve the requested value or return an error.
// 🟫️GetFolders retrieves and returns the folders.
func (c *RepoContext) GetFolders() []*model.Folder {
	ctx := &codebase.CodebaseContext{RootDir:c.rootDir, Bundles:c.bundles}
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
			Kind:      (model.ArtifactIndex{RootDir:c.rootDir,Bundles:c.bundles}).FolderKind(entry.Path),
			Generated: model.IsGeneratedFolder(entry.Path),
		})
	}
	return results
}

// 📄️GetFiles MUST retrieve the requested value or return an error.
// 💠️GetFiles retrieves and returns the files.
func (c *RepoContext) GetFiles() []*model.File {
	ctx := &codebase.CodebaseContext{RootDir:c.rootDir, Bundles:c.bundles}
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
	ctx := &codebase.CodebaseContext{RootDir:c.rootDir, Bundles:c.bundles}
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Definition{}
	}
	var results []*model.Definition
	for _, file := range ctx.Files {
		content,err:=workspace.ReadTextFile(filepath.Join(c.rootDir,file))
 if err!=nil { continue }
 definitions:=codebase.FileDefinitions(content,file)
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
	ctx := &codebase.CodebaseContext{RootDir:c.rootDir, Bundles:c.bundles}
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Section{}
	}
	var results []*model.Section
	for _, file := range ctx.Files {
		content,err:=workspace.ReadTextFile(filepath.Join(c.rootDir,file))
 if err!=nil { continue }
 sections:=languages.ParseSections(content,file)
		fileID := ctx.GetFileID(file)
		for i := range sections {
			sec := rootedSection(sections[i],fileID,file)
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
	tickets, err := c.tickets.List(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*model.Ticket
	for i := range tickets {
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, tickets[i])
		}
	}
	return result, nil
}

// ⛳️GetGoals MUST retrieve the requested value or return an error.
// ◾GetGoals retrieves and returns the goals.
func (c *RepoContext) GetGoals() ([]*model.Goal, error) {
	return c.goals.List()
}

// #endregion 🩻️Repo Context

// 🪺️rootedSection retains scoped identities through every nested source node.
func rootedSection(section model.Section,fileID,filePath string) model.Section { section.FilePath=filePath;section.ID=model.BuildSectionID(fileID,languages.NormalizeSectionPath(section.Path));for i:=range section.Children { section.Children[i]=rootedSection(section.Children[i],fileID,filePath) };for i:=range section.Definitions { section.Definitions[i].FilePath=filePath;section.Definitions[i].ID=codebase.DefinitionID(fileID,section.Definitions[i]) };return section }

// 🐙️goalManagement implements the goal management port with one root-bound provider.
type goalManagement struct { providers.ManagementProvider; repositoryURL string }

// 🏁️CreateMilestone preserves the goal domain's integer identity.
func (port goalManagement) CreateMilestone(title, description string) (int64,error) { number,err:=port.ManagementProvider.CreateMilestone(title,description); return int64(number),err }

// 🛠️UpdateMilestone updates the selected repository's milestone.
func (port goalManagement) UpdateMilestone(number int64,title,description,state,due string) error { value,err:=goalNumber(number);if err!=nil { return err };return port.ManagementProvider.UpdateMilestone(value,title,description,state,due) }

// 🗑️DeleteMilestone deletes the selected repository's milestone.
func (port goalManagement) DeleteMilestone(number int64) error { value,err:=goalNumber(number);if err!=nil { return err };return port.ManagementProvider.DeleteMilestone(value) }

// 🎯️CreateGoalIssue preserves the optional milestone identity.
func (port goalManagement) CreateGoalIssue(title,description string,milestone *int64) (string,error) { var selected *int; if milestone!=nil { value,err:=goalNumber(*milestone);if err!=nil { return "",err };selected=&value }; return port.ManagementProvider.CreateGoalIssue(title,description,selected) }

// 🌐️RepoURL returns the origin captured with this repository context.
func (port goalManagement) RepoURL() string { return port.repositoryURL }

// 📡️ticketEvents delivers actual domain events through the captured coordinator.
type ticketEvents struct { emitter events.Emitter }

// 📤️Emit preserves the already encoded ticket payload.
func (port ticketEvents) Emit(kind,source,payload string) { port.emitter.Emit(events.EventKind(kind),source,jsonPayload(payload)) }

// 🧾️jsonPayload retains a ticket's canonical JSON payload as a JSON value.
func jsonPayload(payload string) interface{} { return json.RawMessage(payload) }

// 🧭️bindServices captures all repository-sensitive production ports once.
func (c *RepoContext) bindServices() {
 c.ticketSession=contributorspkg.CurrentTicketSessionID()
 emitter:=events.NewHTTPEmitter(os.Getenv("COMPOSE_SERVER_ADDR"),os.Getenv("COMPOSE_SERVER_TOKEN"))
 home,_:=os.UserHomeDir()
 c.tickets=ticketspkg.NewTicketService(ticketspkg.NewTicketLayout(workspace.RepoMetaDirForRoot(c.rootDir)),&ticketspkg.FileTicketStore{},c.managementProvider,ticketspkg.SystemTicketClock{},ticketEvents{emitter:emitter}).WithRoots(ticketspkg.PlanRoots{RepoRoot:c.rootDir,HomeDir:home})
 remote,_,status:=workspace.ExecCommand("git",[]string{"config","--get","remote.origin.url"},c.rootDir)
 repositoryURL:=""
 if status==0 { repositoryURL=strings.TrimSuffix(strings.TrimSpace(remote),".git"); if strings.HasPrefix(repositoryURL,"git@github.com:") { repositoryURL="https://github.com/"+strings.TrimPrefix(repositoryURL,"git@github.com:") } }
 if strings.HasPrefix(repositoryURL,"ssh://git@github.com/") { repositoryURL="https://github.com/"+strings.TrimPrefix(repositoryURL,"ssh://git@github.com/") }
 c.repositoryURL=repositoryURL
 c.goals=goalspkg.NewGoals(goalspkg.NewFsGoalStore(c.rootDir),goalManagement{ManagementProvider:c.managementProvider,repositoryURL:repositoryURL},emitter,workspace.GetGitAuthor(c.rootDir))
}

// 🔢️goalNumber refuses an identity outside the system port's representable range.
func goalNumber(number int64) (int,error) { value:=int(number);if int64(value)!=number { return 0,fmt.Errorf("milestone identity exceeds system range") };return value,nil }
