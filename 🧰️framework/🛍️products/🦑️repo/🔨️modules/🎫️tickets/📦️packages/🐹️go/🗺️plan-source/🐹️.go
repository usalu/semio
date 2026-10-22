package tickets

import (
	fmt "fmt"
	os "os"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗺️Plan Source

// 🗺️PlanSource is where a plan or spec id resolves to.
type PlanSource struct {
	Path        string `json:"path"`
	IsDirectory bool   `json:"isDirectory"`
}

// 🏠️PlanRoots are the two roots plan resolution reads.
type PlanRoots struct {
	RepoRoot string `json:"repoRoot"`
	HomeDir  string `json:"homeDir"`
}

// 🏷️PlanClientTag returns the value stored for the plan attachment of a client.
func PlanClientTag(kind providers.McpClientKind) string {
	switch kind {
	case providers.McpClientCursor:
		return "cursor"
	case providers.McpClientKiro:
		return "kiro"
	case providers.McpClientCopilot:
		return "copilot"
	case providers.McpClientClaude:
		return "claude"
	case providers.McpClientCodex:
		return "codex"
	}
	return "generic"
}

// 🏠️HostPlanRoots are the plan roots of the repository and the user the process runs as.
func HostPlanRoots() PlanRoots {
	home, _ := os.UserHomeDir()
	return PlanRoots{RepoRoot: workspace.GetRootDir(), HomeDir: home}
}

// 🗺️ResolvePlanSource resolves a plan or spec id to a store path, per agent client. A ticket owns the `*.plan.md` files of its own folder; no plan is read from a client directory inside the repository.
func ResolvePlanSource(store TicketStore, roots PlanRoots, kind providers.McpClientKind, id string) (PlanSource, error) {
	id = strings.TrimSpace(id)
	if id == "" {
		return PlanSource{}, errInvalid("plan or spec id is empty")
	}
	root := strings.TrimSpace(roots.RepoRoot)
	if root == "" {
		return PlanSource{}, errInvalid("repository root is not set")
	}
	switch kind {
	case providers.McpClientKiro:
		directory := JoinPath(root, ".kiro/specs/"+id)
		metadata, found := store.Metadata(directory)
		switch {
		case found && metadata.Kind == NodeDirectory:
			return PlanSource{Path: directory, IsDirectory: true}, nil
		case found:
			return PlanSource{}, errInvalid(fmt.Sprintf("Kiro spec %q is not a directory", id))
		}
		return PlanSource{}, errTicketNotFound(fmt.Sprintf("Kiro spec %q: no such directory", id))
	case providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		home := strings.TrimSpace(roots.HomeDir)
		if home == "" {
			return PlanSource{}, errInvalid("home directory is not set")
		}
		repoBase := BaseName(root)
		path := JoinPath(home, ".codex/memory/"+repoBase+"/"+id+".md")
		switch kind {
		case providers.McpClientCopilot:
			path = JoinPath(home, ".copilot/projects/"+repoBase+"/memory/"+id+".md")
		case providers.McpClientClaude:
			path = JoinPath(home, ".claude/plans/"+id+".md")
		}
		metadata, found := store.Metadata(path)
		switch {
		case found && metadata.Kind == NodeDirectory:
			return PlanSource{}, errInvalid("expected file at " + path)
		case found:
			return PlanSource{Path: path}, nil
		}
		return PlanSource{}, errTicketNotFound(fmt.Sprintf("plan file for id %q: no such file", id))
	}
	return PlanSource{}, errInvalid(fmt.Sprintf("plan or spec attachment is not supported for mcp kind %q", string(kind)))
}

// 📎️ApplyTicketPlanFromIDs resolves a plan or spec id and attaches it to a ticket before it is saved.
func ApplyTicketPlanFromIDs(store TicketStore, roots PlanRoots, ticket *model.Ticket, kind providers.McpClientKind, planID, specID string) error {
	planID = strings.TrimSpace(planID)
	specID = strings.TrimSpace(specID)
	if planID == "" && specID == "" {
		return nil
	}
	if planID != "" && specID != "" {
		return errInvalid("pass only one of plan_id or spec_id")
	}
	id := planID
	if kind == providers.McpClientKiro {
		if planID != "" {
			return errInvalid("use spec_id for Kiro, not plan_id")
		}
		id = specID
	} else if specID != "" {
		return errInvalid("use plan_id for this client, not spec_id")
	}
	source, err := ResolvePlanSource(store, roots, kind, id)
	if err != nil {
		return err
	}
	ticket.Plan = &model.TicketPlan{Client: PlanClientTag(kind), ID: id, Source: source.Path}
	return nil
}

// 📦️MovePlanIntoFolderIn moves an attached plan or spec into the ticket folder as it closes.
func MovePlanIntoFolderIn(store TicketStore, ticket *model.Ticket) error {
	if ticket.Plan == nil || strings.TrimSpace(ticket.Plan.Source) == "" {
		return nil
	}
	if ticket.FolderPath == "" {
		return errInvalid("ticket folder path is empty")
	}
	source := NormalizeSeparators(ticket.Plan.Source)
	destinationName := BaseName(source)
	destination := JoinPath(ticket.FolderPath, destinationName)
	if !StoreExists(store, source) {
		if StoreExists(store, destination) {
			ticket.Plan.Local = destinationName
			ticket.Plan.Source = ""
			return nil
		}
		return errTicketNotFound(fmt.Sprintf("plan source %q missing and destination %q not found", source, destination))
	}
	if err := store.Rename(source, destination); err != nil {
		return err
	}
	ticket.Plan.Local = destinationName
	ticket.Plan.Source = ""
	return nil
}

// 📝️StripPlanFrontmatter strips YAML frontmatter from plan markdown.
func StripPlanFrontmatter(content string) string {
	if !strings.HasPrefix(content, "---") {
		return strings.TrimSpace(content)
	}
	end := strings.Index(content[3:], "\n---")
	if end < 0 {
		return strings.TrimSpace(content)
	}
	rest := content[3+end+4:]
	switch {
	case strings.HasPrefix(rest, "\r\n"):
		rest = rest[2:]
	case strings.HasPrefix(rest, "\n"):
		rest = rest[1:]
	}
	return strings.TrimSpace(rest)
}

// 📝️FormatPlanFileSection formats one plan file as a collapsible markdown section.
func FormatPlanFileSection(name, raw string) string {
	body := StripPlanFrontmatter(raw)
	if strings.TrimSpace(body) == "" {
		return ""
	}
	return "<details>\n<summary>" + name + "</summary>\n\n" + body + "\n\n</details>"
}

// #endregion 🗺️Plan Source
