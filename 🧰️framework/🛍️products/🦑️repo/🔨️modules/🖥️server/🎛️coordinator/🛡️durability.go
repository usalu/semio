// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Owned cross-platform metadata durability operations for the coordinator store.

// #endregion 🧲️Header

package main

import (
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"time"
)

// #region 💾️Contract

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
	return renameStorePath(source, destination)
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
	if err := syncStoreParent(parent); err != nil {
		return fmt.Errorf("%w: %s: %v", ErrDurabilityUnsupported, parent, err)
	}
	return nil
}

func (nativeStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	return os.Chtimes(path, access, modified)
}

// #endregion 💾️Contract
