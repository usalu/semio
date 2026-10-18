// 🪨️ Tests of the entity property collection and renderings.

package model

import (
	strings "strings"
	testing "testing"
)

func TestCollectEntityProps_MultilineEscaped(t *testing.T) {
	tests := []struct {
		name string
		kind string
		data map[string]interface{}
	}{
		{"ticket summary with newlines", "ticket", map[string]interface{}{
			"slug": "T1", "status": "closed", "title": "Fix Bug",
			"year": float64(2026), "month": float64(1), "day": float64(1),
			"summary": "Fixed three areas:\n\n1. First fix\n2. Second fix\n3. Third fix",
		}},
		{"ticket prompt with newlines", "ticket", map[string]interface{}{
			"slug": "T2", "status": "open", "title": "Add Feature",
			"year": float64(2026), "month": float64(1), "day": float64(2),
			"prompt": "Please implement:\n- item A\n- item B",
		}},
		{"goal description with newlines", "goal", map[string]interface{}{
			"id": "GOAL1", "title": "Goal One", "status": "open",
			"description": "Line one\nLine two\r\nLine three",
		}},
		{"policy description with newlines", "policy", map[string]interface{}{
			"id": "P1", "description": "Rule one\nRule two",
		}},
		{"checkpoint message with newlines", "checkpoint", map[string]interface{}{
			"id": "abc123", "message": "feat: add feature\n\nDetailed description here",
		}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			props := CollectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "\n") || strings.Contains(p, "\r") {
					t.Errorf("property contains newline: %q", p)
				}
			}
		})
	}
}

func TestRenderEntityMarkdownLink_AllKinds(t *testing.T) {
	entities := []struct {
		kind string
		data map[string]interface{}
	}{
		{"ticket", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goal", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path",
		}},
		{"folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"file", map[string]interface{}{
			"id": "file.ts",
		}},
		{"section", map[string]interface{}{
			"name": "Sec", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"definition", map[string]interface{}{
			"name": "def", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"contributor", map[string]interface{}{
			"github": "octocat", "name": "Cat",
		}},
		{"todo", map[string]interface{}{
			"name": "Todo",
		}},
		{"draft", map[string]interface{}{
			"id": "draft1",
		}},
		{"policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"statute", map[string]interface{}{
			"id": "vk1", "description": "Desc",
		}},
		{"technology", map[string]interface{}{
			"id": "proj", "description": "Desc",
		}},
		{"checkpoint", map[string]interface{}{
			"sha": "abc123", "message": "msg",
		}},
		{"root", map[string]interface{}{
			"name": "myrepo",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind, func(t *testing.T) {
			output := RenderEntityMarkdownLink(tt.kind, tt.data)
			if !strings.HasPrefix(output, "[") {
				t.Errorf("link for %s missing '[' prefix: %s", tt.kind, output)
			}
			if !strings.Contains(output, "](") {
				t.Errorf("link for %s missing '](': %s", tt.kind, output)
			}
			if !strings.Contains(output, "repo://") {
				t.Errorf("link for %s missing 'repo://' uri: %s", tt.kind, output)
			}
			if strings.Contains(output, "```") {
				t.Errorf("link for %s contains code fence: %s", tt.kind, output)
			}
		})
	}
}

func TestInferEntityKind(t *testing.T) {
	cases := []struct {
		key      string
		expected string
	}{
		{"ticketOpen", "ticket"},
		{"ticketClose", "ticket"},
		{"ticketReopen", "ticket"},
		{"ticketChange", "ticket"},
		{"goalCreate", "goal"},
		{"goalClose", "goal"},
		{"goalReopen", "goal"},
		{"goalChange", "goal"},
		{"folderCreate", "folder"},
		{"folderDelete", "folder"},
		{"folderMove", "folder"},
		{"fileCreate", "file"},
		{"fileDelete", "file"},
		{"fileMove", "file"},
		{"sectionCreate", "section"},
		{"sectionDelete", "section"},
		{"sectionMove", "section"},
		{"definitionList", "definition"},
		{"contributorRemove", "contributor"},
		{"todoCreate", "todo"},
		{"todoChange", "todo"},
		{"todoDelete", "todo"},
		{"syncManagement", "root"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "root"},
		{"unknownKey", ""},
	}

	for _, tt := range cases {
		t.Run(tt.key, func(t *testing.T) {
			got := InferEntityKind(tt.key)
			if got != tt.expected {
				t.Errorf("inferEntityKind(%q) = %q, want %q", tt.key, got, tt.expected)
			}
		})
	}
}

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
		props := CollectEntityProps("goal", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("section", data, false)
		if len(props) < 1 || !strings.Contains(props[0], ":10-20") {
			t.Errorf("section props should contain :10-20, got: %v", props)
		}
	})

	t.Run("definition props include name and line range", func(t *testing.T) {
		data := map[string]interface{}{
			"name": "myFunc", "startLine": float64(5), "endLine": float64(15),
		}
		props := CollectEntityProps("definition", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("goal", data, false)
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
		props := CollectEntityProps("checkpoint", data, false)
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
		props := CollectEntityProps("policy", data, false)
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
