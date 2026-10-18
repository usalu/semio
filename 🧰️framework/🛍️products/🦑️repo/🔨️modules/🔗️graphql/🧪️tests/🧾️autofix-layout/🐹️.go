// 🧾️ Tests of the header, folder and region layout autofixes.

package graphql

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestFixHeaderWithShebang(t *testing.T) {
	tmpDir := t.TempDir()
	originalRootDir := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(originalRootDir)

	filePath := "script.py"
	absPath := filepath.Join(tmpDir, filePath)
	content := "#!/usr/bin/env python3\n" +
		"#region 🔖️Header\n\n" +
		"# wrong/path.py\n\n" +
		"# 2025 Test <t@t.com>\n\n" +
		"# #region 🔖️License\n" +
		"# AGPL\n" +
		"# #endregion 🔖️License\n\n" +
		"# #region 🔖️Requirements\n" +
		"# 💯️Requirements\n" +
		"# #endregion 🔖️Requirements\n\n" +
		"#endregion 🔖️Header\n\n" +
		"print(\"hello\")\n"
	if err := os.WriteFile(absPath, []byte(content), 0644); err != nil {
		t.Fatalf("failed to write fixture: %v", err)
	}

	bundles := codebase.LoadBundles()
	autofixableBreachs := func() []model.Breach {
		t.Helper()
		breachs, err := statutes.CheckPolicies(workspace.ParseScope(filePath), bundles, nil)
		if err != nil {
			t.Fatalf("CheckPolicies failed: %v", err)
		}
		pending := make([]model.Breach, 0, len(breachs))
		for _, v := range breachs {
			if v.Autofixable() {
				pending = append(pending, v)
			}
		}
		return pending
	}

	before := autofixableBreachs()
	if len(before) == 0 {
		t.Fatal("expected autofixable breachs in the shebang fixture")
	}

	fixed, err := applyAutofixes(filePath, before)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Fatal("expected at least one fix applied")
	}

	newContent, err := workspace.ReadTextFile(absPath)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}
	if !strings.HasPrefix(newContent, "#!/usr/bin/env python3\n") {
		t.Errorf("shebang must stay on the first line of the fixed content:\n%s", newContent)
	}
	if !strings.Contains(newContent, "print(\"hello\")") {
		t.Errorf("body lost by the autofix:\n%s", newContent)
	}

	after := autofixableBreachs()
	if len(after) >= len(before) {
		t.Errorf("autofix did not reduce the autofixable breachs: %d before, %d after", len(before), len(after))
	}
}

func TestFolderPolicyEmptyFolderAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "remove", "me")
	os.MkdirAll(emptyDir, 0755)
	breachs := []model.Breach{{
		Kind:    model.BreachFolderIllegalEmpty,
		Scope:   "remove/me/",
		Excerpt: "remove/me",
	}}
	fixed, err := applySystemAutofixes(breachs)
	if err != nil {
		t.Fatalf("applySystemAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}
	if _, statErr := os.Stat(emptyDir); !os.IsNotExist(statErr) {
		t.Error("empty folder should have been removed")
	}
}

func TestSectionNewlineAfterRegion(t *testing.T) {
	t.Run("detect_blank_line_after_region_typescript", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_go", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.go](repo://file/test.go)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Package\n\n// [🔖️test.go#Package](repo://section/test.go/Package)\n// Package declaration.\n\npackage main\n\n// #endregion 🔖️Package\n"
		testFile := "test.go"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Package section")
		}
	})

	t.Run("detect_blank_line_after_region_python", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "# region Header\n# [💻️test.py](repo://file/test.py)\n# 2025 Test <t@t.com>\n# AGPL\n# endregion Header\n\n# region Functions\n\n# [🔖️test.py#Functions](repo://section/test.py/Functions)\n# Utility functions.\n\ndef add(a, b):\n    return a + b\n\n# endregion Functions\n"
		testFile := "test.py"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_csharp", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "#region 🔖️Header\n// [💻️test.cs](repo://file/test.cs)\n// 2025 Test <t@t.com>\n// AGPL\n#endregion 🔖️Header\n\n#region 🔖️Classes\n\n// [🔖️test.cs#Classes](repo://section/test.cs/Classes)\n// Domain classes.\n\npublic class Foo {}\n\n#endregion 🔖️Classes\n"
		testFile := "test.cs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Classes section")
		}
	})

	t.Run("detect_blank_line_after_region_rust", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\npub mod structs { // 🔖️Structs\n\n// Struct definitions.\n\nstruct Foo {}\n\n} // 🔖️Structs\n"
		testFile := "test.rs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Structs section")
		}
	})

	t.Run("no_false_positive_without_blank_line", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		for _, v := range breachs {
			if v.Kind == model.BreachCodeSectionWrongFormatNewlineAfterRegion {
				t.Fatal("unexpected newline-after-region breach when no blank line exists")
			}
		}
	})

	t.Run("autofix_removes_blank_line_after_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		expected := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []model.Breach{
			{Kind: model.BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Functions", Line: 8},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("autofix_removes_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		expected := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []model.Breach{
			{Kind: model.BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Header", Line: 2},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("detect_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Header section")
		}
	})
}
