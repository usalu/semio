// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Oversized ticket artifact purging.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"

)

const (
	ticketOversizedFileBytes   = 5 << 20
	ticketOversizedFolderBytes = 10 << 20
)

// 🧺️purgeOversizedTicketArtifacts removes only oversized direct generated ticket output and preserves authored inputs.
func purgeOversizedTicketArtifacts(ticketDir string) error {
	ticketDir = strings.TrimSpace(ticketDir)
	if ticketDir == "" {
		return nil
	}
	absDir, err := filepath.Abs(ticketDir)
	if err != nil {
		return err
	}
	info, err := os.Stat(absDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return err
	}
	if !info.IsDir() {
		return fmt.Errorf("ticket path is not a directory: %s", absDir)
	}

	absDir = filepath.Join(absDir, "🗑️generated")
	info, err = os.Lstat(absDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return err
	}
	if info.Mode()&fs.ModeSymlink != 0 {
		return nil
	}
	if !info.IsDir() {
		return fmt.Errorf("generated ticket output is not a directory: %s", absDir)
	}

	type fileEntry struct {
		path string
		size int64
	}
	dirSizes := make(map[string]int64)
	var dirs []string
	var files []fileEntry

	walkErr := filepath.WalkDir(absDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			writeWarningf("Failed to walk ticket folder %s: %v", path, err)
			return nil
		}
		if !ticketPathContained(absDir, path) {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.Type()&fs.ModeSymlink != 0 {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.IsDir() {
			if path != absDir {
				dirs = append(dirs, path)
			}
			return nil
		}
		fi, err := os.Lstat(path)
		if err != nil {
			writeWarningf("Failed to stat ticket artifact %s: %v", path, err)
			return nil
		}
		if fi.Mode()&fs.ModeSymlink != 0 {
			return nil
		}
		size := fi.Size()
		files = append(files, fileEntry{path: path, size: size})
		for parent := filepath.Dir(path); ticketPathContained(absDir, parent); parent = filepath.Dir(parent) {
			dirSizes[parent] += size
			if parent == absDir {
				break
			}
		}
		return nil
	})
	if walkErr != nil {
		return walkErr
	}

	sort.Slice(dirs, func(i, j int) bool {
		return strings.Count(dirs[i], string(os.PathSeparator)) > strings.Count(dirs[j], string(os.PathSeparator))
	})

	deleted := make(map[string]bool)
	for _, dir := range dirs {
		if ticketArtifactUnderDeleted(dir, deleted) {
			continue
		}
		if dirSizes[dir] <= ticketOversizedFolderBytes {
			continue
		}
		if err := os.RemoveAll(dir); err != nil {
			writeWarningf("Failed to delete oversized ticket folder %s: %v", dir, err)
			continue
		}
		deleted[dir] = true
	}

	ticketJSONBase := "🎫️ticket.json"
	for _, file := range files {
		if ticketArtifactUnderDeleted(file.path, deleted) {
			continue
		}
		if filepath.Base(file.path) == ticketJSONBase {
			continue
		}
		if file.size <= ticketOversizedFileBytes {
			continue
		}
		if err := os.Remove(file.path); err != nil {
			writeWarningf("Failed to delete oversized ticket file %s: %v", file.path, err)
		}
	}
	return nil
}

// 🧹️collectTicketFolderRoots returns every slug directory under the dated ticket tree.
func collectTicketFolderRoots() ([]string, error) {
	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil, nil
	}
	var roots []string
	yearEntries, err := os.ReadDir(ticketsDir)
	if err != nil {
		return nil, err
	}
	for _, yearEntry := range yearEntries {
		if !yearEntry.IsDir() {
			continue
		}
		yearPath := filepath.Join(ticketsDir, yearEntry.Name())
		monthEntries, err := os.ReadDir(yearPath)
		if err != nil {
			continue
		}
		for _, monthEntry := range monthEntries {
			if !monthEntry.IsDir() {
				continue
			}
			monthPath := filepath.Join(yearPath, monthEntry.Name())
			dayEntries, err := os.ReadDir(monthPath)
			if err != nil {
				continue
			}
			for _, dayEntry := range dayEntries {
				if !dayEntry.IsDir() {
					continue
				}
				dayPath := filepath.Join(monthPath, dayEntry.Name())
				slugEntries, err := os.ReadDir(dayPath)
				if err != nil {
					continue
				}
				for _, slugEntry := range slugEntries {
					if !slugEntry.IsDir() {
						continue
					}
					if strings.HasPrefix(slugEntry.Name(), ".") {
						continue
					}
					roots = append(roots, filepath.Join(dayPath, slugEntry.Name()))
				}
			}
		}
	}
	return roots, nil
}

// 🧹️PurgeAllOversizedTicketArtifacts deletes oversized artifacts in every ticket folder.
func PurgeAllOversizedTicketArtifacts() (int, error) {
	roots, err := collectTicketFolderRoots()
	if err != nil {
		return 0, err
	}
	purged := 0
	for _, root := range roots {
		if err := purgeOversizedTicketArtifacts(root); err != nil {
			return purged, err
		}
		purged++
	}
	return purged, nil
}

// 🛡️ticketPathContained reports whether path stays inside the ticket root directory.
func ticketPathContained(root, path string) bool {
	rel, err := filepath.Rel(root, path)
	if err != nil {
		return false
	}
	return rel != ".." && !strings.HasPrefix(rel, ".."+string(filepath.Separator))
}

// 🗑️ticketArtifactUnderDeleted reports whether path lies inside a deleted ticket subtree.
func ticketArtifactUnderDeleted(path string, deleted map[string]bool) bool {
	for dir := range deleted {
		if path == dir || strings.HasPrefix(path, dir+string(os.PathSeparator)) {
			return true
		}
	}
	return false
}
