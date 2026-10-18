package todos

import (
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎨️Draft Store

// 🪪️DraftID returns the artifact identifier of a draft.
func DraftID(draft *model.Draft) string {
	return identity.EmojiText(identity.Entity("draft")) + workspace.Flat(draft.ID)
}

// 🔗️DraftURI returns the artifact URI of a draft.
func DraftURI(draft *model.Draft) string { return "repo://draft/" + DraftID(draft) }

// 🗄️DraftStore is where drafts live: one directory of copied files per draft identifier.
type DraftStore interface {
	// 📋️IDs returns every draft identifier, in ascending order.
	IDs() []string
	// 🔍️Exists reports whether a draft exists.
	Exists(id string) bool
	// 🆕️Create creates a draft holding these files.
	Create(id string, files []TreeFile) error
	// 📋️Files returns the files of one draft, in file name order.
	Files(id string) []TreeFile
	// 🗑️Delete removes a draft, which is a no-operation when it does not exist.
	Delete(id string) error
}

// 🧠️MemoryDraftStore is the in-memory draft store.
type MemoryDraftStore struct {
	drafts map[string][]TreeFile
}

// 🆕️NewMemoryDraftStore builds an empty store.
func NewMemoryDraftStore() *MemoryDraftStore {
	return &MemoryDraftStore{drafts: map[string][]TreeFile{}}
}

// 📋️IDs returns every draft identifier, in ascending order.
func (store *MemoryDraftStore) IDs() []string {
	ids := make([]string, 0, len(store.drafts))
	for id := range store.drafts {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// 🔍️Exists reports whether a draft exists.
func (store *MemoryDraftStore) Exists(id string) bool {
	_, found := store.drafts[id]
	return found
}

// 🆕️Create creates a draft holding these files.
func (store *MemoryDraftStore) Create(id string, files []TreeFile) error {
	stored := append([]TreeFile{}, files...)
	sort.SliceStable(stored, func(left, right int) bool { return stored[left].Path < stored[right].Path })
	store.drafts[id] = stored
	return nil
}

// 📋️Files returns the files of one draft, in file name order.
func (store *MemoryDraftStore) Files(id string) []TreeFile {
	return append([]TreeFile{}, store.drafts[id]...)
}

// 🗑️Delete removes a draft.
func (store *MemoryDraftStore) Delete(id string) error {
	delete(store.drafts, id)
	return nil
}

// 💽️FsDraftStore is the store over a real notes tree of a repository root.
type FsDraftStore struct {
	root string
}

// 🆕️NewFsDraftStore builds the store of the notes directory of a repository root.
func NewFsDraftStore(root string) *FsDraftStore { return &FsDraftStore{root: root} }

// 📋️IDs returns every draft identifier, in ascending order.
func (store *FsDraftStore) IDs() []string {
	entries, err := os.ReadDir(store.root)
	if err != nil {
		return nil
	}
	ids := make([]string, 0, len(entries))
	for _, entry := range entries {
		if entry.IsDir() {
			ids = append(ids, entry.Name())
		}
	}
	sort.Strings(ids)
	return ids
}

// 🔍️Exists reports whether a draft directory exists.
func (store *FsDraftStore) Exists(id string) bool {
	info, err := os.Stat(filepath.Join(store.root, id))
	return err == nil && info.IsDir()
}

// 🆕️Create writes a draft directory holding these files.
func (store *FsDraftStore) Create(id string, files []TreeFile) error {
	directory := filepath.Join(store.root, id)
	if err := os.MkdirAll(directory, 0o755); err != nil {
		return errTodoPort(err.Error())
	}
	for _, file := range files {
		if err := os.WriteFile(filepath.Join(directory, file.Path), []byte(file.Content), 0o644); err != nil {
			return errTodoPort(err.Error())
		}
	}
	return nil
}

// 📋️Files reads the files of one draft, in file name order.
func (store *FsDraftStore) Files(id string) []TreeFile {
	entries, err := os.ReadDir(filepath.Join(store.root, id))
	if err != nil {
		return nil
	}
	files := make([]TreeFile, 0, len(entries))
	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		data, err := os.ReadFile(filepath.Join(store.root, id, entry.Name()))
		if err != nil {
			continue
		}
		files = append(files, TreeFile{Path: entry.Name(), Content: string(data)})
	}
	sort.SliceStable(files, func(left, right int) bool { return files[left].Path < files[right].Path })
	return files
}

// 🗑️Delete removes a draft directory.
func (store *FsDraftStore) Delete(id string) error {
	if !store.Exists(id) {
		return nil
	}
	if err := os.RemoveAll(filepath.Join(store.root, id)); err != nil {
		return errTodoPort(err.Error())
	}
	return nil
}

// 📥️LoadTreeFiles reads host files into tree files carrying their base name and their content.
func LoadTreeFiles(paths []string) ([]TreeFile, error) {
	files := make([]TreeFile, 0, len(paths))
	for _, path := range paths {
		content, err := os.ReadFile(path)
		if err != nil {
			return nil, errTodoPort(err.Error())
		}
		files = append(files, TreeFile{Path: filepath.Base(path), Content: string(content)})
	}
	return files, nil
}

// 📋️ListDrafts returns every draft the store carries.
func ListDrafts(store DraftStore) []*model.Draft {
	ids := store.IDs()
	drafts := make([]*model.Draft, 0, len(ids))
	for _, id := range ids {
		drafts = append(drafts, &model.Draft{ID: id})
	}
	return drafts
}

// 📝️CreateDraft creates a draft from a title and a set of files. The identifier is the title's
// slug, a title carrying no slug is refused, an existing identifier is refused, and each file is
// copied under its base name.
func CreateDraft(store DraftStore, title string, files []TreeFile) (*model.Draft, error) {
	id := identity.Slugify(title)
	if id == "" {
		return nil, errInvalidTitle()
	}
	if store.Exists(id) {
		return nil, errDraftExists(id)
	}
	copied := make([]TreeFile, 0, len(files))
	for _, file := range files {
		name := file.Path
		if index := strings.LastIndexAny(name, "/\\"); index >= 0 {
			name = name[index+1:]
		}
		copied = append(copied, TreeFile{Path: name, Content: file.Content})
	}
	if err := store.Create(id, copied); err != nil {
		return nil, err
	}
	return &model.Draft{ID: id}, nil
}

// 🗑️DeleteDraft removes a draft. Removing one that does not exist is not an error.
func DeleteDraft(store DraftStore, id string) error { return store.Delete(id) }

// #endregion 🎨️Draft Store
