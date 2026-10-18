// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Tree sorting, filtering, and text/markdown rendering.

// #endregion 🧲️Header

package client

import (
	"sort"
	"strings"
)

// 📶️sortTreeChildren holds the data fields for a sortTreeChildren record.
func sortTreeChildren(node *TreeNode) {
	sort.Slice(node.Children, func(i, j int) bool {
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
		sortTreeChildren(c)
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

// 🎨️RenderMonorepoTree MUST produce a complete monorepo tree output.
// 🎨️RenderMonorepoTree renders the monorepo tree into its output representation.
func RenderMonorepoTree(root *TreeNode) string {
	var sb strings.Builder
	for i, c := range root.Children {
		renderTreeNodeText(&sb, c, "", i == len(root.Children)-1, true)
	}
	return sb.String()
}

// 📰️RenderMonorepoTreeMarkdown MUST produce a complete monorepo tree markdown output.
// 📰️RenderMonorepoTreeMarkdown renders the monorepo tree markdown into its output representation.
func RenderMonorepoTreeMarkdown(root *TreeNode) string {
	var sb strings.Builder
	for _, c := range root.Children {
		renderTreeNodeMarkdown(&sb, c, "")
	}
	return sb.String()
}

// 🌿️treeNodeKindToEntityKind holds the data fields for a treeNodeKindToEntityKind record.
func treeNodeKindToEntityKind(k TreeNodeKind) string {
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
func renderTreeNodeText(sb *strings.Builder, node *TreeNode, prefix string, isLast bool, isRoot bool) {
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
		entityKind := treeNodeKindToEntityKind(node.Kind)
		if entityKind == "" {
			entityKind = string(node.Kind)
		}
		origParentId, hadParentId := node.Data["parentId"]
		node.Data["parentId"] = ""
		result := renderEntityHuman(entityKind, node.Data, false)
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
		renderTreeNodeText(sb, c, newPrefix, i == len(node.Children)-1, false)
	}
}

// 🔻️renderTreeNodeMarkdown holds the data fields for a renderTreeNodeMarkdown record.
func renderTreeNodeMarkdown(sb *strings.Builder, node *TreeNode, indent string) {
	if node.Kind == TreeNodeCategory {
		sb.WriteString(renderTemplate(mdTpl, "md/tree_node_category", map[string]interface{}{
			"Indent": indent,
			"Label":  node.Label,
			"URI":    node.URI,
		}) + "\n")
	} else if node.Data != nil {
		entityKind := treeNodeKindToEntityKind(node.Kind)
		if entityKind == "" {
			entityKind = string(node.Kind)
		}
		sb.WriteString(renderTemplate(mdTpl, "md/tree_node_entity", map[string]interface{}{
			"Indent":  indent,
			"Content": renderEntityMarkdown(entityKind, node.Data),
		}) + "\n")
	} else {
		sb.WriteString(renderTemplate(mdTpl, "md/tree_node_category", map[string]interface{}{
			"Indent": indent,
			"Label":  node.Label,
			"URI":    node.URI,
		}) + "\n")
	}
	for _, c := range node.Children {
		renderTreeNodeMarkdown(sb, c, indent+"  ")
	}
}
