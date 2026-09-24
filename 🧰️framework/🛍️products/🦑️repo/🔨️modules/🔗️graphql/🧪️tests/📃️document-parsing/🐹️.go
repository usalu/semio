// 🐹️ Go side of the document-parsing case. It parses the shared corpus with the owner's own
// grammar and emits the canonical AST projection of `🧬️schema/🔣️.json` — nothing else.
package adapter

import (
	"encoding/json"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

type corpus struct {
	Documents []struct {
		ID     string `json:"id"`
		Source string `json:"source"`
	} `json:"documents"`
}

func readCorpus(ctx *host.Context) (corpus, error) {
	bytes, err := ctx.FixtureBytes("shared://📃️document-parsing/🔣️documents.json")
	if err != nil {
		return corpus{}, err
	}
	var parsed corpus
	return parsed, json.Unmarshal(bytes, &parsed)
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func corpusProjectsIdentically(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readCorpus(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Documents))
	for _, entry := range parsed.Documents {
		document, err := graphql.Parse(entry.Source)
		if err != nil {
			return host.Outcome{}, err
		}
		rows = append(rows, map[string]any{"id": entry.ID, "document": document.Projection()})
	}
	return host.Outcome{Projection: map[string]any{"documents": rows}}, nil
}

func operationKindIsRecovered(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readCorpus(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Documents))
	for _, entry := range parsed.Documents {
		operation, err := graphql.OperationType(entry.Source)
		if err != nil {
			return host.Outcome{}, err
		}
		rows = append(rows, map[string]any{"id": entry.ID, "operation": operation})
	}
	return host.Outcome{Projection: map[string]any{"documents": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("corpus-projects-identically", corpusProjectsIdentically).
		Subject("operation-kind-is-recovered", operationKindIsRecovered)
}

// endregion 🔖️Registration
