// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Rename casing and rename tool.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"testing"
)

// 🔤️#region 🪪️Rename

func TestApplyRenameCasingsReplacesAllVariants(t *testing.T) {
	input := "MODEL Model model getModel ModelFile USER_MODEL"
	got := applyRenameCasings(input, "model", "representation")
	want := "REPRESENTATION Representation representation getRepresentation RepresentationFile USER_REPRESENTATION"
	if got != want {
		t.Fatalf("applyRenameCasings mismatch:\n  got:  %q\n  want: %q", got, want)
	}
}

func TestApplyRenameCasingsKeepsUnrelatedContent(t *testing.T) {
	input := "foo bar baz"
	got := applyRenameCasings(input, "model", "representation")
	if got != input {
		t.Fatalf("applyRenameCasings changed unrelated content: %q", got)
	}
}

func TestToolRenameRewritesFilesAndFilenames(t *testing.T) {
	tmpRoot := t.TempDir()
	oldRoot := GetRootDir()
	SetRootDir(tmpRoot)
	defer SetRootDir(oldRoot)
	rootDir = tmpRoot

	nestedDir := filepath.Join(tmpRoot, "pkg", "model")
	if err := os.MkdirAll(nestedDir, 0755); err != nil {
		t.Fatalf("mkdir failed: %v", err)
	}
	contentFile := filepath.Join(nestedDir, "Model.go")
	contents := "package model\n\ntype Model struct{}\nconst USER_MODEL = \"model\"\n"
	if err := os.WriteFile(contentFile, []byte(contents), 0644); err != nil {
		t.Fatalf("write contents: %v", err)
	}
	unrelated := filepath.Join(tmpRoot, "pkg", "other.go")
	if err := os.WriteFile(unrelated, []byte("package other\n"), 0644); err != nil {
		t.Fatalf("write other: %v", err)
	}
	ignoredPath := filepath.Join(tmpRoot, "ignored", "model.txt")
	if err := os.MkdirAll(filepath.Dir(ignoredPath), 0755); err != nil {
		t.Fatalf("mkdir ignored: %v", err)
	}
	if err := os.WriteFile(ignoredPath, []byte("model stays"), 0644); err != nil {
		t.Fatalf("write ignored: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tmpRoot, ".gitignore"), []byte("ignored/\n"), 0644); err != nil {
		t.Fatalf("write .gitignore: %v", err)
	}

	result := ToolRename("model", "representation", "")
	if result.Error != "" {
		t.Fatalf("ToolRename error: %s", result.Error)
	}

	renamedContent := filepath.Join(tmpRoot, "pkg", "representation", "Representation.go")
	data, err := os.ReadFile(renamedContent)
	if err != nil {
		t.Fatalf("expected renamed file at %s: %v", renamedContent, err)
	}
	want := "package representation\n\ntype Representation struct{}\nconst USER_REPRESENTATION = \"representation\"\n"
	if string(data) != want {
		t.Fatalf("content mismatch:\n  got:  %q\n  want: %q", string(data), want)
	}
	if _, err := os.Stat(contentFile); !os.IsNotExist(err) {
		t.Fatalf("expected original path to be gone, stat err: %v", err)
	}
	if _, err := os.Stat(filepath.Join(tmpRoot, "pkg", "other.go")); err != nil {
		t.Fatalf("unrelated file disappeared: %v", err)
	}
	ignoredData, err := os.ReadFile(ignoredPath)
	if err != nil {
		t.Fatalf("read ignored file: %v", err)
	}
	if string(ignoredData) != "model stays" {
		t.Fatalf("gitignored file was modified: %q", string(ignoredData))
	}
	if _, err := os.Stat(ignoredPath); err != nil {
		t.Fatalf("gitignored file was renamed: %v", err)
	}
}

func TestToolRenameRejectsEmptyAndIdenticalTokens(t *testing.T) {
	if got := ToolRename("", "new", ""); got.Error == "" {
		t.Fatalf("expected error for empty old token")
	}
	if got := ToolRename("model", "", ""); got.Error == "" {
		t.Fatalf("expected error for empty new token")
	}
	if got := ToolRename("Model", "MODEL", ""); got.Error == "" {
		t.Fatalf("expected error for case-only identical tokens")
	}
}

func TestToolRenameScopeLimitsWalk(t *testing.T) {
	tmpRoot := t.TempDir()
	oldRoot := GetRootDir()
	SetRootDir(tmpRoot)
	defer SetRootDir(oldRoot)
	rootDir = tmpRoot

	inScope := filepath.Join(tmpRoot, "compose", "model.txt")
	outOfScope := filepath.Join(tmpRoot, "elements", "model.txt")
	if err := os.MkdirAll(filepath.Dir(inScope), 0755); err != nil {
		t.Fatalf("mkdir in-scope: %v", err)
	}
	if err := os.MkdirAll(filepath.Dir(outOfScope), 0755); err != nil {
		t.Fatalf("mkdir out-of-scope: %v", err)
	}
	if err := os.WriteFile(inScope, []byte("model in"), 0644); err != nil {
		t.Fatalf("write in: %v", err)
	}
	if err := os.WriteFile(outOfScope, []byte("model out"), 0644); err != nil {
		t.Fatalf("write out: %v", err)
	}

	result := ToolRename("model", "representation", "compose")
	if result.Error != "" {
		t.Fatalf("ToolRename error: %s", result.Error)
	}

	if _, err := os.Stat(inScope); !os.IsNotExist(err) {
		t.Fatalf("expected in-scope file to be renamed, stat err: %v", err)
	}
	renamed := filepath.Join(tmpRoot, "compose", "representation.txt")
	data, err := os.ReadFile(renamed)
	if err != nil {
		t.Fatalf("expected renamed file at %s: %v", renamed, err)
	}
	if string(data) != "representation in" {
		t.Fatalf("in-scope content not rewritten: %q", string(data))
	}
	outData, err := os.ReadFile(outOfScope)
	if err != nil {
		t.Fatalf("out-of-scope file disappeared: %v", err)
	}
	if string(outData) != "model out" {
		t.Fatalf("out-of-scope file was rewritten: %q", string(outData))
	}
}
