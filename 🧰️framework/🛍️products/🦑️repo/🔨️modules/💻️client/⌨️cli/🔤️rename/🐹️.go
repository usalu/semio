// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Rename tool with sealed taxonomy guards and casing helpers.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"errors"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"unicode/utf8"
)

func applyRenameCasings(content, oldToken, newToken string) string {
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

// 🔏️sealedTaxonomyPath is the repository-relative taxonomy that registers every digest-sealed evidence document; twin of `move.SealedTaxonomyPath`.
const sealedTaxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

// 🧾️sealedEvidence is the slice of the taxonomy that names digest-sealed evidence documents.
type sealedEvidence struct {
	Coordinates map[string]sealedDocument `json:"frozenCoordinateEvidenceContracts"`
	Markdown    map[string]sealedDocument `json:"frozenMarkdownCoordinateEvidenceContracts"`
}

// 📄️sealedDocument is one sealed registration; a retired one names a document that no longer exists.
type sealedDocument struct {
	Path    string          `json:"path"`
	Retired json.RawMessage `json:"retired"`
}

// 🔏️sealedDocumentPaths returns every live digest-sealed evidence document the taxonomy registers; a path-renaming tool never rewrites, moves or relocates one.
func sealedDocumentPaths() (map[string]bool, error) {
	sealed := map[string]bool{}
	data, err := os.ReadFile(filepath.Join(rootDir, filepath.FromSlash(sealedTaxonomyPath)))
	if errors.Is(err, fs.ErrNotExist) {
		return sealed, nil
	}
	if err != nil {
		return nil, err
	}
	var evidence sealedEvidence
	if err := json.Unmarshal(data, &evidence); err != nil {
		return nil, err
	}
	for _, contracts := range []map[string]sealedDocument{evidence.Coordinates, evidence.Markdown} {
		for _, contract := range contracts {
			if len(contract.Retired) == 0 && contract.Path != "" {
				sealed[contract.Path] = true
			}
		}
	}
	return sealed, nil
}

// 🛡️sealedWithin returns the sealed documents at rel or beneath it, byte ordered.
func sealedWithin(sealed map[string]bool, rel string) []string {
	found := []string{}
	for path := range sealed {
		if path == rel || strings.HasPrefix(path, rel+"/") {
			found = append(found, path)
		}
	}
	sort.Strings(found)
	return found
}

// 🚫️refuseSealedMove is the refusal a file or folder move of sealed evidence produces, or nil when the source holds none.
func refuseSealedMove(source string) *ToolResult {
	sealed, err := sealedDocumentPaths()
	if err != nil {
		result := toolErrorResult(err)
		return &result
	}
	if found := sealedWithin(sealed, strings.Trim(filepath.ToSlash(filepath.Clean(source)), "/")); len(found) > 0 {
		result := toolErrorMsg(fmt.Sprintf("Refusing to move digest-sealed evidence: %s", strings.Join(found, ", ")))
		return &result
	}
	return nil
}

// 🔤️ToolRename rewrites every UPPER/Title/lower variant of oldToken to newToken across non-gitignored file contents and filenames, keeping digest-sealed evidence untouched. When scope is non-empty the walk is restricted to rootDir/scope.
func ToolRename(oldToken, newToken, scope string) ToolResult {
	output := NewOutput()
	if oldToken == "" || newToken == "" {
		return toolErrorMsg("Old and new token must be non-empty")
	}
	if strings.EqualFold(oldToken, newToken) {
		return toolErrorMsg("Old and new token are identical")
	}
	walkRoot := rootDir
	scope = strings.Trim(filepath.ToSlash(scope), "/")
	if scope != "" {
		walkRoot = filepath.Join(rootDir, filepath.FromSlash(scope))
		info, statErr := os.Stat(walkRoot)
		if statErr != nil || !info.IsDir() {
			return toolErrorMsg(fmt.Sprintf("Scope is not a directory: %s", scope))
		}
	}
	var files []string
	var dirs []string
	err := filepath.WalkDir(walkRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		rel, relErr := filepath.Rel(rootDir, path)
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
		if isGitIgnored(rel) {
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
		return toolErrorResult(err)
	}
	sealed, err := sealedDocumentPaths()
	if err != nil {
		return toolErrorResult(err)
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
		if base := filepath.Base(e.rel); applyRenameCasings(base, oldToken, newToken) != base {
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
		return toolErrorMsg(fmt.Sprintf("Rename would relocate digest-sealed evidence: %s", strings.Join(relocated, ", ")))
	}
	filesChanged := 0
	filesSealed := 0
	for _, rel := range files {
		abs := filepath.Join(rootDir, filepath.FromSlash(rel))
		data, readErr := os.ReadFile(abs)
		if readErr != nil {
			continue
		}
		if !utf8.Valid(data) {
			continue
		}
		original := string(data)
		replaced := applyRenameCasings(original, oldToken, newToken)
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
			return toolErrorResult(writeErr)
		}
		filesChanged++
	}
	filesRenamed := 0
	foldersRenamed := 0
	for _, e := range entries {
		base := filepath.Base(e.rel)
		newBase := applyRenameCasings(base, oldToken, newToken)
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
		absSrc := filepath.Join(rootDir, filepath.FromSlash(e.rel))
		absDst := filepath.Join(rootDir, filepath.FromSlash(newRel))
		if _, statErr := os.Stat(absDst); statErr == nil {
			return toolErrorMsg(fmt.Sprintf("Rename target already exists: %s", newRel))
		}
		if err := EnsureDir(filepath.Dir(absDst)); err != nil {
			return toolErrorResult(err)
		}
		if err := os.Rename(absSrc, absDst); err != nil {
			return toolErrorResult(err)
		}
		if e.isDir {
			foldersRenamed++
		} else {
			filesRenamed++
		}
	}
	output.Success(fmt.Sprintf("\n🔤️Renamed %s → %s: %d files edited, %d files renamed, %d folders renamed, %d sealed files kept", oldToken, newToken, filesChanged, filesRenamed, foldersRenamed, filesSealed))
	return ToolResult{Output: *output, Data: map[string]int{
		"filesChanged":   filesChanged,
		"filesRenamed":   filesRenamed,
		"filesSealed":    filesSealed,
		"foldersRenamed": foldersRenamed,
	}}
}

// 🔤️renameCommand returns a command command that renames a token across the repo in all case variants.
func renameCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "rename <old> <new> [scope]",
		Short: "Rename a token across non-gitignored files (all case variants)",
		Long:  "Rewrite UPPER, Title and lower case variants of <old> to <new> in every non-gitignored file's contents and filenames (including folder names) under the optional scope directory (default: repo root). Example: repo rename model representation compose.",
		Args:  command.RangeArgs(2, 3),
		RunE: func(cmd *command.Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}
			scope := ""
			if len(args) == 3 {
				scope = args[2]
			}
			result := ToolRename(args[0], args[1], scope)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
}
