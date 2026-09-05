//go:build !windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unix parent-directory durability for coordinator metadata changes.

// #endregion 🧲️Header

package main

import "os"

func renameStorePath(source string, destination string) error {
	return os.Rename(source, destination)
}

func syncStoreParent(path string) error {
	directory, err := os.Open(path)
	if err != nil {
		return err
	}
	defer directory.Close()
	return directory.Sync()
}
