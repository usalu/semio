package cli

import (
	"os"
	"path/filepath"
	"testing"
)

type pairedWorkspaceTool struct {
	root  string
	calls int
}

func (tool *pairedWorkspaceTool) Run(root string, _ []string) error {
	tool.root = root
	tool.calls++
	return nil
}

func TestNativePairedWorkspace(t *testing.T) {
	outer := filepath.Clean(t.TempDir())
	inner := filepath.Join(outer, "nested")
	cwd := filepath.Join(inner, "work")
	for _, root := range []string{outer, inner} {
		if err := os.MkdirAll(root, 0o755); err != nil {
			t.Fatal(err)
		}
		for name, value := range map[string]string{"package.json": `{"workspaces":[]}`, "📋️project.json": `{"name":"paired-workspace-fixture"}`} {
			if err := os.WriteFile(filepath.Join(root, name), []byte(value), 0o644); err != nil {
				t.Fatal(err)
			}
		}
	}
	if err := os.Mkdir(cwd, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Chdir(cwd)
	t.Setenv("SEMIO_WORKSPACE_ROOT", "")
	t.Setenv("REPO_ROOT", "")
	tool := &pairedWorkspaceTool{}
	if err := RunNativeScript([]string{"./probe.mjs"}, tool); err != nil {
		t.Fatal(err)
	}
	if tool.root != inner || tool.calls != 1 {
		t.Fatalf("nearest paired root: root=%s calls=%d", tool.root, tool.calls)
	}
	t.Setenv("SEMIO_WORKSPACE_ROOT", outer)
	if err := RunNativeScript([]string{"./probe.mjs"}, tool); err != nil {
		t.Fatal(err)
	}
	if tool.root != outer {
		t.Fatalf("explicit paired root ignored: %s", tool.root)
	}
}
