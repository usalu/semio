// 🐹️ Go side of the query-execution case. It runs the owner's own executor over the frozen
// repository records and emits the `data` payload of every query — nothing else.
package adapter

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

type corpus struct {
	Queries []struct {
		ID        string                 `json:"id"`
		Source    string                 `json:"source"`
		Variables map[string]interface{} `json:"variables"`
	} `json:"queries"`
}

// 🗄️ Builds the context every query of the corpus runs against.
func newContext(ctx *host.Context) (*graphql.RecordingContext, error) {
	raw, err := ctx.FixtureBytes("shared://🔣️repo-records.json")
	if err != nil {
		return nil, err
	}
	return graphql.NewRecordingContext(raw)
}

// 📥️ Reads the query corpus.
func readCorpus(ctx *host.Context) (corpus, error) {
	raw, err := ctx.FixtureBytes("shared://▶️query-execution/🔣️queries.json")
	if err != nil {
		return corpus{}, err
	}
	var parsed corpus
	return parsed, json.Unmarshal(raw, &parsed)
}

// ⚙️ Runs every query whose entry the predicate accepts.
func run(ctx *host.Context, keep func(source string, variables map[string]interface{}) bool) (host.Outcome, error) {
	repo, err := newContext(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	parsed, err := readCorpus(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	executor, err := graphql.NewExecutorWithContext(repo.GetRootDir(), repo)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Queries))
	for _, entry := range parsed.Queries {
		if !keep(entry.Source, entry.Variables) {
			continue
		}
		data, err := executor.Execute(context.Background(), entry.Source, entry.Variables)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: %w", entry.ID, err)
		}
		rows = append(rows, map[string]any{"id": entry.ID, "data": data})
	}
	return host.Outcome{Projection: map[string]any{"queries": rows}}, nil
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func corpusExecutesIdentically(ctx *host.Context) (host.Outcome, error) {
	return run(ctx, func(string, map[string]interface{}) bool { return true })
}

func argumentsCoerceBeforeResolution(ctx *host.Context) (host.Outcome, error) {
	return run(ctx, func(source string, variables map[string]interface{}) bool {
		return variables != nil || strings.Contains(source, "(")
	})
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("corpus-executes-identically", corpusExecutesIdentically).
		Subject("arguments-coerce-before-resolution", argumentsCoerceBeforeResolution)
}

// endregion 🔖️Registration
