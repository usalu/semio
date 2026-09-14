// 🔬️ Tests of the tickets domain, split out of the pre-split godfile suite.

package tickets

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	contributors "github.com/usalu/semio/repo/contributors"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestStreamAndListTicketsIgnoreNestedWorkspaceFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRootDir := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(oldRootDir)

	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets", model.FormatYearDir(26), model.FormatMonthDir(3), model.FormatDayDir(7), "SAMPLE")
	if err := os.MkdirAll(filepath.Join(ticketDir, "workspace", "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create nested workspace: %v", err)
	}
	ticketJSON := `{
  "title": "Sample Ticket",
  "status": "open",
  "description": "ticket for streaming"
}`
	if err := os.WriteFile(filepath.Join(ticketDir, "🎫️ticket.json"), []byte(ticketJSON), 0644); err != nil {
		t.Fatalf("failed to create ticket.json: %v", err)
	}
	if err := os.WriteFile(filepath.Join(ticketDir, "workspace", "node_modules", "pkg", "🎫️ticket.json"), []byte(`{"title":"nested"}`), 0644); err != nil {
		t.Fatalf("failed to create nested ticket.json: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	ticketCh := make(chan model.Ticket)
	errCh := make(chan error, 1)
	go func() {
		errCh <- StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	var streamed []model.Ticket
	for ticket := range ticketCh {
		streamed = append(streamed, ticket)
	}
	if err := <-errCh; err != nil {
		t.Fatalf("StreamTickets returned error: %v", err)
	}
	if len(streamed) != 1 || streamed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one streamed SAMPLE ticket, got %+v", streamed)
	}

	listed, err := ListTickets(nil, nil, nil)
	if err != nil {
		t.Fatalf("ListTickets failed: %v", err)
	}
	if len(listed) != 1 || listed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one listed SAMPLE ticket, got %+v", listed)
	}
}

func TestSectionHeaderIdAndUri(t *testing.T) {
	id := SectionHeaderId("src/app.ts", "Functions")
	if !strings.Contains(id, model.EmojiText(model.EmojiSection)+workspace.Flat("Functions")) {
		t.Fatalf("unexpected section header id: %s", id)
	}
	if strings.HasPrefix(id, model.EmojiText(model.EmojiSection)) {
		t.Fatalf("section header id should include file parent before section emoji: %s", id)
	}
	nestedId := SectionHeaderId("src/shared.ts", "Types#YPath Types")
	if !strings.Contains(nestedId, model.EmojiText(model.EmojiSection)+workspace.Flat("Types")+model.EmojiText(model.EmojiSection)+workspace.Flat("YPath Types")) {
		t.Fatalf("nested section header id should include section emoji before each nested segment, got: %s", nestedId)
	}
	uri := SectionHeaderUri("src/app.ts", "Functions")
	if !strings.HasPrefix(uri, "repo://section/") {
		t.Fatalf("unexpected section header uri: %s", uri)
	}
	if !strings.Contains(uri, model.EmojiText(model.EmojiSection)) {
		t.Fatalf("section uri should contain section emoji: %s", uri)
	}
}

func TestDefinitionHeaderIdAndUri(t *testing.T) {
	id := DefinitionHeaderId("src/app.ts", "Functions", "doWork", "implementation")
	if !strings.Contains(id, model.EmojiText(model.EmojiDefinitionImpl)+workspace.Flat("doWork")) {
		t.Fatalf("unexpected definition header id: %s", id)
	}
	uri := DefinitionHeaderUri("src/app.ts", "Functions", "doWork")
	if !strings.HasPrefix(uri, "repo://definition/") {
		t.Fatalf("unexpected definition header uri: %s", uri)
	}
	if !strings.Contains(uri, workspace.Flat("doWork")) {
		t.Fatalf("definition uri should contain flattened def name: %s", uri)
	}
}

func setupTicketDir(t *testing.T) (string, string) {
	t.Helper()
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets",
		model.FormatYearDir(now.Year()%100),
		model.FormatMonthDir(int(now.Month())),
		model.FormatDayDir(now.Day()),
		"TEST-TICKET")
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "🎫️ticket.json")
	initialTicket := `{"title":"Test Ticket","status":"open","goal":"TEST/GOAL"}`
	if err := os.WriteFile(ticketJSON, []byte(initialTicket), 0644); err != nil {
		t.Fatal(err)
	}
	importantPath := filepath.Join(ticketDir, "📌️important", "📝️.md")
	if err := os.MkdirAll(filepath.Dir(importantPath), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(importantPath, nil, 0644); err != nil {
		t.Fatal(err)
	}
	return tmpDir, ticketJSON
}

func setupImportantTicket(t *testing.T, status model.TicketStatus, content []byte) (*model.Ticket, string) {
	t.Helper()
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	t.Cleanup(func() { workspace.RootDir = oldRoot })
	ticket := &model.Ticket{
		Year:       26,
		Month:      8,
		Day:        26,
		Slug:       "IMPORTANT-LIFECYCLE",
		Title:      "Important Lifecycle",
		Status:     status,
		Goal:       "TEST/GOAL",
		FolderPath: GetTicketPath(26, 8, 26, "IMPORTANT-LIFECYCLE"),
		JsonPath:   GetTicketJsonPath(26, 8, 26, "IMPORTANT-LIFECYCLE"),
		Interactions: []model.Interaction{{
			Kind: "ticket.open",
			Date: "2026-08-26 00:00:00",
		}},
	}
	ticket.ImportantPath = GetImportantFilePath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	if err := os.MkdirAll(filepath.Dir(ticket.ImportantPath), 0755); err != nil {
		t.Fatal(err)
	}
	if content != nil {
		if err := os.WriteFile(ticket.ImportantPath, content, 0644); err != nil {
			t.Fatal(err)
		}
	}
	data, err := json.Marshal(ticket)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(ticket.JsonPath, data, 0644); err != nil {
		t.Fatal(err)
	}
	return ticket, tmpDir
}

func TestTicketImportantPathCanonical(t *testing.T) {
	root := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = root
	t.Cleanup(func() { workspace.RootDir = oldRoot })
	want := filepath.Join(GetTicketPath(26, 8, 26, "PATH-TEST"), "📌️important", "📝️.md")
	if got := GetImportantFilePath(26, 8, 26, "PATH-TEST"); got != want {
		t.Fatalf("GetImportantFilePath() = %q, want %q", got, want)
	}
}

func TestCreateReadAndRenameTicketUseCanonicalImportantPath(t *testing.T) {
	root := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = root
	t.Cleanup(func() { workspace.RootDir = oldRoot })
	contributors.TestSessionIDOverride = "important-create-session"
	t.Cleanup(func() { contributors.TestSessionIDOverride = "" })
	ticket, err := CreateTicket("🎫️", "Canonical Important", "Prompt", "", "", "copilot-chat", "", true, "TEST/GOAL", "", true, "", providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("CreateTicket() failed: %v", err)
	}
	want := filepath.Join(ticket.FolderPath, "📌️important", "📝️.md")
	if ticket.ImportantPath != want {
		t.Fatalf("created ImportantPath = %q, want %q", ticket.ImportantPath, want)
	}
	if data, err := os.ReadFile(want); err != nil || len(data) != 0 {
		t.Fatalf("created important document = %q, %v; want zero bytes", data, err)
	}
	if _, err := os.Lstat(filepath.Join(ticket.FolderPath, "📌️important.md")); !os.IsNotExist(err) {
		t.Fatalf("legacy important path exists: %v", err)
	}
	read, err := ReadTicket(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	if err != nil {
		t.Fatalf("ReadTicket() failed: %v", err)
	}
	if read.ImportantPath != want {
		t.Fatalf("read ImportantPath = %q, want %q", read.ImportantPath, want)
	}
	if err := os.WriteFile(want, []byte("preserve"), 0644); err != nil {
		t.Fatal(err)
	}
	oldPath := want
	if err := UpdateTicketTitle(read, "Renamed Important"); err != nil {
		t.Fatalf("UpdateTicketTitle() failed: %v", err)
	}
	want = filepath.Join(read.FolderPath, "📌️important", "📝️.md")
	if read.ImportantPath != want {
		t.Fatalf("renamed ImportantPath = %q, want %q", read.ImportantPath, want)
	}
	if data, err := os.ReadFile(want); err != nil || string(data) != "preserve" {
		t.Fatalf("renamed important document = %q, %v", data, err)
	}
	if _, err := os.Lstat(oldPath); !os.IsNotExist(err) {
		t.Fatalf("old important path exists after owner rename: %v", err)
	}
}

func TestCreateTicketNeverOverwritesExistingImportantDocument(t *testing.T) {
	root := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = root
	t.Cleanup(func() { workspace.RootDir = oldRoot })
	year, month, day := workspace.FormatDate(time.Now())
	path := GetImportantFilePath(year, month, day, "CANONICAL-IMPORTANT-COLLISION")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte("preserve"), 0644); err != nil {
		t.Fatal(err)
	}
	if _, err := CreateTicket("🎫️", "Canonical Important Collision", "Prompt", "", "", "copilot-chat", "", true, "TEST/GOAL", "", true, "", providers.McpClientGeneric, "", ""); err == nil {
		t.Fatal("CreateTicket() overwrote an existing ticket root")
	}
	if data, err := os.ReadFile(path); err != nil || string(data) != "preserve" {
		t.Fatalf("existing important document changed: %q, %v", data, err)
	}
}

func TestTicketLifecycleRejectsInvalidStatusWithoutMutation(t *testing.T) {
	ticket, _ := setupImportantTicket(t, model.TicketStatusOpen, []byte{})
	manifest := []byte("preserve manifest")
	if err := os.WriteFile(ticket.JsonPath, manifest, 0644); err != nil {
		t.Fatal(err)
	}
	ticket.Status = model.TicketStatus("paused")
	if _, err := json.Marshal(ticket); err == nil {
		t.Fatal("json.Marshal() accepted an invalid status")
	}
	if err := SaveTicket(ticket); err == nil {
		t.Fatal("SaveTicket() accepted an invalid status")
	}
	if data, err := os.ReadFile(ticket.JsonPath); err != nil || !bytes.Equal(data, manifest) {
		t.Fatalf("invalid SaveTicket() changed manifest: %q, %v", data, err)
	}
	beforeInteractions := len(ticket.Interactions)
	if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
		t.Fatal("FinishTicket() accepted an invalid status")
	}
	if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", ""); err == nil {
		t.Fatal("ReopenTicket() accepted an invalid status")
	}
	if ticket.Status != model.TicketStatus("paused") || len(ticket.Interactions) != beforeInteractions {
		t.Fatalf("invalid lifecycle mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
	}
}

func TestFinishTicketImportantLifecycle(t *testing.T) {
	for _, bulk := range []bool{false, true} {
		t.Run(fmt.Sprintf("empty-bulk-%v", bulk), func(t *testing.T) {
			ticket, root := setupImportantTicket(t, model.TicketStatusOpen, []byte{})
			changed := filepath.Join(root, "changed.txt")
			if err := os.WriteFile(changed, []byte("changed"), 0644); err != nil {
				t.Fatal(err)
			}
			if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, bulk); err != nil {
				t.Fatalf("FinishTicket() failed: %v", err)
			}
			if ticket.Status != model.TicketStatusClosed {
				t.Fatalf("status = %q, want closed", ticket.Status)
			}
			if _, err := os.Lstat(ticket.ImportantPath); !os.IsNotExist(err) {
				t.Fatalf("important leaf still exists: %v", err)
			}
			if _, err := os.Lstat(filepath.Dir(ticket.ImportantPath)); !os.IsNotExist(err) {
				t.Fatalf("important directory still exists: %v", err)
			}
		})
	}

	for _, test := range []struct {
		name    string
		content []byte
		bulk    bool
	}{
		{name: "nonempty", content: []byte("action")},
		{name: "whitespace", content: []byte(" \n\t")},
		{name: "nonempty-bulk", content: []byte("action"), bulk: true},
	} {
		t.Run(test.name, func(t *testing.T) {
			ticket, _ := setupImportantTicket(t, model.TicketStatusOpen, test.content)
			beforeInteractions := len(ticket.Interactions)
			if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, test.bulk); err == nil {
				t.Fatal("FinishTicket() succeeded for a nonempty important document")
			}
			if ticket.Status != model.TicketStatusOpen || len(ticket.Interactions) != beforeInteractions {
				t.Fatalf("failed close mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
			}
			if data, err := os.ReadFile(ticket.ImportantPath); err != nil || !bytes.Equal(data, test.content) {
				t.Fatalf("important document changed: %q, %v", data, err)
			}
		})
	}

	t.Run("missing", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusOpen, nil)
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded without an important document")
		}
		if ticket.Status != model.TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
	})

	t.Run("non-regular", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusOpen, nil)
		if err := os.MkdirAll(ticket.ImportantPath, 0755); err != nil {
			t.Fatal(err)
		}
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded for a non-regular important document")
		}
		if ticket.Status != model.TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
	})

	t.Run("save-failure-rolls-back", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusOpen, []byte{})
		beforeInteractions := append([]model.Interaction(nil), ticket.Interactions...)
		ticket.JsonPath = ticket.FolderPath
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded with an unwritable manifest path")
		}
		if ticket.Status != model.TicketStatusOpen || len(ticket.Interactions) != len(beforeInteractions) {
			t.Fatalf("failed close mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || len(data) != 0 {
			t.Fatalf("important document was not restored: %q, %v", data, err)
		}
	})
}

func TestReopenTicketImportantLifecycle(t *testing.T) {
	t.Run("recreates-missing", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusClosed, nil)
		contributors.TestSessionIDOverride = "important-reopen-create"
		t.Cleanup(func() { contributors.TestSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", ""); err != nil {
			t.Fatalf("ReopenTicket() failed: %v", err)
		}
		if ticket.Status != model.TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || len(data) != 0 {
			t.Fatalf("reopened important document = %q, %v", data, err)
		}
	})

	t.Run("preserves-nonempty", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusClosed, []byte("preserve"))
		contributors.TestSessionIDOverride = "important-reopen-preserve"
		t.Cleanup(func() { contributors.TestSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", ""); err != nil {
			t.Fatalf("ReopenTicket() failed: %v", err)
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || string(data) != "preserve" {
			t.Fatalf("existing important document changed: %q, %v", data, err)
		}
	})

	t.Run("save-failure-rolls-back-created-document", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, model.TicketStatusClosed, nil)
		beforeInteractions := len(ticket.Interactions)
		beforeSessions := len(ticket.Sessions)
		ticket.JsonPath = ticket.FolderPath
		contributors.TestSessionIDOverride = "important-reopen-rollback"
		t.Cleanup(func() { contributors.TestSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", ""); err == nil {
			t.Fatal("ReopenTicket() succeeded with an unwritable manifest path")
		}
		if ticket.Status != model.TicketStatusClosed || len(ticket.Interactions) != beforeInteractions || len(ticket.Sessions) != beforeSessions {
			t.Fatalf("failed reopen mutated ticket: status=%q interactions=%d sessions=%d", ticket.Status, len(ticket.Interactions), len(ticket.Sessions))
		}
		if _, err := os.Lstat(ticket.ImportantPath); !os.IsNotExist(err) {
			t.Fatalf("failed reopen left a created important document: %v", err)
		}
	})
}

func writeSparseTicketArtifact(t *testing.T, path string, size int64) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatal(err)
	}
	f, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := f.Truncate(size); err != nil {
		_ = f.Close()
		t.Fatal(err)
	}
	if err := f.Close(); err != nil {
		t.Fatal(err)
	}
}

func TestFinishTicketPurgesOversizedArtifacts(t *testing.T) {
	tmpDir, ticketJSON := setupTicketDir(t)
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	ticketDir := filepath.Dir(ticketJSON)
	now := time.Now().UTC()
	ticket := &model.Ticket{
		Year:       now.Year() % 100,
		Month:      int(now.Month()),
		Day:        now.Day(),
		Slug:       "TEST-TICKET",
		Title:      "Test Ticket",
		Status:     model.TicketStatusOpen,
		FolderPath: ticketDir,
		JsonPath:   ticketJSON,
		Interactions: []model.Interaction{{
			Kind: "ticket.open",
			Date: "2026-01-01 00:00:00",
		}},
	}

	const mib = 1024 * 1024
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "large.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "exact-5mb.bin"), 5*mib)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, ".cache.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "huge-dir", "blob.bin"), 10*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "small-dir", "small.bin"), 1*mib)

	testFile := "changed.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("x"), 0644); err != nil {
		t.Fatal(err)
	}

	if err := FinishTicket(ticket, "Summary", []string{testFile}, true, false); err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.Status != model.TicketStatusClosed {
		t.Fatalf("ticket status = %v, want closed", ticket.Status)
	}

	assertNotExists := func(path string) {
		t.Helper()
		if _, err := os.Stat(path); !os.IsNotExist(err) {
			t.Fatalf("expected deleted: %s", path)
		}
	}
	assertExists := func(path string) {
		t.Helper()
		if _, err := os.Stat(path); err != nil {
			t.Fatalf("expected kept: %s (%v)", path, err)
		}
	}

	assertNotExists(filepath.Join(ticketDir, "large.bin"))
	assertNotExists(filepath.Join(ticketDir, ".cache.bin"))
	assertNotExists(filepath.Join(ticketDir, "huge-dir"))
	assertExists(filepath.Join(ticketDir, "exact-5mb.bin"))
	assertExists(filepath.Join(ticketDir, "small-dir", "small.bin"))
	assertExists(ticketJSON)
}

func TestPurgeAllOversizedTicketArtifacts(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	now := time.Now().UTC()
	year := now.Year() % 100
	month := int(now.Month())
	day := now.Day()
	slug := "TEST-TICKET"
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets", model.FormatYearDir(year), model.FormatMonthDir(month), model.FormatDayDir(day), slug)
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "🎫️ticket.json")
	if err := os.WriteFile(ticketJSON, []byte(`{"title":"Test Ticket","goal":"TEST/GOAL"}`), 0644); err != nil {
		t.Fatal(err)
	}

	ticket := &model.Ticket{
		Year:       year,
		Month:      month,
		Day:        day,
		Slug:       slug,
		Title:      "Test Ticket",
		Status:     model.TicketStatusOpen,
		FolderPath: ticketDir,
		JsonPath:   ticketJSON,
	}
	if err := SaveTicket(ticket); err != nil {
		t.Fatal(err)
	}

	const mib = 1024 * 1024
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "large.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "huge-dir", "blob.bin"), 10*mib+1)

	count, err := PurgeAllOversizedTicketArtifacts()
	if err != nil {
		t.Fatalf("PurgeAllOversizedTicketArtifacts failed: %v", err)
	}
	if count != 1 {
		t.Fatalf("purged ticket count = %d, want 1", count)
	}
	if _, err := os.Stat(filepath.Join(ticketDir, "large.bin")); !os.IsNotExist(err) {
		t.Fatalf("expected deleted large.bin")
	}
	if _, err := os.Stat(filepath.Join(ticketDir, "huge-dir")); !os.IsNotExist(err) {
		t.Fatalf("expected deleted huge-dir")
	}
	if _, err := os.Stat(ticketJSON); err != nil {
		t.Fatalf("expected kept ticket.json: %v", err)
	}
}

func TestResolvePlanSourceCursorPlanID(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	planFile := filepath.Join(planDir, "kit_store_"+id+".plan.md")
	if err := os.WriteFile(planFile, []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := workspace.RootDir
	workspace.RootDir = tmp
	defer func() { workspace.RootDir = oldRoot }()

	got, err := ResolvePlanSource(NewFileTicketStore(), HostPlanRoots(), providers.McpClientCursor, id)
	if err != nil {
		t.Fatal(err)
	}
	if got.IsDirectory {
		t.Fatal("expected file")
	}
	if filepath.Clean(got.Path) != filepath.Clean(planFile) {
		t.Fatalf("got %q want %q", got.Path, planFile)
	}
}

func TestResolvePlanSourceKiroSpecID(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, ".kiro", "specs", "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "design.md"), []byte("d"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := workspace.RootDir
	workspace.RootDir = tmp
	defer func() { workspace.RootDir = oldRoot }()

	got, err := ResolvePlanSource(NewFileTicketStore(), HostPlanRoots(), providers.McpClientKiro, "my-spec")
	if err != nil {
		t.Fatal(err)
	}
	if !got.IsDirectory {
		t.Fatal("expected directory")
	}
	if filepath.Clean(got.Path) != filepath.Clean(specDir) {
		t.Fatalf("got %q want %q", got.Path, specDir)
	}
}

func TestMoveTicketPlanIntoFolderFile(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	src := filepath.Join(tmp, "outside.md")
	if err := os.WriteFile(src, []byte("body"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &model.Ticket{FolderPath: ticketDir, Plan: &model.TicketPlan{Source: src, Client: "cursor", ID: "x"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	dst := filepath.Join(ticketDir, "outside.md")
	if _, err := os.Stat(dst); err != nil {
		t.Fatal(err)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "outside.md" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestMoveTicketPlanIntoFolderSpecDir(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	specRoot := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(filepath.Join(specRoot, "nested"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "a.md"), []byte("a"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "nested", "b.md"), []byte("b"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &model.Ticket{FolderPath: ticketDir, Plan: &model.TicketPlan{Source: specRoot, Client: "kiro", ID: "my-spec"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(specRoot); !os.IsNotExist(err) {
		t.Fatalf("source spec dir should be gone: %v", err)
	}
	destRoot := filepath.Join(ticketDir, "my-spec")
	if b, err := os.ReadFile(filepath.Join(destRoot, "a.md")); err != nil || string(b) != "a" {
		t.Fatalf("a.md: %v %q", err, b)
	}
	if b, err := os.ReadFile(filepath.Join(destRoot, "nested", "b.md")); err != nil || string(b) != "b" {
		t.Fatalf("nested/b.md: %v %q", err, b)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "my-spec" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestApplyTicketPlanFromIDsCursor(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	planFile := filepath.Join(planDir, "kit_store_backbone_generalization_"+id+".plan.md")
	if err := os.WriteFile(planFile, []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := workspace.RootDir
	workspace.RootDir = tmp
	defer func() { workspace.RootDir = oldRoot }()

	ticket := &model.Ticket{}
	if err := ApplyTicketPlanFromIDs(NewFileTicketStore(), HostPlanRoots(), ticket, providers.McpClientCursor, id, ""); err != nil {
		t.Fatal(err)
	}
	if ticket.Plan == nil || ticket.Plan.ID != id || ticket.Plan.Client != "cursor" {
		t.Fatalf("plan: %+v", ticket.Plan)
	}
	if filepath.Clean(ticket.Plan.Source) != filepath.Clean(planFile) {
		t.Fatalf("source %q want %q", ticket.Plan.Source, planFile)
	}
}

func TestStripPlanFrontmatter(t *testing.T) {
	raw := "---\nname: Test\noverview: x\n---\n\n# Body\n"
	got := stripPlanFrontmatter(raw)
	if got != "# Body" {
		t.Fatalf("got %q", got)
	}
	if stripPlanFrontmatter("# No frontmatter") != "# No frontmatter" {
		t.Fatal("expected unchanged content without frontmatter")
	}
}

func TestFormatPlanCommentFile(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "feature_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Feature\n---\n\n## Steps\n\nDo work.\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&model.TicketPlan{Client: "cursor", ID: "abcd1234"}, planPath)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(body, "# 📋️ Plan") {
		t.Fatalf("missing heading: %q", body)
	}
	if !strings.Contains(body, "<details>") || !strings.Contains(body, "feature_abcd1234.plan.md") {
		t.Fatalf("missing details block: %q", body)
	}
	if strings.Contains(body, "name: Feature") {
		t.Fatalf("frontmatter should be stripped: %q", body)
	}
	if !strings.Contains(body, "## Steps") {
		t.Fatalf("missing body: %q", body)
	}
}

func TestFormatPlanCommentSpecDir(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "b.md"), []byte("## B\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "a.md"), []byte("## A\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&model.TicketPlan{Client: "kiro", ID: "my-spec"}, specDir)
	if err != nil {
		t.Fatal(err)
	}
	aPos := strings.Index(body, "a.md")
	bPos := strings.Index(body, "b.md")
	if aPos < 0 || bPos < 0 || aPos > bPos {
		t.Fatalf("expected sorted sections a before b: %q", body)
	}
}

type captureCommentProvider struct {
	providers.NullManagementProvider
	comments []struct {
		url  string
		body string
	}
}

func (p *captureCommentProvider) AddComment(issueURL, comment string) error {
	p.comments = append(p.comments, struct {
		url  string
		body string
	}{issueURL, comment})
	return nil
}

func TestPostTicketPlanComment(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "task_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Task\n---\n\n## Work\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	capture := &captureCommentProvider{}
	old := providers.MgmtProvider
	providers.MgmtProvider = capture
	defer func() { providers.MgmtProvider = old }()

	ticket := &model.Ticket{
		Management: &model.TicketManagementData{Issue: "https://github.com/example/repo/issues/1"},
		Plan:       &model.TicketPlan{Source: planPath, Client: "cursor", ID: "abcd1234"},
	}
	postTicketPlanComment(ticket, false)
	if len(capture.comments) != 1 {
		t.Fatalf("expected 1 comment, got %d", len(capture.comments))
	}
	if !strings.Contains(capture.comments[0].body, "# 📋️ Plan") {
		t.Fatalf("missing plan heading: %q", capture.comments[0].body)
	}
	if !strings.Contains(capture.comments[0].body, "## Work") {
		t.Fatalf("missing plan body: %q", capture.comments[0].body)
	}
	postTicketPlanComment(ticket, true)
	if len(capture.comments) != 1 {
		t.Fatalf("noManagement should skip comment, got %d", len(capture.comments))
	}
}

func TestPostTicketPlanCommentLive(t *testing.T) {
	if os.Getenv("REPO_LIVE_GH") != "1" {
		t.Skip("set REPO_LIVE_GH=1 to run live GitHub plan comment test")
	}
	planFile := filepath.Join(workspace.RootDir, ".cursor", "plans", "post_plan_to_issue_c256e28e.plan.md")
	if _, err := os.Stat(planFile); err != nil {
		t.Skip("plan file missing")
	}
	issueURL, err := providers.GhCreateIssue("TEST Post Plan Comment", "# temp", nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = providers.GhCloseIssue(issueURL) })
	ticket := &model.Ticket{
		Management: &model.TicketManagementData{Issue: issueURL},
		Plan:       &model.TicketPlan{Source: planFile, Client: "cursor", ID: "c256e28e"},
	}
	postTicketPlanComment(ticket, false)
	issueNum := issueURL[strings.LastIndex(issueURL, "/")+1:]
	if issueNum == "" {
		t.Fatal("could not parse issue number")
	}
	stdout, stderr, code := workspace.ExecCommand("gh", []string{"api", "repos/{owner}/{repo}/issues/" + issueNum + "/comments", "--jq", ".[].body"}, "")
	if code != 0 {
		t.Fatalf("gh api comments failed: %s", stderr)
	}
	if !strings.Contains(stdout, "# 📋️ Plan") {
		t.Fatalf("issue comment missing plan heading: %q", stdout)
	}
	if !strings.Contains(stdout, "post_plan_to_issue_c256e28e.plan.md") {
		t.Fatalf("issue comment missing plan filename: %q", stdout)
	}
}
