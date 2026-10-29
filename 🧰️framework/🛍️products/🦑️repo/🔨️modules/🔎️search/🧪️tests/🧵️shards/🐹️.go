package search

import (
	"bufio"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
	"time"
)

type shardFixture struct {
	Eligibility struct {
		Documents map[string]struct {
			Text string
			Kind string
		}
		Query string
		Kind  string
		Size  int
		Total uint64
		Hits  []string
	}

	Schema string `json:"schema"`
	Small  struct {
		Documents map[string]string `json:"documents"`
		Query     string            `json:"query"`
		Size      int               `json:"size"`
		Total     uint64            `json:"total"`
		Next      []string
		Hits      []string `json:"hits"`
	} `json:"small"`
	Large struct {
		Documents int      `json:"documents"`
		Query     string   `json:"query"`
		Size      int      `json:"size"`
		Total     uint64   `json:"total"`
		Hits      []string `json:"hits"`
		Last      string   `json:"last"`
	} `json:"large"`
}

func shardVectors(t *testing.T) shardFixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "🧪️tests", "🧫️fixtures", "🧵️shards", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var value shardFixture
	if err = json.Unmarshal(data, &value); err != nil {
		t.Fatal(err)
	}
	if value.Schema != "semio.search.shards/1" {
		t.Fatal(value.Schema)
	}
	return value
}

func TestShardedIndexPreservesOrderingTotalsAndPages(t *testing.T) {
	value := shardVectors(t)
	path := filepath.Join(physicalSearchTempDir(t), "index")
	index, err := NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	for id, text := range value.Small.Documents {
		if err = index.Index(id, map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	if err = index.Close(); err != nil {
		t.Fatal(err)
	}
	opened, err := OpenSharded(path)
	if err != nil {
		t.Fatal(err)
	}
	defer opened.Close()
	request := NewSearchRequest(NewMatchQuery(value.Small.Query))
	request.Size = value.Small.Size
	result, err := opened.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != value.Small.Total || len(result.Hits) != len(value.Small.Hits) {
		t.Fatalf("%+v", result)
	}
	for i, id := range value.Small.Hits {
		if result.Hits[i].ID != id || result.Hits[i].Score != 1 {
			t.Fatalf("%+v", result)
		}
	}
	if result.Next == nil {
		t.Fatal("next cursor required")
	}
	request.After = result.Next
	next, err := opened.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	if next.Total != 3 || len(next.Hits) != 1 || next.Hits[0].ID != value.Small.Next[0] || next.Next != nil {
		t.Fatalf("%+v", next)
	}
	saved := result.Next
	request.After = saved
	request.Query = NewMatchQuery("blue")
	if _, err = opened.Search(request); !errors.Is(err, ErrStaleCursor) {
		t.Fatalf("changed query cursor: %v", err)
	}
	changed := NewMatchQuery(value.Small.Query)
	changed.SetFuzziness(1)
	request.Query = changed
	if _, err = opened.Search(request); !errors.Is(err, ErrStaleCursor) {
		t.Fatalf("changed fuzziness cursor: %v", err)
	}
	request.Query = NewMatchQuery(value.Small.Query)
	request.After = &SearchCursor{Fingerprint: "different", ID: "a", Score: 1}
	if _, err = opened.Search(request); !errors.Is(err, ErrStaleCursor) {
		t.Fatalf("stale cursor: %v", err)
	}
}

func TestShardedIndexExceedsOneResidentShardWithoutLosingLateMatches(t *testing.T) {
	value := shardVectors(t)
	path := filepath.Join(physicalSearchTempDir(t), "index")
	index, err := NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	for i := 0; i < value.Large.Documents; i++ {
		text := "plain"
		if i == 0 || i == value.Large.Documents-1 {
			text = "needle"
		}
		if err = index.Index(fmt.Sprintf("d-%07d", i), map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	if err = index.Close(); err != nil {
		t.Fatal(err)
	}
	for attempt := 0; attempt < 2; attempt++ {
		opened, err := OpenSharded(path)
		if err != nil {
			t.Fatal(err)
		}
		if opened.manifest.Documents != value.Large.Documents {
			t.Fatalf("manifest lost documents: %d", opened.manifest.Documents)
		}
		request := NewSearchRequest(NewMatchQuery(value.Large.Query))
		request.Size = value.Large.Size
		result, err := opened.Search(request)
		if err != nil {
			t.Fatal(err)
		}
		if result.Total != value.Large.Total || len(result.Hits) != 1 || result.Hits[0].ID != value.Large.Hits[0] {
			t.Fatalf("%+v", result)
		}
		if result.Statistics.PeakResidentDocuments > MaxDocuments || result.Statistics.PeakResidentDocumentBytes > MaxIndexBytes || result.Statistics.PeakRetainedHits > request.Size || result.Statistics.PeakPendingEvents > MaxPendingEvents {
			t.Fatalf("unbounded residency: %+v", result.Statistics)
		}
		request.After = result.Next
		next, err := opened.Search(request)
		if err != nil || len(next.Hits) != 1 || next.Hits[0].ID != value.Large.Last {
			t.Fatalf("late shard match: %+v %v", next, err)
		}
		if err = opened.Close(); err != nil {
			t.Fatal(err)
		}
	}
}

func TestShardedIndexKeepsLastWriteAndDeletionSemantics(t *testing.T) {
	path := filepath.Join(physicalSearchTempDir(t), "index")
	index, err := NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	for _, text := range []string{"old", "needle"} {
		if err = index.Index("same", map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	if err = index.Index("deleted", map[string]interface{}{"text": "needle"}); err != nil {
		t.Fatal(err)
	}
	if err = index.Delete("deleted"); err != nil {
		t.Fatal(err)
	}
	if err = index.Close(); err != nil {
		t.Fatal(err)
	}
	opened, err := OpenSharded(path)
	if err != nil {
		t.Fatal(err)
	}
	defer opened.Close()
	result, err := opened.Search(NewSearchRequest(NewMatchQuery("needle")))
	if err != nil || result.Total != 1 || result.Hits[0].ID != "same" {
		t.Fatalf("%+v %v", result, err)
	}
}

func TestShardedIndexRejectsCorruptionAndCancelsPersistenceAndQuery(t *testing.T) {
	path := filepath.Join(physicalSearchTempDir(t), "index")
	index, err := NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	if err = index.Index("a", map[string]interface{}{"text": "needle"}); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	err = index.CloseContext(ctx, func(value Progress) {
		if value.Step == "persisted" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("%v", err)
	}
	index.Abort()
	if _, err = os.Stat(filepath.Join(path, "manifest.json")); !os.IsNotExist(err) {
		t.Fatalf("cancelled publication: %v", err)
	}
	path = filepath.Join(physicalSearchTempDir(t), "valid")
	index, err = NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	if err = index.Index("a", map[string]interface{}{"text": "needle"}); err != nil {
		t.Fatal(err)
	}
	if err = index.Close(); err != nil {
		t.Fatal(err)
	}
	opened, err := OpenSharded(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel = context.WithCancel(context.Background())
	_, err = opened.SearchContext(ctx, NewSearchRequest(NewMatchQuery("needle")), func(int, int) { cancel() })
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("query cancellation: %v", err)
	}
	opened.Close()
	data, err := os.ReadFile(filepath.Join(path, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest ShardManifest
	if err = json.Unmarshal(data, &manifest); err != nil {
		t.Fatal(err)
	}
	name := filepath.Join(path, manifest.Shards[0].Name, "events.jsonl")
	file, err := os.OpenFile(name, os.O_APPEND|os.O_WRONLY, 0)
	if err != nil {
		t.Fatal(err)
	}
	file.WriteString("{broken\n")
	file.Close()
	if _, err = OpenSharded(path); !errors.Is(err, ErrCorrupt) {
		t.Fatalf("corrupt shard: %v", err)
	}
}

func TestFiniteSearchRetainsOnlyRequestedTopK(t *testing.T) {
	index := memoryIndex(t, map[string]string{"z": "needle", "b": "needle", "a": "needle"})
	request := NewSearchRequest(NewMatchQuery("needle"))
	request.Size = 1
	result, err := index.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	if result.Total != 3 || len(result.Hits) != 1 || result.Hits[0].ID != "a" || result.Statistics.PeakRetainedHits > 1 {
		t.Fatalf("%+v", result)
	}
	request.Size = MaxQueryResults + 1
	if _, err = index.Search(request); !errors.Is(err, ErrTooLarge) {
		t.Fatalf("query hit bound: %v", err)
	}
	if err = index.Index("huge", map[string]interface{}{"text": strings.Repeat("x", MaxDocumentBytes+1)}); !errors.Is(err, ErrTooLarge) {
		t.Fatalf("per-document bound: %v", err)
	}
}

func TestShardedEligibilityPrecedesTopKAndBindsContinuation(t *testing.T) {
	value := shardVectors(t).Eligibility
	path := filepath.Join(physicalSearchTempDir(t), "index")
	writer, err := NewSharded(path, "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	for id, document := range value.Documents {
		if err = writer.Index(id, map[string]interface{}{"text": document.Text, "attributes": map[string]string{"kind": document.Kind}}); err != nil {
			t.Fatal(err)
		}
	}
	if err = writer.Close(); err != nil {
		t.Fatal(err)
	}
	reader, err := OpenSharded(path)
	if err != nil {
		t.Fatal(err)
	}
	defer reader.Close()
	request := NewSearchRequest(NewMatchQuery(value.Query))
	request.Size = value.Size
	request.FilterKey = value.Kind
	request.Accept = func(id string, attributes map[string]string) bool { return attributes["kind"] == value.Kind }
	result, err := reader.Search(request)
	if err != nil || result.Total != value.Total || len(result.Hits) != len(value.Hits) || result.Hits[0].ID != value.Hits[0] {
		t.Fatalf("eligibility: %+v %v", result, err)
	}
	request.After = result.Next
	request.FilterKey = "hidden"
	if _, err = reader.Search(request); !errors.Is(err, ErrStaleCursor) {
		t.Fatalf("changed eligibility: %v", err)
	}
}

func TestGenerationLeaseSharesReadersAndCancelsExclusiveWait(t *testing.T) {
	path := filepath.Join(physicalSearchTempDir(t), "lease")
	first, err := AcquireGenerationLease(context.Background(), path, true, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer first.Close()
	second, err := AcquireGenerationLease(context.Background(), path, true, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer second.Close()
	before, err := os.Lstat(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err = AcquireGenerationLease(ctx, path, false, func(value Progress) {
		if value.Step == "waiting-lock" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("exclusive wait: %v", err)
	}
	if err = first.Close(); err != nil {
		t.Fatal(err)
	}
	if err = second.Close(); err != nil {
		t.Fatal(err)
	}
	exclusive, err := AcquireGenerationLease(context.Background(), path, false, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer exclusive.Close()
	after, err := os.Lstat(path)
	if err != nil || !os.SameFile(before, after) {
		t.Fatalf("persistent lease identity: %v", err)
	}
}

func TestGenerationLeaseKernelReleasesKilledOwner(t *testing.T) {
	if path := os.Getenv("SEMIO_SEARCH_LEASE_CHILD"); path != "" {
		lease, err := AcquireGenerationLease(context.Background(), path, false, nil)
		if err != nil {
			t.Fatal(err)
		}
		defer lease.Close()
		fmt.Println("ready")
		for {
			time.Sleep(time.Hour)
		}
	}
	path := filepath.Join(physicalSearchTempDir(t), "lease")
	executable, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	command := exec.CommandContext(ctx, executable, "-test.run=^TestGenerationLeaseKernelReleasesKilledOwner$")
	command.Env = append(os.Environ(), "SEMIO_SEARCH_LEASE_CHILD="+path)
	output, err := command.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	if err = command.Start(); err != nil {
		t.Fatal(err)
	}
	defer func() { _ = command.Process.Kill(); _ = command.Wait() }()
	ready, err := bufio.NewReader(output).ReadString(10)
	if err != nil || ready != "ready\n" {
		t.Fatalf("child ownership: %q %v", ready, err)
	}
	if err = command.Process.Kill(); err != nil {
		t.Fatal(err)
	}
	_ = command.Wait()
	lease, err := AcquireGenerationLease(context.Background(), path, false, nil)
	if err != nil {
		t.Fatalf("kernel did not release dead owner: %v", err)
	}
	if err = lease.Close(); err != nil {
		t.Fatal(err)
	}
}

func TestBandedDistanceAgreesWithIndependentMatrix(t *testing.T) {
	words := []string{"", "a", "b", "ab", "ba", "aaa", "abab", "replay", "replya", "é", "🧬️"}
	oracle := func(a, b string) int {
		matrix := make([][]int, len(a)+1)
		for i := range matrix {
			matrix[i] = make([]int, len(b)+1)
			matrix[i][0] = i
		}
		for j := 0; j <= len(b); j++ {
			matrix[0][j] = j
		}
		for i := 1; i <= len(a); i++ {
			for j := 1; j <= len(b); j++ {
				cost := 0
				if a[i-1] != b[j-1] {
					cost = 1
				}
				matrix[i][j] = min(matrix[i-1][j]+1, matrix[i][j-1]+1, matrix[i-1][j-1]+cost)
			}
		}
		return matrix[len(a)][len(b)]
	}
	for _, a := range words {
		for _, b := range words {
			for budget := 0; budget <= 3; budget++ {
				actual := boundedDistance(a, b, budget)
				expected := min(oracle(a, b), budget+1)
				if min(actual, budget+1) != expected {
					t.Fatalf("distance(%q,%q,%d)=%d expected %d", a, b, budget, actual, expected)
				}
			}
		}
	}
}

func TestShardedCursorBindsRepositorySnapshot(t *testing.T) {
	var cursor *SearchCursor
	for _, snapshot := range []string{"source-a", "source-b"} {
		path := filepath.Join(physicalSearchTempDir(t), "index")
		writer, err := NewSharded(path, snapshot)
		if err != nil {
			t.Fatal(err)
		}
		for _, id := range []string{"a", "b"} {
			if err = writer.Index(id, map[string]interface{}{"text": "needle"}); err != nil {
				t.Fatal(err)
			}
		}
		if err = writer.Close(); err != nil {
			t.Fatal(err)
		}
		reader, err := OpenSharded(path)
		if err != nil {
			t.Fatal(err)
		}
		request := NewSearchRequest(NewMatchQuery("needle"))
		request.Size = 1
		request.After = cursor
		result, err := reader.Search(request)
		if cursor == nil {
			if err != nil || result.Next == nil {
				t.Fatalf("first generation: %+v %v", result, err)
			}
			cursor = result.Next
		} else if !errors.Is(err, ErrStaleCursor) {
			t.Fatalf("changed source snapshot: %v", err)
		}
		if err = reader.Close(); err != nil {
			t.Fatal(err)
		}
	}
}

func TestSearchStorageProtocolMatchesNeutralSchema(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "🧬️schema", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var schema struct {
		Definitions map[string]struct{ Const map[string]interface{} }
	}
	var raw map[string]json.RawMessage
	if err = json.Unmarshal(data, &raw); err != nil {
		t.Fatal(err)
	}
	if err = json.Unmarshal(raw["$defs"], &schema.Definitions); err != nil {
		t.Fatal(err)
	}
	protocol := schema.Definitions["StorageProtocol"].Const
	expected := map[string]int{"shards": ShardCount, "residentShards": 1, "maximumDocumentBytes": MaxDocumentBytes, "maximumShardDocuments": MaxDocuments, "maximumShardEvents": MaxPendingEvents, "maximumShardBytes": MaxIndexBytes, "maximumManifestBytes": MaxManifestBytes, "maximumQueryResults": MaxQueryResults, "defaultTreeHits": DefaultTreeHits, "maximumTreeResultNodes": MaxResultTreeNodes, "maximumTreeResultBytes": MaxResultTreeBytes, "maximumTraversalDepth": MaxTraversalDepth, "maximumFuzziness": MaxQueryFuzziness, "maximumIdentifierBytes": MaxIdentifierBytes}
	for key, value := range expected {
		if protocol[key] != float64(value) {
			t.Fatalf("protocol %s=%v implementation=%d", key, protocol[key], value)
		}
	}
}

func TestSearchRejectsOversizedIdentifiersBeforeRetention(t *testing.T) {
	id := strings.Repeat("x", MaxIdentifierBytes+1)
	memory, err := NewMemOnly(NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	defer memory.Close()
	writer, err := NewSharded(filepath.Join(physicalSearchTempDir(t), "index"), "fixture-generation")
	if err != nil {
		t.Fatal(err)
	}
	defer writer.Abort()
	for _, index := range []Index{memory, writer} {
		if err = index.Index(id, map[string]interface{}{"text": "needle"}); !errors.Is(err, ErrTooLarge) {
			t.Fatalf("unbounded hit identifier accepted: %v", err)
		}
	}
}

func TestMemoryIndexBoundsPendingBytesWithoutLosingAcceptedDocuments(t *testing.T) {
	index, err := NewMemOnly(NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	defer index.Close()
	text := "needle" + strings.Repeat("x", MaxDocumentBytes-len("needle"))
	accepted := 0
	for i := 0; i <= MaxIndexBytes/MaxDocumentBytes; i++ {
		err = index.Index(fmt.Sprint(i), map[string]interface{}{"text": text})
		if errors.Is(err, ErrTooLarge) {
			break
		}
		if err != nil {
			t.Fatal(err)
		}
		accepted++
	}
	if !errors.Is(err, ErrTooLarge) || accepted == 0 {
		t.Fatalf("pending byte budget: accepted=%d error=%v", accepted, err)
	}
	request := NewSearchRequest(NewMatchQuery("needle"))
	request.Size = 1
	result, err := index.Search(request)
	if err != nil || result.Total != uint64(accepted) {
		t.Fatalf("accepted documents lost: %+v %v", result, err)
	}
}

func TestSnapshotCacheNeutralLifecycleAndNodeGzipOracle(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "🧪️tests", "🧫️fixtures", "🌲️snapshot", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Value    map[string]any
		NodeGzip string
		Cases    []struct {
			Fingerprint string
			Builds      int
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	directory := filepath.Join(physicalSearchTempDir(t), "cache")
	built := 0
	for _, row := range fixture.Cases {
		value, err := LoadOrBuildSnapshot(context.Background(), SnapshotCacheRequest[map[string]any]{Directory: directory, Fingerprint: row.Fingerprint, Build: func(context.Context) (map[string]any, error) { built++; return fixture.Value, nil }})
		if err != nil || built != row.Builds || !reflect.DeepEqual(value, fixture.Value) {
			t.Fatalf("snapshot=%v builds=%d expected=%d error=%v", value, built, row.Builds, err)
		}
	}
	encoded, err := base64.StdEncoding.DecodeString(fixture.NodeGzip)
	if err != nil {
		t.Fatal(err)
	}
	digest := sha256.Sum256(encoded)
	generation := filepath.Join(physicalSearchTempDir(t), "oracle")
	if err := os.Mkdir(generation, 0755); err != nil {
		t.Fatal(err)
	}
	metadata, _ := json.Marshal(snapshotMeta{Schema: snapshotSchema, Fingerprint: "oracle", SHA256: hex.EncodeToString(digest[:]), Bytes: int64(len(encoded))})
	for name, bytes := range map[string][]byte{"meta.json": metadata, "payload.json.gz": encoded} {
		if err := os.WriteFile(filepath.Join(generation, name), bytes, 0600); err != nil {
			t.Fatal(err)
		}
	}
	decoded, _, err := readSnapshot[map[string]any](context.Background(), generation)
	if err != nil || !reflect.DeepEqual(decoded, fixture.Value) {
		t.Fatalf("Node gzip oracle: %v %v", decoded, err)
	}
}
func TestSnapshotCancellationAndFailedBuildPreservePublishedBytes(t *testing.T) {
	directory := filepath.Join(physicalSearchTempDir(t), "cache")
	request := SnapshotCacheRequest[string]{Directory: directory, Fingerprint: "old", Build: func(context.Context) (string, error) { return "old", nil }}
	if _, err := LoadOrBuildSnapshot(context.Background(), request); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(directory, "snapshot", "payload.json.gz")
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	request.Fingerprint = "new"
	request.Build = func(context.Context) (string, error) { cancel(); return "partial", nil }
	value, err := LoadOrBuildSnapshot(ctx, request)
	if !errors.Is(err, context.Canceled) || value != "" {
		t.Fatalf("cancelled partial projection escaped: %q %v", value, err)
	}
	after, err := os.ReadFile(path)
	if err != nil || !bytes.Equal(before, after) {
		t.Fatal("cancelled publication replaced prior bytes")
	}
	request.Build = func(context.Context) (string, error) { return "", errors.New("fixture failure") }
	if _, err := LoadOrBuildSnapshot(context.Background(), request); err == nil {
		t.Fatal("failed builder was hidden")
	}
	after, err = os.ReadFile(path)
	if err != nil || !bytes.Equal(before, after) {
		t.Fatal("failed publication replaced prior bytes")
	}
}
func TestSnapshotCorruptionFailsWithoutRebuilding(t *testing.T) {
	directory := filepath.Join(physicalSearchTempDir(t), "cache")
	built := 0
	request := SnapshotCacheRequest[string]{Directory: directory, Fingerprint: "same", Build: func(context.Context) (string, error) { built++; return "value", nil }}
	if _, err := LoadOrBuildSnapshot(context.Background(), request); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(directory, "snapshot", "payload.json.gz")
	if err := os.WriteFile(path, []byte("invalid gzip"), 0600); err != nil {
		t.Fatal(err)
	}
	if _, err := LoadOrBuildSnapshot(context.Background(), request); err == nil || built != 1 {
		t.Fatalf("corruption rebuilt or escaped: builds=%d error=%v", built, err)
	}
}

func TestSnapshotWaitCancellationUsesKernelLeaseWithoutStartingBuilder(t *testing.T) {
	directory := physicalSearchTempDir(t)
	held, err := AcquireGenerationLease(context.Background(), filepath.Join(directory, ".snapshot.lock"), false, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer held.Close()
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()
	built := false
	value, err := LoadOrBuildSnapshot(ctx, SnapshotCacheRequest[string]{Directory: directory, Fingerprint: "one", Build: func(context.Context) (string, error) { built = true; return "partial", nil }})
	if !errors.Is(err, context.DeadlineExceeded) || built || value != "" {
		t.Fatalf("cache waiter: value=%q built=%v error=%v", value, built, err)
	}
}
func TestSnapshotProtocolMatchesNeutralSchema(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "🧬️schema", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var schema struct {
		Definitions map[string]struct{ Const map[string]any }
	}
	var raw map[string]json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(raw["$defs"], &schema.Definitions); err != nil {
		t.Fatal(err)
	}
	protocol := schema.Definitions["SnapshotCacheProtocol"].Const
	if protocol["schema"] != snapshotSchema || protocol["maximumPayloadBytes"] != float64(MaxSnapshotBytes) || protocol["maximumCompressedBytes"] != float64(MaxSnapshotCompressedBytes) || protocol["maximumMetadataBytes"] != float64(MaxManifestBytes) {
		t.Fatalf("snapshot protocol drift: %v", protocol)
	}
}

func TestSourceFingerprintMatchesNeutralNodeSHA256Oracle(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "🧪️tests", "🧫️fixtures", "🌲️snapshot", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		SourceCases []struct {
			Name     string
			Files    map[string]string
			Excluded []string
			SHA256   string
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.SourceCases {
		t.Run(row.Name, func(t *testing.T) {
			root := physicalSearchTempDir(t)
			for path, body := range row.Files {
				if err := os.WriteFile(filepath.Join(root, path), []byte(body), 0600); err != nil {
					t.Fatal(err)
				}
			}
			actual, err := FingerprintSource(context.Background(), root, func(path string) bool {
				for _, excluded := range row.Excluded {
					if path == excluded {
						return true
					}
				}
				return false
			}, nil)
			if err != nil {
				t.Fatal(err)
			}
			if actual != row.SHA256 {
				t.Fatalf("digest=%s want=%s", actual, row.SHA256)
			}
		})
	}
}

func TestSnapshotRefusesSymlinkAncestorBeforeCreatingChildren(t *testing.T) {
	root, target := physicalSearchTempDir(t), physicalSearchTempDir(t)
	link := filepath.Join(root, "link")
	if err := os.Symlink(target, link); err != nil {
		t.Skipf("symlink capability unavailable: %v", err)
	}
	_, err := LoadOrBuildSnapshot(context.Background(), SnapshotCacheRequest[string]{Directory: filepath.Join(link, "child"), Fingerprint: "one", Build: func(context.Context) (string, error) { t.Fatal("builder reached through symlink"); return "", nil }})
	if err == nil {
		t.Fatal("symlink ancestor accepted")
	}
	if _, err := os.Stat(filepath.Join(target, "child")); !os.IsNotExist(err) {
		t.Fatalf("rejected request created child: %v", err)
	}
}

func TestSnapshotRejectsTrailingMetadataWithoutRebuilding(t *testing.T) {
	directory := physicalSearchTempDir(t)
	built := 0
	request := SnapshotCacheRequest[string]{Directory: directory, Fingerprint: "same", Build: func(context.Context) (string, error) { built++; return "value", nil }}
	if _, err := LoadOrBuildSnapshot(context.Background(), request); err != nil {
		t.Fatal(err)
	}
	metadata := filepath.Join(directory, "snapshot", "meta.json")
	file, err := os.OpenFile(metadata, os.O_APPEND|os.O_WRONLY, 0)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := file.WriteString("{}"); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := LoadOrBuildSnapshot(context.Background(), request); !errors.Is(err, ErrCorrupt) || built != 1 {
		t.Fatalf("trailing metadata: builds=%d error=%v", built, err)
	}
}

// 🏖️physicalSearchTempDir canonicalizes only a newly allocated, owned temporary root.
func physicalSearchTempDir(t *testing.T) string {
	t.Helper()
	root, err := filepath.EvalSymlinks(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}
	return root
}
