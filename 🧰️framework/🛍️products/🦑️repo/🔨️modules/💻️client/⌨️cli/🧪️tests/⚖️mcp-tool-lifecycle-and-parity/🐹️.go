// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP tool lifecycles, ticket path parsing, and CLI/tool parity.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
	"unicode"
)

func executeCommandMd(args ...string) (string, string, error) {
	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(stdout)
	root.SetErr(stderr)
	root.SetArgs(args)
	config.Format = "md"
	err := root.Execute()
	if err != nil {
		fmt.Fprintln(stderr, err)
	}
	return stdout.String(), stderr.String(), err
}

func toolOutputText(result ToolResult) string {
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return strings.Join(lines, "\n")
}

var relativeTimePattern = regexp.MustCompile(`\b(opened |closed |created )?(a long while ago|\d+ (?:second|minute|hour|day|week|month|year)s? (?:ago|from now))\b`)

func normalizeRelativeTimes(s string) string {
	return relativeTimePattern.ReplaceAllString(s, "<TIME>")
}

func TestParseTicketPath(t *testing.T) {
	tests := []struct {
		name    string
		path    string
		year    int
		month   int
		day     int
		slug    string
		wantErr bool
	}{
		{"two-digit year", "26/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"four-digit year normalized", "2026/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"nested slug", "26/03/27/PARENT/CHILD", 26, 3, 27, "PARENT/CHILD", false},
		{"too few parts", "26/03", 0, 0, 0, "", true},
		{"empty slug", "26/03/27/", 0, 0, 0, "", true},
		{"non-numeric year", "abc/03/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric month", "26/abc/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric day", "26/03/abc/SLUG", 0, 0, 0, "", true},
		{"empty string", "", 0, 0, 0, "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			y, m, d, s, err := parseTicketPath(tt.path)
			if (err != nil) != tt.wantErr {
				t.Fatalf("parseTicketPath(%q) error = %v, wantErr %v", tt.path, err, tt.wantErr)
			}
			if !tt.wantErr {
				if y != tt.year || m != tt.month || d != tt.day || s != tt.slug {
					t.Errorf("parseTicketPath(%q) = (%d,%d,%d,%q), want (%d,%d,%d,%q)", tt.path, y, m, d, s, tt.year, tt.month, tt.day, tt.slug)
				}
			}
		})
	}
}

func TestExhaustiveMcpTicketCloseAutoResolve(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close auto-resolve test in short mode")
	}
	withExhaustiveRepo(t)

	result := ToolTicketOpen("🎫️", "Test Auto Resolve Close", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "cleanup", []string{"repo/client/package.json"}, "", true)
		os.RemoveAll(GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	year, month, day, slug, err := resolveTicketForClose("")
	if err != nil {
		t.Fatalf("resolveTicketForClose('') error: %v", err)
	}
	resolved, err := ReadTicket(year, month, day, slug)
	if err != nil {
		t.Fatalf("resolved ticket not readable: %v", err)
	}
	if resolved.Status != TicketStatusOpen {
		t.Errorf("resolveTicketForClose('') resolved to non-open ticket (status=%s)", resolved.Status)
	}
}

func TestExhaustiveMcpTicketCloseWithFullYearPath(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close full-year-path test in short mode")
	}
	withExhaustiveRepo(t)

	result := ToolTicketOpen("🎫️", "Test Full Year Path", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		os.RemoveAll(GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	fullYearPath := fmt.Sprintf("%d/%02d/%02d/%s", 2000+ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	year, month, day, slug, err := parseTicketPath(fullYearPath)
	if err != nil {
		t.Fatalf("parseTicketPath(%q) error: %v", fullYearPath, err)
	}

	closeResult := ToolTicketClose(year, month, day, slug, "Test summary", []string{"repo/client/package.json"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose with full year path returned error: %s", closeResult.Error)
	}
}

func TestToolDraftLifecycle(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolDraftCreate("test-mcp-draft", nil)
	if result.Error != "" {
		t.Fatalf("ToolDraftCreate returned error: %s", result.Error)
	}

	listResult := ToolDraftList()
	if listResult.Error != "" {
		t.Fatalf("ToolDraftList returned error: %s", listResult.Error)
	}

	deleteResult := ToolDraftDelete("test-mcp-draft")
	if deleteResult.Error != "" {
		t.Fatalf("ToolDraftDelete returned error: %s", deleteResult.Error)
	}
}

// ♻️TestRepoContextTodoChange MUST rewrite the matching TODO source line and return an error for an unknown ID.
// ✏️TestRepoContextTodoChange verifies the repoContext TodoChange mutation against a temp-dir-isolated fixture.
func TestRepoContextTodoChange(t *testing.T) {
	root := t.TempDir()

	mdDir := filepath.Join(root, "sub")
	if err := os.MkdirAll(mdDir, 0755); err != nil {
		t.Fatal(err)
	}
	todosPath := filepath.Join(mdDir, ".todos.md")
	if err := os.WriteFile(todosPath, []byte("- TODO OriginalName: original description\n"), 0644); err != nil {
		t.Fatal(err)
	}

	codePath := filepath.Join(root, "main.go")
	if err := os.WriteFile(codePath, []byte("package main\n\n// TODO CodeTodo: code description\nfunc main() {}\n"), 0644); err != nil {
		t.Fatal(err)
	}

	ctx := &repoContext{rootDir: root}

	name := "RenamedName"
	description := "renamed description"
	mdID := Slugify("OriginalName")
	changed, err := ctx.TodoChange(TodoChangeInput{ID: mdID, Name: &name, Description: &description})
	if err != nil {
		t.Fatalf("markdown TodoChange returned error: %v", err)
	}
	if changed.Name != "RenamedName" || changed.Description != "renamed description" {
		t.Fatalf("unexpected changed todo: %+v", changed)
	}
	content, _ := os.ReadFile(todosPath)
	if !strings.Contains(string(content), "- TODO RenamedName: renamed description") {
		t.Fatalf("markdown file not updated, got: %s", content)
	}
	if strings.Contains(string(content), "OriginalName") {
		t.Fatalf("old name still present: %s", content)
	}

	codeName := "CodeTodoRenamed"
	codeDescription := "new code description"
	codeID := Slugify("CodeTodo")
	changedCode, err := ctx.TodoChange(TodoChangeInput{ID: codeID, Name: &codeName, Description: &codeDescription})
	if err != nil {
		t.Fatalf("inline TodoChange returned error: %v", err)
	}
	if changedCode.Name != "CodeTodoRenamed" || changedCode.Description != "new code description" {
		t.Fatalf("unexpected changed code todo: %+v", changedCode)
	}
	codeContent, _ := os.ReadFile(codePath)
	if !strings.Contains(string(codeContent), "// TODO CodeTodoRenamed: new code description") {
		t.Fatalf("code file not updated, got: %s", codeContent)
	}

	missingName := "X"
	if _, err := ctx.TodoChange(TodoChangeInput{ID: "does-not-exist", Name: &missingName}); err == nil {
		t.Fatal("expected error for missing todo")
	}
}

func TestToolGoalUri(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*Goal)
	if !ok || len(goals) == 0 {
		t.Skip("no goals to verify URI")
	}
	for _, g := range goals {
		uri := g.GetURI()
		if uri == "" {
			t.Errorf("goal %s has empty URI", g.ID)
		}
		if !strings.HasPrefix(uri, "repo://goal/") {
			t.Errorf("goal %s URI %q should start with repo://goal/", g.ID, uri)
		}
	}
}

// ⛳️#region 🕹️Output Parity
func TestExhaustiveParityGoalList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity goal list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-goal")
		if err != nil {
			t.Fatalf("CLI goal list failed: %v", err)
		}
		toolResult := ToolGoalList()
		if toolResult.Error != "" {
			t.Fatalf("ToolGoalList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of goals", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		toolResult := ToolGoalList()
		mcpOut := toolOutputText(toolResult)
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})

	t.Run("empty output when no goals match filter", func(t *testing.T) {

		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		mcpOut := toolOutputText(ToolGoalList())
		if len(cliOut) == 0 && len(mcpOut) != 0 {
			t.Error("CLI produced empty output but MCP did not")
		}
		if len(cliOut) != 0 && len(mcpOut) == 0 {
			t.Error("MCP produced empty output but CLI did not")
		}
	})
}

func TestExhaustiveParityContributorList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity contributor list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-contributor")
		if err != nil {
			t.Fatalf("CLI contributor list failed: %v", err)
		}
		toolResult := ToolContributorList()
		if toolResult.Error != "" {
			t.Fatalf("ToolContributorList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of contributors", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-contributor")
		mcpOut := toolOutputText(ToolContributorList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestExhaustiveParityTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity ticket list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-ticket")
		if err != nil {
			t.Fatalf("CLI ticket list failed: %v", err)
		}
		toolResult := ToolTicketList(nil, nil, nil)
		if toolResult.Error != "" {
			t.Fatalf("ToolTicketList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of tickets", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-ticket")
		mcpOut := toolOutputText(ToolTicketList(nil, nil, nil))
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestParityDraftList(t *testing.T) {
	setupToolTest(t)

	t.Run("uses event rendering format", func(t *testing.T) {
		toolResult := ToolDraftList()
		if toolResult.Error != "" {
			t.Fatalf("ToolDraftList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)

		drafts, _ := ListDrafts()
		if len(drafts) > 0 && mcpOut == "" {
			t.Error("ToolDraftList returned empty output despite having drafts")
		}
		if len(drafts) == 0 && mcpOut != "" {
			t.Error("ToolDraftList returned output despite having no drafts")
		}
	})

	t.Run("renders same as manual event rendering", func(t *testing.T) {
		drafts, err := ListDrafts()
		if err != nil {
			t.Fatalf("ListDrafts failed: %v", err)
		}
		var events []Event
		for _, d := range drafts {
			data, _ := json.Marshal(map[string]interface{}{"draft": d})
			events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
		}
		expected := renderEventsToMarkdown(events)
		actual := toolOutputText(ToolDraftList())
		if expected != actual {
			t.Errorf("output mismatch:\nexpected:\n%s\nactual:\n%s", expected, actual)
		}
	})
}

func TestExhaustiveParityTechnologyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology list failed: %v", err)
		}
		toolResult := ToolTechnologyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return non-empty output", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-technology")
		mcpOut := toolOutputText(ToolTechnologyList())
		if len(cliOut) == 0 {
			t.Error("CLI technology list returned empty output")
		}
		if len(mcpOut) == 0 {
			t.Error("MCP technology list returned empty output")
		}
	})
}

func TestExhaustiveParityTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology tree test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("search", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology tree failed: %v", err)
		}
		toolResult := ToolTechnologyTree(context.Background())
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyTree returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("technologies are sorted alphabetically", func(t *testing.T) {
		mcpOut := toolOutputText(ToolTechnologyTree(context.Background()))
		lines := strings.Split(strings.TrimSpace(mcpOut), "\n")
		var technologyNames []string
		for _, line := range lines {
			trimmed := strings.TrimSpace(line)
			if idx := strings.Index(trimmed, "repo://technology/"); idx >= 0 {
				rest := trimmed[idx+len("repo://technology/"):]
				endParen := strings.Index(rest, ")")
				if endParen >= 0 {
					technologyNames = append(technologyNames, strings.TrimLeftFunc(rest[:endParen], func(r rune) bool { return !unicode.IsLetter(r) && !unicode.IsDigit(r) }))
				}
			}
		}
		for i := 1; i < len(technologyNames); i++ {
			if technologyNames[i] < technologyNames[i-1] {
				t.Errorf("technologies not sorted: %q comes after %q", technologyNames[i], technologyNames[i-1])
			}
		}
	})
}

func TestExhaustiveParityPolicyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity policy list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-policy")
		if err != nil {
			t.Fatalf("CLI policy list failed: %v", err)
		}
		toolResult := ToolPolicyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolPolicyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of policies", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-policy")
		mcpOut := toolOutputText(ToolPolicyList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}
