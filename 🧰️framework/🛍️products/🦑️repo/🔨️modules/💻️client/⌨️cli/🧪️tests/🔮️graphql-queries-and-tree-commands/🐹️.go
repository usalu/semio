// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// GraphQL queries and tree commands.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"context"
	"encoding/json"
	"strings"
	"testing"
)

// 🕸️#region 🎙️GraphQL
func TestGraphQLRepoQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "compose") {
		t.Errorf("Expected result to contain 'compose', got: %s", result)
	}
}

func TestGraphQLBundlesQuery(t *testing.T) {
	root := withMonorepoFixture(t)
	result, err := newExecutorFor(t, root).ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL bundles returned error: %v", err)
	}
	if !strings.Contains(result, "compose/js") {
		t.Errorf("Expected result to contain 'compose/js', got: %s", result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestExhaustiveGraphQLTicketsQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow graphql tickets query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "tickets") {
		t.Errorf("Expected result to contain 'tickets', got: %s", result)
	}
}

func TestExhaustiveGraphQLAnalyzeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow analyze query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ analyze(scope: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐹️.go") { metrics { total } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL analyze returned error: %v", err)
	}
	if !strings.Contains(result, "total") {
		t.Errorf("Expected result to contain 'total', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

func TestGraphQLFixMutationIsRemoved(t *testing.T) {
	_, err := executor.ExecuteJSON(context.Background(), `mutation { fix(scope: "repo/go/main_test.go") { fixed remaining } }`, nil)
	if err == nil {
		t.Fatal("expected the removed fix mutation to be rejected")
	}
	if !strings.Contains(err.Error(), `unknown field "fix"`) {
		t.Errorf("expected an unknown field error for fix, got: %v", err)
	}
}

// 🌳️#region 📜️Tree
func executeTreeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	root, _ := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)

	err := root.Execute()
	return buf.String(), err
}

func TestExhaustiveTreeCommands(t *testing.T) {

	if testing.Short() {
		t.Skip("skipping slow tree test")
	}

	output, err := executeTreeCommand("search", "compose/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻️compose") {
		t.Errorf("repo tree compose/go missing compose.go, got:\n%s", output)
	}
	if strings.Contains(output, "├️─️─️ ") || strings.Contains(output, "└️─️─️ ") {
		t.Errorf("repo tree default output must be markdown, got:\n%s", output)
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("repo tree default output missing markdown list items, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-folder", "compose/go")
	if err != nil {
		t.Errorf("folder tree failed: %v", err)
	}
	if !strings.Contains(output, "compose.go") {

		if len(output) < 10 {
			t.Errorf("folder tree output suspicious: %s", output)
		}
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("folder tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-file", "compose/go")
	if err != nil {
		t.Errorf("file tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻️compose") {
		t.Errorf("file tree missing compose.go")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("file tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-ticket")
	if err != nil {
		t.Errorf("ticket tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("ticket tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("ticket tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-goal")
	if err != nil {
		t.Errorf("goal tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("goal tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("goal tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "compose/go", "--text")
	if err != nil {
		t.Errorf("repo tree text failed: %v", err)
	}
	if !strings.Contains(output, "├️─️─️ ") && !strings.Contains(output, "└️─️─️ ") {
		t.Errorf("repo tree text output should use connectors, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "compose/go", "--json")
	if err != nil {
		t.Errorf("repo tree json failed: %v", err)
	}
	var parsed map[string]interface{}
	if parseErr := json.Unmarshal([]byte(strings.TrimSpace(output)), &parsed); parseErr != nil {
		t.Errorf("repo tree json output is invalid JSON: %v\noutput:\n%s", parseErr, output)
	}
	if _, ok := parsed["kind"]; !ok {
		if _, ok := parsed["Kind"]; !ok {
			t.Errorf("repo tree json output missing kind field: %s", output)
		}
	}
}
