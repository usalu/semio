// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket important documents, lifecycle, and generated output retention.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func setupTicketDir(t *testing.T) (string, string) {
	t.Helper()
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets",
		FormatYearDir(now.Year()%100),
		FormatMonthDir(int(now.Month())),
		FormatDayDir(now.Day()),
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

func setupImportantTicket(t *testing.T, status TicketStatus, content []byte) (*Ticket, string) {
	t.Helper()
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	t.Cleanup(func() { rootDir = oldRoot })
	ticket := &Ticket{
		Year:       26,
		Month:      8,
		Day:        26,
		Slug:       "IMPORTANT-LIFECYCLE",
		Title:      "Important Lifecycle",
		Status:     status,
		Goal:       "TEST/GOAL",
		FolderPath: GetTicketPath(26, 8, 26, "IMPORTANT-LIFECYCLE"),
		JsonPath:   GetTicketJsonPath(26, 8, 26, "IMPORTANT-LIFECYCLE"),
		Interactions: []Interaction{{
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
	oldRoot := rootDir
	rootDir = root
	t.Cleanup(func() { rootDir = oldRoot })
	want := filepath.Join(GetTicketPath(26, 8, 26, "PATH-TEST"), "📌️important", "📝️.md")
	if got := GetImportantFilePath(26, 8, 26, "PATH-TEST"); got != want {
		t.Fatalf("GetImportantFilePath() = %q, want %q", got, want)
	}
}

func TestTicketUnmarshalRequiresExplicitValidStatus(t *testing.T) {
	for _, test := range []struct {
		name    string
		payload string
		wantErr bool
	}{
		{name: "open", payload: `{"title":"Ticket","status":"open"}`},
		{name: "closed", payload: `{"title":"Ticket","status":"closed"}`},
		{name: "missing", payload: `{"title":"Ticket"}`, wantErr: true},
		{name: "empty", payload: `{"title":"Ticket","status":""}`, wantErr: true},
		{name: "unknown", payload: `{"title":"Ticket","status":"paused"}`, wantErr: true},
	} {
		t.Run(test.name, func(t *testing.T) {
			var ticket Ticket
			err := json.Unmarshal([]byte(test.payload), &ticket)
			if (err != nil) != test.wantErr {
				t.Fatalf("json.Unmarshal() error = %v, wantErr %v", err, test.wantErr)
			}
		})
	}
}

func TestCreateReadAndRenameTicketUseCanonicalImportantPath(t *testing.T) {
	root := t.TempDir()
	oldRoot := rootDir
	rootDir = root
	t.Cleanup(func() { rootDir = oldRoot })
	testSessionIDOverride = "important-create-session"
	t.Cleanup(func() { testSessionIDOverride = "" })
	ticket, err := CreateTicket("🎫️", "Canonical Important", "Prompt", "", "", "copilot-chat", "", true, "TEST/GOAL", "", true, "", McpClientGeneric, "", "")
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
	oldRoot := rootDir
	rootDir = root
	t.Cleanup(func() { rootDir = oldRoot })
	year, month, day := FormatDate(time.Now())
	path := GetImportantFilePath(year, month, day, "CANONICAL-IMPORTANT-COLLISION")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte("preserve"), 0644); err != nil {
		t.Fatal(err)
	}
	if _, err := CreateTicket("🎫️", "Canonical Important Collision", "Prompt", "", "", "copilot-chat", "", true, "TEST/GOAL", "", true, "", McpClientGeneric, "", ""); err == nil {
		t.Fatal("CreateTicket() overwrote an existing ticket root")
	}
	if data, err := os.ReadFile(path); err != nil || string(data) != "preserve" {
		t.Fatalf("existing important document changed: %q, %v", data, err)
	}
}

func TestTicketLifecycleRejectsInvalidStatusWithoutMutation(t *testing.T) {
	ticket, _ := setupImportantTicket(t, TicketStatusOpen, []byte{})
	manifest := []byte("preserve manifest")
	if err := os.WriteFile(ticket.JsonPath, manifest, 0644); err != nil {
		t.Fatal(err)
	}
	ticket.Status = TicketStatus("paused")
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
	if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, McpClientGeneric, "", ""); err == nil {
		t.Fatal("ReopenTicket() accepted an invalid status")
	}
	if ticket.Status != TicketStatus("paused") || len(ticket.Interactions) != beforeInteractions {
		t.Fatalf("invalid lifecycle mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
	}
}

func TestFinishTicketImportantLifecycle(t *testing.T) {
	for _, bulk := range []bool{false, true} {
		t.Run(fmt.Sprintf("empty-bulk-%v", bulk), func(t *testing.T) {
			ticket, root := setupImportantTicket(t, TicketStatusOpen, []byte{})
			changed := filepath.Join(root, "changed.txt")
			if err := os.WriteFile(changed, []byte("changed"), 0644); err != nil {
				t.Fatal(err)
			}
			if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, bulk); err != nil {
				t.Fatalf("FinishTicket() failed: %v", err)
			}
			if ticket.Status != TicketStatusClosed {
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
			ticket, _ := setupImportantTicket(t, TicketStatusOpen, test.content)
			beforeInteractions := len(ticket.Interactions)
			if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, test.bulk); err == nil {
				t.Fatal("FinishTicket() succeeded for a nonempty important document")
			}
			if ticket.Status != TicketStatusOpen || len(ticket.Interactions) != beforeInteractions {
				t.Fatalf("failed close mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
			}
			if data, err := os.ReadFile(ticket.ImportantPath); err != nil || !bytes.Equal(data, test.content) {
				t.Fatalf("important document changed: %q, %v", data, err)
			}
		})
	}

	t.Run("missing", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusOpen, nil)
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded without an important document")
		}
		if ticket.Status != TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
	})

	t.Run("non-regular", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusOpen, nil)
		if err := os.MkdirAll(ticket.ImportantPath, 0755); err != nil {
			t.Fatal(err)
		}
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded for a non-regular important document")
		}
		if ticket.Status != TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
	})

	t.Run("save-failure-rolls-back", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusOpen, []byte{})
		beforeInteractions := append([]Interaction(nil), ticket.Interactions...)
		ticket.JsonPath = ticket.FolderPath
		if err := FinishTicket(ticket, "done", []string{"changed.txt"}, true, false); err == nil {
			t.Fatal("FinishTicket() succeeded with an unwritable manifest path")
		}
		if ticket.Status != TicketStatusOpen || len(ticket.Interactions) != len(beforeInteractions) {
			t.Fatalf("failed close mutated ticket: status=%q interactions=%d", ticket.Status, len(ticket.Interactions))
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || len(data) != 0 {
			t.Fatalf("important document was not restored: %q, %v", data, err)
		}
	})
}

func TestReopenTicketImportantLifecycle(t *testing.T) {
	t.Run("recreates-missing", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusClosed, nil)
		testSessionIDOverride = "important-reopen-create"
		t.Cleanup(func() { testSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, McpClientGeneric, "", ""); err != nil {
			t.Fatalf("ReopenTicket() failed: %v", err)
		}
		if ticket.Status != TicketStatusOpen {
			t.Fatalf("status = %q, want open", ticket.Status)
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || len(data) != 0 {
			t.Fatalf("reopened important document = %q, %v", data, err)
		}
	})

	t.Run("preserves-nonempty", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusClosed, []byte("preserve"))
		testSessionIDOverride = "important-reopen-preserve"
		t.Cleanup(func() { testSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, McpClientGeneric, "", ""); err != nil {
			t.Fatalf("ReopenTicket() failed: %v", err)
		}
		if data, err := os.ReadFile(ticket.ImportantPath); err != nil || string(data) != "preserve" {
			t.Fatalf("existing important document changed: %q, %v", data, err)
		}
	})

	t.Run("save-failure-rolls-back-created-document", func(t *testing.T) {
		ticket, _ := setupImportantTicket(t, TicketStatusClosed, nil)
		beforeInteractions := len(ticket.Interactions)
		beforeSessions := len(ticket.Sessions)
		ticket.JsonPath = ticket.FolderPath
		testSessionIDOverride = "important-reopen-rollback"
		t.Cleanup(func() { testSessionIDOverride = "" })
		if err := ReopenTicket(ticket, "continue", "", "", "copilot-chat", "", "", "", true, McpClientGeneric, "", ""); err == nil {
			t.Fatal("ReopenTicket() succeeded with an unwritable manifest path")
		}
		if ticket.Status != TicketStatusClosed || len(ticket.Interactions) != beforeInteractions || len(ticket.Sessions) != beforeSessions {
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

// 🛟️ TestTicketGeneratedOutputRetention checks the language-neutral generated-only lifecycle boundary.
func TestTicketGeneratedOutputRetention(t *testing.T) {
	data, err := os.ReadFile(filepath.Join(GetRootDir(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests/🧫️fixtures/🔓️open-close-reopen-lifecycle/🔓️lifecycle.json"))
	if err != nil { t.Fatal(err) }
	var fixture struct { Purge struct { Retention []struct { Path string; Bytes int64; Retained bool } } }
	if err := json.Unmarshal(data, &fixture); err != nil { t.Fatal(err) }
	if len(fixture.Purge.Retention) != 16 { t.Fatalf("retention vectors = %d", len(fixture.Purge.Retention)) }
	root := t.TempDir()
	for _, row := range fixture.Purge.Retention { writeSparseTicketArtifact(t, filepath.Join(root, filepath.FromSlash(row.Path)), row.Bytes) }
	if err := purgeOversizedTicketArtifacts(root); err != nil { t.Fatal(err) }
	for _, row := range fixture.Purge.Retention {
		info, err := os.Stat(filepath.Join(root, filepath.FromSlash(row.Path)))
		if row.Retained && (err != nil || info.Size() != row.Bytes) { t.Errorf("required retained artifact %s: %v", row.Path, err) }
		if !row.Retained && !os.IsNotExist(err) { t.Errorf("generated artifact still exists: %s", row.Path) }
	}
}

// 🧭️ TestTicketGeneratedOutputRoots preserves authored data with missing, non-directory and symbolic generated roots.
func TestTicketGeneratedOutputRoots(t *testing.T) {
	for _, mode := range []string{"absent", "file", "symlink"} {
		t.Run(mode, func(t *testing.T) {
			root, external := t.TempDir(), t.TempDir()
			authored := filepath.Join(root, "📥️authored-inputs", "source.json")
			outside := filepath.Join(external, "blob.bin")
			writeSparseTicketArtifact(t, authored, 11<<20)
			writeSparseTicketArtifact(t, outside, 11<<20)
			output := filepath.Join(root, "🗑️generated")
			if mode == "file" { writeSparseTicketArtifact(t, output, 11<<20) }
			if mode == "symlink" { if err := os.Symlink(external, output); err != nil { t.Skipf("platform does not permit symbolic directory: %v", err) } }
			err := purgeOversizedTicketArtifacts(root)
			if (mode == "file") != (err != nil) { t.Fatalf("generated root %s: %v", mode, err) }
			for _, path := range []string{authored, outside} { info, err := os.Stat(path); if err != nil || info.Size() != 11<<20 { t.Fatalf("retained source %s: %v", path, err) } }
		})
	}
}

func TestFinishTicketPurgesOversizedArtifacts(t *testing.T) {
	tmpDir, ticketJSON := setupTicketDir(t)
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	ticketDir := filepath.Dir(ticketJSON)
	now := time.Now().UTC()
	ticket := &Ticket{
		Year:       now.Year() % 100,
		Month:      int(now.Month()),
		Day:        now.Day(),
		Slug:       "TEST-TICKET",
		Title:      "Test Ticket",
		Status:     TicketStatusOpen,
		FolderPath: ticketDir,
		JsonPath:   ticketJSON,
		Interactions: []Interaction{{
			Kind: "ticket.open",
			Date: "2026-01-01 00:00:00",
		}},
	}

	const mib = 1024 * 1024
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "large.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "exact-5mb.bin"), 5*mib)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", ".cache.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "huge-dir", "blob.bin"), 10*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "small-dir", "small.bin"), 1*mib)

	testFile := "changed.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("x"), 0644); err != nil {
		t.Fatal(err)
	}

	if err := FinishTicket(ticket, "Summary", []string{testFile}, true, false); err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.Status != TicketStatusClosed {
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

	assertNotExists(filepath.Join(ticketDir, "🗑️generated", "large.bin"))
	assertNotExists(filepath.Join(ticketDir, "🗑️generated", ".cache.bin"))
	assertNotExists(filepath.Join(ticketDir, "🗑️generated", "huge-dir"))
	assertExists(filepath.Join(ticketDir, "🗑️generated", "exact-5mb.bin"))
	assertExists(filepath.Join(ticketDir, "🗑️generated", "small-dir", "small.bin"))
	assertExists(ticketJSON)
}

func TestPurgeAllOversizedTicketArtifacts(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	now := time.Now().UTC()
	year := now.Year() % 100
	month := int(now.Month())
	day := now.Day()
	slug := "TEST-TICKET"
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets", FormatYearDir(year), FormatMonthDir(month), FormatDayDir(day), slug)
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "🎫️ticket.json")
	if err := os.WriteFile(ticketJSON, []byte(`{"title":"Test Ticket","goal":"TEST/GOAL"}`), 0644); err != nil {
		t.Fatal(err)
	}

	ticket := &Ticket{
		Year:       year,
		Month:      month,
		Day:        day,
		Slug:       slug,
		Title:      "Test Ticket",
		Status:     TicketStatusOpen,
		FolderPath: ticketDir,
		JsonPath:   ticketJSON,
	}
	if err := SaveTicket(ticket); err != nil {
		t.Fatal(err)
	}

	const mib = 1024 * 1024
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "large.bin"), 5*mib+1)
	writeSparseTicketArtifact(t, filepath.Join(ticketDir, "🗑️generated", "huge-dir", "blob.bin"), 10*mib+1)

	count, err := PurgeAllOversizedTicketArtifacts()
	if err != nil {
		t.Fatalf("PurgeAllOversizedTicketArtifacts failed: %v", err)
	}
	if count != 1 {
		t.Fatalf("purged ticket count = %d, want 1", count)
	}
	if _, err := os.Stat(filepath.Join(ticketDir, "🗑️generated", "large.bin")); !os.IsNotExist(err) {
		t.Fatalf("expected deleted large.bin")
	}
	if _, err := os.Stat(filepath.Join(ticketDir, "🗑️generated", "huge-dir")); !os.IsNotExist(err) {
		t.Fatalf("expected deleted huge-dir")
	}
	if _, err := os.Stat(ticketJSON); err != nil {
		t.Fatalf("expected kept ticket.json: %v", err)
	}
}
