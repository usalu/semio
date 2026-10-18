// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Unified rendering identity across entity kinds.

// #endregion 🧲️Header

package client

import (
	"strings"
	"testing"
)

func TestUnifiedRenderingGoalIdentity(t *testing.T) {
	data := map[string]interface{}{
		"id":          "TEST-GOAL",
		"title":       "Test Goal",
		"status":      "open",
		"dueDate":     "2030-01-01",
		"createdAt":   "2025-01-01T00:00:00Z",
		"description": "A test goal",
	}

	mdLink := renderEntityMarkdownLink("goal", data)
	mdItem := renderEntityMarkdown("goal", data)
	humanItem := renderEntityHuman("goal", data, false)

	t.Run("renderEntityMarkdown is dash-prefixed renderEntityMarkdownLink", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("markdown link has artifact ID and URI", func(t *testing.T) {
		if !strings.Contains(mdLink, "["+emojiText(EmojiGoal)) {
			t.Errorf("markdown link missing goal emoji prefix: %s", mdLink)
		}
		if !strings.Contains(mdLink, "](repo://goal/") {
			t.Errorf("markdown link missing goal URI: %s", mdLink)
		}
	})

	t.Run("human has artifact ID", func(t *testing.T) {
		if !strings.Contains(humanItem, emojiText(EmojiGoal)) {
			t.Errorf("human output missing goal emoji: %s", humanItem)
		}
	})

	t.Run("both formats share same props from collectEntityProps", func(t *testing.T) {
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("goalNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &GoalNode{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}
		nodeData := goalNodeToData(node)
		fromNode := renderEntityMarkdownLink("goal", nodeData)
		fromDirect := renderEntityMarkdownLink("goal", data)
		if fromNode != fromDirect {
			t.Errorf("goalNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree markdown uses renderEntityMarkdownLink for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		expectedLink := renderEntityMarkdownLink("goal", data)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("goal tree markdown must not have double dash: %q", treeOutput)
		}
	})

	t.Run("goal tree text uses renderEntityHuman for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "text")
		expectedHuman := renderEntityHuman("goal", data, false)
		if !strings.Contains(treeOutput, expectedHuman) {
			t.Errorf("goal tree text should contain renderEntityHuman output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("monorepo tree node markdown matches goal tree markdown", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + emojiText(EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "md"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree markdown and goal tree markdown differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})

	t.Run("monorepo tree node text matches goal tree text", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + emojiText(EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeText(&sb, treeNode, "", true, true)
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "text"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree text and goal tree text differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})
}

func TestUnifiedRenderingTicketIdentity(t *testing.T) {
	data := map[string]interface{}{
		"slug":     "MY-TICKET",
		"title":    "My Ticket",
		"status":   "open",
		"started":  "2025-01-01T00:00:00Z",
		"finished": "",
		"prompt":   "Fix something",
		"summary":  "",
		"year":     float64(2025),
		"month":    float64(1),
		"day":      float64(1),
	}

	mdLink := renderEntityMarkdownLink("ticket", data)
	mdItem := renderEntityMarkdown("ticket", data)
	humanItem := renderEntityHuman("ticket", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("ticketNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &TicketNode{
			Slug:        "MY-TICKET",
			Title:       "My Ticket",
			Status:      "open",
			Created:     "2025-01-01T00:00:00Z",
			Finished:    "",
			Description: "Fix something",
			Summary:     "",
		}
		nodeData := ticketNodeToData(node)
		nodeData["year"] = float64(2025)
		nodeData["month"] = float64(1)
		nodeData["day"] = float64(1)
		fromNode := renderEntityMarkdownLink("ticket", nodeData)
		fromDirect := renderEntityMarkdownLink("ticket", data)
		if fromNode != fromDirect {
			t.Errorf("ticketNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree ticket markdown uses renderEntityMarkdownLink", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "MY-TICKET", Title: "My Ticket", Status: "open",
				Created: "2025-01-01T00:00:00Z", Description: "Fix something",
			}},
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		ticketData := ticketNodeToData(roots[0].Tickets[0])
		expectedLink := renderEntityMarkdownLink("ticket", ticketData)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree ticket markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("ticket in goal tree must not have double dash: %q", treeOutput)
		}
	})

	t.Run("ticket list markdown matches renderEntityMarkdown", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, true))
		directMD := strings.TrimSpace(renderEntityMarkdown("ticket", data))
		if listOutput != directMD {
			t.Errorf("ticket list markdown should match renderEntityMarkdown.\n  List:   %q\n  Direct: %q", listOutput, directMD)
		}
	})

	t.Run("ticket list text matches renderEntityHuman", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, false))
		directHuman := renderEntityHuman("ticket", data, false)
		if !strings.Contains(listOutput, directHuman) {
			t.Errorf("ticket list text should contain renderEntityHuman output.\n  List:   %q\n  Direct: %q", listOutput, directHuman)
		}
	})
}

func TestUnifiedRenderingSectionIdentity(t *testing.T) {
	data := map[string]interface{}{
		"path":      "test/file.ts#MySection",
		"name":      "MySection",
		"startLine": float64(10),
		"endLine":   float64(20),
	}

	mdLink := renderEntityMarkdownLink("section", data)
	mdItem := renderEntityMarkdown("section", data)
	humanItem := renderEntityHuman("section", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("section", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("section tree markdown uses renderEntityMarkdown", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, true))
		expectedMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if treeOutput != expectedMD {
			t.Errorf("section tree markdown root should match renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedMD)
		}
	})

	t.Run("section tree text uses renderEntityHuman", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, false))
		expectedHuman := renderEntityHuman("section", data, false)
		if treeOutput != expectedHuman {
			t.Errorf("section tree text root should match renderEntityHuman.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("section tree markdown preserves indentation for children", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#Parent",
			Name:      "Parent",
			StartLine: 1,
			EndLine:   30,
			Children: []Section{{
				Path:      "test/file.ts#Child",
				Name:      "Child",
				StartLine: 5,
				EndLine:   15,
			}},
		}
		treeOutput := renderSectionTree(s, false, true)
		lines := strings.Split(strings.TrimSpace(treeOutput), "\n")
		if len(lines) < 2 {
			t.Fatalf("expected at least 2 lines, got %d: %q", len(lines), treeOutput)
		}
		if !strings.HasPrefix(lines[0], "- [") {
			t.Errorf("root section should start with '- [': %q", lines[0])
		}
		if !strings.HasPrefix(lines[1], "  - [") {
			t.Errorf("child section should start with '  - [' for 2-space indent: %q", lines[1])
		}
	})

	t.Run("monorepo tree node markdown matches direct rendering", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeSection,
			ID:    "sec1",
			Label: "MySection",
			URI:   "repo://section/" + emojiText(EmojiFileCode) + "file" + emojiText(EmojiSection) + "mysection",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())
		directMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if monorepoOutput != directMD {
			t.Errorf("monorepo tree section markdown should match renderEntityMarkdown.\n  Monorepo: %q\n  Direct:   %q", monorepoOutput, directMD)
		}
	})
}

func TestUnifiedRenderingAllKindIdentity(t *testing.T) {
	entities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"technology", TreeNodeTechnology, map[string]interface{}{
			"name": "mytechnology", "description": "A technology",
		}},
		{"bundle", TreeNodeBundle, map[string]interface{}{
			"name": "mybundle", "root": "path/to/bundle",
		}},
		{"folder", TreeNodeFolder, map[string]interface{}{
			"path": "src/folder", "name": "folder",
		}},
		{"file", TreeNodeFile, map[string]interface{}{
			"path": "src/file.ts", "name": "file.ts",
		}},
		{"contributor", TreeNodeContributor, map[string]interface{}{
			"github": "dev1", "name": "Developer One",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "code-hygiene", "name": "Code Hygiene", "description": "Clean code policy",
		}},
		{"statute", TreeNodeStatute, map[string]interface{}{
			"id": "inline-comment", "description": "No inline comments",
		}},
		{"draft", TreeNodeDraft, map[string]interface{}{
			"id": "draft-1", "slug": "my-draft",
		}},
		{"checkpoint", TreeNodeCheckpoint, map[string]interface{}{
			"sha": "abc1234567890", "message": "fix: something",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind+"_markdown_identity", func(t *testing.T) {
			directMD := renderEntityMarkdown(tt.kind, tt.data)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			treeOutput := strings.TrimSpace(sb.String())
			directMDTrimmed := strings.TrimSpace(directMD)
			if treeOutput != directMDTrimmed {
				t.Errorf("%s: monorepo tree markdown differs from direct renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directMDTrimmed)
			}
		})

		t.Run(tt.kind+"_text_identity", func(t *testing.T) {
			directHuman := renderEntityHuman(tt.kind, tt.data, false)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			treeOutput := strings.TrimSpace(sb.String())
			if treeOutput != directHuman {
				t.Errorf("%s: monorepo tree text differs from direct renderEntityHuman.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directHuman)
			}
		})

		t.Run(tt.kind+"_props_in_both_formats", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			mdLink := renderEntityMarkdownLink(tt.kind, tt.data)
			human := renderEntityHuman(tt.kind, tt.data, false)
			for _, p := range props {
				if !strings.Contains(mdLink, p) {
					t.Errorf("%s: markdown link missing prop %q: %s", tt.kind, p, mdLink)
				}
				if !strings.Contains(human, p) {
					t.Errorf("%s: human output missing prop %q: %s", tt.kind, p, human)
				}
			}
		})
	}
}
