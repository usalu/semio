// 🐹️ Go side of the variable-coercion case. It coerces every ROOT selection's arguments the way the
// executor does — variables substituted first, declared defaults filling only absent argument names.
package adapter

import (
	"encoding/json"
	"sort"

	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Corpus

type corpus struct {
	Cases []struct {
		ID        string                 `json:"id"`
		Source    string                 `json:"source"`
		Variables map[string]interface{} `json:"variables"`
		Defaults  map[string]interface{} `json:"defaults"`
	} `json:"cases"`
}

func coerceCorpus(ctx *host.Context, onlyWithDefaults bool) (host.Outcome, error) {
	bytes, err := ctx.FixtureBytes("local://🔣️coercions.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var parsed corpus
	if err := json.Unmarshal(bytes, &parsed); err != nil {
		return host.Outcome{}, err
	}
	rows := make([]any, 0, len(parsed.Cases))
	for _, entry := range parsed.Cases {
		if onlyWithDefaults && len(entry.Defaults) == 0 {
			continue
		}
		document, err := graphql.Parse(entry.Source)
		if err != nil {
			return host.Outcome{}, err
		}
		selections := make([]any, 0, len(document.Selections))
		for _, selection := range document.Selections {
			args := graphql.CoerceArguments(selection.Arguments, entry.Defaults, entry.Variables)
			names := make([]string, 0, len(args))
			for name := range args {
				names = append(names, name)
			}
			sort.Strings(names)
			arguments := make([]any, 0, len(names))
			for _, name := range names {
				arguments = append(arguments, map[string]any{"name": name, "value": args[name]})
			}
			selections = append(selections, map[string]any{"key": selection.Key(), "arguments": arguments})
		}
		rows = append(rows, map[string]any{"id": entry.ID, "selections": selections})
	}
	return host.Outcome{Projection: map[string]any{"cases": rows}}, nil
}

// endregion 🔖️Corpus

// region 🔖️Scenarios

func argumentsResolveAgainstVariables(ctx *host.Context) (host.Outcome, error) {
	return coerceCorpus(ctx, false)
}

func defaultsFillOnlyAbsentArguments(ctx *host.Context) (host.Outcome, error) {
	return coerceCorpus(ctx, true)
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("arguments-resolve-against-variables", argumentsResolveAgainstVariables).
		Subject("defaults-fill-only-absent-arguments", defaultsFillOnlyAbsentArguments)
}

// endregion 🔖️Registration
