// 🔬️ Tests of the todos domain, split out of the pre-split godfile suite.

package todos

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestMergeTicketAgentPlanSteps(t *testing.T) {
	second1 := "2026-03-02T10:00:00Z"
	second2 := "2026-03-02T10:01:00Z"

	t.Run("empty existing adopts incoming", func(t *testing.T) {
		incoming := []model.HookPlanStep{
			{Name: "A", Status: "in-progress"},
			{Name: "B", Status: "pending"},
		}
		merged := MergeTicketAgentPlanSteps(nil, incoming, second1)
		if len(merged) != 2 {
			t.Fatalf("expected 2, got %d", len(merged))
		}
		if merged[0].Ideated != second1 {
			t.Errorf("expected A ideated=%s, got %s", second1, merged[0].Ideated)
		}
		if merged[0].Started != second1 {
			t.Errorf("expected A started=%s, got %s", second1, merged[0].Started)
		}
		if merged[1].Ideated != second1 || merged[1].Started != "" || merged[1].Completed != "" {
			t.Errorf("expected B pending, got %+v", merged[1])
		}
	})

	t.Run("preserves existing timestamps", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
		}
		incoming := []model.HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if merged[0].Ideated != second1 {
			t.Errorf("expected ideated preserved as %s, got %s", second1, merged[0].Ideated)
		}
		if merged[0].Started != second1 {
			t.Errorf("expected started preserved as %s, got %s", second1, merged[0].Started)
		}
		if merged[0].Completed != second2 {
			t.Errorf("expected completed=%s, got %s", second2, merged[0].Completed)
		}
	})

	t.Run("marks removed never-started steps as abandoned", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
			{Name: "B"},
		}
		incoming := []model.HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A abandoned), got %d", len(merged))
		}
		if merged[0].Name != "B" {
			t.Errorf("expected B first, got %s", merged[0].Name)
		}
		abandoned := merged[1]
		if abandoned.Name != "A" || abandoned.Ideated != second1 || abandoned.Started != "" || abandoned.Abandoned != second2 {
			t.Errorf("expected A abandoned at %s, got %+v", second2, abandoned)
		}
	})

	t.Run("does not abandon removed started steps", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
			{Name: "B"},
		}
		incoming := []model.HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A historical), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Started != second1 || historical.Abandoned != "" {
			t.Errorf("expected A started history preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("preserves already abandoned steps", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Abandoned: second1},
		}
		incoming := []model.HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		foundA := 0
		for _, s := range merged {
			if s.Name == "A" {
				foundA++
				if s.Abandoned != second1 {
					t.Errorf("expected A abandoned timestamp unchanged, got %s", s.Abandoned)
				}
			}
		}
		if foundA != 1 {
			t.Errorf("expected abandoned step A to be preserved exactly once, got %d", foundA)
		}
	})

	t.Run("does not abandon completed steps removed from incoming", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Completed: second1},
			{Name: "B", Started: second1},
		}
		incoming := []model.HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A completed history), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Completed != second1 || historical.Abandoned != "" {
			t.Errorf("expected completed A preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("adds new steps not in existing", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1, Completed: second1},
		}
		incoming := []model.HookPlanStep{
			{Name: "A", Status: "completed"},
			{Name: "C", Status: "pending"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(merged))
		}
		if merged[1].Name != "C" || merged[1].Ideated != second2 || merged[1].Started != "" {
			t.Errorf("expected C as new pending step, got %+v", merged[1])
		}
	})

	t.Run("does not set completed for step never started", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
		}
		incoming := []model.HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 1 {
			t.Fatalf("expected 1 step, got %d", len(merged))
		}
		if merged[0].Completed != "" {
			t.Errorf("expected no completed timestamp when step was never started, got %+v", merged[0])
		}
	})

	t.Run("backfills legacy name-only steps with lifecycle dates", func(t *testing.T) {
		existing := []model.TicketAgentPlanStep{
			{Name: "Legacy Pending"},
		}
		incoming := []model.HookPlanStep{
			{Name: "Legacy Pending", Status: "pending"},
			{Name: "New Pending", Status: "pending"},
		}
		merged := MergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(merged))
		}
		for _, step := range merged {
			if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
				t.Errorf("expected non-empty lifecycle on step %q, got %+v", step.Name, step)
			}
		}
	})
}

func TestDeriveRepoOpFromMCPTool(t *testing.T) {
	cases := []struct {
		name     string
		tool     string
		expected string
	}{
		{"ticket open", "mcp__repo__ticket_open", "ticket.open"},
		{"ticket close", "mcp__repo__ticket_close", "ticket.close"},
		{"ticket reopen", "mcp__repo__ticket_reopen", "ticket.reopen"},
		{"ticket read", "mcp__repo__ticket_read", "ticket.read"},
		{"goal open", "mcp__repo__goal_open", "goal.open"},
		{"goal close", "mcp__repo__goal_close", "goal.close"},
		{"goal reopen", "mcp__repo__goal_reopen", "goal.reopen"},
		{"contributor add", "mcp__repo__contributor_add", "contributor.add"},
		{"contributor remove", "mcp__repo__contributor_remove", "contributor.remove"},
		{"draft create", "mcp__repo__draft_create", "draft.create"},
		{"draft delete", "mcp__repo__draft_delete", "draft.delete"},
		{"file create", "mcp__repo__file_create", "file.create"},
		{"file move", "mcp__repo__file_move", "file.move"},
		{"file delete", "mcp__repo__file_delete", "file.delete"},
		{"folder create", "mcp__repo__folder_create", "folder.create"},
		{"folder move", "mcp__repo__folder_move", "folder.move"},
		{"folder delete", "mcp__repo__folder_delete", "folder.delete"},
		{"section create", "mcp__repo__section_create", "section.create"},
		{"section move", "mcp__repo__section_move", "section.move"},
		{"section delete", "mcp__repo__section_delete", "section.delete"},
		{"integrate", "mcp__repo__integrate", "integrate"},
		{"extract", "mcp__repo__extract", "extract"},
		{"export", "mcp__repo__export", "export"},
		{"analyze", "mcp__repo__analyze", "analyze"},
		{"fix", "mcp__repo__fix", "fix"},
		{"tree", "mcp__repo__tree", "tree"},
		{"graphql", "mcp__repo__graphql", "graphql"},
		{"move", "mcp__repo__move", "move"},
		{"policy check", "mcp__repo__policy_check", "policy.check"},
		{"non-compose tool", "mcp__other__thing", ""},
		{"bash tool", "Bash", ""},
		{"empty", "", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := DeriveRepoOpFromMCPTool(tc.tool)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromMCPTool(%q) = %q, want %q", tc.tool, got, tc.expected)
			}
		})
	}
}

func TestDeriveRepoOpFromCLICommand(t *testing.T) {
	cases := []struct {
		name     string
		cmd      string
		expected string
	}{
		{"ticket open full path", "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket open exe path", ".\\repo\\cli\\cli.exe ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket close", "./cli ticket close 26 03 05 MY-SLUG 'Summary' compose/go/compose.go", "ticket.close"},
		{"ticket reopen", "/workspaces/semio/repo/client/client ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"ticket reopen go run", "go run ./repo/client/mcp/go ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"goal open", "./cli goal open 'Title' 'Desc' 'Prompt' claude-code sonnet-4-5", "goal.open"},
		{"goal close", "./cli goal close MY-GOAL 'Summary'", "goal.close"},
		{"contributor add", "./cli contributor add github-user", "contributor.add"},
		{"single subcommand", "./cli analyze", "analyze"},
		{"non-cli binary", "./other-tool ticket open", ""},
		{"empty", "", ""},
		{"just cli", "./cli", ""},
		{"cli with flag first", "./cli -v ticket open", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := DeriveRepoOpFromCLICommand(tc.cmd)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromCLICommand(%q) = %q, want %q", tc.cmd, got, tc.expected)
			}
		})
	}
}

func TestClassifyTool(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		expect   model.ToolKind
	}{
		{"manage_todo_list", "manage_todo_list", model.ToolKindPlan},
		{"Task", "Task", model.ToolKindPlan},
		{"todo_tool", "todo_tool", model.ToolKindPlan},
		{"TodoWrite", "TodoWrite", model.ToolKindPlan},
		{"read_file", "read_file", model.ToolKindCodeSearch},
		{"grep_search", "grep_search", model.ToolKindCodeSearch},
		{"rg", "rg", model.ToolKindCodeSearch},
		{"ripgrep", "ripgrep", model.ToolKindCodeSearch},
		{"file_search", "file_search", model.ToolKindCodeSearch},
		{"semantic_search", "semantic_search", model.ToolKindCodeSearch},
		{"list_dir", "list_dir", model.ToolKindCodeSearch},
		{"get_errors", "get_errors", model.ToolKindCodeSearch},
		{"Read", "Read", model.ToolKindCodeSearch},
		{"replace_string_in_file", "replace_string_in_file", model.ToolKindCodeEdit},
		{"create_file", "create_file", model.ToolKindCodeEdit},
		{"multi_replace_string_in_file", "multi_replace_string_in_file", model.ToolKindCodeEdit},
		{"Edit", "Edit", model.ToolKindCodeEdit},
		{"Write", "Write", model.ToolKindCodeEdit},
		{"run_in_terminal", "run_in_terminal", model.ToolKindTerminal},
		{"get_terminal_output", "get_terminal_output", model.ToolKindTerminal},
		{"Bash", "Bash", model.ToolKindTerminal},
		{"runSubagent", "runSubagent", model.ToolKindGeneric},
		{"runTests", "runTests", model.ToolKindTest},
		{"run_tests", "run_tests", model.ToolKindTest},
		{"run_task", "run_task", model.ToolKindBuild},
		{"create_and_run_task", "create_and_run_task", model.ToolKindBuild},
		{"fetch_webpage", "fetch_webpage", model.ToolKindCodeSearch},
		{"open_simple_browser", "open_simple_browser", model.ToolKindCodeSearch},
		{"Glob", "Glob", model.ToolKindCodeSearch},
		{"tool_search_tool_regex", "tool_search_tool_regex", model.ToolKindGeneric},
		{"fs_read", "fs_read", model.ToolKindCodeSearch},
		{"fs_write", "fs_write", model.ToolKindCodeEdit},
		{"execute_bash", "execute_bash", model.ToolKindTerminal},
		{"code", "code", model.ToolKindCodeSearch},
		{"grep_kiro", "grep", model.ToolKindCodeSearch},
		{"glob_kiro", "glob", model.ToolKindCodeSearch},
		{"web_search", "web_search", model.ToolKindCodeSearch},
		{"web_fetch", "web_fetch", model.ToolKindCodeSearch},
		{"use_subagent", "use_subagent", model.ToolKindGeneric},
		{"use_aws", "use_aws", model.ToolKindGeneric},
		{"empty", "", model.ToolKindGeneric},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := ClassifyTool(tc.toolName)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestResolveAllTestDefinitionIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

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

func TestExtractPlanStepsFromInput(t *testing.T) {
	t.Run("todoList format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"todoList":[{"title":"Task A","status":"completed"},{"title":"Task B","status":"in-progress"}]}}`)
		steps := ExtractPlanStepsFromInput(input, "")
		if len(steps) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(steps))
		}
		if steps[0].Name != "Task A" || steps[0].Status != "completed" {
			t.Errorf("unexpected step 0: %+v", steps[0])
		}
	})
	t.Run("steps format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"steps":[{"name":"Build","status":"pending"}]}}`)
		steps := ExtractPlanStepsFromInput(input, "")
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "Build" {
			t.Errorf("expected name=Build, got %s", steps[0].Name)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		steps := ExtractPlanStepsFromInput(nil, `{"todoList":[{"title":"FromArgs","status":"done"}]}`)
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "FromArgs" {
			t.Errorf("expected name=FromArgs, got %s", steps[0].Name)
		}
	})
	t.Run("empty", func(t *testing.T) {
		steps := ExtractPlanStepsFromInput(nil, "")
		if steps != nil {
			t.Errorf("expected nil steps, got %v", steps)
		}
	})
}

func TestExtractSearchFromInput(t *testing.T) {
	t.Run("grep_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("file_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"**/*.ts"}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("read_file style", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		if err := os.WriteFile(tempFile, []byte("one\ntwo\nthree\n"), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"filePath":%q}}`, tempFile))
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		pages, ranges := ExtractSearchFromInput(nil, `{"query":"fromArgs"}`)
		if len(pages) != 0 {
			t.Errorf("expected no webpages from non-url query, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("webpages only", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"url":"https://example.com/docs","pages":["https://compose.dev","not-a-url"]}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
		if len(pages) != 2 || pages[0] != "https://example.com/docs" || pages[1] != "https://compose.dev" {
			t.Errorf("expected only valid webpages, got %v", pages)
		}
	})
	t.Run("read tool with limit only", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"limit":3}}`, tempFile))
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("read tool with offset and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"offset":2,"limit":3}}`, tempFile))
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L2-L4"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format with file_path and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q,"limit":100}}}}`, tempFile))
		_, ranges := ExtractSearchFromInput(input, "")

		want := tempFile + "#L1-L100"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format without limit reads full file", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q}}}}`, tempFile))
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("grep tool with pattern ignores path for line range", func(t *testing.T) {
		tempDir := t.TempDir()
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","path":%q}}`, tempDir))
		_, ranges := ExtractSearchFromInput(input, "")

		if len(ranges) != 0 {
			t.Errorf("grep with pattern+path should produce no file ranges, got %v", ranges)
		}
	})
	t.Run("grep tool with pattern and file_path uses file_path for range", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","file_path":%q}}`, tempFile))
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L1-L2"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
}

func TestExtractToolInputFromStdinNativeFormat(t *testing.T) {
	t.Run("native.event.tool_input extracted", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go","limit":100}}}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input from native format")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["file_path"] != "/tmp/test.go" {
			t.Errorf("expected file_path=/tmp/test.go, got %v", data["file_path"])
		}
	})
	t.Run("direct tool_input still works", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go"}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
	})
}

func TestExtractToolInputFromStdin(t *testing.T) {
	t.Run("with tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test","tool_input":{"key":"val"}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["key"] != "val" {
			t.Errorf("expected key=val, got %v", data["key"])
		}
	})
	t.Run("without tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := ExtractToolInputFromStdin(input)
		if result != nil {
			t.Error("expected nil tool input")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := ExtractToolInputFromStdin(nil)
		if result != nil {
			t.Error("expected nil for empty input")
		}
	})
}

func TestExtractToolResponseFromStdin(t *testing.T) {
	t.Run("tool_output", func(t *testing.T) {
		input := json.RawMessage(`{"tool_output":"response data"}`)
		result := ExtractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("tool_response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_response":"data"}`)
		result := ExtractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("no response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := ExtractToolResponseFromStdin(input)
		if result != nil {
			t.Error("expected nil response")
		}
	})
}

func TestEventIDsUseComposeRepoFormat(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess event id test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	workspace.SetRootDir(tmpDir)

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

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

// 🧲️#endregion 🌡️Technology Generate
func TestExtractSearchFromInputLineNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		toolArgs string
		expected string
	}{
		{
			name: "single line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
		{
			name: "line range",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35540
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490-L35540",
		},
		{
			name: "only start line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, ranges := ExtractSearchFromInput(json.RawMessage(tt.input), tt.toolArgs)
			if len(ranges) != 1 || ranges[0] != tt.expected {
				t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, tt.expected)
			}
		})
	}
}

func TestExtractSearchFromInputCompleteFileRange(t *testing.T) {
	tempFile := filepath.Join(t.TempDir(), "all.go")
	if err := os.WriteFile(tempFile, []byte("a\nb\nc"), 0o644); err != nil {
		t.Fatalf("failed to write temp file: %v", err)
	}

	input := fmt.Sprintf(`{
		"tool_name": "read_file",
		"tool_input": {
			"filePath": %q
		}
	}`, tempFile)

	pages, ranges := ExtractSearchFromInput(json.RawMessage(input), "")
	if len(pages) != 0 {
		t.Errorf("expected no webpages, got %v", pages)
	}
	expected := tempFile + "#L1-L3"
	if len(ranges) != 1 || ranges[0] != expected {
		t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, expected)
	}
}
