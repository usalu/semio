// 🔗️ Tests of the cli domain: MCP tool schemas and prompt rendering.

package cli

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// 🧪️#endregion 🧬️Consolidated
func TestMcpToolsSchemas(t *testing.T) {
	s := CreateMcpServer(providers.McpClientGeneric, DefaultCommandTimeout)
	tools := s.ListTools()
	allowedTools := []string{
		"ticket_open",
		"ticket_close",
		"ticket_reopen",
		"goal_open",
		"goal_close",
		"goal_reopen",
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

func TestRenderPromptTemplateUsesRepoMetaRoot(t *testing.T) {
	repoRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	templateDir := filepath.Join(repoRoot, ".🧬semio", "🦑️repo", "💬️prompts")
	if err := os.MkdirAll(templateDir, 0755); err != nil {
		t.Fatalf("mkdir template dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(templateDir, "enhance.tpl"), []byte("Prompt={{ .prompt }}"), 0644); err != nil {
		t.Fatalf("write template: %v", err)
	}
	otherDir := filepath.Join(repoRoot, "repo", "client")
	if err := os.MkdirAll(otherDir, 0755); err != nil {
		t.Fatalf("mkdir other dir: %v", err)
	}
	oldRoot := workspace.RootDir
	defer func() { workspace.SetRootDir(oldRoot) }()
	workspace.SetRootDir(repoRoot)
	oldWD, err := os.Getwd()
	if err != nil {
		t.Fatalf("getwd: %v", err)
	}
	defer func() { _ = os.Chdir(oldWD) }()
	if err := os.Chdir(otherDir); err != nil {
		t.Fatalf("chdir: %v", err)
	}
	out, err := renderPromptTemplate("enhance", map[string]string{"prompt": "hello"})
	if err != nil {
		t.Fatalf("renderPromptTemplate failed: %v", err)
	}
	if strings.TrimSpace(out) != "Prompt=hello" {
		t.Fatalf("unexpected rendered prompt: %q", out)
	}
}
