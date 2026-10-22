// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Plan source resolution and ticket plan comments.

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

func ResolvePlanSource(kind McpClientKind, id string) (absPath string, isDir bool, err error) {
	id = strings.TrimSpace(id)
	if id == "" {
		return "", false, fmt.Errorf("plan or spec id is empty")
	}
	root := strings.TrimSpace(rootDir)
	if root == "" {
		return "", false, fmt.Errorf("repository root is not set")
	}
	switch kind {
	case McpClientKiro:
		dir := filepath.Join(root, ".kiro", "specs", id)
		st, statErr := os.Stat(dir)
		if statErr != nil {
			return "", false, fmt.Errorf("Kiro spec %q: %w", id, statErr)
		}
		if !st.IsDir() {
			return "", false, fmt.Errorf("Kiro spec %q is not a directory", id)
		}
		return filepath.Clean(dir), true, nil
	case McpClientCopilot, McpClientClaude, McpClientCodex:
		home, herr := os.UserHomeDir()
		if herr != nil {
			return "", false, herr
		}
		repoBase := filepath.Base(filepath.Clean(root))
		var p string
		switch kind {
		case McpClientCopilot:
			p = filepath.Join(home, ".copilot", "projects", repoBase, "memory", id+".md")
		case McpClientClaude:
			p = filepath.Join(home, ".claude", "plans", id+".md")
		case McpClientCodex:
			p = filepath.Join(home, ".codex", "memory", repoBase, id+".md")
		}
		st, statErr := os.Stat(p)
		if statErr != nil {
			return "", false, fmt.Errorf("plan file for id %q: %w", id, statErr)
		}
		if st.IsDir() {
			return "", false, fmt.Errorf("expected file at %s", p)
		}
		return filepath.Clean(p), false, nil
	default:
		return "", false, fmt.Errorf("plan or spec attachment is not supported for mcp kind %q", kind)
	}
}

// 🗓️planClientTag returns the value stored in ticket.json for the plan attachment.
func planClientTag(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "cursor"
	case McpClientKiro:
		return "kiro"
	case McpClientCopilot:
		return "copilot"
	case McpClientClaude:
		return "claude"
	case McpClientCodex:
		return "codex"
	default:
		return string(kind)
	}
}

// 📱️ApplyTicketPlanFromIDs resolves plan_id or spec_id and attaches it to the ticket before save.
func ApplyTicketPlanFromIDs(ticket *Ticket, kind McpClientKind, planID, specID string) error {
	planID = strings.TrimSpace(planID)
	specID = strings.TrimSpace(specID)
	if planID == "" && specID == "" {
		return nil
	}
	if planID != "" && specID != "" {
		return fmt.Errorf("pass only one of plan_id or spec_id")
	}
	var resolveKind McpClientKind
	var id string
	switch kind {
	case McpClientKiro:
		if planID != "" {
			return fmt.Errorf("use spec_id for Kiro, not plan_id")
		}
		resolveKind = McpClientKiro
		id = specID
	default:
		if specID != "" {
			return fmt.Errorf("use plan_id for this client, not spec_id")
		}
		resolveKind = kind
		id = planID
	}
	src, isDir, err := ResolvePlanSource(resolveKind, id)
	if err != nil {
		return err
	}
	ticket.Plan = &TicketPlan{
		Client: planClientTag(kind),
		ID:     id,
		Source: src,
	}
	_ = isDir // stored implicitly: directories use trailing handling in move
	return nil
}

func moveTicketPlanIntoFolder(ticket *Ticket) error {
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
			if err := copyDirTree(src, dest); err != nil {
				return fmt.Errorf("copy spec directory: %w", err)
			}
			if err := os.RemoveAll(src); err != nil {
				return fmt.Errorf("remove spec source after copy: %w", err)
			}
		}
	} else {
		if err := MoveFile(src, dest); err != nil {
			return fmt.Errorf("move plan file: %w", err)
		}
	}
	ticket.Plan.Local = destName
	ticket.Plan.Source = ""
	return nil
}

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
	raw, err := ReadTextFile(path)
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
func formatPlanComment(plan *TicketPlan, src string) (string, error) {
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
func postTicketPlanComment(ticket *Ticket, noManagement bool) {
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
		writeWarningf("Failed to format plan comment: %v", err)
		return
	}
	if strings.TrimSpace(body) == "" {
		return
	}
	issueURL := ticket.Management.Issue
	if err := GetManagementProvider().AddComment(issueURL, body); err != nil {
		writeWarningf("Failed to add plan comment to GitHub issue: %v", err)
	}
}

func copyDirTree(srcRoot, dstRoot string) error {
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
