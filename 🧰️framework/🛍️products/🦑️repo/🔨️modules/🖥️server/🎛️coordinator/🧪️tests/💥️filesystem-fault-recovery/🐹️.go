// 🐹️ Go side of the filesystem-fault case. Replays the fixture's failure indices against a real log
// and projects, per index, whether the append was refused and the committed prefix survived.
package adapter

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"

	coordinator "github.com/usalu/semio/repo/coordinator"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type faultVector struct {
	Stream     string          `json:"stream"`
	ID         string          `json:"id"`
	Generation uint64          `json:"generation"`
	Type       string          `json:"type"`
	Payload    json.RawMessage `json:"payload"`
}

type faultPlan struct {
	Seed   faultVector `json:"seed"`
	Second faultVector `json:"second"`
	FailAt []int       `json:"failAt"`
}

func (vector faultVector) input() coordinator.EventInput {
	return coordinator.EventInput{Stream: vector.Stream, ID: vector.ID, Generation: vector.Generation, Type: vector.Type, Payload: vector.Payload}
}

func loadFaultPlan(ctx *host.Context) (faultPlan, error) {
	var plan faultPlan
	data, err := ctx.FixtureBytes("local://💥️fault-plan.json")
	if err != nil {
		return plan, err
	}
	return plan, json.Unmarshal(data, &plan)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyInjectedFaultPreservesTheCommittedLog(ctx *host.Context) (host.Outcome, error) {
	plan, err := loadFaultPlan(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	root := filepath.Join(ctx.WorkDir, "filesystem-fault-recovery")
	if err := os.RemoveAll(root); err != nil {
		return host.Outcome{}, err
	}
	if err := os.MkdirAll(root, 0o755); err != nil {
		return host.Outcome{}, err
	}
	attempts := make([]map[string]any, 0, len(plan.FailAt))
	for _, index := range plan.FailAt {
		path := filepath.Join(root, "fault.jsonl")
		if err := os.RemoveAll(path); err != nil {
			return host.Outcome{}, err
		}
		store, err := coordinator.OpenEventStore(context.Background(), path, coordinator.DefaultStoreLimits())
		if err != nil {
			return host.Outcome{}, err
		}
		if _, err := store.Append(context.Background(), 0, []coordinator.EventInput{plan.Seed.input()}, nil); err != nil {
			return host.Outcome{}, err
		}
		before, err := os.ReadFile(path)
		if err != nil {
			return host.Outcome{}, err
		}
		store.ArmFault(index)
		result, appendErr := store.Append(context.Background(), 1, []coordinator.EventInput{plan.Second.input()}, nil)
		store.ArmFault(0)
		after, err := os.ReadFile(path)
		if err != nil {
			return host.Outcome{}, err
		}
		reopened, reopenErr := coordinator.OpenEventStore(context.Background(), path, coordinator.DefaultStoreLimits())
		replayed := 0
		replayOk := reopenErr == nil
		if reopenErr == nil {
			events, replayErr := reopened.Replay(context.Background(), nil)
			replayOk = replayErr == nil
			replayed = len(events)
		}
		attempts = append(attempts, map[string]any{
			"failAt":       index,
			"refused":      appendErr != nil,
			"committed":    result.Committed,
			"logUnchanged": string(before) == string(after),
			"reopened":     reopenErr == nil,
			"replayOk":     replayOk,
			"replayCount":  replayed,
			"artifacts":    recoveryArtifacts(path),
		})
	}
	return host.Outcome{Projection: map[string]any{"attempts": attempts}}, nil
}

func recoveryArtifacts(path string) []string {
	present := []string{}
	for _, suffix := range []string{".stage", ".stage.next", ".next", ".backup"} {
		if _, err := os.Lstat(path + suffix); err == nil {
			present = append(present, suffix)
		}
	}
	return present
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-injected-fault-preserves-the-committed-log", everyInjectedFaultPreservesTheCommittedLog)
}

// endregion 🔖️Registration
