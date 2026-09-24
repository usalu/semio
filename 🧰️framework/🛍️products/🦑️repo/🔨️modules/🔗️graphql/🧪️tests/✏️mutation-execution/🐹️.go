// 🐹️ Go side of the mutation-execution case. It runs the whole write script against one recording
// context and reports the payloads, the resulting records and the event trail together.
package adapter

import (
	"context"
	"encoding/json"
	"fmt"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

type script struct {
	Mutations []struct {
		ID        string                 `json:"id"`
		Source    string                 `json:"source"`
		Variables map[string]interface{} `json:"variables"`
	} `json:"mutations"`
}

// 🗄️ Builds the context the whole script runs against.
func newContext(ctx *host.Context) (*graphql.RecordingContext, error) {
	raw, err := ctx.FixtureBytes("shared://🔣️repo-records.json")
	if err != nil {
		return nil, err
	}
	return graphql.NewRecordingContext(raw)
}

// 📥️ Reads the write script.
func readScript(ctx *host.Context) (script, error) {
	raw, err := ctx.FixtureBytes("shared://✏️mutation-execution/🔣️mutations.json")
	if err != nil {
		return script{}, err
	}
	var parsed script
	return parsed, json.Unmarshal(raw, &parsed)
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func scriptChangesRecordsAndEmitsEvents(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readScript(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	repo, err := newContext(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	executor, err := graphql.NewExecutorWithContext(repo.GetRootDir(), repo)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Mutations))
	for _, entry := range parsed.Mutations {
		data, err := executor.Execute(context.Background(), entry.Source, entry.Variables)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: %w", entry.ID, err)
		}
		rows = append(rows, map[string]any{"id": entry.ID, "data": data})
	}
	return host.Outcome{Projection: map[string]any{
		"mutations": rows,
		"events":    repo.Events(),
		"records":   repo.Snapshot(),
	}}, nil
}

func aMutationThatCannotApplyIsRefused(ctx *host.Context) (host.Outcome, error) {
	repo, err := newContext(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	executor, err := graphql.NewExecutorWithContext(repo.GetRootDir(), repo)
	if err != nil {
		return host.Outcome{}, err
	}
	source := `mutation { ticketClose(input: { year: 2026, month: 9, day: 6, slug: "NO-SUCH-TICKET", summary: "never" }) { status } }`
	if _, err := executor.Execute(context.Background(), source, nil); err == nil {
		return host.Outcome{}, fmt.Errorf("a ticket that does not exist cannot be closed")
	} else {
		return host.Outcome{Projection: map[string]any{"message": err.Error(), "events": repo.Events()}}, nil
	}
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("script-changes-records-and-emits-events", scriptChangesRecordsAndEmitsEvents).
		Subject("a-mutation-that-cannot-apply-is-refused", aMutationThatCannotApplyIsRefused)
}

// endregion 🔖️Registration
