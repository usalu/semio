package todos

import (
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌲️Tree

// 📍️TreeEntry is one entry of a scanned tree.
type TreeEntry struct {
	Path  string `json:"path"`
	IsDir bool   `json:"isDir"`
}

// 📄️TreeFile is one path and content pair of a tree.
type TreeFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 🌲️TodoTree is the tree a scan reads and a rewrite writes back to.
type TodoTree interface {
	// 🚶️Walk returns every entry in pre-order, a directory before everything it contains.
	Walk() []TreeEntry
	// 📖️Read returns the content of one file.
	Read(path string) (string, bool)
	// 💾️Write replaces or creates one file.
	Write(path string, content string) error
	// 📁️IsDir reports whether a path is a directory that exists.
	IsDir(path string) bool
	// 🔍️Exists reports whether a path exists at all.
	Exists(path string) bool
}

// ➕️appendToTree appends to a file of a tree, creating it when it does not exist.
func appendToTree(tree TodoTree, path, text string) error {
	existing, _ := tree.Read(path)
	return tree.Write(path, existing+text)
}

// 🧠️MemoryTodoTree is the in-memory tree: a set of directories and a map of file contents.
type MemoryTodoTree struct {
	directories []string
	files       map[string]string
}

// 🌱️NewMemoryTodoTree seeds a tree with files. Every ancestor directory of every file exists.
func NewMemoryTodoTree(files []TreeFile) *MemoryTodoTree {
	tree := &MemoryTodoTree{files: map[string]string{}}
	for _, file := range files {
		prefix := ""
		segments := strings.Split(file.Path, "/")
		for _, segment := range segments[:len(segments)-1] {
			if prefix == "" {
				prefix = segment
			} else {
				prefix = prefix + "/" + segment
			}
			if !containsTodoString(tree.directories, prefix) {
				tree.directories = append(tree.directories, prefix)
			}
		}
		tree.files[file.Path] = file.Content
	}
	return tree
}

// 📸️Snapshot returns every path and content pair, in path order.
func (tree *MemoryTodoTree) Snapshot() []TreeFile {
	paths := make([]string, 0, len(tree.files))
	for path := range tree.files {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	snapshot := make([]TreeFile, 0, len(paths))
	for _, path := range paths {
		snapshot = append(snapshot, TreeFile{Path: path, Content: tree.files[path]})
	}
	return snapshot
}

// 🚶️Walk returns the root, every directory and every file, in path order.
func (tree *MemoryTodoTree) Walk() []TreeEntry {
	entries := []TreeEntry{{Path: "", IsDir: true}}
	for _, path := range tree.directories {
		entries = append(entries, TreeEntry{Path: path, IsDir: true})
	}
	for path := range tree.files {
		entries = append(entries, TreeEntry{Path: path, IsDir: false})
	}
	sort.SliceStable(entries, func(left, right int) bool {
		if entries[left].Path != entries[right].Path {
			return entries[left].Path < entries[right].Path
		}
		return entries[left].IsDir && !entries[right].IsDir
	})
	return entries
}

// 📖️Read returns the content of one file.
func (tree *MemoryTodoTree) Read(path string) (string, bool) {
	content, found := tree.files[path]
	return content, found
}

// 💾️Write replaces or creates one file.
func (tree *MemoryTodoTree) Write(path string, content string) error {
	tree.files[path] = content
	return nil
}

// 📁️IsDir reports whether a path is a directory that exists.
func (tree *MemoryTodoTree) IsDir(path string) bool {
	return path == "" || containsTodoString(tree.directories, path)
}

// 🔍️Exists reports whether a path exists at all.
func (tree *MemoryTodoTree) Exists(path string) bool {
	if tree.IsDir(path) {
		return true
	}
	_, found := tree.files[path]
	return found
}

// 🔎️containsTodoString reports whether a slice carries a value.
func containsTodoString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 💽️FsTodoTree is the tree over a real checkout, applying the scan's own skip rules.
type FsTodoTree struct {
	root string
}

// 🆕️NewFsTodoTree builds the tree rooted at a directory.
func NewFsTodoTree(root string) *FsTodoTree { return &FsTodoTree{root: root} }

// 🛤️absolute resolves a tree relative path against the root.
func (tree *FsTodoTree) absolute(path string) string {
	segments := make([]string, 0, 4)
	segments = append(segments, tree.root)
	for _, segment := range strings.Split(path, "/") {
		if segment != "" {
			segments = append(segments, segment)
		}
	}
	return filepath.Join(segments...)
}

// 🚶️Walk returns the root and everything the scan rules let it descend into.
func (tree *FsTodoTree) Walk() []TreeEntry {
	found := []TreeEntry{{Path: "", IsDir: true}}
	tree.descend("", &found)
	return found
}

// 🚶️descend collects one directory level and recurses into the directories it keeps.
func (tree *FsTodoTree) descend(relative string, found *[]TreeEntry) {
	entries, err := os.ReadDir(tree.absolute(relative))
	if err != nil {
		return
	}
	names := make([]TreeEntry, 0, len(entries))
	for _, entry := range entries {
		names = append(names, TreeEntry{Path: entry.Name(), IsDir: entry.IsDir()})
	}
	sort.SliceStable(names, func(left, right int) bool {
		if names[left].Path != names[right].Path {
			return names[left].Path < names[right].Path
		}
		return !names[left].IsDir && names[right].IsDir
	})
	for _, entry := range names {
		path := joinTodoPath(relative, entry.Path)
		if !entry.IsDir {
			*found = append(*found, TreeEntry{Path: path, IsDir: false})
			continue
		}
		if containsTodoString(TodoSkippedDirectories, entry.Path) {
			continue
		}
		if strings.HasPrefix(entry.Path, ".") && entry.Path != workspace.SemioDirName {
			continue
		}
		*found = append(*found, TreeEntry{Path: path, IsDir: true})
		tree.descend(path, found)
	}
}

// 📖️Read returns the content of one file.
func (tree *FsTodoTree) Read(path string) (string, bool) {
	data, err := os.ReadFile(tree.absolute(path))
	if err != nil {
		return "", false
	}
	return string(data), true
}

// 💾️Write replaces or creates one file.
func (tree *FsTodoTree) Write(path string, content string) error {
	target := tree.absolute(path)
	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		return errTodoPort(err.Error())
	}
	if err := os.WriteFile(target, []byte(content), 0o644); err != nil {
		return errTodoPort(err.Error())
	}
	return nil
}

// 📁️IsDir reports whether a path is a directory that exists.
func (tree *FsTodoTree) IsDir(path string) bool {
	info, err := os.Stat(tree.absolute(path))
	return err == nil && info.IsDir()
}

// 🔍️Exists reports whether a path exists at all.
func (tree *FsTodoTree) Exists(path string) bool {
	_, err := os.Stat(tree.absolute(path))
	return err == nil
}

// 🛤️LocateTodosOnDisk rewrites the tree relative locations of scanned todos into host paths
// under a tree root, which is what a caller reading and rewriting real files needs.
func LocateTodosOnDisk(root string, found []*model.Todo) []*model.Todo {
	for _, todo := range found {
		todo.ParentID = filepath.Join(root, filepath.FromSlash(todo.ParentID))
		if todo.Location != nil {
			todo.Location.FilePath = filepath.Join(root, filepath.FromSlash(todo.Location.FilePath))
		}
	}
	return found
}

// 🎯️ScanTodos returns every todo a tree carries: each directory's own todo markdown items when
// the directory is entered, and each scanned file's todo comments when the file is reached.
func ScanTodos(tree TodoTree) []*model.Todo {
	found := make([]*model.Todo, 0)
	for _, entry := range tree.Walk() {
		if entry.IsDir {
			if content, ok := tree.Read(joinTodoPath(entry.Path, TodoMarkdownName)); ok {
				found = append(found, ParseTodoMarkdown(content, entry.Path)...)
			}
			continue
		}
		if strings.HasSuffix(entry.Path, TodoMarkdownName) {
			continue
		}
		if !containsTodoString(TodoScanExtensions, todoExtensionOf(entry.Path)) {
			continue
		}
		if content, ok := tree.Read(entry.Path); ok {
			found = append(found, ParseTodoComments(content, entry.Path)...)
		}
	}
	return found
}

// 🔎️SearchTodos returns every todo whose name or description contains the term.
func SearchTodos(tree TodoTree, term string) []*model.Todo {
	needle := strings.ToLower(term)
	found := make([]*model.Todo, 0)
	for _, todo := range ScanTodos(tree) {
		if needle == "" || strings.Contains(strings.ToLower(todo.Name), needle) || strings.Contains(strings.ToLower(todo.Description), needle) {
			found = append(found, todo)
		}
	}
	return found
}

// #endregion 🌲️Tree
