// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket paths, lookup, and checkpoint file access.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// 🎫️GetTicketsDir MUST return the stored value without modification.
// 📖️GetTicketsDir returns the tickets dir of the value.
func GetTicketsDir() string {
	return filepath.Join(GetRepoMetaDir(), "🎫️tickets")
}

// 🏪️GetTicketPath MUST return the stored value without modification.
// 📦️GetTicketPath returns the ticket path of the value.
func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), FormatYearDir(year), FormatMonthDir(month), FormatDayDir(day), slug)
}

// 📥️GetImportantFilePath MUST return the stored value without modification.
// 📥️GetImportantFilePath returns the important file path of the value.
func GetImportantFilePath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "📌️important", "📝️.md")
}

type ticketImportantPreimage struct {
	path string
	dir  string
	mode os.FileMode
}

type ticketImportantCreation struct {
	path        string
	dir         string
	fileCreated bool
	dirCreated  bool
}

// 🧷️canonicalTicketImportantPath returns and stores the ticket-owned important document path.
func canonicalTicketImportantPath(ticket *Ticket) (string, error) {
	if ticket == nil {
		return "", fmt.Errorf("ticket is nil")
	}
	if ticket.Slug == "" {
		return "", fmt.Errorf("ticket slug is required")
	}
	ticket.ImportantPath = GetImportantFilePath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	return ticket.ImportantPath, nil
}

// 🔎️inspectTicketImportantDocument requires the canonical document to be an exact empty regular-file bundle.
func inspectTicketImportantDocument(ticket *Ticket) (ticketImportantPreimage, error) {
	path, err := canonicalTicketImportantPath(ticket)
	if err != nil {
		return ticketImportantPreimage{}, err
	}
	info, err := os.Lstat(path)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: required important document %s is unavailable: %w", path, err)
	}
	if !info.Mode().IsRegular() {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important document %s is not a regular file", path)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: read important document %s: %w", path, err)
	}
	if len(data) != 0 {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important document %s is not empty", path)
	}
	dir := filepath.Dir(path)
	entries, err := os.ReadDir(dir)
	if err != nil {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: read important directory %s: %w", dir, err)
	}
	if len(entries) != 1 || entries[0].Name() != filepath.Base(path) {
		return ticketImportantPreimage{}, fmt.Errorf("cannot finish ticket: important directory %s must contain only %s", dir, filepath.Base(path))
	}
	return ticketImportantPreimage{path: path, dir: dir, mode: info.Mode()}, nil
}

// 🔄️restoreTicketImportantDocument restores an empty document without overwriting an existing node.
func restoreTicketImportantDocument(preimage ticketImportantPreimage) error {
	if err := os.MkdirAll(preimage.dir, 0755); err != nil {
		return err
	}
	file, err := os.OpenFile(preimage.path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, preimage.mode.Perm())
	if err != nil {
		return err
	}
	return file.Close()
}

// 🗑️removeTicketImportantDocument removes the exact empty document bundle and restores it on partial failure.
func removeTicketImportantDocument(preimage ticketImportantPreimage) error {
	if err := os.Remove(preimage.path); err != nil {
		return err
	}
	if err := os.Remove(preimage.dir); err != nil {
		if restoreErr := restoreTicketImportantDocument(preimage); restoreErr != nil {
			return fmt.Errorf("remove important directory: %w; restore important document: %v", err, restoreErr)
		}
		return fmt.Errorf("remove important directory: %w", err)
	}
	return nil
}

// ✨️ensureTicketImportantDocument preserves an existing regular document or exclusively creates an empty one.
func ensureTicketImportantDocument(ticket *Ticket) (ticketImportantCreation, error) {
	path, err := canonicalTicketImportantPath(ticket)
	if err != nil {
		return ticketImportantCreation{}, err
	}
	creation := ticketImportantCreation{path: path, dir: filepath.Dir(path)}
	info, err := os.Lstat(path)
	if err == nil {
		if !info.Mode().IsRegular() {
			return creation, fmt.Errorf("important document %s is not a regular file", path)
		}
		if _, err := os.ReadFile(path); err != nil {
			return creation, fmt.Errorf("read important document %s: %w", path, err)
		}
		return creation, nil
	}
	if !os.IsNotExist(err) {
		return creation, err
	}
	dirInfo, dirErr := os.Lstat(creation.dir)
	if dirErr == nil {
		if !dirInfo.IsDir() || dirInfo.Mode()&os.ModeSymlink != 0 {
			return creation, fmt.Errorf("important directory %s is not a physical directory", creation.dir)
		}
	} else if os.IsNotExist(dirErr) {
		if err := os.MkdirAll(creation.dir, 0755); err != nil {
			return creation, err
		}
		creation.dirCreated = true
	} else {
		return creation, dirErr
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		if creation.dirCreated {
			_ = os.Remove(creation.dir)
		}
		return ticketImportantCreation{}, err
	}
	if err := file.Close(); err != nil {
		_ = os.Remove(path)
		if creation.dirCreated {
			_ = os.Remove(creation.dir)
		}
		return ticketImportantCreation{}, err
	}
	creation.fileCreated = true
	return creation, nil
}

// ↩️rollbackTicketImportantCreation removes only nodes created by the current reopen operation.
func rollbackTicketImportantCreation(creation ticketImportantCreation) error {
	if creation.fileCreated {
		if err := os.Remove(creation.path); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	if creation.dirCreated {
		if err := os.Remove(creation.dir); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	return nil
}

// 🛤️GetTicketJsonPath MUST return the stored value without modification.
// ⬛️GetTicketJsonPath returns the ticket json path of the value.
func GetTicketJsonPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "🎫️ticket.json")
}

// 💿️normalizeTicketKeyword holds the data fields for a normalizeTicketKeyword record.
func normalizeTicketKeyword(value string) string {
	return strings.ToUpper(strings.TrimSpace(value))
}

// 🔷️hasTicketKeyword holds the data fields for a hasTicketKeyword record.
func hasTicketKeyword(text, keyword string) bool {
	return strings.Contains(strings.ToUpper(text), keyword)
}

// 🎯️FindTicketBySlug MUST return nil when no match is found.
// 🔎️FindTicketBySlug searches for and returns the matching ticket by slug.
func FindTicketBySlug(slug string) (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	for i := len(tickets) - 1; i >= 0; i-- {
		t := tickets[i]
		if t.Slug == slug || filepath.Base(t.Slug) == slug {
			return &t, nil
		}
	}
	return nil, fmt.Errorf("ticket not found: %s", slug)
}

// 🧪️LatestTicket MUST complete the operation and return consistent results.
func LatestTicket() (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, fmt.Errorf("no tickets found")
	}
	sort.Slice(tickets, func(i, j int) bool {
		if tickets[i].Year != tickets[j].Year {
			return tickets[i].Year > tickets[j].Year
		}
		if tickets[i].Month != tickets[j].Month {
			return tickets[i].Month > tickets[j].Month
		}
		if tickets[i].Day != tickets[j].Day {
			return tickets[i].Day > tickets[j].Day
		}
		return tickets[i].GetDateStarted().After(tickets[j].GetDateStarted())
	})
	return &tickets[0], nil
}

// 🔶️shouldContinueTicket holds the data fields for a shouldContinueTicket record.
func shouldContinueTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "CONTINUE")
}

// 🔹️shouldSkipTicket holds the data fields for a shouldSkipTicket record.
func shouldSkipTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "NOTICKET")
}

// 🔻️CountLines MUST complete the operation and return consistent results.
func CountLines(content string) int {
	if content == "" {
		return 0
	}
	return strings.Count(content, "\n") + 1
}

// 📄️CountLinesInFile MUST complete the operation and return consistent results.
func CountLinesInFile(path string) int {
	content, err := ReadTextFile(path)
	if err != nil {
		return 0
	}
	return CountLines(content)
}

// 💾️CountLinesAtCheckpoint MUST complete the operation and return consistent results.
func CountLinesAtCheckpoint(checkpoint, filePath string) int {
	stdout, _, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return 0
	}
	return CountLines(stdout)
}

// ❌️ReadTextFileAtCheckpoint MUST return the text file at checkpoint content or an error if unavailable.
// ✔️ReadTextFileAtCheckpoint reads and returns text file at checkpoint from the source.
func ReadTextFileAtCheckpoint(checkpoint, filePath string) (string, error) {
	if checkpoint == "" {
		return ReadTextFile(filepath.Join(rootDir, filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

// ✔️ListFilesAtCheckpoint MUST return all available files at checkpoint entries.
// 🔢️ListFilesAtCheckpoint returns a list of files at checkpoint entries.
func ListFilesAtCheckpoint(checkpoint string) ([]string, error) {
	if checkpoint == "" {
		files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, GetTechnologies())
		if err != nil {
			return nil, err
		}
		return files, nil
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"ls-tree", "-r", "--name-only", checkpoint}, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git ls-tree failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		files = append(files, strings.TrimSpace(line))
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

// 📰️formatPromptHeading holds the data fields for a formatPromptHeading record.
func formatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖️ Prompt"
	}
	return "# 🤖️ Prompt\n\n" + body
}

// 📋️formatSummaryHeading holds the data fields for a formatSummaryHeading record.
func formatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍️ Summary"
	}
	return "# 🔍️ Summary\n\n" + body
}

// 📑️replaceSectionContent holds the data fields for a replaceSectionContent record.
func replaceSectionContent(content, sectionHeading, newContent string) string {
	idx := strings.Index(content, sectionHeading)
	if idx == -1 {
		return content
	}
	before := content[:idx]
	after := content[idx+len(sectionHeading):]

	nextHeading := strings.Index(after, "\n## ")
	if nextHeading == -1 {

		return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + "\n"
	}

	rest := after[nextHeading:]
	return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + rest
}

// 📏️Requirements: Accept repo-relative paths, absolute paths, repo file URIs, and file artifact IDs.
// Docs: Used by ticket close to map file identifiers to repo paths.
// 📝️normalizeTicketFileInput holds the data fields for a normalizeTicketFileInput record.
func normalizeTicketFileInput(filePath string) string {
	normalized := strings.TrimSpace(filePath)
	if normalized == "" {
		return ""
	}
	if strings.HasPrefix(normalized, "repo://") {
		id := UriToId(normalized)
		if id != "" {
			path := IdToPath(id)
			if path != "" {
				return normalizeRepoPath(path)
			}
		}
	}
	if strings.HasPrefix(normalized, "file://") {
		path := strings.TrimPrefix(normalized, "file://")
		return normalizeRepoPath(path)
	}
	if filepath.IsAbs(normalized) || strings.ContainsAny(normalized, `/\`) || filepath.Ext(normalized) != "" {
		return normalizeRepoPath(normalized)
	}
	uri := IdToUri(normalized)
	if uri != "" && strings.HasPrefix(uri, "repo://") {
		id := UriToId(uri)
		if id != "" {
			path := IdToPath(id)
			if path != "" {
				return normalizeRepoPath(path)
			}
		}
	}
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err == nil {
		for _, file := range ctx.Files {
			if FileHeaderId(file) == normalized {
				return normalizeRepoPath(file)
			}
		}
	}
	return normalizeRepoPath(normalized)
}

// 💿️normalizeTicketFileInputs holds the data fields for a normalizeTicketFileInputs record.
func normalizeTicketFileInputs(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	seen := make(map[string]struct{}, len(files))
	for _, filePath := range files {
		normalized := normalizeTicketFileInput(filePath)
		if normalized == "" {
			continue
		}
		if _, ok := seen[normalized]; ok {
			continue
		}
		seen[normalized] = struct{}{}
		filtered = append(filtered, normalized)
	}
	return filtered
}

// 🧹️FilterTicketWorkspaceFiles MUST return only entries that match the filter criteria.
// 🗺️FilterTicketWorkspaceFiles returns the subset of ticket workspace files matching the criteria.
func FilterTicketWorkspaceFiles(ticket *Ticket, files []string) []string {
	if ticket == nil {
		return files
	}
	if len(files) == 0 {
		return files
	}
	if ticket.FolderPath == "" {
		return files
	}
	relative := NormalizePath(ticket.FolderPath)
	if filepath.IsAbs(ticket.FolderPath) {
		relative = GetRelativePath(ticket.FolderPath)
	}
	relative = strings.TrimPrefix(relative, "./")
	if relative == "" {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		if normalized == relative || strings.HasPrefix(normalized, relative+"/") {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}
