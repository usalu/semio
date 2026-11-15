// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️fleet-store looks a pinned artifact up by its SHA-256 in the group store that the worktree names in `.cache/f/machine.json` and publishes a verified download there, so that the first machine of the fleet pays the download and the others copy it from the local network.

// #endregion 🧲️Header

package launcher

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io"
	"os"
	"path/filepath"
	"time"
)

// #region 🛰️Fleet

const fleetStoreTimeout = 20 * time.Second

// 🗂️fleetStoreDirectory answers the group store of a worktree, or "" when the fleet tier is off (`SEMIO_FLEET_FETCH=off`) or the machine has no store.
func fleetStoreDirectory(root string, env Environment) string {
	if env("SEMIO_FLEET_FETCH") == "off" {
		return ""
	}
	if named := env("SEMIO_FLEET_STORE_ROOT"); named != "" {
		return named
	}
	data, err := os.ReadFile(filepath.Join(root, ".cache", "f", "machine.json"))
	if err != nil {
		return ""
	}
	var machine struct {
		StoreRoot string `json:"storeRoot"`
	}
	if json.Unmarshal(data, &machine) != nil || !filepath.IsAbs(machine.StoreRoot) {
		return ""
	}
	return machine.StoreRoot
}

// 🗂️fleetBlobPath is `<store>/v1/fleet/fetch/<first two hex>/<sha256>`, the same place the TypeScript fetch layer uses.
func fleetBlobPath(store string, sha string) string {
	return filepath.Join(store, "v1", "fleet", "fetch", sha[:2], sha)
}

// ⏱️fleetWithin runs work and gives up on it after the timeout, because a stalled network file system must never stall a launch.
func fleetWithin(timeout time.Duration, work func() bool) bool {
	done := make(chan bool, 1)
	go func() { done <- work() }()
	select {
	case ok := <-done:
		return ok
	case <-time.After(timeout):
		return false
	}
}

// 📥️fleetFetchInto copies the pinned blob from the store to destination and keeps it only when size and SHA-256 match the pin.
func fleetFetchInto(store string, row Distribution, destination string) bool {
	if store == "" || len(row.Sha256) != 64 {
		return false
	}
	return fleetWithin(fleetStoreTimeout, func() bool {
		source, err := os.Open(fleetBlobPath(store, row.Sha256))
		if err != nil {
			return false
		}
		defer source.Close()
		target, err := os.OpenFile(destination, os.O_CREATE|os.O_TRUNC|os.O_WRONLY, 0o600)
		if err != nil {
			return false
		}
		hash := sha256.New()
		size, copyFailure := io.Copy(io.MultiWriter(target, hash), source)
		closeFailure := target.Close()
		if copyFailure != nil || closeFailure != nil || size != row.Bytes || hex.EncodeToString(hash.Sum(nil)) != row.Sha256 {
			os.Remove(destination)
			return false
		}
		return true
	})
}

// 📤️fleetPublish copies a verified archive into the store under its SHA-256 by temporary file and rename; a failure is silent because the launch has already succeeded.
func fleetPublish(store string, row Distribution, source string) {
	if store == "" || len(row.Sha256) != 64 {
		return
	}
	fleetWithin(fleetStoreTimeout*3, func() bool {
		target := fleetBlobPath(store, row.Sha256)
		if _, err := os.Stat(target); err == nil {
			return true
		}
		if os.MkdirAll(filepath.Dir(target), 0o755) != nil {
			return false
		}
		input, err := os.Open(source)
		if err != nil {
			return false
		}
		defer input.Close()
		temporary, err := os.CreateTemp(filepath.Dir(target), ".fleet-*.tmp")
		if err != nil {
			return false
		}
		_, copyFailure := io.Copy(temporary, input)
		closeFailure := temporary.Close()
		if copyFailure != nil || closeFailure != nil || os.Chmod(temporary.Name(), 0o644) != nil || os.Rename(temporary.Name(), target) != nil {
			os.Remove(temporary.Name())
			return false
		}
		return true
	})
}

// #endregion 🛰️Fleet
