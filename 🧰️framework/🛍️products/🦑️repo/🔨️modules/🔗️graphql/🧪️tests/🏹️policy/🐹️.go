// 🛡️ Tests of the breach cache analysis and the system policy.

package graphql

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	reflect "reflect"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestAnalyzeReadsBreachCacheJSON(t *testing.T) {
	tmp := t.TempDir()
	cacheDir := filepath.Join(tmp, ".🧬semio", "🦑️repo", "⚡️cache", "breaches")
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		t.Fatal(err)
	}
	cachePath := filepath.Join(cacheDir, "unit-test.json")
	payload := `{
  "entityId": "test",
  "script": "unit.test.lint.script.ts",
  "breachs": [
    {
      "id": "e1",
      "summary": "hello",
      "kind": "lint/test/rule",
      "scope": "repo/example.go",
      "priority": "medium"
    }
  ]
}`
	if err := os.WriteFile(cachePath, []byte(payload), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := workspace.RootDir
	workspace.RootDir = tmp
	defer func() { workspace.RootDir = oldRoot }()
	ctx := NewRepoContext(tmp)
	ar, err := ctx.Analyze(nil)
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if ar == nil || len(ar.Breachs) != 1 {
		t.Fatalf("expected 1 breach, got %#v", ar)
	}
	if ar.Breachs[0].Summary != "hello" {
		t.Fatalf("unexpected breach: %+v", ar.Breachs[0])
	}
}

func TestSystemPolicy(t *testing.T) {
	const settings = `{"editor.fontSize": 14, "python.testing.cwd": "${workspaceFolder}", "terminal.integrated.env.windows": {"A": "b"}}`
	const extensions = `{"recommendations": ["ms-python.python", "ms-vscode-remote.remote-containers"]}`
	const overlay = `{"settings": {"omit": ["terminal.integrated.env.windows"], "set": {"python.defaultInterpreterPath": "${containerWorkspaceFolder}/.venv/bin/python"}}, "extensions": {"omit": ["ms-vscode-remote.remote-containers"], "add": ["anthropic.claude-code"]}}`
	const devcontainer = "// SPDX-License-Identifier: AGPL-3.0-only\n{\n  /* derived */\n  \"name\": \"https://example.test/a\",\n  \"customizations\": {\"vscode\": {\"settings\": {\"editor.fontSize\": 14, \"python.testing.cwd\": \"${containerWorkspaceFolder}\", \"python.defaultInterpreterPath\": \"${containerWorkspaceFolder}/.venv/bin/python\"}, \"extensions\": [\"ms-python.python\", \"anthropic.claude-code\"]}}\n}\n"
	arrange := func(t *testing.T, files map[string]string) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		t.Cleanup(func() { workspace.RootDir = oldRoot })
		for name, content := range files {
			os.MkdirAll(filepath.Dir(filepath.Join(tmpDir, name)), 0o755)
			workspace.WriteTextFile(filepath.Join(tmpDir, name), content)
		}
	}
	kinds := func() map[model.Statute]bool {
		found := map[model.Statute]bool{}
		for _, v := range statutes.SystemPolicy(statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})) {
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
		if !found[model.BreachSystemDevcontainerVscodeSettingsDrift] || found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
			t.Errorf("expected only the settings drift breach, got %v", found)
		}
	})
	t.Run("detects drifted extensions", func(t *testing.T) {
		arrange(t, files(map[string]string{".vscode/extensions.json": `{"recommendations": ["ms-python.python", "golang.go"]}`}))
		found := kinds()
		if found[model.BreachSystemDevcontainerVscodeSettingsDrift] || !found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
			t.Errorf("expected only the extensions drift breach, got %v", found)
		}
	})
	t.Run("a missing overlay declares no overlay", func(t *testing.T) {
		arrange(t, files(map[string]string{".devcontainer/editor-overlay.json": ""}))
		found := kinds()
		if !found[model.BreachSystemDevcontainerVscodeSettingsDrift] || !found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
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
		data, err := os.ReadFile("../../../📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/🧫️fixtures/📝️derived-config/🔣️.json")
		if err != nil {
			t.Fatal(err)
		}
		var fixture struct {
			Editor []struct {
				Name      string
				Canonical statutes.EditorSurface
				Overlay   statutes.EditorOverlay
				Expected  statutes.EditorSurface
			}
		}
		if err := json.Unmarshal(data, &fixture); err != nil {
			t.Fatal(err)
		}
		if len(fixture.Editor) == 0 {
			t.Fatal("fixture holds no editor cases")
		}
		for _, row := range fixture.Editor {
			derived := statutes.DeriveContainerEditor(row.Canonical, row.Overlay)
			if !reflect.DeepEqual(derived.Settings, row.Expected.Settings) || !reflect.DeepEqual(derived.Extensions, row.Expected.Extensions) {
				t.Errorf("%s: derived %v, expected %v", row.Name, derived, row.Expected)
			}
		}
	})
	t.Run("the checked-in devcontainer block equals its derivation", func(t *testing.T) {
		root, err := filepath.Abs("../../../../../../..")
		if err != nil {
			t.Fatal(err)
		}
		if _, err := os.Stat(filepath.Join(root, ".devcontainer", "editor-overlay.json")); err != nil {
			t.Skipf("not running inside the repository checkout: %v", err)
		}
		oldRoot := workspace.RootDir
		workspace.RootDir = root
		t.Cleanup(func() { workspace.RootDir = oldRoot })
		if found := kinds(); len(found) != 0 {
			t.Errorf("run bun repo run workspace:generate-config; the checked-in block drifts: %v", found)
		}
	})
	t.Run("policy registered with correct id", func(t *testing.T) {
		p, found := statutes.FindPolicy("system")
		if !found {
			t.Fatal("expected system policy to be registered")
		}
		if p.Name != "System" {
			t.Errorf("expected name System, got %s", p.Name)
		}
		kinds := p.AllKinds()
		if len(kinds) != 2 || kinds[0] != model.BreachSystemDevcontainerVscodeSettingsDrift || kinds[1] != model.BreachSystemDevcontainerVscodeExtensionsDrift {
			t.Fatalf("expected the settings and extensions drift statutes, got %v", kinds)
		}
	})
	t.Run("statute meta is correct", func(t *testing.T) {
		for _, kind := range []model.Statute{model.BreachSystemDevcontainerVscodeSettingsDrift, model.BreachSystemDevcontainerVscodeExtensionsDrift} {
			meta := kind.Info()
			if meta.Autofixable {
				t.Errorf("%s: a derived copy is rewritten by generate-config, never autofixed", kind)
			}
			if meta.Priority != model.BreachPriorityHigh {
				t.Errorf("%s: expected high priority", kind)
			}
		}
	})
}
