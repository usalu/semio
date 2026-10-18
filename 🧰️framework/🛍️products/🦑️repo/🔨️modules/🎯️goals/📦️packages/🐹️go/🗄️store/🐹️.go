package goals

import (
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗄️Store

// 📄️StoredGoal is one identifier and document pair of a goal store.
type StoredGoal struct {
	ID       string `json:"id"`
	Document string `json:"document"`
}

// 🗄️GoalStore is where goal documents live, one entry per identifier.
type GoalStore interface {
	// 📖️Read returns the document at an identifier.
	Read(id string) (string, error)
	// 💾️Write replaces or creates the document at an identifier.
	Write(id string, document string) error
	// 🔍️Exists reports whether a document exists at an identifier.
	Exists(id string) bool
	// 🚚️Rename moves a goal and everything below it to a new identifier.
	Rename(from, to string) error
	// 🗑️Remove removes a goal and everything below it.
	Remove(id string) error
	// 📋️IDs returns every identifier that carries a document, in ascending order.
	IDs() []string
}

// 🧠️MemoryGoalStore is the in-memory store the language-agnostic tests run the lifecycle against.
type MemoryGoalStore struct {
	documents map[string]string
}

// 🆕️NewMemoryGoalStore builds an empty store.
func NewMemoryGoalStore() *MemoryGoalStore {
	return &MemoryGoalStore{documents: map[string]string{}}
}

// 🌱️NewSeededMemoryGoalStore builds a store seeded with identifier and document pairs.
func NewSeededMemoryGoalStore(entries []StoredGoal) *MemoryGoalStore {
	store := NewMemoryGoalStore()
	for _, entry := range entries {
		store.documents[entry.ID] = entry.Document
	}
	return store
}

// 📸️Snapshot returns every stored pair, in identifier order.
func (store *MemoryGoalStore) Snapshot() []StoredGoal {
	snapshot := make([]StoredGoal, 0, len(store.documents))
	for _, id := range store.IDs() {
		snapshot = append(snapshot, StoredGoal{ID: id, Document: store.documents[id]})
	}
	return snapshot
}

// 📖️Read returns the document at an identifier.
func (store *MemoryGoalStore) Read(id string) (string, error) {
	document, found := store.documents[id]
	if !found {
		return "", errNotFound(id)
	}
	return document, nil
}

// 💾️Write replaces or creates the document at an identifier.
func (store *MemoryGoalStore) Write(id string, document string) error {
	store.documents[id] = document
	return nil
}

// 🔍️Exists reports whether a document exists at an identifier.
func (store *MemoryGoalStore) Exists(id string) bool {
	_, found := store.documents[id]
	return found
}

// 🚚️Rename moves a goal and everything below it to a new identifier.
func (store *MemoryGoalStore) Rename(from, to string) error {
	moved := make([]string, 0)
	for _, id := range store.IDs() {
		if id == from || strings.HasPrefix(id, from+"/") {
			moved = append(moved, id)
		}
	}
	if len(moved) == 0 {
		return errNotFound(from)
	}
	for _, id := range moved {
		document := store.documents[id]
		delete(store.documents, id)
		store.documents[to+id[len(from):]] = document
	}
	return nil
}

// 🗑️Remove removes a goal and everything below it.
func (store *MemoryGoalStore) Remove(id string) error {
	for _, candidate := range store.IDs() {
		if candidate == id || strings.HasPrefix(candidate, id+"/") {
			delete(store.documents, candidate)
		}
	}
	return nil
}

// 📋️IDs returns every stored identifier in ascending order.
func (store *MemoryGoalStore) IDs() []string {
	ids := make([]string, 0, len(store.documents))
	for id := range store.documents {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// 💽️FsGoalStore is the store over a real goals tree of a repository root.
type FsGoalStore struct {
	root string
}

// 🆕️NewFsGoalStore builds the store of the goals directory of a repository root.
func NewFsGoalStore(repoRoot string) *FsGoalStore {
	return &FsGoalStore{root: workspace.GoalsDirForRoot(repoRoot)}
}

// 📁️Directory returns the directory of one goal.
func (store *FsGoalStore) Directory(id string) string {
	return filepath.Join(append([]string{store.root}, strings.Split(id, "/")...)...)
}

// 📄️documentPath returns the document path of one goal.
func (store *FsGoalStore) documentPath(id string) string {
	return filepath.Join(store.Directory(id), "🎯️goal.json")
}

// 📖️Read returns the document at an identifier.
func (store *FsGoalStore) Read(id string) (string, error) {
	data, err := os.ReadFile(store.documentPath(id))
	if err != nil {
		return "", errNotFound(id)
	}
	return string(data), nil
}

// 💾️Write replaces or creates the document at an identifier.
func (store *FsGoalStore) Write(id string, document string) error {
	path := store.documentPath(id)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return errPort(err.Error())
	}
	if err := os.WriteFile(path, []byte(document), 0o644); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 🔍️Exists reports whether a document exists at an identifier.
func (store *FsGoalStore) Exists(id string) bool {
	_, err := os.Stat(store.documentPath(id))
	return err == nil
}

// 🚚️Rename moves a goal directory and everything below it.
func (store *FsGoalStore) Rename(from, to string) error {
	target := store.Directory(to)
	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		return errPort(err.Error())
	}
	if err := os.Rename(store.Directory(from), target); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 🗑️Remove removes a goal directory and everything below it.
func (store *FsGoalStore) Remove(id string) error {
	if err := os.RemoveAll(store.Directory(id)); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 📋️IDs returns every identifier that carries a document, in ascending order.
func (store *FsGoalStore) IDs() []string {
	found := make([]string, 0)
	collectGoalIDs(store.root, "", &found)
	sort.Strings(found)
	return found
}

// 🚶️collectGoalIDs descends a goals directory collecting every identifier that carries a document.
func collectGoalIDs(directory, prefix string, found *[]string) {
	entries, err := os.ReadDir(directory)
	if err != nil {
		return
	}
	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		id := entry.Name()
		if prefix != "" {
			id = prefix + "/" + entry.Name()
		}
		child := filepath.Join(directory, entry.Name())
		if _, err := os.Stat(filepath.Join(child, "🎯️goal.json")); err == nil {
			*found = append(*found, id)
		}
		collectGoalIDs(child, id, found)
	}
}

// #endregion 🗄️Store
