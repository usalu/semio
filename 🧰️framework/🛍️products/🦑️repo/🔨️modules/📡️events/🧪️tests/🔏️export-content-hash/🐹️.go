// 🐹️ Go side of the export content-hash case. Builds a snapshot through the ExportSource port and
// projects its identity, its namespaced input ids and the refusal of an unchanged re-export.
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

type exportVectors struct {
	Entities []struct {
		Kind  string            `json:"kind"`
		ID    string            `json:"id"`
		Value map[string]string `json:"value"`
	} `json:"entities"`
	SortedIds []string `json:"sortedIds"`
	Snapshot  string   `json:"snapshot"`
	InputIds  []string `json:"inputIds"`
}

type fixtureSource struct{ entities []events.ExportEntity }

func (source fixtureSource) ExportEntities() []events.ExportEntity { return source.entities }

func loadSource(ctx *host.Context) (fixtureSource, exportVectors, error) {
	var vectors exportVectors
	data, err := ctx.FixtureBytes("shared://📤️export-vectors.json")
	if err != nil {
		return fixtureSource{}, vectors, err
	}
	if err := json.Unmarshal(data, &vectors); err != nil {
		return fixtureSource{}, vectors, err
	}
	entities := make([]events.ExportEntity, 0, len(vectors.Entities))
	for _, entity := range vectors.Entities {
		entities = append(entities, events.ExportEntity{Kind: entity.Kind, ID: entity.ID, Value: entity.Value})
	}
	return fixtureSource{entities: entities}, vectors, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func snapshotIdentity(ctx *host.Context) (host.Outcome, error) {
	source, _, err := loadSource(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	snapshot, err := events.BuildExportSnapshot(context.Background(), source.ExportEntities())
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"snapshot": snapshot.Snapshot,
		"counts":   snapshot.Counts,
	}}, nil
}

func inputIdsAreNamespaced(ctx *host.Context) (host.Outcome, error) {
	source, _, err := loadSource(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	snapshot, err := events.BuildExportSnapshot(context.Background(), source.ExportEntities())
	if err != nil {
		return host.Outcome{}, err
	}
	ids := make([]string, 0, len(snapshot.Inputs))
	for _, input := range snapshot.Inputs {
		ids = append(ids, input.ID)
	}
	return host.Outcome{Projection: map[string]any{"inputIds": ids}}, nil
}

func unchangedExportIsRefused(ctx *host.Context) (host.Outcome, error) {
	source, _, err := loadSource(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	snapshot, err := events.BuildExportSnapshot(context.Background(), source.ExportEntities())
	if err != nil {
		return host.Outcome{}, err
	}
	// The harness reuses one work directory per (case, role, implementation) and never empties it, so a
	// log left by an earlier run would make the FIRST append the duplicate. The scenario owns a
	// directory it clears first.
	dir := filepath.Join(ctx.WorkDir, "unchanged-export-is-refused")
	if err := os.RemoveAll(dir); err != nil {
		return host.Outcome{}, err
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return host.Outcome{}, err
	}
	store := events.Store{Path: filepath.Join(dir, "export.events.jsonl")}
	if _, err := store.Append(context.Background(), snapshot.Inputs, nil); err != nil {
		return host.Outcome{}, err
	}
	before, err := os.ReadFile(store.Path)
	if err != nil {
		return host.Outcome{}, err
	}
	_, duplicateErr := store.Append(context.Background(), snapshot.Inputs, nil)
	after, err := os.ReadFile(store.Path)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"duplicateRefused": errors.Is(duplicateErr, events.ErrDuplicate),
		"logUnchanged":     string(before) == string(after),
		"eventCount":       len(snapshot.Inputs),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("snapshot-identity", snapshotIdentity).
		Subject("input-ids-are-namespaced", inputIdsAreNamespaced).
		Subject("unchanged-export-is-refused", unchangedExportIsRefused)
}

// endregion 🔖️Registration
