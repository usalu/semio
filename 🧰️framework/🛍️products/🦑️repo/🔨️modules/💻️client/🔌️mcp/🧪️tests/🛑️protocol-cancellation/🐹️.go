// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol cancellation, pipe saturation and reconnect contract tests.

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
	"testing"
	"time"
)

// #region 🛑️ProtocolCancellation

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

// #endregion 🛑️ProtocolCancellation
