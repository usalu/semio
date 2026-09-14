// 🔬️ Tests of the goals domain, split out of the pre-split godfile suite.

package goals

import (
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

func TestComposeIDToGoalPath(t *testing.T) {

	goals, err := ListGoals()
	if err != nil || len(goals) == 0 {
		t.Skip("no goals available for round-trip test")
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
