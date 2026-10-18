// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Scope resolution and ticket file computation.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

// ❌️ScopeToFiles MUST return a non-nil error when the operation fails.
func ScopeToFiles(scope Scope, bundles []Bundle) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case ScopeRepo:
		files, err = globByExtension(rootDir, "**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
	case ScopeTechnology:
		for _, proj := range bundles {
			if proj.Name == scope.TechnologyName {
				files, err = globByExtension(rootDir, proj.Root+"/**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
				break
			}
		}
	case ScopeFolder:
		if scope.FilePath != "" {
			files, err = globByExtension(rootDir, scope.FilePath+"**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
		}
	case ScopeFile, ScopeSection, ScopeDefinition:
		if scope.FilePath != "" {
			files = []string{scope.FilePath}
		}
	}
	if err != nil {
		return nil, err
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

func normalizeRepoPath(path string) string {
	normalized := NormalizePath(path)
	if filepath.IsAbs(path) {
		normalized = GetRelativePath(path)
	}
	normalized = strings.TrimPrefix(normalized, "./")
	return normalized
}

// 💿️isRepoExcludedPath holds the data fields for a isRepoExcludedPath record.
func isRepoExcludedPath(path string) bool {
	normalized := normalizeRepoPath(path)
	if normalized == "" {
		return false
	}
	if normalized == ".🧬semio" || strings.HasPrefix(normalized, ".🧬semio/") {
		return true
	}
	if normalized == "assets/repo" || strings.HasPrefix(normalized, "assets/repo/") || strings.Contains(normalized, "/asset/repo/") {
		return true
	}
	if normalized == "node_modules" || strings.HasPrefix(normalized, "node_modules/") ||
		strings.Contains(normalized, "/node_modules/") {
		return true
	}
	if normalized == ".git" || strings.HasPrefix(normalized, ".git/") || strings.Contains(normalized, "/.git/") {
		return true
	}
	for _, segment := range []string{"/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"} {
		if strings.Contains(normalized, segment) {
			return true
		}
	}
	base := filepath.Base(normalized)
	if strings.HasSuffix(base, ".Designer.cs") {
		return true
	}
	if strings.Contains(normalized, "/codegen/") {
		return true
	}
	return false
}

// 🧹️filterConsideredFiles holds the data fields for a filterConsideredFiles record.
func filterConsideredFiles(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		if isRepoExcludedPath(filePath) {
			continue
		}
		if isGitIgnored(filePath) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

// 🌳️GitIndexRef is the git ref for the staging index. Used for unstaged-only diffs (index vs working tree).
// ⏹️Requirements: ticket close and interaction finish use only unstaged diffs; git diff runs without tree-ish for index vs working tree.
const GitIndexRef = ":0"

// 🎫️ComputeTicketFiles MUST return the computed result deterministically.
// ComputeTicketFiles computes and returns the ticket files.
// 💾️Uses unstaged diffs only (index vs working tree) for complete, current working state.
func ComputeTicketFiles(ticket *Ticket, files []string) (*TicketDiffs, error) {
	files = normalizeTicketFileInputs(files)
	files = FilterTicketWorkspaceFiles(ticket, files)
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}
	baseRef := GitIndexRef
	diffStatuses, err := GetGitDiffStatus(baseRef, "", files)
	if err != nil {
		return nil, err
	}
	diffLines, err := GetGitDiffLines(baseRef, "", files)
	if err != nil {
		return nil, err
	}

	currentFiles := make(map[string]struct{})
	baseFiles := make(map[string]struct{})
	for _, file := range files {
		currentFiles[file] = struct{}{}
		baseFiles[file] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.To != "" {
			currentFiles[status.To] = struct{}{}
		}
		if status.From != "" {
			baseFiles[status.From] = struct{}{}
		}
	}
	var currentFileList []string
	for file := range currentFiles {
		currentFileList = append(currentFileList, file)
	}
	var baseFileList []string
	for file := range baseFiles {
		baseFileList = append(baseFileList, file)
	}

	bundles := GetTechnologies()
	baseCodebase, err := BuildCodebaseSnapshot(baseFileList, bundles, baseRef)
	if err != nil {
		return nil, err
	}
	currentCodebase, err := BuildCodebaseSnapshot(currentFileList, bundles, "")
	if err != nil {
		return nil, err
	}
	result := BuildSemanticDiffs(baseCodebase, currentCodebase, baseRef, diffLines, diffStatuses, bundles)
	return result, nil
}

// 📨️GetGitDiffLines MUST retrieve the requested value or return an error.
// GetGitDiffLines retrieves and returns the git diff lines.
// ✔️For unstaged-only diffs use baseCheckpoint GitIndexRef (index vs working tree).
func GetGitDiffLines(baseCheckpoint, headCheckpoint string, paths []string) (map[string]*DiffLines, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := BuildGitDiffArgs("-U0", baseCheckpoint, headCheckpoint, paths)
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff failed: %s", strings.TrimSpace(stderr))
	}
	result := make(map[string]*DiffLines)
	var currentFile string
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Fields(line)
			if len(parts) >= 4 {
				newPath := strings.TrimPrefix(parts[3], "b/")
				currentFile = newPath
				if result[currentFile] == nil {
					result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
				}
			}
		} else if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

				newStart, _ := strconv.Atoi(match[3])
				newCount := 1
				if match[4] != "" {
					newCount, _ = strconv.Atoi(match[4])
				}
				for i := 0; i < newCount; i++ {
					result[currentFile].Added = append(result[currentFile].Added, newStart+i)
				}
			}
		}
	}
	return result, nil
}

// 🏗️buildBreachID holds the data fields for a buildBreachID record.
func buildBreachID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("repo/breach/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("repo/breach/%s#%d", scope, line)
	}
	return fmt.Sprintf("repo/breach/%s", scope)
}

// 📪️CanCloseTicket MUST return a deterministic boolean result.
func CanCloseTicket(ticket *Ticket) (bool, []string) {
	var reasons []string
	if ticket == nil {
		reasons = append(reasons, "Ticket data is nil")
		return false, reasons
	}
	return len(reasons) == 0, reasons
}
