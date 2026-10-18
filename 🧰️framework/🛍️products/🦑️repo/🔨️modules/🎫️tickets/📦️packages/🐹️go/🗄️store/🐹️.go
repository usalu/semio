package tickets

import (
	fmt "fmt"
	os "os"
	sort "sort"
	strings "strings"
)

// #region 🗄️Store

// 🧱️NodeKind states what a node in the store is.
type NodeKind string

// 📄️NodeFile is a regular file.
const NodeFile NodeKind = "file"

// 🗂️NodeDirectory is a directory.
const NodeDirectory NodeKind = "directory"

// 🔗️NodeSymlink is a symbolic link.
const NodeSymlink NodeKind = "symlink"

// 📏️NodeMetadata is what the store knows about one node.
type NodeMetadata struct {
	Kind NodeKind `json:"kind"`
	Size int64    `json:"size"`
	Mode uint32   `json:"mode"`
}

// 📇️DirectoryEntry is one child of a directory.
type DirectoryEntry struct {
	Name string   `json:"name"`
	Kind NodeKind `json:"kind"`
}

// 🗄️TicketStore is every filesystem touch the ticket domain performs.
type TicketStore interface {
	// 🔍️Metadata returns what is at a path, without following a symlink.
	Metadata(path string) (NodeMetadata, bool)
	// 📖️Read reads a document as text.
	Read(path string) (string, error)
	// ✍️Write writes a document, creating parent directories.
	Write(path string, content string) error
	// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
	CreateExclusive(path string, mode uint32) error
	// 🗂️CreateDirectory creates a directory and every missing ancestor.
	CreateDirectory(path string) error
	// 🗑️RemoveFile removes one file.
	RemoveFile(path string) error
	// 🗑️RemoveDirectory removes one directory, which must be empty.
	RemoveDirectory(path string) error
	// 🧨️RemoveTree removes a whole subtree.
	RemoveTree(path string) error
	// 🚚️Rename moves a node.
	Rename(from, to string) error
	// 📇️Entries lists the children of a directory, sorted by name.
	Entries(path string) ([]DirectoryEntry, error)
}

// ✔️StoreExists reports whether anything is at a path.
func StoreExists(store TicketStore, path string) bool {
	_, found := store.Metadata(path)
	return found
}

// 🗂️StoreIsDirectory reports whether a directory is at a path.
func StoreIsDirectory(store TicketStore, path string) bool {
	metadata, found := store.Metadata(path)
	return found && metadata.Kind == NodeDirectory
}

// 📄️StoreIsFile reports whether a regular file is at a path.
func StoreIsFile(store TicketStore, path string) bool {
	metadata, found := store.Metadata(path)
	return found && metadata.Kind == NodeFile
}

// 🧠️memoryNode is one node of the in-memory store.
type memoryNode struct {
	kind    NodeKind
	content string
	mode    uint32
}

// 🧠️MemoryTicketStore is a whole ticket tree in memory, with failure injection.
type MemoryTicketStore struct {
	nodes         map[string]memoryNode
	writeFailures map[string]string
}

// 🆕️NewMemoryTicketStore builds an empty store.
func NewMemoryTicketStore() *MemoryTicketStore {
	return &MemoryTicketStore{nodes: map[string]memoryNode{}, writeFailures: map[string]string{}}
}

// 💣️FailWrite makes every following write to a path fail with a message.
func (store *MemoryTicketStore) FailWrite(path, message string) {
	store.writeFailures[NormalizeSeparators(path)] = message
}

// 🧹️ClearFailure lifts an injected failure.
func (store *MemoryTicketStore) ClearFailure(path string) {
	delete(store.writeFailures, NormalizeSeparators(path))
}

// 📸️Snapshot returns every file path with its content, in path order.
func (store *MemoryTicketStore) Snapshot() []TicketFile {
	snapshot := make([]TicketFile, 0, len(store.nodes))
	for _, path := range store.Paths() {
		if store.nodes[path].kind == NodeFile {
			snapshot = append(snapshot, TicketFile{Path: path, Content: store.nodes[path].content})
		}
	}
	return snapshot
}

// 📄️TicketFile is one path and content pair of a store snapshot.
type TicketFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 📸️Paths returns every path in the store, files and directories alike, in ascending order.
func (store *MemoryTicketStore) Paths() []string {
	paths := make([]string, 0, len(store.nodes))
	for path := range store.nodes {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	return paths
}

// 🌱️SeedFile seeds one file, creating its ancestors.
func (store *MemoryTicketStore) SeedFile(path, content string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: content, mode: 0o644}
}

// 🌱️SeedSizedFile seeds one file of a size.
func (store *MemoryTicketStore) SeedSizedFile(path string, size int64) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: strings.Repeat("\x00", int(size)), mode: 0o644}
}

// 🌱️SeedDirectory seeds one directory.
func (store *MemoryTicketStore) SeedDirectory(path string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeDirectory, mode: 0o755}
}

// 🔗️SeedSymlink seeds a symlink, which every transaction guard must refuse.
func (store *MemoryTicketStore) SeedSymlink(path string) {
	path = NormalizeSeparators(path)
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeSymlink, mode: 0o777}
}

// 🗂️ensureAncestors materialises every missing ancestor directory of a path.
func (store *MemoryTicketStore) ensureAncestors(path string) {
	current := DirName(path)
	pending := make([]string, 0)
	for current != "." && current != "/" && current != "" {
		if _, found := store.nodes[current]; found {
			break
		}
		pending = append(pending, current)
		parent := DirName(current)
		if parent == current {
			break
		}
		current = parent
	}
	for index := len(pending) - 1; index >= 0; index-- {
		store.nodes[pending[index]] = memoryNode{kind: NodeDirectory, mode: 0o755}
	}
}

// 💣️injected returns the injected failure of a path.
func (store *MemoryTicketStore) injected(path string) (string, bool) {
	message, found := store.writeFailures[path]
	return message, found
}

// 🔍️Metadata returns what is at a path.
func (store *MemoryTicketStore) Metadata(path string) (NodeMetadata, bool) {
	path = NormalizeSeparators(path)
	node, found := store.nodes[path]
	if !found {
		return NodeMetadata{}, false
	}
	return NodeMetadata{Kind: node.kind, Size: int64(len(node.content)), Mode: node.mode}, true
}

// 📖️Read reads a document as text.
func (store *MemoryTicketStore) Read(path string) (string, error) {
	path = NormalizeSeparators(path)
	node, found := store.nodes[path]
	if !found {
		return "", errTicketNotFound(path + " does not exist")
	}
	if node.kind != NodeFile {
		return "", errStore(path + " is not a regular file")
	}
	return node.content, nil
}

// ✍️Write writes a document, creating parent directories.
func (store *MemoryTicketStore) Write(path string, content string) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeFile, content: content, mode: 0o644}
	return nil
}

// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
func (store *MemoryTicketStore) CreateExclusive(path string, mode uint32) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	if _, found := store.nodes[path]; found {
		return errConflict(path + " already exists")
	}
	store.nodes[path] = memoryNode{kind: NodeFile, mode: mode}
	return nil
}

// 🗂️CreateDirectory creates a directory and every missing ancestor.
func (store *MemoryTicketStore) CreateDirectory(path string) error {
	path = NormalizeSeparators(path)
	if message, found := store.injected(path); found {
		return errStore(message)
	}
	if existing, found := store.nodes[path]; found {
		if existing.kind == NodeDirectory {
			return nil
		}
		return errConflict(path + " is not a directory")
	}
	store.ensureAncestors(path)
	store.nodes[path] = memoryNode{kind: NodeDirectory, mode: 0o755}
	return nil
}

// 🗑️RemoveFile removes one file.
func (store *MemoryTicketStore) RemoveFile(path string) error {
	path = NormalizeSeparators(path)
	if _, found := store.nodes[path]; !found {
		return errTicketNotFound(path + " does not exist")
	}
	delete(store.nodes, path)
	return nil
}

// 🗑️RemoveDirectory removes one directory, which must be empty.
func (store *MemoryTicketStore) RemoveDirectory(path string) error {
	path = NormalizeSeparators(path)
	prefix := path + "/"
	for candidate := range store.nodes {
		if strings.HasPrefix(candidate, prefix) {
			return errConflict(path + " is not empty")
		}
	}
	if _, found := store.nodes[path]; !found {
		return errTicketNotFound(path + " does not exist")
	}
	delete(store.nodes, path)
	return nil
}

// 🧨️RemoveTree removes a whole subtree.
func (store *MemoryTicketStore) RemoveTree(path string) error {
	path = NormalizeSeparators(path)
	prefix := path + "/"
	for candidate := range store.nodes {
		if candidate == path || strings.HasPrefix(candidate, prefix) {
			delete(store.nodes, candidate)
		}
	}
	return nil
}

// 🚚️Rename moves a node and everything below it.
func (store *MemoryTicketStore) Rename(from, to string) error {
	from = NormalizeSeparators(from)
	to = NormalizeSeparators(to)
	if message, found := store.injected(to); found {
		return errStore(message)
	}
	if _, found := store.nodes[from]; !found {
		return errTicketNotFound(from + " does not exist")
	}
	if _, found := store.nodes[to]; found {
		return errConflict(to + " already exists")
	}
	prefix := from + "/"
	moved := make([]string, 0)
	for candidate := range store.nodes {
		if candidate == from || strings.HasPrefix(candidate, prefix) {
			moved = append(moved, candidate)
		}
	}
	sort.Strings(moved)
	store.ensureAncestors(to)
	for _, candidate := range moved {
		node := store.nodes[candidate]
		delete(store.nodes, candidate)
		target := to
		if candidate != from {
			target = to + "/" + candidate[len(prefix):]
		}
		store.nodes[target] = node
	}
	return nil
}

// 📇️Entries lists the children of a directory, sorted by name.
func (store *MemoryTicketStore) Entries(path string) ([]DirectoryEntry, error) {
	path = NormalizeSeparators(path)
	if !StoreIsDirectory(store, path) {
		return nil, errTicketNotFound(path + " is not a directory")
	}
	prefix := path + "/"
	listed := make([]DirectoryEntry, 0)
	for candidate, node := range store.nodes {
		if !strings.HasPrefix(candidate, prefix) {
			continue
		}
		rest := candidate[len(prefix):]
		if strings.Contains(rest, "/") {
			continue
		}
		listed = append(listed, DirectoryEntry{Name: rest, Kind: node.kind})
	}
	sort.SliceStable(listed, func(left, right int) bool { return listed[left].Name < listed[right].Name })
	return listed, nil
}

// 💽️FileTicketStore is the real filesystem.
type FileTicketStore struct{}

// 🆕️NewFileTicketStore builds the store rooted at the machine's filesystem.
func NewFileTicketStore() *FileTicketStore { return &FileTicketStore{} }

// 🔍️Metadata returns what is at a path, without following a symlink.
func (store *FileTicketStore) Metadata(path string) (NodeMetadata, bool) {
	info, err := os.Lstat(path)
	if err != nil {
		return NodeMetadata{}, false
	}
	kind := NodeFile
	switch {
	case info.Mode()&os.ModeSymlink != 0:
		kind = NodeSymlink
	case info.IsDir():
		kind = NodeDirectory
	}
	return NodeMetadata{Kind: kind, Size: info.Size(), Mode: uint32(info.Mode().Perm())}, true
}

// 📖️Read reads a document as text.
func (store *FileTicketStore) Read(path string) (string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return "", errStore(fmt.Sprintf("read %s: %v", path, err))
	}
	return string(data), nil
}

// ✍️Write writes a document, creating parent directories.
func (store *FileTicketStore) Write(path string, content string) error {
	parent := DirName(path)
	if parent != "." && parent != "/" {
		if err := os.MkdirAll(parent, 0o755); err != nil {
			return errStore(fmt.Sprintf("create %s: %v", parent, err))
		}
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		return errStore(fmt.Sprintf("write %s: %v", path, err))
	}
	return nil
}

// ✨️CreateExclusive creates an empty file, failing when anything occupies the path.
func (store *FileTicketStore) CreateExclusive(path string, mode uint32) error {
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, os.FileMode(mode))
	if err != nil {
		return errStore(fmt.Sprintf("create %s: %v", path, err))
	}
	return file.Close()
}

// 🗂️CreateDirectory creates a directory and every missing ancestor.
func (store *FileTicketStore) CreateDirectory(path string) error {
	if err := os.MkdirAll(path, 0o755); err != nil {
		return errStore(fmt.Sprintf("create %s: %v", path, err))
	}
	return nil
}

// 🗑️RemoveFile removes one file.
func (store *FileTicketStore) RemoveFile(path string) error {
	if err := os.Remove(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🗑️RemoveDirectory removes one directory, which must be empty.
func (store *FileTicketStore) RemoveDirectory(path string) error {
	if err := os.Remove(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🧨️RemoveTree removes a whole subtree.
func (store *FileTicketStore) RemoveTree(path string) error {
	if err := os.RemoveAll(path); err != nil {
		return errStore(fmt.Sprintf("remove %s: %v", path, err))
	}
	return nil
}

// 🚚️Rename moves a node.
func (store *FileTicketStore) Rename(from, to string) error {
	if err := os.Rename(from, to); err != nil {
		return errStore(fmt.Sprintf("rename %s -> %s: %v", from, to, err))
	}
	return nil
}

// 📇️Entries lists the children of a directory, sorted by name.
func (store *FileTicketStore) Entries(path string) ([]DirectoryEntry, error) {
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, errStore(fmt.Sprintf("read directory %s: %v", path, err))
	}
	listed := make([]DirectoryEntry, 0, len(entries))
	for _, entry := range entries {
		kind := NodeFile
		switch {
		case entry.Type()&os.ModeSymlink != 0:
			kind = NodeSymlink
		case entry.IsDir():
			kind = NodeDirectory
		}
		listed = append(listed, DirectoryEntry{Name: entry.Name(), Kind: kind})
	}
	sort.SliceStable(listed, func(left, right int) bool { return listed[left].Name < listed[right].Name })
	return listed, nil
}

// #endregion 🗄️Store
