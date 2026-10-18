// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Entity property consistency and single-line/markdown output laws.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"strings"
	"testing"
)

func TestCollectEntityPropsConsistency(t *testing.T) {
	t.Run("goal props include all fields", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "G1",
			"title":       "My Goal",
			"status":      "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description",
		}
		props := collectEntityProps("goal", data, false)
		if len(props) < 4 {
			t.Errorf("goal props should have >= 4 entries (title, status, created, due, desc), got %d: %v", len(props), props)
		}
		found := map[string]bool{}
		for _, p := range props {
			if strings.Contains(p, "My Goal") {
				found["title"] = true
			}
			if strings.Contains(p, "open") {
				found["status"] = true
			}
			if strings.Contains(p, "created") {
				found["created"] = true
			}
			if strings.Contains(p, "Description") {
				found["description"] = true
			}
		}
		for _, key := range []string{"title", "status", "created", "description"} {
			if !found[key] {
				t.Errorf("goal props missing %s: %v", key, props)
			}
		}
	})

	t.Run("ticket open props include prompt", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "open",
			"started": "2025-01-01T00:00:00Z", "prompt": "Please fix",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Please fix") {
				found = true
			}
		}
		if !found {
			t.Errorf("open ticket props should contain prompt: %v", props)
		}
	})

	t.Run("ticket closed props include summary", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z", "summary": "Fixed the bug",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Fixed the bug") {
				found = true
			}
		}
		if !found {
			t.Errorf("closed ticket props should contain summary: %v", props)
		}
	})

	t.Run("section props include line range", func(t *testing.T) {
		data := map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(10), "endLine": float64(20),
		}
		props := collectEntityProps("section", data, false)
		if len(props) < 1 || !strings.Contains(props[0], ":10-20") {
			t.Errorf("section props should contain :10-20, got: %v", props)
		}
	})

	t.Run("definition props include name and line range", func(t *testing.T) {
		data := map[string]interface{}{
			"name": "myFunc", "startLine": float64(5), "endLine": float64(15),
		}
		props := collectEntityProps("definition", data, false)
		foundName := false
		foundRange := false
		for _, p := range props {
			if strings.Contains(p, "myFunc") {
				foundName = true
			}
			if strings.Contains(p, ":5-15") {
				foundRange = true
			}
		}
		if !foundName {
			t.Errorf("definition props should contain name: %v", props)
		}
		if !foundRange {
			t.Errorf("definition props should contain line range: %v", props)
		}
	})

	t.Run("props strip newlines from multi-line content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\nLine two.\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("prop contains newline: %q", p)
			}
			if strings.Contains(p, "\r") {
				t.Errorf("prop contains carriage return: %q", p)
			}
		}
	})

	t.Run("props strip backticks from content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed the `title` parameter in `UpdateTicketTitle`.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "`") {
				t.Errorf("prop contains backtick: %q", p)
			}
		}
	})

	t.Run("props collapse multiple spaces", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed.\n\n1. First.\n2. Second.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "  ") {
				t.Errorf("prop contains double space: %q", p)
			}
		}
	})

	t.Run("props handle Windows line endings", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\r\nLine two.\r\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\r") || strings.Contains(p, "\n") {
				t.Errorf("prop contains line break: %q", p)
			}
		}
	})

	t.Run("goal props strip newlines from description", func(t *testing.T) {
		data := map[string]interface{}{
			"id": "G1", "title": "My Goal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Goal with\nmultiple\nlines and `backticks`.",
		}
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("goal prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("goal prop contains backtick: %q", p)
			}
		}
	})

	t.Run("checkpoint props strip newlines from message", func(t *testing.T) {
		data := map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}
		props := collectEntityProps("checkpoint", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("checkpoint prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("checkpoint prop contains backtick: %q", p)
			}
		}
	})

	t.Run("policy props strip newlines", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "code-hygiene",
			"name":        "Code Hygiene",
			"description": "Clean code\npolicy with `rules`.",
		}
		props := collectEntityProps("policy", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("policy prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("policy prop contains backtick: %q", p)
			}
		}
	})
}

func TestSingleLineOutput(t *testing.T) {
	multiLineEntities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Added folder renaming.\n\n1. MCP ticketReopen handler: reads the `title` parameter.\n2. MCP ticketClose handler: reads `title`.\n3. Goals: added `UpdateGoalTitle()` helper.\n\nAlso fixed a test bug.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T2", "title": "Open Ticket", "status": "open",
			"started": "2025-01-01T00:00:00Z",
			"prompt":  "Fix the `config` module.\nIt has multiple issues:\n- Issue 1\n- Issue 2",
			"year":    float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"goal", TreeNodeGoal, map[string]interface{}{
			"id": "G1", "title": "Multi\nLine\nGoal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description with `code`\nand\r\nnewlines.",
		}},
		{"checkpoint", TreeNodeCheckpoint, map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "p1", "name": "Policy", "description": "Rule 1\nRule 2\n`Rule 3`",
		}},
		{"technology", TreeNodeTechnology, map[string]interface{}{
			"name": "proj1", "description": "Technology\nwith\nnewlines",
		}},
	}

	assertSingleLine := func(t *testing.T, label, output string) {
		t.Helper()
		lines := strings.Split(output, "\n")
		if len(lines) > 1 {
			t.Errorf("%s is multi-line (%d lines):\n%q", label, len(lines), output)
		}
		if strings.Contains(output, "\r") {
			t.Errorf("%s contains carriage return:\n%q", label, output)
		}
	}

	assertNoRawBackticks := func(t *testing.T, label, output string) {
		t.Helper()
		for _, p := range collectEntityProps("ticket", multiLineEntities[0].data, false) {
			if strings.Contains(p, "`") {
				t.Errorf("%s prop contains backtick: %q", label, p)
			}
		}
		_ = output
	}

	for _, tt := range multiLineEntities {
		t.Run(tt.kind+"_renderEntityMarkdownLink_single_line", func(t *testing.T) {
			output := renderEntityMarkdownLink(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdownLink("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityMarkdown_single_line", func(t *testing.T) {
			output := renderEntityMarkdown(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, false)
			assertSingleLine(t, "renderEntityHuman("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_tty_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, true)
			assertSingleLine(t, "renderEntityHuman_tty("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_props_no_backticks", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "`") {
					t.Errorf("prop contains backtick: %q", p)
				}
			}
			assertNoRawBackticks(t, tt.kind, "")
		})

		t.Run(tt.kind+"_monorepoTreeNodeMarkdown_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_monorepoTreeNodeText_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeText("+tt.kind+")", output)
		})
	}

	t.Run("goal_tree_with_multi_line_tickets_all_single_line", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent\nGoal", Status: "open",
			Tickets: []*TicketNode{
				{
					Slug: "T1", Title: "Ticket `One`", Status: "closed",
					Created:  "2025-01-01T00:00:00Z",
					Finished: "2025-01-02T00:00:00Z",
					Summary:  "Fixed things.\n\n1. First fix.\n2. Second fix with `code`.",
				},
				{
					Slug: "T2", Title: "Ticket Two", Status: "open",
					Created:     "2025-01-01T00:00:00Z",
					Description: "Please fix:\n- Item 1\n- Item 2",
				},
			},
			Children: []*GoalNode{{
				ID: "G2", Title: "Child Goal", Status: "open",
				Description: "Description\nwith\nnewlines.",
			}},
		}}
		for _, format := range []string{"md", "text"} {
			output := renderGoalTreeNodes(roots, format)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("goal tree (%s) line %d is empty (blank line in output)", format, i)
				}
			}
		}
	})

	t.Run("ticket_list_with_multi_line_summary_single_line", func(t *testing.T) {
		tickets := []interface{}{
			map[string]interface{}{
				"slug": "T1", "title": "Ticket", "status": "closed",
				"finished": "2025-01-02T00:00:00Z",
				"summary":  "Summary with\nnewlines and `backticks`.",
				"year":     float64(2025), "month": float64(1), "day": float64(1),
			},
		}
		for _, useMD := range []bool{true, false} {
			output := renderTicketList(tickets, false, useMD)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("ticket list (md=%v) line %d is empty", useMD, i)
				}
			}
		}
	})

	t.Run("formatMarkdownResult_ticket_list_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"tickets": []interface{}{
					map[string]interface{}{
						"slug": "T1", "title": "Ticket", "status": "closed",
						"finished": "2025-01-02T00:00:00Z",
						"summary":  "Line 1\nLine 2\n`code`",
						"year":     float64(2025), "month": float64(1), "day": float64(1),
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("ticket list", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult ticket list line %d is empty", i)
			}
		}
	})

	t.Run("formatMarkdownResult_goal_tree_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"goals": []interface{}{
					map[string]interface{}{
						"id": "G1", "title": "Goal", "status": "open",
						"dueDate": "2030-01-01", "createdAt": "2025-01-01T00:00:00Z",
						"description": "Desc\nwith\nnewlines",
					},
				},
				"tickets": []interface{}{
					map[string]interface{}{
						"id": "T1", "slug": "T1", "title": "Ticket", "status": "closed",
						"goal":    "G1",
						"date":    map[string]interface{}{"created": "2025-01-01T00:00:00Z", "finished": "2025-01-02T00:00:00Z"},
						"summary": "Summary\nwith `code`\nrefs.",
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("goal tree", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult goal tree line %d is empty", i)
			}
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
			output := renderEntityMarkdown(tt.kind, tt.data)
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
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := sb.String()
			if strings.HasPrefix(output, "- - ") {
				t.Errorf("renderTreeNodeMarkdown(%s) has double dash: %q", tt.kind, output)
			}
		})
	}

	t.Run("goalTreeNodes_no_double_dash", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Goal", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "T1", Title: "Ticket", Status: "open",
			}},
		}}
		output := renderGoalTreeNodes(roots, "md")
		for i, line := range strings.Split(output, "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if strings.HasPrefix(trimmed, "- - ") {
				t.Errorf("line %d has double dash: %q", i, line)
			}
		}
	})
}
