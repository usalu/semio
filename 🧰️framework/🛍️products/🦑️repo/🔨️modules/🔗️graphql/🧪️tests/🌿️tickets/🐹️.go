// 🎫️ Tests of the ticket lifecycle and the todo mutations of the repository context.

package graphql

import (
	json "encoding/json"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	contributors "github.com/usalu/semio/repo/contributors"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	tickets "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

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

	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	goal, err := tickets.OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}

	contributors.TestSessionIDOverride = "session-open-1"
	defer func() { contributors.TestSessionIDOverride = "" }()
	ticket, err := tickets.OpenTicket("🎫️", "Test Title NoGH", "Test Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", providers.McpClientGeneric, "", "")
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

	err = tickets.FinishTicket(ticket, "Summary", []string{testFile}, true, false)
	if err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.GetStatus() != model.TicketStatusClosed {
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

	contributors.TestSessionIDOverride = "session-reopen-2"
	err = tickets.ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("ReopenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.GetStatus() != model.TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.reopen" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.reopen")
	}
	if len(ticket.Sessions) != openSessionCount+1 {
		t.Fatalf("ReopenTicket must append exactly one session: expected=%d got=%d", openSessionCount+1, len(ticket.Sessions))
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := model.GoalCreateInput{
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

	_, err = ctx.GoalClose(model.GoalCloseInput{ID: goal2.ID, Summary: "Done", NoManagement: true})
	if err != nil {
		t.Fatalf("GoalClose failed: %v", err)
	}
	closedGoalPath := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎯️goals", goal2.ID, "🎯️goal.json")
	closedGoalContent, err := workspace.ReadTextFile(closedGoalPath)
	if err != nil {
		t.Fatalf("failed to read closed goal: %v", err)
	}
	var closedGoal model.Goal
	if err := json.Unmarshal([]byte(closedGoalContent), &closedGoal); err != nil {
		t.Fatalf("failed to unmarshal closed goal: %v", err)
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

	ctx := &RepoContext{rootDir: root}

	name := "RenamedName"
	description := "renamed description"
	mdID := workspace.Slugify("OriginalName")
	changed, err := ctx.TodoChange(model.TodoChangeInput{ID: mdID, Name: &name, Description: &description})
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
	codeID := workspace.Slugify("CodeTodo")
	changedCode, err := ctx.TodoChange(model.TodoChangeInput{ID: codeID, Name: &codeName, Description: &codeDescription})
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
	if _, err := ctx.TodoChange(model.TodoChangeInput{ID: "does-not-exist", Name: &missingName}); err == nil {
		t.Fatal("expected error for missing todo")
	}
}
