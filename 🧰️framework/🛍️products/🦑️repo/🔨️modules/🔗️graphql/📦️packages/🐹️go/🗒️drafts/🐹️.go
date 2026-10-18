// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗒️drafts carries the draft store accessors of the repository context.

// #endregion 🧲️Header

package graphql

import (
	model "github.com/usalu/semio/repo/model"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗒️Drafts

// 🔬️scanTodos returns every todo of the repository tree the context reads, at host paths.
func (c *RepoContext) scanTodos() []*model.Todo {
	return todospkg.LocateTodosOnDisk(c.rootDir, todospkg.ScanTodos(todospkg.NewFsTodoTree(c.rootDir)))
}

// 🎨️draftStore returns the store the notes directory of the repository keeps drafts in.
func (c *RepoContext) draftStore() todospkg.DraftStore {
	return todospkg.NewFsDraftStore(workspace.GetDraftsPath())
}

// ⬛️GetDrafts MUST retrieve the requested value or return an error.
// 🟠️GetDrafts retrieves and returns the drafts.
func (c *RepoContext) GetDrafts() ([]*model.Draft, error) {
	return todospkg.ListDrafts(c.draftStore()), nil
}

// ⬜️DraftCreate MUST return a non-nil error when the operation fails.
// 🟡️DraftCreate performs the draft create operation on the repo context.
func (c *RepoContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	files, err := todospkg.LoadTreeFiles(input.Files)
	if err != nil {
		return nil, err
	}
	return todospkg.CreateDraft(c.draftStore(), input.Title, files)
}

// 🟥️DraftDelete MUST return a non-nil error when the operation fails.
// 🟢️DraftDelete performs the draft delete operation on the repo context.
func (c *RepoContext) DraftDelete(id string) (bool, error) {
	return true, todospkg.DeleteDraft(c.draftStore(), id)
}

// #endregion 🗒️Drafts
