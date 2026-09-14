// 🔬️ Tests of the move domain, split out of the pre-split godfile suite.

package move

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

const sectionFixtureRelPath = "🗃️pkg/💻️index.ts"

const sectionFixtureContent = `// #region 🔖️Header
// [💻️pkg/index.ts](repo://file/💻️pkg)
// #endregion 🔖️Header

// #region 🧩️State
export const state = 1;

// #region 🏪️Store
export const store = 2;
// #endregion 🏪️Store
// #endregion 🧩️State
`

// 🧫️withSectionFixture points the workspace root at a throwaway tree holding one sectioned file.
func withSectionFixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	abs := filepath.Join(root, filepath.FromSlash(sectionFixtureRelPath))
	if err := os.MkdirAll(filepath.Dir(abs), 0755); err != nil {
		t.Fatalf("mkdir fixture dir: %v", err)
	}
	if err := os.WriteFile(abs, []byte(sectionFixtureContent), 0644); err != nil {
		t.Fatalf("write fixture file: %v", err)
	}
	previous := workspace.RootDir
	workspace.RootDir = root
	t.Cleanup(func() { workspace.RootDir = previous })
	codebase.InvalidateTechnologyCache()
	return sectionFixtureRelPath
}

func TestFolderCreateMoveDelete(t *testing.T) {
	testFolder := "temp/test-folder-cli"
	createResult := ToolFolderCreate(testFolder)
	if createResult.Error != "" {
		t.Errorf("ToolFolderCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFolderMove(testFolder, testFolder+"-moved")
	if moveResult.Error != "" {
		t.Errorf("ToolFolderMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFolderDelete(testFolder + "-moved")
	if deleteResult.Error != "" {
		t.Errorf("ToolFolderDelete returned error: %s", deleteResult.Error)
	}
}

func TestFileCreateMoveDelete(t *testing.T) {
	testFile := "temp/test-file-cli.txt"
	createResult := ToolFileCreate(testFile)
	if createResult.Error != "" {
		t.Errorf("ToolFileCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFileMove(testFile, "temp/test-file-cli-moved.txt")
	if moveResult.Error != "" {
		t.Errorf("ToolFileMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFileDelete("temp/test-file-cli-moved.txt")
	if deleteResult.Error != "" {
		t.Errorf("ToolFileDelete returned error: %s", deleteResult.Error)
	}
}

func TestApplyRenameCasingsReplacesAllVariants(t *testing.T) {
	input := "MODEL Model model getModel ModelFile USER_MODEL"
	got := ApplyRenameCasings(input, "model", "representation")
	want := "REPRESENTATION Representation representation getRepresentation RepresentationFile USER_REPRESENTATION"
	if got != want {
		t.Fatalf("applyRenameCasings mismatch:\n  got:  %q\n  want: %q", got, want)
	}
}

func TestApplyRenameCasingsKeepsUnrelatedContent(t *testing.T) {
	input := "foo bar baz"
	got := ApplyRenameCasings(input, "model", "representation")
	if got != input {
		t.Fatalf("applyRenameCasings changed unrelated content: %q", got)
	}
}

// 📑️#region 🖲️Section
func TestSectionListCommand(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolSectionList returned nil data")
	}
	sections, ok := result.Data.([]model.Section)
	if !ok {
		t.Error("ToolSectionList data is not []SectionInfo")
		return
	}
	if len(sections) == 0 {
		t.Error("ToolSectionList returned no sections")
	}
	foundHeader := false
	for _, s := range sections {
		if s.Name == "Header" {
			foundHeader = true
			break
		}
	}
	if !foundHeader {
		t.Errorf("Expected to find 'Header' section in %s", sectionFixtureRelPath)
	}
}

// 📖️#region 🐍️Definition
func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

func TestSectionListIDs(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Fatalf("ToolSectionList returned error: %s", result.Error)
	}
	sections, ok := result.Data.([]model.Section)
	if !ok {
		t.Fatal("ToolSectionList data is not []Section")
	}
	if len(sections) == 0 {
		t.Fatal("ToolSectionList returned no sections")
	}
	seenEmojis := make(map[string]string)
	for _, s := range sections {
		localID := s.GetID()
		emoji := s.Emoji
		if emoji == "" {
			t.Errorf("section %q has no emoji", s.Name)
			continue
		}
		flatName := workspace.Flat(s.Name)
		expectedID := model.EmojiText(emoji) + flatName
		if localID != expectedID {
			t.Errorf("section %q local id: expected %q, got %q", s.Name, expectedID, localID)
		}
		if prev, exists := seenEmojis[emoji]; exists {
			t.Errorf("section %q has duplicate emoji %q (same as %q)", s.Name, emoji, prev)
		}
		seenEmojis[emoji] = s.Name
	}
}

func TestToolSectionList(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
}

func TestToolDefinitionList(t *testing.T) {
	result := ToolDefinitionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

func TestToolFileCRUD(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	result := ToolFileCreate("test.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileCreate returned error: %s", result.Error)
	}

	result = ToolFileMove("test.txt", "renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileMove returned error: %s", result.Error)
	}

	result = ToolFileDelete("renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileDelete returned error: %s", result.Error)
	}
}

func TestToolRenameRewritesFilesAndFilenames(t *testing.T) {
	tmpRoot := t.TempDir()
	oldRoot := workspace.GetRootDir()
	workspace.SetRootDir(tmpRoot)
	defer workspace.SetRootDir(oldRoot)
	workspace.RootDir = tmpRoot

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
	oldRoot := workspace.GetRootDir()
	workspace.SetRootDir(tmpRoot)
	defer workspace.SetRootDir(oldRoot)
	workspace.RootDir = tmpRoot

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
