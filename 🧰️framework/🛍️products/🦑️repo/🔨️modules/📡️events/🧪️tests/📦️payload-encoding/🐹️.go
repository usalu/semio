// 🐹️ Go side of the payload encoding case. Decodes every fixture input into its typed payload and
// projects the re-encoded JSON, so field order and omit-empty semantics are compared, not described.
package adapter

import (
	"bytes"
	"encoding/json"
	"fmt"

	events "github.com/usalu/semio/repo/events"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type payloadVectors struct {
	Cases []struct {
		ID      string          `json:"id"`
		Type    string          `json:"type"`
		Input   json.RawMessage `json:"input"`
		Encoded string          `json:"encoded"`
	} `json:"cases"`
}

func newPayload(name string) any {
	switch name {
	case "TicketPayload":
		return &events.TicketPayload{}
	case "TicketOpenPayload":
		return &events.TicketOpenPayload{}
	case "TicketClosePayload":
		return &events.TicketClosePayload{}
	case "TicketReopenPayload":
		return &events.TicketReopenPayload{}
	case "TicketChangePayload":
		return &events.TicketChangePayload{}
	case "GoalPayload":
		return &events.GoalPayload{}
	case "GoalOpenPayload":
		return &events.GoalOpenPayload{}
	case "GoalClosePayload":
		return &events.GoalClosePayload{}
	case "GoalReopenPayload":
		return &events.GoalReopenPayload{}
	case "GoalChangePayload":
		return &events.GoalChangePayload{}
	case "ContributorPayload":
		return &events.ContributorPayload{}
	case "CheckpointPayload":
		return &events.CheckpointPayload{}
	case "TodoPayload":
		return &events.TodoPayload{}
	case "TodoCreatePayload":
		return &events.TodoCreatePayload{}
	case "TodoChangePayload":
		return &events.TodoChangePayload{}
	case "TodoDeletePayload":
		return &events.TodoDeletePayload{}
	case "WorkItem":
		return &events.WorkItem{}
	case "ContributorWork":
		return &events.ContributorWork{}
	case "DraftPayload":
		return &events.DraftPayload{}
	case "FilePayload":
		return &events.FilePayload{}
	case "FolderPayload":
		return &events.FolderPayload{}
	case "SectionPayload":
		return &events.SectionPayload{}
	case "IntegratePayload":
		return &events.IntegratePayload{}
	case "ExtractPayload":
		return &events.ExtractPayload{}
	}
	return nil
}

func loadVectors(ctx *host.Context) (payloadVectors, error) {
	var vectors payloadVectors
	data, err := ctx.FixtureBytes("shared://✉️payload-vectors.json")
	if err != nil {
		return vectors, err
	}
	return vectors, json.Unmarshal(data, &vectors)
}

func encodeAll(ctx *host.Context) ([]map[string]any, error) {
	vectors, err := loadVectors(ctx)
	if err != nil {
		return nil, err
	}
	encodings := make([]map[string]any, 0, len(vectors.Cases))
	for _, testCase := range vectors.Cases {
		value := newPayload(testCase.Type)
		if value == nil {
			return nil, fmt.Errorf("unknown payload type %s", testCase.Type)
		}
		decoder := json.NewDecoder(bytes.NewReader(testCase.Input))
		decoder.DisallowUnknownFields()
		if err := decoder.Decode(value); err != nil {
			return nil, fmt.Errorf("%s: %w", testCase.ID, err)
		}
		encoded, err := json.Marshal(value)
		if err != nil {
			return nil, fmt.Errorf("%s: %w", testCase.ID, err)
		}
		encodings = append(encodings, map[string]any{"id": testCase.ID, "type": testCase.Type, "encoded": string(encoded)})
	}
	return encodings, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func goldenEncodingPerPayload(ctx *host.Context) (host.Outcome, error) {
	encodings, err := encodeAll(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"encodings": encodings, "count": len(encodings)}}, nil
}

func omitEmptyAndExplicitNull(ctx *host.Context) (host.Outcome, error) {
	encodings, err := encodeAll(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	optional := make([]map[string]any, 0)
	for _, encoding := range encodings {
		encoded, _ := encoding["encoded"].(string)
		optional = append(optional, map[string]any{
			"id":            encoding["id"],
			"hasEmptyValue": bytes.Contains([]byte(encoded), []byte(`:""`)),
			"hasNullValue":  bytes.Contains([]byte(encoded), []byte(`:null`)),
		})
	}
	return host.Outcome{Projection: map[string]any{"optional": optional}}, nil
}

func envelopeRoundTrip(ctx *host.Context) (host.Outcome, error) {
	_ = ctx
	envelope := events.Event{
		Kind:    events.EventTicketOpenStarting,
		Source:  "repo-cli",
		Payload: json.RawMessage(`{"id":"a"}`),
	}
	encoded, err := json.Marshal(envelope)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"envelope": string(encoded)}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("golden-encoding-per-payload", goldenEncodingPerPayload).
		Subject("omit-empty-and-explicit-null", omitEmptyAndExplicitNull).
		Subject("envelope-round-trip", envelopeRoundTrip)
}

// endregion 🔖️Registration
