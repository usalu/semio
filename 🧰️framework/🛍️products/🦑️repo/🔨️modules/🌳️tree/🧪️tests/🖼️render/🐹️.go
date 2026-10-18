// 🖼️ Tests of the monorepo tree text and markdown rendering.

package tree

import (
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

func TestRenderMonorepoTree(t *testing.T) {
	t.Run("renders basic tree", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "🖥️Codebase") {
			t.Error("output should contain Codebase label")
		}
		if !strings.Contains(output, "compose") {
			t.Error("output should contain technology name")
		}
	})

	t.Run("renders category URI", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals"},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "[🎯️Goals](repo://goals)") {
			t.Errorf("output should contain category with URI link, got: %s", output)
		}
	})

	t.Run("renders nested tree with connectors", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "proj1"},
					{Kind: TreeNodeTechnology, ID: "p2", Label: "proj2"},
				}},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "├️─️─️ ") || !strings.Contains(output, "└️─️─️ ") {
			t.Errorf("output should contain tree connectors, got: %s", output)
		}
	})

	t.Run("empty tree renders nothing", func(t *testing.T) {
		tree := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if output != "" {
			t.Errorf("empty tree should render nothing, got: %q", output)
		}
	})

	t.Run("markdown renderer uses list bullets", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTreeMarkdown(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "- [🖥️Codebase](repo://codebase)") {
			t.Errorf("markdown tree should contain markdown link list item, got: %s", output)
		}
		if !strings.Contains(output, "  - compose") {
			t.Errorf("markdown tree should contain nested bullet item, got: %s", output)
		}
		if strings.Contains(output, "├️─️─️ ") || strings.Contains(output, "└️─️─️ ") {
			t.Errorf("markdown tree must not contain ascii connectors, got: %s", output)
		}
	})

	t.Run("text tree shows only own ID segment not full parent chain", func(t *testing.T) {
		parentGoalData := map[string]interface{}{
			"id":     "parentgoal",
			"title":  "Parent Goal",
			"status": "open",
		}
		childGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal",
			"title":    "Child Goal",
			"status":   "open",
			"parentId": "🎯️parentgoal",
		}
		grandchildGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal/grandchildgoal",
			"title":    "Grandchild Goal",
			"status":   "open",
			"parentId": "🎯️parentgoal🎯️childgoal",
		}
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "parentgoal", Label: "Parent Goal", Data: parentGoalData, Children: []*TreeNode{
						{Kind: TreeNodeGoal, ID: "childgoal", Label: "Child Goal", Data: childGoalData, Children: []*TreeNode{
							{Kind: TreeNodeGoal, ID: "grandchildgoal", Label: "Grandchild Goal", Data: grandchildGoalData},
						}},
					}},
				}},
			},
		}
		goalEmoji := model.EmojiText(model.EmojiGoal)
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		lines := strings.Split(strings.TrimRight(output, "\n"), "\n")
		for _, line := range lines {
			if strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji+"childgoal") {
				t.Errorf("tree text should not contain full hierarchical ID, got line: %s", line)
			}
			if strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji) {
				t.Errorf("tree text should not contain parent prefix in child line, got line: %s", line)
			}
		}
		childFound := false
		grandchildFound := false
		for _, line := range lines {
			if strings.Contains(line, goalEmoji+"childgoal") && !strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji+"childgoal") {
				childFound = true
			}
			if strings.Contains(line, goalEmoji+"grandchildgoal") && !strings.Contains(line, goalEmoji+"childgoal"+goalEmoji+"grandchildgoal") {
				grandchildFound = true
			}
		}
		if !childFound {
			t.Errorf("tree text should contain short child ID 🎯️childgoal, got:\n%s", output)
		}
		if !grandchildFound {
			t.Errorf("tree text should contain short grandchild ID 🎯️grandchildgoal, got:\n%s", output)
		}
	})

	t.Run("text tree preserves parentId on data after rendering", func(t *testing.T) {
		data := map[string]interface{}{
			"id":       "parent/child",
			"title":    "Child",
			"status":   "open",
			"parentId": "🎯️parent",
		}
		node := &TreeNode{Kind: TreeNodeGoal, ID: "child", Label: "Child", Data: data}
		var sb strings.Builder
		RenderTreeNodeText(&sb, node, "", true, true, DefaultEntityRenderer{})
		if data["parentId"] != "🎯️parent" {
			t.Errorf("renderTreeNodeText should restore parentId, got: %v", data["parentId"])
		}
	})
}

// 🌳️#region 📋️Unified Rendering Identity
func TestTreeNodeKindToEntityKindCoversAll(t *testing.T) {
	kinds := []struct {
		kind     TreeNodeKind
		expected string
	}{
		{TreeNodeTechnology, "technology"},
		{TreeNodeBundle, "bundle"},
		{TreeNodeFolder, "folder"},
		{TreeNodeFile, "file"},
		{TreeNodeSection, "section"},
		{TreeNodeDefinition, "definition"},
		{TreeNodeGoal, "goal"},
		{TreeNodeTicket, "ticket"},
		{TreeNodeDraft, "draft"},
		{TreeNodePolicy, "policy"},
		{TreeNodeStatute, ""},
		{TreeNodeContributor, "contributor"},
		{TreeNodeCheckpoint, "checkpoint"},
		{TreeNodeCategory, ""},
	}
	for _, tt := range kinds {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := TreeNodeKindToEntityKind(tt.kind)
			if got != tt.expected {
				t.Errorf("treeNodeKindToEntityKind(%q) = %q, want %q", tt.kind, got, tt.expected)
			}
		})
	}
	t.Run("unknown returns empty", func(t *testing.T) {
		got := TreeNodeKindToEntityKind(TreeNodeKind("unknown"))
		if got != "" {
			t.Errorf("treeNodeKindToEntityKind(unknown) = %q, want empty", got)
		}
	})
}

func TestNoDoubleDashInMarkdownOutput(t *testing.T) {
	kinds := []struct {
		kind string
		data map[string]interface{}
	}{
		{"goal", map[string]interface{}{
			"id": "G1", "title": "Goal", "status": "open",
		}},
		{"ticket", map[string]interface{}{
			"slug": "T1", "title": "Ticket", "status": "open",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"section", map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(1), "endLine": float64(5),
		}},
		{"bundle", map[string]interface{}{
			"name": "b1", "root": "path",
		}},
		{"folder", map[string]interface{}{
			"path": "src/f",
		}},
		{"file", map[string]interface{}{
			"path": "src/a.ts",
		}},
		{"contributor", map[string]interface{}{
			"github": "dev",
		}},
		{"checkpoint", map[string]interface{}{
			"sha": "abc",
		}},
	}

	for _, tt := range kinds {
		t.Run(tt.kind+"_renderEntityMarkdown", func(t *testing.T) {
			output := model.RenderEntityMarkdown(tt.kind, tt.data)
			if count := strings.Count(output, "- "); count > 1 {
				dashPositions := []int{}
				idx := 0
				for {
					pos := strings.Index(output[idx:], "- ")
					if pos == -1 {
						break
					}
					dashPositions = append(dashPositions, idx+pos)
					idx += pos + 2
				}
				if len(dashPositions) >= 2 && dashPositions[1]-dashPositions[0] <= 3 {
					t.Errorf("renderEntityMarkdown(%s) has double dash at start: %q", tt.kind, output)
				}
			}
		})

		t.Run(tt.kind+"_treeNodeMarkdown", func(t *testing.T) {
			nodeKind := TreeNodeKind(tt.kind)
			switch tt.kind {
			case "goal":
				nodeKind = TreeNodeGoal
			case "ticket":
				nodeKind = TreeNodeTicket
			case "section":
				nodeKind = TreeNodeSection
			case "bundle":
				nodeKind = TreeNodeBundle
			case "folder":
				nodeKind = TreeNodeFolder
			case "file":
				nodeKind = TreeNodeFile
			case "contributor":
				nodeKind = TreeNodeContributor
			case "checkpoint":
				nodeKind = TreeNodeCheckpoint
			}

			treeNode := &TreeNode{Kind: nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			RenderTreeNodeMarkdown(&sb, treeNode, "", DefaultEntityRenderer{})
			output := sb.String()
			if strings.HasPrefix(output, "- - ") {
				t.Errorf("renderTreeNodeMarkdown(%s) has double dash: %q", tt.kind, output)
			}
		})
	}

	t.Run("goalTreeNodes_no_double_dash", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID: "G1", Title: "Goal", Status: "open",
			Tickets: []*model.TicketNode{{
				Slug: "T1", Title: "Ticket", Status: "open",
			}},
		}}
		output := RenderModelGoalTreeNodes(roots, "md")
		for i, line := range strings.Split(output, "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if strings.HasPrefix(trimmed, "- - ") {
				t.Errorf("line %d has double dash: %q", i, line)
			}
		}
	})
}

func TestMermaidEscapeLabel(t *testing.T) {
	if got := mermaidEscapeLabel("hello \"world\""); got != "hello 'world'" {
		t.Errorf("expected hello 'world', got: %s", got)
	}
	if got := mermaidEscapeLabel("no quotes"); got != "no quotes" {
		t.Errorf("expected no quotes, got: %s", got)
	}
}
