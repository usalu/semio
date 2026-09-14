// 🐹️ Go subject for the hash-chained event log. Commits the fixture's inputs through the owned log
// and projects the digests and the JSONL rendering the chain produces.
package adapter

import (
	"context"
	"encoding/json"
	"strings"

	mcp "github.com/usalu/semio/repo/mcp"
	host "semio.tech/repo/test"
)

// region 🔖️Support

type chainInput struct {
	Kind       string `json:"kind"`
	Peer       string `json:"peer"`
	Generation uint64 `json:"generation"`
	RequestID  string `json:"requestId"`
	Payload    string `json:"payload"`
}

type chainFixture struct {
	Inputs []chainInput `json:"inputs"`
	JSONL  string       `json:"jsonl"`
}

// 🧫️document reads the golden chain the whole case is stated against.
func document(ctx *host.Context) (chainFixture, error) {
	raw, err := ctx.FixtureBytes("shared://🔗️event-chain.json")
	if err != nil {
		return chainFixture{}, err
	}
	var fixture chainFixture
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return chainFixture{}, err
	}
	return fixture, nil
}

// 🔗️chain rebuilds the whole chain from the fixture's declared inputs.
func chain(fixture chainFixture) ([]mcp.Event, []byte, error) {
	log := mcp.NewEventLog(0, 0)
	inputs := make([]mcp.EventInput, 0, len(fixture.Inputs))
	for _, input := range fixture.Inputs {
		inputs = append(inputs, mcp.EventInput{Kind: input.Kind, Peer: input.Peer, Generation: input.Generation, RequestID: input.RequestID, Payload: json.RawMessage(input.Payload)})
	}
	if err := log.Commit(context.Background(), inputs...); err != nil {
		return nil, nil, err
	}
	return log.Events(), log.Snapshot(), nil
}

// endregion 🔖️Support

// region 🔖️Scenarios

func chainDigestsMatchTheGolden(ctx *host.Context) (host.Outcome, error) {
	fixture, err := document(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	events, jsonl, err := chain(fixture)
	if err != nil {
		return host.Outcome{}, err
	}
	hashes := make([]string, 0, len(events))
	for _, event := range events {
		hashes = append(hashes, event.Hash)
	}
	return host.Outcome{Projection: map[string]any{"hashes": hashes, "jsonl": string(jsonl)}}, nil
}

func aTamperedChainIsRefused(ctx *host.Context) (host.Outcome, error) {
	fixture, err := document(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	_, intact := mcp.ReplayEvents(context.Background(), []byte(fixture.JSONL), 0, 0)
	_, tampered := mcp.ReplayEvents(context.Background(), []byte(strings.Replace(fixture.JSONL, "session.opened", "session.tampered", 1)), 0, 0)
	return host.Outcome{Projection: map[string]any{"intactAccepted": intact == nil, "tamperedRefused": tampered != nil}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("chain-digests-match-the-golden", chainDigestsMatchTheGolden).
		Subject("a-tampered-chain-is-refused", aTamperedChainIsRefused)
}

// endregion 🔖️Registration
