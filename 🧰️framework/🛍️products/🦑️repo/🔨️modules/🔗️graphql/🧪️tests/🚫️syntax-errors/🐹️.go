// 🐹️ Go side of the malformed-input case. Rejection is the observation; the verbatim message rides
// along as `detail`, which the `diagnostic-v1` profile drops before comparison.
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

func malformedInputsAreRejected(ctx *host.Context) (host.Outcome, error) {
	bytes, err := ctx.FixtureBytes("local://🔣️malformed.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var parsed corpus
	if err := json.Unmarshal(bytes, &parsed); err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Inputs))
	for _, entry := range parsed.Inputs {
		failure := graphql.Validate(entry.Source)
		detail := ""
		if failure != nil {
			detail = failure.Error()
		}
		rows = append(rows, map[string]any{"input": entry.ID, "rejected": failure != nil, "detail": detail})
	}
	return host.Outcome{Projection: map[string]any{"inputs": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").Subject("malformed-inputs-are-rejected", malformedInputsAreRejected)
}

// endregion 🔖️Registration
