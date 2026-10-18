// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol payload, nesting, identifier and cursor bound contract tests.

// #endregion 🧲️Header

package main

import (
	"context"
	"strings"
	"testing"
)

// #region 📏️ProtocolBounds

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

// #endregion 📏️ProtocolBounds
