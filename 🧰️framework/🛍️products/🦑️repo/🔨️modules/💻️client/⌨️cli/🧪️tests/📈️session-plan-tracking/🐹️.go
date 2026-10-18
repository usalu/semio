// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Session JSON plan tracking and ticket agent plan merging.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestSessionJsonTracksPlan(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	sessionID := "plan-track-session"
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	startPayload := json.RawMessage(`{"session_id":"plan-track-session","second":"2026-03-02T10:00:00Z"}`)
	RunHook(HookContext{
		Event:    HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-03-02T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    startPayload,
	})

	plan1Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"in-progress"},{"title":"Step B","status":"pending"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    plan1Payload,
	})

	metaPath := filepath.Join(logDir, "session.json")
	metaBytes, err := os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after plan update, got error: %v", err)
	}
	var meta SessionMeta
	assertHasLifecycle := func(step TicketAgentPlanStep) {
		if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
			t.Errorf("expected step %q to include at least one lifecycle timestamp, got %+v", step.Name, step)
		}
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 2 {
		t.Fatalf("expected 2 plan steps, got %v", meta.Plan)
	}
	stepA := meta.Plan.Steps[0]
	if stepA.Name != "Step A" {
		t.Errorf("expected Step A, got %s", stepA.Name)
	}
	if stepA.Ideated == "" {
		t.Errorf("expected Step A ideated timestamp, got empty")
	}
	if stepA.Started == "" {
		t.Errorf("expected Step A started timestamp, got empty")
	}
	if stepA.Completed != "" {
		t.Errorf("expected Step A not completed, got %s", stepA.Completed)
	}
	stepB := meta.Plan.Steps[1]
	if stepB.Name != "Step B" {
		t.Errorf("expected Step B, got %s", stepB.Name)
	}
	if stepB.Ideated == "" {
		t.Errorf("expected Step B ideated timestamp, got empty")
	}
	if stepB.Started != "" || stepB.Completed != "" {
		t.Errorf("expected Step B pending, got started=%s completed=%s", stepB.Started, stepB.Completed)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan2Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"completed"},{"title":"Step B","status":"in-progress"},{"title":"Step C","status":"pending"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    plan2Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after second plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after second update, got %v", meta.Plan)
	}
	stepA2 := meta.Plan.Steps[0]
	if stepA2.Name != "Step A" || stepA2.Completed == "" {
		t.Errorf("expected Step A completed, got %+v", stepA2)
	}
	if stepA2.Ideated != stepA.Ideated {
		t.Errorf("expected Step A ideated timestamp preserved, got %s vs %s", stepA2.Ideated, stepA.Ideated)
	}
	if stepA2.Started != stepA.Started {
		t.Errorf("expected Step A started timestamp preserved, got %s vs %s", stepA2.Started, stepA.Started)
	}
	stepB2 := meta.Plan.Steps[1]
	if stepB2.Name != "Step B" || stepB2.Started == "" {
		t.Errorf("expected Step B in-progress, got %+v", stepB2)
	}
	stepC := meta.Plan.Steps[2]
	if stepC.Name != "Step C" || stepC.Ideated == "" || stepC.Started != "" || stepC.Completed != "" {
		t.Errorf("expected Step C pending, got %+v", stepC)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan3Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step B","status":"completed"},{"title":"Step C","status":"in-progress"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:03:00Z",
		RepoRoot: tmpDir,
		Input:    plan3Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after third plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after third update (Step A preserved as completed), got %v", meta.Plan)
	}
	stepB3 := meta.Plan.Steps[0]
	if stepB3.Name != "Step B" || stepB3.Completed == "" {
		t.Errorf("expected Step B completed, got %+v", stepB3)
	}
	if stepB3.Started != stepB2.Started {
		t.Errorf("expected Step B started timestamp preserved, got %s vs %s", stepB3.Started, stepB2.Started)
	}
	stepC2 := meta.Plan.Steps[1]
	if stepC2.Name != "Step C" || stepC2.Started == "" {
		t.Errorf("expected Step C in-progress, got %+v", stepC2)
	}
	stepACompleted := meta.Plan.Steps[2]
	if stepACompleted.Name != "Step A" || stepACompleted.Completed == "" || stepACompleted.Abandoned != "" {
		t.Errorf("expected Step A to remain completed and not abandoned, got %+v", stepACompleted)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}
}

func TestMergeTicketAgentPlanSteps(t *testing.T) {
	second1 := "2026-03-02T10:00:00Z"
	second2 := "2026-03-02T10:01:00Z"

	t.Run("empty existing adopts incoming", func(t *testing.T) {
		incoming := []HookPlanStep{
			{Name: "A", Status: "in-progress"},
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(nil, incoming, second1)
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
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
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
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
			{Name: "B"},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
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
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
			{Name: "B"},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A historical), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Started != second1 || historical.Abandoned != "" {
			t.Errorf("expected A started history preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("preserves already abandoned steps", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Abandoned: second1},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
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
		existing := []TicketAgentPlanStep{
			{Name: "A", Completed: second1},
			{Name: "B", Started: second1},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A completed history), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Completed != second1 || historical.Abandoned != "" {
			t.Errorf("expected completed A preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("adds new steps not in existing", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1, Completed: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
			{Name: "C", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(merged))
		}
		if merged[1].Name != "C" || merged[1].Ideated != second2 || merged[1].Started != "" {
			t.Errorf("expected C as new pending step, got %+v", merged[1])
		}
	})

	t.Run("does not set completed for step never started", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 1 {
			t.Fatalf("expected 1 step, got %d", len(merged))
		}
		if merged[0].Completed != "" {
			t.Errorf("expected no completed timestamp when step was never started, got %+v", merged[0])
		}
	})

	t.Run("backfills legacy name-only steps with lifecycle dates", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "Legacy Pending"},
		}
		incoming := []HookPlanStep{
			{Name: "Legacy Pending", Status: "pending"},
			{Name: "New Pending", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
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

func TestStreamSessionsListsHookSessionLogs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "stream-session")
	if err := os.MkdirAll(logDir, 0o755); err != nil {
		t.Fatal(err)
	}
	out := make(chan Session, 4)
	if err := StreamSessions(context.Background(), out); err != nil {
		t.Fatalf("StreamSessions failed: %v", err)
	}
	var found []Session
	for session := range out {
		found = append(found, session)
	}
	if len(found) != 1 {
		t.Fatalf("expected exactly the written session, got %v", found)
	}
	if found[0].UUID != "stream-session" || found[0].Year != now.Year()%100 || found[0].Month != int(now.Month()) || found[0].Day != now.Day() {
		t.Errorf("session %+v does not carry the written log date and id", found[0])
	}
}
