// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol contract harness: controls, test repositories, the production pipe, server construction and assertion helpers.

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
	"sync/atomic"
	"testing"
	"time"
)

// #region 🧰️ProtocolHarness

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

// #endregion 🧰️ProtocolHarness
