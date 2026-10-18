// 🔃️ Tests of the cli domain: sync command.

package cli

import (
	bytes "bytes"
	strings "strings"
	testing "testing"
)

func TestSyncCommandRunsGitHubSynchronization(t *testing.T) {
	newRoot := func(recorder *recordingGraphQLExecutor) *Command {
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
