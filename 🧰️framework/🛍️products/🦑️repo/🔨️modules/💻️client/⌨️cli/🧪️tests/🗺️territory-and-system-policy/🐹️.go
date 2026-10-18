// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Territory, policy definitions, system policy, and policy groups.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

func TestTerritory(t *testing.T) {
	t.Run("AllKinds flat", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
			Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingSummary {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingSummary, kinds[1])
		}
	})
	t.Run("AllKinds nested groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
				{
					Name:        "Section",
					Description: "Section-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeSectionEmpty},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
	})
	t.Run("AllKinds mixed kinds and groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Kinds:       []Statute{BreachCodeCommentInline},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeCommentInline {
			t.Errorf("expected %s first, got %s", BreachCodeCommentInline, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s second, got %s", BreachCodeFileMissingHeaderRegion, kinds[1])
		}
	})
	t.Run("AllKinds deeply nested", func(t *testing.T) {
		g := Territory{
			Name:   "Root",
			Scopes: []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "Level1",
					Scopes: []string{"**/*"},
					Groups: []Territory{
						{
							Name:   "Level2",
							Scopes: []string{"**/*"},
							Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
						},
					},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 1 {
			t.Fatalf("expected 1 kind, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
	})
	t.Run("AllKinds empty group", func(t *testing.T) {
		g := Territory{
			Name:   "Empty",
			Scopes: []string{"**/*"},
		}
		kinds := g.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
	t.Run("GetID and GetURI", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
		}
		id := g.GetID()
		if id == "" {
			t.Error("expected non-empty ID")
		}
		if !strings.Contains(id, "File") {
			t.Errorf("expected ID to contain 'File', got %s", id)
		}
		uri := g.GetURI()
		if uri == "" {
			t.Error("expected non-empty URI")
		}
		if !strings.HasPrefix(uri, "repo://") {
			t.Errorf("expected URI to start with 'repo://', got %s", uri)
		}
	})
}

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestSystemPolicy(t *testing.T) {
	const settings = `{"editor.fontSize": 14, "python.testing.cwd": "${workspaceFolder}", "terminal.integrated.env.windows": {"A": "b"}}`
	const extensions = `{"recommendations": ["ms-python.python", "ms-vscode-remote.remote-containers"]}`
	const overlay = `{"settings": {"omit": ["terminal.integrated.env.windows"], "set": {"python.defaultInterpreterPath": "${containerWorkspaceFolder}/.venv/bin/python"}}, "extensions": {"omit": ["ms-vscode-remote.remote-containers"], "add": ["anthropic.claude-code"]}}`
	const devcontainer = "// SPDX-License-Identifier: AGPL-3.0-only\n{\n  /* derived */\n  \"name\": \"https://example.test/a\",\n  \"customizations\": {\"vscode\": {\"settings\": {\"editor.fontSize\": 14, \"python.testing.cwd\": \"${containerWorkspaceFolder}\", \"python.defaultInterpreterPath\": \"${containerWorkspaceFolder}/.venv/bin/python\"}, \"extensions\": [\"ms-python.python\", \"anthropic.claude-code\"]}}\n}\n"
	arrange := func(t *testing.T, files map[string]string) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		t.Cleanup(func() { rootDir = oldRoot })
		for name, content := range files {
			os.MkdirAll(filepath.Dir(filepath.Join(tmpDir, name)), 0o755)
			WriteTextFile(filepath.Join(tmpDir, name), content)
		}
	}
	kinds := func() map[Statute]bool {
		found := map[Statute]bool{}
		for _, v := range systemPolicy(NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})) {
			found[v.Kind] = true
		}
		return found
	}
	files := func(overrides map[string]string) map[string]string {
		base := map[string]string{".vscode/settings.json": settings, ".vscode/extensions.json": extensions, ".devcontainer/editor-overlay.json": overlay, ".devcontainer/devcontainer.json": devcontainer}
		for name, content := range overrides {
			base[name] = content
		}
		return base
	}
	t.Run("no breach when the devcontainer block equals its derivation", func(t *testing.T) {
		arrange(t, files(nil))
		if found := kinds(); len(found) != 0 {
			t.Errorf("expected no breach, got %v", found)
		}
	})
	t.Run("detects drifted settings", func(t *testing.T) {
		arrange(t, files(map[string]string{".vscode/settings.json": `{"editor.fontSize": 15}`}))
		found := kinds()
		if !found[BreachSystemDevcontainerVscodeSettingsDrift] || found[BreachSystemDevcontainerVscodeExtensionsDrift] {
			t.Errorf("expected only the settings drift breach, got %v", found)
		}
	})
	t.Run("detects drifted extensions", func(t *testing.T) {
		arrange(t, files(map[string]string{".vscode/extensions.json": `{"recommendations": ["ms-python.python", "golang.go"]}`}))
		found := kinds()
		if found[BreachSystemDevcontainerVscodeSettingsDrift] || !found[BreachSystemDevcontainerVscodeExtensionsDrift] {
			t.Errorf("expected only the extensions drift breach, got %v", found)
		}
	})
	t.Run("a missing overlay declares no overlay", func(t *testing.T) {
		arrange(t, files(map[string]string{".devcontainer/editor-overlay.json": ""}))
		found := kinds()
		if !found[BreachSystemDevcontainerVscodeSettingsDrift] || !found[BreachSystemDevcontainerVscodeExtensionsDrift] {
			t.Errorf("expected both drift breaches without overlay, got %v", found)
		}
	})
	t.Run("no breachs when the editor files are absent", func(t *testing.T) {
		arrange(t, nil)
		if found := kinds(); len(found) != 0 {
			t.Errorf("expected 0 breachs, got %v", found)
		}
	})
	t.Run("derivation equals the language-agnostic fixture", func(t *testing.T) {
		data, err := os.ReadFile("../../📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/🧫️fixtures/📝️derived-config/🔣️.json")
		if err != nil {
			t.Fatal(err)
		}
		var fixture struct {
			Editor []struct {
				Name      string
				Canonical EditorSurface
				Overlay   EditorOverlay
				Expected  EditorSurface
			}
		}
		if err := json.Unmarshal(data, &fixture); err != nil {
			t.Fatal(err)
		}
		if len(fixture.Editor) == 0 {
			t.Fatal("fixture holds no editor cases")
		}
		for _, row := range fixture.Editor {
			derived := DeriveContainerEditor(row.Canonical, row.Overlay)
			if !reflect.DeepEqual(derived.Settings, row.Expected.Settings) || !reflect.DeepEqual(derived.Extensions, row.Expected.Extensions) {
				t.Errorf("%s: derived %v, expected %v", row.Name, derived, row.Expected)
			}
		}
	})
	t.Run("policy registered with correct id", func(t *testing.T) {
		p, found := FindPolicy("system")
		if !found {
			t.Fatal("expected system policy to be registered")
		}
		if p.Name != "System" {
			t.Errorf("expected name System, got %s", p.Name)
		}
		kinds := p.AllKinds()
		if len(kinds) != 2 || kinds[0] != BreachSystemDevcontainerVscodeSettingsDrift || kinds[1] != BreachSystemDevcontainerVscodeExtensionsDrift {
			t.Fatalf("expected the settings and extensions drift statutes, got %v", kinds)
		}
	})
	t.Run("statute meta is correct", func(t *testing.T) {
		for _, kind := range []Statute{BreachSystemDevcontainerVscodeSettingsDrift, BreachSystemDevcontainerVscodeExtensionsDrift} {
			meta := kind.Info()
			if meta.Autofixable {
				t.Errorf("%s: a derived copy is rewritten by generate-config, never autofixed", kind)
			}
			if meta.Priority != BreachPriorityHigh {
				t.Errorf("%s: expected high priority", kind)
			}
		}
	})
}

func TestBuildTerritoryTree(t *testing.T) {
	t.Run("single group with kinds", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "File",
				Description: "File breachs",
				Scopes:      []string{"**/*.ts"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 node, got %d", len(nodes))
		}
		if nodes[0].Label != "File" {
			t.Errorf("expected label 'File', got %s", nodes[0].Label)
		}
		if nodes[0].Kind != TreeNodeCategory {
			t.Errorf("expected category kind, got %s", nodes[0].Kind)
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		for _, child := range nodes[0].Children {
			if child.Kind != TreeNodeStatute {
				t.Errorf("expected statute node, got %s", child.Kind)
			}
		}
	})
	t.Run("nested groups", func(t *testing.T) {
		groups := []Territory{
			{
				Name:   "Code",
				Scopes: []string{"**/*.ts"},
				Groups: []Territory{
					{
						Name:   "File",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
					},
					{
						Name:   "Section",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeSectionEmpty},
					},
				},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 root node, got %d", len(nodes))
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		fileGroup := nodes[0].Children[0]
		if fileGroup.Label != "File" {
			t.Errorf("expected label 'File', got %s", fileGroup.Label)
		}
		if len(fileGroup.Children) != 1 {
			t.Fatalf("expected 1 child in File group, got %d", len(fileGroup.Children))
		}
	})
	t.Run("empty groups", func(t *testing.T) {
		nodes := buildTerritoryTree(nil)
		if len(nodes) != 0 {
			t.Fatalf("expected 0 nodes, got %d", len(nodes))
		}
	})
	t.Run("group node data contains scopes", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "Sketchpad",
				Description: "Sketchpad breachs",
				Scopes:      []string{"js/sketchpad/**/*.ts", "js/sketchpad/**/*.tsx"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
			},
		}
		nodes := buildTerritoryTree(groups)
		data := nodes[0].Data
		if data == nil {
			t.Fatal("expected non-nil data")
		}
		scopes, ok := data["scopes"].([]string)
		if !ok {
			t.Fatal("expected scopes in data")
		}
		if len(scopes) != 2 {
			t.Fatalf("expected 2 scopes, got %d", len(scopes))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestExhaustivePolicyGroupsGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy groups graphql test in short mode")
	}
	executor := getTestExecutor(t)
	query := `{ policies { id name groups { name description scopes kinds { id } groups { name kinds { id } } } } }`
	result, err := executor.Execute(context.Background(), query, nil)
	if err != nil {
		t.Fatalf("GraphQL query failed: %v", err)
	}
	data, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected map result")
	}
	policiesData, ok := data["policies"].([]interface{})
	if !ok {
		t.Fatal("expected policies array")
	}
	if len(policiesData) == 0 {
		t.Fatal("expected at least one policy")
	}
	for _, pRaw := range policiesData {
		p, ok := pRaw.(map[string]interface{})
		if !ok {
			continue
		}
		groups, ok := p["groups"].([]interface{})
		if !ok {
			t.Fatalf("expected groups array for policy %v", p["id"])
		}
		if len(groups) == 0 {
			t.Errorf("expected at least one group for policy %v", p["id"])
		}
	}
}
