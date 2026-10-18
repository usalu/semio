// 🥅️ Tests of the cli domain: goal commands and goal resources.

package cli

import (
	context "context"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	goalspkg "github.com/usalu/semio/repo/goals"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// 🆕️#region 💌️Goal
func TestGoalCreateValidation(t *testing.T) {

	result := ToolGoalCreate("", "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing title")
	}

	result = ToolGoalCreate("Test Goal", "", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing description")
	}

	result = ToolGoalCreate("Test Goal", "desc", "", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing prompt")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing due date")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing client")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "invalid-llm", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "invalid-client", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid client")
	}
}

func TestGoalCreateAndCleanup(t *testing.T) {

	result := ToolGoalCreate("Test Goal Creation", "Test description", "Test prompt for goal", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error != "" {
		t.Fatalf("ToolGoalCreate returned error: %s", result.Error)
	}
	goal, ok := result.Data.(*model.Goal)
	if !ok || goal == nil {
		t.Fatal("expected goal data")
	}
	if goal.Title != "Test Goal Creation" {
		t.Errorf("expected title 'Test Goal Creation', got '%s'", goal.Title)
	}
	if goal.Dates.Due != "2026-02-15" {
		t.Errorf("expected due date '2026-02-15', got '%s'", goal.Dates.Due)
	}

	goalPath := filepath.Join(workspace.GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestGoalList(t *testing.T) {
	result := ToolGoalList()

	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok {
		t.Fatalf("ToolGoalList Data was not []*Goal, got %T", result.Data)
	}
	if len(goals) == 0 {
		t.Fatal("ToolGoalList returned no goals; expected the seeded goal document")
	}
	for _, g := range goals {
		if g.ID == "" {
			t.Errorf("goal has empty ID: %+v", g)
		}
		if g.Title == "" {
			t.Errorf("goal %s has empty title", g.ID)
		}
	}
}

func TestGoalsMcpResource(t *testing.T) {
	req := ReadResourceRequest{}
	req.Params.URI = "repo://goals"
	contents, err := handleGoalsResource(context.Background(), req)
	if err != nil {
		t.Fatalf("handleGoalsResource returned error: %v", err)
	}
	if len(contents) != 1 {
		t.Fatalf("expected 1 ResourceContents entry, got %d", len(contents))
	}
	text, ok := contents[0].(TextResourceContents)
	if !ok {
		t.Fatalf("expected TextResourceContents, got %T", contents[0])
	}
	if text.URI != "repo://goals" {
		t.Errorf("URI: expected repo://goals, got %q", text.URI)
	}
	if text.MIMEType != "text/plain" {
		t.Errorf("MIMEType: expected text/plain, got %q", text.MIMEType)
	}
	if text.Text == "" {
		t.Fatal("expected non-empty YAML text body")
	}
	goals, err := goalspkg.ListGoals()
	if err != nil {
		t.Fatalf("ListGoals failed: %v", err)
	}
	if len(goals) == 0 {
		t.Skip("no goals seeded in repo; cannot assert content")
	}
	for _, g := range goals {
		if !strings.Contains(text.Text, g.Title) {
			t.Errorf("YAML body missing goal title %q", g.Title)
		}
	}
}
