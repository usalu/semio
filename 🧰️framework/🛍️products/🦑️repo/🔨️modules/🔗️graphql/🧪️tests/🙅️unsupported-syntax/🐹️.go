// 🐹️ Go side of the subset-boundary case. Both edges are recorded: what the grammar refuses that
// GraphQL allows, and what it accepts that GraphQL refuses — with the verbatim diagnostic.
package adapter

import (
	"encoding/json"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

type corpus struct {
	Inputs []struct {
		ID     string `json:"id"`
		Source string `json:"source"`
	} `json:"inputs"`
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func subsetBoundaryIsIdentical(ctx *host.Context) (host.Outcome, error) {
	bytes, err := ctx.FixtureBytes("local://🔣️divergences.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var parsed corpus
	if err := json.Unmarshal(bytes, &parsed); err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Inputs))
	for _, entry := range parsed.Inputs {
		document, failure := graphql.Parse(entry.Source)
		row := map[string]any{"input": entry.ID, "accepted": failure == nil, "message": ""}
		if failure != nil {
			row["message"] = failure.Error()
		} else {
			row["document"] = document.Projection()
		}
		rows = append(rows, row)
	}
	return host.Outcome{Projection: map[string]any{"inputs": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").Subject("subset-boundary-is-identical", subsetBoundaryIsIdentical)
}

// endregion 🔖️Registration
