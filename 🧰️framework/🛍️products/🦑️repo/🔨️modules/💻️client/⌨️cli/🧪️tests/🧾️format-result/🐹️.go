// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// JSON and mutation result formatting.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"strings"
	"testing"
)

// 📑️#region 🧬️Consolidated
func TestFormatResult_Section(t *testing.T) {
	payload := map[string]interface{}{
		"section": map[string]interface{}{
			"name":      "MySection",
			"filePath":  "path/to/file.ts",
			"startLine": float64(10),
			"endLine":   float64(20),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("section list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mysection",
		":10-20",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Definition(t *testing.T) {
	payload := map[string]interface{}{
		"definition": map[string]interface{}{
			"name":      "MyDefinition",
			"kind":      "function",
			"filePath":  "path/to/file.ts",
			"startLine": float64(30),
			"endLine":   float64(40),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("definition list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mydefinition",
		"MyDefinition",
		":30-40",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Bundle(t *testing.T) {
	payload := map[string]interface{}{
		"bundle": map[string]interface{}{
			"name": "MyBundle",
			"root": "/path/to/bundle",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("bundle list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mybundle",
		"/path/to/bundle",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Folder(t *testing.T) {
	payload := map[string]interface{}{
		"folder": map[string]interface{}{
			"path": "path/to/folder",
			"kind": "custom",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("folder list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"folder",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_File(t *testing.T) {
	payload := map[string]interface{}{
		"file": map[string]interface{}{
			"id": "path/to/file.ts",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("file list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"file",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Additional(t *testing.T) {
	t.Run("Goal", func(t *testing.T) {
		payload := map[string]interface{}{
			"goal": map[string]interface{}{
				"id":          "SKETCHPAD/MVP",
				"title":       "Sketchpad MVP",
				"status":      "open",
				"description": "Get sketchpad running at MVP level",
				"dates": map[string]interface{}{
					"due": "2026-02-15",
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)
		var p map[string]interface{}
		json.Unmarshal(jsonBytes, &p)

		output := formatResult("goal list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"goal\":") {
			t.Errorf("expected formatted goal, got: %s", output)
		}
		if !strings.Contains(output, "Sketchpad MVP") {
			t.Error("output missing title")
		}
		if strings.Contains(output, "2026-02-15") {
			t.Error("output should not contain absolute due date")
		}
		if !strings.Contains(output, "in ") && !strings.Contains(output, "from now") && !strings.Contains(output, "ago") {
			t.Error("output missing relative due date")
		}
		if !strings.Contains(strings.ToLower(output), "mvp") {
			t.Error("output missing id/slug")
		}
	})

	t.Run("Contributor", func(t *testing.T) {
		payload := map[string]interface{}{
			"contributor": map[string]interface{}{
				"github": "octocat",
				"name":   "The Octocat",
				"contributions": map[string]interface{}{
					"checkpoints": 10,
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("contributor list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"contributor\":") {
			t.Errorf("expected formatted contributor, got: %s", output)
		}
		if !strings.Contains(output, "octocat") {
			t.Error("output missing github handle")
		}
		if !strings.Contains(output, "The Octocat") {
			t.Error("output missing name")
		}
	})

	t.Run("Policy", func(t *testing.T) {
		payload := map[string]interface{}{
			"policy": map[string]interface{}{
				"id":          "code",
				"description": "Validates source file headers",
				"kinds":       []interface{}{"code/header"},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("policy list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"policy\":") {
			t.Errorf("expected formatted policy, got: %s", output)
		}
		if !strings.Contains(output, "code") {
			t.Error("output missing id")
		}
		if !strings.Contains(output, "Validates") {
			t.Error("output missing description")
		}
	})

	t.Run("File", func(t *testing.T) {
		payload := map[string]interface{}{
			"file": map[string]interface{}{
				"id":        "path/to/file.md",
				"extension": ".md",
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("file list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"file\":") {
			t.Errorf("expected formatted file, got: %s", output)
		}
		if !strings.Contains(output, "file") {
			t.Error("output missing path")
		}
	})

	t.Run("Fix", func(t *testing.T) {
		payload := map[string]interface{}{
			"fix": map[string]interface{}{
				"fixed":     5,
				"remaining": 2,
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("fix", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"fix\":") {
			t.Errorf("expected formatted fix, got: %s", output)
		}
		if !strings.Contains(output, "fixed 5 breachs") {
			t.Error("output missing fixed count")
		}
	})
}

func TestFormatResult_MutationKeys(t *testing.T) {
	mutations := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticketOpen", "ticketOpen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketClose", "ticketClose", map[string]interface{}{
			"slug": "MY-TICKET", "status": "closed", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketReopen", "ticketReopen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goalCreate", "goalCreate", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalClose", "goalClose", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "closed",
		}},
		{"goalReopen", "goalReopen", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"folderCreate", "folderCreate", map[string]interface{}{
			"path": "new/folder", "kind": "custom",
		}},
		{"folderDelete", "folderDelete", map[string]interface{}{
			"path": "old/folder", "kind": "custom",
		}},
		{"fileCreate", "fileCreate", map[string]interface{}{
			"id": "new/file.ts",
		}},
		{"fileDelete", "fileDelete", map[string]interface{}{
			"id": "old/file.ts",
		}},
		{"sectionCreate", "sectionCreate", map[string]interface{}{
			"name": "NewSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(10),
		}},
		{"contributorRemove", "contributorRemove", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
	}

	for _, tt := range mutations {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatResult(tt.name, json.RawMessage(jsonBytes), false)
			trimmed := strings.TrimSpace(output)
			if trimmed == "" {
				t.Errorf("output is empty for %s", tt.name)
			}
			if trimmed[0] == '{' {
				t.Errorf("output is raw JSON for %s: %s", tt.name, output)
			}
			if strings.Contains(output, "\""+tt.key+"\"") {
				t.Errorf("output contains raw key %q for %s: %s", tt.key, tt.name, output)
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
			output := renderEntityMarkdownLink(tt.kind, tt.data)
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
			got := inferEntityKind(tt.key)
			if got != tt.expected {
				t.Errorf("inferEntityKind(%q) = %q, want %q", tt.key, got, tt.expected)
			}
		})
	}
}
