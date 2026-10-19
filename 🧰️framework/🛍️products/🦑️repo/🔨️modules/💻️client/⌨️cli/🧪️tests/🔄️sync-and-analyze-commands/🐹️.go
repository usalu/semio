// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// GitHub sync, analyze, and fix commands.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"context"
	command "github.com/usalu/semio/repo/client/internal/command"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type recordingGraphQLExecutor struct {
	queries []string
}

func (e *recordingGraphQLExecutor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	e.queries = append(e.queries, query)
	return map[string]interface{}{"syncManagement": true}, nil
}

func TestSyncCommandRunsGitHubSynchronization(t *testing.T) {
	newRoot := func(recorder *recordingGraphQLExecutor) *command.Command {
		factory := func(config Config) (*Engine, error) {
			return NewEngine(recorder), nil
		}
		root, config := NewRootWithConfig(factory)
		config.Format = "json"
		return root
	}

	t.Run("github target executes sync management mutation", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		stdout := new(bytes.Buffer)
		stderr := new(bytes.Buffer)
		root.SetOut(stdout)
		root.SetErr(stderr)
		root.SetArgs([]string{"sync", "github"})

		if err := root.Execute(); err != nil {
			t.Fatalf("sync github failed: %v\nstdout: %s\nstderr: %s", err, stdout.String(), stderr.String())
		}
		if len(recorder.queries) != 1 {
			t.Fatalf("expected one GraphQL query, got %d", len(recorder.queries))
		}
		if !strings.Contains(recorder.queries[0], "syncManagement") {
			t.Fatalf("expected syncManagement mutation, got: %s", recorder.queries[0])
		}
		if !strings.Contains(stdout.String(), "syncManagement") {
			t.Fatalf("expected sync result in stdout, got: %s", stdout.String())
		}
	})

	t.Run("management target executes same mutation", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		root.SetOut(new(bytes.Buffer))
		root.SetErr(new(bytes.Buffer))
		root.SetArgs([]string{"sync", "management"})

		if err := root.Execute(); err != nil {
			t.Fatalf("sync management failed: %v", err)
		}
		if len(recorder.queries) != 1 || !strings.Contains(recorder.queries[0], "syncManagement") {
			t.Fatalf("expected syncManagement mutation, got queries: %v", recorder.queries)
		}
	})

	t.Run("unknown target fails instead of printing help as success", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		root.SetOut(new(bytes.Buffer))
		root.SetErr(new(bytes.Buffer))
		root.SetArgs([]string{"sync", "githb"})

		err := root.Execute()
		if err == nil {
			t.Fatal("expected unknown sync target to fail")
		}
		if !strings.Contains(err.Error(), `unknown sync target "githb"`) {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(recorder.queries) != 0 {
			t.Fatalf("unknown sync target must not execute GraphQL, got queries: %v", recorder.queries)
		}
	})
}

// 🧪️#region 🏩️Codebase
func TestExhaustiveCodebaseCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow codebase test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

// 🔬️#region 🗝️Analyze
func TestAnalyzeCommand(t *testing.T) {
	result := ToolAnalyze("compose/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := ToolAnalyze("compose/js/compose.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

func TestAnalyzeReadsBreachCacheJSON(t *testing.T) {
	tmp := t.TempDir()
	cacheDir := filepath.Join(tmp, ".🧬semio", "🦑️repo", "⚡️cache", "breaches")
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		t.Fatal(err)
	}
	cachePath := filepath.Join(cacheDir, "unit-test.json")
	payload := `{
  "entityId": "test",
  "script": "unit.test.lint.script.ts",
  "breachs": [
    {
      "id": "e1",
      "summary": "hello",
      "kind": "lint/test/rule",
      "scope": "repo/example.go",
      "priority": "medium"
    }
  ]
}`
	if err := os.WriteFile(cachePath, []byte(payload), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()
	ctx := NewRepoContext(tmp)
	ar, err := ctx.Analyze(nil)
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if ar == nil || len(ar.Breachs) != 1 {
		t.Fatalf("expected 1 breach, got %#v", ar)
	}
	if ar.Breachs[0].Summary != "hello" {
		t.Fatalf("unexpected breach: %+v", ar.Breachs[0])
	}
}
