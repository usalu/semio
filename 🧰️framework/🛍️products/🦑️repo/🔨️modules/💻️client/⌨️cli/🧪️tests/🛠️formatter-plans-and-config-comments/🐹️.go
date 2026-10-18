// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Formatter plans and config-file comment scanning.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"strings"
	"testing"
)

func TestFormatterPlansCoverRegisteredLanguages(t *testing.T) {
	for _, language := range languageRegistry {
		plans := formatterPlansForLanguage(language.Name(), "example.file")
		if len(plans) == 0 {
			t.Fatalf("expected formatter plans for language %q", language.Name())
		}
		for _, plan := range plans {
			if strings.TrimSpace(plan.binary) == "" {
				t.Fatalf("formatter plan for language %q has empty binary", language.Name())
			}
		}
	}
}

func TestFixConfigIgnored(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// comment in config\nconst x = 1;\n"
	testFile := "package.json"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	if len(breachs) != 0 {
		t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
	}
}
