// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package client

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
	"text/template"

	command "github.com/usalu/semio/repo/client/internal/command"
	eventstore "github.com/usalu/semio/repo/client/internal/eventstore"
	glob "github.com/usalu/semio/repo/client/internal/glob"
	search "github.com/usalu/semio/repo/client/internal/search"
	templatefunc "github.com/usalu/semio/repo/client/internal/templatefunc"
	yaml "github.com/usalu/semio/repo/client/internal/yaml"
)

// #region 📜️Fixture

type g1Fixture struct {
	Schema  string `json:"schema"`
	Command struct {
		InvalidArgs   []string `json:"invalidArgs"`
		ErrorContains string   `json:"errorContains"`
		HelpArgs      []string `json:"helpArgs"`
		HelpContains  string   `json:"helpContains"`
		DispatchArgs  []string `json:"dispatchArgs"`
	} `json:"command"`
	Glob struct {
		Pattern string   `json:"pattern"`
		Paths   []string `json:"paths"`
		Matches []string `json:"matches"`
	} `json:"glob"`
	Template struct {
		Invalid string `json:"invalid"`
		Error   bool   `json:"error"`
	} `json:"template"`
	Search struct {
		Query                  string            `json:"query"`
		Documents              map[string]string `json:"documents"`
		Matches                []string          `json:"matches"`
		ReindexInterruptPhases []string          `json:"reindexInterruptPhases"`
		MaxDocumentBytes       int               `json:"maxDocumentBytes"`
	} `json:"search"`
	EventStore struct {
		Inputs          []eventstore.Input `json:"inputs"`
		Sequences       []uint64           `json:"sequences"`
		InterruptPhases []string           `json:"interruptPhases"`
	} `json:"eventStore"`
	YAML struct {
		Source  string   `json:"source"`
		Name    string   `json:"name"`
		Paths   []string `json:"paths"`
		Enabled bool     `json:"enabled"`
	} `json:"yaml"`
}

func loadG1Fixture(t *testing.T) g1Fixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("🧫️fixtures", "g1-contract.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture g1Fixture
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.cli.g1/1" {
		t.Fatalf("unexpected fixture schema %q", fixture.Schema)
	}
	return fixture
}

// #endregion 📜️Fixture

// #region ⌨️CommandTemplateGlob

func TestG1InvalidFlagFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	root := &command.Command{Use: "repo"}
	root.AddCommand(&command.Command{Use: "inspect", Run: func(*command.Command, []string) {}})
	root.SetArgs(fixture.Command.InvalidArgs)
	err := root.Execute()
	if err == nil || !strings.Contains(err.Error(), fixture.Command.ErrorContains) {
		t.Fatalf("invalid flag error = %v", err)
	}
}

func TestG1CommandHelpAndDispatchFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	var output bytes.Buffer
	var dispatched []string
	root := &command.Command{Use: "repo"}
	root.SetOut(&output)
	root.AddCommand(&command.Command{
		Use:   "inspect target",
		Short: fixture.Command.HelpContains,
		Args:  command.ExactArgs(1),
		Run: func(_ *command.Command, args []string) {
			dispatched = append([]string(nil), args...)
		},
	})
	root.SetArgs(fixture.Command.HelpArgs)
	if err := root.Execute(); err != nil {
		t.Fatal(err)
	}
	if strings.Count(output.String(), fixture.Command.HelpContains) != 1 || dispatched != nil {
		t.Fatalf("help output = %q, dispatched = %v", output.String(), dispatched)
	}
	root.SetArgs(fixture.Command.DispatchArgs)
	if err := root.Execute(); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(dispatched, fixture.Command.DispatchArgs[1:]) {
		t.Fatalf("dispatch args = %v", dispatched)
	}
}

func TestG1RecursiveGlobFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	root := t.TempDir()
	for _, relative := range fixture.Glob.Paths {
		path := filepath.Join(root, filepath.FromSlash(relative))
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte(relative), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	matches, err := glob.FilepathGlob(filepath.Join(root, filepath.FromSlash(fixture.Glob.Pattern)))
	if err != nil {
		t.Fatal(err)
	}
	for index := range matches {
		relative, err := filepath.Rel(root, matches[index])
		if err != nil {
			t.Fatal(err)
		}
		matches[index] = filepath.ToSlash(relative)
	}
	if !reflect.DeepEqual(matches, fixture.Glob.Matches) {
		t.Fatalf("glob matches = %v, want %v", matches, fixture.Glob.Matches)
	}
}

func TestG1BadTemplateFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	_, err := template.New("invalid").Funcs(templatefunc.TxtFuncMap()).Parse(fixture.Template.Invalid)
	if (err != nil) != fixture.Template.Error {
		t.Fatalf("template error = %v", err)
	}
}

// #endregion ⌨️CommandTemplateGlob

// #region 🔎️Search

func TestG1SearchFixtureAndCancellation(t *testing.T) {
	fixture := loadG1Fixture(t)
	indexValue, err := search.NewMemOnly(search.NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	for id, text := range fixture.Search.Documents {
		if err := indexValue.Index(id, map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	queries := make([]search.Query, 0)
	for _, term := range strings.Fields(fixture.Search.Query) {
		query := search.NewMatchQuery(term)
		query.SetFuzziness(0)
		queries = append(queries, query)
	}
	request := search.NewSearchRequest(search.NewConjunctionQuery(queries...))
	request.Size = 10
	result, err := indexValue.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	for _, hit := range result.Hits {
		ids = append(ids, hit.ID)
	}
	if !reflect.DeepEqual(ids, fixture.Search.Matches) {
		t.Fatalf("search ids = %v, want %v", ids, fixture.Search.Matches)
	}
	cancellable := indexValue.(interface {
		SearchContext(context.Context, *search.SearchRequest, func(int, int)) (*search.SearchResult, error)
	})
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := cancellable.SearchContext(cancelled, request, nil); !errors.Is(err, context.Canceled) {
		t.Fatalf("search cancellation = %v", err)
	}
}

func TestG1CorruptIndex(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "events.jsonl"), []byte("{broken\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := search.Open(root); err == nil || !strings.Contains(err.Error(), "corrupt index") {
		t.Fatalf("corrupt index error = %v", err)
	}
}

func TestG1ReindexCancellationPreservesLastValidIndex(t *testing.T) {
	fixture := loadG1Fixture(t)
	for _, phase := range fixture.Search.ReindexInterruptPhases {
		t.Run(phase, func(t *testing.T) {
			_, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
			invalidateG1SearchFingerprint(t, GetRootDir(), phase)
			ctx, cancel := context.WithCancel(context.Background())
			tree := g1SearchTree("replacement", "second")
			_, err := ensureCacheIndexed(ctx, tree, func(value search.Progress) {
				if value.Step == phase {
					cancel()
				}
			})
			if !errors.Is(err, context.Canceled) {
				t.Fatalf("%s cancellation = %v", phase, err)
			}
			assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
			assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
			if _, err := os.Stat(indexPath + ".next"); !os.IsNotExist(err) {
				t.Fatalf("%s cancellation left staged index: %v", phase, err)
			}
		})
	}
}

func TestG1ReindexMaximumPlusOnePreservesLastValidIndex(t *testing.T) {
	fixture := loadG1Fixture(t)
	if fixture.Search.MaxDocumentBytes != search.MaxDocumentBytes {
		t.Fatalf("fixture maximum = %d, implementation = %d", fixture.Search.MaxDocumentBytes, search.MaxDocumentBytes)
	}
	_, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, GetRootDir(), "maximum")
	oversized := g1SearchTree(strings.Repeat("x", fixture.Search.MaxDocumentBytes+1))
	if _, err := ensureCacheIndexed(context.Background(), oversized, nil); !errors.Is(err, search.ErrTooLarge) {
		t.Fatalf("maximum + 1 reindex = %v", err)
	}
	assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
	assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
	if _, err := os.Stat(indexPath + ".next"); !os.IsNotExist(err) {
		t.Fatalf("maximum + 1 left staged index: %v", err)
	}
}

func TestG1CorruptCurrentIndexErrorPropagates(t *testing.T) {
	tree, indexPath, _, _ := prepareG1SearchCache(t)
	eventPath := filepath.Join(indexPath, "events.jsonl")
	file, err := os.OpenFile(eventPath, os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := file.WriteString("{broken\n"); err != nil {
		file.Close()
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := ensureCacheIndexed(context.Background(), tree, nil); !errors.Is(err, search.ErrCorrupt) {
		t.Fatalf("corrupt current index = %v", err)
	}
}

func TestG1ReindexLockWaitCancellation(t *testing.T) {
	tree, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, GetRootDir(), "lock")
	lockPath := filepath.Join(filepath.Dir(indexPath), ".lock")
	if err := os.WriteFile(lockPath, []byte("held"), 0o644); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err := ensureCacheIndexed(ctx, tree, func(value search.Progress) {
		if value.Step == "waiting-lock" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("lock wait cancellation = %v", err)
	}
	assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
	assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
}

func prepareG1SearchCache(t *testing.T) (*TreeNode, string, []byte, []byte) {
	t.Helper()
	repoRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(RepoMetaDirForRoot(repoRoot), 0o755); err != nil {
		t.Fatal(err)
	}
	previousRoot := GetRootDir()
	SetRootDir(repoRoot)
	t.Cleanup(func() { SetRootDir(previousRoot) })
	tree := g1SearchTree("last-valid", "stable")
	steps := map[string]bool{}
	indexValue, err := ensureCacheIndexed(context.Background(), tree, func(value search.Progress) { steps[value.Step] = true })
	if err != nil {
		t.Fatal(err)
	}
	if err := indexValue.Close(); err != nil {
		t.Fatal(err)
	}
	for _, step := range []string{"collecting", "indexed", "committed"} {
		if !steps[step] {
			t.Fatalf("missing reindex progress %q: %v", step, steps)
		}
	}
	indexPath := filepath.Join(getCacheDir(), "index.search")
	events, err := os.ReadFile(filepath.Join(indexPath, "events.jsonl"))
	if err != nil {
		t.Fatal(err)
	}
	meta, err := os.ReadFile(filepath.Join(indexPath, "meta.json"))
	if err != nil {
		t.Fatal(err)
	}
	return tree, indexPath, events, meta
}

func invalidateG1SearchFingerprint(t *testing.T, repoRoot, value string) {
	t.Helper()
	directory := filepath.Join(RepoMetaDirForRoot(repoRoot), "✍️notes")
	if err := os.MkdirAll(directory, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(directory, value), []byte(value), 0o644); err != nil {
		t.Fatal(err)
	}
}

func g1SearchTree(descriptions ...string) *TreeNode {
	root := &TreeNode{Kind: TreeNodeCategory, Label: "."}
	for index, description := range descriptions {
		root.Children = append(root.Children, &TreeNode{
			Kind:        TreeNodeBundle,
			ID:          fmt.Sprintf("bundle:%d", index),
			Label:       fmt.Sprintf("bundle-%d", index),
			Description: description,
		})
	}
	return root
}

// #endregion 🔎️Search

// #region 📨️EventStore

func TestG1EventStoreFixtureDeterministicReplay(t *testing.T) {
	fixture := loadG1Fixture(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := eventstore.Store{Path: path}
	if _, err := store.Append(context.Background(), fixture.EventStore.Inputs, nil); err != nil {
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
	if !reflect.DeepEqual(sequences, fixture.EventStore.Sequences) {
		t.Fatalf("sequences = %v", sequences)
	}
	secondPath := filepath.Join(t.TempDir(), "events.jsonl")
	if _, err := (eventstore.Store{Path: secondPath}).Append(context.Background(), fixture.EventStore.Inputs, nil); err != nil {
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

func TestG1DuplicateInterruptedAndCorruptEvent(t *testing.T) {
	fixture := loadG1Fixture(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := eventstore.Store{Path: path}
	if _, err := store.Append(context.Background(), fixture.EventStore.Inputs[:1], nil); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := store.Append(context.Background(), fixture.EventStore.Inputs[:1], nil); !errors.Is(err, eventstore.ErrDuplicate) {
		t.Fatalf("duplicate error = %v", err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err = store.Append(ctx, fixture.EventStore.Inputs[1:], func(progress eventstore.Progress) {
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
	if _, err := store.Replay(context.Background(), nil); !errors.Is(err, eventstore.ErrCorrupt) {
		t.Fatalf("corrupt replay error = %v", err)
	}
}

func TestG1EventStoreCancellationAndMaximum(t *testing.T) {
	store := eventstore.Store{Path: filepath.Join(t.TempDir(), "events.jsonl")}
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := store.Append(cancelled, []eventstore.Input{{ID: "a", Kind: "recorded", Data: "a"}}, nil); !errors.Is(err, context.Canceled) {
		t.Fatalf("cancelled append = %v", err)
	}
	maximum := strings.Repeat("x", eventstore.MaxEventSize-2)
	if _, err := store.Append(context.Background(), []eventstore.Input{{ID: "max", Kind: "recorded", Data: maximum}}, nil); err != nil {
		t.Fatalf("maximum input: %v", err)
	}
	before, err := os.ReadFile(store.Path)
	if err != nil {
		t.Fatal(err)
	}
	plusOne := strings.Repeat("x", eventstore.MaxEventSize-1)
	if _, err := store.Append(context.Background(), []eventstore.Input{{ID: "plus-one", Kind: "recorded", Data: plusOne}}, nil); !errors.Is(err, eventstore.ErrTooLarge) {
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

func TestG1EventStoreInterruptionsPreserveCommittedLog(t *testing.T) {
	fixture := loadG1Fixture(t)
	for _, phase := range fixture.EventStore.InterruptPhases {
		t.Run(phase, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "events.jsonl")
			store := eventstore.Store{Path: path}
			if _, err := store.Append(context.Background(), fixture.EventStore.Inputs[:1], nil); err != nil {
				t.Fatal(err)
			}
			before, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			ctx, cancel := context.WithCancel(context.Background())
			_, err = store.Append(ctx, fixture.EventStore.Inputs[1:], func(value eventstore.Progress) {
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
			if err != nil || len(events) != 1 || events[0].ID != fixture.EventStore.Inputs[0].ID {
				t.Fatalf("%s replay = %v, %v", phase, events, err)
			}
		})
	}
}

func TestG1EventStoreReplayCancellationPreservesLog(t *testing.T) {
	fixture := loadG1Fixture(t)
	path := filepath.Join(t.TempDir(), "events.jsonl")
	store := eventstore.Store{Path: path}
	if _, err := store.Append(context.Background(), fixture.EventStore.Inputs, nil); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err = store.Replay(ctx, func(value eventstore.Progress) {
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
	if err != nil || len(events) != len(fixture.EventStore.Inputs) {
		t.Fatalf("replay after interruption = %v, %v", events, err)
	}
}

func TestG1ExportRetainsHistoryAndRejectsDuplicateSnapshot(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	firstContext := &testExportContext{technologies: []*Technology{{Name: "first", Root: "first", Kind: TechnologyKindUser}}}
	first, err := ExportToEventLog(path, firstContext)
	if err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := ExportToEventLog(path, firstContext); !errors.Is(err, eventstore.ErrDuplicate) {
		t.Fatalf("duplicate snapshot = %v", err)
	}
	afterDuplicate, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, afterDuplicate) {
		t.Fatal("duplicate snapshot changed history")
	}
	secondContext := &testExportContext{technologies: []*Technology{{Name: "second", Root: "second", Kind: TechnologyKindUser}}}
	second, err := ExportToEventLog(path, secondContext)
	if err != nil {
		t.Fatal(err)
	}
	if first.Snapshot == second.Snapshot {
		t.Fatal("changed snapshots have the same identity")
	}
	events, err := (eventstore.Store{Path: path}).Replay(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 2 || events[0].Sequence != 1 || events[1].Sequence != 2 {
		t.Fatalf("retained export history = %+v", events)
	}
}

func TestG1FailedAndInterruptedExportPreserveExistingLog(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	base := &testExportContext{technologies: []*Technology{{Name: "base", Root: "base", Kind: TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, base); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	oversized := &testExportContext{technologies: []*Technology{{Name: "large", Root: strings.Repeat("x", eventstore.MaxEventSize+1), Kind: TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, oversized); !errors.Is(err, eventstore.ErrTooLarge) {
		t.Fatalf("failed export = %v", err)
	}
	assertFileBytes(t, path, before)
	ctx, cancel := context.WithCancel(context.Background())
	interrupted := &testExportContext{technologies: []*Technology{{Name: "interrupted", Root: "interrupted", Kind: TechnologyKindUser}}}
	_, err = ExportToEventLogContext(ctx, path, interrupted, func(value eventstore.Progress) {
		if value.Step == "appended" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("interrupted export = %v", err)
	}
	assertFileBytes(t, path, before)
}

func assertFileBytes(t *testing.T, path string, expected []byte) {
	t.Helper()
	actual, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(actual, expected) {
		t.Fatalf("%s changed", path)
	}
}

// #endregion 📨️EventStore

// #region ⚙️Config

func TestG1YAMLFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	var config struct {
		Name    string   `yaml:"name"`
		Paths   []string `yaml:"paths"`
		Enabled bool     `yaml:"enabled"`
	}
	if err := yaml.Unmarshal([]byte(fixture.YAML.Source), &config); err != nil {
		t.Fatal(err)
	}
	if config.Name != fixture.YAML.Name || !reflect.DeepEqual(config.Paths, fixture.YAML.Paths) || config.Enabled != fixture.YAML.Enabled {
		t.Fatalf("yaml config = %+v", config)
	}
}

// #endregion ⚙️Config
