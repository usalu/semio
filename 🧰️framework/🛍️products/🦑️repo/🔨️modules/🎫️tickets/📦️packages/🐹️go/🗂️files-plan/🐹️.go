package tickets

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗂️Files And Plan

// 📏️Ticket file input normalization for close operations.
// Requirements: Accept repo-relative paths, absolute paths, repo file URIs, and file artifact IDs.
// Docs: Used by ticket close to map file identifiers to repo paths.
// 📝️normalizeTicketFileInput holds the data fields for a normalizeTicketFileInput record.
func normalizeTicketFileInput(filePath string) string {
	normalized := strings.TrimSpace(filePath)
	if normalized == "" {
		return ""
	}
	if strings.HasPrefix(normalized, "repo://") {
		id := model.UriToId(normalized)
		if id != "" {
			path := model.IdToPath(id)
			if path != "" {
				return workspace.NormalizeRepoPath(path)
			}
		}
	}
	if strings.HasPrefix(normalized, "file://") {
		path := strings.TrimPrefix(normalized, "file://")
		return workspace.NormalizeRepoPath(path)
	}
	if filepath.IsAbs(normalized) || strings.ContainsAny(normalized, `/\`) || filepath.Ext(normalized) != "" {
		return workspace.NormalizeRepoPath(normalized)
	}
	uri := model.IdToUri(normalized)
	if uri != "" && strings.HasPrefix(uri, "repo://") {
		id := model.UriToId(uri)
		if id != "" {
			path := model.IdToPath(id)
			if path != "" {
				return workspace.NormalizeRepoPath(path)
			}
		}
	}
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err == nil {
		for _, file := range ctx.Files {
			if codebase.FileHeaderId(file) == normalized {
				return workspace.NormalizeRepoPath(file)
			}
		}
	}
	return workspace.NormalizeRepoPath(normalized)
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

// 🧲️extractMilestoneNumber holds the data fields for a extractMilestoneNumber record.
func extractMilestoneNumber(milestoneURL string) int {
	parts := strings.Split(milestoneURL, "/")
	if len(parts) == 0 {
		return 0
	}
	num, _ := strconv.Atoi(parts[len(parts)-1])
	return num
}

// 🔲️SaveTicket MUST persist the ticket atomically to the data store.
// 💾️SaveTicket persists ticket to the data store.
func SaveTicket(ticket *model.Ticket) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	jsonBytes, err := json.MarshalIndent(ticket, "", "  ")
	if err != nil {
		return err
	}
	if err := workspace.WriteTextFile(ticket.JsonPath, string(jsonBytes)); err != nil {
		return err
	}
	// Sync ticket state to server (fire-and-forget, don't block on failure)
	action := "open"
	if ticket.Status == model.TicketStatusClosed {
		action = "close"
	}
	go syncTicketToServer(ticket, action)
	return nil
}

// 📖️ReadTicket MUST return the ticket content or an error if unavailable.
// 🔸️ReadTicket reads and returns ticket from the source.
func ReadTicket(year, month, day int, slug string) (*model.Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	if !workspace.FileExists(jsonPath) {
		return nil, fmt.Errorf("ticket not found: %s", jsonPath)
	}
	raw, err := workspace.ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var ticket model.Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		return nil, err
	}

	ticket.Year = year
	ticket.Month = month
	ticket.Day = day
	ticket.Slug = slug
	ticket.FolderPath = folderPath
	ticket.JsonPath = jsonPath
	ticket.ImportantPath = GetImportantFilePath(year, month, day, slug)

	return &ticket, nil
}

// 🎫️ComputeTicketFiles MUST return the computed result deterministically.
// ComputeTicketFiles computes and returns the ticket files.
// 💾️Uses unstaged diffs only (index vs working tree) for complete, current working state.
func ComputeTicketFiles(ticket *model.Ticket, files []string) (*model.TicketDiffs, error) {
	files = normalizeTicketFileInputs(files)
	if ticket != nil && ticket.FolderPath != "" {
		files = FilterTicketWorkspaceFiles(workspace.NormalizeRepoPath(ticket.FolderPath), files)
	}
	files = model.FilterConsideredFiles(files)
	files = model.FilterGitIgnored(files)
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}
	baseRef := providers.GitIndexRef
	diffStatuses, err := providers.GetGitDiffStatus(baseRef, "", files)
	if err != nil {
		return nil, err
	}
	diffLines, err := providers.GetGitDiffLines(baseRef, "", files)
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

	bundles := codebase.GetTechnologies()
	baseCodebase, err := codebase.BuildCodebaseSnapshot(baseFileList, bundles, baseRef)
	if err != nil {
		return nil, err
	}
	currentCodebase, err := codebase.BuildCodebaseSnapshot(currentFileList, bundles, "")
	if err != nil {
		return nil, err
	}
	result := BuildSemanticDiffs(baseCodebase, currentCodebase, baseRef, diffLines, diffStatuses, bundles)
	return result, nil
}

// ⚙️syncTicketToServer sends ticket state to the server. No-operation if server is not configured.
func syncTicketToServer(ticket *model.Ticket, action string) {
	addr := events.GetServerAddr()
	if addr == "" {
		return
	}
	ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	payload := map[string]interface{}{
		"action":    action,
		"ticket_id": ticketID,
		"title":     ticket.Title,
		"prompt":    ticket.Description,
		"summary":   ticket.Summary,
		"goal":      ticket.Goal,
		"parent":    ticket.Parent,
		"author":    contributors.GetGitAuthorAlias(),
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		payload["github_issue"] = ticket.Management.Issue
	}
	for _, s := range ticket.Sessions {
		if s != "" {
			payload["session_id"] = s
			break
		}
	}
	resp, err := events.ServerRequest("POST", "/api/v1/tickets", payload)
	if err != nil {
		return
	}
	resp.Body.Close()
}

func moveTicketPlanIntoFolder(ticket *model.Ticket) error {
	if ticket == nil || ticket.Plan == nil || strings.TrimSpace(ticket.Plan.Source) == "" {
		return nil
	}
	src := filepath.Clean(ticket.Plan.Source)
	if ticket.FolderPath == "" {
		return fmt.Errorf("ticket folder path is empty")
	}
	destName := filepath.Base(src)
	dest := filepath.Join(ticket.FolderPath, destName)
	if _, err := os.Stat(src); os.IsNotExist(err) {
		if _, err2 := os.Stat(dest); err2 == nil {
			ticket.Plan.Local = destName
			ticket.Plan.Source = ""
			return nil
		}
		return fmt.Errorf("plan source %q missing and destination %q not found", src, dest)
	}
	st, err := os.Stat(src)
	if err != nil {
		return fmt.Errorf("plan source missing: %w", err)
	}
	if st.IsDir() {
		if err := os.Rename(src, dest); err != nil {
			if err := move.CopyDirTree(src, dest); err != nil {
				return fmt.Errorf("copy spec directory: %w", err)
			}
			if err := os.RemoveAll(src); err != nil {
				return fmt.Errorf("remove spec source after copy: %w", err)
			}
		}
	} else {
		if err := move.MoveFile(src, dest); err != nil {
			return fmt.Errorf("move plan file: %w", err)
		}
	}
	ticket.Plan.Local = destName
	ticket.Plan.Source = ""
	return nil
}

// 📝️stripPlanFrontmatter removes YAML frontmatter from plan markdown.
func stripPlanFrontmatter(content string) string {
	if !strings.HasPrefix(content, "---") {
		return strings.TrimSpace(content)
	}
	end := strings.Index(content[3:], "\n---")
	if end < 0 {
		return strings.TrimSpace(content)
	}
	rest := content[3+end+4:]
	if strings.HasPrefix(rest, "\r\n") {
		rest = rest[2:]
	} else if strings.HasPrefix(rest, "\n") {
		rest = rest[1:]
	}
	return strings.TrimSpace(rest)
}

// 📝️formatPlanFileSection formats one plan file as a collapsible GitHub markdown section.
func formatPlanFileSection(path string) (string, error) {
	raw, err := workspace.ReadTextFile(path)
	if err != nil {
		return "", err
	}
	body := stripPlanFrontmatter(raw)
	if strings.TrimSpace(body) == "" {
		return "", nil
	}
	name := filepath.Base(path)
	return fmt.Sprintf("<details>\n<summary>%s</summary>\n\n%s\n\n</details>", name, body), nil
}

// 📝️formatPlanComment builds a GitHub issue comment body from a bound plan or spec.
func formatPlanComment(plan *model.TicketPlan, src string) (string, error) {
	if plan == nil || strings.TrimSpace(src) == "" {
		return "", nil
	}
	st, err := os.Stat(src)
	if err != nil {
		return "", err
	}
	var sections []string
	if st.IsDir() {
		matches, gerr := filepath.Glob(filepath.Join(src, "*.md"))
		if gerr != nil {
			return "", gerr
		}
		sort.Strings(matches)
		for _, p := range matches {
			sec, serr := formatPlanFileSection(p)
			if serr != nil {
				return "", serr
			}
			if sec != "" {
				sections = append(sections, sec)
			}
		}
	} else {
		sec, serr := formatPlanFileSection(src)
		if serr != nil {
			return "", serr
		}
		if sec != "" {
			sections = append(sections, sec)
		}
	}
	if len(sections) == 0 {
		return "", nil
	}
	return "# 📋️ Plan\n\n" + strings.Join(sections, "\n\n"), nil
}

// 📝️postTicketPlanComment posts the bound plan markdown to the ticket GitHub issue.
func postTicketPlanComment(ticket *model.Ticket, noManagement bool) {
	if noManagement || ticket == nil || ticket.Plan == nil {
		return
	}
	src := strings.TrimSpace(ticket.Plan.Source)
	if src == "" {
		return
	}
	if ticket.Management == nil || strings.TrimSpace(ticket.Management.Issue) == "" {
		return
	}
	body, err := formatPlanComment(ticket.Plan, src)
	if err != nil {
		workspace.WriteWarningf("Failed to format plan comment: %v", err)
		return
	}
	if strings.TrimSpace(body) == "" {
		return
	}
	issueURL := ticket.Management.Issue
	if err := providers.GetManagementProvider().AddComment(issueURL, body); err != nil {
		workspace.WriteWarningf("Failed to add plan comment to GitHub issue: %v", err)
	}
}

// 🔌️init wires the ticket read 🗂️codebase declares but cannot implement.
func init() {
	codebase.LookupCodebaseTickets = func() ([]model.Ticket, error) { return ListTickets(nil, nil, nil) }
}

// #endregion 🗂️Files And Plan
