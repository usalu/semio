// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo meta dir, root dir canonicalization, cache dir, and compose policy registration.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestRepoMetaDirUsesSemioRoot(t *testing.T) {
	root := t.TempDir()
	previous := GetRootDir()
	SetRootDir(root)
	t.Cleanup(func() { SetRootDir(previous) })

	got := GetRepoMetaDir()
	want := filepath.Join(root, ".🧬semio", "🦑️repo")
	if got != want {
		t.Fatalf("GetRepoMetaDir() = %q, want %q", got, want)
	}
}

func TestSetRootDirResetsGitignoreCache(t *testing.T) {
	tmpDirA := t.TempDir()
	tmpDirB := t.TempDir()
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(tmpDirA)
	_ = isGitIgnored("any.txt")
	os.WriteFile(filepath.Join(tmpDirB, ".gitignore"), []byte("node_modules/\n"), 0644)
	SetRootDir(tmpDirB)
	ignored := isGitIgnored(filepath.Join(tmpDirB, "node_modules", "x.ts"))
	if !ignored {
		t.Error("expected SetRootDir to refresh gitignore cache for new root")
	}
}

// 🙈️TestBuildBinaryArtifactsGitIgnored proves GetGitIgnoredSet answers from the ignore rules of the bound root, using a fixture ignore file. The checked-in root ignore rules are held by the checked-in asset contracts law.
func TestBuildBinaryArtifactsGitIgnored(t *testing.T) {
	root := withTempRepo(t)
	rules := "*.exe\n.🧬semio/🦑️repo/⚡️cache/\nserver\n"
	if err := os.WriteFile(filepath.Join(root, ".gitignore"), []byte(rules), 0o644); err != nil {
		t.Fatal(err)
	}
	SetRootDir(root)
	artifacts := []string{
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo",
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/repo.exe",
		"modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server",
		"modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server.exe",
		"coda/example/.coda/validators/programming.exe",
	}
	ignored := GetGitIgnoredSet(append(artifacts, "src/kept.go"))
	for _, path := range artifacts {
		if !ignored[path] {
			t.Errorf("expected build artifact %q to be gitignored", path)
		}
	}
	if ignored["src/kept.go"] {
		t.Error("src/kept.go must not be gitignored")
	}
}

func TestSetRootDirCanonicalizesToRepoRoot(t *testing.T) {
	repoRoot := t.TempDir()
	nested := filepath.Join(repoRoot, "a", "b", "c")
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(nested, 0755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(nested)
	if got := GetRootDir(); got != repoRoot {
		t.Fatalf("expected repo root %q, got %q", repoRoot, got)
	}
	if got := GetRepoMetaDir(); got != filepath.Join(repoRoot, ".🧬semio", "🦑️repo") {
		t.Fatalf("expected repo meta dir at monorepo root, got %q", got)
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
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(repoRoot)
	t.Chdir(otherDir)
	out, err := renderPromptTemplate("enhance", map[string]string{"prompt": "hello"})
	if err != nil {
		t.Fatalf("renderPromptTemplate failed: %v", err)
	}
	if strings.TrimSpace(out) != "Prompt=hello" {
		t.Fatalf("unexpected rendered prompt: %q", out)
	}
}

func TestFindRepoRootPrefersMonorepoMarkerOverLocalGoMod(t *testing.T) {
	monoRoot := t.TempDir()
	clientDir := filepath.Join(monoRoot, "repo", "cli")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	got := findRepoRoot(clientDir)
	if got != monoRoot {
		t.Fatalf("expected monorepo root %q, got %q", monoRoot, got)
	}
}

func TestGetCacheDirUsesMonorepoRoot(t *testing.T) {
	monoRoot := t.TempDir()
	clientDir := filepath.Join(monoRoot, "repo", "cli")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(clientDir)
	cacheDir := getCacheDir()
	wantPrefix := filepath.Join(monoRoot, ".🧬semio", "🦑️repo", "⚡️cache") + string(os.PathSeparator)
	if !strings.HasPrefix(cacheDir, wantPrefix) {
		t.Fatalf("expected cache dir under %q, got %q", wantPrefix, cacheDir)
	}
}

func TestFilePolicyNoGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("content"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	if len(breachs) != 0 {
		t.Errorf("expected no breachs when godfile is missing, got %d", len(breachs))
	}
}

func TestFolderPolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("folder")
	if !found {
		t.Fatal("folder policy not registered")
	}
	if policy.Name != "Folder" {
		t.Errorf("expected policy name Folder, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFolderIllegalEmpty {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("folder policy should contain BreachFolderIllegalEmpty kind")
	}
}

func TestFilePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("file")
	if !found {
		t.Fatal("file policy not registered")
	}
	if policy.Name != "File" {
		t.Errorf("expected policy name File, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFileIllegalUseGodfile {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("file policy should contain BreachFileIllegalUseGodfile kind")
	}
}

func TestComposePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("compose")
	if !found {
		t.Fatal("compose policy not registered")
	}
	if policy.Name != "Compose" {
		t.Errorf("expected policy name Compose, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachComposeNoUiDependency {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("compose policy should contain BreachComposeNoUiDependency kind")
	}
}

func TestComposePolicyNoUiDependency(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	t.Run("detects tailwind-merge import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { twMerge } from \"tailwind-merge\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects elements/ui import", func(t *testing.T) {
		relPath := "compose.test.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as UI from \"../../elements/ui\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects clsx import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { ClassValue, clsx } from \"clsx\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects three import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as THREE from \"three\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects multiple ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\nimport { twMerge } from \"tailwind-merge\";\nimport * as THREE from \"three\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 3 {
			t.Fatalf("expected 3 breachs, got %d", len(breachs))
		}
	})
	t.Run("allows non-ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { z } from \"zod\";\nimport { v7 as uuidv7 } from \"uuid\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("ignores non-compose files", func(t *testing.T) {
		relPath := "other.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for non-compose file, got %d", len(breachs))
		}
	})
	t.Run("not autofixable", func(t *testing.T) {
		info := BreachComposeNoUiDependency.Info()
		if info.Autofixable {
			t.Error("BreachComposeNoUiDependency should not be autofixable")
		}
	})
}
