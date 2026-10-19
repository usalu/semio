package todos

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// #region 🆔️Definition Id Tests

func TestResolveAllTestDefinitionIDs(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	t.Run("go_test_file", func(t *testing.T) {
		goDir := filepath.Join(tmpDir, "pkg")
		os.MkdirAll(goDir, 0755)
		goContent := `package pkg
// 🧪️#region 📐️Tests
func TestAlpha(t *testing.T) {}
func TestBeta(t *testing.T) {}
func helperNotATest() {}
// #endregion 📐️Tests
`
		testFile := filepath.Join(goDir, "pkg_test.go")
		os.WriteFile(testFile, []byte(goContent), 0644)

		ids := ResolveAllTestDefinitionIDs([]string{"pkg/pkg_test.go"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}

		foundAlpha := false
		foundBeta := false
		for _, id := range ids {
			if strings.Contains(id, "testalpha") {
				foundAlpha = true
			}
			if strings.Contains(id, "testbeta") {
				foundBeta = true
			}
		}
		if !foundAlpha {
			t.Errorf("expected to find TestAlpha in IDs, got %v", ids)
		}
		if !foundBeta {
			t.Errorf("expected to find TestBeta in IDs, got %v", ids)
		}

		for _, id := range ids {
			if strings.Contains(id, "helpernotatest") {
				t.Errorf("expected helperNotATest to be excluded, but found in IDs: %s", id)
			}
		}
	})

	t.Run("python_test_file", func(t *testing.T) {
		pyDir := filepath.Join(tmpDir, "pytests")
		os.MkdirAll(pyDir, 0755)
		pyContent := `def test_foo():
    pass

def test_bar():
    pass

def helper():
    pass
`
		testFile := filepath.Join(pyDir, "test_stuff.py")
		os.WriteFile(testFile, []byte(pyContent), 0644)

		ids := ResolveAllTestDefinitionIDs([]string{"pytests/test_stuff.py"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}
		foundFoo := false
		foundBar := false
		for _, id := range ids {
			if strings.Contains(id, "testfoo") {
				foundFoo = true
			}
			if strings.Contains(id, "testbar") {
				foundBar = true
			}
		}
		if !foundFoo {
			t.Errorf("expected test_foo in IDs, got %v", ids)
		}
		if !foundBar {
			t.Errorf("expected test_bar in IDs, got %v", ids)
		}
	})

	t.Run("no_files", func(t *testing.T) {
		ids := ResolveAllTestDefinitionIDs(nil)
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nil files, got %d", len(ids))
		}
	})

	t.Run("nonexistent_file", func(t *testing.T) {
		ids := ResolveAllTestDefinitionIDs([]string{"nonexistent/file_test.go"})
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nonexistent file, got %d", len(ids))
		}
	})
}

func TestEventIDsUseComposeRepoFormat(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess event id test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	sandbox.BindRoot(t, tmpDir)

	os.MkdirAll(filepath.Join(tmpDir, "src"), 0755)
	os.WriteFile(filepath.Join(tmpDir, "src", "main.go"), []byte("package main"), 0644)

	fileID := codebase.ResolvePathToFileID(filepath.Join(tmpDir, "src", "main.go"))

	if fileID == "" {
		t.Error("expected non-empty file ID")
	}
	if strings.Contains(fileID, "/") {
		t.Errorf("file ID should not contain path separators: %s", fileID)
	}

	rangeRef, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10")
	if err != nil {
		t.Errorf("unexpected error resolving range ref: %v", err)
	}
	if rangeRef == "" {
		t.Error("expected non-empty range ref")
	}
	lineEmoji := model.EmojiText(model.EmojiLine)
	if !strings.Contains(rangeRef, lineEmoji) {
		t.Errorf("expected %s in range ref, got: %s", lineEmoji, rangeRef)
	}
	if !strings.Contains(rangeRef, "10") {
		t.Errorf("expected line number 10 in range ref, got: %s", rangeRef)
	}

	rangeRefFull, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10-L20")
	if err != nil {
		t.Errorf("unexpected error resolving full range ref: %v", err)
	}
	fullRange := lineEmoji + "10" + lineEmoji + "20"
	if !strings.Contains(rangeRefFull, fullRange) {
		t.Errorf("expected %s in full range ref, got: %s", fullRange, rangeRefFull)
	}
}

// #endregion 🆔️Definition Id Tests
