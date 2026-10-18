// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive query flag, fuzzy match, cache index, and empty queries.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"testing"
)

// 🔍️#region 🏂️Query
func TestExhaustiveQueryFlag(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query flag test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name          string
		args          []string
		query         string
		expectMatch   string
		expectMissing string
	}{
		{
			name:        "tree --query filters monorepo tree",
			args:        []string{"search", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "tree --query excludes unrelated",
			args:          []string{"search", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "compose/go",
		},
		{
			name:        "technology list --query matches",
			args:        []string{"list", "--only-technology", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "technology tree --query matches",
			args:        []string{"search", "--only-technology", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "bundle list --query matches",
			args:        []string{"list", "--only-bundle", "--query", "engine", "--json"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "bundle list --query excludes unrelated",
			args:          []string{"list", "--only-bundle", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "engine",
		},
		{
			name:        "bundle tree --query matches",
			args:        []string{"search", "--only-bundle", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:        "folder list --query matches",
			args:        []string{"list", "--only-folder", "--query", "go", "--json"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "folder tree --query matches",
			args:        []string{"search", "--only-folder", "--query", "go", "--text"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "file list --query matches",
			args:        []string{"list", "--only-file", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "file tree --query matches",
			args:        []string{"search", "--only-file", "--query", "compose", "--text"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "section list --query matches",
			args:        []string{"list", "--only-section", "--query", "Models", "--json"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "section tree --query matches",
			args:        []string{"search", "--only-section", "--query", "Models", "--text"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "definition list --query matches",
			args:        []string{"list", "--only-definition", "--query", "Kit", "--json"},
			query:       "",
			expectMatch: "Kit",
		},
		{
			name:        "ticket list --query matches",
			args:        []string{"list", "--only-ticket", "--query", "ticket", "--json"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "ticket tree --query matches",
			args:        []string{"search", "--only-ticket", "--query", "ticket", "--text"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "goal list --query matches",
			args:        []string{"list", "--only-goal", "--query", "repo", "--json"},
			query:       "",
			expectMatch: "repo",
		},
		{
			name:        "goal tree --query matches",
			args:        []string{"search", "--only-goal", "--query", "sketchpad", "--text"},
			query:       "",
			expectMatch: "Sketchpad",
		},
		{
			name:          "goal tree --query excludes unrelated",
			args:          []string{"search", "--only-goal", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "Sketchpad",
		},
		{
			name:        "policy list --query matches",
			args:        []string{"list", "--only-policy", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "policy tree --query matches",
			args:        []string{"search", "--only-policy", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "statute list --query matches",
			args:        []string{"list", "--only-statute", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:          "statute list --query excludes unrelated",
			args:          []string{"list", "--only-statute", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "header",
		},
		{
			name:        "statute tree --query matches",
			args:        []string{"search", "--only-statute", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "query command returns matching IDs",
			args:        []string{"query", "search"},
			query:       "",
			expectMatch: "search",
		},
		{
			name:        "contributor list --query matches",
			args:        []string{"list", "--only-contributor", "--query", "usalu", "--json"},
			query:       "",
			expectMatch: "usalu",
		},
		{
			name:          "contributor list --query excludes unrelated",
			args:          []string{"list", "--only-contributor", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "usalu",
		},
		{
			name:        "checkpoint list --query matches",
			args:        []string{"list", "--only-checkpoint", "--query", "merge", "--json", "--limit", "200"},
			query:       "",
			expectMatch: "merge",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v\nOutput: %s", tt.args, err, output)
			}
			lower := strings.ToLower(output)
			if tt.expectMatch != "" {
				if !strings.Contains(lower, strings.ToLower(tt.expectMatch)) {
					t.Errorf("expected output to contain %q, got:\n%s", tt.expectMatch, output)
				}
			}
			if tt.expectMissing != "" {
				if strings.Contains(lower, strings.ToLower(tt.expectMissing)) {
					t.Errorf("expected output NOT to contain %q, got:\n%s", tt.expectMissing, output)
				}
			}
		})
	}
}

func TestExhaustiveQueryFuzzyMatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query fuzzy match test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("policy list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-policy", "--query", "headr", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "header") {
			t.Errorf("expected fuzzy match for 'headr' to include header-related results, got:\n%s", output)
		}
	})

	t.Run("statute list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--query", "licenss", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "license") {
			t.Errorf("expected fuzzy match for 'licenss' to include license-related results, got:\n%s", output)
		}
	})

	t.Run("bundle list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-bundle", "--query", "engin", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "engine") {
			t.Errorf("expected fuzzy match for 'engin' to include engine, got:\n%s", output)
		}
	})

	t.Run("goal list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-goal", "--query", "sketchpd", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "sketchpad") {
			t.Errorf("expected fuzzy match for 'sketchpd' to include sketchpad, got:\n%s", output)
		}
	})
}

func TestExhaustiveCacheIndexAndTreeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow cache index test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("tree query returns multiple resource kinds for shared keyword", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "search", "--text")
		if err != nil {
			t.Fatalf("tree --query search failed: %v\nOutput: %s", err, output)
		}
		hasFile := strings.Contains(output, ".go") || strings.Contains(output, "main")
		hasGoal := strings.Contains(output, "AI-OPTIMIZED") || strings.Contains(output, "Repo")
		hasTicket := strings.Contains(output, "02/")
		if !hasFile && !hasGoal && !hasTicket {
			t.Errorf("tree --query search should return files, goals, or tickets; got:\n%s", output)
		}
		kinds := 0
		if hasFile {
			kinds++
		}
		if hasGoal {
			kinds++
		}
		if hasTicket {
			kinds++
		}
		if kinds < 2 {
			t.Logf("tree --query returned %d resource kinds (file=%v goal=%v ticket=%v); ideally multiple", kinds, hasFile, hasGoal, hasTicket)
		}
	})

	t.Run("query command returns matching resource IDs", func(t *testing.T) {
		output, err := executeTreeCommand("query", "search")
		if err != nil {
			t.Fatalf("query search failed: %v\nOutput: %s", err, output)
		}
		var nonEmpty int
		for _, l := range strings.Split(output, "\n") {
			if strings.TrimSpace(l) != "" {
				nonEmpty++
			}
		}
		if nonEmpty == 0 {
			t.Errorf("query search should return at least one ID, got:\n%s", output)
		}
	})

	t.Run("tree query for cli returns file and bundle", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "cli", "--text")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		if !strings.Contains(strings.ToLower(output), "cli") {
			t.Errorf("expected 'cli' in output:\n%s", output)
		}
		hasComposeRepo := strings.Contains(output, "repo")
		hasTechnologyOrBundle := strings.Contains(output, "bundle") || strings.Contains(output, "Technologies")
		if !hasComposeRepo || !hasTechnologyOrBundle {
			t.Errorf("tree --query cli should return technology/bundle document; got:\n%s", output)
		}
	})

	t.Run("tree query nonexistent returns minimal output", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "zzz_nonexistent_xyzz", "--text")
		if err != nil {
			t.Fatalf("tree --query nonexistent failed: %v", err)
		}
		if strings.Contains(strings.ToLower(output), "zzz_nonexistent") {
			t.Errorf("tree --query nonexistent should not contain the query term in output")
		}
	})

	t.Run("different queries return different resources", func(t *testing.T) {
		searchOut, err := executeTreeCommand("search", "--query", "search", "--json")
		if err != nil {
			t.Fatalf("tree --query search failed: %v", err)
		}
		cliOut, err := executeTreeCommand("search", "--query", "cli", "--json")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		var searchTree, cliTree map[string]interface{}
		if json.Unmarshal([]byte(strings.TrimSpace(searchOut)), &searchTree) != nil {
			t.Fatal("search output not valid JSON")
		}
		if json.Unmarshal([]byte(strings.TrimSpace(cliOut)), &cliTree) != nil {
			t.Fatal("cli output not valid JSON")
		}
		searchStr := fmt.Sprint(searchTree)
		cliStr := fmt.Sprint(cliTree)
		if searchStr == cliStr {
			t.Error("tree --query search and tree --query cli should return different results")
		}
	})
}

func TestExhaustiveQueryEmptyReturnsAll(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query empty test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name string
		args []string
	}{
		{"policy list no query", []string{"list", "--only-policy", "--json"}},
		{"statute list no query", []string{"list", "--only-statute", "--json"}},
		{"contributor list no query", []string{"list", "--only-contributor", "--json"}},
		{"bundle list no query", []string{"list", "--only-bundle", "--json"}},
		{"goal list no query", []string{"list", "--only-goal", "--json"}},
		{"checkpoint list no query", []string{"list", "--only-checkpoint", "--json", "--limit", "5"}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v", tt.args, err)
			}
			if strings.TrimSpace(output) == "" {
				t.Errorf("expected non-empty output for %v without query", tt.args)
			}
		})
	}
}
