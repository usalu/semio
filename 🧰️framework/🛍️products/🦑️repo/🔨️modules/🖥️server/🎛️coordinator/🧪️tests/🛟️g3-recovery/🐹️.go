// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 event-store crash-recovery contract tests.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// #region 🛟️G3Recovery

func TestG3InterruptedWriteAtEveryDurablePhase(t *testing.T) {
	phases := []string{"next-synced", "stage-synced", "prior-backed-up", "log-replaced"}
	for _, phase := range phases {
		t.Run(phase, func(t *testing.T) {
			store, path := testStore(t)
			appendTestEvent(t, store, 0, "prior")
			interrupted := errors.New("simulated interruption")
			store.interrupt = func(current string) error {
				if current == phase {
					return interrupted
				}
				return nil
			}
			_, err := store.Append(context.Background(), 1, []EventInput{testInput("next", `{}`)}, nil)
			if !errors.Is(err, interrupted) {
				t.Fatalf("expected interruption, got %v", err)
			}
			reopened, err := OpenEventStore(context.Background(), path, DefaultStoreLimits())
			if err != nil {
				t.Fatalf("recover: %v", err)
			}
			events, err := reopened.Replay(context.Background(), nil)
			if err != nil {
				t.Fatal(err)
			}
			if len(events) != 1 || events[0].ID != "prior" {
				t.Fatalf("phase=%s events=%+v", phase, events)
			}
			for _, suffix := range []string{".next", ".stage", ".stage.next", ".backup"} {
				if _, statErr := os.Stat(path + suffix); !os.IsNotExist(statErr) {
					t.Fatalf("recovery retained %s: %v", suffix, statErr)
				}
			}
		})
	}
}

func TestG3CommitAcknowledgementNeverReportsFailureAfterDurability(t *testing.T) {
	store, _ := testStore(t)
	store.interrupt = func(phase string) error {
		if phase == "committed" {
			return errors.New("late interruption")
		}
		return nil
	}
	result, err := store.Append(context.Background(), 0, []EventInput{testInput("committed", `{}`)}, nil)
	if err != nil || len(result.Events) != 1 {
		t.Fatalf("durable commit reported failure: result=%+v err=%v", result, err)
	}
	events, replayErr := store.Replay(context.Background(), nil)
	if replayErr != nil || len(events) != 1 || events[0].ID != "committed" {
		t.Fatalf("durable commit was not replayable: events=%+v err=%v", events, replayErr)
	}
}

func TestG3MetadataDurabilityProtocolAndExplicitFailure(t *testing.T) {
	t.Run("every-mutation-is-synced", func(t *testing.T) {
		store, operations, _ := memoryTestStore(t)
		appendTestEvent(t, store, 0, "prior")
		operations.mu.Lock()
		operations.operations = nil
		operations.mu.Unlock()
		appendTestEvent(t, store, 1, "next")
		operations.mu.Lock()
		recorded := append([]memoryStoreOperation(nil), operations.operations...)
		operations.mu.Unlock()
		for index, operation := range recorded {
			if (operation.Kind != "rename" && operation.Kind != "remove") || strings.HasSuffix(operation.Path, ".lock") {
				continue
			}
			if index+1 >= len(recorded) || recorded[index+1].Kind != "sync-parent" {
				t.Fatalf("metadata mutation lacks immediate parent sync: %v", recorded)
			}
		}
	})
	t.Run("live-replacement-fails-and-rolls-back", func(t *testing.T) {
		store, operations, path := memoryTestStore(t)
		appendTestEvent(t, store, 0, "prior")
		before := operations.bytes(path)
		injected := errors.New("live replacement failed")
		operations.setFault(func(operation memoryStoreOperation) bool {
			return operation.Kind == "rename" && operation.Source == filepath.Clean(path+".next") && operation.Destination == filepath.Clean(path)
		}, injected)
		result, err := store.Append(context.Background(), 1, []EventInput{testInput("rejected", `{}`)}, nil)
		if !errors.Is(err, injected) || result.Committed {
			t.Fatalf("replacement result=%+v err=%v", result, err)
		}
		if after := operations.bytes(path); !bytes.Equal(before, after) {
			t.Fatal("failed live replacement changed last-valid log")
		}
		operations.clearFault()
		reopened, openErr := openEventStoreWithOperations(context.Background(), path, DefaultStoreLimits(), operations)
		if openErr != nil {
			t.Fatal(openErr)
		}
		events, replayErr := reopened.Replay(context.Background(), nil)
		if replayErr != nil || len(events) != 1 || events[0].ID != "prior" {
			t.Fatalf("rollback recovery events=%+v err=%v", events, replayErr)
		}
	})
}

func TestG3PartialTailRecoveryPreservesLastValidEvent(t *testing.T) {
	store, path := testStore(t)
	appendTestEvent(t, store, 0, "valid")
	valid, _ := os.ReadFile(path)
	partial := []byte(`{"stream":"coordinator","sequence":2,"id":"partial"`)
	if err := os.WriteFile(path, append(append([]byte(nil), valid...), partial...), 0o600); err != nil {
		t.Fatal(err)
	}
	reopened, err := OpenEventStore(context.Background(), path, DefaultStoreLimits())
	if err != nil {
		t.Fatal(err)
	}
	events, err := reopened.Replay(context.Background(), nil)
	if err != nil || len(events) != 1 || events[0].ID != "valid" {
		t.Fatalf("events=%+v err=%v", events, err)
	}
	after, _ := os.ReadFile(path)
	if !bytes.Equal(after, valid) {
		t.Fatal("partial-tail recovery did not retain exact valid prefix")
	}
}

func TestG3CorruptTruncatedAndChecksumFailuresPreserveBytes(t *testing.T) {
	store, path := testStore(t)
	appendTestEvent(t, store, 0, "valid")
	valid, _ := os.ReadFile(path)
	cases := map[string][]byte{
		"malformed": []byte("{not-json}\n"),
		"checksum":  bytes.Replace(valid, []byte(`"checksum":"`), []byte(`"checksum":"0`), 1),
		"sequence":  bytes.Replace(valid, []byte(`"sequence":1`), []byte(`"sequence":2`), 1),
	}
	for name, data := range cases {
		t.Run(name, func(t *testing.T) {
			corruptPath := filepath.Join(t.TempDir(), "corrupt.events")
			if err := os.WriteFile(corruptPath, data, 0o600); err != nil {
				t.Fatal(err)
			}
			before, _ := os.ReadFile(corruptPath)
			_, err := OpenEventStore(context.Background(), corruptPath, DefaultStoreLimits())
			if !errors.Is(err, ErrStoreCorrupt) && !errors.Is(err, ErrDuplicateEvent) {
				t.Fatalf("expected explicit corruption, got %v", err)
			}
			after, _ := os.ReadFile(corruptPath)
			if !bytes.Equal(before, after) {
				t.Fatal("full-record corruption was silently mutated")
			}
		})
	}
}

func TestG3InvalidStageAndOrphanBackupFailClosed(t *testing.T) {
	store, path := testStore(t)
	appendTestEvent(t, store, 0, "valid")
	if err := os.WriteFile(path+".stage", []byte("{}\n"), 0o600); err != nil {
		t.Fatal(err)
	}
	if _, err := OpenEventStore(context.Background(), path, DefaultStoreLimits()); !errors.Is(err, ErrStoreCorrupt) {
		t.Fatalf("invalid stage error=%v", err)
	}
	_ = os.Remove(path + ".stage")
	copyFile(t, path, path+".backup")
	if _, err := OpenEventStore(context.Background(), path, DefaultStoreLimits()); !errors.Is(err, ErrStoreCorrupt) {
		t.Fatalf("orphan backup error=%v", err)
	}
}

// #endregion 🛟️G3Recovery
