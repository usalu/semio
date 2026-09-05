// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 language-neutral event-store contract tests.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"runtime"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	repopkg "github.com/usalu/semio/repo/go"
)

// #region 🧰️Fixtures

func testStore(t *testing.T) (*EventStore, string) {
	t.Helper()
	path := filepath.Join(t.TempDir(), "coordinator.events")
	store, err := OpenEventStore(context.Background(), path, DefaultStoreLimits())
	if err != nil {
		t.Fatalf("open store: %v", err)
	}
	return store, path
}

func testInput(id string, payload string) EventInput {
	return EventInput{Stream: coordinatorStream, ID: id, Generation: 1, Type: "test.recorded", Payload: json.RawMessage(payload)}
}

func appendTestEvent(t *testing.T, store *EventStore, expected uint64, id string) EventEnvelope {
	t.Helper()
	result, err := store.Append(context.Background(), expected, []EventInput{testInput(id, fmt.Sprintf(`{"id":%q}`, id))}, nil)
	if err != nil {
		t.Fatalf("append %s: %v", id, err)
	}
	return result.Events[0]
}

func copyFile(t *testing.T, source string, destination string) {
	t.Helper()
	data, err := os.ReadFile(source)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(destination, data, 0o600); err != nil {
		t.Fatal(err)
	}
}

type auditRepository struct {
	CoordinatorRepository
	err error
}

func (repository *auditRepository) recordPublishedEvent(context.Context, Event) error {
	return nil
}

func (repository *auditRepository) recordScopes(context.Context, string, []Scope) error {
	return repository.err
}

func (repository *auditRepository) recordCheckpoint(context.Context, string, []string) error {
	return repository.err
}

func (repository *auditRepository) recordContributorWork(context.Context, string, string, string) error {
	return repository.err
}

func (repository *auditRepository) projectContributorsOnItem(context.Context, string, string, ...ProjectionQuery) ([]string, error) {
	return nil, nil
}

// #endregion 🧰️Fixtures

// #region 📜️Golden

func TestG3LanguageNeutralGoldenEnvelope(t *testing.T) {
	var schema struct {
		Schema   string   `json:"schema"`
		Encoding string   `json:"encoding"`
		Fields   []string `json:"fields"`
		Checksum string   `json:"checksum"`
	}
	schemaBytes, err := os.ReadFile(filepath.Join("🧫️fixtures", "🧬️g3-event-schema.json"))
	if err != nil || json.Unmarshal(schemaBytes, &schema) != nil {
		t.Fatalf("read language-neutral schema: %v", err)
	}
	fields := []string{"stream", "sequence", "id", "generation", "type", "payload", "checksum"}
	if schema.Schema != "semio.coordinator.event/1" || schema.Encoding != "canonical-jsonl-lf" || !equalStringsInOrder(schema.Fields, fields) || schema.Checksum == "" {
		t.Fatalf("unexpected language-neutral schema %+v", schema)
	}
	store, path := testStore(t)
	payload := json.RawMessage(`{"id":"T-1","status":"open"}`)
	result, err := store.Append(context.Background(), 0, []EventInput{{Stream: coordinatorStream, ID: "evt-1", Generation: 7, Type: "ticket.recorded", Payload: payload}}, nil)
	if err != nil {
		t.Fatal(err)
	}
	if result.Events[0].Checksum != "6d6e63ada99d3b4b6ed21114f00421e3c0bb6d480c830351f4b7f323c6fa82ec" {
		t.Fatalf("unexpected checksum %s", result.Events[0].Checksum)
	}
	actual, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	expected, err := os.ReadFile(filepath.Join("🧫️fixtures", "📜️g3-event-log.jsonl"))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(actual, expected) {
		t.Fatalf("golden mismatch\nactual: %s\nexpected: %s", actual, expected)
	}
	if bytes.Contains(actual, []byte{'\r'}) {
		t.Fatal("event bytes must use platform-neutral LF records")
	}
}

func TestG3EmptyAppendReplayAndProgress(t *testing.T) {
	store, _ := testStore(t)
	events, err := store.Replay(context.Background(), nil)
	if err != nil || len(events) != 0 {
		t.Fatalf("empty replay events=%d err=%v", len(events), err)
	}
	var phases []string
	inputs := []EventInput{testInput("a", `{"n":1}`), testInput("b", `{"n":2}`)}
	result, err := store.Append(context.Background(), 0, inputs, func(progress StoreProgress) {
		phases = append(phases, progress.Phase)
	})
	if err != nil {
		t.Fatal(err)
	}
	if len(result.Events) != 2 || result.Events[0].Sequence != 1 || result.Events[1].Sequence != 2 {
		t.Fatalf("unexpected append result %+v", result)
	}
	events, err = store.Replay(context.Background(), nil)
	if err != nil || len(events) != 2 || events[0].ID != "a" || events[1].ID != "b" {
		t.Fatalf("unexpected replay %+v err=%v", events, err)
	}
	for _, required := range []string{"encoded", "next-synced", "stage-synced", "prior-backed-up", "log-replaced", "committed"} {
		if !containsString(phases, required) {
			t.Fatalf("missing progress phase %q in %v", required, phases)
		}
	}
}

// #endregion 📜️Golden

// #region 🔐️Concurrency

func TestG3ConcurrentWritersAndExpectedSequence(t *testing.T) {
	store, path := testStore(t)
	const writers = 24
	var wait sync.WaitGroup
	var failures atomic.Int64
	for index := 0; index < writers; index++ {
		wait.Add(1)
		go func(index int) {
			defer wait.Done()
			writer, err := OpenEventStore(context.Background(), path, DefaultStoreLimits())
			if err != nil {
				failures.Add(1)
				return
			}
			id := fmt.Sprintf("writer-%02d", index)
			for attempt := 0; attempt < writers*4; attempt++ {
				events, replayErr := writer.Replay(context.Background(), nil)
				if replayErr != nil {
					failures.Add(1)
					return
				}
				_, appendErr := writer.Append(context.Background(), uint64(len(events)), []EventInput{testInput(id, fmt.Sprintf(`{"writer":%d}`, index))}, nil)
				if appendErr == nil {
					return
				}
				if !errors.Is(appendErr, ErrSequenceConflict) {
					failures.Add(1)
					return
				}
				runtime.Gosched()
			}
			failures.Add(1)
		}(index)
	}
	wait.Wait()
	if failures.Load() != 0 {
		t.Fatalf("concurrent writer failures=%d", failures.Load())
	}
	events, err := store.Replay(context.Background(), nil)
	if err != nil || len(events) != writers {
		t.Fatalf("replay count=%d err=%v", len(events), err)
	}
	seen := map[string]struct{}{}
	for index, event := range events {
		if event.Sequence != uint64(index+1) {
			t.Fatalf("sequence %d at index %d", event.Sequence, index)
		}
		seen[event.ID] = struct{}{}
	}
	if len(seen) != writers {
		t.Fatalf("unique ids=%d", len(seen))
	}
}

func TestG3ExpectedSequenceConflictPreservesLog(t *testing.T) {
	store, path := testStore(t)
	appendTestEvent(t, store, 0, "first")
	before, _ := os.ReadFile(path)
	_, err := store.Append(context.Background(), 0, []EventInput{testInput("second", `{}`)}, nil)
	if !errors.Is(err, ErrSequenceConflict) {
		t.Fatalf("expected sequence conflict, got %v", err)
	}
	after, _ := os.ReadFile(path)
	if !bytes.Equal(before, after) {
		t.Fatal("sequence conflict changed durable bytes")
	}
}

func TestG3DuplicateEventIsIdempotentAndMismatchIsExplicit(t *testing.T) {
	store, path := testStore(t)
	input := testInput("same", `{"value":1}`)
	first, err := store.Append(context.Background(), 0, []EventInput{input}, nil)
	if err != nil {
		t.Fatal(err)
	}
	before, _ := os.ReadFile(path)
	retry, err := store.Append(context.Background(), 0, []EventInput{input}, nil)
	if err != nil || !retry.Duplicate || retry.Events[0].Sequence != first.Events[0].Sequence {
		t.Fatalf("idempotent retry=%+v err=%v", retry, err)
	}
	after, _ := os.ReadFile(path)
	if !bytes.Equal(before, after) {
		t.Fatal("idempotent retry changed bytes")
	}
	canonicalRetry := input
	canonicalRetry.Payload = json.RawMessage(`{ "value" : 1 }`)
	canonical, err := store.Append(context.Background(), 0, []EventInput{canonicalRetry}, nil)
	if err != nil || !canonical.Duplicate {
		t.Fatalf("canonical idempotent retry=%+v err=%v", canonical, err)
	}
	mismatch := input
	mismatch.Payload = json.RawMessage(`{"value":2}`)
	_, err = store.Append(context.Background(), 1, []EventInput{mismatch}, nil)
	if !errors.Is(err, ErrDuplicateEvent) {
		t.Fatalf("expected duplicate mismatch, got %v", err)
	}
	_, err = store.Append(context.Background(), 1, []EventInput{input, testInput("new", `{}`)}, nil)
	if !errors.Is(err, ErrDuplicateEvent) {
		t.Fatalf("expected mixed duplicate rejection, got %v", err)
	}
}

// #endregion 🔐️Concurrency

// #region 🛟️Recovery

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

// #endregion 🛟️Recovery

// #region ⛔️CancellationAndBounds

func TestG3CancellationBeforeDuringAndAfterCommit(t *testing.T) {
	t.Run("before", func(t *testing.T) {
		store, path := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		cancel()
		_, err := store.Append(ctx, 0, []EventInput{testInput("before", `{}`)}, nil)
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("error=%v", err)
		}
		if _, statErr := os.Stat(path); !os.IsNotExist(statErr) {
			t.Fatalf("cancel-before created log: %v", statErr)
		}
	})
	t.Run("during", func(t *testing.T) {
		store, path := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		_, err := store.Append(ctx, 0, []EventInput{testInput("during", `{}`)}, func(progress StoreProgress) {
			if progress.Phase == "stage-synced" {
				cancel()
			}
		})
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("error=%v", err)
		}
		if _, statErr := os.Stat(path); !os.IsNotExist(statErr) {
			t.Fatalf("cancel-during created log: %v", statErr)
		}
	})
	t.Run("after", func(t *testing.T) {
		store, _ := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		result, err := store.Append(ctx, 0, []EventInput{testInput("after", `{}`)}, func(progress StoreProgress) {
			if progress.Phase == "committed" {
				cancel()
			}
		})
		if err != nil || len(result.Events) != 1 {
			t.Fatalf("committed append result=%+v err=%v", result, err)
		}
		events, replayErr := store.Replay(context.Background(), nil)
		if replayErr != nil || len(events) != 1 {
			t.Fatalf("events=%+v err=%v", events, replayErr)
		}
	})
	t.Run("during-replay", func(t *testing.T) {
		store, _ := testStore(t)
		_, err := store.Append(context.Background(), 0, []EventInput{testInput("replay-a", `{}`), testInput("replay-b", `{}`)}, nil)
		if err != nil {
			t.Fatal(err)
		}
		ctx, cancel := context.WithCancel(context.Background())
		_, err = store.Replay(ctx, func(progress StoreProgress) {
			if progress.Phase == "replayed" && progress.Current == 1 {
				cancel()
			}
		})
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("replay cancellation error=%v", err)
		}
	})
}

func TestG3PayloadEventDepthAndLogBounds(t *testing.T) {
	limits := DefaultStoreLimits()
	limits.MaxPayloadBytes = len(`{"a":1}`)
	limits.MaxAppendEvents = 2
	limits.MaxJSONDepth = 2
	limits.MaxLogBytes = 4096
	limits.MaxAppendBytes = 2048
	path := filepath.Join(t.TempDir(), "bounded.events")
	store, err := OpenEventStore(context.Background(), path, limits)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := store.Append(context.Background(), 0, []EventInput{testInput("maximum", `{"a":1}`)}, nil); err != nil {
		t.Fatalf("maximum payload rejected: %v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("max-plus-one", `{"a":10}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("payload max+1 error=%v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("one", `{}`), testInput("two", `{}`), testInput("three", `{}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("event max+1 error=%v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("depth-maximum", `[[]]`)}, nil); err != nil {
		t.Fatalf("maximum depth rejected: %v", err)
	}
	if _, err := store.Append(context.Background(), 2, []EventInput{testInput("depth-plus-one", `[[[]]]`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("depth max+1 error=%v", err)
	}
	invalidScalar := testInput("invalid\x00id", `{}`)
	if _, err := store.Append(context.Background(), 2, []EventInput{invalidScalar}, nil); err == nil {
		t.Fatal("NUL-containing identity was accepted")
	}
	logLimits := DefaultStoreLimits()
	logLimits.MaxLogBytes = 1
	logStore, err := OpenEventStore(context.Background(), filepath.Join(t.TempDir(), "log.events"), logLimits)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := logStore.Append(context.Background(), 0, []EventInput{testInput("too-large", `{}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("log max+1 error=%v", err)
	}
	replayPath := filepath.Join(t.TempDir(), "replay.events")
	replayStore, err := OpenEventStore(context.Background(), replayPath, DefaultStoreLimits())
	if err != nil {
		t.Fatal(err)
	}
	if _, err := replayStore.Append(context.Background(), 0, []EventInput{testInput("replay-one", `{}`), testInput("replay-two", `{}`)}, nil); err != nil {
		t.Fatal(err)
	}
	replayLimits := DefaultStoreLimits()
	replayLimits.MaxReplayEvents = 1
	if _, err := OpenEventStore(context.Background(), replayPath, replayLimits); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("replay event max+1 error=%v", err)
	}
}

func TestG3UnavailableStoreAndBoundedLockWait(t *testing.T) {
	parent := filepath.Join(t.TempDir(), "not-a-directory")
	if err := os.WriteFile(parent, []byte("x"), 0o600); err != nil {
		t.Fatal(err)
	}
	if _, err := OpenEventStore(context.Background(), filepath.Join(parent, "events"), DefaultStoreLimits()); !errors.Is(err, ErrStoreUnavailable) {
		t.Fatalf("unavailable error=%v", err)
	}
	store, path := testStore(t)
	if err := os.WriteFile(path+".lock", []byte("busy"), 0o600); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Millisecond)
	defer cancel()
	started := time.Now()
	_, err := store.Replay(ctx, nil)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("lock wait error=%v", err)
	}
	if time.Since(started) > time.Second {
		t.Fatal("lock shortage froze past the explicit deadline")
	}
	_ = os.Remove(path + ".lock")
	if events, replayErr := store.Replay(context.Background(), nil); replayErr != nil || len(events) != 0 {
		t.Fatalf("store did not recover after external shortage: events=%v err=%v", events, replayErr)
	}
	unlock, err := store.lock(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel = context.WithTimeout(context.Background(), 15*time.Millisecond)
	started = time.Now()
	_, err = store.Replay(ctx, nil)
	cancel()
	unlock()
	if !errors.Is(err, context.DeadlineExceeded) || time.Since(started) > time.Second {
		t.Fatalf("in-process shortage error=%v duration=%s", err, time.Since(started))
	}
}

// #endregion ⛔️CancellationAndBounds

// #region 🧬️Repository

func TestG3RepositoryCommandsReplayDeterministicProjectionsAndReopen(t *testing.T) {
	path := filepath.Join(t.TempDir(), "repository.events")
	repository, err := openDatabase(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.Background()
	created := time.Date(2026, 8, 25, 10, 11, 12, 0, time.UTC)
	ticketA := Ticket{ID: "A", Status: "open", Title: "Alpha", CreatedAt: created}
	ticketB := Ticket{ID: "B", Status: "open", Title: "Beta", CreatedAt: created}
	for _, ticket := range []Ticket{ticketB, ticketA} {
		if err := repository.recordTicket(ctx, ticket); err != nil {
			t.Fatal(err)
		}
	}
	scope := Scope{ID: "scope", Kind: "definition", FilePath: "a.go", StartLine: 1, EndLine: 2, UpdatedAt: created}
	if err := repository.recordScopes(ctx, "a.go", []Scope{scope}); err != nil {
		t.Fatal(err)
	}
	if err := repository.recordClaim(ctx, "A", "scope", "touched", created); err != nil {
		t.Fatal(err)
	}
	if err := repository.recordClaim(ctx, "B", "scope", "touched", created); err != nil {
		t.Fatal(err)
	}
	warning := Warning{ID: "warning", Kind: "conflict", Severity: "error", TicketID: "A", ScopeID: "scope", CreatedAt: created}
	if err := repository.recordWarnings(ctx, []Warning{warning}); err != nil {
		t.Fatal(err)
	}
	if err := repository.recordContributorWork(ctx, "ueli", "file", "a.go"); err != nil {
		t.Fatal(err)
	}
	before, err := repository.store.Replay(ctx, nil)
	if err != nil {
		t.Fatal(err)
	}
	if err := repository.recordTicket(ctx, ticketA); err != nil {
		t.Fatal(err)
	}
	after, _ := repository.store.Replay(ctx, nil)
	if len(after) != len(before) {
		t.Fatal("identical command was not idempotent")
	}
	if err := repository.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := repository.projectTickets(ctx, ""); !errors.Is(err, ErrStoreUnavailable) {
		t.Fatalf("closed query error=%v", err)
	}
	if err := repository.Reopen(ctx); err != nil {
		t.Fatal(err)
	}
	tickets, err := repository.projectTickets(ctx, "open")
	if err != nil || len(tickets) != 2 || tickets[0].ID != "A" || tickets[1].ID != "B" {
		t.Fatalf("tickets=%+v err=%v", tickets, err)
	}
	claims, _ := repository.projectClaimsByTicket(ctx, "A")
	conflicts, _ := repository.projectConflicts(ctx)
	warnings, _ := repository.projectWarnings(ctx, "A")
	contributors, _ := repository.projectContributorsOnItem(ctx, "file", "a.go")
	if len(claims) != 1 || len(conflicts) != 1 || !equalStrings(conflicts[0].Tickets, []string{"A", "B"}) || len(warnings) != 1 || !equalStrings(contributors, []string{"ueli"}) {
		t.Fatalf("claims=%+v conflicts=%+v warnings=%+v contributors=%v", claims, conflicts, warnings, contributors)
	}
	if err := repository.recordCheckpoint(ctx, "ueli", []string{"a.go"}); err != nil {
		t.Fatal(err)
	}
	contributors, _ = repository.projectContributorsOnItem(ctx, "file", "a.go")
	if len(contributors) != 0 {
		t.Fatalf("checkpoint projection retained contributors %v", contributors)
	}
	reopened, err := openDatabase(path)
	if err != nil {
		t.Fatal(err)
	}
	replayedTickets, _ := reopened.projectTickets(ctx, "")
	if !equalTickets(tickets, replayedTickets) {
		t.Fatalf("replayed projection differs: %+v vs %+v", tickets, replayedTickets)
	}
}

func TestG3ProjectionCancellationAndNotFound(t *testing.T) {
	repository, err := openDatabase(filepath.Join(t.TempDir(), "repository.events"))
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := repository.projectTickets(ctx, ""); !errors.Is(err, context.Canceled) {
		t.Fatalf("query cancellation error=%v", err)
	}
	if _, err := repository.projectTicket(context.Background(), "missing"); !errors.Is(err, ErrProjectionNotFound) {
		t.Fatalf("not found error=%v", err)
	}
}

func TestG3ProjectionBoundsProgressAndDuringCancellationPreserveLastValid(t *testing.T) {
	repository, err := openDatabase(filepath.Join(t.TempDir(), "repository.events"))
	if err != nil {
		t.Fatal(err)
	}
	repository.mu.Lock()
	for index := 0; index < 4; index++ {
		id := fmt.Sprintf("T-%d", index)
		repository.projection.tickets[id] = Ticket{ID: id, Status: "open", Title: id}
	}
	repository.mu.Unlock()
	var progress []ProjectionProgress
	valid, err := repository.projectTickets(context.Background(), "", ProjectionQuery{
		MaxItems:   1_000,
		MaxResults: 4,
		Progress: func(value ProjectionProgress) {
			progress = append(progress, value)
		},
	})
	if err != nil || len(valid) != 4 || len(progress) == 0 {
		t.Fatalf("bounded exact query result=%v progress=%d err=%v", valid, len(progress), err)
	}
	exactItems := progress[len(progress)-1].Current
	if result, err := repository.projectTickets(context.Background(), "", ProjectionQuery{MaxItems: exactItems, MaxResults: 4}); err != nil || !equalTickets(valid, result) {
		t.Fatalf("exact item bound result=%v err=%v", result, err)
	}
	if result, err := repository.projectTickets(context.Background(), "", ProjectionQuery{MaxItems: exactItems - 1, MaxResults: 4}); !errors.Is(err, ErrProjectionLimit) || result != nil {
		t.Fatalf("item max+1 result=%v err=%v", result, err)
	}
	if result, err := repository.projectTickets(context.Background(), "", ProjectionQuery{MaxItems: 1_000, MaxResults: 3}); !errors.Is(err, ErrProjectionLimit) || result != nil {
		t.Fatalf("result max+1 result=%v err=%v", result, err)
	}
	traversal := &projectionTraversal{ctx: context.Background(), query: ProjectionQuery{MaxItems: 1, MaxResults: 10_000}}
	if capacity := projectionResultCapacity(traversal, 10_000); capacity != 1 {
		t.Fatalf("small item budget allocated capacity %d", capacity)
	}
	ctx, cancel := context.WithCancel(context.Background())
	result, err := repository.projectTickets(ctx, "", ProjectionQuery{
		MaxItems:   1_000,
		MaxResults: 4,
		Progress: func(value ProjectionProgress) {
			if strings.Contains(value.Phase, ".sort") {
				cancel()
			}
		},
	})
	if !errors.Is(err, context.Canceled) || result != nil {
		t.Fatalf("during-query cancellation result=%v err=%v", result, err)
	}
	after, err := repository.projectTickets(context.Background(), "")
	if err != nil || !equalTickets(valid, after) {
		t.Fatalf("failed query changed last-valid projection: before=%v after=%v err=%v", valid, after, err)
	}
}

func TestG3EveryProjectionFamilyRetainsOneBudgetedCursor(t *testing.T) {
	repository, err := openDatabase(filepath.Join(t.TempDir(), "repository.events"))
	if err != nil {
		t.Fatal(err)
	}
	repository.mu.Lock()
	repository.projection.tickets["A"] = Ticket{ID: "A", Status: "open"}
	repository.projection.tickets["B"] = Ticket{ID: "B", Status: "open"}
	repository.projection.scopes["a.go"] = map[string]Scope{
		"s1": {ID: "s1", FilePath: "a.go"},
		"s2": {ID: "s2", FilePath: "a.go"},
	}
	repository.projection.claims["A"] = map[string]claimProjection{"s1": {}, "s2": {}}
	repository.projection.claims["B"] = map[string]claimProjection{"s1": {}, "s2": {}}
	repository.projection.warnings["w1"] = Warning{ID: "w1"}
	repository.projection.warnings["w2"] = Warning{ID: "w2"}
	repository.projection.breachs["b1"] = Breach{ID: "b1"}
	repository.projection.breachs["b2"] = Breach{ID: "b2"}
	key := contributorKey("file", "a.go")
	repository.projection.contributors["a"] = map[string]struct{}{key: {}}
	repository.projection.contributors["b"] = map[string]struct{}{key: {}}
	repository.mu.Unlock()

	type queryFamily struct {
		name string
		run  func(context.Context, ProjectionQuery) (any, error)
	}
	families := []queryFamily{
		{name: "tickets", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectTickets(ctx, "", query)
		}},
		{name: "ticket", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectTicket(ctx, "A", query)
		}},
		{name: "scopes", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectScopesByFile(ctx, "a.go", query)
		}},
		{name: "claims", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectClaimsByTicket(ctx, "A", query)
		}},
		{name: "warnings", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectWarnings(ctx, "", query)
		}},
		{name: "breachs", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectBreachs(ctx, "", query)
		}},
		{name: "conflicts", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectConflicts(ctx, query)
		}},
		{name: "contributors", run: func(ctx context.Context, query ProjectionQuery) (any, error) {
			return repository.projectContributorsOnItem(ctx, "file", "a.go", query)
		}},
	}
	for _, family := range families {
		t.Run(family.name, func(t *testing.T) {
			var progress []ProjectionProgress
			valid, err := family.run(context.Background(), ProjectionQuery{
				MaxItems:   10_000,
				MaxResults: 100,
				Progress: func(value ProjectionProgress) {
					progress = append(progress, value)
				},
			})
			if err != nil || valid == nil || len(progress) == 0 {
				t.Fatalf("baseline result=%v progress=%d err=%v", valid, len(progress), err)
			}
			validJSON, _ := json.Marshal(valid)
			exactItems := progress[len(progress)-1].Current
			exact, err := family.run(context.Background(), ProjectionQuery{MaxItems: exactItems, MaxResults: 100})
			exactJSON, _ := json.Marshal(exact)
			if err != nil || !bytes.Equal(validJSON, exactJSON) {
				t.Fatalf("exact bound result=%v err=%v", exact, err)
			}
			if result, err := family.run(context.Background(), ProjectionQuery{MaxItems: exactItems - 1, MaxResults: 100}); !errors.Is(err, ErrProjectionLimit) || !nilProjectionResult(result) {
				t.Fatalf("item max+1 result=%v err=%v", result, err)
			}
			resultCount := 1
			if value := reflect.ValueOf(valid); value.Kind() == reflect.Slice {
				resultCount = value.Len()
			}
			if result, err := family.run(context.Background(), ProjectionQuery{MaxItems: 10_000, MaxResults: resultCount - 1}); !errors.Is(err, ErrProjectionLimit) || !nilProjectionResult(result) {
				t.Fatalf("result max+1 result=%v err=%v", result, err)
			}
			if result, err := family.run(context.Background(), ProjectionQuery{MaxItems: 1, MaxResults: 1_000_000}); family.name != "ticket" && (!errors.Is(err, ErrProjectionLimit) || !nilProjectionResult(result)) {
				t.Fatalf("small-work/huge-result result=%v err=%v", result, err)
			}
			ctx, cancel := context.WithCancel(context.Background())
			result, err := family.run(ctx, ProjectionQuery{
				MaxItems:   10_000,
				MaxResults: 100,
				Progress: func(value ProjectionProgress) {
					if family.name == "ticket" || strings.Contains(value.Phase, "sort") {
						cancel()
					}
				},
			})
			if !errors.Is(err, context.Canceled) || !nilProjectionResult(result) {
				t.Fatalf("during-work cancellation result=%v err=%v", result, err)
			}
			after, err := family.run(context.Background(), ProjectionQuery{MaxItems: 10_000, MaxResults: 100})
			afterJSON, _ := json.Marshal(after)
			if err != nil || !bytes.Equal(validJSON, afterJSON) {
				t.Fatalf("failed queries changed last-valid result: after=%v err=%v", after, err)
			}
		})
	}
}

func nilProjectionResult(value any) bool {
	if value == nil {
		return true
	}
	reflected := reflect.ValueOf(value)
	return (reflected.Kind() == reflect.Slice || reflected.Kind() == reflect.Pointer) && reflected.IsNil()
}

func TestG3PersistenceFailuresDoNotMutateCachesOrEscapeHandlers(t *testing.T) {
	failure := errors.New("injected persistence failure")
	repository := &auditRepository{err: failure}
	server := NewServer(Config{}, repository, NewEventBus(repository))
	server.cache.Files["a.go"] = Scope{ID: "old-file"}
	server.cache.Sections["a.go"] = []Scope{{ID: "old-section"}}
	server.cache.Definitions["a.go"] = []Scope{{ID: "old-definition"}}
	if err := server.updateIndexForFile(context.Background(), "a.go", "package changed"); !errors.Is(err, failure) {
		t.Fatalf("index persistence error=%v", err)
	}
	if server.cache.Files["a.go"].ID != "old-file" || server.cache.Sections["a.go"][0].ID != "old-section" || server.cache.Definitions["a.go"][0].ID != "old-definition" {
		t.Fatalf("failed append changed cache: %+v", server.cache)
	}
	push := map[string]interface{}{
		"sender":  map[string]interface{}{"login": "ueli"},
		"commits": []interface{}{map[string]interface{}{"modified": []interface{}{"a.go"}}},
	}
	if err := server.handleGitHubPushEvent(context.Background(), push); !errors.Is(err, failure) {
		t.Fatalf("checkpoint failure was not propagated: %v", err)
	}
	checkpoint := Event{Payload: `{"author":"ueli","files":["a.go"]}`}
	if err := server.onCheckpointEvent(context.Background(), checkpoint); !errors.Is(err, failure) {
		t.Fatalf("checkpoint handler failure was not propagated: %v", err)
	}
	event := Event{Payload: `{"id":"2026/08/25/test","author":"ueli"}`}
	if err := server.onCLIEvent(context.Background(), repopkg.EventTicketOpenEnded, event); !errors.Is(err, failure) {
		t.Fatalf("contributor handler failure was not propagated: %v", err)
	}
	bus := NewEventBus(repository)
	bus.Subscribe("fails", func(context.Context, Event) error { return failure })
	bus.Start()
	defer bus.Stop()
	if err := bus.Publish(context.Background(), "fails", "test", map[string]string{}); !errors.Is(err, failure) {
		t.Fatalf("event handler failure did not reach publisher: %v", err)
	}
}

func TestG3FailedAppendAtEveryPrecommitPhasePreservesProjection(t *testing.T) {
	repository, err := openDatabase(filepath.Join(t.TempDir(), "repository.events"))
	if err != nil {
		t.Fatal(err)
	}
	prior := Ticket{ID: "prior", Status: "open", Title: "Prior"}
	if err := repository.recordTicket(context.Background(), prior); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(repository.store.path)
	if err != nil {
		t.Fatal(err)
	}
	for _, phase := range []string{"next-synced", "stage-synced", "prior-backed-up", "log-replaced"} {
		t.Run(phase, func(t *testing.T) {
			injected := errors.New("injected append failure")
			repository.store.interrupt = func(current string) error {
				if current == phase {
					return injected
				}
				return nil
			}
			err := repository.recordTicket(context.Background(), Ticket{ID: "rejected-" + phase, Status: "open"})
			repository.store.interrupt = nil
			if !errors.Is(err, injected) {
				t.Fatalf("append error=%v", err)
			}
			tickets, queryErr := repository.projectTickets(context.Background(), "")
			if queryErr != nil || len(tickets) != 1 || tickets[0].ID != "prior" {
				t.Fatalf("failed append changed projection: tickets=%v err=%v", tickets, queryErr)
			}
			after, readErr := os.ReadFile(repository.store.path)
			if readErr != nil || !bytes.Equal(before, after) {
				t.Fatalf("failed append changed durable bytes: err=%v", readErr)
			}
		})
	}
}

// #endregion 🧬️Repository

// #region 🧯️HostileSource

func TestG3OwnedSourceHasNoExternalOrLegacyStoreFallback(t *testing.T) {
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatal(err)
	}
	prohibited := []string{"modernc.org", "database/sql", "sqlite", "sql.Open", "CREATE TABLE", "ON CONFLICT", "crdt", "legacy store", "store fallback", "_ = s.db.", "_ = s.bus.publish"}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".go") || strings.HasSuffix(entry.Name(), "_test.go") {
			continue
		}
		data, readErr := os.ReadFile(entry.Name())
		if readErr != nil {
			t.Fatal(readErr)
		}
		lower := strings.ToLower(string(data))
		for _, token := range prohibited {
			if strings.Contains(lower, strings.ToLower(token)) {
				t.Fatalf("production source %s retains prohibited token %q", entry.Name(), token)
			}
		}
	}
	module, err := os.ReadFile("go.mod")
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(module, []byte("modernc.org")) || bytes.Contains(module, []byte("golang.org/")) {
		t.Fatalf("external module retained:\n%s", module)
	}
	if _, err := os.Stat("go.sum"); !os.IsNotExist(err) {
		t.Fatalf("go.sum must be naturally absent: %v", err)
	}
	windowsDurability, err := os.ReadFile("🪟️durability_windows.go")
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Contains(windowsDurability, []byte("syscall.FlushFileBuffers")) || !bytes.Contains(windowsDurability, []byte("moveFileWriteThrough")) || bytes.Contains(windowsDurability, []byte("runtime.GOOS")) {
		t.Fatal("Windows durability must write through replacements, flush metadata, or return its explicit error")
	}
	eventStoreSource, err := os.ReadFile("🗄️event_store.go")
	if err != nil {
		t.Fatal(err)
	}
	for _, mutation := range []string{"os.OpenFile(", "os.Remove(", "os.Rename(", "os.Chtimes(", "os.MkdirAll("} {
		if bytes.Contains(eventStoreSource, []byte(mutation)) {
			t.Fatalf("event-store mutation escaped the owned operation boundary: %s", mutation)
		}
	}
	repositorySource, err := os.ReadFile("📚️repository.go")
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(repositorySource, []byte("sort.Strings")) || !bytes.Contains(repositorySource, []byte("projectionResultCapacity")) || !bytes.Contains(repositorySource, []byte("sortProjectionStrings")) {
		t.Fatal("projection ordering escaped the retained budgeted cursor")
	}
}

// #endregion 🧯️HostileSource

// #region 🔢️Assertions

func containsString(values []string, expected string) bool {
	for _, value := range values {
		if value == expected {
			return true
		}
	}
	return false
}

func equalStrings(actual []string, expected []string) bool {
	actualCopy := append([]string(nil), actual...)
	expectedCopy := append([]string(nil), expected...)
	sort.Strings(actualCopy)
	sort.Strings(expectedCopy)
	return strings.Join(actualCopy, "\x00") == strings.Join(expectedCopy, "\x00")
}

func equalStringsInOrder(actual []string, expected []string) bool {
	return strings.Join(actual, "\x00") == strings.Join(expected, "\x00")
}

func equalTickets(actual []Ticket, expected []Ticket) bool {
	actualJSON, _ := json.Marshal(actual)
	expectedJSON, _ := json.Marshal(expected)
	return bytes.Equal(actualJSON, expectedJSON)
}

// #endregion 🔢️Assertions
