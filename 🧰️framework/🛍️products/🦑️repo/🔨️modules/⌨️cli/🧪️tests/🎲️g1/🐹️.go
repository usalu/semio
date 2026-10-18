// 🎲️ Tests of the cli domain: G1 contract fixtures.

package cli

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	os "os"
	filepath "path/filepath"
	reflect "reflect"
	strings "strings"
	testing "testing"
	template "text/template"

	eventspkg "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
	yaml "github.com/usalu/semio/repo/yaml"
)

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
	YAML struct {
		Source  string   `json:"source"`
		Name    string   `json:"name"`
		Paths   []string `json:"paths"`
		Enabled bool     `json:"enabled"`
	} `json:"yaml"`
}

func loadG1Fixture(t *testing.T) g1Fixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "🧫️fixtures", "1️⃣g1-contract", "🔣️.json"))
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

func TestG1InvalidFlagFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	root := &Command{Use: "repo"}
	root.AddCommand(&Command{Use: "inspect", Run: func(*Command, []string) {}})
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
	root := &Command{Use: "repo"}
	root.SetOut(&output)
	root.AddCommand(&Command{
		Use:   "inspect target",
		Short: fixture.Command.HelpContains,
		Args:  ExactArgs(1),
		Run: func(_ *Command, args []string) {
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
	matches, err := workspace.FilepathGlob(filepath.Join(root, filepath.FromSlash(fixture.Glob.Pattern)))
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
	_, err := template.New("invalid").Funcs(model.TxtFuncMap()).Parse(fixture.Template.Invalid)
	if (err != nil) != fixture.Template.Error {
		t.Fatalf("template error = %v", err)
	}
}

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

func TestG1ExportRetainsHistoryAndRejectsDuplicateSnapshot(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	firstContext := &testExportContext{technologies: []*model.Technology{{Name: "first", Root: "first", Kind: model.TechnologyKindUser}}}
	first, err := ExportToEventLog(path, firstContext)
	if err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := ExportToEventLog(path, firstContext); !errors.Is(err, eventspkg.ErrDuplicate) {
		t.Fatalf("duplicate snapshot = %v", err)
	}
	afterDuplicate, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, afterDuplicate) {
		t.Fatal("duplicate snapshot changed history")
	}
	secondContext := &testExportContext{technologies: []*model.Technology{{Name: "second", Root: "second", Kind: model.TechnologyKindUser}}}
	second, err := ExportToEventLog(path, secondContext)
	if err != nil {
		t.Fatal(err)
	}
	if first.Snapshot == second.Snapshot {
		t.Fatal("changed snapshots have the same identity")
	}
	events, err := (eventspkg.Store{Path: path}).Replay(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 2 || events[0].Sequence != 1 || events[1].Sequence != 2 {
		t.Fatalf("retained export history = %+v", events)
	}
}

func TestG1FailedAndInterruptedExportPreserveExistingLog(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	base := &testExportContext{technologies: []*model.Technology{{Name: "base", Root: "base", Kind: model.TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, base); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	oversized := &testExportContext{technologies: []*model.Technology{{Name: "large", Root: strings.Repeat("x", eventspkg.MaxEventSize+1), Kind: model.TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, oversized); !errors.Is(err, eventspkg.ErrTooLarge) {
		t.Fatalf("failed export = %v", err)
	}
	assertFileBytes(t, path, before)
	ctx, cancel := context.WithCancel(context.Background())
	interrupted := &testExportContext{technologies: []*model.Technology{{Name: "interrupted", Root: "interrupted", Kind: model.TechnologyKindUser}}}
	_, err = ExportToEventLogContext(ctx, path, interrupted, func(value eventspkg.Progress) {
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
