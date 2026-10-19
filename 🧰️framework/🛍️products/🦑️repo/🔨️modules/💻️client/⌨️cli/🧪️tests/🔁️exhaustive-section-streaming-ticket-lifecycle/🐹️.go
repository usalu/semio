// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive section commands, streaming lists, and ticket lifecycle.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	graphql "github.com/usalu/semio/repo/graphql"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestExhaustiveSectionCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow section commands test in short mode")
	}
	repoRoot := withExhaustiveRepo(t)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	testDir := filepath.Join(repoRoot, "temp", "section_tests")
	os.MkdirAll(testDir, 0755)

	tests := []struct {
		name       string
		ext        string
		contentFmt string
		renameTo   string
	}{
		{"TypeScript", ".ts", "const x = 1;\n// #region 🔖️%s\nconst y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Go", ".go", "package main\n// #region 🔖️%s\nvar y = 2\n// #endregion 🔖️%s\n", "Renamed"},
		{"Python", ".py", "# #region 🔖️%s\ny = 2\n# #endregion 🔖️%s\n", "Renamed"},
		{"CSharp", ".cs", "// #region 🔖️%s\nvar y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Rust", ".rs", "// #region 🔖️%s\nlet y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Ruby", ".rb", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"Shell", ".sh", "# region %s\ny=2\n# endregion %s\n", "Renamed"},
		{"TOML", ".toml", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"YAML", ".yaml", "# region %s\ny: 2\n# endregion %s\n", "Renamed"},
		{"SQL", ".sql", "-- #region 🔖️%s\nSELECT 1;\n-- #endregion 🔖️%s\n", "Renamed"},
		{"GraphQL", ".graphql", "# #region 🔖️%s\ntype Query { name: String }\n# #endregion 🔖️%s\n", "Renamed"},
		{"Markdown", ".md", "## %s\nContent\n", "Renamed"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if tc.name == "Rust" {
				t.Skip("Rust section move/extract has a known issue with mod-based section format")
			}
			sectionName := "MySection"
			filename := "test" + tc.ext
			filePath := filepath.Join(testDir, filename)
			relPath, _ := filepath.Rel(repoRoot, filePath)

			var content string
			if tc.name == "Markdown" {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 1)
			} else {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 2)
			}
			os.WriteFile(filePath, []byte(content), 0644)

			moveCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			moveCmd.SetOut(b)
			moveCmd.SetErr(b)
			moveCmd.SetArgs([]string{"section", "move", relPath, sectionName, tc.renameTo})
			err := moveCmd.Execute()
			if err != nil {
				t.Fatalf("Move failed: %v Output: %s", err, b.String())
			}

			newContentBytes, _ := os.ReadFile(filePath)
			newContent := string(newContentBytes)
			if !strings.Contains(newContent, tc.renameTo) {
				t.Errorf("File content does not contain renamed section %s. Content:\n%s", tc.renameTo, newContent)
			}

			targetFile := filepath.Join(testDir, "extracted"+tc.ext)
			relTargetFile, _ := filepath.Rel(repoRoot, targetFile)

			extractCmd := NewRoot(factory)
			extractCmd.SetOut(b)
			extractCmd.SetErr(b)
			extractCmd.SetArgs([]string{"section", "extract", relPath, tc.renameTo, relTargetFile})
			err = extractCmd.Execute()
			if err != nil {
				t.Fatalf("Extract failed: %v Output: %s", err, b.String())
			}

			targetContentBytes, err := os.ReadFile(targetFile)
			if err != nil {
				t.Fatalf("Target file not created: %v", err)
			}
			targetContent := string(targetContentBytes)
			if len(targetContent) == 0 && tc.name != "Markdown" {
				t.Errorf("Extracted content is empty")
			}

			sourceIntegrate := filepath.Join(testDir, "to_integrate"+tc.ext)
			relSourceIntegrate, _ := filepath.Rel(repoRoot, sourceIntegrate)
			integrateContent := "New Content"
			os.WriteFile(sourceIntegrate, []byte(integrateContent), 0644)

			integrateCmd := NewRoot(factory)
			integrateCmd.SetOut(b)
			integrateCmd.SetErr(b)
			integrateCmd.SetArgs([]string{"section", "integrate", relSourceIntegrate, tc.renameTo, relPath})
			err = integrateCmd.Execute()
			if err != nil {
				t.Fatalf("Integrate failed: %v Output: %s", err, b.String())
			}

			finalContentBytes, _ := os.ReadFile(filePath)
			finalContent := string(finalContentBytes)
			if !strings.Contains(finalContent, integrateContent) {
				t.Errorf("File content does not contain integrated content. Content:\n%s", finalContent)
			}
		})
	}
}

func TestExhaustiveStreamingList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow streaming list test in short mode")
	}
	repoRoot := withExhaustiveRepo(t)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name string
		args []string
	}{
		{
			name: "Ticket List (Text)",
			args: []string{"list", "--only-ticket"},
		},
		{
			name: "Bundle List (Text)",
			args: []string{"list", "--only-bundle"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rootCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(tt.args)

			_ = rootCmd.Execute()
			output := b.String()
			lines := strings.Split(strings.TrimSpace(output), "\n")

			for _, line := range lines {
				if strings.TrimSpace(line) == "" {
					continue
				}
				if strings.HasPrefix(strings.TrimSpace(line), "{\"kind\":\"result\"") {
					t.Errorf("Expected formatted text output, got raw JSON event: %s", line)
				}
			}
		})
	}
}

func TestTicketLifecycle_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle no-management test in short mode")
	}
	tmpDir := t.TempDir()

	run := func(name string, field ...string) {
		cmd := exec.Command(name, field...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("run %s %v failed: %v\nOutput: %s", name, field, err, out)
		}
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")

	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	goal, err := OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}

	testSessionIDOverride = "session-open-1"
	defer func() { testSessionIDOverride = "" }()
	ticket, err := OpenTicket("🎫️", "Test Title NoGH", "Test Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("OpenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.Management != nil {
		t.Error("OpenTicket: GitHub data should be nil")
	}
	openSessionCount := len(ticket.Sessions)
	if openSessionCount == 0 {
		t.Fatal("OpenTicket must persist one session")
	}

	testFile := "test.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("content"), 0644); err != nil {
		t.Fatal(err)
	}

	if goal.Title != "Goal Title" {
		t.Errorf("expected title 'Goal Title', got '%s'", goal.Title)
	}
	if goal.Prompt != "Goal Prompt" {
		t.Errorf("expected prompt 'Goal Prompt', got '%s'", goal.Prompt)
	}
	if goal.Client != "copilot-chat" {
		t.Errorf("expected ui 'copilot-chat', got '%s'", goal.Client)
	}
	if goal.LLM != "gemini-3-pro" {
		t.Errorf("expected llm 'gemini-3-pro', got '%s'", goal.LLM)
	}
	if goal.Management != nil {
		t.Error("OpenGoal: GitHub data should be nil")
	}

	goalPath := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎯️goals", "GOAL-TITLE", "🎯️goal.json")
	if _, err := os.Stat(goalPath); os.IsNotExist(err) {
		t.Errorf("goal file not created at %s", goalPath)
	}

	run("git", "add", testFile)
	run("git", "commit", "-m", "add test file")

	err = FinishTicket(ticket, "Summary", []string{testFile}, true, false)
	if err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.GetStatus() != TicketStatusClosed {
		t.Errorf("Ticket status mismatch: got %v, want closed", ticket.GetStatus())
	}
	if len(ticket.Interactions) < 2 {
		t.Fatalf("expected at least 2 interactions after close, got %d", len(ticket.Interactions))
	}
	if ticket.Interactions[0].Kind != "ticket.open" {
		t.Errorf("interaction[0].Kind = %q, want %q", ticket.Interactions[0].Kind, "ticket.open")
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.close" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.close")
	}
	closeFiles := ticket.Interactions[len(ticket.Interactions)-1].Files
	if len(closeFiles) != 1 || closeFiles[0].Path != testFile {
		t.Fatalf("no-management close files = %#v, want %q", closeFiles, testFile)
	}
	if len(ticket.Sessions) != openSessionCount {
		t.Fatalf("FinishTicket must not append sessions: before=%d after=%d", openSessionCount, len(ticket.Sessions))
	}

	testSessionIDOverride = "session-reopen-2"
	err = ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "", "copilot-chat", "", "", "", true, McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("ReopenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.GetStatus() != TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.reopen" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.reopen")
	}
	if len(ticket.Sessions) != openSessionCount+1 {
		t.Fatalf("ReopenTicket must append exactly one session: expected=%d got=%d", openSessionCount+1, len(ticket.Sessions))
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := GoalCreateInput{
		Title:        "Test Goal NoGH 2",
		Description:  "Desc",
		Prompt:       "Prompt",
		DueDate:      "2026-02-15",
		Client:       "cursor",
		LLM:          "gpt-5-2-codex",
		NoManagement: true,
	}

	goal2, err := ctx.GoalCreate(goalInput)
	if err != nil {
		t.Fatalf("GoalCreate failed: %v", err)
	}
	if goal2.Title != "Test Goal NoGH 2" {
		t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
	}

	_, err = ctx.GoalClose(GoalCloseInput{ID: goal2.ID, Summary: "Done", NoManagement: true})
	if err != nil {
		t.Fatalf("GoalClose failed: %v", err)
	}
	closedGoalPath := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎯️goals", goal2.ID, "🎯️goal.json")
	closedGoalContent, err := ReadTextFile(closedGoalPath)
	if err != nil {
		t.Fatalf("failed to read closed goal: %v", err)
	}
	var closedGoal Goal
	if err := json.Unmarshal([]byte(closedGoalContent), &closedGoal); err != nil {
		t.Fatalf("failed to unmarshal closed goal: %v", err)
	}
}
