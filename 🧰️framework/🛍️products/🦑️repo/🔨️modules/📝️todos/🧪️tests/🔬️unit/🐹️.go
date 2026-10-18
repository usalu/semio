// 🔬️ Tests of the todos domain, split out of the pre-split godfile suite.

package todos

import (
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

// #region 🔬️Hook Tool Tests

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

// #endregion 🔬️Hook Tool Tests
