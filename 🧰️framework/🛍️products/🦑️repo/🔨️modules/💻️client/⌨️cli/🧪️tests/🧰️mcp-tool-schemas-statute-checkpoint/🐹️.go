// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP tool schemas, statute commands, and checkpoint commands.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
	"testing"
)

// 🧪️#endregion 🧬️Consolidated
func TestMcpToolsSchemas(t *testing.T) {
	s := CreateMcpServer(McpClientGeneric, DefaultCommandTimeout)
	tools := s.ListTools()
	allowedTools := []string{
		"ticket_open",
		"ticket_close",
		"ticket_reopen",
		"section_move",
		"section_extract",
		"file_integrate",
	}
	if len(tools) != len(allowedTools) {
		t.Fatalf("expected %d MCP tools, got %d", len(allowedTools), len(tools))
	}
	for _, allowed := range allowedTools {
		if _, exists := tools[allowed]; !exists {
			t.Fatalf("required MCP tool %q is not registered", allowed)
		}
	}

	var validateSchema func(path string, schema map[string]any) error
	validateSchema = func(path string, schema map[string]any) error {
		typeVal, ok := schema["type"].(string)

		if ok && typeVal == "array" {
			if _, hasItems := schema["items"]; !hasItems {
				return fmt.Errorf("property '%s' is of type 'array' but missing 'items' field", path)
			}
		}

		if props, ok := schema["properties"].(map[string]any); ok {
			for k, v := range props {
				if propMap, ok := v.(map[string]any); ok {
					if err := validateSchema(path+"."+k, propMap); err != nil {
						return err
					}
				}
			}
		}

		if items, ok := schema["items"].(map[string]any); ok {
			if err := validateSchema(path+".items", items); err != nil {
				return err
			}
		}

		return nil
	}

	for name, tool := range tools {
		t.Run(name, func(t *testing.T) {

			for propName, propSchema := range tool.Tool.InputSchema.Properties {
				if propMap, ok := propSchema.(map[string]any); ok {
					if err := validateSchema(propName, propMap); err != nil {
						t.Errorf("Invalid schema for tool '%s': %v", name, err)
					}
				}
			}
		})
	}
}

func TestExhaustiveStatuteCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statute commands test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("statute list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--json")
		if err != nil {
			t.Fatalf("statute list failed: %v", err)
		}
		if !strings.Contains(output, "statute") {
			t.Errorf("expected statute JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) < 5 {
			t.Errorf("expected multiple statutes, got %d lines", len(lines))
		}
	})

	t.Run("statute tree returns results", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--text")
		if err != nil {
			t.Fatalf("statute tree failed: %v", err)
		}
		if !strings.Contains(output, "header") {
			t.Errorf("expected statute tree categories in output")
		}
	})

	t.Run("statute list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("statute tree markdown", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute tree md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})
}

func TestExhaustiveCheckpointCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow checkpoint commands test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("checkpoint list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		if !strings.Contains(output, "checkpoint") {
			t.Errorf("expected checkpoint JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) == 0 {
			t.Error("expected at least one checkpoint")
		}
	})

	t.Run("checkpoint list --query filters", func(t *testing.T) {
		allOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		allLines := strings.Split(strings.TrimSpace(allOutput), "\n")

		filteredOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200", "--query", "merge")
		if err != nil {
			t.Fatalf("checkpoint list --query failed: %v", err)
		}
		filteredLines := strings.Split(strings.TrimSpace(filteredOutput), "\n")
		if len(filteredLines) >= len(allLines) && len(allLines) > 1 {
			t.Errorf("expected --query to reduce results: all=%d filtered=%d", len(allLines), len(filteredLines))
		}
	})

	t.Run("checkpoint list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--md", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("checkpoint list text", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--text", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list text failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty text output")
		}
	})
}
