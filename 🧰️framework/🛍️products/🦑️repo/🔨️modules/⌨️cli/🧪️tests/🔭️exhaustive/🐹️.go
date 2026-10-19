//go:build exhaustive

// 🔭️ Exhaustive tests of the cli domain: codebase, policy, bundle, goal document, ticket identifiers and tree commands.

package cli

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	graphql "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// 🧪️#region 🏩️Codebase
func TestExhaustiveCodebaseCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow codebase test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

func TestExhaustivePolicyBreachListCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy breach list test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolPolicyBreachList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyBreachList returned error: %s", result.Error)
	}
}

func TestExhaustiveBundleTreeCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle tree test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolTechnologyTree()
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
	}
}

func TestExhaustiveGoalDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow goal document test in short mode")
	}
	withExhaustiveRepo(t)
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-PARENT-GOAL"))
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "RENAMED-PARENT"))
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL"))

	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*model.Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

	childTitle := "Test Child Goal"
	childRes := ToolGoalCreate(childTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent.ID, "")
	if childRes.Error != "" {
		t.Fatalf("Failed to create child: %s", childRes.Error)
	}
	child, ok := childRes.Data.(*model.Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	expectedChildID := "TEST-PARENT-GOAL/TEST-CHILD-GOAL"
	if child.ID != expectedChildID {
		t.Errorf("Expected child ID '%s', got '%s'", expectedChildID, child.ID)
	}

	childPath := filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(child.ID), "🎯️goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	parent.Title = "Renamed Parent"
	err := graphql.UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(newChildID), "🎯️goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*model.Goal)
	var foundChild *model.Goal
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

	ctx := &graphql.RepoContext{}
	emptyParent := ""
	changeInput := model.GoalChangeInput{
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

	rootChildPath := filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL", "🎯️goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL"))
}

func TestExhaustiveTicketListIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket list ids test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Skipf("ToolTicketList returned error: %s", result.Error)
	}
	tickets, ok := result.Data.([]model.Ticket)
	if !ok {
		t.Skip("ToolTicketList data is not []Ticket")
	}
	for _, tk := range tickets {
		id := tk.GetID()
		expectedPrefix := model.EmojiText(model.EmojiTicket)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
		}
		expectedID := expectedPrefix + workspace.Flat(tk.Slug)
		if id != expectedID {
			t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
		}
	}
}

func withoutSelectors(value string) string {
	return strings.NewReplacer("\uFE0E", "", "\uFE0F", "").Replace(value)
}

func TestExhaustiveTreeCommands(t *testing.T) {

	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)

	output, err := executeTreeCommand("search", "compose/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(withoutSelectors(output), "💻compose") {
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
	if !strings.Contains(withoutSelectors(output), "💻compose") {
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
