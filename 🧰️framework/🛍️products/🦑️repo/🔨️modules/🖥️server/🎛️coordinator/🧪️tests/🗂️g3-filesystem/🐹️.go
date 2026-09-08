// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 faithful in-memory filesystem and durability fault laws.

// #endregion 🧲️Header

package main

import (
	"context"
	"errors"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// #region 🧠️Filesystem

type memoryStoreOperation struct {
	Kind        string
	Path        string
	Source      string
	Destination string
}

type memoryStoreNode struct {
	data     []byte
	modified time.Time
}

type memoryStoreOperations struct {
	mu         sync.Mutex
	files      map[string]*memoryStoreNode
	operations []memoryStoreOperation
	fault      func(memoryStoreOperation) bool
	faultErr   error
	faulted    bool
}

type memoryStoreFile struct {
	operations *memoryStoreOperations
	path       string
	offset     int
	closed     bool
}

type memoryStoreInfo struct {
	name     string
	size     int64
	modified time.Time
}

func newMemoryStoreOperations() *memoryStoreOperations {
	return &memoryStoreOperations{files: map[string]*memoryStoreNode{}}
}

func (operations *memoryStoreOperations) setFault(fault func(memoryStoreOperation) bool, failure error) {
	operations.mu.Lock()
	operations.fault = fault
	operations.faultErr = failure
	operations.faulted = false
	operations.mu.Unlock()
}

func (operations *memoryStoreOperations) clearFault() {
	operations.setFault(nil, nil)
}

func (operations *memoryStoreOperations) check(operation memoryStoreOperation) error {
	operations.mu.Lock()
	defer operations.mu.Unlock()
	operation.Path = filepath.Clean(operation.Path)
	operation.Source = filepath.Clean(operation.Source)
	operation.Destination = filepath.Clean(operation.Destination)
	operations.operations = append(operations.operations, operation)
	if !operations.faulted && operations.fault != nil && operations.fault(operation) {
		operations.faulted = true
		return operations.faultErr
	}
	return nil
}

func (operations *memoryStoreOperations) MkdirAll(path string, mode os.FileMode) error {
	return operations.check(memoryStoreOperation{Kind: "mkdir", Path: path})
}

func (operations *memoryStoreOperations) Open(path string) (storeFile, error) {
	if err := operations.check(memoryStoreOperation{Kind: "open-read", Path: path}); err != nil {
		return nil, err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	if _, exists := operations.files[path]; !exists {
		return nil, memoryPathError("open", path, os.ErrNotExist)
	}
	return &memoryStoreFile{operations: operations, path: path}, nil
}

func (operations *memoryStoreOperations) OpenFile(path string, flag int, mode os.FileMode) (storeFile, error) {
	kind := "open-write"
	if flag&os.O_CREATE != 0 {
		kind = "create"
	}
	if err := operations.check(memoryStoreOperation{Kind: kind, Path: path}); err != nil {
		return nil, err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	node, exists := operations.files[path]
	if flag&os.O_CREATE != 0 && flag&os.O_EXCL != 0 && exists {
		return nil, memoryPathError("create", path, os.ErrExist)
	}
	if !exists {
		if flag&os.O_CREATE == 0 {
			return nil, memoryPathError("open", path, os.ErrNotExist)
		}
		node = &memoryStoreNode{modified: time.Now()}
		operations.files[path] = node
	}
	if flag&os.O_TRUNC != 0 {
		node.data = nil
		node.modified = time.Now()
	}
	return &memoryStoreFile{operations: operations, path: path}, nil
}

func (operations *memoryStoreOperations) ReadFile(path string) ([]byte, error) {
	if err := operations.check(memoryStoreOperation{Kind: "read-file", Path: path}); err != nil {
		return nil, err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	node, exists := operations.files[path]
	if !exists {
		return nil, memoryPathError("read", path, os.ErrNotExist)
	}
	return append([]byte(nil), node.data...), nil
}

func (operations *memoryStoreOperations) Lstat(path string) (os.FileInfo, error) {
	return operations.fileInfo("lstat", path)
}

func (operations *memoryStoreOperations) Stat(path string) (os.FileInfo, error) {
	return operations.fileInfo("stat", path)
}

func (operations *memoryStoreOperations) fileInfo(kind string, path string) (os.FileInfo, error) {
	if err := operations.check(memoryStoreOperation{Kind: kind, Path: path}); err != nil {
		return nil, err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	node, exists := operations.files[path]
	if !exists {
		return nil, memoryPathError(kind, path, os.ErrNotExist)
	}
	return memoryStoreInfo{name: filepath.Base(path), size: int64(len(node.data)), modified: node.modified}, nil
}

func (operations *memoryStoreOperations) Rename(source string, destination string) error {
	operation := memoryStoreOperation{Kind: "rename", Source: source, Destination: destination}
	if err := operations.check(operation); err != nil {
		return err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	source = filepath.Clean(source)
	destination = filepath.Clean(destination)
	node, exists := operations.files[source]
	if !exists {
		return memoryPathError("rename", source, os.ErrNotExist)
	}
	operations.files[destination] = node
	delete(operations.files, source)
	node.modified = time.Now()
	return nil
}

func (operations *memoryStoreOperations) Remove(path string) error {
	if err := operations.check(memoryStoreOperation{Kind: "remove", Path: path}); err != nil {
		return err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	if _, exists := operations.files[path]; !exists {
		return nil
	}
	delete(operations.files, path)
	return nil
}

func (operations *memoryStoreOperations) SyncParent(path string) error {
	return operations.check(memoryStoreOperation{Kind: "sync-parent", Path: path})
}

func (operations *memoryStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	if err := operations.check(memoryStoreOperation{Kind: "heartbeat", Path: path}); err != nil {
		return err
	}
	operations.mu.Lock()
	defer operations.mu.Unlock()
	path = filepath.Clean(path)
	node, exists := operations.files[path]
	if !exists {
		return memoryPathError("chtimes", path, os.ErrNotExist)
	}
	node.modified = modified
	return nil
}

func (operations *memoryStoreOperations) bytes(path string) []byte {
	operations.mu.Lock()
	defer operations.mu.Unlock()
	node := operations.files[filepath.Clean(path)]
	if node == nil {
		return nil
	}
	return append([]byte(nil), node.data...)
}

func (operations *memoryStoreOperations) exists(path string) bool {
	operations.mu.Lock()
	defer operations.mu.Unlock()
	_, exists := operations.files[filepath.Clean(path)]
	return exists
}

func (file *memoryStoreFile) Read(buffer []byte) (int, error) {
	if err := file.operations.check(memoryStoreOperation{Kind: "read", Path: file.path}); err != nil {
		return 0, err
	}
	file.operations.mu.Lock()
	defer file.operations.mu.Unlock()
	if file.closed {
		return 0, os.ErrClosed
	}
	node := file.operations.files[file.path]
	if node == nil {
		return 0, os.ErrNotExist
	}
	if file.offset >= len(node.data) {
		return 0, io.EOF
	}
	read := copy(buffer, node.data[file.offset:])
	file.offset += read
	return read, nil
}

func (file *memoryStoreFile) Write(data []byte) (int, error) {
	if err := file.operations.check(memoryStoreOperation{Kind: "write", Path: file.path}); err != nil {
		return 0, err
	}
	file.operations.mu.Lock()
	defer file.operations.mu.Unlock()
	if file.closed {
		return 0, os.ErrClosed
	}
	node := file.operations.files[file.path]
	if node == nil {
		return 0, os.ErrNotExist
	}
	end := file.offset + len(data)
	if end > len(node.data) {
		node.data = append(node.data, make([]byte, end-len(node.data))...)
	}
	copy(node.data[file.offset:end], data)
	file.offset = end
	node.modified = time.Now()
	return len(data), nil
}

func (file *memoryStoreFile) Stat() (os.FileInfo, error) {
	return file.operations.fileInfo("file-stat", file.path)
}

func (file *memoryStoreFile) Sync() error {
	return file.operations.check(memoryStoreOperation{Kind: "sync-file", Path: file.path})
}

func (file *memoryStoreFile) Truncate(size int64) error {
	if err := file.operations.check(memoryStoreOperation{Kind: "truncate", Path: file.path}); err != nil {
		return err
	}
	file.operations.mu.Lock()
	defer file.operations.mu.Unlock()
	node := file.operations.files[file.path]
	if node == nil {
		return os.ErrNotExist
	}
	if size < 0 {
		return errors.New("negative truncate")
	}
	if size <= int64(len(node.data)) {
		node.data = node.data[:size]
	} else {
		node.data = append(node.data, make([]byte, int(size)-len(node.data))...)
	}
	node.modified = time.Now()
	return nil
}

func (file *memoryStoreFile) Close() error {
	if err := file.operations.check(memoryStoreOperation{Kind: "close", Path: file.path}); err != nil {
		return err
	}
	file.operations.mu.Lock()
	file.closed = true
	file.operations.mu.Unlock()
	return nil
}

func (info memoryStoreInfo) Name() string       { return info.name }
func (info memoryStoreInfo) Size() int64        { return info.size }
func (info memoryStoreInfo) Mode() os.FileMode  { return 0o600 }
func (info memoryStoreInfo) ModTime() time.Time { return info.modified }
func (info memoryStoreInfo) IsDir() bool        { return false }
func (info memoryStoreInfo) Sys() any           { return nil }

func memoryPathError(operation string, path string, failure error) error {
	return &os.PathError{Op: operation, Path: path, Err: failure}
}

func memoryTestStore(t *testing.T) (*EventStore, *memoryStoreOperations, string) {
	t.Helper()
	operations := newMemoryStoreOperations()
	path := filepath.Join(string(filepath.Separator), "memory", strings.ReplaceAll(t.Name(), "/", "-"), "coordinator.events")
	store, err := openEventStoreWithOperations(context.Background(), path, DefaultStoreLimits(), operations)
	if err != nil {
		t.Fatal(err)
	}
	return store, operations, path
}

// #endregion 🧠️Filesystem

// #region 💥️FaultLaws

func TestG3FaithfulFilesystemFaultsEveryPrecommitOperation(t *testing.T) {
	type faultCase struct {
		name  string
		match func(memoryStoreOperation, string) bool
	}
	pathIs := func(operation memoryStoreOperation, kind string, suffix string, path string) bool {
		return operation.Kind == kind && operation.Path == filepath.Clean(path+suffix)
	}
	cases := []faultCase{
		{name: "lock-create", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "create", ".lock", path)
		}},
		{name: "lock-write", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "write", ".lock", path)
		}},
		{name: "lock-file-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-file", ".lock", path)
		}},
		{name: "lock-close", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "close", ".lock", path)
		}},
		{name: "next-create", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "create", ".next", path)
		}},
		{name: "next-write", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "write", ".next", path)
		}},
		{name: "next-file-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-file", ".next", path)
		}},
		{name: "next-close", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "close", ".next", path)
		}},
		{name: "next-parent-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-parent", ".next", path)
		}},
		{name: "stage-create", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "create", ".stage.next", path)
		}},
		{name: "stage-write", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "write", ".stage.next", path)
		}},
		{name: "stage-file-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-file", ".stage.next", path)
		}},
		{name: "stage-close", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "close", ".stage.next", path)
		}},
		{name: "stage-entry-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-parent", ".stage.next", path)
		}},
		{name: "stage-rename", match: func(operation memoryStoreOperation, path string) bool {
			return operation.Kind == "rename" && operation.Source == filepath.Clean(path+".stage.next") && operation.Destination == filepath.Clean(path+".stage")
		}},
		{name: "stage-parent-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-parent", ".stage", path)
		}},
		{name: "prior-rename", match: func(operation memoryStoreOperation, path string) bool {
			return operation.Kind == "rename" && operation.Source == filepath.Clean(path) && operation.Destination == filepath.Clean(path+".backup")
		}},
		{name: "prior-parent-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-parent", ".backup", path)
		}},
		{name: "live-replacement", match: func(operation memoryStoreOperation, path string) bool {
			return operation.Kind == "rename" && operation.Source == filepath.Clean(path+".next") && operation.Destination == filepath.Clean(path)
		}},
		{name: "live-parent-sync", match: func(operation memoryStoreOperation, path string) bool {
			return pathIs(operation, "sync-parent", "", path)
		}},
	}
	for _, candidate := range cases {
		t.Run(candidate.name, func(t *testing.T) {
			store, operations, path := memoryTestStore(t)
			appendTestEvent(t, store, 0, "prior")
			before := operations.bytes(path)
			injected := errors.New("injected " + candidate.name)
			operations.setFault(func(operation memoryStoreOperation) bool {
				return candidate.match(operation, path)
			}, injected)
			result, err := store.Append(context.Background(), 1, []EventInput{testInput("rejected", `{}`)}, nil)
			if err == nil || result.Committed {
				t.Fatalf("fault result=%+v err=%v", result, err)
			}
			if after := operations.bytes(path); !bytesEqual(before, after) {
				t.Fatal("precommit fault changed exact durable bytes")
			}
			operations.clearFault()
			reopened, err := openEventStoreWithOperations(context.Background(), path, DefaultStoreLimits(), operations)
			if err != nil {
				t.Fatalf("reopen after fault: %v", err)
			}
			events, err := reopened.Replay(context.Background(), nil)
			if err != nil || len(events) != 1 || events[0].ID != "prior" {
				t.Fatalf("reopened events=%+v err=%v", events, err)
			}
			for _, suffix := range []string{".next", ".stage.next", ".stage", ".backup", ".lock"} {
				if operations.exists(path + suffix) {
					t.Fatalf("recovery retained artifact %s", suffix)
				}
			}
		})
	}
}

func TestG3CommittedCleanupFailureIsExplicitAndRecoveryRetryObservable(t *testing.T) {
	cases := []struct {
		name  string
		fault func(memoryStoreOperation, string) bool
	}{
		{name: "remove", fault: func(operation memoryStoreOperation, path string) bool {
			return operation.Kind == "remove" && operation.Path == filepath.Clean(path+".backup")
		}},
		{name: "parent-sync", fault: func() func(memoryStoreOperation, string) bool {
			matches := 0
			return func(operation memoryStoreOperation, path string) bool {
				if operation.Kind == "sync-parent" && operation.Path == filepath.Clean(path+".backup") {
					matches++
					return matches == 2
				}
				return false
			}
		}()},
	}
	for _, candidate := range cases {
		t.Run(candidate.name, func(t *testing.T) {
			store, operations, path := memoryTestStore(t)
			appendTestEvent(t, store, 0, "prior")
			injected := errors.New("cleanup " + candidate.name + " failed")
			operations.setFault(func(operation memoryStoreOperation) bool {
				return candidate.fault(operation, path)
			}, injected)
			result, err := store.Append(context.Background(), 1, []EventInput{testInput("committed", `{}`)}, nil)
			var pending *PendingCleanupError
			if !errors.As(err, &pending) || !errors.Is(err, injected) || !result.Committed || !result.PendingCleanup || len(result.Events) != 1 {
				t.Fatalf("pending cleanup result=%+v err=%v", result, err)
			}
			if !pending.IsCommitted() || len(operations.bytes(path)) == 0 || !operations.exists(path+".stage") {
				t.Fatal("committed cleanup state was not explicit and recoverable")
			}
			operations.clearFault()
			reopened, err := openEventStoreWithOperations(context.Background(), path, DefaultStoreLimits(), operations)
			if err != nil {
				t.Fatalf("cleanup retry reopen: %v", err)
			}
			status := reopened.RecoveryStatus()
			if !status.Recovered || status.Action != "committed-cleanup" {
				t.Fatalf("recovery retry was not observable: %+v", status)
			}
			events, err := reopened.Replay(context.Background(), nil)
			if err != nil || len(events) != 2 || events[1].ID != "committed" {
				t.Fatalf("committed recovery events=%+v err=%v", events, err)
			}
			for _, suffix := range []string{".next", ".stage.next", ".stage", ".backup", ".lock"} {
				if operations.exists(path + suffix) {
					t.Fatalf("recovery retry retained artifact %s", suffix)
				}
			}
		})
	}
}

func TestG3RepositoryAppliesCommittedEventWhilePropagatingPendingCleanup(t *testing.T) {
	store, operations, path := memoryTestStore(t)
	repository := &EventRepository{store: store, projection: newCoordinatorProjection()}
	if err := repository.recordTicket(context.Background(), Ticket{ID: "prior", Status: "open"}); err != nil {
		t.Fatal(err)
	}
	operations.setFault(func(operation memoryStoreOperation) bool {
		return operation.Kind == "remove" && operation.Path == filepath.Clean(path+".backup")
	}, errors.New("cleanup failed"))
	err := repository.recordTicket(context.Background(), Ticket{ID: "committed", Status: "open"})
	var pending *PendingCleanupError
	if !errors.As(err, &pending) || !pending.IsCommitted() {
		t.Fatalf("repository cleanup error=%v", err)
	}
	tickets, queryErr := repository.projectTickets(context.Background(), "")
	if queryErr != nil || len(tickets) != 2 || tickets[1].ID != "prior" {
		t.Fatalf("committed projection tickets=%v err=%v", tickets, queryErr)
	}
	operations.clearFault()
	if err := repository.Reopen(context.Background()); err != nil {
		t.Fatalf("repository recovery retry: %v", err)
	}
	status := store.RecoveryStatus()
	if !status.Recovered || status.Action != "committed-cleanup" {
		t.Fatalf("repository recovery status=%+v", status)
	}
	replayed, queryErr := repository.projectTickets(context.Background(), "")
	if queryErr != nil || len(replayed) != 2 {
		t.Fatalf("replayed committed projection=%v err=%v", replayed, queryErr)
	}
}

func TestG3LockRemoveAndHeartbeatFailuresAreObservableCommittedState(t *testing.T) {
	t.Run("lock-remove", func(t *testing.T) {
		store, operations, path := memoryTestStore(t)
		operations.setFault(func(operation memoryStoreOperation) bool {
			return operation.Kind == "remove" && operation.Path == filepath.Clean(path+".lock")
		}, errors.New("lock remove failed"))
		result, err := store.Append(context.Background(), 0, []EventInput{testInput("committed", `{}`)}, nil)
		if err == nil || !result.Committed || result.PendingCleanup {
			t.Fatalf("lock cleanup result=%+v err=%v", result, err)
		}
		operations.clearFault()
		if err := operations.Remove(path + ".lock"); err != nil {
			t.Fatal(err)
		}
	})
	t.Run("heartbeat", func(t *testing.T) {
		store, operations, _ := memoryTestStore(t)
		store.heartbeatInterval = time.Millisecond
		injected := errors.New("heartbeat failed")
		operations.setFault(func(operation memoryStoreOperation) bool {
			return operation.Kind == "heartbeat"
		}, injected)
		store.interrupt = func(phase string) error {
			if phase == "next-synced" {
				time.Sleep(5 * time.Millisecond)
			}
			return nil
		}
		result, err := store.Append(context.Background(), 0, []EventInput{testInput("committed", `{}`)}, nil)
		if !errors.Is(err, injected) || !result.Committed {
			t.Fatalf("heartbeat result=%+v err=%v", result, err)
		}
	})
}

func bytesEqual(first []byte, second []byte) bool {
	return string(first) == string(second)
}

// #endregion 💥️FaultLaws
