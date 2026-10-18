//go:build exhaustive

// 🔬️ Tests of the tickets domain, split out of the pre-split godfile suite.

package tickets

import (
	context "context"
	json "encoding/json"
	fs "io/fs"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	goals "github.com/usalu/semio/repo/goals"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveNormalizeTicketFileInput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow normalize ticket file input test in short mode")
	}
	absRoot := workspace.GetRootDir()
	filePath := filepath.ToSlash(filepath.Join("repo", "client", "main.go"))
	absPath := filepath.Join(absRoot, filePath)
	fileID := codebase.FileHeaderId(filePath)
	fileUri := model.BuildFileUriFromPath(filePath)
	cases := []struct {
		name  string
		input string
		want  string
	}{
		{"path", filePath, filePath},
		{"abs path", absPath, filePath},
		{"file uri", fileUri, filePath},
		{"id", fileID, filePath},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeTicketFileInput(tc.input)
			if got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestExhaustiveMigrateAuthorFieldsToString(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow migration test in short mode")
	}
	ctx := context.Background()

	ticketCh := make(chan model.Ticket)
	var ticketErr error
	go func() {
		ticketErr = StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	ticketCount := 0
	for ticket := range ticketCh {
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save ticket %s: %v", ticket.Slug, err)
		}
		ticketCount++
	}
	if ticketErr != nil {
		t.Fatalf("stream tickets failed: %v", ticketErr)
	}
	t.Logf("migrated %d tickets via stream", ticketCount)

	ticketsDir := GetTicketsDir()
	remainingCount := 0
	filepath.WalkDir(ticketsDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || d.Name() != "🎫️ticket.json" {
			return nil
		}
		raw, err := workspace.ReadTextFile(path)
		if err != nil {
			return nil
		}
		if !strings.Contains(raw, `"author": {`) {
			return nil
		}
		var ticket model.Ticket
		if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
			t.Logf("failed to parse %s: %v", path, err)
			return nil
		}
		ticket.JsonPath = path
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save remaining ticket %s: %v", path, err)
		}
		remainingCount++
		return nil
	})
	t.Logf("migrated %d remaining tickets", remainingCount)

	goalCh := make(chan *model.Goal)
	var goalErr error
	go func() {
		goalErr = goals.StreamGoals(ctx, goalCh)
	}()
	goalCount := 0
	for goal := range goalCh {
		if err := goals.SaveGoal(*goal); err != nil {
			t.Errorf("failed to save goal %s: %v", goal.ID, err)
		}
		goalCount++
	}
	if goalErr != nil {
		t.Fatalf("stream goals failed: %v", goalErr)
	}
	t.Logf("migrated %d goals", goalCount)
}
