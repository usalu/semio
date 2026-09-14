// 🐹️ Go subject for the golden call vectors. Dispatches each `2️⃣g2-contract.json` request against a
// session of the contract server and projects the parsed result, never the raw bytes, so a reference
// implementation with different whitespace and escaping can project the same thing.
package adapter

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	mcp "github.com/usalu/semio/repo/mcp"
	host "semio.tech/repo/test"
)

// region 🔖️Support

type callVector struct {
	Name    string `json:"name"`
	Request string `json:"request"`
	Ready   bool   `json:"ready"`
}

// 🧫️vectors reads the golden call vectors the whole case is stated against.
func vectors(ctx *host.Context) ([]callVector, error) {
	raw, err := ctx.FixtureBytes("shared://2️⃣g2-contract.json")
	if err != nil {
		return nil, err
	}
	var fixture struct {
		Vectors []callVector `json:"vectors"`
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return nil, err
	}
	return fixture.Vectors, nil
}

// 📞️dispatch answers one vector on a fresh contract server and returns the parsed response envelope.
func dispatch(request string, ready bool) (map[string]json.RawMessage, error) {
	server, err := mcp.NewContractServer(mcp.DefaultLimits())
	if err != nil {
		return nil, err
	}
	session, err := server.Connect("fixture", nil)
	if err != nil {
		return nil, err
	}
	if ready {
		initialize := fmt.Sprintf(`{"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"test","version":"1"}}}`, mcp.ProtocolVersion)
		if _, err := session.Dispatch(context.Background(), []byte(initialize)); err != nil {
			return nil, err
		}
		if _, err := session.Dispatch(context.Background(), []byte(`{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`)); err != nil {
			return nil, err
		}
	}
	response, err := session.Dispatch(context.Background(), []byte(request))
	if err != nil {
		return nil, err
	}
	if len(response) == 0 {
		return nil, errors.New("no response")
	}
	var envelope map[string]json.RawMessage
	if err := json.Unmarshal(response, &envelope); err != nil {
		return nil, err
	}
	return envelope, nil
}

// endregion 🔖️Support

// region 🔖️Scenarios

func toolResourcePromptRoundtrip(ctx *host.Context) (host.Outcome, error) {
	all, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]map[string]any, 0, 3)
	for _, vector := range all {
		if vector.Name != "tool" && vector.Name != "resource" && vector.Name != "prompt" {
			continue
		}
		envelope, err := dispatch(vector.Request, vector.Ready)
		if err != nil {
			return host.Outcome{}, err
		}
		var result any
		if raw, ok := envelope["result"]; ok {
			if err := json.Unmarshal(raw, &result); err != nil {
				return host.Outcome{}, err
			}
		}
		rows = append(rows, map[string]any{"name": vector.Name, "result": result})
	}
	return host.Outcome{Projection: map[string]any{"vectors": rows}}, nil
}

func protocolErrorVectors(ctx *host.Context) (host.Outcome, error) {
	all, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]map[string]any, 0, 1)
	for _, vector := range all {
		if vector.Name != "unknown-method" {
			continue
		}
		envelope, err := dispatch(vector.Request, vector.Ready)
		if err != nil {
			return host.Outcome{}, err
		}
		var failure struct {
			Code float64 `json:"code"`
		}
		if raw, ok := envelope["error"]; ok {
			if err := json.Unmarshal(raw, &failure); err != nil {
				return host.Outcome{}, err
			}
		}
		rows = append(rows, map[string]any{"name": vector.Name, "code": failure.Code})
	}
	return host.Outcome{Projection: map[string]any{"vectors": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("tool-resource-prompt-roundtrip", toolResourcePromptRoundtrip).
		Subject("protocol-error-vectors", protocolErrorVectors)
}

// endregion 🔖️Registration
