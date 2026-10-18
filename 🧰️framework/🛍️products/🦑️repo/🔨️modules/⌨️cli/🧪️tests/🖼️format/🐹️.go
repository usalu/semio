// 🖼️ Tests of the cli domain: result formatting.

package cli

import (
	json "encoding/json"
	strings "strings"
	testing "testing"
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

func assertValidMarkdownLink(t *testing.T, output string) {
	t.Helper()
	trimmed := strings.TrimSpace(output)
	if trimmed == "" {
		t.Error("output is empty")
		return
	}
	for _, line := range strings.Split(trimmed, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		stripped := strings.TrimLeft(line, " ")
		if strings.HasPrefix(stripped, "- ") {
			stripped = strings.TrimPrefix(stripped, "- ")
		}
		if strings.HasPrefix(stripped, "**") {
			continue
		}
		if !strings.Contains(stripped, "[") || !strings.Contains(stripped, "](") {
			t.Errorf("line missing markdown link syntax [...](...): %q", line)
		}
		if strings.Contains(line, "```json") || strings.Contains(line, "```\n") {
			t.Errorf("output contains JSON code block: %q", line)
		}
	}
}
