// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 repository replay and projection contract tests.

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
	"strings"
	"testing"
	"time"

	repopkg "github.com/usalu/semio/repo/go"
)

// #region 🧠️G3RepositoryProjection

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

// #endregion 🧠️G3RepositoryProjection
