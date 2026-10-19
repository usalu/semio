// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔤️rename carries the rename tool of the move domain with the casing helpers and the file and directory copy helpers.

// #endregion 🧲️Header

package move

import (
	fmt "fmt"
	io "io"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
	utf8 "unicode/utf8"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔤️Rename

// 🔤️applyRenameCasings rewrites UPPER, Title and lower case variants of old→new in the input string.
func ApplyRenameCasings(content, oldToken, newToken string) string {
	oldUpper := strings.ToUpper(oldToken)
	newUpper := strings.ToUpper(newToken)
	oldLower := strings.ToLower(oldToken)
	newLower := strings.ToLower(newToken)
	oldTitle := titleCaseToken(oldLower)
	newTitle := titleCaseToken(newLower)
	out := content
	if oldUpper != oldLower {
		out = strings.ReplaceAll(out, oldUpper, newUpper)
	}
	if oldTitle != oldUpper && oldTitle != oldLower {
		out = strings.ReplaceAll(out, oldTitle, newTitle)
	}
	out = strings.ReplaceAll(out, oldLower, newLower)
	return out
}

// 🅰️titleCaseToken returns s with the first rune upper-cased and the rest lower-cased.
func titleCaseToken(s string) string {
	if s == "" {
		return s
	}
	return strings.ToUpper(s[:1]) + strings.ToLower(s[1:])
}

// #endregion 🔤️Rename

// #region ⏰️File Utilities

// 🚚️MoveFile MUST return a non-nil error when the operation fails.
func MoveFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	outputFile, err := os.Create(destPath)
	if err != nil {
		inputFile.Close()
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	inputFile.Close()
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}

	err = os.Remove(sourcePath)
	if err != nil {
		return fmt.Errorf("failed removing original file: %v", err)
	}
	return nil
}

// ❌️CopyFile MUST return a non-nil error when the operation fails.
func CopyFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	defer inputFile.Close()
	outputFile, err := os.Create(destPath)
	if err != nil {
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}
	return nil
}

// 🔤️ToolRename rewrites every UPPER/Title/lower variant of oldToken to newToken across non-gitignored file contents and filenames. When scope is non-empty the walk is restricted to rootDir/scope.
func ToolRename(oldToken, newToken, scope string) workspace.ToolResult {
	output := workspace.NewOutput()
	if oldToken == "" || newToken == "" {
		return workspace.ToolErrorMsg("Old and new token must be non-empty")
	}
	if strings.EqualFold(oldToken, newToken) {
		return workspace.ToolErrorMsg("Old and new token are identical")
	}
	walkRoot := workspace.GetRootDir()
	scope = strings.Trim(filepath.ToSlash(scope), "/")
	if scope != "" {
		walkRoot = filepath.Join(workspace.GetRootDir(), filepath.FromSlash(scope))
		info, statErr := os.Stat(walkRoot)
		if statErr != nil || !info.IsDir() {
			return workspace.ToolErrorMsg(fmt.Sprintf("Scope is not a directory: %s", scope))
		}
	}
	var files []string
	var dirs []string
	err := filepath.WalkDir(walkRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		rel, relErr := filepath.Rel(workspace.GetRootDir(), path)
		if relErr != nil {
			return nil
		}
		rel = filepath.ToSlash(rel)
		if rel == "." {
			return nil
		}
		base := filepath.Base(rel)
		if d.IsDir() && (base == ".git" || base == "node_modules") {
			return filepath.SkipDir
		}
		if workspace.IsGitIgnored(rel) {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if d.IsDir() {
			dirs = append(dirs, rel)
		} else {
			files = append(files, rel)
		}
		return nil
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sealed, err := SealedDocumentPaths()
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	type renameEntry struct {
		rel   string
		isDir bool
	}
	entries := make([]renameEntry, 0, len(files)+len(dirs))
	for _, f := range files {
		entries = append(entries, renameEntry{f, false})
	}
	for _, d := range dirs {
		entries = append(entries, renameEntry{d, true})
	}
	sort.Slice(entries, func(i, j int) bool {
		di := strings.Count(entries[i].rel, "/")
		dj := strings.Count(entries[j].rel, "/")
		if di != dj {
			return di > dj
		}
		return entries[i].rel > entries[j].rel
	})
	relocated, seen := []string{}, map[string]bool{}
	for _, e := range entries {
		if base := filepath.Base(e.rel); ApplyRenameCasings(base, oldToken, newToken) != base {
			for _, path := range sealedWithin(sealed, e.rel) {
				if !seen[path] {
					seen[path] = true
					relocated = append(relocated, path)
				}
			}
		}
	}
	if len(relocated) > 0 {
		sort.Strings(relocated)
		return workspace.ToolErrorMsg(fmt.Sprintf("Rename would relocate digest-sealed evidence: %s", strings.Join(relocated, ", ")))
	}
	filesChanged := 0
	filesSealed := 0
	for _, rel := range files {
		abs := filepath.Join(workspace.GetRootDir(), filepath.FromSlash(rel))
		data, readErr := os.ReadFile(abs)
		if readErr != nil {
			continue
		}
		if !utf8.Valid(data) {
			continue
		}
		original := string(data)
		replaced := ApplyRenameCasings(original, oldToken, newToken)
		if replaced == original {
			continue
		}
		if sealed[rel] {
			filesSealed++
			continue
		}
		info, statErr := os.Stat(abs)
		mode := os.FileMode(0644)
		if statErr == nil {
			mode = info.Mode().Perm()
		}
		if writeErr := os.WriteFile(abs, []byte(replaced), mode); writeErr != nil {
			return workspace.ToolErrorResult(writeErr)
		}
		filesChanged++
	}
	filesRenamed := 0
	foldersRenamed := 0
	for _, e := range entries {
		base := filepath.Base(e.rel)
		newBase := ApplyRenameCasings(base, oldToken, newToken)
		if newBase == base {
			continue
		}
		parent := filepath.Dir(e.rel)
		var newRel string
		if parent == "." || parent == "" {
			newRel = newBase
		} else {
			newRel = parent + "/" + newBase
		}
		absSrc := filepath.Join(workspace.GetRootDir(), filepath.FromSlash(e.rel))
		absDst := filepath.Join(workspace.GetRootDir(), filepath.FromSlash(newRel))
		if _, statErr := os.Stat(absDst); statErr == nil {
			return workspace.ToolErrorMsg(fmt.Sprintf("Rename target already exists: %s", newRel))
		}
		if err := workspace.EnsureDir(filepath.Dir(absDst)); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if err := os.Rename(absSrc, absDst); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if e.isDir {
			foldersRenamed++
		} else {
			filesRenamed++
		}
	}
	output.Success(fmt.Sprintf("\n🔤️Renamed %s → %s: %d files edited, %d files renamed, %d folders renamed, %d sealed files kept", oldToken, newToken, filesChanged, filesRenamed, foldersRenamed, filesSealed))
	return workspace.ToolResult{Output: *output, Data: map[string]int{
		"filesChanged":   filesChanged,
		"filesRenamed":   filesRenamed,
		"filesSealed":    filesSealed,
		"foldersRenamed": foldersRenamed,
	}}
}

// #endregion ⏰️File Utilities

// #region 🔖️Missing Hook Functions

func CopyDirTree(srcRoot, dstRoot string) error {
	return filepath.WalkDir(srcRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(srcRoot, path)
		if err != nil {
			return err
		}
		if rel == "." {
			return os.MkdirAll(dstRoot, 0o755)
		}
		target := filepath.Join(dstRoot, rel)
		if d.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		return CopyFile(path, target)
	})
}

// #endregion 🔖️Missing Hook Functions
