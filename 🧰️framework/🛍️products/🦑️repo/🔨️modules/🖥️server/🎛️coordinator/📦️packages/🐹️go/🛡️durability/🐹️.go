// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️durability carries the coordinator store file-operation seam: the native operations and the fault-injecting operations.

// #endregion 🧲️Header

package coordinator

import (
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	durability "github.com/usalu/semio/framework/filesystem/durability"
)

// #region 🛡️Durability

// #region 🛡️Durability
var ErrDurabilityUnsupported = errors.New("store metadata durability unsupported")

type storeFile interface {
	io.Reader
	io.Writer
	Stat() (os.FileInfo, error)
	Sync() error
	Truncate(int64) error
	Close() error
}

type storeOperations interface {
	MkdirAll(string, os.FileMode) error
	Open(string) (storeFile, error)
	OpenFile(string, int, os.FileMode) (storeFile, error)
	ReadFile(string) ([]byte, error)
	Lstat(string) (os.FileInfo, error)
	Stat(string) (os.FileInfo, error)
	Rename(string, string) error
	Remove(string) error
	SyncParent(string) error
	Chtimes(string, time.Time, time.Time) error
}

type nativeStoreOperations struct{}

func (nativeStoreOperations) MkdirAll(path string, mode os.FileMode) error {
	return os.MkdirAll(path, mode)
}

func (nativeStoreOperations) Open(path string) (storeFile, error) {
	return os.Open(path)
}

func (nativeStoreOperations) OpenFile(path string, flag int, mode os.FileMode) (storeFile, error) {
	return os.OpenFile(path, flag, mode)
}

func (nativeStoreOperations) ReadFile(path string) ([]byte, error) {
	return os.ReadFile(path)
}

func (nativeStoreOperations) Lstat(path string) (os.FileInfo, error) {
	return os.Lstat(path)
}

func (nativeStoreOperations) Stat(path string) (os.FileInfo, error) {
	return os.Stat(path)
}

func (nativeStoreOperations) Rename(source string, destination string) error {
	return durability.Rename(source, destination)
}

func (nativeStoreOperations) Remove(path string) error {
	err := os.Remove(path)
	if os.IsNotExist(err) {
		return nil
	}
	return err
}

func (nativeStoreOperations) SyncParent(path string) error {
	parent := filepath.Dir(path)
	if err := durability.SyncParent(parent); err != nil {
		return fmt.Errorf("%w: %s: %v", ErrDurabilityUnsupported, parent, err)
	}
	return nil
}

func (nativeStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	return os.Chtimes(path, access, modified)
}

// 💥️ErrInjectedFault is the failure a store armed for fault injection raises.
var ErrInjectedFault = errors.New("injected durability fault")

// 💥️faultStoreOperations fails the armed durable mutation and passes every other call through.
type faultStoreOperations struct {
	inner storeOperations
	mu    sync.Mutex
	armed int
	seen  int
}

func (operations *faultStoreOperations) arm(at int) {
	operations.mu.Lock()
	operations.armed = at
	operations.seen = 0
	operations.mu.Unlock()
}

func (operations *faultStoreOperations) mutate(path string) error {
	operations.mu.Lock()
	defer operations.mu.Unlock()
	if operations.armed <= 0 || strings.HasSuffix(path, ".lock") {
		return nil
	}
	operations.seen++
	if operations.seen == operations.armed {
		return fmt.Errorf("%w at mutation %d on %s", ErrInjectedFault, operations.seen, path)
	}
	return nil
}

func (operations *faultStoreOperations) MkdirAll(path string, mode os.FileMode) error {
	return operations.inner.MkdirAll(path, mode)
}

func (operations *faultStoreOperations) Open(path string) (storeFile, error) {
	return operations.inner.Open(path)
}

func (operations *faultStoreOperations) OpenFile(path string, flag int, mode os.FileMode) (storeFile, error) {
	if flag&os.O_CREATE != 0 {
		if err := operations.mutate(path); err != nil {
			return nil, err
		}
	}
	return operations.inner.OpenFile(path, flag, mode)
}

func (operations *faultStoreOperations) ReadFile(path string) ([]byte, error) {
	return operations.inner.ReadFile(path)
}

func (operations *faultStoreOperations) Lstat(path string) (os.FileInfo, error) {
	return operations.inner.Lstat(path)
}

func (operations *faultStoreOperations) Stat(path string) (os.FileInfo, error) {
	return operations.inner.Stat(path)
}

func (operations *faultStoreOperations) Rename(source string, destination string) error {
	if err := operations.mutate(destination); err != nil {
		return err
	}
	return operations.inner.Rename(source, destination)
}

func (operations *faultStoreOperations) Remove(path string) error {
	if err := operations.mutate(path); err != nil {
		return err
	}
	return operations.inner.Remove(path)
}

func (operations *faultStoreOperations) SyncParent(path string) error {
	return operations.inner.SyncParent(path)
}

func (operations *faultStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	return operations.inner.Chtimes(path, access, modified)
}

// #endregion 🛡️Durability
