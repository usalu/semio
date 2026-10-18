package tickets

import (
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 📐️File Scope

// 📏️DiffLines is how many lines one file gained and lost.
type DiffLines struct {
	Added   int `json:"added"`
	Removed int `json:"removed"`
}

// 🔢️GitIndexRef is the reference a ticket diff is taken against.
const GitIndexRef = ":0"

// 🔧️VersionControl is everything the ticket domain asks of version control.
type VersionControl interface {
	// 🙈️IsIgnored reports whether a path is ignored.
	IsIgnored(path string) bool
	// 📇️NameStatus returns the name-status output of a diff.
	NameStatus(ref, against string, files []string) (string, error)
	// 📄️UnifiedDiff returns the unified diff output.
	UnifiedDiff(ref, against string, files []string) (string, error)
}

// 📼️RecordedVersionControl answers from a frozen recording.
type RecordedVersionControl struct {
	Ignored        []string `json:"ignored"`
	NameStatusOut  string   `json:"nameStatus"`
	UnifiedDiffOut string   `json:"unifiedDiff"`
}

// 🙈️IsIgnored reports whether a path was recorded as ignored.
func (control RecordedVersionControl) IsIgnored(path string) bool {
	for _, candidate := range control.Ignored {
		if candidate == path {
			return true
		}
	}
	return false
}

// 📇️NameStatus returns the recorded name-status output.
func (control RecordedVersionControl) NameStatus(string, string, []string) (string, error) {
	return control.NameStatusOut, nil
}

// 📄️UnifiedDiff returns the recorded unified diff output.
func (control RecordedVersionControl) UnifiedDiff(string, string, []string) (string, error) {
	return control.UnifiedDiffOut, nil
}

// 🧹️IsRepoExcludedPath reports whether a path is excluded from every ticket file scope.
func IsRepoExcludedPath(path string) bool {
	normalized := strings.TrimPrefix(NormalizeSeparators(strings.TrimSpace(path)), "./")
	if normalized == "" {
		return false
	}
	under := func(root string) bool {
		return normalized == root || strings.HasPrefix(normalized, root+"/")
	}
	if under(".🧬semio") || under("assets/repo") || strings.Contains(normalized, "/asset/repo/") {
		return true
	}
	if under("node_modules") || strings.Contains(normalized, "/node_modules/") {
		return true
	}
	if under(".git") || strings.Contains(normalized, "/.git/") {
		return true
	}
	for _, segment := range []string{"/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"} {
		if strings.Contains(normalized, segment) {
			return true
		}
	}
	return strings.HasSuffix(BaseName(normalized), ".Designer.cs") || strings.Contains(normalized, "/codegen/")
}

// 📝️NormalizeTicketFileInput normalises one file identifier into a repository-relative path.
func NormalizeTicketFileInput(filePath string) string {
	normalized := strings.TrimSpace(filePath)
	if normalized == "" {
		return ""
	}
	if rest, found := strings.CutPrefix(normalized, "file://"); found {
		return strings.TrimPrefix(NormalizeSeparators(rest), "./")
	}
	return strings.TrimPrefix(NormalizeSeparators(normalized), "./")
}

// 💿️NormalizeTicketFileInputs normalises and de-duplicates file identifiers, preserving order.
func NormalizeTicketFileInputs(files []string) []string {
	seen := map[string]bool{}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizeTicketFileInput(filePath)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		filtered = append(filtered, normalized)
	}
	return filtered
}

// 🗺️FilterTicketWorkspaceFiles drops the files that live inside a ticket's own folder.
func FilterTicketWorkspaceFiles(folderPath string, files []string) []string {
	relative := strings.TrimPrefix(NormalizeSeparators(folderPath), "./")
	if relative == "" {
		return files
	}
	kept := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := strings.TrimPrefix(NormalizeSeparators(filePath), "./")
		if normalized == relative || strings.HasPrefix(normalized, relative+"/") {
			continue
		}
		kept = append(kept, filePath)
	}
	return kept
}

// 📐️TicketFileScope is the file scope a ticket closes with.
type TicketFileScope struct {
	Files     []string             `json:"files"`
	Excluded  []string             `json:"excluded"`
	DiffLines map[string]DiffLines `json:"diffLines"`
	Statuses  []string             `json:"statuses"`
}

// 🎫️ComputeTicketFileScope computes the file scope of a ticket close through version control.
func ComputeTicketFileScope(control VersionControl, folderPath string, files []string) (TicketFileScope, error) {
	normalized := NormalizeTicketFileInputs(files)
	workspaceFiles := FilterTicketWorkspaceFiles(folderPath, normalized)
	kept := make([]string, 0, len(workspaceFiles))
	excluded := make([]string, 0)
	for _, filePath := range workspaceFiles {
		if IsRepoExcludedPath(filePath) || control.IsIgnored(filePath) {
			excluded = append(excluded, filePath)
			continue
		}
		kept = append(kept, filePath)
	}
	if len(kept) == 0 {
		return TicketFileScope{}, errInvalid("at least one file is required")
	}
	nameStatus, err := control.NameStatus(GitIndexRef, "", kept)
	if err != nil {
		return TicketFileScope{}, err
	}
	statuses := make([]string, 0)
	for _, line := range strings.Split(nameStatus, "\n") {
		if strings.TrimSpace(line) != "" {
			statuses = append(statuses, line)
		}
	}
	diff, err := control.UnifiedDiff(GitIndexRef, "", kept)
	if err != nil {
		return TicketFileScope{}, err
	}
	return TicketFileScope{Files: kept, Excluded: excluded, DiffLines: ParseDiffLines(diff), Statuses: statuses}, nil
}

// 📏️ParseDiffLines counts the added and removed lines of a unified diff, per file.
func ParseDiffLines(stdout string) map[string]DiffLines {
	counted := map[string]DiffLines{}
	current := ""
	for _, line := range strings.Split(stdout, "\n") {
		switch {
		case strings.HasPrefix(line, "+++ b/"):
			current = strings.TrimPrefix(line, "+++ b/")
			if _, found := counted[current]; !found {
				counted[current] = DiffLines{}
			}
		case strings.HasPrefix(line, "--- a/") || strings.HasPrefix(line, "diff --git ") || strings.HasPrefix(line, "@@"):
		case current == "":
		case strings.HasPrefix(line, "+"):
			entry := counted[current]
			entry.Added++
			counted[current] = entry
		case strings.HasPrefix(line, "-"):
			entry := counted[current]
			entry.Removed++
			counted[current] = entry
		}
	}
	return counted
}

// 📪️CanCloseTicket reports whether a ticket may be closed, with the reasons it may not.
func CanCloseTicket(ticket *model.Ticket) (bool, []string) {
	if ticket == nil {
		return false, []string{"Ticket data is nil"}
	}
	return true, nil
}

// #endregion 📐️File Scope
