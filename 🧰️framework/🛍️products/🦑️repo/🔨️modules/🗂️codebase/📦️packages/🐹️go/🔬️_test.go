// 🔬️ Tests of the codebase domain, split out of the pre-split godfile suite.

package codebase

import (
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

func findTestRepoRoot(start string) string {
	for _, candidate := range []string{start, func() string {
		_, file, _, ok := runtime.Caller(0)
		if !ok {
			return ""
		}
		return filepath.Dir(file)
	}()} {
		dir := candidate
		if dir == "" {
			continue
		}
		for {
			if _, err := os.Stat(filepath.Join(dir, "repo", "client", "main.go")); err == nil {
				return dir
			}
			if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
				return dir
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break
			}
			dir = parent
		}
	}
	return start
}

func TestFileHeaderId(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"config json", "tsconfig.json", model.EmojiText(model.EmojiFileConfig) + "tsconfig"},
		{"docs md", "README.md", model.EmojiText(model.EmojiFileDocs) + "readme"},
		{"script sh", "build.sh", model.EmojiText(model.EmojiFileScript) + "build"},
		{"script bash", "deploy.bash", model.EmojiText(model.EmojiFileScript) + "deploy"},
		{"script ps1", "setup.ps1", model.EmojiText(model.EmojiFileScript) + "setup"},
		{"resource png", "🖼️logo.png", model.EmojiText(model.EmojiFileResource) + workspace.Flat("🖼️logo")},
		{"license", "LICENSE.md", model.EmojiText(model.EmojiFileLicense) + "license"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := FileHeaderId(tt.path)
			if got != tt.want {
				t.Errorf("FileHeaderId(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}

	t.Run("shebang ts file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		filePath := "tools/build.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env tsx\nconsole.log('build');\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := model.EmojiText(model.EmojiFolderOrg) + "tools" + model.EmojiText(model.EmojiFileScript) + workspace.Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("shebang py file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		filePath := "scripts/run.py"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env python3\nprint('hello')\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := model.EmojiText(model.EmojiFolderOrg) + "scripts" + model.EmojiText(model.EmojiFileScript) + workspace.Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("code ts without shebang stays code", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		filePath := "src/index.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("export const x = 1;\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := model.EmojiText(model.EmojiFolderOrg) + "src" + model.EmojiText(model.EmojiFileCode) + workspace.Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) without shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("nonexistent code file stays code", func(t *testing.T) {
		got := FileHeaderId("nonexistent/file.ts")
		want := model.EmojiText(model.EmojiFolderOrg) + "nonexistent" + model.EmojiText(model.EmojiFileCode) + workspace.Flat("file")
		if got != want {
			t.Errorf("FileHeaderId for nonexistent file = %q, want %q", got, want)
		}
	})
}

func TestFixNonAutofixableNotFixed(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := workspace.RootDir
	workspace.RootDir = findTestRepoRoot(cwd)
	defer func() { workspace.RootDir = oldRoot }()

	bundles := LoadBundles()
	path := "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx"
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: path}
	ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	for _, v := range breachs {
		info := v.Kind.Info()
		if v.Autofixable() != info.Autofixable {
			t.Errorf("breach %s: Autofixable() = %v, Info().Autofixable = %v", v.Kind, v.Autofixable(), info.Autofixable)
		}
	}

	autofixableKinds := []model.Statute{
		model.BreachCodeFileWrongLicense,
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	for _, kind := range autofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected autofixable statute %s to be detected", kind)
		}
		if !kind.Info().Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
	}
	nonAutofixableKinds := []model.Statute{
		model.BreachCodeFileMissingContributors,
		model.BreachCodeSectionMissingStartName,
		model.BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected non-autofixable statute %s to be detected", kind)
		}
		if kind.Info().Autofixable {
			t.Errorf("statute %s should not be autofixable", kind)
		}
	}
}

func TestFixtureBreachsGroupedInline(t *testing.T) {
	path := "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx"
	bundles := LoadBundles()
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: path}
	ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("fixture policy check failed: %v", err)
	}
	if len(breachs) == 0 {
		t.Fatal("expected fixture breachs")
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	required := []model.Statute{
		model.BreachCodeSectionMissingSummary,
		model.BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range required {
		if counts[kind] == 0 {
			t.Fatalf("expected statute %s", kind)
		}
	}
}

func TestFixtureBreachsByLanguage(t *testing.T) {
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []model.Statute
	}{
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
			requiredKinds: []model.Statute{model.BreachCodeDefMissingSummary},
		},
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
			requiredKinds: []model.Statute{model.BreachCodeSectionMissingSummary},
		},
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
			requiredKinds: []model.Statute{model.BreachCodeSectionMissingSummary},
		},
	}
	for _, fixture := range fixtures {
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: fixture.path}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{fixture.path})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", fixture.path, err)
		}
		if len(breachs) == 0 {
			t.Fatalf("expected fixture breachs for %s", fixture.path)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		for _, kind := range fixture.requiredKinds {
			if counts[kind] == 0 {
				t.Fatalf("expected statute %s in %s", kind, fixture.path)
			}
		}
	}
	clean := []string{
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐹️.go",
	}
	for _, path := range clean {
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: path}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{path})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", path, err)
		}
		if len(breachs) != 0 {
			for _, v := range breachs {
				t.Logf("[DEBUG] breach in %s: kind=%s scope=%s line=%d summary=%s", path, v.Kind, v.Scope, v.Line, v.Summary)
			}
			t.Fatalf("expected no breachs for %s, got %d", path, len(breachs))
		}
	}
}
