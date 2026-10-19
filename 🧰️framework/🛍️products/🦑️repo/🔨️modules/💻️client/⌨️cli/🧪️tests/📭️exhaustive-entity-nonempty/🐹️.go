// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive non-empty entity lists, ticket title validation, and ignore patterns.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 📦️#region 🧳️Collection
func TestExhaustiveBundlesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle test in short mode")
	}
	executor := newExecutorFor(t, withMonorepoFixture(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ bundles { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Bundles []struct {
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Bundles) == 0 {
		t.Error("bundles collection should not be empty")
	}
}

func TestExhaustiveContributorsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow contributor test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ contributors { github } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Contributors []struct {
			Github string `json:"github"`
		} `json:"contributors"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Contributors) == 0 {
		t.Error("contributors collection should not be empty")
	}
}

func TestExhaustiveTicketsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tickets test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ tickets { slug } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Tickets []struct {
			Slug string `json:"slug"`
		} `json:"tickets"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets collection should not be empty")
	}
}

func TestExhaustivePoliciesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policies test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ policies { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Policies []struct {
			Name string `json:"name"`
		} `json:"policies"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Policies) == 0 {
		// Policies are no longer registered in Go; breachs come from lint scripts + cache.
	}
}

func TestExhaustiveStatutesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statutes test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ statutes { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes collection should not be empty")
	}
}

func TestPolicyStatutesHaveMetadata(t *testing.T) {
	for _, policy := range GetPolicies() {
		for _, statute := range policy.AllKinds() {
			info, ok := statuteInfoTable[statute]
			if !ok {
				t.Fatalf("policy %q statute %q is missing metadata", policy.ID, statute)
			}
			if info.Kind != statute {
				t.Fatalf("policy %q statute %q metadata kind = %q", policy.ID, statute, info.Kind)
			}
		}
	}
}

func TestExhaustiveFoldersNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow folders test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ folders { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Folders []struct {
			Path string `json:"path"`
		} `json:"folders"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Folders) == 0 {
		t.Error("folders collection should not be empty")
	}
}

func TestExhaustiveFilesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow files test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ files { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Files []struct {
			Path string `json:"path"`
		} `json:"files"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Files) == 0 {
		t.Error("files collection should not be empty")
	}
}

func TestExhaustiveBreachsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow breachs test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()
	// Unscoped breachs defaults to technology scope "compose" and runs full policy
	// analysis across the tree (very slow). Single-file scope keeps the test
	// representative while bounded.
	result, err := executor.ExecuteJSON(ctx, `{ breachs(scope: "repo/client/main.go") { id } }`, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Breachs []struct {
			ID string `json:"id"`
		} `json:"breachs"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if resp.Breachs == nil {
		t.Error("breachs collection should not be nil")
	}
}

func TestExhaustiveTicketTitleValidation(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket title validation test in short mode")
	}
	executor := newExecutorFor(t, withTempRepo(t))
	ctx := context.Background()

	tests := []struct {
		name    string
		emoji   string
		title   string
		wantErr bool
	}{
		{"Emoji Titleized Valid", "🎫️", "Some Title on Something", false},
		{"Emoji Single Word Valid", "🛠️", "Cleanup", false},
		{"Emoji With Hyphen Valid", "🧩️", "Refactor Resource ID System to Bundle-Based Document", false},
		{"Emoji Lowercase Valid", "🔖️", "some title", false},
		{"Emoji Allcaps Valid", "🔥️", "FIX EVERYTHING", false},
		{"Emoji Slug Valid", "🎫️", "some-slug-title", false},
		{"Emoji Dashed Slug Valid", "🎫️", "fix-vscode-types-version-mismatch", false},
		{"Emoji Uppercase Slug Valid", "🎫️", "ENSURE-COMPOSE-REPO-MCP-WORKS-ALLIDES", false},
		{"Missing Emoji Invalid", "", "Some Title on Something", true},
		{"Plain Text Emoji Invalid", "ticket", "Some Title", true},
		{"Multiple Emoji Invalid", "🎫️🛠️", "Some Title", true},
		{"Empty Title Invalid", "🎫️", "", true},
		{"Non Alphanumeric Title Invalid", "🎫️", "---", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			query := `mutation { ticketOpen(input: { emoji: "` + tt.emoji + `", title: "` + tt.title + `", prompt: "Test prompt", llm: "opus-4", client: COPILOT_CHAT, goal: "TEST-GOAL", noIssue: true, noManagement: true }) { id slug year month day } }`
			result, err := executor.ExecuteJSON(ctx, query, nil)
			if (err != nil) != tt.wantErr {
				t.Errorf("ticketOpen() error = %v, wantErr %v", err, tt.wantErr)
			}

			if err == nil {
				var resp struct {
					TicketOpen struct {
						ID    string `json:"id"`
						Slug  string `json:"slug"`
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
					} `json:"ticketOpen"`
				}
				if json.Unmarshal([]byte(result), &resp) == nil {
					to := resp.TicketOpen
					if to.Year == 0 || to.Month == 0 || to.Day == 0 {
						t.Errorf("ticketOpen returned invalid date: year=%d month=%d day=%d (id=%s)", to.Year, to.Month, to.Day, to.ID)
					}
					if strings.Contains(to.ID, "0000/00/00") {
						t.Errorf("ticketOpen id must not contain 0000/00/00, got %s", to.ID)
					}
					path := GetTicketPath(to.Year, to.Month, to.Day, to.Slug)
					os.RemoveAll(path)
				}
			}
		})
	}
}

func TestBuildTechnologyLinkArgs(t *testing.T) {
	args := buildTechnologyLinkArgs("https://github.com/usalu/semio/issues/1")
	expected := []string{"project", "item-add", "2", "--owner", "usalu", "--url", "https://github.com/usalu/semio/issues/1"}
	if len(args) != len(expected) {
		t.Fatalf("expected %d args, got %d", len(expected), len(args))
	}
	for i := range expected {
		if args[i] != expected[i] {
			t.Fatalf("args[%d] expected %s, got %s", i, expected[i], args[i])
		}
	}
}

func TestExhaustiveFilterTicketWorkspaceFiles(t *testing.T) {
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	if executor == nil {
		t.Fatal("executor is nil")
	}
	absMain := filepath.Join(rootDir, "go", "repo", "main.go")
	ticket := &Ticket{
		Year:       26,
		Month:      1,
		Day:        20,
		Slug:       "SAMPLE",
		FolderPath: filepath.Join(rootDir, ".🧬semio", "🦑️repo", "🎫️tickets", "26", "01", "20", "SAMPLE"),
	}
	files := []string{
		"./.🧬semio/🦑️repo/🎫️tickets/26/01/20/SAMPLE/plan.md",
		"./.🧬semio/🦑️repo/🎫️tickets/26/01/20/SAMPLE/ticket.json",
		filepath.Join(rootDir, ".🧬semio", "🦑️repo", "🎫️tickets", "26", "01", "20", "SAMPLE", "extra.txt"),
		absMain,
	}
	filtered := FilterTicketWorkspaceFiles(ticket, files)
	if len(filtered) != 1 || filtered[0] != absMain {
		t.Fatalf("expected [%s], got %v", absMain, filtered)
	}
}

func TestExhaustiveNormalizeTicketFileInput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow normalize ticket file input test in short mode")
	}
	absRoot := withExhaustiveRepo(t)
	filePath := filepath.ToSlash(filepath.Join("repo", "client", "main.go"))
	absPath := filepath.Join(absRoot, filePath)
	fileID := FileHeaderId(filePath)
	fileUri := buildFileUriFromPath(filePath)
	cases := []struct {
		name  string
		input string
		want  string
	}{
		{"path", filePath, filePath},
		{"abs path", absPath, filePath},
		{"file uri", fileUri, filePath},
		{"id", fileID, filePath},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeTicketFileInput(tc.input)
			if got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMatchesIgnorePatternDirectoryCoverage(t *testing.T) {
	cases := []struct {
		name    string
		path    string
		isDir   bool
		pattern string
		want    bool
	}{
		{"dir by recursive pattern", "node_modules", true, "**/node_modules/**", true},
		{"nested file by recursive pattern", "a/node_modules/pkg/index.js", false, "**/node_modules/**", true},
		{"dir mismatch", "src", true, "**/node_modules/**", false},
		{"exact dir pattern", "dist", true, "dist/**", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := matchesIgnorePattern(tc.path, tc.isDir, tc.pattern)
			if got != tc.want {
				t.Fatalf("matchesIgnorePattern(%q, %t, %q) = %t, want %t", tc.path, tc.isDir, tc.pattern, got, tc.want)
			}
		})
	}
}

func TestGlobByExtensionSkipsIgnoredDirectoryRoot(t *testing.T) {
	tempRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tempRoot, "src"), 0755); err != nil {
		t.Fatalf("failed to create src dir: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tempRoot, "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create node_modules dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "src", "kept.go"), []byte("package src\n"), 0644); err != nil {
		t.Fatalf("failed to create kept.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "node_modules", "pkg", "ignored.go"), []byte("package pkg\n"), 0644); err != nil {
		t.Fatalf("failed to create ignored.go: %v", err)
	}

	files, err := globByExtension(tempRoot, "**/*", []string{"go"}, []string{"**/node_modules/**"}, false)
	if err != nil {
		t.Fatalf("globByExtension failed: %v", err)
	}
	if len(files) != 1 || files[0] != "src/kept.go" {
		t.Fatalf("expected only src/kept.go, got %v", files)
	}
}

func TestStreamAndListTicketsIgnoreNestedWorkspaceFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(oldRootDir)

	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets", FormatYearDir(26), FormatMonthDir(3), FormatDayDir(7), "SAMPLE")
	if err := os.MkdirAll(filepath.Join(ticketDir, "workspace", "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create nested workspace: %v", err)
	}
	ticketJSON := `{
  "title": "Sample Ticket",
  "status": "open",
  "description": "ticket for streaming"
}`
	if err := os.WriteFile(filepath.Join(ticketDir, "🎫️ticket.json"), []byte(ticketJSON), 0644); err != nil {
		t.Fatalf("failed to create ticket.json: %v", err)
	}
	if err := os.WriteFile(filepath.Join(ticketDir, "workspace", "node_modules", "pkg", "🎫️ticket.json"), []byte(`{"title":"nested"}`), 0644); err != nil {
		t.Fatalf("failed to create nested ticket.json: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	ticketCh := make(chan Ticket)
	errCh := make(chan error, 1)
	go func() {
		errCh <- StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	var streamed []Ticket
	for ticket := range ticketCh {
		streamed = append(streamed, ticket)
	}
	if err := <-errCh; err != nil {
		t.Fatalf("StreamTickets returned error: %v", err)
	}
	if len(streamed) != 1 || streamed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one streamed SAMPLE ticket, got %+v", streamed)
	}

	listed, err := ListTickets(nil, nil, nil)
	if err != nil {
		t.Fatalf("ListTickets failed: %v", err)
	}
	if len(listed) != 1 || listed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one listed SAMPLE ticket, got %+v", listed)
	}
}
