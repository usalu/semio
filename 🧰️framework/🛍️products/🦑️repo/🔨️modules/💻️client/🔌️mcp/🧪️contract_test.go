// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"os"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

// #region 🧫️Fixture

type contractFixture struct {
	Schema          string `json:"schema"`
	ProtocolVersion string `json:"protocolVersion"`
	Vectors         []struct {
		Name     string `json:"name"`
		Ready    bool   `json:"ready"`
		Request  string `json:"request"`
		Response string `json:"response"`
	} `json:"vectors"`
}

func TestG2CanonicalGoldenVectors(t *testing.T) {
	data, err := os.ReadFile("🧫️fixtures/g2-contract.json")
	if err != nil {
		t.Fatal(err)
	}
	var fixture contractFixture
	if err := decodeExact(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.mcp.contract/1" || fixture.ProtocolVersion != ProtocolVersion {
		t.Fatalf("unexpected fixture identity: %#v", fixture)
	}
	for _, vector := range fixture.Vectors {
		t.Run(vector.Name, func(t *testing.T) {
			server := newContractServer(t, DefaultLimits(), nil, nil)
			session, err := server.Connect("fixture", nil)
			if err != nil {
				t.Fatal(err)
			}
			if vector.Ready {
				initializeSession(t, session)
			}
			response, err := session.Dispatch(context.Background(), []byte(vector.Request))
			if err != nil {
				t.Fatal(err)
			}
			if string(response) != vector.Response {
				t.Fatalf("response\n got: %s\nwant: %s", response, vector.Response)
			}
		})
	}
}

func TestG2ProductionOwnedRuntimePipe(t *testing.T) {
	repository := &testRepository{called: make(chan string, 1)}
	serverPipe, clientPipe := net.Pipe()
	type outcome struct {
		server *Server
		err    error
	}
	finished := make(chan outcome, 1)
	go func() {
		server, err := runMCP(context.Background(), serverPipe, repository)
		finished <- outcome{server: server, err: err}
	}()
	reader := bufio.NewReader(clientPipe)
	writeLine := func(payload string) {
		t.Helper()
		if _, err := io.WriteString(clientPipe, payload+"\n"); err != nil {
			t.Fatal(err)
		}
	}
	readLine := func() []byte {
		t.Helper()
		line, err := reader.ReadBytes('\n')
		if err != nil {
			t.Fatal(err)
		}
		return bytes.TrimSpace(line)
	}
	writeLine(fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"production-test","version":"1"}}}`, ProtocolVersion))
	line := readLine()
	assertResponseCode(t, line, 0)
	if !bytes.Contains(line, []byte(`"protocolVersion":"`+ProtocolVersion+`"`)) || !bytes.Contains(line, []byte(`"tools":{}`)) {
		t.Fatalf("production lifecycle/capability response: %s", line)
	}
	writeLine(`{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`)
	writeLine(`{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"ticket_open","arguments":{"emoji":"🎫️","title":"Fixture","prompt":"Fixture","goal":"fixture"}}}`)
	assertResponseCode(t, readLine(), 0)
	if called := <-repository.called; called != "ticket_open" {
		t.Fatalf("production handler got=%q", called)
	}
	_ = clientPipe.Close()
	result := <-finished
	if result.err != nil {
		t.Fatal(result.err)
	}
	if result.server == nil {
		t.Fatal("production run did not return its owned server")
	}
	events := result.server.Events().Events()
	if len(events) < 6 || events[0].Kind != "session.opened" {
		t.Fatalf("production event evidence: %#v", events)
	}
}

func TestG2ProductionComponentRejectsDirectDelegationMutation(t *testing.T) {
	source, err := os.ReadFile("🐹️.go")
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(source, []byte("client.RunMCP")) {
		t.Fatal("production restored direct G1 MCP delegation")
	}
	for _, required := range [][]byte{[]byte("runMCP("), []byte("NewRepositoryServer("), []byte("server.Serve(")} {
		if !bytes.Contains(source, required) {
			t.Fatalf("production component bypasses owned runtime marker %q", required)
		}
	}
}

func TestG2TransportRejectsBusyBranchMutation(t *testing.T) {
	source, err := os.ReadFile("🐹️transport.go")
	if err != nil {
		t.Fatal(err)
	}
	if err := verifyBusyBranch(source); err != nil {
		t.Fatal(err)
	}
	mutations := []struct {
		name   string
		source []byte
	}{
		{name: "blocking", source: bytes.Replace(source, []byte("case jobs <- payload:\n\t\tdefault:"), []byte("case jobs <- payload:\n\t\tcase <-ctx.Done():"), 1)},
		{name: "code", source: bytes.Replace(source, []byte("CodeServerBusy"), []byte("CodeInternalError"), 1)},
		{name: "credit", source: bytes.Replace(source, []byte("default:\n\t\t\tserver.handlerQueued.Add(-1)"), []byte("default:\n\t\t\tserver.handlerQueued.Add(0)"), 1)},
	}
	for _, mutation := range mutations {
		t.Run(mutation.name, func(t *testing.T) {
			if err := verifyBusyBranch(mutation.source); err == nil {
				t.Fatal("hostile busy-branch mutation survived")
			}
		})
	}
}

func verifyBusyBranch(source []byte) error {
	start := bytes.Index(source, []byte("server.handlerQueued.Add(1)"))
	if start < 0 {
		return errors.New("transport removed bounded queue admission")
	}
	endOffset := bytes.Index(source[start:], []byte("if err := write(ctx, response)"))
	if endOffset < 0 {
		return errors.New("transport removed busy response write")
	}
	branch := source[start : start+endOffset]
	for _, required := range [][]byte{[]byte("select {"), []byte("case jobs <- payload:"), []byte("default:"), []byte("server.handlerQueued.Add(-1)"), []byte("CodeServerBusy"), []byte(`"handler queue full"`)} {
		if !bytes.Contains(branch, required) {
			return fmt.Errorf("transport weakened bounded busy branch marker %q", required)
		}
	}
	return nil
}

// #endregion 🧫️Fixture

// #region 🧰️Harness

type contractControls struct {
	started chan struct{}
	release chan struct{}
	calls   atomic.Int64
}

type testRepository struct {
	called  chan string
	started chan struct{}
}

type saturationRepository struct {
	started chan string
	active  atomic.Int64
	peak    atomic.Int64
	calls   atomic.Int64
}

func (repository *saturationRepository) Call(ctx context.Context, _ string, raw json.RawMessage) (RepositoryResult, error) {
	var arguments struct {
		Title string `json:"title"`
	}
	if err := json.Unmarshal(raw, &arguments); err != nil || arguments.Title == "" {
		return RepositoryResult{}, errors.New("mcp: saturation title required")
	}
	active := repository.active.Add(1)
	for peak := repository.peak.Load(); active > peak && !repository.peak.CompareAndSwap(peak, active); peak = repository.peak.Load() {
	}
	repository.calls.Add(1)
	repository.started <- arguments.Title
	defer repository.active.Add(-1)
	if arguments.Title == "after" {
		return RepositoryResult{Text: "recovered", Structured: json.RawMessage(`{"recovered":true}`)}, nil
	}
	<-ctx.Done()
	return RepositoryResult{}, ctx.Err()
}

func (*saturationRepository) Read(_ context.Context, uri string) (ResourceContent, error) {
	return ResourceContent{URI: uri, MIMEType: "text/plain", Text: "ok"}, nil
}

func (*saturationRepository) Prompt(_ context.Context, name string, _ map[string]string) (GetPromptResult, error) {
	return GetPromptResult{Description: name}, nil
}

type productionPipe struct {
	server   *Server
	client   net.Conn
	reader   *bufio.Reader
	finished chan error
}

func newProductionPipe(t *testing.T, peer string, limits Limits, repository RepositoryHandlers) *productionPipe {
	t.Helper()
	server, err := NewRepositoryServerWithLimits(repository, limits)
	if err != nil {
		t.Fatal(err)
	}
	serverPipe, clientPipe := net.Pipe()
	harness := &productionPipe{server: server, client: clientPipe, reader: bufio.NewReader(clientPipe), finished: make(chan error, 1)}
	go func() { harness.finished <- server.Serve(context.Background(), peer, serverPipe) }()
	return harness
}

func (harness *productionPipe) write(t *testing.T, payload string) {
	t.Helper()
	if err := harness.client.SetWriteDeadline(time.Now().Add(time.Second)); err != nil {
		t.Fatal(err)
	}
	if _, err := io.WriteString(harness.client, payload+"\n"); err != nil {
		t.Fatal(err)
	}
}

func (harness *productionPipe) read(t *testing.T) []byte {
	t.Helper()
	if err := harness.client.SetReadDeadline(time.Now().Add(time.Second)); err != nil {
		t.Fatal(err)
	}
	line, err := harness.reader.ReadBytes('\n')
	if err != nil {
		t.Fatal(err)
	}
	return bytes.TrimSpace(line)
}

func (harness *productionPipe) initialize(t *testing.T) {
	t.Helper()
	harness.write(t, fmt.Sprintf(`{"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"saturation","version":"1"}}}`, ProtocolVersion))
	assertResponseCode(t, harness.read(t), 0)
	harness.write(t, `{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`)
}

func waitHandlerStats(t *testing.T, server *Server, active, queued int64) HandlerStats {
	t.Helper()
	deadline := time.Now().Add(time.Second)
	for {
		stats := server.HandlerStats()
		if stats.Workers == int64(stats.Capacity) && stats.Active == active && stats.Queued == queued && stats.Credits == int64(stats.Capacity)-queued {
			return stats
		}
		if time.Now().After(deadline) {
			t.Fatalf("handler stats got=%#v want active=%d queued=%d", stats, active, queued)
		}
		time.Sleep(time.Millisecond)
	}
}

func assertResponseID(t *testing.T, response []byte, id string) {
	t.Helper()
	var envelope Response
	if err := json.Unmarshal(response, &envelope); err != nil {
		t.Fatal(err)
	}
	if string(envelope.ID) != fmt.Sprintf("%q", id) {
		t.Fatalf("response id got=%s want=%q response=%s", envelope.ID, id, response)
	}
}

func assertAtomicRequestPairs(t *testing.T, events []Event, keys ...string) {
	t.Helper()
	found := make(map[string]int, len(keys))
	for index, event := range events {
		if event.Kind != "request.received" {
			continue
		}
		if index+1 >= len(events) || events[index+1].Kind != "response.sent" || events[index+1].RequestID != event.RequestID {
			t.Fatalf("partial request event at index %d: %#v", index, events)
		}
		found[event.RequestID]++
	}
	for _, key := range keys {
		if found[key] != 1 {
			t.Fatalf("request pair %q count=%d events=%#v", key, found[key], events)
		}
	}
}

func (repository *testRepository) Call(ctx context.Context, name string, _ json.RawMessage) (RepositoryResult, error) {
	if repository.called != nil {
		repository.called <- name
	}
	if repository.started != nil {
		select {
		case <-repository.started:
		default:
			close(repository.started)
		}
		<-ctx.Done()
		return RepositoryResult{}, ctx.Err()
	}
	return RepositoryResult{Text: "ok", Structured: json.RawMessage(`{"ok":true}`)}, nil
}

func (*testRepository) Read(_ context.Context, uri string) (ResourceContent, error) {
	return ResourceContent{URI: uri, MIMEType: "text/plain", Text: "ok"}, nil
}

func (*testRepository) Prompt(_ context.Context, name string, _ map[string]string) (GetPromptResult, error) {
	return GetPromptResult{Description: name, Messages: []PromptMessage{{Role: "user", Content: Content{Type: "text", Text: name}}}}, nil
}

func newContractServer(t *testing.T, limits Limits, controls *contractControls, sink Sink) *Server {
	t.Helper()
	server, err := NewServer(Config{ServerInfo: Implementation{Name: "repo", Version: "1.0.0"}, Instructions: "owned", Limits: limits})
	if err != nil {
		t.Fatal(err)
	}
	if controls == nil {
		controls = &contractControls{}
	}
	register := func(err error) {
		if err != nil {
			t.Fatal(err)
		}
	}
	register(server.RegisterTool(Tool{Name: "echo", InputSchema: Schema{Type: "object"}}, func(ctx context.Context, params CallToolParams, progress ProgressReporter) (CallToolResult, error) {
		controls.calls.Add(1)
		var arguments struct {
			Text string `json:"text"`
		}
		if err := DecodeParams(params.Arguments, &arguments); err != nil {
			return CallToolResult{}, &HandlerError{Code: -32010, Message: "invalid echo arguments"}
		}
		if err := progress.Report(ctx, 1, nil, "echo"); err != nil {
			return CallToolResult{}, err
		}
		return CallToolResult{Content: []Content{{Type: "text", Text: arguments.Text}}}, nil
	}))
	register(server.RegisterTool(Tool{Name: "block", InputSchema: Schema{Type: "object"}}, func(ctx context.Context, _ CallToolParams, progress ProgressReporter) (CallToolResult, error) {
		controls.calls.Add(1)
		if controls.started != nil {
			select {
			case <-controls.started:
			default:
				close(controls.started)
			}
		}
		if err := progress.Report(ctx, 0, nil, "started"); err != nil {
			return CallToolResult{}, err
		}
		if controls.release != nil {
			select {
			case <-controls.release:
			case <-ctx.Done():
				return CallToolResult{}, ctx.Err()
			}
		} else {
			<-ctx.Done()
			return CallToolResult{}, ctx.Err()
		}
		return CallToolResult{Content: []Content{{Type: "text", Text: "released"}}}, nil
	}))
	register(server.RegisterResource(Resource{URI: "repo://goals", Name: "goals", MIMEType: "application/json"}, func(context.Context, ReadResourceParams, ProgressReporter) (ReadResourceResult, error) {
		return ReadResourceResult{Contents: []ResourceContent{{URI: "repo://goals", MIMEType: "application/json", Text: "[]"}}}, nil
	}))
	register(server.RegisterResourceTemplate(ResourceTemplate{URITemplate: "repo://ticket/{id}", Name: "ticket"}))
	register(server.RegisterPrompt(Prompt{Name: "review", Arguments: []PromptArgument{{Name: "scope", Required: true}}}, func(_ context.Context, params GetPromptParams, _ ProgressReporter) (GetPromptResult, error) {
		scope := params.Arguments["scope"]
		return GetPromptResult{Description: "Review " + scope, Messages: []PromptMessage{{Role: "user", Content: Content{Type: "text", Text: scope}}}}, nil
	}))
	_ = sink
	return server
}

func initializeSession(t *testing.T, session *Session) {
	t.Helper()
	request := fmt.Sprintf(`{"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"test","version":"1"}}}`, ProtocolVersion)
	response, err := session.Dispatch(context.Background(), []byte(request))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, response, 0)
	initialized := `{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`
	if response, err := session.Dispatch(context.Background(), []byte(initialized)); err != nil || response != nil {
		t.Fatalf("initialized notification: response=%s err=%v", response, err)
	}
}

func assertResponseCode(t *testing.T, response []byte, code int) {
	t.Helper()
	var envelope Response
	if err := json.Unmarshal(response, &envelope); err != nil {
		t.Fatalf("decode response %s: %v", response, err)
	}
	if code == 0 && envelope.Error != nil {
		t.Fatalf("unexpected error: %v", envelope.Error)
	}
	if code != 0 && (envelope.Error == nil || envelope.Error.Code != code) {
		t.Fatalf("error code got=%v want=%d response=%s", envelope.Error, code, response)
	}
}

func request(method, id, params string) []byte {
	return []byte(fmt.Sprintf(`{"jsonrpc":"2.0","id":%s,"method":%q,"params":%s}`, id, method, params))
}

func cancelNotification(id, reason string) []byte {
	return []byte(fmt.Sprintf(`{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":%s,"reason":%q}}`, id, reason))
}

// #endregion 🧰️Harness

// #region 🚦Lifecycle

func TestG2LifecycleAndPagination(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxPageItems = 2
	server := newContractServer(t, limits, nil, nil)
	if err := server.RegisterTool(Tool{Name: "zeta", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, nil
	}); err != nil {
		t.Fatal(err)
	}
	session, err := server.Connect("page", nil)
	if err != nil {
		t.Fatal(err)
	}
	response, err := session.Dispatch(context.Background(), request("tools/list", "1", `{}`))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, response, CodeNotInitialized)
	initializeSession(t, session)
	response, err = session.Dispatch(context.Background(), request("tools/list", "2", `{}`))
	if err != nil {
		t.Fatal(err)
	}
	var envelope struct {
		Result ListToolsResult `json:"result"`
	}
	if err := json.Unmarshal(response, &envelope); err != nil {
		t.Fatal(err)
	}
	if len(envelope.Result.Tools) != 2 || envelope.Result.Tools[0].Name != "block" || envelope.Result.Tools[1].Name != "echo" || envelope.Result.NextCursor != "2" {
		t.Fatalf("unexpected first page: %#v", envelope.Result)
	}
	response, err = session.Dispatch(context.Background(), request("tools/list", "3", `{"cursor":"2"}`))
	if err != nil {
		t.Fatal(err)
	}
	envelope = struct {
		Result ListToolsResult `json:"result"`
	}{}
	if err := json.Unmarshal(response, &envelope); err != nil {
		t.Fatal(err)
	}
	if len(envelope.Result.Tools) != 1 || envelope.Result.Tools[0].Name != "zeta" || envelope.Result.NextCursor != "" {
		t.Fatalf("unexpected second page: %#v", envelope.Result)
	}
	response, err = session.Dispatch(context.Background(), request("resources/templates/list", "4", `{}`))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, response, 0)
	response, err = session.Dispatch(context.Background(), request("initialize", "5", fmt.Sprintf(`{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"again","version":"1"}}`, ProtocolVersion)))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, response, CodeInvalidRequest)
}

func TestG2RegistryMaximumPlusOne(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxRegistryItems = 3
	server := newContractServer(t, limits, nil, nil)
	handler := func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, nil
	}
	if err := server.RegisterTool(Tool{Name: "third", InputSchema: Schema{Type: "object"}}, handler); err != nil {
		t.Fatal(err)
	}
	if err := server.RegisterTool(Tool{Name: "fourth", InputSchema: Schema{Type: "object"}}, handler); !errors.Is(err, ErrLimit) {
		t.Fatalf("got %v, want ErrLimit", err)
	}
}

// #endregion 🚦Lifecycle

// #region 🛑Cancellation

func TestG2CancellationBeforeDuringAndAfterHandler(t *testing.T) {
	t.Run("before", func(t *testing.T) {
		controls := &contractControls{}
		server := newContractServer(t, DefaultLimits(), controls, nil)
		session, _ := server.Connect("before", nil)
		initializeSession(t, session)
		_, _ = session.Dispatch(context.Background(), cancelNotification(`"before"`, "before"))
		response, err := session.Dispatch(context.Background(), request("tools/call", `"before"`, `{"name":"echo","arguments":{"text":"never"}}`))
		if err != nil {
			t.Fatal(err)
		}
		assertResponseCode(t, response, CodeRequestCancelled)
		if controls.calls.Load() != 0 {
			t.Fatal("handler ran after pre-cancellation")
		}
	})
	t.Run("during", func(t *testing.T) {
		controls := &contractControls{started: make(chan struct{})}
		server := newContractServer(t, DefaultLimits(), controls, nil)
		session, _ := server.Connect("during", nil)
		initializeSession(t, session)
		response := make(chan []byte, 1)
		go func() {
			result, _ := session.Dispatch(context.Background(), request("tools/call", `"during"`, `{"name":"block","arguments":{}}`))
			response <- result
		}()
		<-controls.started
		_, _ = session.Dispatch(context.Background(), cancelNotification(`"during"`, "during"))
		assertResponseCode(t, <-response, CodeRequestCancelled)
	})
	t.Run("after", func(t *testing.T) {
		server := newContractServer(t, DefaultLimits(), nil, nil)
		session, _ := server.Connect("after", nil)
		initializeSession(t, session)
		response, _ := session.Dispatch(context.Background(), request("tools/call", `"after"`, `{"name":"echo","arguments":{"text":"done"}}`))
		assertResponseCode(t, response, 0)
		_, _ = session.Dispatch(context.Background(), cancelNotification(`"after"`, "late"))
		response, _ = session.Dispatch(context.Background(), request("tools/call", `"next"`, `{"name":"echo","arguments":{"text":"still-live"}}`))
		assertResponseCode(t, response, 0)
		response, _ = session.Dispatch(context.Background(), request("tools/call", `"after"`, `{"name":"echo","arguments":{"text":"duplicate"}}`))
		assertResponseCode(t, response, CodeDuplicateRequest)
	})
}

func TestG2SamePipeCancellationAndProgress(t *testing.T) {
	repository := &testRepository{started: make(chan struct{})}
	serverPipe, clientPipe := net.Pipe()
	type outcome struct {
		server *Server
		err    error
	}
	finished := make(chan outcome, 1)
	go func() {
		server, err := runMCP(context.Background(), serverPipe, repository)
		finished <- outcome{server: server, err: err}
	}()
	reader := bufio.NewReader(clientPipe)
	write := func(payload string) {
		t.Helper()
		if _, err := io.WriteString(clientPipe, payload+"\n"); err != nil {
			t.Fatal(err)
		}
	}
	read := func() []byte {
		t.Helper()
		line, err := reader.ReadBytes('\n')
		if err != nil {
			t.Fatal(err)
		}
		return bytes.TrimSpace(line)
	}
	write(fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"pipe","version":"1"}}}`, ProtocolVersion))
	assertResponseCode(t, read(), 0)
	write(`{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`)
	write(`{"jsonrpc":"2.0","id":"blocked","method":"tools/call","params":{"name":"ticket_open","arguments":{"emoji":"🎫️","title":"Blocked","prompt":"Blocked","goal":"fixture"},"_meta":{"progressToken":"progress"}}}`)
	progress := read()
	if !bytes.Contains(progress, []byte(`"method":"notifications/progress"`)) || !bytes.Contains(progress, []byte(`"progressToken":"progress"`)) {
		t.Fatalf("missing same-pipe progress: %s", progress)
	}
	<-repository.started
	write(`{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":"blocked","reason":"same pipe"}}`)
	response := read()
	assertResponseCode(t, response, CodeRequestCancelled)
	_ = clientPipe.Close()
	result := <-finished
	if result.err != nil {
		t.Fatal(result.err)
	}
	events := result.server.Events().Events()
	foundPair := false
	for index := 0; index+1 < len(events); index++ {
		if events[index].RequestID == "s:blocked" && events[index].Kind == "request.received" && events[index+1].RequestID == "s:blocked" && events[index+1].Kind == "response.sent" {
			foundPair = true
		}
	}
	if !foundPair {
		t.Fatalf("cancelled request lacked atomic event pair: %#v", events)
	}
}

func TestG2ProductionPipeSaturationControlAndRecovery(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxHandlers = 2
	repository := &saturationRepository{started: make(chan string, 8)}
	harness := newProductionPipe(t, "saturation", limits, repository)
	harness.initialize(t)
	initial := waitHandlerStats(t, harness.server, 0, 0)
	if initial.Capacity != 2 || initial.Credits != 2 {
		t.Fatalf("initial queue credits: %#v", initial)
	}
	call := func(id, title, token string) string {
		return fmt.Sprintf(`{"jsonrpc":"2.0","id":%q,"method":"tools/call","params":{"name":"ticket_open","arguments":{"title":%q},"_meta":{"progressToken":%q}}}`, id, title, token)
	}
	assertProgress := func(token string) {
		t.Helper()
		progress := harness.read(t)
		if !bytes.Contains(progress, []byte(`"method":"notifications/progress"`)) || !bytes.Contains(progress, []byte(fmt.Sprintf(`"progressToken":%q`, token))) {
			t.Fatalf("progress token %q missing: %s", token, progress)
		}
	}
	harness.write(t, call("active-a", "active-a", "progress-a"))
	assertProgress("progress-a")
	if started := <-repository.started; started != "active-a" {
		t.Fatalf("first handler=%q", started)
	}
	harness.write(t, call("active-b", "active-b", "progress-b"))
	assertProgress("progress-b")
	if started := <-repository.started; started != "active-b" {
		t.Fatalf("second handler=%q", started)
	}
	zero := waitHandlerStats(t, harness.server, 2, 0)
	if zero.Credits != 2 {
		t.Fatalf("zero queue credits: %#v", zero)
	}
	harness.write(t, call("queued-a", "queued-a", "progress-queued-a"))
	waitHandlerStats(t, harness.server, 2, 1)
	harness.write(t, call("queued-b", "queued-b", "progress-queued-b"))
	full := waitHandlerStats(t, harness.server, 2, 2)
	if full.Credits != 0 {
		t.Fatalf("full queue credits: %#v", full)
	}
	harness.write(t, call("overflow", "overflow", "progress-overflow"))
	busy := harness.read(t)
	assertResponseID(t, busy, "overflow")
	assertResponseCode(t, busy, CodeServerBusy)
	var busyEnvelope Response
	if err := json.Unmarshal(busy, &busyEnvelope); err != nil || busyEnvelope.Error == nil || busyEnvelope.Error.Message != "handler queue full" {
		t.Fatalf("owned busy error: response=%s err=%v", busy, err)
	}
	if stats := waitHandlerStats(t, harness.server, 2, 2); stats.Credits != 0 || repository.calls.Load() != 2 {
		t.Fatalf("max+1 admitted a handler: stats=%#v calls=%d", stats, repository.calls.Load())
	}
	harness.write(t, string(cancelNotification(`"active-a"`, "release queue credit")))
	cancelled := harness.read(t)
	assertResponseID(t, cancelled, "active-a")
	assertResponseCode(t, cancelled, CodeRequestCancelled)
	assertProgress("progress-queued-a")
	if started := <-repository.started; started != "queued-a" {
		t.Fatalf("first queued handler=%q", started)
	}
	if stats := waitHandlerStats(t, harness.server, 2, 1); stats.Credits != 1 {
		t.Fatalf("first returned credit: %#v", stats)
	}
	harness.write(t, string(cancelNotification(`"active-b"`, "release final queue credit")))
	cancelled = harness.read(t)
	assertResponseID(t, cancelled, "active-b")
	assertResponseCode(t, cancelled, CodeRequestCancelled)
	assertProgress("progress-queued-b")
	if started := <-repository.started; started != "queued-b" {
		t.Fatalf("second queued handler=%q", started)
	}
	if stats := waitHandlerStats(t, harness.server, 2, 0); stats.Credits != 2 {
		t.Fatalf("all queue credits returned: %#v", stats)
	}
	for _, id := range []string{"queued-a", "queued-b"} {
		harness.write(t, string(cancelNotification(fmt.Sprintf("%q", id), "drain saturation")))
		cancelled = harness.read(t)
		assertResponseID(t, cancelled, id)
		assertResponseCode(t, cancelled, CodeRequestCancelled)
	}
	waitHandlerStats(t, harness.server, 0, 0)
	harness.write(t, call("after", "after", "progress-after"))
	assertProgress("progress-after")
	if started := <-repository.started; started != "after" {
		t.Fatalf("recovery handler=%q", started)
	}
	assertProgress("progress-after")
	recovered := harness.read(t)
	assertResponseID(t, recovered, "after")
	assertResponseCode(t, recovered, 0)
	waitHandlerStats(t, harness.server, 0, 0)
	if repository.peak.Load() != 2 || repository.active.Load() != 0 || repository.calls.Load() != 5 {
		t.Fatalf("handler accounting active=%d peak=%d calls=%d", repository.active.Load(), repository.peak.Load(), repository.calls.Load())
	}
	if err := harness.client.Close(); err != nil {
		t.Fatal(err)
	}
	select {
	case err := <-harness.finished:
		if !errors.Is(err, ErrPeerDropped) {
			t.Fatalf("serve close error=%v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("serve leaked after saturation recovery")
	}
	stats := harness.server.HandlerStats()
	if stats.Workers != 0 || stats.Active != 0 || stats.Queued != 0 || stats.Credits != 2 {
		t.Fatalf("worker or credit leak: %#v", stats)
	}
	assertAtomicRequestPairs(t, harness.server.Events().Events(), "s:overflow", "s:active-a", "s:active-b", "s:queued-a", "s:queued-b", "s:after")
}

func TestG2ProductionPipeCloseDuringSaturation(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxHandlers = 2
	repository := &saturationRepository{started: make(chan string, 8)}
	harness := newProductionPipe(t, "saturated-close", limits, repository)
	harness.initialize(t)
	waitHandlerStats(t, harness.server, 0, 0)
	call := func(id string) string {
		return fmt.Sprintf(`{"jsonrpc":"2.0","id":%q,"method":"tools/call","params":{"name":"ticket_open","arguments":{"title":%q},"_meta":{"progressToken":%q}}}`, id, id, "progress-"+id)
	}
	for _, id := range []string{"active-a", "active-b"} {
		harness.write(t, call(id))
		progress := harness.read(t)
		if !bytes.Contains(progress, []byte(`"method":"notifications/progress"`)) {
			t.Fatalf("missing saturation progress: %s", progress)
		}
		if started := <-repository.started; started != id {
			t.Fatalf("started=%q want=%q", started, id)
		}
	}
	waitHandlerStats(t, harness.server, 2, 0)
	for index, id := range []string{"queued-a", "queued-b"} {
		harness.write(t, call(id))
		waitHandlerStats(t, harness.server, 2, int64(index+1))
	}
	if err := harness.client.Close(); err != nil {
		t.Fatal(err)
	}
	select {
	case err := <-harness.finished:
		if !errors.Is(err, ErrPeerDropped) {
			t.Fatalf("serve close error=%v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("serve leaked while closing saturation")
	}
	stats := harness.server.HandlerStats()
	if stats.Workers != 0 || stats.Active != 0 || stats.Queued != 0 || stats.Credits != 2 {
		t.Fatalf("saturated close leaked accounting: %#v", stats)
	}
	if repository.active.Load() != 0 || repository.peak.Load() != 2 || repository.calls.Load() != 2 || len(repository.started) != 0 {
		t.Fatalf("saturated close handler leak active=%d peak=%d calls=%d pending-starts=%d", repository.active.Load(), repository.peak.Load(), repository.calls.Load(), len(repository.started))
	}
	harness.server.mu.RLock()
	session := harness.server.sessions["saturated-close"]
	harness.server.mu.RUnlock()
	if session == nil {
		t.Fatal("saturated close session missing")
	}
	select {
	case <-session.Done():
	case <-time.After(time.Second):
		t.Fatal("saturated close left request lifecycle running")
	}
	session.mu.Lock()
	activeRequests := len(session.active)
	session.mu.Unlock()
	if activeRequests != 0 {
		t.Fatalf("saturated close active requests=%d", activeRequests)
	}
	assertAtomicRequestPairs(t, harness.server.Events().Events(), "s:active-a", "s:active-b", "s:queued-a", "s:queued-b")
}

func TestG2DuplicateAndABASafeReconnect(t *testing.T) {
	controls := &contractControls{started: make(chan struct{})}
	server := newContractServer(t, DefaultLimits(), controls, nil)
	first, _ := server.Connect("peer", nil)
	initializeSession(t, first)
	firstResponse := make(chan []byte, 1)
	go func() {
		result, _ := first.Dispatch(context.Background(), request("tools/call", `"same"`, `{"name":"block","arguments":{}}`))
		firstResponse <- result
	}()
	<-controls.started
	duplicate, _ := first.Dispatch(context.Background(), request("tools/call", `"same"`, `{"name":"echo","arguments":{"text":"duplicate"}}`))
	assertResponseCode(t, duplicate, CodeDuplicateRequest)
	second, err := server.Connect("peer", nil)
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, <-firstResponse, CodeRequestCancelled)
	initializeSession(t, second)
	stale, _ := first.Dispatch(context.Background(), request("ping", `"old"`, `{}`))
	assertResponseCode(t, stale, CodeStaleSession)
	secondControls := &contractControls{started: make(chan struct{})}
	server.mu.Lock()
	registration := server.tools["block"]
	registration.handler = func(ctx context.Context, _ CallToolParams, _ ProgressReporter) (CallToolResult, error) {
		close(secondControls.started)
		<-ctx.Done()
		return CallToolResult{}, ctx.Err()
	}
	server.tools["block"] = registration
	server.mu.Unlock()
	secondResponse := make(chan []byte, 1)
	go func() {
		result, _ := second.Dispatch(context.Background(), request("tools/call", `"same"`, `{"name":"block","arguments":{}}`))
		secondResponse <- result
	}()
	<-secondControls.started
	_, _ = first.Dispatch(context.Background(), cancelNotification(`"same"`, "stale generation"))
	select {
	case <-secondResponse:
		t.Fatal("stale generation cancelled current request")
	case <-time.After(20 * time.Millisecond):
	}
	_, _ = second.Dispatch(context.Background(), cancelNotification(`"same"`, "current generation"))
	assertResponseCode(t, <-secondResponse, CodeRequestCancelled)
}

func TestG2InterruptedCloseCompletesWithoutPartialExchange(t *testing.T) {
	controls := &contractControls{started: make(chan struct{}), release: make(chan struct{})}
	server := newContractServer(t, DefaultLimits(), controls, nil)
	session, _ := server.Connect("close", nil)
	initializeSession(t, session)
	response := make(chan []byte, 1)
	go func() {
		result, _ := session.Dispatch(context.Background(), request("tools/call", `"close"`, `{"name":"block","arguments":{}}`))
		response <- result
	}()
	<-controls.started
	closeContext, cancel := context.WithCancel(context.Background())
	cancel()
	if err := session.Close(closeContext); !errors.Is(err, context.Canceled) {
		t.Fatalf("got %v, want context.Canceled", err)
	}
	assertResponseCode(t, <-response, CodeRequestCancelled)
	select {
	case <-session.Done():
	case <-time.After(time.Second):
		t.Fatal("close did not finish after interrupted waiter")
	}
	events := server.Events().Events()
	for index := 0; index < len(events); index++ {
		if events[index].Kind == "request.received" && (index+1 >= len(events) || events[index+1].Kind != "response.sent") {
			t.Fatal("partial request/response event commit")
		}
	}
}

// #endregion 🛑Cancellation

// #region 📏Bounds

func TestG2PayloadNestingAndResponseBounds(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxPayloadBytes = 512
	limits.MaxNesting = 6
	server := newContractServer(t, limits, nil, nil)
	session, _ := server.Connect("bounds", nil)
	initializeSession(t, session)
	base := `{"jsonrpc":"2.0","id":"exact","method":"tools/call","params":{"name":"echo","arguments":{"text":"` + `"}}}`
	padding := strings.Repeat("x", limits.MaxPayloadBytes-len(base))
	exact := []byte(strings.Replace(base, `""}}}`, `"`+padding+`"}}}`, 1))
	if len(exact) != limits.MaxPayloadBytes {
		t.Fatalf("exact payload length=%d", len(exact))
	}
	response, _ := session.Dispatch(context.Background(), exact)
	assertResponseCode(t, response, 0)
	response, _ = session.Dispatch(context.Background(), append(exact, ' '))
	assertResponseCode(t, response, CodePayloadTooLarge)
	accepted := request("missing", `"nested"`, `{"nested":[[[[0]]]]}`)
	response, _ = session.Dispatch(context.Background(), accepted)
	assertResponseCode(t, response, CodeMethodNotFound)
	within := request("tools/call", `"within"`, `{"name":"echo","arguments":{"text":"ok"}}`)
	response, _ = session.Dispatch(context.Background(), within)
	assertResponseCode(t, response, 0)
	tooDeep := request("missing", `"deep"`, `{"nested":[[[[[0]]]]]}`)
	response, _ = session.Dispatch(context.Background(), tooDeep)
	assertResponseCode(t, response, CodeInvalidRequest)
}

func TestG2InvalidIDAndCursor(t *testing.T) {
	server := newContractServer(t, DefaultLimits(), nil, nil)
	session, _ := server.Connect("invalid", nil)
	for _, invalid := range []string{
		`{"jsonrpc":"2.0","id":null,"method":"ping","params":{}}`,
		`{"jsonrpc":"2.0","id":1.5,"method":"ping","params":{}}`,
	} {
		response, _ := session.Dispatch(context.Background(), []byte(invalid))
		assertResponseCode(t, response, CodeInvalidRequest)
	}
	initializeSession(t, session)
	response, _ := session.Dispatch(context.Background(), request("tools/list", `"cursor"`, `{"cursor":"999"}`))
	assertResponseCode(t, response, CodeInvalidParams)
}

// #endregion 📏Bounds

// #region 📈ProgressAndReplay

func TestG2ProgressAndDroppedSink(t *testing.T) {
	var mu sync.Mutex
	var messages [][]byte
	sink := func(_ context.Context, payload []byte) error {
		mu.Lock()
		messages = append(messages, append([]byte(nil), payload...))
		mu.Unlock()
		return nil
	}
	server := newContractServer(t, DefaultLimits(), nil, sink)
	session, _ := server.Connect("progress", sink)
	initializeSession(t, session)
	response, _ := session.Dispatch(context.Background(), request("tools/call", `"progress"`, `{"name":"echo","arguments":{"text":"ok"},"_meta":{"progressToken":"p"}}`))
	assertResponseCode(t, response, 0)
	mu.Lock()
	defer mu.Unlock()
	if len(messages) != 1 || !bytes.Contains(messages[0], []byte(`"method":"notifications/progress"`)) || !bytes.Contains(messages[0], []byte(`"progressToken":"p"`)) {
		t.Fatalf("unexpected progress messages: %q", messages)
	}
}

func TestG2DroppedPeerRecoversOnReconnect(t *testing.T) {
	server := newContractServer(t, DefaultLimits(), nil, nil)
	dropped := func(context.Context, []byte) error { return errors.New("peer unavailable: private detail") }
	first, _ := server.Connect("recover", dropped)
	initializeSession(t, first)
	response, _ := first.Dispatch(context.Background(), request("tools/call", `"lost"`, `{"name":"echo","arguments":{"text":"lost"},"_meta":{"progressToken":"p"}}`))
	assertResponseCode(t, response, CodeRequestCancelled)
	select {
	case <-first.Done():
	case <-time.After(time.Second):
		t.Fatal("dropped session did not close")
	}
	second, err := server.Connect("recover", nil)
	if err != nil {
		t.Fatal(err)
	}
	if second.Generation() != first.Generation()+1 {
		t.Fatalf("generation got=%d want=%d", second.Generation(), first.Generation()+1)
	}
	initializeSession(t, second)
	response, _ = second.Dispatch(context.Background(), request("tools/call", `"live"`, `{"name":"echo","arguments":{"text":"live"}}`))
	assertResponseCode(t, response, 0)
}

func TestG2OwnedErrorsAndStructuralOutputBounds(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxNesting = 6
	server := newContractServer(t, limits, nil, nil)
	if err := server.RegisterTool(Tool{Name: "fault", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, errors.New("private provider detail")
	}); err != nil {
		t.Fatal(err)
	}
	if err := server.RegisterTool(Tool{Name: "deep", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{StructuredContent: json.RawMessage(`[[[[[[0]]]]]]`)}, nil
	}); err != nil {
		t.Fatal(err)
	}
	if err := server.RegisterTool(Tool{Name: "invalid_error", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, &HandlerError{Code: -32010, Message: "invalid data", Data: json.RawMessage(`not-json`)}
	}); err != nil {
		t.Fatal(err)
	}
	session, _ := server.Connect("output", nil)
	initializeSession(t, session)
	response, _ := session.Dispatch(context.Background(), request("tools/call", `"fault"`, `{"name":"fault","arguments":{}}`))
	assertResponseCode(t, response, CodeInternalError)
	if bytes.Contains(response, []byte("private")) {
		t.Fatalf("private handler error leaked: %s", response)
	}
	response, _ = session.Dispatch(context.Background(), request("tools/call", `"deep"`, `{"name":"deep","arguments":{}}`))
	assertResponseCode(t, response, CodePayloadTooLarge)
	response, _ = session.Dispatch(context.Background(), request("tools/call", `"null"`, `null`))
	assertResponseCode(t, response, CodeInvalidParams)
	response, _ = session.Dispatch(context.Background(), request("tools/call", `"invalid-error"`, `{"name":"invalid_error","arguments":{}}`))
	assertResponseCode(t, response, CodeInternalError)
}

func TestG2ErrorEnvelopeMaximumAndMaximumPlusOne(t *testing.T) {
	limits := DefaultLimits()
	limits.MaxPayloadBytes = 512
	limits.MaxNesting = 16
	server := newContractServer(t, limits, nil, nil)
	exactData := errorDataAtSize(t, json.RawMessage(`"exact-error"`), -32010, "bounded error", limits.MaxPayloadBytes)
	overData := errorDataAtSize(t, json.RawMessage(`"over-error"`), -32010, "bounded error", limits.MaxPayloadBytes+1)
	if err := server.RegisterTool(Tool{Name: "exact_error", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, &HandlerError{Code: -32010, Message: "bounded error", Data: exactData}
	}); err != nil {
		t.Fatal(err)
	}
	if err := server.RegisterTool(Tool{Name: "over_error", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, &HandlerError{Code: -32010, Message: "bounded error", Data: overData}
	}); err != nil {
		t.Fatal(err)
	}
	if err := server.RegisterTool(Tool{Name: "oversized_page", Description: strings.Repeat("x", limits.MaxPayloadBytes), InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		return CallToolResult{}, nil
	}); err != nil {
		t.Fatal(err)
	}
	session, _ := server.Connect("error-bounds", nil)
	initializeSession(t, session)
	response, err := session.Dispatch(context.Background(), request("tools/call", `"exact-error"`, `{"name":"exact_error","arguments":{}}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(response) != limits.MaxPayloadBytes {
		t.Fatalf("maximum error response bytes=%d want=%d", len(response), limits.MaxPayloadBytes)
	}
	assertResponseCode(t, response, -32010)
	response, err = session.Dispatch(context.Background(), request("tools/call", `"over-error"`, `{"name":"over_error","arguments":{}}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(response) > limits.MaxPayloadBytes {
		t.Fatalf("maximum-plus-one error escaped bound: %d", len(response))
	}
	assertResponseCode(t, response, CodePayloadTooLarge)
	response, err = session.Dispatch(context.Background(), request("tools/list", `"page-error"`, `{}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(response) > limits.MaxPayloadBytes {
		t.Fatalf("oversized page escaped bound: %d", len(response))
	}
	assertResponseCode(t, response, CodePayloadTooLarge)
	events := server.Events().Events()
	for index := 0; index < len(events); index++ {
		if events[index].Kind == "request.received" && (index+1 >= len(events) || events[index+1].Kind != "response.sent" || events[index].RequestID != events[index+1].RequestID) {
			t.Fatalf("partial bounded response event at %d: %#v", index, events)
		}
	}
}

func errorDataAtSize(t *testing.T, id json.RawMessage, code int, message string, target int) json.RawMessage {
	t.Helper()
	for size := 0; size <= target; size++ {
		data, err := json.Marshal(map[string]string{"padding": strings.Repeat("x", size)})
		if err != nil {
			t.Fatal(err)
		}
		encoded, err := json.Marshal(Response{JSONRPC: JSONRPCVersion, ID: id, Error: &RPCError{Code: code, Message: message, Data: data}})
		if err != nil {
			t.Fatal(err)
		}
		if len(encoded) == target {
			return data
		}
	}
	t.Fatalf("could not construct %d-byte error envelope", target)
	return nil
}

func TestG2DeterministicEncodingAndReplay(t *testing.T) {
	run := func() []byte {
		server := newContractServer(t, DefaultLimits(), nil, nil)
		session, _ := server.Connect("deterministic", nil)
		initializeSession(t, session)
		response, _ := session.Dispatch(context.Background(), request("tools/call", `"echo"`, `{"name":"echo","arguments":{"text":"same"}}`))
		assertResponseCode(t, response, 0)
		return server.Events().Snapshot()
	}
	first, second := run(), run()
	if !bytes.Equal(first, second) {
		t.Fatalf("event encoding is not deterministic\nfirst=%s\nsecond=%s", first, second)
	}
	events, err := ReplayEvents(context.Background(), first, len(first), 100)
	if err != nil || len(events) == 0 {
		t.Fatalf("replay events=%d err=%v", len(events), err)
	}
	corrupt := append([]byte(nil), first...)
	corrupt[len(corrupt)/2] ^= 1
	if _, err := ReplayEvents(context.Background(), corrupt, len(corrupt), 100); err == nil {
		t.Fatal("corrupt replay accepted")
	}
	log := NewEventLog(1024, 10)
	before := log.Snapshot()
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if err := log.Commit(cancelled, EventInput{Kind: "test", Peer: "p", Generation: 1, Payload: json.RawMessage(`{}`)}); !errors.Is(err, context.Canceled) {
		t.Fatalf("got %v, want context.Canceled", err)
	}
	if !bytes.Equal(before, log.Snapshot()) {
		t.Fatal("cancelled event commit changed log")
	}
	bounded := NewEventLog(1024, 1)
	inputs := []EventInput{
		{Kind: "request.received", Peer: "p", Generation: 1, Payload: json.RawMessage(`{}`)},
		{Kind: "response.sent", Peer: "p", Generation: 1, Payload: json.RawMessage(`{}`)},
	}
	if err := bounded.Commit(context.Background(), inputs...); !errors.Is(err, ErrLimit) {
		t.Fatalf("got %v, want ErrLimit", err)
	}
	if len(bounded.Snapshot()) != 0 {
		t.Fatal("maximum-plus-one event batch partially committed")
	}
}

// #endregion 📈ProgressAndReplay
