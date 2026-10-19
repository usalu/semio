// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎥️repo_context carries the repo context port and its lookup hooks.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"
)

// #region 🎥️Repo Context

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
	GetInteractions() ([]InteractionResource, error)
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

// 🎯️parseMilestoneNumber holds the data fields for a parseMilestoneNumber record.
func ParseMilestoneNumber(milestone string) (int, error) {
	if n, err := strconv.Atoi(milestone); err == nil {
		return n, nil
	}
	parts := strings.Split(milestone, "/")
	if len(parts) > 0 {
		if n, err := strconv.Atoi(parts[len(parts)-1]); err == nil {
			return n, nil
		}
	}
	return 0, fmt.Errorf("could not parse milestone number from %s", milestone)
}

// 🧭️LookupBundles answers every known bundle. 🗂️codebase installs the real walk in its init.
var LookupBundles = func() []Bundle { return nil }

// 🧬️LookupTechnologies answers every known technology. 🗂️codebase installs the real walk.
var LookupTechnologies = func() []Technology { return nil }

// 🧑️LookupContributors answers every known contributor. 🧑️contributors installs the real read.
var LookupContributors = func() ([]Contributor, error) { return nil, nil }

// 🔗️LookupRepoContext builds the repository port for a root. 🔗️graphql installs the real one.
var LookupRepoContext = func(root string) RepoContext { return nil }

// #endregion 🎥️Repo Context
