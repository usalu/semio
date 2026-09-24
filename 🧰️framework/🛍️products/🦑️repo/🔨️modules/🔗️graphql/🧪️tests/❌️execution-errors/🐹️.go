// 🐹️ Go side of the execution-errors case. Every input of the corpus MUST be refused with the
// verbatim message the corpus records, and a refused mutation MUST leave no trace behind it.
package adapter

import (
	"context"
	"encoding/json"
	"fmt"
	"reflect"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

// ❌️ One recorded refusal: an input, the message it owes, and whether it is a write.
type refusal struct {
	ID       string `json:"id"`
	Source   string `json:"source"`
	Message  string `json:"message"`
	Mutation bool   `json:"mutation"`
}

// 🗄️ Builds the context every input runs against.
func newContext(ctx *host.Context) (*graphql.RecordingContext, error) {
	raw, err := ctx.FixtureBytes("shared://🔣️repo-records.json")
	if err != nil {
		return nil, err
	}
	return graphql.NewRecordingContext(raw)
}

// 📥️ Reads the refusal corpus.
func readCorpus(ctx *host.Context) ([]refusal, error) {
	raw, err := ctx.FixtureBytes("shared://❌️execution-errors/🔣️refusals.json")
	if err != nil {
		return nil, err
	}
	var parsed struct {
		Refusals []refusal `json:"refusals"`
	}
	return parsed.Refusals, json.Unmarshal(raw, &parsed)
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func everyRefusalCarriesItsVerbatimMessage(ctx *host.Context) (host.Outcome, error) {
	repo, err := newContext(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	corpus, err := readCorpus(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	executor, err := graphql.NewExecutorWithContext(repo.GetRootDir(), repo)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(corpus))
	for _, entry := range corpus {
		data, failure := executor.Execute(context.Background(), entry.Source, nil)
		if failure == nil {
			return host.Outcome{}, fmt.Errorf("%s: answered instead of refusing — %v", entry.ID, data)
		}
		if failure.Error() != entry.Message {
			return host.Outcome{}, fmt.Errorf("%s: expected %q, got %q", entry.ID, entry.Message, failure.Error())
		}
		rows = append(rows, map[string]any{"input": entry.ID, "message": failure.Error()})
	}
	return host.Outcome{Projection: map[string]any{"refusals": rows}}, nil
}

func aRefusalWritesNoEventAndChangesNoRecord(ctx *host.Context) (host.Outcome, error) {
	repo, err := newContext(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	corpus, err := readCorpus(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	before := repo.Snapshot()
	executor, err := graphql.NewExecutorWithContext(repo.GetRootDir(), repo)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(corpus))
	for _, entry := range corpus {
		if !entry.Mutation {
			continue
		}
		data, failure := executor.Execute(context.Background(), entry.Source, nil)
		if failure == nil {
			return host.Outcome{}, fmt.Errorf("%s: answered instead of refusing — %v", entry.ID, data)
		}
		rows = append(rows, map[string]any{"input": entry.ID, "message": failure.Error()})
	}
	after := repo.Snapshot()
	if !reflect.DeepEqual(before, after) {
		return host.Outcome{}, fmt.Errorf("a refused mutation changed the record set")
	}
	return host.Outcome{Projection: map[string]any{
		"mutations": rows,
		"events":    repo.Events(),
		"records":   after,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-refusal-carries-its-verbatim-message", everyRefusalCarriesItsVerbatimMessage).
		Subject("a-refusal-writes-no-event-and-changes-no-record", aRefusalWritesNoEventAndChangesNoRecord)
}

// endregion 🔖️Registration
