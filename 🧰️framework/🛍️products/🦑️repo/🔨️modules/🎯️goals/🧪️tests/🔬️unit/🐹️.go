// 🔬️ Tests of the goals domain, split out of the pre-split godfile suite.

package goals

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func withGoalFixture(t *testing.T, ids ...string) {
	t.Helper()
	root := t.TempDir()
	sandbox.BindRoot(t, root)
	for _, id := range ids {
		dir := filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(id))
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(dir, "🎯️goal.json"), []byte("{}\n"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
}

func TestComposeIDToGoalPath(t *testing.T) {
	withGoalFixture(t, "AI-OPTIMIZED-REPO", "AI-OPTIMIZED-REPO/REPO-CLIENT", "ANIMATE")
	goals, err := ListGoals()
	if err != nil || len(goals) != 3 {
		t.Fatalf("expected the three fixture goals, got %d (%v)", len(goals), err)
	}
	for _, g := range goals {
		composeID := model.GoalPathToComposeID(g.ID)
		roundTrip := model.ComposeIDToGoalPath(composeID)
		if roundTrip != g.ID {
			t.Errorf("round-trip failed: %q -> %q -> %q (expected %q)", g.ID, composeID, roundTrip, g.ID)
		}
	}
}

func TestGoalIDForFilesystem(t *testing.T) {
	tests := []struct {
		in   string
		want string
	}{
		{"AI-OPTIMIZED-REPO/REPO-CLIENT", "AI-OPTIMIZED-REPO/REPO-CLIENT"},
		{"🎯️aioptimizedrepo🎯️repoclient", ComposeIDToGoalPath("🎯️aioptimizedrepo🎯️repoclient")},
	}
	for _, tt := range tests {
		got := GoalIDForFilesystem(tt.in)
		if got != tt.want {
			t.Errorf("GoalIDForFilesystem(%q) = %q, want %q", tt.in, got, tt.want)
		}
	}
}
