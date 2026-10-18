// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Fix scopes, non-autofixable breaches, and statute meta.

// #endregion 🧲️Header

package client

import (
	"os"
	"testing"
)

func TestFixNonAutofixableNotFixed(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	bundles := LoadBundles()
	path := "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx"
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	for _, v := range breachs {
		info := v.Kind.Info()
		if v.Autofixable() != info.Autofixable {
			t.Errorf("breach %s: Autofixable() = %v, Info().Autofixable = %v", v.Kind, v.Autofixable(), info.Autofixable)
		}
	}

	autofixableKinds := []Statute{
		BreachCodeFileWrongLicense,
	}
	counts := map[Statute]int{}
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
	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
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

func TestFixExtractFileFromScope(t *testing.T) {
	tests := []struct {
		scope    string
		expected string
	}{
		{"file.ts", "file.ts"},
		{"file.ts#Section", "file.ts"},
		{"file.ts::definition", "file.ts"},
		{"path/to/file.ts#Section/Sub", "path/to/file.ts"},
		{"path/to/file.ts::myFunc", "path/to/file.ts"},
	}
	for _, tt := range tests {
		result := extractFileFromScope(tt.scope)
		if result != tt.expected {
			t.Errorf("extractFileFromScope(%q) = %q, want %q", tt.scope, result, tt.expected)
		}
	}
}

func TestFixStatuteMeta(t *testing.T) {
	autofixableKinds := []Statute{
		BreachCodeFileMissingHeaderRegion,
		BreachCodeFileMissingLicense,
		BreachCodeFileWrongLicense,
		BreachCodeSectionEmpty,
		BreachCodeSectionMissingEndName,
		BreachCodeSectionNameMismatch,
		BreachCodeCommentInline,
		BreachCodeCommentBlock,
		BreachCodeCommentJSDoc,
	}
	for _, kind := range autofixableKinds {
		info := kind.Info()
		if !info.Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
		if info.Reason == "" {
			t.Errorf("statute %s has empty reason", kind)
		}
		if info.Solution == "" {
			t.Errorf("statute %s has empty solution", kind)
		}
	}

	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		info := kind.Info()
		if info.Autofixable {
			t.Errorf("statute %s should NOT be autofixable", kind)
		}
	}
}

func TestFindMatchingSectionStartName(t *testing.T) {
	lines := []string{
		"// #region 🔖️Outer",
		"",
		"// #region 🔖️Inner",
		"const x = 1;",
		"// #endregion 🔖️Inner",
		"",
		"// #endregion",
	}
	language := NewTypeScriptLanguage()

	name := findMatchingSectionStartName(lines, 6, language)
	if name != "Outer" {
		t.Errorf("expected Outer, got %q", name)
	}

	name = findMatchingSectionStartName(lines, 4, language)
	if name != "Inner" {
		t.Errorf("expected Inner, got %q", name)
	}
}
