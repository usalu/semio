package statutes

import (
	bytes "bytes"
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗂️Folder And File Policy Tests

func TestFolderPolicyEmptyFolder(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "some", "empty")
	os.MkdirAll(emptyDir, 0755)
	nonEmptyDir := filepath.Join(tmpDir, "some", "nonempty")
	os.MkdirAll(nonEmptyDir, 0755)
	os.WriteFile(filepath.Join(nonEmptyDir, "file.txt"), []byte("content"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	foundEmpty := false
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty && v.Excerpt == "some/empty" {
			foundEmpty = true
			if !v.Autofixable() {
				t.Error("BreachFolderIllegalEmpty should be autofixable")
			}
		}
	}
	if !foundEmpty {
		t.Error("expected BreachFolderIllegalEmpty for some/empty")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty && v.Excerpt == "some/nonempty" {
			t.Error("should not report BreachFolderIllegalEmpty for non-empty folder")
		}
	}
}

func TestPathEmojiStatutesLanguageNeutralFixture(t *testing.T) {
	fixturePath := filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "🧫️fixtures", "🔏️path-emoji-statutes", "🔣️.json")
	data, err := os.ReadFile(fixturePath)
	if err != nil {
		t.Fatalf("read shared path emoji fixture: %v", err)
	}
	var fixture struct {
		GenericEmojiIdentities []string `json:"genericEmojiIdentities"`
		Cases                  []struct {
			ID       string             `json:"id"`
			Entries  []pathEmojiEntry   `json:"entries"`
			Expected []pathEmojiFinding `json:"expected"`
		} `json:"cases"`
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatalf("parse shared path emoji fixture: %v", err)
	}
	for _, scenario := range fixture.Cases {
		observed := pathEmojiStatuteFindings(scenario.Entries, fixture.GenericEmojiIdentities)
		observedJSON, _ := json.Marshal(observed)
		expectedJSON, _ := json.Marshal(scenario.Expected)
		if !bytes.Equal(observedJSON, expectedJSON) {
			t.Errorf("%s: observed %s, expected %s", scenario.ID, observedJSON, expectedJSON)
		}
	}
}

func TestFolderPolicySkipsExcludedDirs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	for _, dir := range []string{".git/objects", ".🦑️repo/cache", "node_modules/.cache"} {
		os.MkdirAll(filepath.Join(tmpDir, dir), 0755)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty {
			if strings.HasPrefix(v.Excerpt, ".git") || strings.HasPrefix(v.Excerpt, ".🧬semio") || strings.HasPrefix(v.Excerpt, "node_modules") {
				t.Errorf("should skip excluded dir, got breach for %s", v.Excerpt)
			}
		}
	}
}

func TestFilePolicyGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/main.ts"]`
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(srcDir, 0755)
	os.WriteFile(filepath.Join(srcDir, "main.ts"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/main.ts") {
			t.Errorf("should not report breach for allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSupportsGlobPatterns(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/**/*.ts", "docs/*.md"]`
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcNestedDir := filepath.Join(tmpDir, "src", "nested")
	os.MkdirAll(srcNestedDir, 0755)
	os.WriteFile(filepath.Join(srcNestedDir, "main.ts"), []byte("ok"), 0644)
	docsDir := filepath.Join(tmpDir, "docs")
	os.MkdirAll(docsDir, 0755)
	os.WriteFile(filepath.Join(docsDir, "guide.md"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/nested/main.ts" || v.Excerpt == "docs/guide.md") {
			t.Errorf("should not report breach for glob-allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsComposeRepo(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(`[]`), 0644)
	os.WriteFile(filepath.Join(metaDir, "some_internal.json"), []byte("internal"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && strings.HasPrefix(v.Excerpt, ".🧬semio") {
			t.Errorf("should skip .🦑️repo files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsNestedNodeModules(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.SetRootDir(tmpDir)
	defer func() { workspace.SetRootDir(oldRoot) }()
	os.WriteFile(filepath.Join(tmpDir, ".gitignore"), []byte("node_modules/\n"), 0644)
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(`[]`), 0644)
	nested := filepath.Join(tmpDir, "repo", "vscode", "node_modules", "undici-types")
	os.MkdirAll(nested, 0755)
	os.WriteFile(filepath.Join(nested, "fetch.d.ts"), []byte("export {};"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && strings.Contains(v.Excerpt, "node_modules/") {
			t.Errorf("should skip ignored node_modules files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyNoGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("content"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	if len(breachs) != 0 {
		t.Errorf("expected no breachs when godfile is missing, got %d", len(breachs))
	}
}

// #endregion 🗂️Folder And File Policy Tests
