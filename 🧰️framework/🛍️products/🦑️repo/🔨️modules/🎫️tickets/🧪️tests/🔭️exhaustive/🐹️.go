//go:build exhaustive

// 🔬️ Tests of the tickets domain, split out of the pre-split godfile suite.

package tickets

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveNormalizeTicketFileInput(t *testing.T) {
	root := t.TempDir()
	write := func(rel, content string) {
		abs := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
			t.Fatalf("mkdir %s: %v", rel, err)
		}
		if err := os.WriteFile(abs, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", rel, err)
		}
	}
	write("repo/README.md", "---\nname: repo\nkind: infrastructure\n---\n\n# repo\n")
	write("repo/client/AGENTS.md", "---\nbundle:\n  emoji: ⌨️\n---\n")
	write("repo/client/main.go", "package main\n\nfunc main() {}\n")
	oldRoot := workspace.GetRootDir()
	workspace.SetRootDir(root)
	codebase.InvalidateTechnologyCache()
	t.Cleanup(func() {
		workspace.SetRootDir(oldRoot)
		codebase.InvalidateTechnologyCache()
	})

	filePath := filepath.ToSlash(filepath.Join("repo", "client", "main.go"))
	cases := []struct {
		name  string
		input string
	}{
		{"path", filePath},
		{"abs path", filepath.Join(root, filePath)},
		{"file uri", model.BuildFileUriFromPath(filePath)},
		{"id", codebase.FileHeaderId(filePath)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			if got := normalizeTicketFileInput(tc.input); got != filePath {
				t.Fatalf("expected %s, got %s", filePath, got)
			}
		})
	}
}
