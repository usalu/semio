// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unit tests for the append-replayed index, its bounds and its ranking rules.

// #endregion 🧲️Header

package search

import (
	"context"
	"errors"
	"path/filepath"
	"strings"
	"testing"
)

// #region 🧰️Helpers

func conjunction(query string) Query {
	var parts []Query
	for _, term := range strings.Fields(query) {
		parts = append(parts, NewMatchQuery(term))
	}
	return NewConjunctionQuery(parts...)
}

func memoryIndex(t *testing.T, documents map[string]string) Index {
	t.Helper()
	index, err := NewMemOnly(NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	for id, text := range documents {
		if err := index.Index(id, map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	return index
}

// #endregion 🧰️Helpers

// #region 🔎️Query

func TestSearchRequiresEveryTerm(t *testing.T) {
	index := memoryIndex(t, map[string]string{
		"a": "deterministic event replay",
		"b": "recursive glob matcher",
		"c": "event storage",
	})
	result, err := index.Search(NewSearchRequest(conjunction("event replay")))
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != 1 || len(result.Hits) != 1 || result.Hits[0].ID != "a" {
		t.Fatalf("result = %+v", result)
	}
}

func TestSearchBreaksTiesOnTheIdentifier(t *testing.T) {
	index := memoryIndex(t, map[string]string{"b": "alpha", "a": "alpha", "c": "alpha"})
	result, err := index.Search(NewSearchRequest(conjunction("alpha")))
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	for _, hit := range result.Hits {
		ids = append(ids, hit.ID)
	}
	if strings.Join(ids, ",") != "a,b,c" {
		t.Fatalf("ids = %v", ids)
	}
}

func TestSearchTruncatesToSizeButReportsTheFullTotal(t *testing.T) {
	index := memoryIndex(t, map[string]string{"a": "alpha", "b": "alpha", "c": "alpha"})
	request := NewSearchRequest(conjunction("alpha"))
	request.Size = 2
	result, err := index.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != 3 || len(result.Hits) != 2 {
		t.Fatalf("result = %+v", result)
	}
}

func TestFuzzinessWidensTheMatchAndLowersTheScore(t *testing.T) {
	index := memoryIndex(t, map[string]string{"a": "replay"})
	exact, err := index.Search(NewSearchRequest(NewMatchQuery("replayy")))
	if err != nil {
		t.Fatal(err)
	}
	if exact.Total != 0 {
		t.Fatalf("an exact query must not match: %+v", exact)
	}
	fuzzy := NewMatchQuery("replayy")
	fuzzy.SetFuzziness(1)
	loose, err := index.Search(NewSearchRequest(fuzzy))
	if err != nil {
		t.Fatal(err)
	}
	if loose.Total != 1 || loose.Hits[0].Score != 1 {
		t.Fatalf("result = %+v", loose)
	}
}

func TestSearchRejectsAnEmptyRequest(t *testing.T) {
	index := memoryIndex(t, nil)
	if _, err := index.Search(nil); err == nil {
		t.Fatal("a nil request must be an error")
	}
	if _, err := index.Search(&SearchRequest{}); err == nil {
		t.Fatal("a request without a query must be an error")
	}
}

func TestSearchEnforcesTheQueryBounds(t *testing.T) {
	index := memoryIndex(t, map[string]string{"a": "alpha"})
	var many []Query
	for count := 0; count <= MaxQueryTerms; count++ {
		many = append(many, NewMatchQuery("x"))
	}
	if _, err := index.Search(NewSearchRequest(NewConjunctionQuery(many...))); !errors.Is(err, ErrTooLarge) {
		t.Fatalf("too many terms must exceed the limit, got %v", err)
	}
	if _, err := index.Search(NewSearchRequest(NewMatchQuery(strings.Repeat("x", MaxQueryBytes+1)))); !errors.Is(err, ErrTooLarge) {
		t.Fatalf("an oversized query must exceed the limit, got %v", err)
	}
}

func TestScoreSplitsOnNonAlphanumerics(t *testing.T) {
	if _, matched := score("event-replay", []*MatchQuery{NewMatchQuery("replay")}); !matched {
		t.Error("a hyphen must separate words")
	}
	if _, matched := score("EVENT", []*MatchQuery{NewMatchQuery("event")}); !matched {
		t.Error("matching is case-insensitive")
	}
}

func TestBoundedDistanceStopsAtTheBudget(t *testing.T) {
	if got := boundedDistance("kitten", "sitting", 1); got != 2 {
		t.Errorf("boundedDistance = %d, want the budget plus one", got)
	}
	if got := boundedDistance("kitten", "kitten", 0); got != 0 {
		t.Errorf("boundedDistance = %d, want 0", got)
	}
}

// #endregion 🔎️Query

// #region 📝️AppendReplay

func TestIndexEnforcesTheDocumentSize(t *testing.T) {
	index := memoryIndex(t, nil)
	err := index.Index("a", map[string]interface{}{"text": strings.Repeat("x", MaxDocumentBytes+1)})
	if !errors.Is(err, ErrTooLarge) {
		t.Fatalf("an oversized document must exceed the limit, got %v", err)
	}
}

func TestDeleteRemovesTheDocument(t *testing.T) {
	index := memoryIndex(t, map[string]string{"a": "alpha", "b": "beta"})
	if err := index.Delete("a"); err != nil {
		t.Fatal(err)
	}
	result, err := index.Search(NewSearchRequest(conjunction("alpha")))
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != 0 {
		t.Fatalf("result = %+v", result)
	}
}

func TestPersistedIndexReplaysThroughOpen(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "index")
	index, err := New(dir, NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	if err := index.Index("a", map[string]interface{}{"text": "deterministic event replay"}); err != nil {
		t.Fatal(err)
	}
	if err := index.Index("b", map[string]interface{}{"text": "event storage"}); err != nil {
		t.Fatal(err)
	}
	if err := index.Close(); err != nil {
		t.Fatal(err)
	}
	reopened, err := Open(dir)
	if err != nil {
		t.Fatal(err)
	}
	defer reopened.Close()
	result, err := reopened.Search(NewSearchRequest(conjunction("event")))
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != 2 {
		t.Fatalf("result = %+v", result)
	}
}

func TestCancellationStopsIndexing(t *testing.T) {
	index := memoryIndex(t, nil)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if err := index.IndexContext(ctx, "a", map[string]interface{}{"text": "alpha"}, nil); !errors.Is(err, context.Canceled) {
		t.Fatalf("a cancelled context must stop indexing, got %v", err)
	}
}

// #endregion 📝️AppendReplay
