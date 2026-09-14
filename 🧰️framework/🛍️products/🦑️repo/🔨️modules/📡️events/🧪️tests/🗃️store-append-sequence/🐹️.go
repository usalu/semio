// 🐹️ Go side of the append-only store case. Appends the fixture vectors into the case work
// directory and projects sequences, file bytes, checksums and the outcome of every interruption.
package adapter

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"

	events "github.com/usalu/semio/repo/events"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type storeVectors struct {
	Inputs          []events.Input `json:"inputs"`
	Sequences       []uint64       `json:"sequences"`
	InterruptPhases []string       `json:"interruptPhases"`
}

func loadStoreVectors(ctx *host.Context) (storeVectors, error) {
	var vectors storeVectors
	data, err := ctx.FixtureBytes("shared://🗄️store-vectors.json")
	if err != nil {
		return vectors, err
	}
	return vectors, json.Unmarshal(data, &vectors)
}

// freshScenarioDir is an empty directory of its own for one scenario.
//
// The harness reuses one work directory per (case, role, implementation) and never empties it, so a
// log file left by an earlier scenario — or by an earlier run of this same case — is still there when
// the next append starts, and the store rightly refuses it as a duplicate. Every scenario that writes
// a log therefore owns a directory it clears first.
func freshScenarioDir(ctx *host.Context, name string) (string, error) {
	dir := filepath.Join(ctx.WorkDir, name)
	if err := os.RemoveAll(dir); err != nil {
		return "", err
	}
	return dir, os.MkdirAll(dir, 0o755)
}

type phaseCancel struct {
	phase  string
	cancel context.CancelFunc
}

func (p phaseCancel) report(progress events.Progress) {
	if progress.Step == p.phase {
		p.cancel()
	}
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func appendThenReplaySequences(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadStoreVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "append-then-replay-sequences")
	if err != nil {
		return host.Outcome{}, err
	}
	first := events.Store{Path: filepath.Join(dir, "first.jsonl")}
	if _, err := first.Append(context.Background(), vectors.Inputs, nil); err != nil {
		return host.Outcome{}, err
	}
	firstBytes, err := os.ReadFile(first.Path)
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
	second := events.Store{Path: filepath.Join(dir, "second.jsonl")}
	if _, err := second.Append(context.Background(), vectors.Inputs, nil); err != nil {
		return host.Outcome{}, err
	}
	secondBytes, err := os.ReadFile(second.Path)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"sequences":     sequences,
		"checksums":     checksums,
		"logDigest":     events.Digest(firstBytes),
		"deterministic": string(firstBytes) == string(secondBytes),
	}}, nil
}

func duplicateAndCorruptAreRefused(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadStoreVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "duplicate-and-corrupt-are-refused")
	if err != nil {
		return host.Outcome{}, err
	}
	store := events.Store{Path: filepath.Join(dir, "refused.jsonl")}
	if _, err := store.Append(context.Background(), vectors.Inputs[:1], nil); err != nil {
		return host.Outcome{}, err
	}
	before, err := os.ReadFile(store.Path)
	if err != nil {
		return host.Outcome{}, err
	}
	_, duplicateErr := store.Append(context.Background(), vectors.Inputs[:1], nil)
	corrupt := append([]byte(nil), before...)
	corrupt[len(corrupt)/2] ^= 1
	if err := os.WriteFile(store.Path, corrupt, 0o644); err != nil {
		return host.Outcome{}, err
	}
	_, corruptErr := store.Replay(context.Background(), nil)
	return host.Outcome{Projection: map[string]any{
		"duplicateRefused": errors.Is(duplicateErr, events.ErrDuplicate),
		"corruptDetected":  errors.Is(corruptErr, events.ErrCorrupt),
	}}, nil
}

func interruptionPreservesCommittedLog(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadStoreVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "interruption-preserves-committed-log")
	if err != nil {
		return host.Outcome{}, err
	}
	phases := make([]map[string]any, 0, len(vectors.InterruptPhases))
	for _, phase := range vectors.InterruptPhases {
		path := filepath.Join(dir, phase+".jsonl")
		store := events.Store{Path: path}
		if _, err := store.Append(context.Background(), vectors.Inputs[:1], nil); err != nil {
			return host.Outcome{}, err
		}
		before, err := os.ReadFile(path)
		if err != nil {
			return host.Outcome{}, err
		}
		cancelCtx, cancel := context.WithCancel(context.Background())
		watcher := phaseCancel{phase: phase, cancel: cancel}
		_, appendErr := store.Append(cancelCtx, vectors.Inputs[1:], watcher.report)
		after, err := os.ReadFile(path)
		if err != nil {
			return host.Outcome{}, err
		}
		_, stageErr := os.Stat(path + ".stage")
		replayed, replayErr := store.Replay(context.Background(), nil)
		phases = append(phases, map[string]any{
			"phase":       phase,
			"cancelled":   errors.Is(appendErr, context.Canceled),
			"unchanged":   string(before) == string(after),
			"stageGone":   os.IsNotExist(stageErr),
			"replayCount": len(replayed),
			"replayOk":    replayErr == nil,
		})
	}
	return host.Outcome{Projection: map[string]any{"phases": phases}}, nil
}

func recordChecksumIsSha256(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadStoreVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := freshScenarioDir(ctx, "record-checksum-is-sha256")
	if err != nil {
		return host.Outcome{}, err
	}
	store := events.Store{Path: filepath.Join(dir, "checksum.jsonl")}
	appended, err := store.Append(context.Background(), vectors.Inputs, nil)
	if err != nil {
		return host.Outcome{}, err
	}
	records := make([]map[string]any, 0, len(appended))
	for _, event := range appended {
		records = append(records, map[string]any{
			"id":       event.ID,
			"kind":     event.Kind,
			"sequence": event.Sequence,
			"data":     string(event.Data),
			"checksum": event.Checksum,
		})
	}
	return host.Outcome{Projection: map[string]any{"records": records}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("append-then-replay-sequences", appendThenReplaySequences).
		Subject("duplicate-and-corrupt-are-refused", duplicateAndCorruptAreRefused).
		Subject("interruption-preserves-committed-log", interruptionPreservesCommittedLog).
		Subject("record-checksum-is-sha256", recordChecksumIsSha256)
}

// endregion 🔖️Registration
