// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚚️move is the move domain of the semio repository tooling: the sealed evidence guard and the folder and file tools.

// #endregion 🧲️Header

package move

import (
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	events "github.com/usalu/semio/repo/events"
	languages "github.com/usalu/semio/repo/languages"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔏️Sealed

// 🔏️SealedTaxonomyPath is the repository-relative taxonomy that registers every digest-sealed evidence document.
const SealedTaxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

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

// 🔏️SealedDocumentPaths returns every live digest-sealed evidence document the workspace taxonomy registers; a path-renaming tool never rewrites, moves or relocates one.
func SealedDocumentPaths() (map[string]bool, error) {
	sealed := map[string]bool{}
	data, err := os.ReadFile(filepath.Join(workspace.GetRootDir(), filepath.FromSlash(SealedTaxonomyPath)))
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
func refuseSealedMove(source string) *workspace.ToolResult {
	sealed, err := SealedDocumentPaths()
	if err != nil {
		result := workspace.ToolErrorResult(err)
		return &result
	}
	if found := sealedWithin(sealed, strings.Trim(filepath.ToSlash(filepath.Clean(source)), "/")); len(found) > 0 {
		result := workspace.ToolErrorMsg(fmt.Sprintf("Refusing to move digest-sealed evidence: %s", strings.Join(found, ", ")))
		return &result
	}
	return nil
}

// #endregion 🔏️Sealed

// #region 📋️Tickets

// 🔖️ToolFolderCreate MUST complete the operation successfully.
func ToolFolderCreate(path string) workspace.ToolResult {
	events.Emit(events.EventFolderCreateStarting, "repo-cli", events.FolderPayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), path)
	if workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder already exists: %s", path))
	}
	if err := workspace.EnsureDir(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFolderCreateEnded, "repo-cli", events.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n📁️Created folder: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFolderMove MUST complete the operation successfully.
func ToolFolderMove(source, target string) workspace.ToolResult {
	events.Emit(events.EventFolderMoveStarting, "repo-cli", events.FolderPayload{Path: target, From: source})
	output := workspace.NewOutput()
	absSource := filepath.Join(workspace.GetRootDir(), source)
	absTarget := filepath.Join(workspace.GetRootDir(), target)
	if !workspace.FileExists(absSource) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source folder not found: %s", source))
	}
	if workspace.FileExists(absTarget) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Target folder already exists: %s", target))
	}
	if refusal := refuseSealedMove(source); refusal != nil {
		return *refusal
	}
	if err := workspace.EnsureDir(filepath.Dir(absTarget)); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return workspace.ToolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	events.Emit(events.EventFolderMoveEnded, "repo-cli", events.FolderPayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📁️Moved folder: %s → %s", source, target))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFolderDelete MUST complete the operation successfully.
func ToolFolderDelete(path string) workspace.ToolResult {
	events.Emit(events.EventFolderDeleteStarting, "repo-cli", events.FolderPayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), path)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	if err := os.RemoveAll(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFolderDeleteEnded, "repo-cli", events.FolderPayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted folder: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileCreate MUST complete the operation successfully.
func ToolFileCreate(path string) workspace.ToolResult {
	events.Emit(events.EventFileCreateStarting, "repo-cli", events.FilePayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), path)
	if workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File already exists: %s", path))
	}
	language := languages.GetLanguage(path)
	content := GenerateFileHeader(path, language)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFileCreateEnded, "repo-cli", events.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n📄️Created file: %s", path))
	return workspace.ToolResult{Output: *output}
}

// 🔖️generateFileHeader holds the data fields for a generateFileHeader record.
func GenerateFileHeader(path string, language languages.LanguagePlugin) string {
	if language == nil || !language.SupportsHeaders() {
		return ""
	}
	gitAuthor := workspace.GetGitAuthor()
	year := strconv.Itoa(time.Now().Year())
	contributors := year + " " + gitAuthor
	return language.FormatHeader(codebase.FileHeaderId(path), codebase.FileHeaderUri(path), "", contributors, workspace.AGPLLicenseText(), "")
}

// 🔖️ToolFileMove MUST complete the operation successfully.
func ToolFileMove(source, target string) workspace.ToolResult {
	events.Emit(events.EventFileMoveStarting, "repo-cli", events.FilePayload{Path: target, From: source})
	output := workspace.NewOutput()
	absSource := filepath.Join(workspace.GetRootDir(), source)
	absTarget := filepath.Join(workspace.GetRootDir(), target)
	if !workspace.FileExists(absSource) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Source file not found: %s", source))
	}
	if workspace.FileExists(absTarget) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Target file already exists: %s", target))
	}
	if refusal := refuseSealedMove(source); refusal != nil {
		return *refusal
	}
	if err := workspace.EnsureDir(filepath.Dir(absTarget)); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		return workspace.ToolErrorResult(err)
	}
	UpdateAgentsDocsPath(source, target)
	events.Emit(events.EventFileMoveEnded, "repo-cli", events.FilePayload{Path: target, From: source})
	output.Success(fmt.Sprintf("\n📄️Moved file: %s → %s", source, target))
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileDelete MUST complete the operation successfully.
func ToolFileDelete(path string) workspace.ToolResult {
	events.Emit(events.EventFileDeleteStarting, "repo-cli", events.FilePayload{Path: path})
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.GetRootDir(), path)
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", path))
	}
	if err := os.Remove(absPath); err != nil {
		return workspace.ToolErrorResult(err)
	}
	events.Emit(events.EventFileDeleteEnded, "repo-cli", events.FilePayload{Path: path})
	output.Success(fmt.Sprintf("\n🗑️ Deleted file: %s", path))
	return workspace.ToolResult{Output: *output}
}

// #endregion 📋️Tickets
