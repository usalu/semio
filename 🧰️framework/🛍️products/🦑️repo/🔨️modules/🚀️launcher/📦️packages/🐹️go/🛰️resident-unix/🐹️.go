//go:build !windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident-unix dials the service over a unix socket and refuses a runtime directory that is not private and owned by the caller.

// #endregion 🧲️Header

package launcher

import (
	"io"
	"net"
	"os"
	"path/filepath"
	"syscall"
	"time"
)

// #region 🛰️ResidentUnix

const residentGOOS = "unix"

func residentDialEndpoint(endpoint string) (io.ReadWriteCloser, error) {
	return net.DialTimeout("unix", endpoint, 250*time.Millisecond)
}

// 🔐️residentTrusted accepts the runtime directory of a worktree only when it is a real directory (no link) of the caller that no one else can enter, and its parent `semio-repo-<uid>` is a real directory of the caller that no one else can write: under the shared temporary directory the parent is created by whoever comes first, and a parent of another user could swap the private directory for its own after this check.
func residentTrusted(directory string) bool {
	return residentOwnedDirectory(directory, 0o077) && residentOwnedDirectory(filepath.Dir(directory), 0o022)
}

// 🔐️residentOwnedDirectory is true for a directory (not a link) of the calling user whose permission bits do not include any of `forbidden`.
func residentOwnedDirectory(directory string, forbidden os.FileMode) bool {
	info, err := os.Lstat(directory)
	if err != nil || !info.IsDir() {
		return false
	}
	status, ok := info.Sys().(*syscall.Stat_t)
	return ok && int(status.Uid) == os.Getuid() && info.Mode().Perm()&forbidden == 0
}

// #endregion 🛰️ResidentUnix
