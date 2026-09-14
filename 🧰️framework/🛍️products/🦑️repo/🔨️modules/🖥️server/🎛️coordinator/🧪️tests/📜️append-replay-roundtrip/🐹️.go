// 🐹️ Go side of the append/replay case. Appends the fixture vectors into the case work directory
// and projects sequences, checksums, the log digest and the refusals.
package adapter

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"

	coordinator "github.com/usalu/semio/repo/coordinator"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type appendVector struct {
	Stream     string          `json:"stream"`
	ID         string          `json:"id"`
	Generation uint64          `json:"generation"`
	Type       string          `json:"type"`
	Payload    json.RawMessage `json:"payload"`
}

type appendVectors struct {
	Inputs    []appendVector `json:"inputs"`
	Sequences []uint64       `json:"sequences"`
}

func loadAppendVectors(ctx *host.Context) (appendVectors, []coordinator.EventInput, error) {
	var vectors appendVectors
	data, err := ctx.FixtureBytes("shared://📜️append-vectors.json")
	if err != nil {
		return vectors, nil, err
	}
	if err := json.Unmarshal(data, &vectors); err != nil {
		return vectors, nil, err
	}
	inputs := make([]coordinator.EventInput, len(vectors.Inputs))
	for index, vector := range vectors.Inputs {
		inputs[index] = coordinator.EventInput{Stream: vector.Stream, ID: vector.ID, Generation: vector.Generation, Type: vector.Type, Payload: vector.Payload}
	}
	return vectors, inputs, nil
}

// freshScenarioDir is an empty directory of its own for one scenario, because the harness reuses one
// work directory per (case, role, implementation) and never empties it.
func freshScenarioDir(ctx *host.Context, name string) (string, error) {
	dir := filepath.Join(ctx.WorkDir, name)
	if err := os.RemoveAll(dir); err != nil {
		return "", err
	}
	return dir, os.MkdirAll(dir, 0o755)
}

func openStore(dir string, name string) (*coordinator.EventStore, string, error) {
	path := filepath.Join(dir, name)
	store, err := coordinator.OpenEventStore(context.Background(), path, coordinator.DefaultStoreLimits())
	return store, path, err
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func appendThenReplayYieldsFrozenSequences(ctx *host.Context) (host.Outcome, error) {
	vectors, inputs, err := loadAppendVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "append-then-replay")
	if err != nil {
		return host.Outcome{}, err
	}
	first, firstPath, err := openStore(dir, "first.jsonl")
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := first.Append(context.Background(), 0, inputs, nil); err != nil {
		return host.Outcome{}, err
	}
	firstBytes, err := os.ReadFile(firstPath)
	if err != nil {
		return host.Outcome{}, err
	}
	replayed, err := first.Replay(context.Background(), nil)
	if err != nil {
		return host.Outcome{}, err
	}
	sequences := make([]uint64, 0, len(replayed))
	checksums := make([]string, 0, len(replayed))
	for _, event := range replayed {
		sequences = append(sequences, event.Sequence)
		checksums = append(checksums, event.Checksum)
	}
	second, secondPath, err := openStore(dir, "second.jsonl")
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := second.Append(context.Background(), 0, inputs, nil); err != nil {
		return host.Outcome{}, err
	}
	secondBytes, err := os.ReadFile(secondPath)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"sequences":     sequences,
		"frozen":        vectors.Sequences,
		"checksums":     checksums,
		"log":           string(firstBytes),
		"deterministic": string(firstBytes) == string(secondBytes),
	}}, nil
}

func duplicateIsIdempotentAndCorruptionIsRefused(ctx *host.Context) (host.Outcome, error) {
	_, inputs, err := loadAppendVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "duplicate-and-corrupt")
	if err != nil {
		return host.Outcome{}, err
	}
	store, path, err := openStore(dir, "refused.jsonl")
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := store.Append(context.Background(), 0, inputs[:1], nil); err != nil {
		return host.Outcome{}, err
	}
	before, err := os.ReadFile(path)
	if err != nil {
		return host.Outcome{}, err
	}
	duplicate, duplicateErr := store.Append(context.Background(), 1, inputs[:1], nil)
	afterDuplicate, err := os.ReadFile(path)
	if err != nil {
		return host.Outcome{}, err
	}
	_, staleErr := store.Append(context.Background(), 0, inputs[1:2], nil)
	corrupt := append([]byte(nil), before...)
	corrupt[len(corrupt)/2] ^= 1
	if err := os.WriteFile(path, corrupt, 0o644); err != nil {
		return host.Outcome{}, err
	}
	_, corruptErr := store.Replay(context.Background(), nil)
	return host.Outcome{Projection: map[string]any{
		"duplicateReported": duplicate.Duplicate && duplicateErr == nil,
		"logUnchanged":      string(before) == string(afterDuplicate),
		"staleRefused":      errors.Is(staleErr, coordinator.ErrSequenceConflict),
		"corruptDetected":   errors.Is(corruptErr, coordinator.ErrStoreCorrupt),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("append-then-replay-yields-frozen-sequences", appendThenReplayYieldsFrozenSequences).
		Subject("duplicate-is-idempotent-and-corruption-is-refused", duplicateIsIdempotentAndCorruptionIsRefused)
}

// endregion 🔖️Registration
