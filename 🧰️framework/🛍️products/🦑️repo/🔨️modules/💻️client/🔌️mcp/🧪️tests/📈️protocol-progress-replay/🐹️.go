// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol progress, dropped peer, error envelope and deterministic replay contract tests.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"strings"
	"sync"
	"testing"
	"time"
)

// #region 📈️ProtocolProgressReplay

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

// #endregion 📈️ProtocolProgressReplay
