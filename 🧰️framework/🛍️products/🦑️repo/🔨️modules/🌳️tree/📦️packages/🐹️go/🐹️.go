// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌳️tree is the tree domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package tree

import (
	fmt "fmt"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

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

// 🧩️Tree node kinds, filter criteria, and matching logic for monorepo tree queries.
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
