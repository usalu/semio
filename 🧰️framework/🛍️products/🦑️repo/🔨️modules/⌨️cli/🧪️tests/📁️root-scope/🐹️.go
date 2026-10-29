package cli

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"
	"time"
	"unicode/utf16"

	events "github.com/usalu/semio/repo/events"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// 🧭️checkoutRoot is the checkout that owns the fixtures of this module, located from the directory the test binary started in.
func checkoutRoot() string { return sandbox.OriginPath("..", "..", "..", "..", "..", "..", "..") }

func TestMcpServersRetainIndependentRepositoryRoots(t *testing.T) {
	t.Setenv("COMPOSE_SERVER_ADDR", "")
	t.Setenv("COMPOSE_SERVER_TOKEN", "")
	cwd, err := os.Stat(".")
	if err != nil {
		t.Fatal(err)
	}
	originalRoot, originalBound := workspace.LookupRootDir()
	corpusPath := filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🧪️tests/🧫️fixtures/🔣️.json")
	raw, err := os.ReadFile(corpusPath)
	if err != nil {
		t.Fatal(err)
	}
	var corpus struct {
		Profiles []string
		MimeType string
		Cases    []struct {
			Name  string
			Files []struct {
				Path string
				Text string
			}
			Folders []string
		}
		Reads []struct {
			URI   string
			Field string
		}
	}
	if err := json.Unmarshal(raw, &corpus); err != nil {
		t.Fatal(err)
	}
	parent := os.Getenv("SEMIO_TEST_ARTIFACT_DIR")
	if !filepath.IsAbs(parent) || len(utf16.Encode([]rune(parent))) > 256 {
		t.Fatal("bounded owned artifacts required")
	}
	if err := os.MkdirAll(parent, 0755); err != nil {
		t.Fatal(err)
	}
	owned := ""
	for serial := 0; serial < 128; serial++ {
		candidate := filepath.Join(parent, fmt.Sprintf("mc-%d-%d", os.Getpid(), serial))
		if len(utf16.Encode([]rune(candidate))) > 256 {
			t.Fatal("bounded fixture required")
		}
		if err := os.Mkdir(candidate, 0755); err == nil {
			owned = candidate
			break
		} else if !os.IsExist(err) {
			t.Fatal(err)
		}
	}
	if owned == "" {
		t.Fatal("finite fixture allocation exhausted")
	}
	defer os.RemoveAll(owned)
	var roots []string
	for _, row := range corpus.Cases {
		directory := filepath.Join(owned, row.Name)
		if len(utf16.Encode([]rune(directory))) > 256 {
			t.Fatal("bounded fixture root required")
		}
		if err := os.Mkdir(directory, 0755); err != nil {
			t.Fatal(err)
		}
		for _, file := range row.Files {
			destination := filepath.Join(directory, filepath.FromSlash(file.Path))
			if len(utf16.Encode([]rune(destination))) > 256 {
				t.Fatal("bounded fixture file required")
			}
			if err := os.MkdirAll(filepath.Dir(destination), 0755); err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(destination, []byte(file.Text), 0644); err != nil {
				t.Fatal(err)
			}
		}
		promptPath := workspace.RepoMetaPathForRoot(directory, filepath.Join("💬️prompts", "enhance.tpl"))
		if len(utf16.Encode([]rune(promptPath))) > 256 {
			t.Fatal("bounded private prompt required")
		}
		if err := os.MkdirAll(filepath.Dir(promptPath), 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(promptPath, []byte(row.Name+"|{{.prompt}}"), 0644); err != nil {
			t.Fatal(err)
		}
		roots = append(roots, directory)
	}
	for _, profile := range corpus.Profiles {
		var servers []*MCPServer
		for _, directory := range roots {
			server, err := CreateMcpServer(directory, providers.McpClientKind(profile), DefaultCommandTimeout)
			if err != nil {
				t.Fatal(err)
			}
			servers = append(servers, server)
		}

		if profile == corpus.Profiles[0] {
			fixtureRaw, err := os.ReadFile(filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧪️tests/🌿️tickets/📁️scope/🔣️.json"))
			if err != nil {
				t.Fatal(err)
			}
			var lifecycle struct {
				Roots    []struct{ Draft string }
				Open     map[string]interface{}
				Change   map[string]interface{}
				Close    map[string]interface{}
				Reopen   map[string]interface{}
				Expected struct {
					Slug, OriginalSlug string
					DraftFiles         []string
				}
			}
			if err := json.Unmarshal(fixtureRaw, &lifecycle); err != nil {
				t.Fatal(err)
			}
			type outcome struct {
				Path   string
				Ticket model.Ticket
			}
			call := func(index int, name string, args map[string]interface{}) outcome {
				t.Helper()
				wire, err := json.Marshal(map[string]interface{}{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": map[string]interface{}{"name": name, "arguments": args}})
				if err != nil {
					t.Fatal(err)
				}
				var out strings.Builder
				if err := Serve(context.Background(), servers[index], io.NopCloser(strings.NewReader(string(wire)+"\n")), &out); err != nil {
					t.Fatal(err)
				}
				var reply struct {
					Error  json.RawMessage
					Result struct {
						IsError bool
						Content []struct{ Text string }
					}
				}
				if err := json.Unmarshal([]byte(out.String()), &reply); err != nil {
					t.Fatal(err)
				}
				if len(reply.Error) > 0 && string(reply.Error) != "null" || reply.Result.IsError || len(reply.Result.Content) != 1 {
					t.Fatalf("settled ticket protocol: %s", out.String())
				}
				var value outcome
				if err := json.Unmarshal([]byte(reply.Result.Content[0].Text), &value); err != nil {
					t.Fatal(err)
				}
				if _, _, _, _, err := parseTicketPath(value.Path); err != nil {
					t.Fatal(err)
				}
				return value
			}
			optionRaw, err := os.ReadFile(filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🎫️tickets/🧪️tests/🧫️fixtures/🔣️.json"))
			if err != nil {
				t.Fatal(err)
			}
			var options struct {
				ArgumentCases []struct {
					Arguments map[string]interface{}
					Accepted  bool
				}
			}
			if err := json.Unmarshal(optionRaw, &options); err != nil {
				t.Fatal(err)
			}
			for _, row := range options.ArgumentCases {
				if row.Accepted {
					continue
				}
				args := map[string]interface{}{"emoji": lifecycle.Open["emoji"], "title": lifecycle.Open["title"], "prompt": lifecycle.Open["prompt"], "goal": lifecycle.Open["goal"], "client": lifecycle.Open["client"], "no_issue": true, "no_management": true}
				for key, value := range row.Arguments {
					args[key] = value
				}
				wire, _ := json.Marshal(map[string]interface{}{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": map[string]interface{}{"name": "ticket_open", "arguments": args}})
				var out strings.Builder
				if err := Serve(context.Background(), servers[0], io.NopCloser(strings.NewReader(string(wire)+"\n")), &out); err != nil {
					t.Fatal(err)
				}
				var reply struct{ Error json.RawMessage }
				if err := json.Unmarshal([]byte(out.String()), &reply); err != nil {
					t.Fatal(err)
				}
				if len(reply.Error) == 0 || string(reply.Error) == "null" {
					t.Fatalf("malformed authority flags were accepted: %s", out.String())
				}
				reader := graphqlpkg.NewRepoContext(roots[0])
				tickets, err := reader.GetTickets(nil, nil, nil, nil)
				if err != nil || len(tickets) != 0 {
					t.Fatal("malformed authority flags changed private ticket state")
				}
			}
			opened := []outcome{}
			for index := 0; index < 2; index++ {
				for _, draft := range []string{lifecycle.Open["draft"].(string), lifecycle.Reopen["draft"].(string)} {
					destination := workspace.RepoMetaPathForRoot(roots[index], filepath.Join("📝️drafts", draft, "note.md"))
					if len(utf16.Encode([]rune(destination))) > 256 {
						t.Fatal("bounded private draft required")
					}
					if err := os.MkdirAll(filepath.Dir(destination), 0755); err != nil {
						t.Fatal(err)
					}
					if err := os.WriteFile(destination, []byte(lifecycle.Roots[index].Draft), 0644); err != nil {
						t.Fatal(err)
					}
				}
				opened = append(opened, call(index, "ticket_open", map[string]interface{}{"emoji": lifecycle.Open["emoji"], "title": lifecycle.Open["title"], "prompt": lifecycle.Open["prompt"], "goal": lifecycle.Open["goal"], "client": lifecycle.Open["client"], "draft": lifecycle.Open["draft"], "llm": "", "effort": "", "no_issue": lifecycle.Open["noIssue"], "no_management": lifecycle.Open["noManagement"]}))
				if opened[index].Ticket.Status != model.TicketStatusOpen || opened[index].Ticket.Management != nil || !strings.HasSuffix(opened[index].Path, "/"+lifecycle.Expected.OriginalSlug) {
					t.Fatalf("rooted open: %+v", opened[index])
				}
			}
			closed := call(0, "ticket_close", map[string]interface{}{"path": opened[0].Path, "summary": lifecycle.Close["summary"], "files": lifecycle.Close["files"], "title": lifecycle.Change["title"], "no_management": lifecycle.Close["noManagement"]})
			if closed.Ticket.Status != model.TicketStatusClosed || !strings.HasSuffix(closed.Path, "/"+lifecycle.Expected.Slug) {
				t.Fatalf("rooted close: %+v", closed)
			}
			reopened := call(0, "ticket_reopen", map[string]interface{}{"path": closed.Path, "prompt": lifecycle.Reopen["prompt"], "client": lifecycle.Reopen["client"], "draft": lifecycle.Reopen["draft"], "llm": "", "effort": "", "no_management": lifecycle.Reopen["noManagement"]})
			if reopened.Path != closed.Path || reopened.Ticket.Status != model.TicketStatusOpen {
				t.Fatalf("rooted reopen: %+v", reopened)
			}
			for index := 0; index < 2; index++ {
				reader := graphqlpkg.NewRepoContext(roots[index])
				tickets, err := reader.GetTickets(nil, nil, nil, nil)
				if err != nil || len(tickets) != 1 {
					t.Fatalf("rooted persisted ticket %d: %v %d", index, err, len(tickets))
				}
				expectedTitle := lifecycle.Open["title"]
				expectedKinds := []string{"ticket.open"}
				if index == 0 {
					expectedTitle = lifecycle.Change["title"]
					expectedKinds = append(expectedKinds, "ticket.close", "ticket.reopen")
				}
				if tickets[0].Title != expectedTitle || tickets[0].Status != model.TicketStatusOpen {
					t.Fatalf("cross-root ticket mutation %d: %+v", index, tickets[0])
				}
				kinds := []string{}
				for _, interaction := range tickets[0].Interactions {
					kinds = append(kinds, interaction.Kind)
				}
				if !reflect.DeepEqual(kinds, expectedKinds) {
					t.Fatalf("rooted interaction history %d: %v", index, kinds)
				}
				if index == 0 {
					for _, name := range lifecycle.Expected.DraftFiles {
						destination := filepath.Join(tickets[0].FolderPath, name)
						if len(utf16.Encode([]rune(destination))) > 256 {
							t.Fatal("bounded transferred draft required")
						}
						content, err := os.ReadFile(destination)
						if err != nil || string(content) != lifecycle.Roots[index].Draft {
							t.Fatalf("rooted draft %s: %v", name, err)
						}
					}
				}
			}
		}
		for index, server := range servers {
			wire, _ := json.Marshal(map[string]interface{}{"jsonrpc": "2.0", "id": 2, "method": "prompts/get", "params": map[string]interface{}{"name": "enhance", "arguments": map[string]string{"prompt": "probe"}}})
			var out strings.Builder
			if err := Serve(context.Background(), server, io.NopCloser(strings.NewReader(string(wire)+"\n")), &out); err != nil {
				t.Fatal(err)
			}
			var reply struct {
				Error  interface{}
				Result GetPromptResult
			}
			if err := json.Unmarshal([]byte(out.String()), &reply); err != nil {
				t.Fatal(err)
			}
			if reply.Error != nil || len(reply.Result.Messages) != 1 || reply.Result.Messages[0].Content.Text != corpus.Cases[index].Name+"|probe" {
				t.Fatalf("rooted prompt dispatch: %s", out.String())
			}
		}
		firstIDs := []string{}
		for index := 0; index < 2; index++ {
			read := ReadResourceRequest{}
			read.Params.URI = "repo://files"
			contents, err := servers[index].resources[read.Params.URI](context.Background(), read)
			if err != nil {
				t.Fatal(err)
			}
			encoded, _ := json.Marshal(contents[0])
			var envelope struct{ Text string }
			if err := json.Unmarshal(encoded, &envelope); err != nil {
				t.Fatal(err)
			}
			var document struct {
				Repo struct{ Files []struct{ ID string } }
			}
			if err := json.Unmarshal([]byte(envelope.Text), &document); err != nil || len(document.Repo.Files) == 0 {
				t.Fatal("missing real file identity")
			}
			firstIDs = append(firstIDs, document.Repo.Files[0].ID)
		}
		for index, id := range firstIDs {
			wire, _ := json.Marshal(map[string]interface{}{"jsonrpc": "2.0", "id": 1, "method": "resources/read", "params": map[string]string{"uri": "repo://file/" + id}})
			var out strings.Builder
			if err := Serve(context.Background(), servers[0], io.NopCloser(strings.NewReader(string(wire)+"\n")), &out); err != nil {
				t.Fatal(err)
			}
			var reply struct {
				Error  interface{}
				Result interface{}
			}
			if err := json.Unmarshal([]byte(out.String()), &reply); err != nil {
				t.Fatal(err)
			}
			if (reply.Error == nil) != (index == 0) {
				t.Fatalf("rooted template dispatch %d: %s", index, out.String())
			}
		}
		for turn := 0; turn < 3; turn++ {
			for index := len(servers) - 1; index >= 0; index-- {
				for _, read := range corpus.Reads {
					request := ReadResourceRequest{}
					request.Params.URI = read.URI
					contents, err := servers[index].resources[read.URI](context.Background(), request)
					if err != nil || len(contents) != 1 {
						t.Fatalf("%s %s: %v", profile, read.URI, err)
					}
					encoded, err := json.Marshal(contents[0])
					if err != nil {
						t.Fatal(err)
					}
					var wire struct {
						URI      string
						MIMEType string `json:"mimeType"`
						Text     string
					}
					if err := json.Unmarshal(encoded, &wire); err != nil {
						t.Fatal(err)
					}
					if wire.URI != read.URI || wire.MIMEType != corpus.MimeType {
						t.Fatalf("resource envelope: %s", encoded)
					}
					var document struct {
						Repo map[string][]map[string]interface{}
					}
					if err := json.Unmarshal([]byte(wire.Text), &document); err != nil {
						t.Fatal(err)
					}
					entries, found := document.Repo[read.Field]
					if !found || entries == nil {
						t.Fatalf("missing array: %s", wire.Text)
					}
					actual := []string{}
					for _, entry := range entries {
						relative, ok := entry["path"].(string)
						if !ok || filepath.IsAbs(relative) || strings.HasPrefix(relative, "../") {
							t.Fatalf("unscoped path: %v", entry)
						}
						if entry["id"] == "" || entry["name"] == "" {
							t.Fatalf("missing identity: %v", entry)
						}
						actual = append(actual, relative)
					}
					expected := []string{}
					if read.Field == "files" {
						for _, file := range corpus.Cases[index].Files {
							expected = append(expected, file.Path)
						}
					} else {
						expected = append(expected, corpus.Cases[index].Folders...)
					}
					sort.Strings(actual)
					sort.Strings(expected)
					if !reflect.DeepEqual(actual, expected) {
						t.Fatalf("%s/%s/%s: %v != %v", profile, corpus.Cases[index].Name, read.Field, actual, expected)
					}
				}
			}
		}
	}
	after, err := os.Stat(".")
	if err != nil || !os.SameFile(cwd, after) {
		t.Fatal("server changed process cwd")
	}
	afterRoot, afterBound := workspace.LookupRootDir()
	if afterRoot != originalRoot || afterBound != originalBound {
		t.Fatal("server changed ambient root")
	}
}

func TestMcpRepositoryOptionsPreservePresence(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧭️root-selection/🧪️tests/🧫️fixtures/🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var corpus struct {
		OptionCases []struct {
			Args     []string
			Root     *string
			Accepted bool
		}
	}
	if err := json.Unmarshal(raw, &corpus); err != nil {
		t.Fatal(err)
	}
	for _, row := range corpus.OptionCases {
		command, config := NewRootWithConfig(defaultEngineFactory)
		command.SetArgs(append([]string{"mcp", "--dry-run"}, row.Args...))
		err := command.Execute()
		if (err == nil) != row.Accepted {
			t.Fatalf("%v: accepted=%v error=%v", row.Args, row.Accepted, err)
		}
		if err == nil && row.Root != nil && config.Repo != *row.Root {
			t.Fatalf("%v lost explicit root", row.Args)
		}
	}
	for _, argv := range [][]string{{"mcp", "--repo", "chosen"}, {"--repo", "chosen", "mcp"}, {"--repo=chosen", "mcp"}} {
		plan := PlanNativeEntrypoint(argv)
		expected := []string{}
		for _, argument := range argv {
			if argument != "mcp" {
				expected = append(expected, argument)
			}
		}
		if plan.Kind != "mcp" || !reflect.DeepEqual(plan.Arguments, expected) {
			t.Fatalf("root ingress: %v -> %+v", argv, plan)
		}
	}
}

func TestMcpResourceTemplateFixture(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/📄️resources/🧭️routes/🧪️tests/🧫️fixtures/🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Cases []struct {
			Name     string
			Template string
			URI      string
			Expected bool
		}
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.Cases {
		if actual := matchResourceTemplate(row.Template, row.URI); actual != row.Expected {
			t.Errorf("%s: %v != %v", row.Name, actual, row.Expected)
		}
	}
}

type rootProcessRecorder struct{ requests []providers.ProcessRequest }

func (recorder *rootProcessRecorder) Run(request providers.ProcessRequest) providers.ProcessOutcome {
	recorder.requests = append(recorder.requests, request)
	return providers.ProcessOutcome{}
}
func (recorder *rootProcessRecorder) Issued() [][]string {
	result := [][]string{}
	for _, request := range recorder.requests {
		result = append(result, request.Argv)
	}
	return result
}

func TestMcpRepositoryPortsRetainCapturedConfiguration(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(checkoutRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/📄️resources/📥️bindings/🧪️tests/🧫️fixtures/🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Contexts   []struct{ ID, Root, Endpoint, Token string }
		Operations []struct{ Context, Kind, Ambient, Expected string }
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		t.Fatal(err)
	}
	type received struct {
		endpoint, token, path string
		payload               map[string]interface{}
	}
	messages := make(chan received, len(fixture.Operations))
	emitters := map[string]events.HTTPEmitter{}
	recorders := map[string]*rootProcessRecorder{}
	ports := map[string]*providers.GitHubManagementProvider{}
	for _, row := range fixture.Contexts {
		row := row
		server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
			var payload map[string]interface{}
			if err := json.NewDecoder(request.Body).Decode(&payload); err != nil {
				t.Error(err)
			}
			messages <- received{row.Endpoint, request.Header.Get("Authorization"), request.URL.Path, payload}
			writer.WriteHeader(http.StatusNoContent)
		}))
		defer server.Close()
		emitters[row.ID] = events.NewHTTPEmitter(server.URL, row.Token)
		recorder := &rootProcessRecorder{}
		recorders[row.ID] = recorder
		ports[row.ID] = providers.NewGitHubManagementProvider(row.Root, recorder)
	}
	for _, operation := range fixture.Operations {
		if operation.Kind == "process" {
			t.Setenv("SEMIO_REPO_ROOT", operation.Ambient)
			if err := ports[operation.Context].CloseIssue("https://example.invalid/issues/1"); err != nil {
				t.Fatal(err)
			}
			requests := recorders[operation.Context].requests
			if len(requests) == 0 {
				t.Fatal("provider skipped actual process port")
			}
			for _, request := range requests {
				if request.Cwd != operation.Expected {
					t.Fatalf("captured process root: %+v", request)
				}
			}
			continue
		}
		t.Setenv("COMPOSE_SERVER_ADDR", operation.Ambient)
		t.Setenv("COMPOSE_SERVER_TOKEN", "foreign-token")
		emitters[operation.Context].Emit(events.EventGoalChangeEnded, "private-root-test", map[string]string{"context": operation.Context})
		select {
		case message := <-messages:
			expectedToken := ""
			for _, row := range fixture.Contexts {
				if row.ID == operation.Context {
					expectedToken = row.Token
				}
			}
			if message.endpoint != operation.Expected || message.token != "Bearer "+expectedToken || message.path != "/api/v1/events" || message.payload["source"] != "private-root-test" {
				t.Fatalf("captured event port: %+v", message)
			}
		case <-time.After(time.Second):
			t.Fatal("captured endpoint received no event")
		}
	}
}

func TestMcpCancellationClosesOwnedProtocolInput(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	reader, writer := io.Pipe()
	defer writer.Close()
	done := make(chan error, 1)
	server := NewMCPServer("private", "1")
	go func() { done <- Serve(ctx, server, reader, io.Discard) }()
	cancel()
	select {
	case err := <-done:
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("cancellation: %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("owned protocol input did not settle")
	}
	if _, err := writer.Write([]byte("{}\n")); err == nil {
		t.Fatal("cancelled input remained open")
	}
}

func TestMcpScopedIdentitiesRetainTheirParentTechnology(t *testing.T) {
	bundle := model.Bundle{Name: "same/child", Root: "src", Kind: model.BundleKindLibrary}
	first := bundle.WithTechnology(model.Technology{Name: "same", Emoji: "🧬️"})
	second := bundle.WithTechnology(model.Technology{Name: "same", Emoji: "🔬️"})
	a := model.NewArtifactIndex("", []model.Bundle{first})
	b := model.NewArtifactIndex("", []model.Bundle{second})
	left, right := a.BuildFileID("src/file.rs", nil), b.BuildFileID("src/file.rs", nil)
	if left == right || left == "" || right == "" {
		t.Fatal("parent identities were not retained")
	}
	file := model.File{ID: left, Path: "src/file.rs", URI: "repo://file/" + left}
	if file.GetID() != left || file.GetURI() != file.URI {
		t.Fatal("file ignored scoped identity")
	}
	folderID := a.BuildFolderID("src", nil)
	folder := model.Folder{ID: folderID, Path: "src", URI: "repo://folder/" + folderID}
	if folder.GetID() != folderID || folder.GetURI() != folder.URI {
		t.Fatal("folder ignored scoped identity")
	}
	section := model.Section{ID: left + "§section", FilePath: "src/file.rs", Name: "section"}
	definition := model.Definition{ID: left + "§definition", FilePath: "src/file.rs", Name: "definition"}
	if section.GetID() != section.ID || definition.GetID() != definition.ID {
		t.Fatal("nested source ignored scoped identity")
	}
}
