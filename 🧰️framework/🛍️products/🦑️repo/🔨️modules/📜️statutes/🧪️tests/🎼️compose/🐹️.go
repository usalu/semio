package statutes

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// #region 🎼️Compose Policy Tests

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
		if k == model.BreachComposeNoUiDependency {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("compose policy should contain BreachComposeNoUiDependency kind")
	}
}

func TestComposePolicyNoUiDependency(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	t.Run("detects tailwind-merge import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { twMerge } from \"tailwind-merge\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects elements/ui import", func(t *testing.T) {
		relPath := "compose.test.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as UI from \"../../elements/ui\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects clsx import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { ClassValue, clsx } from \"clsx\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects three import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as THREE from \"three\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects multiple ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\nimport { twMerge } from \"tailwind-merge\";\nimport * as THREE from \"three\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 3 {
			t.Fatalf("expected 3 breachs, got %d", len(breachs))
		}
	})
	t.Run("allows non-ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { z } from \"zod\";\nimport { v7 as uuidv7 } from \"uuid\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("ignores non-compose files", func(t *testing.T) {
		relPath := "other.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for non-compose file, got %d", len(breachs))
		}
	})
	t.Run("not autofixable", func(t *testing.T) {
		info := model.BreachComposeNoUiDependency.Info()
		if info.Autofixable {
			t.Error("BreachComposeNoUiDependency should not be autofixable")
		}
	})
}

// #endregion 🎼️Compose Policy Tests
