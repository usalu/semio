// 🐹️ Go side of the slug vocabulary case.
package adapter

import (
	"encoding/json"

	host "semio.tech/repo/test"

	model "github.com/usalu/semio/repo/model"
)

// region 🔖️Vectors

const vectorsURI = "local://🔣️vectors.json"

type vectorGroup struct {
	Normalise []string `json:"normalise"`
	Resolve   []string `json:"resolve"`
	Reject    []string `json:"reject"`
}

type vectors struct {
	LLM    vectorGroup `json:"llm"`
	Effort vectorGroup `json:"effort"`
	Client vectorGroup `json:"client"`
}

func readVectors(ctx *host.Context) (vectors, error) {
	raw, err := ctx.FixtureBytes(vectorsURI)
	if err != nil {
		return vectors{}, err
	}
	var parsed vectors
	if err := json.Unmarshal(raw, &parsed); err != nil {
		return vectors{}, err
	}
	return parsed, nil
}

// 🚫️errorClass names the single outcome class this vocabulary can fail with; the rendered message
// 🚫️carries the whole allowed list and is not part of the contract.
const errorClass = "not-allowed"

func normalised(inputs []string, normalise func(string) string) map[string]any {
	once := make([]string, 0, len(inputs))
	twice := make([]string, 0, len(inputs))
	for _, input := range inputs {
		first := normalise(input)
		once = append(once, first)
		twice = append(twice, normalise(first))
	}
	return map[string]any{"once": once, "twice": twice}
}

func resolved(inputs []string, resolve func(string) (string, error)) []string {
	out := make([]string, 0, len(inputs))
	for _, input := range inputs {
		value, err := resolve(input)
		if err != nil {
			out = append(out, "<"+errorClass+">")
			continue
		}
		out = append(out, value)
	}
	return out
}

func rejected(inputs []string, resolve func(string) (string, error)) []string {
	out := make([]string, 0, len(inputs))
	for _, input := range inputs {
		value, err := resolve(input)
		if err != nil {
			out = append(out, errorClass)
			continue
		}
		out = append(out, "<resolved:"+value+">")
	}
	return out
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func normalisationIsCanonicalAndIdempotent(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"llm":    normalised(parsed.LLM.Normalise, model.NormalizeLLMSlug),
		"effort": normalised(parsed.Effort.Normalise, model.NormalizeEffortSlug),
		"client": normalised(parsed.Client.Normalise, model.NormalizeClientSlug),
	}}, nil
}

func resolutionPicksTheLongestAllowedMatch(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"llm":    resolved(parsed.LLM.Resolve, model.ResolveAllowedLLM),
		"effort": resolved(parsed.Effort.Resolve, model.ResolveAllowedEffort),
		"client": resolved(parsed.Client.Resolve, model.ResolveAllowedClient),
	}}, nil
}

func unresolvableInputIsAnErrorClass(ctx *host.Context) (host.Outcome, error) {
	parsed, err := readVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"llm":    rejected(parsed.LLM.Reject, model.ResolveAllowedLLM),
		"effort": rejected(parsed.Effort.Reject, model.ResolveAllowedEffort),
		"client": rejected(parsed.Client.Reject, model.ResolveAllowedClient),
	}}, nil
}

func theAllowedTableIsLoadedNotRestated(_ *host.Context) (host.Outcome, error) {
	return host.Outcome{Projection: map[string]any{
		"llms":    model.AllowedLLMs,
		"efforts": model.AllowedEfforts,
		"clients": model.AllowedClients,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("normalisation-is-canonical-and-idempotent", normalisationIsCanonicalAndIdempotent).
		Subject("resolution-picks-the-longest-allowed-match", resolutionPicksTheLongestAllowedMatch).
		Subject("unresolvable-input-is-an-error-class", unresolvableInputIsAnErrorClass).
		Subject("the-allowed-table-is-loaded-not-restated", theAllowedTableIsLoadedNotRestated)
}

// endregion 🔖️Registration
