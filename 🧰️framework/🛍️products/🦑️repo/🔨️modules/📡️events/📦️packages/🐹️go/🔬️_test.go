// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧪️ Fixture-driven contract of the repo events package.

// #endregion 🧲️Header

package events

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

// #region 🧫️Fixtures

func fixture(t *testing.T, name string, target interface{}) {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "🧫️fixtures", name))
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(data, target); err != nil {
		t.Fatal(err)
	}
}

// #endregion 🧫️Fixtures

// #region 📋️EventKindCatalog

func TestKindConstantsMatchSchemaCatalog(t *testing.T) {
	catalog, err := LoadKindCatalog()
	if err != nil {
		t.Fatal(err)
	}
	if catalog.SchemaVersion != 1 {
		t.Fatalf("schemaVersion = %d", catalog.SchemaVersion)
	}
	declared := AllEventKinds()
	if len(declared) != len(catalog.Kinds) {
		t.Fatalf("constants = %d, catalog = %d", len(declared), len(catalog.Kinds))
	}
	for index, entry := range catalog.Kinds {
		if declared[index] != entry.Kind {
			t.Fatalf("kind %d = %q, catalog = %q", index, declared[index], entry.Kind)
		}
	}
	seen := map[EventKind]struct{}{}
	for _, kind := range declared {
		if _, duplicate := seen[kind]; duplicate {
			t.Fatalf("duplicate kind %q", kind)
		}
		seen[kind] = struct{}{}
		if strings.Count(string(kind), ".") < 1 {
			t.Fatalf("kind %q is not dotted", kind)
		}
	}
}

// #endregion 📋️EventKindCatalog

// #region ✉️PayloadEncoding

func newPayload(name string) interface{} {
	switch name {
	case "TicketPayload":
		return &TicketPayload{}
	case "TicketOpenPayload":
		return &TicketOpenPayload{}
	case "TicketClosePayload":
		return &TicketClosePayload{}
	case "TicketReopenPayload":
		return &TicketReopenPayload{}
	case "TicketChangePayload":
		return &TicketChangePayload{}
	case "GoalPayload":
		return &GoalPayload{}
	case "GoalOpenPayload":
		return &GoalOpenPayload{}
	case "GoalClosePayload":
		return &GoalClosePayload{}
	case "GoalReopenPayload":
		return &GoalReopenPayload{}
	case "GoalChangePayload":
		return &GoalChangePayload{}
	case "ContributorPayload":
		return &ContributorPayload{}
	case "CheckpointPayload":
		return &CheckpointPayload{}
	case "TodoPayload":
		return &TodoPayload{}
	case "TodoCreatePayload":
		return &TodoCreatePayload{}
	case "TodoChangePayload":
		return &TodoChangePayload{}
	case "TodoDeletePayload":
		return &TodoDeletePayload{}
	case "WorkItem":
		return &WorkItem{}
	case "ContributorWork":
		return &ContributorWork{}
	case "DraftPayload":
		return &DraftPayload{}
	case "FilePayload":
		return &FilePayload{}
	case "FolderPayload":
		return &FolderPayload{}
	case "SectionPayload":
		return &SectionPayload{}
	case "IntegratePayload":
		return &IntegratePayload{}
	case "ExtractPayload":
		return &ExtractPayload{}
	}
	return nil
}

type payloadVectors struct {
	Schema string `json:"schema"`
	Cases  []struct {
		ID      string          `json:"id"`
		Type    string          `json:"type"`
		Input   json.RawMessage `json:"input"`
		Encoded string          `json:"encoded"`
	} `json:"cases"`
}

func TestPayloadGoldenEncoding(t *testing.T) {
	var vectors payloadVectors
	fixture(t, "✉️payload-vectors.json", &vectors)
	if len(vectors.Cases) == 0 {
		t.Fatal("no payload vectors")
	}
	for _, testCase := range vectors.Cases {
		t.Run(testCase.ID, func(t *testing.T) {
			value := newPayload(testCase.Type)
			if value == nil {
				t.Fatalf("unknown payload type %q", testCase.Type)
			}
			decoder := json.NewDecoder(bytes.NewReader(testCase.Input))
			decoder.DisallowUnknownFields()
			if err := decoder.Decode(value); err != nil {
				t.Fatal(err)
			}
			encoded, err := json.Marshal(value)
			if err != nil {
				t.Fatal(err)
			}
			if string(encoded) != testCase.Encoded {
				t.Fatalf("encoded = %s, want %s", encoded, testCase.Encoded)
			}
		})
	}
}

func TestEnvelopeEncoding(t *testing.T) {
	encoded, err := json.Marshal(Event{Kind: EventTicketOpenStarting, Source: "repo-cli", Payload: json.RawMessage(`{"id":"a"}`)})
	if err != nil {
		t.Fatal(err)
	}
	want := `{"kind":"ticket.open.starting","source":"repo-cli","payload":{"id":"a"}}`
	if string(encoded) != want {
		t.Fatalf("envelope = %s, want %s", encoded, want)
	}
}

func TestEmitURLAndNoOperation(t *testing.T) {
	for input, want := range map[string]string{
		"":                       "",
		"   ":                    "",
		"127.0.0.1:8787":         "http://127.0.0.1:8787/api/v1/events",
		"http://host:1/":         "http://host:1/api/v1/events",
		"https://host.example":   "https://host.example/api/v1/events",
		"http://host.example///": "http://host.example///api/v1/events",
	} {
		if got := EmitURL(input); got != want {
			t.Fatalf("EmitURL(%q) = %q, want %q", input, got, want)
		}
	}
	t.Setenv("COMPOSE_SERVER_ADDR", "")
	Emit(EventAnalyzeStarting, "repo-cli", FilePayload{Path: "a"})
}

// #endregion ✉️PayloadEncoding

// #region 🗄️StoreAppendSequence

type storeVectors struct {
	Inputs          []Input  `json:"inputs"`
	Sequences       []uint64 `json:"sequences"`
	InterruptPhases []string `json:"interruptPhases"`
}

func loadStoreVectors(t *testing.T) storeVectors {
	t.Helper()
	var vectors storeVectors
	fixture(t, "🗄️store-vectors.json", &vectors)
	return vectors
}

func TestStoreFixtureDeterministicReplay(t *testing.T) {
	vectors := loadStoreVectors(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := Store{Path: path}
	if _, err := store.Append(context.Background(), vectors.Inputs, nil); err != nil {
		t.Fatal(err)
	}
	first, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	replayed, err := store.Replay(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	var sequences []uint64
	for _, event := range replayed {
		sequences = append(sequences, event.Sequence)
	}
	if !reflect.DeepEqual(sequences, vectors.Sequences) {
		t.Fatalf("sequences = %v", sequences)
	}
	secondPath := filepath.Join(t.TempDir(), "events.jsonl")
	if _, err := (Store{Path: secondPath}).Append(context.Background(), vectors.Inputs, nil); err != nil {
		t.Fatal(err)
	}
	second, err := os.ReadFile(secondPath)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(first, second) {
		t.Fatal("deterministic append produced different bytes")
	}
}

func TestStoreDuplicateInterruptedAndCorruptEvent(t *testing.T) {
	vectors := loadStoreVectors(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := Store{Path: path}
	if _, err := store.Append(context.Background(), vectors.Inputs[:1], nil); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := store.Append(context.Background(), vectors.Inputs[:1], nil); !errors.Is(err, ErrDuplicate) {
		t.Fatalf("duplicate error = %v", err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err = store.Append(ctx, vectors.Inputs[1:], func(progress Progress) {
		if progress.Step == "encoded" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("interrupted append error = %v", err)
	}
	after, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, after) {
		t.Fatal("interrupted append changed committed log")
	}
	corrupt := append([]byte(nil), before...)
	corrupt[len(corrupt)/2] ^= 1
	if err := os.WriteFile(path, corrupt, 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := store.Replay(context.Background(), nil); !errors.Is(err, ErrCorrupt) {
		t.Fatalf("corrupt replay error = %v", err)
	}
}

func TestStoreCancellationAndMaximum(t *testing.T) {
	store := Store{Path: filepath.Join(t.TempDir(), "events.jsonl")}
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := store.Append(cancelled, []Input{{ID: "a", Kind: "recorded", Data: "a"}}, nil); !errors.Is(err, context.Canceled) {
		t.Fatalf("cancelled append = %v", err)
	}
	maximum := strings.Repeat("x", MaxEventSize-2)
	if _, err := store.Append(context.Background(), []Input{{ID: "max", Kind: "recorded", Data: maximum}}, nil); err != nil {
		t.Fatalf("maximum input: %v", err)
	}
	before, err := os.ReadFile(store.Path)
	if err != nil {
		t.Fatal(err)
	}
	plusOne := strings.Repeat("x", MaxEventSize-1)
	if _, err := store.Append(context.Background(), []Input{{ID: "plus-one", Kind: "recorded", Data: plusOne}}, nil); !errors.Is(err, ErrTooLarge) {
		t.Fatalf("maximum + 1 error = %v", err)
	}
	after, err := os.ReadFile(store.Path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, after) {
		t.Fatal("maximum + 1 append changed the last valid log")
	}
}

func TestStoreInterruptionsPreserveCommittedLog(t *testing.T) {
	vectors := loadStoreVectors(t)
	for _, phase := range vectors.InterruptPhases {
		t.Run(phase, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "events.jsonl")
			store := Store{Path: path}
			if _, err := store.Append(context.Background(), vectors.Inputs[:1], nil); err != nil {
				t.Fatal(err)
			}
			before, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			ctx, cancel := context.WithCancel(context.Background())
			_, err = store.Append(ctx, vectors.Inputs[1:], func(value Progress) {
				if value.Step == phase {
					cancel()
				}
			})
			if !errors.Is(err, context.Canceled) {
				t.Fatalf("%s interruption = %v", phase, err)
			}
			after, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(before, after) {
				t.Fatalf("%s interruption changed committed bytes", phase)
			}
			if _, err := os.Stat(path + ".stage"); !os.IsNotExist(err) {
				t.Fatalf("%s interruption left stage: %v", phase, err)
			}
			events, err := store.Replay(context.Background(), nil)
			if err != nil || len(events) != 1 || events[0].ID != vectors.Inputs[0].ID {
				t.Fatalf("%s replay = %v, %v", phase, events, err)
			}
		})
	}
}

func TestStoreReplayCancellationPreservesLog(t *testing.T) {
	vectors := loadStoreVectors(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := Store{Path: path}
	if _, err := store.Append(context.Background(), vectors.Inputs, nil); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err = store.Replay(ctx, func(value Progress) {
		if value.Current == 1 {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("replay interruption = %v", err)
	}
	after, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, after) {
		t.Fatal("interrupted replay changed the log")
	}
	events, err := store.Replay(context.Background(), nil)
	if err != nil || len(events) != len(vectors.Inputs) {
		t.Fatalf("replay after interruption = %v, %v", events, err)
	}
}

func TestStagedAppendRecovery(t *testing.T) {
	for _, test := range []struct {
		name        string
		written     func([]byte) []byte
		wantEvents  int
		wantChanged bool
	}{
		{name: "stage only", written: func([]byte) []byte { return nil }, wantEvents: 1},
		{name: "partial batch", written: func(batch []byte) []byte { return batch[:len(batch)/2] }, wantEvents: 1},
		{name: "complete batch", written: func(batch []byte) []byte { return batch }, wantEvents: 2, wantChanged: true},
	} {
		t.Run(test.name, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "events.jsonl")
			store := Store{Path: path}
			if _, err := store.Append(context.Background(), []Input{{ID: "first", Kind: "recorded", Data: "first"}}, nil); err != nil {
				t.Fatal(err)
			}
			before, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			batch := encodedEvent(t, StoreEvent{Schema: Schema, Sequence: 2, ID: "second", Kind: "recorded", Data: json.RawMessage(`"second"`)})
			staged := stage{
				Schema:        stageSchema,
				PriorExists:   true,
				PriorSize:     int64(len(before)),
				PriorChecksum: Digest(before),
				BatchSize:     len(batch),
				BatchChecksum: Digest(batch),
			}
			stageData, err := json.Marshal(staged)
			if err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(store.stagePath(), stageData, 0o600); err != nil {
				t.Fatal(err)
			}
			written := test.written(batch)
			if len(written) > 0 {
				file, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0o644)
				if err != nil {
					t.Fatal(err)
				}
				if _, err := file.Write(written); err != nil {
					file.Close()
					t.Fatal(err)
				}
				if err := file.Close(); err != nil {
					t.Fatal(err)
				}
			}
			events, err := store.Replay(context.Background(), nil)
			if err != nil {
				t.Fatal(err)
			}
			if len(events) != test.wantEvents {
				t.Fatalf("events = %d, want %d", len(events), test.wantEvents)
			}
			after, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			if bytes.Equal(before, after) != !test.wantChanged {
				t.Fatalf("changed = %t, want %t", !bytes.Equal(before, after), test.wantChanged)
			}
			if _, err := os.Stat(store.stagePath()); !os.IsNotExist(err) {
				t.Fatalf("stage remained: %v", err)
			}
		})
	}
}

func encodedEvent(t *testing.T, event StoreEvent) []byte {
	t.Helper()
	event.Checksum = Checksum(event)
	var output bytes.Buffer
	if err := json.NewEncoder(&output).Encode(event); err != nil {
		t.Fatal(err)
	}
	return output.Bytes()
}

// #endregion 🗄️StoreAppendSequence

// #region 📤️ExportContentHash

type exportVectors struct {
	Entities []struct {
		Kind  string            `json:"kind"`
		ID    string            `json:"id"`
		Value map[string]string `json:"value"`
	} `json:"entities"`
	SortedIds []string       `json:"sortedIds"`
	Snapshot  string         `json:"snapshot"`
	InputIds  []string       `json:"inputIds"`
	Counts    map[string]int `json:"counts"`
}

type fixtureExportSource struct{ entities []ExportEntity }

func (source fixtureExportSource) ExportEntities() []ExportEntity { return source.entities }

func TestExportSnapshotContentHash(t *testing.T) {
	var vectors exportVectors
	fixture(t, "📤️export-vectors.json", &vectors)
	entities := make([]ExportEntity, 0, len(vectors.Entities))
	for _, entity := range vectors.Entities {
		entities = append(entities, ExportEntity{Kind: entity.Kind, ID: entity.ID, Value: entity.Value})
	}
	source := fixtureExportSource{entities: entities}
	snapshot, err := BuildExportSnapshot(context.Background(), source.ExportEntities())
	if err != nil {
		t.Fatal(err)
	}
	if snapshot.Snapshot != vectors.Snapshot {
		t.Fatalf("snapshot = %s, want %s", snapshot.Snapshot, vectors.Snapshot)
	}
	if !reflect.DeepEqual(snapshot.Counts, vectors.Counts) {
		t.Fatalf("counts = %v, want %v", snapshot.Counts, vectors.Counts)
	}
	var ids []string
	for _, input := range snapshot.Inputs {
		ids = append(ids, input.ID)
	}
	if !reflect.DeepEqual(ids, vectors.InputIds) {
		t.Fatalf("input ids = %v, want %v", ids, vectors.InputIds)
	}
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	if _, err := (Store{Path: path}).Append(context.Background(), snapshot.Inputs, nil); err != nil {
		t.Fatal(err)
	}
	if _, err := (Store{Path: path}).Append(context.Background(), snapshot.Inputs, nil); !errors.Is(err, ErrDuplicate) {
		t.Fatalf("duplicate snapshot = %v", err)
	}
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := BuildExportSnapshot(cancelled, source.ExportEntities()); !errors.Is(err, context.Canceled) {
		t.Fatalf("cancelled snapshot = %v", err)
	}
}

func TestDigestAndChecksumAreStable(t *testing.T) {
	if Digest(nil) != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" {
		t.Fatalf("empty digest = %s", Digest(nil))
	}
	if Digest([]byte("semio")) != Digest([]byte("semio")) {
		t.Fatal("digest is not stable")
	}
	event := StoreEvent{Schema: Schema, Sequence: 1, ID: "a", Kind: "recorded", Data: json.RawMessage(`"a"`)}
	if Checksum(event) == Checksum(StoreEvent{Schema: Schema, Sequence: 2, ID: "a", Kind: "recorded", Data: event.Data}) {
		t.Fatal("checksum ignores the sequence")
	}
}

// #endregion 📤️ExportContentHash
