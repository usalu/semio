// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Goal creation, document, listing, and MCP goals resource.

// #endregion 🧲️Header

package client

import (
	"context"
	mcp "github.com/usalu/semio/repo/client/internal/mcp"
	"os"
	"path/filepath"
	"strings"
	"testing"
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
	withTempRepo(t)

	result := ToolGoalCreate("Test Goal Creation", "Test description", "Test prompt for goal", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error != "" {
		t.Fatalf("ToolGoalCreate returned error: %s", result.Error)
	}
	goal, ok := result.Data.(*Goal)
	if !ok || goal == nil {
		t.Fatal("expected goal data")
	}
	if goal.Title != "Test Goal Creation" {
		t.Errorf("expected title 'Test Goal Creation', got '%s'", goal.Title)
	}
	if goal.Dates.Due != "2026-02-15" {
		t.Errorf("expected due date '2026-02-15', got '%s'", goal.Dates.Due)
	}

	goalPath := filepath.Join(GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestExhaustiveGoalDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow goal document test in short mode")
	}
	withExhaustiveRepo(t)

	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

	childTitle := "Test Child Goal"
	childRes := ToolGoalCreate(childTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent.ID, "")
	if childRes.Error != "" {
		t.Fatalf("Failed to create child: %s", childRes.Error)
	}
	child, ok := childRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	expectedChildID := "TEST-PARENT-GOAL/TEST-CHILD-GOAL"
	if child.ID != expectedChildID {
		t.Errorf("Expected child ID '%s', got '%s'", expectedChildID, child.ID)
	}

	childPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(child.ID), "🎯️goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	parent.Title = "Renamed Parent"
	err := UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(newChildID), "🎯️goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*Goal)
	var foundChild *Goal
	for _, g := range allGoals {

		if strings.HasSuffix(g.ID, "/TEST-CHILD-GOAL") && strings.HasPrefix(g.ID, "RENAMED-PARENT") {
			foundChild = g
			break
		}
	}
	if foundChild == nil {
		t.Errorf("Could not find child with new ID prefix in ListGoals output")
	} else {
		if foundChild.ID != newChildID {
			t.Errorf("Expected listed child ID '%s', got '%s'", newChildID, foundChild.ID)
		}
		if foundChild.Parent != parent.ID {
			t.Errorf("Expected listed child parent '%s', got '%s'", parent.ID, foundChild.Parent)
		}
	}

	ctx := &repoContext{}
	emptyParent := ""
	changeInput := GoalChangeInput{
		ID:     newChildID,
		Parent: &emptyParent,
	}

	updatedChild, err := ctx.GoalChange(changeInput)
	if err != nil {
		t.Fatalf("GoalChange failed: %v", err)
	}

	if updatedChild.ID != "TEST-CHILD-GOAL" {
		t.Errorf("Expected reparented child ID 'TEST-CHILD-GOAL', got '%s'", updatedChild.ID)
	}

	if updatedChild.Parent != "" {
		t.Errorf("Expected empty parent, got '%s'", updatedChild.Parent)
	}

	rootChildPath := filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL", "🎯️goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

}

func TestGoalList(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()

	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*Goal)
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
	withExhaustiveRepo(t)
	req := mcp.ReadResourceRequest{}
	req.Params.URI = "repo://goals"
	contents, err := handleGoalsResource(context.Background(), req)
	if err != nil {
		t.Fatalf("handleGoalsResource returned error: %v", err)
	}
	if len(contents) != 1 {
		t.Fatalf("expected 1 ResourceContents entry, got %d", len(contents))
	}
	text, ok := contents[0].(mcp.TextResourceContents)
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
	goals, err := ListGoals()
	if err != nil {
		t.Fatalf("ListGoals failed: %v", err)
	}
	if len(goals) == 0 {
		t.Fatal("the fixture repository must hold the seeded goals")
	}
	for _, g := range goals {
		if !strings.Contains(text.Text, g.Title) {
			t.Errorf("YAML body missing goal title %q", g.Title)
		}
	}
}
