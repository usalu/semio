// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖨️render carries the monorepo tree text and markdown renderers and the tree outline.

// #endregion 🧲️Header

package tree

import (
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🩻️Monorepo Tree

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
