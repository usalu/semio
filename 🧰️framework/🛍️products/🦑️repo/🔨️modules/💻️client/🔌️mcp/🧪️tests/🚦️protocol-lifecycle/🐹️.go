// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol lifecycle and pagination contract tests.

// #endregion 🧲️Header

package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"testing"
)

// #region 🚦️ProtocolLifecycle

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

// #endregion 🚦️ProtocolLifecycle
