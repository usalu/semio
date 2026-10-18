// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 language-neutral event-store contract tests: shared fixtures, the golden envelope, concurrent writers and assertions.

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
	"runtime"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
)

// #region 🗄️G3EventStore

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

func TestG3LanguageNeutralGoldenEnvelope(t *testing.T) {
	contract := loadG3EventLogContract(t)
	fields := []string{"stream", "sequence", "id", "generation", "type", "payload", "checksum"}
	if contract.Schema != "semio.coordinator.event/1" || contract.Encoding != "canonical-jsonl-lf" || contract.Scalar != "nonempty-no-nul" || !equalStringsInOrder(contract.Fields, fields) || contract.Checksum == "" {
		t.Fatalf("unexpected language-neutral contract %+v", contract)
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

// #endregion 🗄️G3EventStore
