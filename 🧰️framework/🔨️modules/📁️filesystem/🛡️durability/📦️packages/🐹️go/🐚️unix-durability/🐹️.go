//go:build !windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unix parent-directory durability for filesystem metadata changes.

// #endregion 🧲️Header

package durability

import "os"

/** 🛡️ Replaces a directory entry with platform durability semantics. */
func Rename(source string, destination string) error {
	return os.Rename(source, destination)
}

/** 💾️ Persists the parent directory after a metadata change. */
func SyncParent(path string) error {
	directory, err := os.Open(path)
	if err != nil {
		return err
	}
	defer directory.Close()
	return directory.Sync()
}
