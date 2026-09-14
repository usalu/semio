// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌳️tree is the tree domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package tree

import (
	bytes "bytes"
	gzip "compress/gzip"
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	io "io"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	sync "sync"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	goalspkg "github.com/usalu/semio/repo/goals"
	identity "github.com/usalu/semio/repo/identity"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	statutespkg "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 🔑️Engine Events

// 📁️buildFolderRoots holds the data fields for a buildFolderRoots record.
func buildFolderRoots(folders []model.Folder) []*TreeNode {
	type folderEntry struct {
		folder *model.Folder
		node   *TreeNode
	}
	folderMap := make(map[string]*folderEntry)
	for i := range folders {
		f := &folders[i]
		folderMap[f.Path] = &folderEntry{
			folder: f,
			node: &TreeNode{
				Kind:    TreeNodeFolder,
				ID:      f.GetID(),
				Label:   f.Name,
				URI:     f.GetURI(),
				SubKind: string(f.Kind),
				Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": string(f.Kind)},
			},
		}
	}
	var roots []*TreeNode
	for path, fe := range folderMap {
		parentPath := filepath.Dir(path)
		if parent, ok := folderMap[parentPath]; ok {
			parent.node.Children = append(parent.node.Children, fe.node)
			continue
		}
		roots = append(roots, fe.node)
	}
	return roots
}

func attachFilesToFolders(root *TreeNode, files []model.File, fileNodes map[string]*TreeNode) {
	folderNodeByPath := make(map[string]*TreeNode)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind == TreeNodeFolder {
			if path, ok := node.Data["path"].(string); ok && path != "" {
				folderNodeByPath[path] = node
			}
		}
		for _, child := range node.Children {
			walk(child)
		}
	}
	walk(root)
	for i := range files {
		f := &files[i]
		templateNode, ok := fileNodes[f.Path]
		if !ok {
			continue
		}
		fileNode := cloneTreeNode(templateNode)
		folderPath := filepath.Dir(f.Path)
		if folderNode, ok := folderNodeByPath[folderPath]; ok {
			folderNode.Children = append(folderNode.Children, fileNode)
			continue
		}
		root.Children = append(root.Children, fileNode)
	}
}

// 🌳️cloneTreeNode holds the data fields for a cloneTreeNode record.
func cloneTreeNode(node *TreeNode) *TreeNode {
	copyNode := *node
	if node.Data != nil {
		copyNode.Data = make(map[string]interface{}, len(node.Data))
		for key, value := range node.Data {
			copyNode.Data[key] = value
		}
	}
	if len(node.Children) > 0 {
		copyNode.Children = make([]*TreeNode, 0, len(node.Children))
		for _, child := range node.Children {
			copyNode.Children = append(copyNode.Children, cloneTreeNode(child))
		}
	} else {
		copyNode.Children = nil
	}
	return &copyNode
}

func buildPolicyEntityKindTree(groups []model.Territory) []*TreeNode {
	statutesByEntityKind := make(map[string]map[model.Statute]bool)
	var collect func(entries []model.Territory)
	collect = func(entries []model.Territory) {
		for _, entry := range entries {
			for _, statute := range entry.Kinds {
				entityKind := model.InferEntityKindFromStatute(statute)
				if statutesByEntityKind[entityKind] == nil {
					statutesByEntityKind[entityKind] = make(map[model.Statute]bool)
				}
				statutesByEntityKind[entityKind][statute] = true
			}
			if len(entry.Groups) > 0 {
				collect(entry.Groups)
			}
		}
	}
	collect(groups)
	entityKinds := make([]string, 0, len(statutesByEntityKind))
	for entityKind := range statutesByEntityKind {
		entityKinds = append(entityKinds, entityKind)
	}
	sort.Strings(entityKinds)
	result := make([]*TreeNode, 0, len(entityKinds))
	for _, entityKind := range entityKinds {
		entityNode := &TreeNode{
			Kind:    TreeNodeCategory,
			ID:      fmt.Sprintf("entitykind:%s", entityKind),
			Label:   entityKind,
			URI:     "repo://entitykind/" + entityKind,
			SubKind: "entitykind",
			Data: map[string]interface{}{
				"kind": entityKind,
			},
		}
		statutes := make([]model.Statute, 0, len(statutesByEntityKind[entityKind]))
		for statute := range statutesByEntityKind[entityKind] {
			statutes = append(statutes, statute)
		}
		sort.Slice(statutes, func(i, j int) bool {
			return statutes[i] < statutes[j]
		})
		for _, statute := range statutes {
			meta := statute.Info()
			priorityIcon := "🟢️"
			if meta.Priority == model.BreachPriorityHigh {
				priorityIcon = "🔴️"
			} else if meta.Priority == model.BreachPriorityMedium {
				priorityIcon = "🟡️"
			}
			statuteNode := &TreeNode{
				Kind:        TreeNodeStatute,
				ID:          meta.GetID(),
				Label:       priorityIcon + workspace.StatutePathToIdValue(string(statute)),
				URI:         meta.GetURI(),
				Description: meta.Reason,
				Data: map[string]interface{}{
					"id":          string(statute),
					"priority":    string(meta.Priority),
					"autofixable": meta.Autofixable,
					"reason":      meta.Reason,
					"solution":    meta.Solution,
				},
			}
			if meta.Autofixable {
				statuteNode.SubKind = "autofixable"
			}
			entityNode.Children = append(entityNode.Children, statuteNode)
		}
		result = append(result, entityNode)
	}
	return result
}

// #endregion 🔑️Engine Events

// #region 🧨️Monorepo Tree Types

// #region 🧨️Monorepo Tree Types
// Tree node kinds, filter criteria, and matching logic for monorepo tree queries.
// 🏷️EntityKinds holds the data fields for a EntityKinds record.
var EntityKinds = []string{
	"root", "year", "month", "day", "hour", "minute", "second",
	"technology", "bundle", "folder", "file", "line", "range",
	"section", "definition", "goal", "ticket", "draft", "todo",
	"policy", "breach", "contributor", "checkpoint", "interaction", "session",
}

// 🎁️ArtifactKinds holds the data fields for a ArtifactKinds record.
var ArtifactKinds = []string{
	"repo", "technology", "bundle", "folder", "file", "section", "definition",
}

// 💿️DiffableKinds holds the data fields for a DiffableKinds record.
var DiffableKinds = []string{
	"root", "year", "month", "day", "hour",
	"technology", "bundle", "folder", "file", "section", "definition",
	"goal", "ticket", "contributor", "checkpoint", "interaction", "session",
}

// 📄️RelatedToFileKinds holds the data fields for a RelatedToFileKinds record.
var RelatedToFileKinds = []string{
	"root", "year", "month", "day", "hour", "minute", "second",
	"technology", "bundle", "folder", "goal", "ticket", "draft", "todo",
	"policy", "breach", "contributor", "checkpoint", "interaction", "session",
}

// 🌳️TreeNodeKind represents a tree node kind value.
type TreeNodeKind string

const TreeNodeTechnology TreeNodeKind = "technology"

const TreeNodeBundle TreeNodeKind = "bundle"

const TreeNodeFolder TreeNodeKind = "folder"

const TreeNodeFile TreeNodeKind = "file"

const TreeNodeSection TreeNodeKind = "section"

const TreeNodeDefinition TreeNodeKind = "definition"

const TreeNodeGoal TreeNodeKind = "goal"

const TreeNodeTicket TreeNodeKind = "ticket"

const TreeNodeDraft TreeNodeKind = "draft"

const TreeNodeTodo TreeNodeKind = "todo"

const TreeNodePolicy TreeNodeKind = "policy"

const TreeNodeBreach TreeNodeKind = "breach"

const TreeNodeContributor TreeNodeKind = "contributor"

const TreeNodeCheckpoint TreeNodeKind = "checkpoint"

const TreeNodeSession TreeNodeKind = "session"

const TreeNodeStatute TreeNodeKind = "statute"

const TreeNodeCategory TreeNodeKind = "category"

// 🌿️TreeNode holds the data fields for a tree node record.
type TreeNode struct {
	Kind        TreeNodeKind
	ID          string
	Label       string
	URI         string
	SubKind     string
	Description string
	Summary     string
	Year        int
	Month       int
	Day         int
	Status      string
	Contributor string
	Data        map[string]interface{}
	Children    []*TreeNode
	matched     bool
}

// 🧹️TreeFilter holds the data fields for a tree filter record.
type TreeFilter struct {
	Query               string
	OnlyKinds           map[TreeNodeKind]bool
	ExcludeKinds        map[TreeNodeKind]bool
	OnlySubKinds        map[TreeNodeKind][]string
	ExcludeSubKinds     map[TreeNodeKind][]string
	OnlyYears           []int
	ExcludeYears        []int
	OnlyMonths          []int
	ExcludeMonths       []int
	OnlyDays            []int
	ExcludeDays         []int
	OnlyStatus          string
	OnlyContributors    []string
	ExcludeContributors []string
	OnlyPolicies        []string
	ExcludePolicies     []string
}

// 🏷️HasOnlyKinds MUST return true only when the property is present.
// 🔍️HasOnlyKinds reports whether the TreeFilter has only kinds.
func (f *TreeFilter) HasOnlyKinds() bool {
	return len(f.OnlyKinds) > 0
}

// 🔷️IsKindVisible MUST return true only when the condition is met.
// ❓️IsKindVisible reports whether the TreeFilter is kind visible.
func (f *TreeFilter) IsKindVisible(kind TreeNodeKind) bool {
	if kind == TreeNodeCategory {
		return true
	}
	if f.HasOnlyKinds() {
		return f.OnlyKinds[kind]
	}
	return !f.ExcludeKinds[kind]
}

// 📥️MatchesSubKind MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesSubKind(kind TreeNodeKind, subKind string) bool {
	if subKind == "" {
		return true
	}
	if only, ok := f.OnlySubKinds[kind]; ok && len(only) > 0 {
		for _, s := range only {
			if strings.EqualFold(s, subKind) {
				return true
			}
		}
		return false
	}
	if exclude, ok := f.ExcludeSubKinds[kind]; ok {
		for _, s := range exclude {
			if strings.EqualFold(s, subKind) {
				return false
			}
		}
	}
	return true
}

// 🎯️MatchesDate MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesDate(year, month, day int) bool {
	if len(f.OnlyYears) > 0 {
		found := false
		for _, y := range f.OnlyYears {
			if y == year {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, y := range f.ExcludeYears {
		if y == year {
			return false
		}
	}
	if len(f.OnlyMonths) > 0 && month > 0 {
		found := false
		for _, m := range f.OnlyMonths {
			if m == month {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, m := range f.ExcludeMonths {
		if m == month {
			return false
		}
	}
	if len(f.OnlyDays) > 0 && day > 0 {
		found := false
		for _, d := range f.OnlyDays {
			if d == day {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, d := range f.ExcludeDays {
		if d == day {
			return false
		}
	}
	return true
}

// 🔶️MatchesStatus MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesStatus(status string) bool {
	if f.OnlyStatus == "" {
		return true
	}
	return strings.EqualFold(f.OnlyStatus, status)
}

// 🤝️MatchesContributor MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesContributor(contributor string) bool {
	if len(f.OnlyContributors) > 0 {
		for _, c := range f.OnlyContributors {
			if strings.EqualFold(c, contributor) {
				return true
			}
		}
		return false
	}
	for _, c := range f.ExcludeContributors {
		if strings.EqualFold(c, contributor) {
			return false
		}
	}
	return true
}

// #endregion 🧨️Monorepo Tree Types

// #region 🏩️Tree Logic

// #region 🏩️Tree Logic
// 🏢️Tree construction, filtering, searching, and rendering for goals, sections, and monorepo nodes.
func buildGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}) []*model.GoalNode {
	goalMap := make(map[string]*model.GoalNode)

	for _, g := range goalsRaw {
		if gm, ok := g.(map[string]interface{}); ok {
			id, _ := gm["id"].(string)
			title, _ := gm["title"].(string)
			status, _ := gm["status"].(string)
			dueDate, _ := gm["dueDate"].(string)
			createdAt, _ := gm["createdAt"].(string)
			description, _ := gm["description"].(string)
			node := &model.GoalNode{ID: id, Title: title, Status: status, DueDate: dueDate, CreatedAt: createdAt, Description: description}
			goalMap[id] = node
		}
	}

	var rootGoals []*model.GoalNode
	for _, g := range goalMap {
		parentID := ""
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			parentID = g.ID[:idx]
		}
		if parentID != "" && parentID != g.ID {
			if parent, ok := goalMap[parentID]; ok {
				parent.Children = append(parent.Children, g)
			} else {
				rootGoals = append(rootGoals, g)
			}
		} else {
			rootGoals = append(rootGoals, g)
		}
	}

	var sortGoals func(goals []*model.GoalNode)
	sortGoals = func(goals []*model.GoalNode) {
		sort.SliceStable(goals, func(i, j int) bool {
			a, b := goals[i], goals[j]
			if a.DueDate != b.DueDate {
				if a.DueDate == "" {
					return false
				}
				if b.DueDate == "" {
					return true
				}
				return a.DueDate < b.DueDate
			}
			return a.ID < b.ID
		})
		for _, g := range goals {
			if len(g.Children) > 0 {
				sortGoals(g.Children)
			}
		}
	}
	sortGoals(rootGoals)

	allTickets := make([]*model.TicketNode, 0)
	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			id, _ := tm["id"].(string)
			slug, _ := tm["slug"].(string)
			status, _ := tm["status"].(string)
			title, _ := tm["title"].(string)
			goalID, _ := tm["goal"].(string)
			parentID, _ := tm["parent"].(string)
			prompt, _ := tm["prompt"].(string)
			summary, _ := tm["summary"].(string)

			created := ""
			finished := ""
			if dates, ok := tm["date"].(map[string]interface{}); ok {
				created, _ = dates["created"].(string)
				finished, _ = dates["finished"].(string)
			}

			if created == "" {
				created, _ = tm["createdAt"].(string)
			}

			uri := fmt.Sprintf("repo://ticket/%s%s", model.EmojiText(model.EmojiTicket), workspace.Flat(slug))

			node := &model.TicketNode{ID: id, Slug: slug, Status: status, Title: title, URI: uri, GoalID: goalID, ParentID: parentID, Created: created, Finished: finished, Description: prompt, Summary: summary}
			allTickets = append(allTickets, node)
		}
	}

	var noGoalTickets []*model.TicketNode
	for _, t := range allTickets {
		if t.GoalID != "" {
			if g, ok := goalMap[t.GoalID]; ok {
				g.Tickets = append(g.Tickets, t)
			} else {
				noGoalTickets = append(noGoalTickets, t)
			}
		} else {
			noGoalTickets = append(noGoalTickets, t)
		}
	}

	nestTickets := func(nodes []*model.TicketNode) []*model.TicketNode {
		var roots []*model.TicketNode
		lookup := make(map[string]*model.TicketNode)
		for _, n := range nodes {
			lookup[n.ID] = n
		}
		for _, n := range nodes {
			if n.ParentID != "" && lookup[n.ParentID] != nil {
				lookup[n.ParentID].Children = append(lookup[n.ParentID].Children, n)
			} else {
				roots = append(roots, n)
			}
		}
		return roots
	}

	for _, g := range goalMap {
		g.Tickets = nestTickets(g.Tickets)
	}

	if len(noGoalTickets) > 0 {
		noGoal := &model.GoalNode{Title: "No Goal", Children: nil, Tickets: nestTickets(noGoalTickets)}
		rootGoals = append(rootGoals, noGoal)
	}

	return rootGoals
}

func countOpenSubgoals(g *model.GoalNode) int {
	c := 0
	for _, child := range g.Children {
		if child.Status == "open" {
			c++
		}
		c += countOpenSubgoals(child)
	}
	return c
}

// 🎫️countOpenTickets holds the data fields for a countOpenTickets record.
func countOpenTickets(g *model.GoalNode) int {
	c := 0
	var countT func(ts []*model.TicketNode)
	countT = func(ts []*model.TicketNode) {
		for _, t := range ts {
			if t.Status == "open" {
				c++
			}
			countT(t.Children)
		}
	}
	countT(g.Tickets)
	for _, child := range g.Children {
		c += countOpenTickets(child)
	}
	return c
}

// 🎨️renderGoalTree holds the data fields for a renderGoalTree record.
func RenderGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	roots := buildGoalTree(goalsRaw, ticketsRaw)
	format := "text"
	if useMD {
		format = "md"
	}
	return RenderModelGoalTreeNodes(roots, format)
}

// 💿️goalNodeToData holds the data fields for a goalNodeToData record.
func goalNodeToData(n *model.GoalNode) map[string]interface{} {
	return map[string]interface{}{
		"id":          n.ID,
		"title":       n.Title,
		"status":      n.Status,
		"dueDate":     n.DueDate,
		"createdAt":   n.CreatedAt,
		"description": n.Description,
	}
}

// 🌿️ticketNodeToData holds the data fields for a ticketNodeToData record.
func ticketNodeToData(n *model.TicketNode) map[string]interface{} {
	data := map[string]interface{}{
		"slug":     n.Slug,
		"title":    n.Title,
		"status":   n.Status,
		"started":  n.Created,
		"finished": n.Finished,
		"prompt":   n.Description,
		"summary":  n.Summary,
	}
	return data
}

// ⛳️renderGoalTreeNodes holds the data fields for a renderGoalTreeNodes record.
func RenderModelGoalTreeNodes(roots []*model.GoalNode, format string) string {
	var sb strings.Builder

	var render func(node interface{}, prefix string, isLast bool, isRoot bool)
	render = func(node interface{}, prefix string, isLast bool, isRoot bool) {
		var lineContent string
		var children []interface{}

		switch n := node.(type) {
		case *model.GoalNode:
			data := goalNodeToData(n)
			if format == "md" {
				lineContent = model.RenderEntityMarkdownLink("goal", data)
			} else {
				lineContent = model.RenderEntityHuman("goal", data, false)
			}
			for _, c := range n.Children {
				children = append(children, c)
			}
			for _, t := range n.Tickets {
				children = append(children, t)
			}
		case *model.TicketNode:
			data := ticketNodeToData(n)
			if format == "md" {
				lineContent = model.RenderEntityMarkdownLink("ticket", data)
			} else {
				lineContent = model.RenderEntityHuman("ticket", data, false)
			}
			for _, c := range n.Children {
				children = append(children, c)
			}
		}

		if format == "text" {
			connector := ""
			if !isRoot {
				if isLast {
					connector = "└️─️─️ "
				} else {
					connector = "├️─️─️ "
				}
			}
			sb.WriteString(prefix + connector + lineContent + "\n")

			newPrefix := prefix
			if !isRoot {
				if isLast {
					newPrefix += "    "
				} else {
					newPrefix += "│️   "
				}
			}

			for i, child := range children {
				render(child, newPrefix, i == len(children)-1, false)
			}
		} else {

			sb.WriteString(prefix + "- " + lineContent + "\n")
			newPrefix := prefix + "  "
			for i, child := range children {
				render(child, newPrefix, i == len(children)-1, false)
			}
		}
	}

	for i, root := range roots {
		render(root, "", i == len(roots)-1, true)
	}
	return sb.String()
}

func RenderSectionTree(root *model.Section, isTTY bool, useMD bool) string {
	var sb strings.Builder

	var render func(s *model.Section, prefix string, isLast bool, isRoot bool)
	render = func(s *model.Section, prefix string, isLast bool, isRoot bool) {
		data := map[string]interface{}{
			"path":      s.Path,
			"name":      s.Name,
			"startLine": float64(s.StartLine),
			"endLine":   float64(s.EndLine),
		}

		var lineContent string
		if useMD {
			lineContent = model.RenderEntityMarkdown("section", data)
		} else {
			lineContent = model.RenderEntityHuman("section", data, false)
		}

		if !useMD {
			connector := ""
			if !isRoot {
				if isLast {
					connector = "└️─️─️ "
				} else {
					connector = "├️─️─️ "
				}
			}
			sb.WriteString(prefix + connector + lineContent + "\n")
			newPrefix := prefix
			if !isRoot {
				if isLast {
					newPrefix += "    "
				} else {
					newPrefix += "│️   "
				}
			}
			for i := range s.Children {
				childPtr := &s.Children[i]
				render(childPtr, newPrefix, i == len(s.Children)-1, false)
			}
		} else {
			sb.WriteString(prefix + lineContent + "\n")
			newPrefix := prefix + "  "
			for i := range s.Children {
				childPtr := &s.Children[i]
				render(childPtr, newPrefix, i == len(s.Children)-1, false)
			}
		}
	}

	render(root, "", true, true)
	return sb.String()
}

// 📋️renderTicketList holds the data fields for a renderTicketList record.
func RenderTicketList(ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	var sb strings.Builder

	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			if useMD {
				sb.WriteString(model.RenderEntityMarkdown("ticket", tm) + "\n")
			} else {
				sb.WriteString("  " + model.RenderEntityHuman("ticket", tm, false) + "\n")
			}
		}
	}
	return sb.String()
}

// #endregion 🏩️Tree Logic

// #region 🩻️Monorepo Tree

// 🌳️TreeBuildOptions holds the data fields for a tree build options record.
type TreeBuildOptions struct {
	IncludeSections bool
}

// 🏢️BuildMonorepoTree MUST assemble the monorepo tree from the available context data.
// 🏢️BuildMonorepoTree constructs and returns the monorepo tree structure.
func BuildMonorepoTreeFromRepo(ctx context.Context, opts ...TreeBuildOptions) *TreeNode {
	var options TreeBuildOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	root := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}

	var technologies []model.Technology
	var goals []*model.Goal
	var drafts []*model.Draft
	var policies []statutespkg.PolicyDef
	var contributors []model.Contributor
	var checkpoints []model.Checkpoint
	var allFolders []model.Folder
	var allFiles []model.File
	var sessions []contributorspkg.Session

	var wg sync.WaitGroup
	wg.Add(9)

	go func() {
		defer wg.Done()
		technologies = codebase.LoadTechnologies()
	}()
	go func() {
		defer wg.Done()
		goalCh := make(chan *model.Goal)
		go func() { goalspkg.StreamGoals(ctx, goalCh) }()
		for g := range goalCh {
			goals = append(goals, g)
		}
	}()
	go func() {
		defer wg.Done()
		drafts = todos.ListDrafts(todos.NewFsDraftStore(workspace.GetDraftsPath()))
	}()
	go func() {
		defer wg.Done()
		policyCh := make(chan statutespkg.PolicyDef)
		go func() { statutespkg.StreamPolicies(ctx, policyCh) }()
		for p := range policyCh {
			policies = append(policies, p)
		}
	}()
	go func() {
		defer wg.Done()
		contribCh := make(chan model.Contributor)
		go func() { contributorspkg.StreamContributors(ctx, contribCh) }()
		for c := range contribCh {
			contributors = append(contributors, c)
		}
	}()
	go func() {
		defer wg.Done()
		limit := 100
		checkpoints = contributorspkg.LoadCheckpoints(&limit)
	}()
	go func() {
		defer wg.Done()
		folderCh := make(chan model.Folder)
		go func() { ticketspkg.StreamFolders(ctx, "", folderCh) }()
		for f := range folderCh {
			allFolders = append(allFolders, f)
		}
	}()
	go func() {
		defer wg.Done()
		fileCh := make(chan model.File)
		go func() { ticketspkg.StreamFiles(ctx, "", fileCh) }()
		for f := range fileCh {
			allFiles = append(allFiles, f)
		}
	}()
	go func() {
		defer wg.Done()
		sessionCh := make(chan contributorspkg.Session)
		go func() { contributorspkg.StreamSessions(ctx, sessionCh) }()
		for s := range sessionCh {
			sessions = append(sessions, s)
		}
	}()
	wg.Wait()

	codebaseNode := &TreeNode{Kind: TreeNodeCategory, ID: "codebase", Label: model.EmojiText(model.EmojiCodebase) + "Codebase", URI: "repo://codebase"}
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })

	type folderEntry struct {
		folder *model.Folder
		node   *TreeNode
	}
	globalFolderMap := make(map[string]*folderEntry)
	for fi := range allFolders {
		f := &allFolders[fi]
		fNode := &TreeNode{
			Kind:    TreeNodeFolder,
			ID:      f.GetID(),
			Label:   f.Name,
			URI:     f.GetURI(),
			SubKind: string(f.Kind),
			Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": string(f.Kind)},
		}
		globalFolderMap[f.Path] = &folderEntry{folder: f, node: fNode}
	}

	globalFileNodes := make(map[string]*TreeNode)
	for fi := range allFiles {
		f := &allFiles[fi]
		fileNode := &TreeNode{
			Kind:    TreeNodeFile,
			ID:      f.GetID(),
			Label:   f.Name,
			URI:     f.GetURI(),
			SubKind: f.Kind,
			Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": f.Kind},
		}
		if options.IncludeSections {
			absPath := filepath.Join(workspace.GetRootDir(), f.Path)
			content, err := workspace.ReadTextFile(absPath)
			if err == nil {
				sections := languages.ParseSections(content, f.Path)
				definitions := languages.ParseDefinitions(content, f.Path)
				sections = languages.HydrateSectionsWithDefinitions(sections, definitions)
				for si := range sections {
					s := &sections[si]
					sNode := buildSectionTreeNode(s)
					fileNode.Children = append(fileNode.Children, sNode)
				}
			}
		}
		globalFileNodes[f.Path] = fileNode
	}

	folderRoots := buildFolderRoots(allFolders)
	for _, folderRoot := range folderRoots {
		codebaseNode.Children = append(codebaseNode.Children, folderRoot)
	}
	attachFilesToFolders(codebaseNode, allFiles, globalFileNodes)

	for pi := range technologies {
		p := &technologies[pi]
		pNode := &TreeNode{
			Kind:    TreeNodeTechnology,
			ID:      p.GetID(),
			Label:   p.Name,
			URI:     p.GetURI(),
			SubKind: string(p.Kind),
			Data:    map[string]interface{}{"name": p.Name, "kind": string(p.Kind), "emoji": p.Emoji},
		}
		sort.Slice(p.Bundles, func(i, j int) bool { return p.Bundles[i].Name < p.Bundles[j].Name })
		for bi := range p.Bundles {
			b := &p.Bundles[bi]
			bNode := &TreeNode{
				Kind:    TreeNodeBundle,
				ID:      b.GetID(),
				Label:   b.Name,
				URI:     b.GetURI(),
				SubKind: string(b.Kind),
				Data:    map[string]interface{}{"name": b.Name, "root": b.Root, "kind": string(b.Kind), "emoji": b.Emoji},
			}

			bundleRoot := b.Root
			if b.SourceRoot != "" {
				bundleRoot = b.SourceRoot
			}

			bundleFolderMap := make(map[string]*folderEntry)
			for path, fe := range globalFolderMap {
				if path == bundleRoot || strings.HasPrefix(path, bundleRoot+"/") {
					bundleFolderMap[path] = fe
				}
			}

			for path, fe := range bundleFolderMap {
				parentPath := filepath.Dir(path)
				if _, ok := bundleFolderMap[parentPath]; ok {
					continue
				}
				bNode.Children = append(bNode.Children, fe.node)
			}
			for path, fe := range bundleFolderMap {
				parentPath := filepath.Dir(path)
				if pfe, ok := bundleFolderMap[parentPath]; ok {
					pfe.node.Children = append(pfe.node.Children, fe.node)
				}
			}

			for path, fileNode := range globalFileNodes {
				if path != bundleRoot && !strings.HasPrefix(path, bundleRoot+"/") {
					continue
				}
				folderPath := filepath.Dir(path)
				if fe, ok := bundleFolderMap[folderPath]; ok {
					fe.node.Children = append(fe.node.Children, fileNode)
				} else {
					bNode.Children = append(bNode.Children, fileNode)
				}
			}

			SortTreeChildren(bNode)
			pNode.Children = append(pNode.Children, bNode)
		}
		codebaseNode.Children = append(codebaseNode.Children, pNode)
	}
	SortTreeChildren(codebaseNode)
	root.Children = append(root.Children, codebaseNode)

	goalsNode := &TreeNode{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals"}
	goalMap := make(map[string]*TreeNode)
	sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	for _, g := range goals {
		goalCreatedAt := ""
		gNode := &TreeNode{
			Kind:        TreeNodeGoal,
			ID:          g.GetID(),
			Label:       g.Title,
			URI:         g.GetURI(),
			SubKind:     g.Status,
			Description: g.Description,
			Status:      g.Status,
			Data: map[string]interface{}{
				"id":          g.ID,
				"title":       g.Title,
				"status":      g.Status,
				"dueDate":     g.DueDate,
				"createdAt":   goalCreatedAt,
				"description": g.Description,
			},
		}
		goalMap[g.ID] = gNode
	}
	var rootGoals []*TreeNode
	for _, g := range goals {
		gNode := goalMap[g.ID]
		parentID := ""
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			parentID = g.ID[:idx]
		}
		if parentID != "" && parentID != g.ID {
			if parent, ok := goalMap[parentID]; ok {
				parent.Children = append(parent.Children, gNode)
			} else {
				rootGoals = append(rootGoals, gNode)
			}
		} else {
			rootGoals = append(rootGoals, gNode)
		}
	}
	goalsNode.Children = rootGoals

	ticketCh := make(chan model.Ticket)
	go func() { ticketspkg.StreamTickets(ctx, nil, nil, nil, ticketCh) }()
	var tickets []model.Ticket
	for t := range ticketCh {
		tickets = append(tickets, t)
	}
	for ti := range tickets {
		t := &tickets[ti]
		ticketStarted := t.GetDateStarted().Format(time.RFC3339)
		ticketFinished := ""
		if finished := t.GetDateFinished(); finished != nil {
			ticketFinished = finished.Format(time.RFC3339)
		}
		tNode := &TreeNode{
			Kind:        TreeNodeTicket,
			ID:          t.GetID(),
			Label:       t.Title,
			URI:         t.GetURI(),
			Description: t.Description,
			Year:        t.Year,
			Month:       t.Month,
			Day:         t.Day,
			Status:      string(t.Status),
			Data: map[string]interface{}{
				"year":     float64(t.Year),
				"month":    float64(t.Month),
				"day":      float64(t.Day),
				"slug":     t.Slug,
				"title":    t.Title,
				"status":   string(t.Status),
				"started":  ticketStarted,
				"finished": ticketFinished,
				"prompt":   t.Description,
				"summary":  t.Summary,
				"goalId":   t.Goal,
			},
		}
		if t.Goal != "" {
			if gNode, ok := goalMap[t.Goal]; ok {
				gNode.Children = append(gNode.Children, tNode)
				continue
			}
		}
		goalsNode.Children = append(goalsNode.Children, tNode)
	}
	root.Children = append(root.Children, goalsNode)

	draftsNode := &TreeNode{Kind: TreeNodeCategory, ID: "drafts", Label: "✍️Drafts", URI: "repo://drafts"}
	if drafts != nil {
		for _, d := range drafts {
			dNode := &TreeNode{
				Kind:  TreeNodeDraft,
				ID:    d.GetID(),
				Label: d.ID,
				URI:   d.GetURI(),
				Data:  map[string]interface{}{"id": d.ID, "slug": d.ID},
			}
			draftsNode.Children = append(draftsNode.Children, dNode)
		}
	}
	root.Children = append(root.Children, draftsNode)

	policiesNode := &TreeNode{Kind: TreeNodeCategory, ID: "policies", Label: "🛡️Policies", URI: "repo://policies"}
	sort.Slice(policies, func(i, j int) bool { return policies[i].ID < policies[j].ID })
	for _, p := range policies {
		policyId := model.EmojiText(model.EmojiPolicy) + workspace.Flat(strings.TrimPrefix(p.ID, "/"))
		pNode := &TreeNode{
			Kind:        TreeNodePolicy,
			ID:          fmt.Sprintf("%s/%s", model.EmojiText(model.EmojiPolicy), p.ID),
			Label:       p.Name,
			URI:         "repo://policy/" + policyId,
			Description: p.Description,
			Data:        map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description},
		}
		pNode.Children = buildPolicyEntityKindTree(p.Groups)
		policiesNode.Children = append(policiesNode.Children, pNode)
	}
	root.Children = append(root.Children, policiesNode)

	contributorsNode := &TreeNode{Kind: TreeNodeCategory, ID: "contributors", Label: "🧑️‍💻️Contributors", URI: "repo://contributors"}
	sort.Slice(contributors, func(i, j int) bool { return contributors[i].Alias < contributors[j].Alias })
	for _, c := range contributors {
		label := c.Alias
		if c.Name != "" {
			label = c.Name + " (" + c.Alias + ")"
		}
		contributorData := map[string]interface{}{
			"alias":   c.Alias,
			"aliases": c.Aliases,
			"github":  c.Github,
			"githubs": c.Githubs,
			"name":    c.Name,
			"names":   c.Names,
			"email":   c.Email,
			"emails":  c.Emails,
		}
		cNode := &TreeNode{
			Kind:        TreeNodeContributor,
			ID:          c.GetID(),
			Label:       label,
			URI:         c.GetURI(),
			Contributor: c.Alias,
			Data:        contributorData,
		}
		contributorsNode.Children = append(contributorsNode.Children, cNode)
	}
	root.Children = append(root.Children, contributorsNode)

	checkpointsNode := &TreeNode{Kind: TreeNodeCategory, ID: "checkpoints", Label: "🔀️Checkpoints", URI: "repo://checkpoints"}
	for ci := range checkpoints {
		c := &checkpoints[ci]
		cNode := &TreeNode{
			Kind:  TreeNodeCheckpoint,
			ID:    c.GetID(),
			Label: c.SHA[:8] + " " + c.Title,
			URI:   c.GetURI(),
			Year:  checkpointDatePart(c.Date, 0, 4),
			Month: checkpointDatePart(c.Date, 5, 7),
			Day:   checkpointDatePart(c.Date, 8, 10),
			Data: map[string]interface{}{"sha": c.SHA, "message": c.Title, "authorId": func() string {
				if c.AuthorID != nil {
					return *c.AuthorID
				}
				return ""
			}()},
		}
		checkpointsNode.Children = append(checkpointsNode.Children, cNode)
	}
	root.Children = append(root.Children, checkpointsNode)

	sessionsNode := &TreeNode{Kind: TreeNodeCategory, ID: "sessions", Label: "⚪️Sessions", URI: "repo://sessions"}
	sort.Slice(sessions, func(i, j int) bool { return sessions[i].StartedAt > sessions[j].StartedAt })
	for si := range sessions {
		s := &sessions[si]
		kindEmoji := contributorspkg.SessionKindEmoji(s.Kind)
		label := kindEmoji + " " + s.UUID
		if s.Client != "" {
			label += " (" + s.Client + ")"
		}
		sNode := &TreeNode{
			Kind:    TreeNodeSession,
			ID:      s.GetID(),
			Label:   label,
			URI:     s.GetURI(),
			SubKind: string(s.Kind),
			Year:    s.Year,
			Month:   s.Month,
			Day:     s.Day,
			Status:  string(s.Kind),
			Data: map[string]interface{}{
				"uuid":       s.UUID,
				"kind":       string(s.Kind),
				"client":     s.Client,
				"startedAt":  s.StartedAt,
				"year":       float64(s.Year),
				"month":      float64(s.Month),
				"day":        float64(s.Day),
				"checkpoint": s.Checkpoint,
			},
		}
		sessionsNode.Children = append(sessionsNode.Children, sNode)
	}
	root.Children = append(root.Children, sessionsNode)

	PropagateParentIDs(root, "", DefaultArtifactIdentifier{})
	return root
}

// 💿️PropagateParentIDs holds the data fields for a PropagateParentIDs record.
func PropagateParentIDs(node *TreeNode, parentArtifactID string, identifier ArtifactIdentifier) {
	if node.Data == nil {
		node.Data = map[string]interface{}{}
	}
	entityKind := TreeNodeKindToEntityKind(node.Kind)
	currentID := ""
	if entityKind != "" && node.Data != nil {
		node.Data["parentId"] = parentArtifactID
		currentID = identifier.ArtifactID(entityKind, node.Data)
	} else {
		currentID = parentArtifactID
	}
	for _, child := range node.Children {
		PropagateParentIDs(child, currentID, identifier)
	}
}

// 📑️buildSectionTreeNode holds the data fields for a buildSectionTreeNode record.
func buildSectionTreeNode(s *model.Section) *TreeNode {
	sNode := &TreeNode{
		Kind:  TreeNodeSection,
		ID:    s.GetID(),
		Label: s.Name,
		URI:   s.GetURI(),
		Data:  map[string]interface{}{"path": s.Path, "name": s.Name, "startLine": float64(s.StartLine), "endLine": float64(s.EndLine)},
	}
	for di := range s.Definitions {
		d := &s.Definitions[di]
		dNode := &TreeNode{
			Kind:    TreeNodeDefinition,
			ID:      d.GetID(),
			Label:   d.Name,
			URI:     d.GetURI(),
			SubKind: string(d.Kind),
			Data:    map[string]interface{}{"path": d.FilePath, "name": d.Name, "kind": string(d.Kind), "startLine": float64(d.StartLine), "endLine": float64(d.EndLine)},
		}
		sNode.Children = append(sNode.Children, dNode)
	}
	for ci := range s.Children {
		child := &s.Children[ci]
		sNode.Children = append(sNode.Children, buildSectionTreeNode(child))
	}
	return sNode
}

// 🏗️buildStatuteTree holds the data fields for a buildStatuteTree record.
func BuildStatuteTreeForRepo(kinds []model.Statute) []*TreeNode {
	type treeEntry struct {
		node     *TreeNode
		children map[string]*treeEntry
	}
	rootEntries := make(map[string]*treeEntry)
	orderedRootKeys := []string{}

	for _, k := range kinds {
		parts := strings.Split(string(k), "/")
		current := rootEntries
		currentOrdered := &orderedRootKeys
		for i, part := range parts {
			if _, ok := current[part]; !ok {
				isLeaf := i == len(parts)-1
				meta := k.Info()
				n := &TreeNode{
					Kind:  TreeNodeStatute,
					ID:    string(k),
					Label: part,
					URI:   meta.GetURI(),
				}
				if isLeaf {
					n.Description = meta.Reason
					priorityIcon := "🟢️"
					if meta.Priority == model.BreachPriorityHigh {
						priorityIcon = "🔴️"
					} else if meta.Priority == model.BreachPriorityMedium {
						priorityIcon = "🟡️"
					}
					n.Label = priorityIcon + part
					if meta.Autofixable {
						n.SubKind = "autofixable"
					}
					n.Data = map[string]interface{}{
						"id":          string(k),
						"priority":    string(meta.Priority),
						"autofixable": meta.Autofixable,
						"reason":      meta.Reason,
						"solution":    meta.Solution,
					}
				} else {
					prefix := strings.Join(parts[:i+1], "/")
					n.ID = "breachCategory:" + prefix
					n.URI = "repo://statute/" + workspace.Flat(prefix)
					n.Kind = TreeNodeCategory
				}
				current[part] = &treeEntry{node: n, children: make(map[string]*treeEntry)}
				*currentOrdered = append(*currentOrdered, part)
			}
			entry := current[part]
			current = entry.children
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			currentOrdered = &childKeys
		}
	}

	var buildNodes func(entries map[string]*treeEntry, keys []string) []*TreeNode
	buildNodes = func(entries map[string]*treeEntry, keys []string) []*TreeNode {
		seen := map[string]bool{}
		var result []*TreeNode
		for _, key := range keys {
			if seen[key] {
				continue
			}
			seen[key] = true
			entry := entries[key]
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			sort.Strings(childKeys)
			entry.node.Children = buildNodes(entry.children, childKeys)
			result = append(result, entry.node)
		}
		return result
	}

	return buildNodes(rootEntries, orderedRootKeys)
}

// 🔷️buildTerritoryTree holds the data fields for a buildTerritoryTree record.
func buildTerritoryTree(groups []model.Territory) []*TreeNode {
	var result []*TreeNode
	for _, g := range groups {
		groupNode := &TreeNode{
			Kind:        TreeNodeCategory,
			ID:          fmt.Sprintf("%s%s", model.EmojiText(model.EmojiTerritory), g.Name),
			Label:       g.Name,
			URI:         "repo://territory/" + g.GetID(),
			Description: g.Description,
			SubKind:     "territory",
			Data: map[string]interface{}{
				"name":        g.Name,
				"description": g.Description,
				"scopes":      g.Scopes,
			},
		}
		for _, k := range g.Kinds {
			meta := k.Info()
			priorityIcon := "🟢️"
			if meta.Priority == model.BreachPriorityHigh {
				priorityIcon = "🔴️"
			} else if meta.Priority == model.BreachPriorityMedium {
				priorityIcon = "🟡️"
			}
			label := priorityIcon + workspace.StatutePathToIdValue(string(k))
			vkNode := &TreeNode{
				Kind:        TreeNodeStatute,
				ID:          meta.GetID(),
				Label:       label,
				URI:         meta.GetURI(),
				Description: meta.Reason,
				Data: map[string]interface{}{
					"id":          string(k),
					"priority":    string(meta.Priority),
					"autofixable": meta.Autofixable,
					"reason":      meta.Reason,
					"solution":    meta.Solution,
				},
			}
			if meta.Autofixable {
				vkNode.SubKind = "autofixable"
			}
			groupNode.Children = append(groupNode.Children, vkNode)
		}
		groupNode.Children = append(groupNode.Children, buildTerritoryTree(g.Groups)...)
		result = append(result, groupNode)
	}
	return result
}

// 📶️sortTreeChildren holds the data fields for a sortTreeChildren record.
func SortTreeChildren(node *TreeNode) {
	sort.SliceStable(node.Children, func(i, j int) bool {
		a, b := node.Children[i], node.Children[j]
		if a.Kind != b.Kind {
			aIsFolder := a.Kind == TreeNodeFolder
			bIsFolder := b.Kind == TreeNodeFolder
			if aIsFolder != bIsFolder {
				return aIsFolder
			}
		}
		return a.Label < b.Label
	})
	for _, c := range node.Children {
		SortTreeChildren(c)
	}
}

// 🧹️FilterMonorepoTree MUST preserve the tree structure while removing non-matching nodes.
// 🔍️FilterMonorepoTree filters the monorepo tree based on the given criteria.
func FilterMonorepoTree(root *TreeNode, filter *TreeFilter) *TreeNode {
	if filter == nil {
		return root
	}

	filtered := filterNode(root, filter)
	if filtered == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
	}

	if filter.HasOnlyKinds() || len(filter.ExcludeKinds) > 0 {
		collapseFilteredKinds(filtered, filter)
	}

	return filtered
}

func filterNode(node *TreeNode, filter *TreeFilter) *TreeNode {
	if node.Kind != TreeNodeCategory {
		if !filter.IsKindVisible(node.Kind) {
			var kept []*TreeNode
			for _, c := range node.Children {
				fc := filterNode(c, filter)
				if fc != nil {
					kept = append(kept, fc)
				}
			}
			if len(kept) > 0 {
				copy := *node
				copy.Children = kept
				return &copy
			}
			return nil
		}
		if !filter.MatchesSubKind(node.Kind, node.SubKind) {
			return nil
		}
		if node.Year > 0 && !filter.MatchesDate(node.Year, node.Month, node.Day) {
			return nil
		}
		if node.Status != "" && !filter.MatchesStatus(node.Status) {
			return nil
		}
		if node.Contributor != "" && !filter.MatchesContributor(node.Contributor) {
			return nil
		}
	}

	var kept []*TreeNode
	for _, c := range node.Children {
		fc := filterNode(c, filter)
		if fc != nil {
			kept = append(kept, fc)
		}
	}

	if node.Kind == TreeNodeCategory && len(kept) == 0 {
		return nil
	}

	copy := *node
	copy.Children = kept
	return &copy
}

// 🏷️collapseFilteredKinds holds the data fields for a collapseFilteredKinds record.
func collapseFilteredKinds(node *TreeNode, filter *TreeFilter) {
	var newChildren []*TreeNode
	for _, c := range node.Children {
		if c.Kind != TreeNodeCategory && !filter.IsKindVisible(c.Kind) {
			collapseFilteredKinds(c, filter)
			newChildren = append(newChildren, c.Children...)
		} else {
			collapseFilteredKinds(c, filter)
			newChildren = append(newChildren, c)
		}
	}
	node.Children = newChildren
}

func SearchMonorepoTreeWithCache(ctx context.Context, root *TreeNode, query string) (*TreeNode, error) {
	if query == "" {
		return root, nil
	}
	idx, err := ensureCacheIndexed(ctx, root, nil)
	if err != nil {
		return nil, err
	}

	searchable, err := countSearchableTreeNodes(ctx, root)
	if err != nil {
		_ = idx.Close()
		return nil, err
	}
	matchedIDs, err := queryCacheIndex(ctx, idx, query, searchable, nil)
	if err != nil {
		_ = idx.Close()
		return nil, err
	}
	if err := idx.Close(); err != nil {
		return nil, err
	}
	if len(matchedIDs) == 0 {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}, nil
	}
	pruned, err := pruneUnmatchedContext(ctx, root, sliceToSet(matchedIDs))
	if err != nil {
		return nil, err
	}
	if pruned == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}, nil
	}
	return pruned, nil
}

// 📝️SearchMonorepoTree MUST match case-insensitively against node labels and descriptions.
// 🔤️SearchMonorepoTree performs a text search across the monorepo tree.
func SearchMonorepoTree(root *TreeNode, query string) *TreeNode {
	if query == "" {
		return root
	}
	return SearchTreeInMemory(root, query)
}

func countSearchableTreeNodes(ctx context.Context, root *TreeNode) (int, error) {
	if root == nil {
		return 0, errors.New("search tree is required")
	}
	count := 0
	visited := 0
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return 0, err
		}
		last := len(stack) - 1
		node := stack[last]
		stack = stack[:last]
		visited++
		if visited > search.MaxTraversalNodes {
			return 0, fmt.Errorf("%w: tree nodes > %d", search.ErrTooLarge, search.MaxTraversalNodes)
		}
		if node.Kind != TreeNodeCategory {
			count++
			if count > search.MaxDocuments {
				return 0, fmt.Errorf("%w: tree documents > %d", search.ErrTooLarge, search.MaxDocuments)
			}
		}
		stack = append(stack, node.Children...)
	}
	if count == 0 {
		return 1, nil
	}
	return count, nil
}

func sliceToSet(values []string) map[string]bool {
	result := make(map[string]bool, len(values))
	for _, value := range values {
		result[value] = true
	}
	return result
}

func serializeTreeNodeData(data map[string]interface{}) string {
	if len(data) == 0 {
		return ""
	}
	parts := make([]string, 0, len(data))
	for _, value := range data {
		switch typed := value.(type) {
		case string:
			parts = append(parts, typed)
		case []string:
			parts = append(parts, strings.Join(typed, " "))
		case []interface{}:
			var values []string
			for _, item := range typed {
				values = append(values, fmt.Sprint(item))
			}
			parts = append(parts, strings.Join(values, " "))
		default:
			parts = append(parts, fmt.Sprint(typed))
		}
	}
	return strings.Join(parts, " ")
}

func buildSearchDocumentText(node *TreeNode) string {
	parts := []string{
		node.Label,
		node.ID,
		node.SubKind,
		node.Description,
		node.URI,
		node.Status,
		node.Contributor,
		string(node.Kind),
		serializeTreeNodeData(node.Data),
	}
	for _, child := range node.Children {
		if child.Kind != TreeNodeCategory {
			parts = append(parts, child.Label, child.ID, child.Description, serializeTreeNodeData(child.Data))
		}
	}
	return strings.Join(parts, " ")
}

// 🔶️levenshtein holds the data fields for a levenshtein record.
func levenshtein(a, b string) int {
	la, lb := len(a), len(b)
	if la == 0 {
		return lb
	}
	if lb == 0 {
		return la
	}
	prev := make([]int, lb+1)
	curr := make([]int, lb+1)
	for j := 0; j <= lb; j++ {
		prev[j] = j
	}
	for i := 1; i <= la; i++ {
		curr[0] = i
		for j := 1; j <= lb; j++ {
			cost := 0
			if a[i-1] != b[j-1] {
				cost = 1
			}
			ins := curr[j-1] + 1
			del := prev[j] + 1
			sub := prev[j-1] + cost
			m := ins
			if del < m {
				m = del
			}
			if sub < m {
				m = sub
			}
			curr[j] = m
		}
		prev, curr = curr, prev
	}
	return prev[lb]
}

// 🔹️fuzzyContains holds the data fields for a fuzzyContains record.
func fuzzyContains(text, term string) bool {
	if strings.Contains(text, term) {
		return true
	}
	maxDist := 1
	if len(term) > 5 {
		maxDist = 2
	}
	words := strings.Fields(text)
	for _, w := range words {
		if levenshtein(w, term) <= maxDist {
			return true
		}
		if len(w) >= len(term) {
			for i := 0; i <= len(w)-len(term)+maxDist && i <= len(w)-1; i++ {
				end := i + len(term) + maxDist
				if end > len(w) {
					end = len(w)
				}
				sub := w[i:end]
				if levenshtein(sub, term) <= maxDist {
					return true
				}
			}
		}
	}
	return false
}

// 🔎️searchTreeInMemory holds the data fields for a searchTreeInMemory record.
func SearchTreeInMemory(root *TreeNode, query string) *TreeNode {
	terms := strings.Fields(strings.ToLower(query))
	if len(terms) == 0 {
		return root
	}
	matchedIDs := make(map[string]bool)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind != TreeNodeCategory {
			docID := node.ID
			if docID == "" {
				docID = node.Label
			}
			text := strings.ToLower(buildSearchDocumentText(node))
			allMatch := true
			for _, term := range terms {
				if !fuzzyContains(text, term) {
					allMatch = false
					break
				}
			}
			if allMatch {
				matchedIDs[docID] = true
			}
		}
		for _, c := range node.Children {
			walk(c)
		}
	}
	walk(root)
	if len(matchedIDs) == 0 {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
	}
	pruned := pruneUnmatched(root, matchedIDs)
	if pruned == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
	}
	return pruned
}

// 🎯️pruneUnmatched holds the data fields for a pruneUnmatched record.
func pruneUnmatched(node *TreeNode, matchedIDs map[string]bool) *TreeNode {
	result, _ := pruneUnmatchedContext(context.Background(), node, matchedIDs)
	return result
}

func pruneUnmatchedContext(ctx context.Context, node *TreeNode, matchedIDs map[string]bool) (*TreeNode, error) {
	return pruneUnmatchedInner(ctx, node, matchedIDs, false)
}

// 🔸️pruneUnmatchedInner holds the data fields for a pruneUnmatchedInner record.
func pruneUnmatchedInner(ctx context.Context, node *TreeNode, matchedIDs map[string]bool, ancestorMatched bool) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	docID := node.ID
	if docID == "" {
		docID = node.Label
	}
	selfMatched := matchedIDs[docID]
	keepDescendants := ancestorMatched || selfMatched
	var kept []*TreeNode
	for _, c := range node.Children {
		fc, err := pruneUnmatchedInner(ctx, c, matchedIDs, keepDescendants)
		if err != nil {
			return nil, err
		}
		if fc != nil {
			kept = append(kept, fc)
		}
	}
	if keepDescendants || len(kept) > 0 || node.Kind == TreeNodeCategory {
		if node.Kind == TreeNodeCategory && len(kept) == 0 && !selfMatched && !ancestorMatched {
			return nil, nil
		}
		copy := *node
		copy.Children = kept
		return &copy, nil
	}
	return nil, nil
}

// 🎨️RenderMonorepoTree MUST produce a complete monorepo tree output.
// 🎨️RenderMonorepoTree renders the monorepo tree into its output representation.
func RenderMonorepoTree(root *TreeNode, renderer EntityRenderer) string {
	var sb strings.Builder
	for i, c := range root.Children {
		RenderTreeNodeText(&sb, c, "", i == len(root.Children)-1, true, renderer)
	}
	return sb.String()
}

// 📰️RenderMonorepoTreeMarkdown MUST produce a complete monorepo tree markdown output.
// 📰️RenderMonorepoTreeMarkdown renders the monorepo tree markdown into its output representation.
func RenderMonorepoTreeMarkdown(root *TreeNode, renderer EntityRenderer) string {
	var sb strings.Builder
	for _, c := range root.Children {
		RenderTreeNodeMarkdown(&sb, c, "", renderer)
	}
	return sb.String()
}

// 🌿️treeNodeKindToEntityKind holds the data fields for a treeNodeKindToEntityKind record.
func TreeNodeKindToEntityKind(k TreeNodeKind) string {
	switch k {
	case TreeNodeTechnology:
		return "technology"
	case TreeNodeBundle:
		return "bundle"
	case TreeNodeFolder:
		return "folder"
	case TreeNodeFile:
		return "file"
	case TreeNodeSection:
		return "section"
	case TreeNodeDefinition:
		return "definition"
	case TreeNodeGoal:
		return "goal"
	case TreeNodeTicket:
		return "ticket"
	case TreeNodeDraft:
		return "draft"
	case TreeNodeTodo:
		return "todo"
	case TreeNodePolicy:
		return "policy"
	case TreeNodeBreach:
		return "breach"
	case TreeNodeContributor:
		return "contributor"
	case TreeNodeCheckpoint:
		return "checkpoint"
	case TreeNodeSession:
		return "session"
	case TreeNodeCategory:
		return ""
	}
	return ""
}

// 🔺️renderTreeNodeText holds the data fields for a renderTreeNodeText record.
func RenderTreeNodeText(sb *strings.Builder, node *TreeNode, prefix string, isLast bool, isRoot bool, renderer EntityRenderer) {
	connector := "├️─️─️ "
	if isLast {
		connector = "└️─️─️ "
	}
	if isRoot {
		connector = ""
	}

	label := node.Label
	if node.Kind == TreeNodeCategory {
		if node.URI != "" {
			label = "[" + node.Label + "](" + node.URI + ")"
		}
	} else if node.Data != nil {
		entityKind := TreeNodeKindToEntityKind(node.Kind)
		if entityKind == "" {
			entityKind = string(node.Kind)
		}
		origParentId, hadParentId := node.Data["parentId"]
		node.Data["parentId"] = ""
		result := renderer.Human(entityKind, node.Data)
		if hadParentId {
			node.Data["parentId"] = origParentId
		} else {
			delete(node.Data, "parentId")
		}
		if result != "" {
			label = result
		}
	}

	sb.WriteString(prefix + connector + label + "\n")

	newPrefix := prefix
	if !isRoot {
		if isLast {
			newPrefix += "    "
		} else {
			newPrefix += "│️   "
		}
	}

	for i, c := range node.Children {
		RenderTreeNodeText(sb, c, newPrefix, i == len(node.Children)-1, false, renderer)
	}
}

// 🔻️renderTreeNodeMarkdown holds the data fields for a renderTreeNodeMarkdown record.
func RenderTreeNodeMarkdown(sb *strings.Builder, node *TreeNode, indent string, renderer EntityRenderer) {
	if node.Kind == TreeNodeCategory {
		sb.WriteString(model.RenderTemplate(model.MdTpl, "md/tree_node_category", map[string]interface{}{
			"Indent": indent,
			"Label":  node.Label,
			"URI":    node.URI,
		}) + "\n")
	} else if node.Data != nil {
		entityKind := TreeNodeKindToEntityKind(node.Kind)
		if entityKind == "" {
			entityKind = string(node.Kind)
		}
		sb.WriteString(model.RenderTemplate(model.MdTpl, "md/tree_node_entity", map[string]interface{}{
			"Indent":  indent,
			"Content": renderer.Markdown(entityKind, node.Data),
		}) + "\n")
	} else {
		sb.WriteString(model.RenderTemplate(model.MdTpl, "md/tree_node_category", map[string]interface{}{
			"Indent": indent,
			"Label":  node.Label,
			"URI":    node.URI,
		}) + "\n")
	}
	for _, c := range node.Children {
		RenderTreeNodeMarkdown(sb, c, indent+"  ", renderer)
	}
}

// #endregion 🩻️Monorepo Tree

// #region 🎊️Query Cache

// #region 🎊️Query Cache
// Local event index under .🦑️repo/⚡️cache for keyword search. Uses a composite repository fingerprint for invalidation.
// 📌️cacheSchemaVersion holds the data fields for a cacheSchemaVersion record.
const cacheSchemaVersion = 3

// 💿️cacheMeta holds the data fields for a cacheMeta record.
type cacheMeta struct {
	SchemaVersion     int               `json:"SchemaVersion"`
	SuperHead         string            `json:"SuperHead"`
	SuperDirtyHash    string            `json:"SuperDirtyHash"`
	SubmodulePointers map[string]string `json:"SubmodulePointers"`
	SubmoduleHeads    map[string]string `json:"SubmoduleHeads,omitempty"`
	SubmoduleDirty    map[string]string `json:"SubmoduleDirty,omitempty"`
	PointersHash      string            `json:"PointersHash"`
	SubWorkingHash    string            `json:"SubWorkingHash"`
	Fingerprint       string            `json:"Fingerprint"`
	IncludeSections   bool              `json:"IncludeSections"`
}

// 💾️getCacheDir holds the data fields for a getCacheDir record.
func getCacheDir() string {
	abs, _ := filepath.Abs(workspace.GetRootDir())
	h := sha256.Sum256([]byte(abs))
	return filepath.Join(workspace.GetRepoMetaDir(), "⚡️cache", hex.EncodeToString(h[:]))
}

func computeCompositeFingerprint(repoRoot string) (fp string, meta *cacheMeta) {
	fp, meta, _ = computeCompositeFingerprintContext(context.Background(), repoRoot)
	return fp, meta
}

func computeCompositeFingerprintContext(ctx context.Context, repoRoot string) (fp string, meta *cacheMeta, err error) {
	meta = &cacheMeta{
		SchemaVersion:     cacheSchemaVersion,
		SubmodulePointers: map[string]string{},
		SubmoduleHeads:    map[string]string{},
		SubmoduleDirty:    map[string]string{},
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	stdout, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SuperHead = strings.TrimSpace(stdout)
	statusOut, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SuperDirtyHash = hashString(statusOut)
	subOut, err := getSubmoduleStatusContext(ctx, repoRoot)
	if err != nil {
		return "", nil, err
	}
	lines := strings.Split(strings.TrimSpace(subOut), "\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		if len(parts) < 2 {
			continue
		}
		path := parts[1]
		meta.SubmodulePointers[path] = parts[0]
		if strings.HasPrefix(parts[0], "-") {
			continue
		}
		if err := ctx.Err(); err != nil {
			return "", nil, err
		}
		subPath := filepath.Join(repoRoot, path)
		subHead, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, subPath)
		meta.SubmoduleHeads[path] = strings.TrimSpace(subHead)
		subStatus, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, subPath)
		if err := ctx.Err(); err != nil {
			return "", nil, err
		}
		meta.SubmoduleDirty[path] = hashString(subStatus)
	}
	meta.PointersHash = hashString(subOut)
	paths := make([]string, 0, len(meta.SubmoduleHeads))
	for k := range meta.SubmoduleHeads {
		paths = append(paths, k)
	}
	sort.Strings(paths)
	var subWork []string
	for _, path := range paths {
		subWork = append(subWork, path+"="+meta.SubmoduleHeads[path]+":"+meta.SubmoduleDirty[path])
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	composeMetaHash := hashComposeMetaState(repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SubWorkingHash = hashString(strings.Join(subWork, "|"))
	fp = hashString(meta.SuperHead + meta.SuperDirtyHash + meta.PointersHash + meta.SubWorkingHash + composeMetaHash + strconv.Itoa(cacheSchemaVersion))
	meta.Fingerprint = fp
	return fp, meta, nil
}

// 🧭️getSubmoduleStatus returns recursive submodule status when available.
func getSubmoduleStatus(repoRoot string) string {
	stdout, _ := getSubmoduleStatusContext(context.Background(), repoRoot)
	return stdout
}

func getSubmoduleStatusContext(ctx context.Context, repoRoot string) (string, error) {
	stdout, _, exitCode := workspace.ExecCommandContext(ctx, "git", []string{"submodule", "status", "--recursive"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if exitCode == 0 {
		return stdout, nil
	}
	fallback, _, fallbackExitCode := workspace.ExecCommandContext(ctx, "git", []string{"submodule", "status"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if fallbackExitCode == 0 {
		return fallback, nil
	}
	return stdout, nil
}

// ♻️hashComposeMetaState MUST produce a stable hash for compose metadata state changes.
// hashComposeMetaState computes and returns a hash for compose metadata content relevant to tree cache invalidation.
// Only structural changes (new goals, tickets, policies, drafts) invalidate the cache.
// ✏️Ephemeral data (agent session logs, ticket agent tracking updates) is excluded.
func hashComposeMetaState(repoRoot string) string {
	metaRoot := workspace.RepoMetaDirForRoot(repoRoot)
	if !workspace.FileExists(metaRoot) {
		return ""
	}
	var parts []string
	for _, top := range []string{"🎯️goals", "👮️", "📝️", "📊️metrics", "✍️notes"} {
		p := filepath.Join(metaRoot, top)
		if st, err := os.Stat(p); err == nil {
			parts = append(parts, "d:"+top+":"+strconv.FormatInt(st.ModTime().UnixNano(), 10))
		}
	}
	tickets, _ := filepath.Glob(filepath.Join(metaRoot, "🎫️tickets", "*", "*", "*", "*"))
	sort.Strings(tickets)
	parts = append(parts, "tickets:"+strings.Join(tickets, ","))
	return hashString(strings.Join(parts, "|"))
}

func treeNodeScopePath(node *TreeNode) string {
	if node.Data == nil {
		return ""
	}
	switch node.Kind {
	case TreeNodeFile, TreeNodeFolder:
		if p, ok := node.Data["path"].(string); ok {
			return p
		}
	case TreeNodeBundle:
		if r, ok := node.Data["root"].(string); ok {
			return r
		}
	case TreeNodeTechnology:
		if n, ok := node.Data["name"].(string); ok {
			return "technology:" + n
		}
	}
	return ""
}

// 🔤️hashString holds the data fields for a hashString record.
func hashString(s string) string {
	h := sha256.Sum256([]byte(s))
	return hex.EncodeToString(h[:])
}

// 🔷️loadCacheMeta holds the data fields for a loadCacheMeta record.
func loadCacheMeta(indexPath string) (*cacheMeta, error) {
	p := filepath.Join(indexPath, "meta.json")
	data, err := os.ReadFile(p)
	if err != nil {
		return nil, err
	}
	var m cacheMeta
	if err := json.Unmarshal(data, &m); err != nil {
		return nil, err
	}
	if m.SchemaVersion != cacheSchemaVersion {
		return nil, fmt.Errorf("schema version mismatch")
	}
	return &m, nil
}

// 🔶️saveCacheMeta holds the data fields for a saveCacheMeta record.
func saveCacheMeta(indexPath string, m *cacheMeta) error {
	dir := indexPath
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(m, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(dir, "meta.json"), data, 0644)
}

// 🛤️getChangedPathsFromGit holds the data fields for a getChangedPathsFromGit record.
func getChangedPathsFromGit(repoRoot string) []string {
	statusOut, _, _ := workspace.ExecCommand("git", []string{"status", "--porcelain", "-z"}, repoRoot)
	diffOut, _, _ := workspace.ExecCommand("git", []string{"diff", "--name-only", "HEAD"}, repoRoot)
	untrackedOut, _, _ := workspace.ExecCommand("git", []string{"ls-files", "-o", "--exclude-standard"}, repoRoot)
	paths := make(map[string]bool)
	for _, s := range strings.Split(strings.TrimSpace(statusOut), "\x00") {
		s = strings.TrimSpace(s)
		if len(s) >= 4 {
			paths[strings.TrimSpace(s[3:])] = true
		}
	}
	for _, p := range strings.Split(strings.TrimSpace(diffOut), "\n") {
		p = strings.TrimSpace(p)
		if p != "" {
			paths[p] = true
		}
	}
	for _, p := range strings.Split(strings.TrimSpace(untrackedOut), "\n") {
		if p != "" {
			paths[p] = true
		}
	}
	var result []string
	for p := range paths {
		result = append(result, p)
	}
	return result
}

// 🔹️expandPathsWithAncestors holds the data fields for a expandPathsWithAncestors record.
func expandPathsWithAncestors(paths []string) []string {
	technologies := codebase.LoadTechnologies()
	bundleByPath := make(map[string]string)
	technologyByBundle := make(map[string]string)
	for _, p := range technologies {
		for _, b := range p.Bundles {
			root := b.Root
			if b.SourceRoot != "" {
				root = b.SourceRoot
			}
			bundleByPath[root] = root
			technologyByBundle[root] = "technology:" + p.Name
		}
	}
	expanded := make(map[string]bool)
	for _, p := range paths {
		expanded[p] = true
		dir := p
		for {
			dir = filepath.Dir(dir)
			if dir == "." || dir == "" {
				break
			}
			expanded[dir] = true
		}
		var bundle string
		for root := range bundleByPath {
			if strings.HasPrefix(p, root+"/") || p == root {
				if len(root) > len(bundle) {
					bundle = root
				}
			}
		}
		if bundle != "" {
			expanded[bundle] = true
			if proj := technologyByBundle[bundle]; proj != "" {
				expanded[proj] = true
			}
		}
	}
	var result []string
	for p := range expanded {
		result = append(result, p)
	}
	return result
}

func pathToNodesMap(root *TreeNode) map[string][]*TreeNode {
	m := make(map[string][]*TreeNode)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind != TreeNodeCategory {
			p := treeNodeScopePath(node)
			if p != "" {
				m[p] = append(m[p], node)
			}
		}
		for _, c := range node.Children {
			walk(c)
		}
	}
	walk(root)
	return m
}

const maxSearchCacheLockAttempts = 60

// 🔸️ensureCacheIndexed builds a bounded staged index and atomically commits it after verification.
func ensureCacheIndexed(ctx context.Context, root *TreeNode, progress func(search.Progress)) (search.Index, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	repoRoot := workspace.GetRootDir()
	reportSearchProgress(progress, search.Progress{Step: "fingerprinting"})
	fingerprint, newMeta, err := computeCompositeFingerprintContext(ctx, repoRoot)
	if err != nil {
		return nil, err
	}
	dir := getCacheDir()
	indexPath := filepath.Join(dir, "index.search")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}
	release, err := acquireSearchCacheLock(ctx, filepath.Join(dir, ".lock"), progress)
	if err != nil {
		return nil, err
	}
	defer release()
	if err := recoverSearchIndexSwap(indexPath); err != nil {
		return nil, err
	}
	existing, metaErr := loadCacheMeta(indexPath)
	if metaErr == nil && existing.Fingerprint == fingerprint {
		index, err := search.Open(indexPath)
		if err != nil {
			return nil, fmt.Errorf("open current search index: %w", err)
		}
		reportSearchProgress(progress, search.Progress{Current: 1, Total: 1, Step: "ready"})
		return index, nil
	}
	if metaErr != nil && !os.IsNotExist(metaErr) {
		return nil, fmt.Errorf("load search index metadata: %w", metaErr)
	}
	nodes, err := collectSearchIndexNodes(ctx, root, progress)
	if err != nil {
		return nil, err
	}
	stagedPath := indexPath + ".next"
	if err := os.RemoveAll(stagedPath); err != nil {
		return nil, err
	}
	staged, err := search.New(stagedPath, search.NewIndexMapping())
	if err != nil {
		return nil, err
	}
	cleanup := func() { _ = os.RemoveAll(stagedPath) }
	for offset, node := range nodes {
		if err := ctx.Err(); err != nil {
			cleanup()
			return nil, err
		}
		documentID := node.ID
		if documentID == "" {
			documentID = node.Label
		}
		document := map[string]interface{}{"text": buildSearchDocumentText(node)}
		if path := treeNodeScopePath(node); path != "" {
			document["path"] = path
		}
		if err := staged.IndexContext(ctx, documentID, document, nil); err != nil {
			cleanup()
			return nil, fmt.Errorf("index %q: %w", documentID, err)
		}
		reportSearchProgress(progress, search.Progress{Current: offset + 1, Total: len(nodes), Step: "indexed"})
	}
	if err := ctx.Err(); err != nil {
		cleanup()
		return nil, err
	}
	if err := staged.CloseContext(ctx, func(value search.Progress) {
		value.Step = "persisting-" + value.Step
		reportSearchProgress(progress, value)
	}); err != nil {
		cleanup()
		return nil, fmt.Errorf("persist staged search index: %w", err)
	}
	if err := saveCacheMeta(stagedPath, newMeta); err != nil {
		cleanup()
		return nil, err
	}
	verified, err := search.Open(stagedPath)
	if err != nil {
		cleanup()
		return nil, fmt.Errorf("verify staged search index: %w", err)
	}
	if err := verified.Close(); err != nil {
		cleanup()
		return nil, err
	}
	if err := ctx.Err(); err != nil {
		cleanup()
		return nil, err
	}
	if err := commitSearchIndexSwap(indexPath, stagedPath); err != nil {
		cleanup()
		return nil, err
	}
	reportSearchProgress(progress, search.Progress{Current: len(nodes), Total: len(nodes), Step: "committed"})
	index, err := search.Open(indexPath)
	if err != nil {
		return nil, fmt.Errorf("open committed search index: %w", err)
	}
	return index, nil
}

func acquireSearchCacheLock(ctx context.Context, path string, progress func(search.Progress)) (func(), error) {
	for attempt := 0; attempt < maxSearchCacheLockAttempts; attempt++ {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		lock, err := os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o644)
		if err == nil {
			if err := lock.Close(); err != nil {
				_ = os.Remove(path)
				return nil, err
			}
			return func() { _ = os.Remove(path) }, nil
		}
		if !os.IsExist(err) {
			return nil, err
		}
		if info, statErr := os.Stat(path); statErr == nil && time.Since(info.ModTime()) > 5*time.Minute {
			if err := os.Remove(path); err != nil && !os.IsNotExist(err) {
				return nil, err
			}
			continue
		}
		reportSearchProgress(progress, search.Progress{Current: attempt + 1, Total: maxSearchCacheLockAttempts, Step: "waiting-lock"})
		timer := time.NewTimer(200 * time.Millisecond)
		select {
		case <-ctx.Done():
			if !timer.Stop() {
				<-timer.C
			}
			return nil, ctx.Err()
		case <-timer.C:
		}
	}
	return nil, errors.New("search index lock timeout")
}

func collectSearchIndexNodes(ctx context.Context, root *TreeNode, progress func(search.Progress)) ([]*TreeNode, error) {
	if root == nil {
		return nil, errors.New("search tree is required")
	}
	stack := []*TreeNode{root}
	nodes := make([]*TreeNode, 0)
	visited := 0
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		last := len(stack) - 1
		node := stack[last]
		stack = stack[:last]
		visited++
		if visited > search.MaxTraversalNodes {
			return nil, fmt.Errorf("%w: tree nodes > %d", search.ErrTooLarge, search.MaxTraversalNodes)
		}
		if node.Kind != TreeNodeCategory {
			if len(nodes) >= search.MaxDocuments {
				return nil, fmt.Errorf("%w: tree documents > %d", search.ErrTooLarge, search.MaxDocuments)
			}
			nodes = append(nodes, node)
		}
		for index := len(node.Children) - 1; index >= 0; index-- {
			stack = append(stack, node.Children[index])
		}
		reportSearchProgress(progress, search.Progress{Current: visited, Step: "collecting"})
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	return nodes, nil
}

func recoverSearchIndexSwap(indexPath string) error {
	stagedPath := indexPath + ".next"
	backupPath := indexPath + ".previous"
	_, backupErr := os.Stat(backupPath)
	if backupErr == nil {
		if _, currentErr := os.Stat(indexPath); os.IsNotExist(currentErr) {
			if err := os.Rename(backupPath, indexPath); err != nil {
				return err
			}
		} else if currentErr != nil {
			return currentErr
		} else if err := os.RemoveAll(backupPath); err != nil {
			return err
		}
	} else if !os.IsNotExist(backupErr) {
		return backupErr
	}
	return os.RemoveAll(stagedPath)
}

func commitSearchIndexSwap(indexPath, stagedPath string) error {
	backupPath := indexPath + ".previous"
	hadCurrent := false
	if _, err := os.Stat(indexPath); err == nil {
		hadCurrent = true
		if err := os.Rename(indexPath, backupPath); err != nil {
			return err
		}
	} else if !os.IsNotExist(err) {
		return err
	}
	if err := os.Rename(stagedPath, indexPath); err != nil {
		if hadCurrent {
			_ = os.Rename(backupPath, indexPath)
		}
		return err
	}
	if hadCurrent {
		_ = os.RemoveAll(backupPath)
	}
	return nil
}

func reportSearchProgress(progress func(search.Progress), value search.Progress) {
	if progress != nil {
		progress(value)
	}
}

// 🔍️queryCacheIndex queries the owned index with cancellation and progress.
func queryCacheIndex(ctx context.Context, idx search.Index, query string, limit int, progress func(int, int)) ([]string, error) {
	if query == "" {
		return nil, nil
	}
	terms := strings.Fields(query)
	queries := make([]search.Query, 0, len(terms))
	for _, term := range terms {
		match := search.NewMatchQuery(term)
		match.SetFuzziness(2)
		queries = append(queries, match)
	}
	var q search.Query
	switch len(queries) {
	case 0:
		return nil, nil
	case 1:
		q = queries[0]
	default:
		q = search.NewConjunctionQuery(queries...)
	}
	req := search.NewSearchRequest(q)
	req.Size = limit
	results, err := idx.SearchContext(ctx, req, progress)
	if err != nil {
		return nil, err
	}
	var ids []string
	for _, hit := range results.Hits {
		ids = append(ids, hit.ID)
	}
	return ids, nil
}

// #endregion 🎊️Query Cache

// #region 📌️Tree Cache

// 🌳️getTreeCachePath holds the data fields for a getTreeCachePath record.
func getTreeCachePath() string {
	return filepath.Join(getCacheDir(), "tree.json.gz")
}

func getTreeCacheMetaPath() string {
	return filepath.Join(getCacheDir(), "tree-meta.json")
}

// 💿️saveTreeCache holds the data fields for a saveTreeCache record.
func saveTreeCache(tree *TreeNode, meta *cacheMeta) error {
	dir := getCacheDir()
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	data, err := json.Marshal(tree)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	gz, err := gzip.NewWriterLevel(&buf, gzip.BestSpeed)
	if err != nil {
		return err
	}
	if _, err := gz.Write(data); err != nil {
		gz.Close()
		return err
	}
	if err := gz.Close(); err != nil {
		return err
	}
	if err := os.WriteFile(getTreeCachePath(), buf.Bytes(), 0644); err != nil {
		return err
	}
	metaData, err := json.MarshalIndent(meta, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(getTreeCacheMetaPath(), metaData, 0644)
}

func loadTreeCache() (*TreeNode, *cacheMeta, error) {
	metaData, err := os.ReadFile(getTreeCacheMetaPath())
	if err != nil {
		return nil, nil, err
	}
	var meta cacheMeta
	if err := json.Unmarshal(metaData, &meta); err != nil {
		return nil, nil, err
	}
	if meta.SchemaVersion != cacheSchemaVersion {
		return nil, nil, fmt.Errorf("schema version mismatch")
	}
	compressed, err := os.ReadFile(getTreeCachePath())
	if err != nil {
		return nil, nil, err
	}
	gz, err := gzip.NewReader(bytes.NewReader(compressed))
	if err != nil {
		return nil, nil, err
	}
	defer gz.Close()
	data, err := io.ReadAll(gz)
	if err != nil {
		return nil, nil, err
	}
	var tree TreeNode
	if err := json.Unmarshal(data, &tree); err != nil {
		return nil, nil, err
	}
	return &tree, &meta, nil
}

// 🏢️BuildMonorepoTreeCached holds the data fields for a BuildMonorepoTreeCached record.
func BuildMonorepoTreeCached(ctx context.Context, opts ...TreeBuildOptions) *TreeNode {
	var buildOpts TreeBuildOptions
	if len(opts) > 0 {
		buildOpts = opts[0]
	}
	repoRoot := workspace.GetRootDir()

	// Acquire a directory-based lock so only one CLI process builds the tree at a time.
	// Other concurrent processes will block here and then use the cache.
	lockPath := filepath.Join(getCacheDir(), "tree.lock")
	os.MkdirAll(filepath.Dir(lockPath), 0755)
	for {
		err := os.Mkdir(lockPath, 0755)
		if err == nil {
			break
		}
		// If the lock directory exists, another process holds the lock.
		// Check if it's stale (older than 60 seconds) and remove it.
		if info, statErr := os.Stat(lockPath); statErr == nil {
			if time.Since(info.ModTime()) > 60*time.Second {
				os.RemoveAll(lockPath)
				continue
			}
		}
		time.Sleep(100 * time.Millisecond)
	}
	defer os.RemoveAll(lockPath)

	fp, newMeta := computeCompositeFingerprint(repoRoot)
	newMeta.IncludeSections = buildOpts.IncludeSections
	cached, cachedMeta, err := loadTreeCache()
	if err == nil && cachedMeta.Fingerprint == fp && cachedMeta.IncludeSections == buildOpts.IncludeSections {
		return cached
	}
	tree := BuildMonorepoTreeFromRepo(ctx, buildOpts)
	saveTreeCache(tree, newMeta)
	return tree
}

// #endregion 📌️Tree Cache

// #region ⏲️Mermaid

// #region ⏲️Mermaid
// 🧜️Mermaid diagram generation for LOC visualizations as treemap-beta strings.
func mermaidEscapeLabel(s string) string {
	return strings.ReplaceAll(s, "\"", "'")
}

func mermaidTechnologyEmoji(kind model.TechnologyKind) string {
	switch kind {
	case model.TechnologyKindInfrastructure:
		return model.EmojiTechnologyInfra
	case model.TechnologyKindResearch:
		return model.EmojiTechnologyResearch
	default:
		return model.EmojiTechnologyUser
	}
}

func mermaidBundleEmoji(kind model.BundleKind) string {
	switch kind {
	case model.BundleKindSchema:
		return model.EmojiBundleSchema
	case model.BundleKindBinary:
		return model.EmojiBundleBinary
	case model.BundleKindUI:
		return model.EmojiBundleUI
	case model.BundleKindSite:
		return model.EmojiBundleSite
	case model.BundleKindAssets:
		return model.EmojiBundleAssets
	default:
		return model.EmojiBundleLibrary
	}
}

func mermaidFileEmoji(kind string) string {
	switch kind {
	case model.FileKindLab:
		return model.EmojiFileLab
	case model.FileKindScript:
		return model.EmojiFileScript
	case model.FileKindDocs:
		return model.EmojiFileDocs
	case model.FileKindConfig:
		return model.EmojiFileConfig
	case model.FileKindResource:
		return model.EmojiFileResource
	case model.FileKindTemplate:
		return model.EmojiFileTemplate
	case model.FileKindLicense:
		return model.EmojiFileLicense
	default:
		return model.EmojiFileCode
	}
}

func MermaidLocByTechnologiesBundlesFoldersFiles() string {
	technologies := codebase.LoadTechnologies()
	ctx := context.Background()
	fileCh := make(chan model.File)
	go func() { ticketspkg.StreamFiles(ctx, "", fileCh) }()
	var allFiles []model.File
	for f := range fileCh {
		if f.Ignored || f.Generated {
			continue
		}
		allFiles = append(allFiles, f)
	}
	type fileEntry struct {
		Name string
		Kind string
		LOC  int
	}
	type folderEntry struct {
		Name  string
		Files []fileEntry
	}
	type bundleEntry struct {
		Name    string
		Kind    model.BundleKind
		Folders map[string]*folderEntry
	}
	type technologyEntry struct {
		Name    string
		Kind    model.TechnologyKind
		Bundles map[string]*bundleEntry
	}
	technologyMap := make(map[string]*technologyEntry)
	for _, p := range technologies {
		pe := &technologyEntry{Name: p.Name, Kind: p.Kind, Bundles: make(map[string]*bundleEntry)}
		for _, b := range p.Bundles {
			pe.Bundles[b.Name] = &bundleEntry{Name: b.Name, Kind: b.Kind, Folders: make(map[string]*folderEntry)}
		}
		technologyMap[p.Name] = pe
	}
	for _, f := range allFiles {
		bundle := model.GetBundleByPath(f.Path)
		if bundle == nil {
			continue
		}
		absPath := filepath.Join(workspace.GetRootDir(), f.Path)
		loc := workspace.CountLinesInFile(absPath)
		if loc == 0 {
			continue
		}
		bundleRoot := workspace.NormalizePath(bundle.Root)
		relPath := strings.TrimPrefix(workspace.NormalizePath(f.Path), bundleRoot+"/")
		folderName := filepath.Dir(relPath)
		if folderName == "." {
			folderName = ""
		}
		parts := strings.SplitN(bundle.Name, "/", 2)
		technologyName := parts[0]
		pe, ok := technologyMap[technologyName]
		if !ok {
			continue
		}
		be, ok := pe.Bundles[bundle.Name]
		if !ok {
			continue
		}
		fe, ok := be.Folders[folderName]
		if !ok {
			fe = &folderEntry{Name: folderName}
			be.Folders[folderName] = fe
		}
		fe.Files = append(fe.Files, fileEntry{Name: f.Name, Kind: f.Kind, LOC: loc})
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code\"\n")
	var technologyNames []string
	for name := range technologyMap {
		technologyNames = append(technologyNames, name)
	}
	sort.Strings(technologyNames)
	for _, pName := range technologyNames {
		pe := technologyMap[pName]
		technologyLOC := 0
		for _, be := range pe.Bundles {
			for _, fe := range be.Folders {
				for _, f := range fe.Files {
					technologyLOC += f.LOC
				}
			}
		}
		if technologyLOC == 0 {
			continue
		}
		emoji := mermaidTechnologyEmoji(pe.Kind)
		sb.WriteString(fmt.Sprintf("    \"%s%s\"\n", emoji, mermaidEscapeLabel(workspace.Flat(pe.Name))))
		var bundleNames []string
		for name := range pe.Bundles {
			bundleNames = append(bundleNames, name)
		}
		sort.Strings(bundleNames)
		for _, bName := range bundleNames {
			be := pe.Bundles[bName]
			bundleLOC := 0
			for _, fe := range be.Folders {
				for _, f := range fe.Files {
					bundleLOC += f.LOC
				}
			}
			if bundleLOC == 0 {
				continue
			}
			bParts := strings.SplitN(bName, "/", 2)
			bundleLabel := bParts[0]
			if len(bParts) > 1 {
				bundleLabel = bParts[1]
			}
			bEmoji := mermaidBundleEmoji(be.Kind)
			sb.WriteString(fmt.Sprintf("        \"%s%s\"\n", bEmoji, mermaidEscapeLabel(workspace.Flat(bundleLabel))))
			var folderNames []string
			for name := range be.Folders {
				folderNames = append(folderNames, name)
			}
			sort.Strings(folderNames)
			for _, fName := range folderNames {
				fe := be.Folders[fName]
				folderLOC := 0
				for _, f := range fe.Files {
					folderLOC += f.LOC
				}
				if folderLOC == 0 {
					continue
				}
				if fName == "" {
					for _, f := range fe.Files {
						fEmoji := mermaidFileEmoji(f.Kind)
						sb.WriteString(fmt.Sprintf("            \"%s%s\": %d\n", fEmoji, mermaidEscapeLabel(f.Name), f.LOC))
					}
				} else {
					sb.WriteString(fmt.Sprintf("            \"%s%s\"\n", model.EmojiFolderOrg, mermaidEscapeLabel(filepath.Base(fName))))
					sort.Slice(fe.Files, func(i, j int) bool { return fe.Files[i].Name < fe.Files[j].Name })
					for _, f := range fe.Files {
						fEmoji := mermaidFileEmoji(f.Kind)
						sb.WriteString(fmt.Sprintf("                \"%s%s\": %d\n", fEmoji, mermaidEscapeLabel(f.Name), f.LOC))
					}
				}
			}
		}
	}
	return sb.String()
}

func MermaidLocByContributors() string {
	bundles := codebase.GetTechnologies()
	files, _ := model.ScopeToFiles(workspace.Scope{Kind: workspace.ScopeRepo}, bundles)
	authorLines := make(map[string]int)
	for _, file := range files {
		absPath := filepath.Join(workspace.GetRootDir(), file)
		stdout, _, exitCode := workspace.ExecCommand("git", []string{"blame", "--line-porcelain", absPath}, workspace.GetRootDir())
		if exitCode != 0 {
			continue
		}
		for _, line := range strings.Split(stdout, "\n") {
			if strings.HasPrefix(line, "author ") {
				author := strings.TrimPrefix(line, "author ")
				authorLines[author]++
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code by Contributor\"\n")
	type authorEntry struct {
		Name string
		LOC  int
	}
	var authors []authorEntry
	for name, loc := range authorLines {
		authors = append(authors, authorEntry{Name: name, LOC: loc})
	}
	sort.Slice(authors, func(i, j int) bool { return authors[i].LOC > authors[j].LOC })
	for _, a := range authors {
		sb.WriteString(fmt.Sprintf("    \"%s\": %d\n", mermaidEscapeLabel(a.Name), a.LOC))
	}
	return sb.String()
}

func MermaidLocByLanguage() string {
	ctx := context.Background()
	fileCh := make(chan model.File)
	go func() { ticketspkg.StreamFiles(ctx, "", fileCh) }()
	langLines := make(map[string]int)
	for f := range fileCh {
		if f.Ignored || f.Generated {
			continue
		}
		lang := languages.GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		absPath := filepath.Join(workspace.GetRootDir(), f.Path)
		loc := workspace.CountLinesInFile(absPath)
		if loc == 0 {
			continue
		}
		langLines[lang.Name()] += loc
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code by Language\"\n")
	type langEntry struct {
		Name string
		LOC  int
	}
	var langs []langEntry
	for name, loc := range langLines {
		langs = append(langs, langEntry{Name: name, LOC: loc})
	}
	sort.Slice(langs, func(i, j int) bool { return langs[i].LOC > langs[j].LOC })
	for _, l := range langs {
		sb.WriteString(fmt.Sprintf("    \"%s\": %d\n", mermaidEscapeLabel(l.Name), l.LOC))
	}
	return sb.String()
}

// #endregion ⏲️Mermaid

// #endregion 🚚️Split

// #region 🔌️Ports

// 🌿️TextBranchConnector is the connector drawn in front of a non-last child in the text tree.
const TextBranchConnector = "├️─️─️ "

// 🌿️TextLastConnector is the connector drawn in front of the last child in the text tree.
const TextLastConnector = "└️─️─️ "

// 🌿️TextPipeIndent is the indent carried down a subtree that still has following siblings.
const TextPipeIndent = "│️   "

// 🌿️TextBlankIndent is the indent carried down the last subtree of a level.
const TextBlankIndent = "    "

// 🟢️PriorityIconLow is the icon of a low-priority statute.
const PriorityIconLow = "🟢️"

// 🟡️PriorityIconMedium is the icon of a medium-priority statute.
const PriorityIconMedium = "🟡️"

// 🔴️PriorityIconHigh is the icon of a high-priority statute.
const PriorityIconHigh = "🔴️"

// 🎫️NoGoalLabel is the label of the synthetic goal that collects every ticket without a goal.
const NoGoalLabel = "No Goal"

// 📌️TreeCacheSchemaVersion is the schema version of the tree cache envelope.
const TreeCacheSchemaVersion = 3

// 🪪️ArtifactIdentifier mints the artifact id of one entity from its tree-node data.
type ArtifactIdentifier interface {
	ArtifactID(entityKind string, data map[string]interface{}) string
}

// 🎨️EntityRenderer renders one entity as a single line.
type EntityRenderer interface {
	Human(entityKind string, data map[string]interface{}) string
	Markdown(entityKind string, data map[string]interface{}) string
	MarkdownLink(entityKind string, data map[string]interface{}) string
}

// 📜️StatuteCatalog resolves statute metadata, ids, uris and labels.
type StatuteCatalog interface {
	Info(statute model.Statute) (model.StatuteMeta, bool)
	StatuteID(statute model.Statute) string
	StatuteURI(statute model.Statute) string
	StatuteLabel(statute model.Statute) string
	EntityKind(statute model.Statute) string
	TerritoryID(territory model.Territory) string
}

// 🗂️TreeSource supplies the already-loaded records the monorepo tree projects.
type TreeSource interface {
	Technologies() []TechnologyRecord
	Folders() []FolderRecord
	Files() []FileRecord
	GoalRecords() []GoalRecord
	TicketRecords() []TicketRecord
	Drafts() []DraftRecord
	Policies() []PolicyRecord
	Contributors() []ContributorRecord
	Checkpoints() []CheckpointRecord
	Sessions() []SessionRecord
	Sections(filePath string) []SectionRecord
}

// 🪪️DefaultArtifactIdentifier mints artifact ids through the repository's own id builder.
type DefaultArtifactIdentifier struct{}

// 🆔️ArtifactID MUST return the artifact id the repository's id builder mints.
func (DefaultArtifactIdentifier) ArtifactID(entityKind string, data map[string]interface{}) string {
	return model.GetArtifactID(entityKind, data)
}

// 🎨️DefaultEntityRenderer renders entities through the repository's own renderers.
type DefaultEntityRenderer struct{}

// 🖥️Human MUST render the human, single-line form.
func (DefaultEntityRenderer) Human(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityHuman(entityKind, data, false)
}

// 📰️Markdown MUST render the markdown list-item form.
func (DefaultEntityRenderer) Markdown(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityMarkdown(entityKind, data)
}

// 🔗️MarkdownLink MUST render the markdown link form used inside a goal tree line.
func (DefaultEntityRenderer) MarkdownLink(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityMarkdownLink(entityKind, data)
}

// #endregion 🔌️Ports

// #region 💿️Records

// 🧪️TechnologyRecord is a technology with its bundles.
type TechnologyRecord struct {
	ID      string         `json:"id"`
	Name    string         `json:"name"`
	URI     string         `json:"uri"`
	Kind    string         `json:"kind"`
	Emoji   string         `json:"emoji"`
	Bundles []BundleRecord `json:"bundles"`
}

// 📦️BundleRecord is one bundle of a technology.
type BundleRecord struct {
	ID         string `json:"id"`
	Name       string `json:"name"`
	URI        string `json:"uri"`
	Kind       string `json:"kind"`
	Emoji      string `json:"emoji"`
	Root       string `json:"root"`
	SourceRoot string `json:"sourceRoot"`
}

// 📁️FolderRecord is one folder of the repository.
type FolderRecord struct {
	ID       string `json:"id"`
	Path     string `json:"path"`
	Name     string `json:"name"`
	URI      string `json:"uri"`
	Kind     string `json:"kind"`
	ParentID string `json:"parentId,omitempty"`
}

// 📄️FileRecord is one file of the repository.
type FileRecord struct {
	ID       string `json:"id"`
	Path     string `json:"path"`
	Name     string `json:"name"`
	URI      string `json:"uri"`
	Kind     string `json:"kind"`
	ParentID string `json:"parentId,omitempty"`
}

// 📑️SectionRecord is one section of a file, with its definitions and nested sections.
type SectionRecord struct {
	ID          string             `json:"id"`
	Path        string             `json:"path"`
	Name        string             `json:"name"`
	URI         string             `json:"uri"`
	StartLine   int                `json:"startLine"`
	EndLine     int                `json:"endLine"`
	Definitions []DefinitionRecord `json:"definitions"`
	Children    []SectionRecord    `json:"children"`
}

// 🏷️DefinitionRecord is one definition inside a section.
type DefinitionRecord struct {
	ID        string `json:"id"`
	Name      string `json:"name"`
	URI       string `json:"uri"`
	Kind      string `json:"kind"`
	FilePath  string `json:"filePath"`
	StartLine int    `json:"startLine"`
	EndLine   int    `json:"endLine"`
}

// 🎯️GoalRecord is one goal.
type GoalRecord struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	URI         string `json:"uri"`
	ArtifactID  string `json:"artifactId"`
	Status      string `json:"status"`
	DueDate     string `json:"dueDate"`
	CreatedAt   string `json:"createdAt"`
	Description string `json:"description"`
}

// 🎫️TicketRecord is one ticket.
type TicketRecord struct {
	ID          string `json:"id"`
	Slug        string `json:"slug"`
	Title       string `json:"title"`
	URI         string `json:"uri"`
	Description string `json:"description"`
	Summary     string `json:"summary"`
	Goal        string `json:"goal"`
	Parent      string `json:"parent"`
	Year        int    `json:"year"`
	Month       int    `json:"month"`
	Day         int    `json:"day"`
	Status      string `json:"status"`
	Started     string `json:"started"`
	Finished    string `json:"finished"`
}

// ✍️DraftRecord is one draft.
type DraftRecord struct {
	ID         string `json:"id"`
	URI        string `json:"uri"`
	ArtifactID string `json:"artifactId"`
}

// 🛡️PolicyRecord is one policy with its territories.
type PolicyRecord struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Groups      []model.Territory `json:"groups"`
}

// 🧑️ContributorRecord is one contributor.
type ContributorRecord struct {
	ID      string   `json:"id"`
	Alias   string   `json:"alias"`
	URI     string   `json:"uri"`
	Name    string   `json:"name"`
	Aliases []string `json:"aliases"`
	Github  string   `json:"github"`
	Githubs []string `json:"githubs"`
	Names   []string `json:"names"`
	Email   string   `json:"email"`
	Emails  []string `json:"emails"`
}

// 🔀️CheckpointRecord is one checkpoint.
type CheckpointRecord struct {
	ID       string `json:"id"`
	Sha      string `json:"sha"`
	Title    string `json:"title"`
	URI      string `json:"uri"`
	Year     int    `json:"year"`
	Month    int    `json:"month"`
	Day      int    `json:"day"`
	AuthorID string `json:"authorId"`
}

// ⚪️SessionRecord is one agent session.
type SessionRecord struct {
	ID         string `json:"id"`
	UUID       string `json:"uuid"`
	URI        string `json:"uri"`
	Kind       string `json:"kind"`
	KindEmoji  string `json:"kindEmoji"`
	Client     string `json:"client"`
	StartedAt  string `json:"startedAt"`
	Year       int    `json:"year"`
	Month      int    `json:"month"`
	Day        int    `json:"day"`
	Checkpoint string `json:"checkpoint"`
}

// 🗄️MemoryTreeSource is a TreeSource backed entirely by already-decoded records.
type MemoryTreeSource struct {
	TechnologyRecords  []TechnologyRecord         `json:"technologies"`
	FolderRecords      []FolderRecord             `json:"folders"`
	FileRecords        []FileRecord               `json:"files"`
	Goals              []GoalRecord               `json:"goals"`
	Tickets            []TicketRecord             `json:"tickets"`
	DraftRecords       []DraftRecord              `json:"drafts"`
	PolicyRecords      []PolicyRecord             `json:"policies"`
	ContributorRecords []ContributorRecord        `json:"contributors"`
	CheckpointRecords  []CheckpointRecord         `json:"checkpoints"`
	SessionRecords     []SessionRecord            `json:"sessions"`
	SectionRecords     map[string][]SectionRecord `json:"sections"`
}

// 📥️ParseTreeSource MUST return an error when the document is malformed.
func ParseTreeSource(document []byte) (*MemoryTreeSource, error) {
	source := &MemoryTreeSource{}
	if err := json.Unmarshal(document, source); err != nil {
		return nil, err
	}
	return source, nil
}

// 🧪️Technologies MUST return the declared technologies in load order.
func (s *MemoryTreeSource) Technologies() []TechnologyRecord { return s.TechnologyRecords }

// 📁️Folders MUST return every declared folder.
func (s *MemoryTreeSource) Folders() []FolderRecord { return s.FolderRecords }

// 📄️Files MUST return every declared file.
func (s *MemoryTreeSource) Files() []FileRecord { return s.FileRecords }

// 🎯️GoalRecords MUST return every declared goal.
func (s *MemoryTreeSource) GoalRecords() []GoalRecord { return s.Goals }

// 🎫️TicketRecords MUST return every declared ticket.
func (s *MemoryTreeSource) TicketRecords() []TicketRecord { return s.Tickets }

// ✍️Drafts MUST return every declared draft.
func (s *MemoryTreeSource) Drafts() []DraftRecord { return s.DraftRecords }

// 🛡️Policies MUST return every declared policy.
func (s *MemoryTreeSource) Policies() []PolicyRecord { return s.PolicyRecords }

// 🧑️Contributors MUST return every declared contributor.
func (s *MemoryTreeSource) Contributors() []ContributorRecord { return s.ContributorRecords }

// 🔀️Checkpoints MUST return every declared checkpoint.
func (s *MemoryTreeSource) Checkpoints() []CheckpointRecord { return s.CheckpointRecords }

// ⚪️Sessions MUST return every declared session.
func (s *MemoryTreeSource) Sessions() []SessionRecord { return s.SessionRecords }

// 📑️Sections MUST return the declared sections of one file.
func (s *MemoryTreeSource) Sections(filePath string) []SectionRecord {
	return s.SectionRecords[filePath]
}

// 🗄️MemoryStatuteCatalog is a StatuteCatalog backed by declared entries.
type MemoryStatuteCatalog struct {
	Statutes    []StatuteCatalogEntry `json:"statutes"`
	Territories map[string]string     `json:"territories"`
}

// 📜️StatuteCatalogEntry is one declared statute of a MemoryStatuteCatalog.
type StatuteCatalogEntry struct {
	ID         string            `json:"id"`
	ArtifactID string            `json:"artifactId"`
	URI        string            `json:"uri"`
	Label      string            `json:"label"`
	EntityKind string            `json:"entityKind"`
	Meta       StatuteMetaRecord `json:"meta"`
}

// 🔖️StatuteMetaRecord is the declared metadata of one catalog entry.
type StatuteMetaRecord struct {
	PolicyID    string `json:"policyId"`
	Priority    string `json:"priority"`
	Reason      string `json:"reason"`
	Solution    string `json:"solution"`
	Autofixable bool   `json:"autofixable"`
}

// 📥️ParseStatuteCatalog MUST return an error when the document is malformed.
func ParseStatuteCatalog(document []byte) (*MemoryStatuteCatalog, error) {
	catalog := &MemoryStatuteCatalog{}
	if err := json.Unmarshal(document, catalog); err != nil {
		return nil, err
	}
	return catalog, nil
}

// 🔎️entry returns the declared entry of a statute.
func (c *MemoryStatuteCatalog) entry(statute model.Statute) (StatuteCatalogEntry, bool) {
	for _, candidate := range c.Statutes {
		if candidate.ID == string(statute) {
			return candidate, true
		}
	}
	return StatuteCatalogEntry{}, false
}

// 🔖️Info MUST return the declared metadata of a statute.
func (c *MemoryStatuteCatalog) Info(statute model.Statute) (model.StatuteMeta, bool) {
	found, ok := c.entry(statute)
	if !ok {
		return model.StatuteMeta{}, false
	}
	return model.StatuteMeta{
		Kind:        statute,
		PolicyID:    found.Meta.PolicyID,
		Priority:    ParseBreachPriority(found.Meta.Priority),
		Reason:      found.Meta.Reason,
		Solution:    found.Meta.Solution,
		Autofixable: found.Meta.Autofixable,
	}, true
}

// 🆔️StatuteID MUST return the artifact id of a statute.
func (c *MemoryStatuteCatalog) StatuteID(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.ArtifactID
}

// 🔗️StatuteURI MUST return the uri of a statute.
func (c *MemoryStatuteCatalog) StatuteURI(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.URI
}

// 🏷️StatuteLabel MUST return the display label of a statute path.
func (c *MemoryStatuteCatalog) StatuteLabel(statute model.Statute) string {
	found, ok := c.entry(statute)
	if !ok {
		return string(statute)
	}
	return found.Label
}

// 🧱️EntityKind MUST return the entity kind a statute applies to.
func (c *MemoryStatuteCatalog) EntityKind(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.EntityKind
}

// 🆔️TerritoryID MUST return the artifact id of a territory.
func (c *MemoryStatuteCatalog) TerritoryID(territory model.Territory) string {
	if id, ok := c.Territories[territory.Name]; ok {
		return id
	}
	return territory.Name
}

// 🌿️TreeNodeSpec is a compact tree-node literal where every field defaults.
type TreeNodeSpec struct {
	Kind        string                 `json:"kind"`
	ID          string                 `json:"id"`
	Label       string                 `json:"label"`
	URI         string                 `json:"uri"`
	SubKind     string                 `json:"subKind"`
	Description string                 `json:"description"`
	Summary     string                 `json:"summary"`
	Year        int                    `json:"year"`
	Month       int                    `json:"month"`
	Day         int                    `json:"day"`
	Status      string                 `json:"status"`
	Contributor string                 `json:"contributor"`
	Data        map[string]interface{} `json:"data"`
	Children    []TreeNodeSpec         `json:"children"`
}

// 📥️ParseTreeNodeSpec MUST return an error when the document is malformed.
func ParseTreeNodeSpec(document []byte) (*TreeNodeSpec, error) {
	spec := &TreeNodeSpec{}
	if err := json.Unmarshal(document, spec); err != nil {
		return nil, err
	}
	return spec, nil
}

// 🌿️ToTreeNode MUST expand the literal into a real tree node.
func (s *TreeNodeSpec) ToTreeNode() *TreeNode {
	node := NewTreeNode(ParseTreeNodeKind(s.Kind), s.ID, s.Label, s.URI)
	node.SubKind = s.SubKind
	node.Description = s.Description
	node.Summary = s.Summary
	node.Year = s.Year
	node.Month = s.Month
	node.Day = s.Day
	node.Status = s.Status
	node.Contributor = s.Contributor
	node.Data = s.Data
	for index := range s.Children {
		node.Children = append(node.Children, s.Children[index].ToTreeNode())
	}
	return node
}

// 🧹️TreeFilterSpec is a compact tree-filter literal where every field defaults.
type TreeFilterSpec struct {
	Query               string              `json:"query"`
	OnlyKinds           []string            `json:"onlyKinds"`
	ExcludeKinds        []string            `json:"excludeKinds"`
	OnlySubKinds        map[string][]string `json:"onlySubKinds"`
	ExcludeSubKinds     map[string][]string `json:"excludeSubKinds"`
	OnlyYears           []int               `json:"onlyYears"`
	ExcludeYears        []int               `json:"excludeYears"`
	OnlyMonths          []int               `json:"onlyMonths"`
	ExcludeMonths       []int               `json:"excludeMonths"`
	OnlyDays            []int               `json:"onlyDays"`
	ExcludeDays         []int               `json:"excludeDays"`
	OnlyStatus          string              `json:"onlyStatus"`
	OnlyContributors    []string            `json:"onlyContributors"`
	ExcludeContributors []string            `json:"excludeContributors"`
	OnlyPolicies        []string            `json:"onlyPolicies"`
	ExcludePolicies     []string            `json:"excludePolicies"`
}

// 📥️ParseTreeFilterSpec MUST return an error when the document is malformed.
func ParseTreeFilterSpec(document []byte) (*TreeFilterSpec, error) {
	spec := &TreeFilterSpec{}
	if err := json.Unmarshal(document, spec); err != nil {
		return nil, err
	}
	return spec, nil
}

// 🧹️ToFilter MUST expand the literal into a real tree filter.
func (s *TreeFilterSpec) ToFilter() *TreeFilter {
	kindSet := func(kinds []string) map[TreeNodeKind]bool {
		out := map[TreeNodeKind]bool{}
		for _, kind := range kinds {
			out[ParseTreeNodeKind(kind)] = true
		}
		return out
	}
	subKindMap := func(entries map[string][]string) map[TreeNodeKind][]string {
		out := map[TreeNodeKind][]string{}
		for kind, values := range entries {
			out[ParseTreeNodeKind(kind)] = values
		}
		return out
	}
	return &TreeFilter{
		Query:               s.Query,
		OnlyKinds:           kindSet(s.OnlyKinds),
		ExcludeKinds:        kindSet(s.ExcludeKinds),
		OnlySubKinds:        subKindMap(s.OnlySubKinds),
		ExcludeSubKinds:     subKindMap(s.ExcludeSubKinds),
		OnlyYears:           s.OnlyYears,
		ExcludeYears:        s.ExcludeYears,
		OnlyMonths:          s.OnlyMonths,
		ExcludeMonths:       s.ExcludeMonths,
		OnlyDays:            s.OnlyDays,
		ExcludeDays:         s.ExcludeDays,
		OnlyStatus:          s.OnlyStatus,
		OnlyContributors:    s.OnlyContributors,
		ExcludeContributors: s.ExcludeContributors,
		OnlyPolicies:        s.OnlyPolicies,
		ExcludePolicies:     s.ExcludePolicies,
	}
}

// 📥️DecodeTerritories MUST return an error when the document is malformed.
func DecodeTerritories(document []byte) ([]model.Territory, error) {
	var territories []model.Territory
	if err := json.Unmarshal(document, &territories); err != nil {
		return nil, err
	}
	return territories, nil
}

// 📥️DecodeStatutes MUST return an error when the document is malformed.
func DecodeStatutes(document []byte) ([]model.Statute, error) {
	var statutes []model.Statute
	if err := json.Unmarshal(document, &statutes); err != nil {
		return nil, err
	}
	return statutes, nil
}

// 🔤️ParseTreeNodeKind MUST default to the structural category kind.
func ParseTreeNodeKind(value string) TreeNodeKind {
	switch value {
	case "technology", "bundle", "folder", "file", "section", "definition", "goal", "ticket", "draft", "todo", "policy", "breach", "contributor", "checkpoint", "session", "statute":
		return TreeNodeKind(value)
	}
	return TreeNodeCategory
}

// 🔤️ParseBreachPriority MUST default to the low priority the icon table also defaults to.
func ParseBreachPriority(value string) model.BreachPriority {
	switch value {
	case "high":
		return model.BreachPriorityHigh
	case "medium":
		return model.BreachPriorityMedium
	}
	return model.BreachPriorityLow
}

// #endregion 💿️Records

// #region 🌿️NodeHelpers

// 🌿️NewTreeNode MUST return an empty node of one kind.
func NewTreeNode(kind TreeNodeKind, id string, label string, uri string) *TreeNode {
	return &TreeNode{Kind: kind, ID: id, Label: label, URI: uri}
}

// ➕️PushChild MUST append one child to a node.
func PushChild(node *TreeNode, child *TreeNode) {
	node.Children = append(node.Children, child)
}

// 🗃️dataOf builds a data map from ordered pairs.
func dataOf(pairs ...interface{}) map[string]interface{} {
	out := make(map[string]interface{}, len(pairs)/2)
	for index := 0; index+1 < len(pairs); index += 2 {
		key, _ := pairs[index].(string)
		out[key] = pairs[index+1]
	}
	return out
}

// 🔢️number wraps a count the way the record projections carry it.
func number(value int) interface{} { return float64(value) }

// 📃️list wraps a string list as a JSON-compatible value.
func list(values []string) interface{} {
	out := make([]interface{}, 0, len(values))
	for _, value := range values {
		out = append(out, value)
	}
	return out
}

// 📁️ParentPath MUST return the parent of a slash-separated path, or `.` when there is none.
func ParentPath(path string) string {
	index := strings.LastIndex(path, "/")
	switch {
	case index == 0:
		return "/"
	case index > 0:
		return path[:index]
	}
	return "."
}

// 📄️BaseName MUST return the last segment of a slash-separated path.
func BaseName(path string) string {
	if index := strings.LastIndex(path, "/"); index >= 0 {
		return path[index+1:]
	}
	return path
}

// 🪜️nestOrderedNodes nests nodes under the node carrying their parent id, preserving input order.
func nestOrderedNodes(ids []string, parents []string, values []*TreeNode) []*TreeNode {
	positions := map[string]int{}
	for index, id := range ids {
		if _, seen := positions[id]; !seen {
			positions[id] = index
		}
	}
	roots := []int{}
	children := map[int][]int{}
	for index := range ids {
		parentIndex, ok := positions[parents[index]]
		if ok && parents[index] != ids[index] && parentIndex != index {
			children[parentIndex] = append(children[parentIndex], index)
			continue
		}
		roots = append(roots, index)
	}
	var assemble func(index int) *TreeNode
	assemble = func(index int) *TreeNode {
		node := values[index]
		if node == nil {
			return nil
		}
		values[index] = nil
		for _, child := range children[index] {
			if assembled := assemble(child); assembled != nil {
				PushChild(node, assembled)
			}
		}
		return node
	}
	out := []*TreeNode{}
	for _, index := range roots {
		if assembled := assemble(index); assembled != nil {
			out = append(out, assembled)
		}
	}
	return out
}

// 🎨️PriorityIcon MUST return the icon of a priority.
func PriorityIcon(priority model.BreachPriority) string {
	switch priority {
	case model.BreachPriorityHigh:
		return PriorityIconHigh
	case model.BreachPriorityMedium:
		return PriorityIconMedium
	}
	return PriorityIconLow
}

// #endregion 🌿️NodeHelpers

// #region 🩻️PortedMonorepoTree

// 🏢️BuildMonorepoTree MUST assemble the monorepo tree from the records a TreeSource supplies.
func BuildMonorepoTree(source TreeSource, options TreeBuildOptions) *TreeNode {
	root := NewTreeNode(TreeNodeCategory, "", ".", "")
	root.Children = []*TreeNode{}
	PushChild(root, buildCodebaseNode(source, options))
	PushChild(root, buildGoalsNode(source))
	PushChild(root, buildDraftsNode(source))
	PushChild(root, buildPoliciesNode(source))
	PushChild(root, buildContributorsNode(source))
	PushChild(root, buildCheckpointsNode(source))
	PushChild(root, buildSessionsNode(source))
	return root
}

// 🏢️BuildMonorepoTreeWithIDs MUST stamp every node with its parent artifact id.
func BuildMonorepoTreeWithIDs(source TreeSource, options TreeBuildOptions, identifier ArtifactIdentifier) *TreeNode {
	root := BuildMonorepoTree(source, options)
	PropagateParentIDs(root, "", identifier)
	return root
}

// 📄️buildFileNode projects one file record, with its sections when they are requested.
func buildFileNode(source TreeSource, file FileRecord, options TreeBuildOptions) *TreeNode {
	node := NewTreeNode(TreeNodeFile, file.ID, file.Name, file.URI)
	node.SubKind = file.Kind
	node.Data = dataOf("path", file.Path, "name", file.Name, "kind", file.Kind, "parentId", file.ParentID)
	if options.IncludeSections {
		for _, section := range source.Sections(file.Path) {
			PushChild(node, BuildSectionTreeNodeRecord(section))
		}
	}
	return node
}

// 📁️buildFolderNode projects one folder record.
func buildFolderNode(folder FolderRecord) *TreeNode {
	node := NewTreeNode(TreeNodeFolder, folder.ID, folder.Name, folder.URI)
	node.SubKind = folder.Kind
	node.Data = dataOf("path", folder.Path, "name", folder.Name, "kind", folder.Kind, "parentId", folder.ParentID)
	return node
}

// 📑️BuildSectionTreeNodeRecord MUST project one section record with definitions and children.
func BuildSectionTreeNodeRecord(section SectionRecord) *TreeNode {
	node := NewTreeNode(TreeNodeSection, section.ID, section.Name, section.URI)
	node.Data = dataOf("path", section.Path, "name", section.Name, "startLine", number(section.StartLine), "endLine", number(section.EndLine))
	for _, definition := range section.Definitions {
		definitionNode := NewTreeNode(TreeNodeDefinition, definition.ID, definition.Name, definition.URI)
		definitionNode.SubKind = definition.Kind
		definitionNode.Data = dataOf("path", definition.FilePath, "name", definition.Name, "kind", definition.Kind, "startLine", number(definition.StartLine), "endLine", number(definition.EndLine))
		PushChild(node, definitionNode)
	}
	for _, child := range section.Children {
		PushChild(node, BuildSectionTreeNodeRecord(child))
	}
	return node
}

// 📁️BuildFolderRoots MUST nest every folder under its parent folder.
func BuildFolderRoots(folders []FolderRecord) []*TreeNode {
	ids := make([]string, 0, len(folders))
	parents := make([]string, 0, len(folders))
	values := make([]*TreeNode, 0, len(folders))
	for _, folder := range folders {
		ids = append(ids, folder.Path)
		parents = append(parents, ParentPath(folder.Path))
		values = append(values, buildFolderNode(folder))
	}
	return nestOrderedNodes(ids, parents, values)
}

// 📄️AttachFilesToFolders MUST attach every file node under the folder node of its directory.
func AttachFilesToFolders(root *TreeNode, files []FileRecord, fileNodes map[string]*TreeNode) {
	for _, file := range files {
		node, ok := fileNodes[file.Path]
		if !ok {
			continue
		}
		if !attachIntoFolder(root, ParentPath(file.Path), node) {
			PushChild(root, cloneTreeNode(node))
		}
	}
}

// 🔎️attachIntoFolder attaches a node under the folder node carrying a path.
func attachIntoFolder(node *TreeNode, folderPath string, fileNode *TreeNode) bool {
	if node.Kind == TreeNodeFolder && node.Data != nil {
		if path, ok := node.Data["path"].(string); ok && path != "" && path == folderPath {
			PushChild(node, cloneTreeNode(fileNode))
			return true
		}
	}
	for _, child := range node.Children {
		if attachIntoFolder(child, folderPath, fileNode) {
			return true
		}
	}
	return false
}

// 🗂️buildCodebaseNode projects the folder/file hierarchy plus the technology/bundle view.
func buildCodebaseNode(source TreeSource, options TreeBuildOptions) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "codebase", identity.EmojiText(identity.Collection("codebase"))+"Codebase", "repo://codebase")
	folders := source.Folders()
	files := source.Files()
	fileNodes := map[string]*TreeNode{}
	for _, file := range files {
		fileNodes[file.Path] = buildFileNode(source, file, options)
	}
	folderNodes := map[string]*TreeNode{}
	for _, folder := range folders {
		folderNodes[folder.Path] = buildFolderNode(folder)
	}
	for _, folderRoot := range BuildFolderRoots(folders) {
		PushChild(node, folderRoot)
	}
	AttachFilesToFolders(node, files, fileNodes)
	technologies := append([]TechnologyRecord{}, source.Technologies()...)
	sort.SliceStable(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })
	for _, technology := range technologies {
		technologyNode := NewTreeNode(TreeNodeTechnology, technology.ID, technology.Name, technology.URI)
		technologyNode.SubKind = technology.Kind
		technologyNode.Data = dataOf("name", technology.Name, "kind", technology.Kind, "emoji", technology.Emoji)
		bundles := append([]BundleRecord{}, technology.Bundles...)
		sort.SliceStable(bundles, func(i, j int) bool { return bundles[i].Name < bundles[j].Name })
		for _, bundle := range bundles {
			PushChild(technologyNode, buildBundleNode(bundle, folderNodes, fileNodes))
		}
		PushChild(node, technologyNode)
	}
	SortTreeChildren(node)
	return node
}

// 📦️buildBundleNode projects one bundle: every folder and file below its source root.
func buildBundleNode(bundle BundleRecord, folderNodes map[string]*TreeNode, fileNodes map[string]*TreeNode) *TreeNode {
	node := NewTreeNode(TreeNodeBundle, bundle.ID, bundle.Name, bundle.URI)
	node.SubKind = bundle.Kind
	node.Data = dataOf("name", bundle.Name, "root", bundle.Root, "kind", bundle.Kind, "emoji", bundle.Emoji)
	bundleRoot := bundle.SourceRoot
	if bundleRoot == "" {
		bundleRoot = bundle.Root
	}
	owned := map[string]*TreeNode{}
	ownedPaths := []string{}
	for path, folderNode := range folderNodes {
		if isUnder(path, bundleRoot) {
			owned[path] = cloneTreeNode(folderNode)
			ownedPaths = append(ownedPaths, path)
		}
	}
	sort.Strings(ownedPaths)
	filePaths := make([]string, 0, len(fileNodes))
	for path := range fileNodes {
		filePaths = append(filePaths, path)
	}
	sort.Strings(filePaths)
	for _, path := range filePaths {
		if !isUnder(path, bundleRoot) {
			continue
		}
		if folderNode, ok := owned[ParentPath(path)]; ok {
			PushChild(folderNode, cloneTreeNode(fileNodes[path]))
			continue
		}
		PushChild(node, cloneTreeNode(fileNodes[path]))
	}
	ids := make([]string, 0, len(ownedPaths))
	parents := make([]string, 0, len(ownedPaths))
	values := make([]*TreeNode, 0, len(ownedPaths))
	for _, path := range ownedPaths {
		ids = append(ids, path)
		parents = append(parents, ParentPath(path))
		values = append(values, owned[path])
	}
	for _, folderRoot := range nestOrderedNodes(ids, parents, values) {
		PushChild(node, folderRoot)
	}
	SortTreeChildren(node)
	return node
}

// 📏️isUnder reports whether a path is the given root or lies below it.
func isUnder(path string, root string) bool {
	return path == root || strings.HasPrefix(path, root+"/")
}

// 🎯️buildGoalsNode projects the goals category, nesting subgoals and attaching tickets.
func buildGoalsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "goals", "🎯️Goals", "repo://goals")
	goals := append([]GoalRecord{}, source.GoalRecords()...)
	sort.SliceStable(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	goalNodes := map[string]*TreeNode{}
	for _, goal := range goals {
		artifactID := goal.ArtifactID
		if artifactID == "" {
			artifactID = goal.ID
		}
		goalNode := NewTreeNode(TreeNodeGoal, artifactID, goal.Title, goal.URI)
		goalNode.SubKind = goal.Status
		goalNode.Description = goal.Description
		goalNode.Status = goal.Status
		goalNode.Data = dataOf("id", goal.ID, "title", goal.Title, "status", goal.Status, "dueDate", goal.DueDate, "createdAt", "", "description", goal.Description)
		goalNodes[goal.ID] = goalNode
	}
	goalTickets := map[string][]*TreeNode{}
	looseTickets := []*TreeNode{}
	for _, ticket := range source.TicketRecords() {
		ticketNode := buildTicketTreeNode(ticket)
		if _, ok := goalNodes[ticket.Goal]; ok && ticket.Goal != "" {
			goalTickets[ticket.Goal] = append(goalTickets[ticket.Goal], ticketNode)
			continue
		}
		looseTickets = append(looseTickets, ticketNode)
	}
	roots := []string{}
	subgoals := map[string][]string{}
	for _, goal := range goals {
		parent := GoalParentID(goal.ID)
		if _, ok := goalNodes[parent]; parent != "" && parent != goal.ID && ok {
			subgoals[parent] = append(subgoals[parent], goal.ID)
			continue
		}
		roots = append(roots, goal.ID)
	}
	var assemble func(id string) *TreeNode
	assemble = func(id string) *TreeNode {
		current, ok := goalNodes[id]
		if !ok {
			return nil
		}
		delete(goalNodes, id)
		for _, childID := range subgoals[id] {
			if child := assemble(childID); child != nil {
				PushChild(current, child)
			}
		}
		for _, ticket := range goalTickets[id] {
			PushChild(current, ticket)
		}
		delete(goalTickets, id)
		return current
	}
	children := []*TreeNode{}
	for _, id := range roots {
		if assembled := assemble(id); assembled != nil {
			children = append(children, assembled)
		}
	}
	children = append(children, looseTickets...)
	node.Children = children
	return node
}

// 🎫️buildTicketTreeNode projects one ticket record as a monorepo tree node.
func buildTicketTreeNode(ticket TicketRecord) *TreeNode {
	node := NewTreeNode(TreeNodeTicket, ticket.ID, ticket.Title, ticket.URI)
	node.Description = ticket.Description
	node.Year = ticket.Year
	node.Month = ticket.Month
	node.Day = ticket.Day
	node.Status = ticket.Status
	node.Data = dataOf(
		"year", number(ticket.Year), "month", number(ticket.Month), "day", number(ticket.Day),
		"slug", ticket.Slug, "title", ticket.Title, "status", ticket.Status,
		"started", ticket.Started, "finished", ticket.Finished,
		"prompt", ticket.Description, "summary", ticket.Summary, "goalId", ticket.Goal)
	return node
}

// 🧬️GoalParentID MUST return the parent goal id of a slash-separated goal id.
func GoalParentID(id string) string {
	if index := strings.LastIndex(id, "/"); index >= 0 {
		return id[:index]
	}
	return ""
}

// ✍️buildDraftsNode projects the drafts category.
func buildDraftsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "drafts", "✍️Drafts", "repo://drafts")
	for _, draft := range source.Drafts() {
		artifactID := draft.ArtifactID
		if artifactID == "" {
			artifactID = draft.ID
		}
		draftNode := NewTreeNode(TreeNodeDraft, artifactID, draft.ID, draft.URI)
		draftNode.Data = dataOf("id", draft.ID, "slug", draft.ID)
		PushChild(node, draftNode)
	}
	return node
}

// 🛡️buildPoliciesNode projects the policies category.
func buildPoliciesNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "policies", "🛡️Policies", "repo://policies")
	policies := append([]PolicyRecord{}, source.Policies()...)
	sort.SliceStable(policies, func(i, j int) bool { return policies[i].ID < policies[j].ID })
	policyEmoji := identity.EmojiText(identity.Entity("policy"))
	for _, policy := range policies {
		flat := identity.Flat(strings.TrimLeft(policy.ID, "/"))
		policyNode := NewTreeNode(TreeNodePolicy, policyEmoji+"/"+policy.ID, policy.Name, "repo://policy/"+policyEmoji+flat)
		policyNode.Description = policy.Description
		policyNode.Data = dataOf("id", policy.ID, "name", policy.Name, "description", policy.Description)
		PushChild(node, policyNode)
	}
	return node
}

// 🧑️buildContributorsNode projects the contributors category.
func buildContributorsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "contributors", "🧑️‍💻️Contributors", "repo://contributors")
	contributors := append([]ContributorRecord{}, source.Contributors()...)
	sort.SliceStable(contributors, func(i, j int) bool { return contributors[i].Alias < contributors[j].Alias })
	for _, contributor := range contributors {
		label := contributor.Alias
		if contributor.Name != "" {
			label = contributor.Name + " (" + contributor.Alias + ")"
		}
		contributorNode := NewTreeNode(TreeNodeContributor, contributor.ID, label, contributor.URI)
		contributorNode.Contributor = contributor.Alias
		contributorNode.Data = dataOf(
			"alias", contributor.Alias, "aliases", list(contributor.Aliases),
			"github", contributor.Github, "githubs", list(contributor.Githubs),
			"name", contributor.Name, "names", list(contributor.Names),
			"email", contributor.Email, "emails", list(contributor.Emails))
		PushChild(node, contributorNode)
	}
	return node
}

// 🔀️buildCheckpointsNode projects the checkpoints category.
func buildCheckpointsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "checkpoints", "🔀️Checkpoints", "repo://checkpoints")
	for _, checkpoint := range source.Checkpoints() {
		short := []rune(checkpoint.Sha)
		if len(short) > 8 {
			short = short[:8]
		}
		checkpointNode := NewTreeNode(TreeNodeCheckpoint, checkpoint.ID, string(short)+" "+checkpoint.Title, checkpoint.URI)
		checkpointNode.Year = checkpoint.Year
		checkpointNode.Month = checkpoint.Month
		checkpointNode.Day = checkpoint.Day
		checkpointNode.Data = dataOf("sha", checkpoint.Sha, "message", checkpoint.Title, "authorId", checkpoint.AuthorID)
		PushChild(node, checkpointNode)
	}
	return node
}

// ⚪️buildSessionsNode projects the sessions category, newest first.
func buildSessionsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "sessions", "⚪️Sessions", "repo://sessions")
	sessions := append([]SessionRecord{}, source.Sessions()...)
	sort.SliceStable(sessions, func(i, j int) bool { return sessions[i].StartedAt > sessions[j].StartedAt })
	for _, session := range sessions {
		label := session.KindEmoji + " " + session.UUID
		if session.Client != "" {
			label += " (" + session.Client + ")"
		}
		sessionNode := NewTreeNode(TreeNodeSession, session.ID, label, session.URI)
		sessionNode.SubKind = session.Kind
		sessionNode.Year = session.Year
		sessionNode.Month = session.Month
		sessionNode.Day = session.Day
		sessionNode.Status = session.Kind
		sessionNode.Data = dataOf(
			"uuid", session.UUID, "kind", session.Kind, "client", session.Client,
			"startedAt", session.StartedAt, "year", number(session.Year), "month", number(session.Month),
			"day", number(session.Day), "checkpoint", session.Checkpoint)
		PushChild(node, sessionNode)
	}
	return node
}

// #endregion 🩻️PortedMonorepoTree

// #region 🎯️PortedGoalTree

// 🎯️GoalNode is one goal of the goal tree, with its subgoals and tickets.
type GoalNode struct {
	ID          string       `json:"id"`
	Title       string       `json:"title"`
	Status      string       `json:"status"`
	DueDate     string       `json:"dueDate"`
	CreatedAt   string       `json:"createdAt"`
	Description string       `json:"description"`
	Children    []GoalNode   `json:"children"`
	Tickets     []TicketNode `json:"tickets"`
}

// 🎫️TicketNode is one ticket of the goal tree, with its child tickets.
type TicketNode struct {
	ID          string       `json:"id"`
	Slug        string       `json:"slug"`
	Status      string       `json:"status"`
	Title       string       `json:"title"`
	URI         string       `json:"uri"`
	GoalID      string       `json:"goalId"`
	ParentID    string       `json:"parentId"`
	Children    []TicketNode `json:"children"`
	Created     string       `json:"created"`
	Finished    string       `json:"finished"`
	Description string       `json:"description"`
	Summary     string       `json:"summary"`
}

// 🎯️BuildGoalTree MUST nest subgoals by id path and tickets by parent.
func BuildGoalTree(goals []GoalRecord, tickets []TicketRecord) []GoalNode {
	nodes := map[string]*GoalNode{}
	for _, goal := range goals {
		nodes[goal.ID] = &GoalNode{ID: goal.ID, Title: goal.Title, Status: goal.Status, DueDate: goal.DueDate, CreatedAt: goal.CreatedAt, Description: goal.Description}
	}
	orphans := []TicketNode{}
	for _, ticket := range tickets {
		node := TicketNode{
			ID: ticket.ID, Slug: ticket.Slug, Status: ticket.Status, Title: ticket.Title,
			URI: TicketURI(ticket), GoalID: ticket.Goal, ParentID: ticket.Parent,
			Created: ticket.Started, Finished: ticket.Finished, Description: ticket.Description, Summary: ticket.Summary,
		}
		if goal, ok := nodes[ticket.Goal]; ok && ticket.Goal != "" {
			goal.Tickets = append(goal.Tickets, node)
			continue
		}
		orphans = append(orphans, node)
	}
	for _, node := range nodes {
		node.Tickets = nestTickets(node.Tickets)
	}
	ids := []string{}
	parents := []string{}
	values := []*GoalNode{}
	for _, goal := range goals {
		node, ok := nodes[goal.ID]
		if !ok {
			continue
		}
		delete(nodes, goal.ID)
		ids = append(ids, goal.ID)
		parents = append(parents, GoalParentID(goal.ID))
		values = append(values, node)
	}
	result := nestOrderedGoals(ids, parents, values)
	SortGoalNodes(result)
	if len(orphans) > 0 {
		result = append(result, GoalNode{Title: NoGoalLabel, Tickets: nestTickets(orphans)})
	}
	return result
}

// 🔗️TicketURI MUST return the declared uri, or build the repository's own.
func TicketURI(ticket TicketRecord) string {
	if ticket.URI != "" {
		return ticket.URI
	}
	return "repo://ticket/" + identity.EmojiText(identity.Entity("ticket")) + identity.Flat(ticket.Slug)
}

// 🪜️nestTickets nests tickets under their parent ticket.
func nestTickets(tickets []TicketNode) []TicketNode {
	if len(tickets) == 0 {
		return nil
	}
	positions := map[string]int{}
	for index, ticket := range tickets {
		if _, seen := positions[ticket.ID]; !seen {
			positions[ticket.ID] = index
		}
	}
	roots := []int{}
	children := map[int][]int{}
	taken := make([]bool, len(tickets))
	for index, ticket := range tickets {
		parentIndex, ok := positions[ticket.ParentID]
		if ok && ticket.ParentID != ticket.ID && parentIndex != index {
			children[parentIndex] = append(children[parentIndex], index)
			continue
		}
		roots = append(roots, index)
	}
	var assemble func(index int) (TicketNode, bool)
	assemble = func(index int) (TicketNode, bool) {
		if taken[index] {
			return TicketNode{}, false
		}
		taken[index] = true
		node := tickets[index]
		node.Children = nil
		for _, child := range children[index] {
			if assembled, ok := assemble(child); ok {
				node.Children = append(node.Children, assembled)
			}
		}
		return node, true
	}
	out := []TicketNode{}
	for _, index := range roots {
		if assembled, ok := assemble(index); ok {
			out = append(out, assembled)
		}
	}
	if len(out) == 0 {
		return nil
	}
	return out
}

// 🪜️nestOrderedGoals nests goals under the goal carrying their parent id.
func nestOrderedGoals(ids []string, parents []string, values []*GoalNode) []GoalNode {
	positions := map[string]int{}
	for index, id := range ids {
		if _, seen := positions[id]; !seen {
			positions[id] = index
		}
	}
	roots := []int{}
	children := map[int][]int{}
	for index := range ids {
		parentIndex, ok := positions[parents[index]]
		if ok && parents[index] != ids[index] && parentIndex != index {
			children[parentIndex] = append(children[parentIndex], index)
			continue
		}
		roots = append(roots, index)
	}
	var assemble func(index int) (GoalNode, bool)
	assemble = func(index int) (GoalNode, bool) {
		if values[index] == nil {
			return GoalNode{}, false
		}
		node := *values[index]
		values[index] = nil
		for _, child := range children[index] {
			if assembled, ok := assemble(child); ok {
				node.Children = append(node.Children, assembled)
			}
		}
		return node, true
	}
	out := []GoalNode{}
	for _, index := range roots {
		if assembled, ok := assemble(index); ok {
			out = append(out, assembled)
		}
	}
	return out
}

// 📶️SortGoalNodes MUST sort goals by due date, blank dates last, ties broken by id.
func SortGoalNodes(goals []GoalNode) {
	sort.SliceStable(goals, func(i, j int) bool {
		left, right := goals[i], goals[j]
		if left.DueDate != right.DueDate {
			if left.DueDate == "" {
				return false
			}
			if right.DueDate == "" {
				return true
			}
			return left.DueDate < right.DueDate
		}
		return left.ID < right.ID
	})
	for index := range goals {
		SortGoalNodes(goals[index].Children)
	}
}

// 🧮️CountOpenSubgoals MUST count the open subgoals below a goal.
func CountOpenSubgoals(goal *GoalNode) int {
	count := 0
	for index := range goal.Children {
		child := &goal.Children[index]
		if child.Status == "open" {
			count++
		}
		count += CountOpenSubgoals(child)
	}
	return count
}

// 🧮️CountOpenTickets MUST count the open tickets below a goal, including every subgoal's.
func CountOpenTickets(goal *GoalNode) int {
	var walk func(tickets []TicketNode) int
	walk = func(tickets []TicketNode) int {
		count := 0
		for index := range tickets {
			if tickets[index].Status == "open" {
				count++
			}
			count += walk(tickets[index].Children)
		}
		return count
	}
	count := walk(goal.Tickets)
	for index := range goal.Children {
		count += CountOpenTickets(&goal.Children[index])
	}
	return count
}

// 💿️GoalNodeData MUST return the data map a goal line renders from.
func GoalNodeData(goal *GoalNode) map[string]interface{} {
	return dataOf("id", goal.ID, "title", goal.Title, "status", goal.Status, "dueDate", goal.DueDate, "createdAt", goal.CreatedAt, "description", goal.Description)
}

// 🌿️TicketNodeData MUST return the data map a ticket line renders from.
func TicketNodeData(ticket *TicketNode) map[string]interface{} {
	return dataOf("slug", ticket.Slug, "title", ticket.Title, "status", ticket.Status, "started", ticket.Created, "finished", ticket.Finished, "prompt", ticket.Description, "summary", ticket.Summary)
}

// 🖨️TreeRenderFormat is the output format of a goal tree rendering.
type TreeRenderFormat string

// 🖥️TreeRenderFormatText is the indented text form with box-drawing connectors.
const TreeRenderFormatText TreeRenderFormat = "text"

// 📰️TreeRenderFormatMarkdown is the markdown list form.
const TreeRenderFormatMarkdown TreeRenderFormat = "md"

// ⛳️RenderGoalTreeNodes MUST render one line per goal and ticket.
func RenderGoalTreeNodes(roots []GoalNode, format TreeRenderFormat, renderer EntityRenderer) string {
	var out strings.Builder
	for index := range roots {
		renderGoalNode(&out, &roots[index], "", index == len(roots)-1, true, format, renderer)
	}
	return out.String()
}

// 🎯️renderGoalNode renders one goal and everything below it.
func renderGoalNode(out *strings.Builder, goal *GoalNode, prefix string, isLast bool, isRoot bool, format TreeRenderFormat, renderer EntityRenderer) {
	data := GoalNodeData(goal)
	line := renderer.Human("goal", data)
	if format == TreeRenderFormatMarkdown {
		line = renderer.MarkdownLink("goal", data)
	}
	total := len(goal.Children) + len(goal.Tickets)
	newPrefix := writeTreeLine(out, prefix, line, isLast, isRoot, format)
	for index := range goal.Children {
		renderGoalNode(out, &goal.Children[index], newPrefix, index == total-1, false, format, renderer)
	}
	for index := range goal.Tickets {
		renderTicketNode(out, &goal.Tickets[index], newPrefix, len(goal.Children)+index == total-1, false, format, renderer)
	}
}

// 🎫️renderTicketNode renders one ticket and everything below it.
func renderTicketNode(out *strings.Builder, ticket *TicketNode, prefix string, isLast bool, isRoot bool, format TreeRenderFormat, renderer EntityRenderer) {
	data := TicketNodeData(ticket)
	line := renderer.Human("ticket", data)
	if format == TreeRenderFormatMarkdown {
		line = renderer.MarkdownLink("ticket", data)
	}
	newPrefix := writeTreeLine(out, prefix, line, isLast, isRoot, format)
	for index := range ticket.Children {
		renderTicketNode(out, &ticket.Children[index], newPrefix, index == len(ticket.Children)-1, false, format, renderer)
	}
}

// ✏️writeTreeLine writes one rendered line and returns the prefix its children carry.
func writeTreeLine(out *strings.Builder, prefix string, line string, isLast bool, isRoot bool, format TreeRenderFormat) string {
	if format == TreeRenderFormatMarkdown {
		out.WriteString(prefix + "- " + line + "\n")
		return prefix + "  "
	}
	connector := TextBranchConnector
	if isRoot {
		connector = ""
	} else if isLast {
		connector = TextLastConnector
	}
	out.WriteString(prefix + connector + line + "\n")
	if isRoot {
		return prefix
	}
	if isLast {
		return prefix + TextBlankIndent
	}
	return prefix + TextPipeIndent
}

// #endregion 🎯️PortedGoalTree

// #region 📜️PortedStatuteTree

// 📜️DeclaredStatuteCatalog is the catalog of the law this repository actually declares, resolved
// through 📜️statutes — the production StatuteCatalog every consumer of the statute and territory
// trees shares.
type DeclaredStatuteCatalog struct{}

// ℹInfo MUST return the declared metadata of the statute, and report whether it is declared.
func (DeclaredStatuteCatalog) Info(statute model.Statute) (model.StatuteMeta, bool) {
	meta, declared := model.StatuteInfoTable[statute]
	return meta, declared
}

// 🆔️StatuteID MUST return the artifact id of the statute.
func (DeclaredStatuteCatalog) StatuteID(statute model.Statute) string {
	return "📜️" + strings.NewReplacer("/", "", ".", "").Replace(string(statute))
}

// 🔗️StatuteURI MUST return the repo uri of the statute.
func (DeclaredStatuteCatalog) StatuteURI(statute model.Statute) string {
	return "repo://statute/" + string(statute)
}

// 🏷️StatuteLabel MUST return the last path segment of the statute.
func (DeclaredStatuteCatalog) StatuteLabel(statute model.Statute) string {
	parts := strings.Split(string(statute), "/")
	return parts[len(parts)-1]
}

// 🧱️EntityKind MUST return the first path segment of the statute.
func (DeclaredStatuteCatalog) EntityKind(statute model.Statute) string {
	return strings.Split(string(statute), "/")[0]
}

// 🆔️TerritoryID MUST return the artifact id of the territory.
func (DeclaredStatuteCatalog) TerritoryID(territory model.Territory) string {
	return "🗺️" + strings.NewReplacer(" ", "", "/", "").Replace(territory.Name)
}

// 📜️DeclaredStatutes returns every declared statute, identifier ascending, so the projection never
// depends on the order a map happened to iterate in.
func DeclaredStatutes() []model.Statute {
	kinds := make([]model.Statute, 0, len(model.StatuteInfoTable))
	for kind := range model.StatuteInfoTable {
		kinds = append(kinds, kind)
	}
	sort.Slice(kinds, func(i, j int) bool { return kinds[i] < kinds[j] })
	return kinds
}

// 🗺️DeclaredTerritories returns every territory a declared policy carries at its top level, name
// ascending, each name claimed by the first policy that declares it.
func DeclaredTerritories() []model.Territory {
	seen := map[string]model.Territory{}
	names := []string{}
	for _, policy := range statutespkg.GetPolicies() {
		for _, territory := range policy.Groups {
			if _, claimed := seen[territory.Name]; claimed {
				continue
			}
			seen[territory.Name] = territory
			names = append(names, territory.Name)
		}
	}
	sort.Strings(names)
	territories := make([]model.Territory, 0, len(names))
	for _, name := range names {
		territories = append(territories, seen[name])
	}
	return territories
}

// 🏗️BuildStatuteTree MUST build one category per path segment with statutes as leaves.
func BuildStatuteTree(statutes []model.Statute, catalog StatuteCatalog) []*TreeNode {
	type entry struct {
		node     *TreeNode
		order    []string
		children map[string]*entry
	}
	newEntry := func(node *TreeNode) *entry {
		return &entry{node: node, children: map[string]*entry{}}
	}
	var insert func(current *entry, parts []string, full model.Statute, depth int)
	insert = func(current *entry, parts []string, full model.Statute, depth int) {
		if depth >= len(parts) {
			return
		}
		part := parts[depth]
		if _, ok := current.children[part]; !ok {
			current.order = append(current.order, part)
			var node *TreeNode
			if depth == len(parts)-1 {
				meta, found := catalog.Info(full)
				priority := model.BreachPriorityLow
				if found {
					priority = meta.Priority
				}
				node = NewTreeNode(TreeNodeStatute, string(full), PriorityIcon(priority)+part, catalog.StatuteURI(full))
				if found {
					node.Description = meta.Reason
					if meta.Autofixable {
						node.SubKind = "autofixable"
					}
					node.Data = dataOf("id", string(full), "priority", string(meta.Priority), "autofixable", meta.Autofixable, "reason", meta.Reason, "solution", meta.Solution)
				}
			} else {
				prefix := strings.Join(parts[:depth+1], "/")
				node = NewTreeNode(TreeNodeCategory, "breachCategory:"+prefix, part, "repo://statute/"+identity.Flat(prefix))
			}
			current.children[part] = newEntry(node)
		}
		insert(current.children[part], parts, full, depth+1)
	}
	var collect func(current *entry, ordered bool) []*TreeNode
	collect = func(current *entry, ordered bool) []*TreeNode {
		keys := current.order
		if !ordered {
			keys = make([]string, 0, len(current.children))
			for key := range current.children {
				keys = append(keys, key)
			}
			sort.Strings(keys)
		}
		seen := map[string]bool{}
		result := []*TreeNode{}
		for _, key := range keys {
			if seen[key] {
				continue
			}
			seen[key] = true
			child, ok := current.children[key]
			if !ok {
				continue
			}
			delete(current.children, key)
			node := child.node
			if grandchildren := collect(child, false); len(grandchildren) > 0 {
				node.Children = grandchildren
			}
			result = append(result, node)
		}
		return result
	}
	root := newEntry(nil)
	for _, statute := range statutes {
		insert(root, strings.Split(string(statute), "/"), statute, 0)
	}
	return collect(root, true)
}

// 🔷️BuildTerritoryTree MUST build one category per territory with its statutes and children.
func BuildTerritoryTree(territories []model.Territory, catalog StatuteCatalog) []*TreeNode {
	territoryEmoji := identity.EmojiText(identity.Entity("territory"))
	result := []*TreeNode{}
	for _, territory := range territories {
		node := NewTreeNode(TreeNodeCategory, territoryEmoji+territory.Name, territory.Name, "repo://territory/"+catalog.TerritoryID(territory))
		node.Description = territory.Description
		node.SubKind = "territory"
		node.Data = dataOf("name", territory.Name, "description", territory.Description, "scopes", list(territory.Scopes))
		for _, statute := range territory.Kinds {
			PushChild(node, StatuteLeafNode(statute, catalog))
		}
		for _, child := range BuildTerritoryTree(territory.Groups, catalog) {
			PushChild(node, child)
		}
		result = append(result, node)
	}
	return result
}

// 📜️StatuteLeafNode MUST build the leaf node of one statute, labelled with its priority icon.
func StatuteLeafNode(statute model.Statute, catalog StatuteCatalog) *TreeNode {
	meta, found := catalog.Info(statute)
	priority := model.BreachPriorityLow
	if found {
		priority = meta.Priority
	}
	node := NewTreeNode(TreeNodeStatute, catalog.StatuteID(statute), PriorityIcon(priority)+catalog.StatuteLabel(statute), catalog.StatuteURI(statute))
	if found {
		node.Description = meta.Reason
		if meta.Autofixable {
			node.SubKind = "autofixable"
		}
		node.Data = dataOf("id", string(statute), "priority", string(meta.Priority), "autofixable", meta.Autofixable, "reason", meta.Reason, "solution", meta.Solution)
	}
	return node
}

// 🧱️BuildPolicyEntityKindTree MUST group the statutes of a policy's territories by entity kind.
func BuildPolicyEntityKindTree(territories []model.Territory, catalog StatuteCatalog) []*TreeNode {
	grouped := map[string]map[string]bool{}
	var collect func(entries []model.Territory)
	collect = func(entries []model.Territory) {
		for _, entry := range entries {
			for _, statute := range entry.Kinds {
				kind := catalog.EntityKind(statute)
				if grouped[kind] == nil {
					grouped[kind] = map[string]bool{}
				}
				grouped[kind][string(statute)] = true
			}
			collect(entry.Groups)
		}
	}
	collect(territories)
	kinds := make([]string, 0, len(grouped))
	for kind := range grouped {
		kinds = append(kinds, kind)
	}
	sort.Strings(kinds)
	result := []*TreeNode{}
	for _, kind := range kinds {
		node := NewTreeNode(TreeNodeCategory, "entitykind:"+kind, kind, "repo://entitykind/"+kind)
		node.SubKind = "entitykind"
		node.Data = dataOf("kind", kind)
		statutes := make([]string, 0, len(grouped[kind]))
		for statute := range grouped[kind] {
			statutes = append(statutes, statute)
		}
		sort.Strings(statutes)
		for _, statute := range statutes {
			PushChild(node, StatuteLeafNode(model.Statute(statute), catalog))
		}
		result = append(result, node)
	}
	return result
}

// #endregion 📜️PortedStatuteTree

// #region 🧾️Outline

// 🧾️TreeOutline MUST return one depth-kind-id-label row per node, in pre-order.
func TreeOutline(root *TreeNode) []string {
	rows := []string{}
	var walk func(node *TreeNode, depth int)
	walk = func(node *TreeNode, depth int) {
		rows = append(rows, strconv.Itoa(depth)+"|"+string(node.Kind)+"|"+node.ID+"|"+node.Label)
		for _, child := range node.Children {
			walk(child, depth+1)
		}
	}
	walk(root, 0)
	return rows
}

// #endregion 🧾️Outline

// #region 🧜️PortedMermaid

// 🧜️MermaidNode is one node of a Mermaid treemap-beta diagram.
type MermaidNode struct {
	Label    string        `json:"label"`
	Value    *int64        `json:"value,omitempty"`
	Children []MermaidNode `json:"children,omitempty"`
}

// 🧜️MermaidTreemap is a Mermaid treemap-beta diagram.
type MermaidTreemap struct {
	Title string        `json:"title"`
	Nodes []MermaidNode `json:"nodes"`
}

// 📥️ParseMermaidTreemap MUST return an error when the document is malformed.
func ParseMermaidTreemap(document []byte) (*MermaidTreemap, error) {
	treemap := &MermaidTreemap{}
	if err := json.Unmarshal(document, treemap); err != nil {
		return nil, err
	}
	return treemap, nil
}

// 🔤️MermaidEscapeLabel MUST turn every double quote into an apostrophe.
func MermaidEscapeLabel(value string) string {
	return strings.ReplaceAll(value, "\"", "'")
}

// 🧜️RenderMermaidTreemap MUST render the header, the quoted title and the indented node forest.
func RenderMermaidTreemap(treemap *MermaidTreemap) string {
	var out strings.Builder
	out.WriteString("treemap-beta\n")
	out.WriteString("\"" + MermaidEscapeLabel(treemap.Title) + "\"\n")
	for index := range treemap.Nodes {
		renderMermaidNode(&out, &treemap.Nodes[index], 1)
	}
	return out.String()
}

// 🧜️renderMermaidNode renders one treemap node at a depth.
func renderMermaidNode(out *strings.Builder, node *MermaidNode, depth int) {
	indent := strings.Repeat(TextBlankIndent, depth)
	if node.Value != nil && len(node.Children) == 0 {
		out.WriteString(indent + "\"" + MermaidEscapeLabel(node.Label) + "\": " + strconv.FormatInt(*node.Value, 10) + "\n")
	} else {
		out.WriteString(indent + "\"" + MermaidEscapeLabel(node.Label) + "\"\n")
	}
	for index := range node.Children {
		renderMermaidNode(out, &node.Children[index], depth+1)
	}
}

// 🌳️MermaidTreemapFromTree MUST project a monorepo tree into a treemap.
func MermaidTreemapFromTree(root *TreeNode, title string, weightKey string) *MermaidTreemap {
	treemap := &MermaidTreemap{Title: title}
	for _, child := range root.Children {
		treemap.Nodes = append(treemap.Nodes, mermaidNodeFromTree(child, weightKey))
	}
	return treemap
}

// 🌿️mermaidNodeFromTree projects one tree node into a treemap node.
func mermaidNodeFromTree(node *TreeNode, weightKey string) MermaidNode {
	out := MermaidNode{Label: node.Label}
	if node.Data != nil {
		switch value := node.Data[weightKey].(type) {
		case float64:
			whole := int64(value)
			out.Value = &whole
		case int:
			whole := int64(value)
			out.Value = &whole
		case int64:
			whole := value
			out.Value = &whole
		case json.Number:
			if whole, err := value.Int64(); err == nil {
				out.Value = &whole
			}
		}
	}
	for _, child := range node.Children {
		out.Children = append(out.Children, mermaidNodeFromTree(child, weightKey))
	}
	return out
}

// #endregion 🧜️PortedMermaid

// #region 📌️PortedCache

// 💿️TreeCacheMeta is the metadata a cached tree carries.
type TreeCacheMeta struct {
	SchemaVersion   int    `json:"SchemaVersion"`
	Fingerprint     string `json:"Fingerprint"`
	IncludeSections bool   `json:"IncludeSections"`
	ContentDigest   string `json:"ContentDigest,omitempty"`
}

// 🆕️TreeCacheMetaOf MUST build the metadata of a freshly computed tree.
func TreeCacheMetaOf(tree *TreeNode, fingerprint string, includeSections bool) TreeCacheMeta {
	return TreeCacheMeta{SchemaVersion: TreeCacheSchemaVersion, Fingerprint: fingerprint, IncludeSections: includeSections, ContentDigest: TreeContentDigest(tree)}
}

// ♻️TreeCacheIsValid MUST return true only when schema, fingerprint and section choice all match.
func TreeCacheIsValid(meta TreeCacheMeta, fingerprint string, includeSections bool) bool {
	return meta.SchemaVersion == TreeCacheSchemaVersion && meta.Fingerprint == fingerprint && meta.IncludeSections == includeSections
}

// 🔏️TreeContentDigest MUST return the SHA-256 of the canonical JSON encoding of a tree.
func TreeContentDigest(tree *TreeNode) string {
	sum := sha256.Sum256([]byte(CanonicalTreeJSON(tree)))
	return hex.EncodeToString(sum[:])
}

// 🔣️CanonicalTreeJSON MUST encode a tree with every map key in sorted order.
func CanonicalTreeJSON(tree *TreeNode) string {
	var out strings.Builder
	writeCanonicalTreeNode(&out, tree)
	return out.String()
}

// 🔣️writeCanonicalTreeNode writes one node in the wire shape both implementations share.
func writeCanonicalTreeNode(out *strings.Builder, node *TreeNode) {
	if node == nil {
		out.WriteString("null")
		return
	}
	out.WriteString("{\"Kind\":")
	writeCanonicalString(out, string(node.Kind))
	out.WriteString(",\"ID\":")
	writeCanonicalString(out, node.ID)
	out.WriteString(",\"Label\":")
	writeCanonicalString(out, node.Label)
	out.WriteString(",\"URI\":")
	writeCanonicalString(out, node.URI)
	out.WriteString(",\"SubKind\":")
	writeCanonicalString(out, node.SubKind)
	out.WriteString(",\"Description\":")
	writeCanonicalString(out, node.Description)
	out.WriteString(",\"Summary\":")
	writeCanonicalString(out, node.Summary)
	out.WriteString(",\"Year\":" + strconv.Itoa(node.Year))
	out.WriteString(",\"Month\":" + strconv.Itoa(node.Month))
	out.WriteString(",\"Day\":" + strconv.Itoa(node.Day))
	out.WriteString(",\"Status\":")
	writeCanonicalString(out, node.Status)
	out.WriteString(",\"Contributor\":")
	writeCanonicalString(out, node.Contributor)
	out.WriteString(",\"Data\":")
	writeCanonicalData(out, node.Data)
	out.WriteString(",\"Children\":")
	if node.Children == nil {
		out.WriteString("null")
	} else {
		out.WriteString("[")
		for index, child := range node.Children {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalTreeNode(out, child)
		}
		out.WriteString("]")
	}
	out.WriteString("}")
}

// 🔣️writeCanonicalData writes a data map with sorted keys.
func writeCanonicalData(out *strings.Builder, data map[string]interface{}) {
	if data == nil {
		out.WriteString("null")
		return
	}
	keys := make([]string, 0, len(data))
	for key := range data {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	out.WriteString("{")
	for index, key := range keys {
		if index > 0 {
			out.WriteString(",")
		}
		writeCanonicalString(out, key)
		out.WriteString(":")
		writeCanonicalValue(out, data[key])
	}
	out.WriteString("}")
}

// 🔣️writeCanonicalValue writes one JSON value the way the Rust encoder writes it.
func writeCanonicalValue(out *strings.Builder, value interface{}) {
	switch typed := value.(type) {
	case nil:
		out.WriteString("null")
	case string:
		writeCanonicalString(out, typed)
	case bool:
		out.WriteString(strconv.FormatBool(typed))
	case float64:
		out.WriteString(canonicalFloat(typed))
	case int:
		out.WriteString(canonicalFloat(float64(typed)))
	case int64:
		out.WriteString(canonicalFloat(float64(typed)))
	case []interface{}:
		out.WriteString("[")
		for index, item := range typed {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalValue(out, item)
		}
		out.WriteString("]")
	case []string:
		out.WriteString("[")
		for index, item := range typed {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalString(out, item)
		}
		out.WriteString("]")
	case map[string]interface{}:
		writeCanonicalData(out, typed)
	default:
		encoded, err := json.Marshal(typed)
		if err != nil {
			out.WriteString("null")
			return
		}
		out.Write(encoded)
	}
}

// 🔢️canonicalFloat formats a float the way the Rust encoder formats an f64.
func canonicalFloat(value float64) string {
	if value == float64(int64(value)) {
		return strconv.FormatInt(int64(value), 10) + ".0"
	}
	return strconv.FormatFloat(value, 'g', -1, 64)
}

// 🔣️writeCanonicalString writes a JSON string escaped the way the Rust encoder escapes it.
func writeCanonicalString(out *strings.Builder, value string) {
	out.WriteByte('"')
	for _, current := range value {
		switch current {
		case '"':
			out.WriteString("\\\"")
		case '\\':
			out.WriteString("\\\\")
		case '\n':
			out.WriteString("\\n")
		case '\r':
			out.WriteString("\\r")
		case '\t':
			out.WriteString("\\t")
		case '\b':
			out.WriteString("\\b")
		case '\f':
			out.WriteString("\\f")
		default:
			if current < 0x20 {
				out.WriteString(fmt.Sprintf("\\u%04x", current))
				continue
			}
			out.WriteRune(current)
		}
	}
	out.WriteByte('"')
}

// 🗜️EncodeTreeCachePayload MUST return the gzip-wrapped canonical JSON of a tree.
func EncodeTreeCachePayload(tree *TreeNode) ([]byte, error) {
	var buffer bytes.Buffer
	writer := gzip.NewWriter(&buffer)
	if _, err := writer.Write([]byte(CanonicalTreeJSON(tree))); err != nil {
		return nil, err
	}
	if err := writer.Close(); err != nil {
		return nil, err
	}
	return buffer.Bytes(), nil
}

// 🗜️DecodeTreeCachePayload MUST return an error when the payload is not a gzip-wrapped tree.
func DecodeTreeCachePayload(payload []byte) (*TreeNode, error) {
	reader, err := gzip.NewReader(bytes.NewReader(payload))
	if err != nil {
		return nil, err
	}
	defer func() { _ = reader.Close() }()
	plain, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	node := &TreeNode{}
	if err := json.Unmarshal(plain, node); err != nil {
		return nil, err
	}
	return node, nil
}

// #endregion 📌️PortedCache

// 📅️checkpointDatePart reads one `YYYY-MM-DD` component out of the ISO timestamp a checkpoint
// carries as text, and reports zero when the timestamp is shorter than the component.
func checkpointDatePart(date string, from int, to int) int {
	if len(date) < to {
		return 0
	}
	value, err := strconv.Atoi(date[from:to])
	if err != nil {
		return 0
	}
	return value
}
