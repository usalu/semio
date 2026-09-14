// 🐹️ Go side of the definition-kind derivation case.
package adapter

import (
	"encoding/json"

	host "semio.tech/repo/test"

	model "github.com/usalu/semio/repo/model"
)

// region 🔖️Vectors

const vectorsURI = "local://🔣️vectors.json"

var declaredKinds = []string{"implementation", "interface", "constant", "test"}

func readKeywords(ctx *host.Context) ([]string, error) {
	raw, err := ctx.FixtureBytes(vectorsURI)
	if err != nil {
		return nil, err
	}
	var parsed struct {
		Keywords []string `json:"keywords"`
	}
	if err := json.Unmarshal(raw, &parsed); err != nil {
		return nil, err
	}
	return parsed.Keywords, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyKeywordDerivesOneKind(ctx *host.Context) (host.Outcome, error) {
	keywords, err := readKeywords(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	derived := make([]map[string]any, 0, len(keywords))
	for _, keyword := range keywords {
		derived = append(derived, map[string]any{"keyword": keyword, "kind": string(model.DeriveDefinitionKind(keyword))})
	}
	return host.Outcome{Projection: derived}, nil
}

func derivationIsTotalAndValid(ctx *host.Context) (host.Outcome, error) {
	keywords, err := readKeywords(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	inside := true
	for _, keyword := range keywords {
		derived := string(model.DeriveDefinitionKind(keyword))
		found := false
		for _, kind := range declaredKinds {
			if derived == kind {
				found = true
				break
			}
		}
		if !found {
			inside = false
		}
	}
	return host.Outcome{Projection: map[string]any{
		"keywords":               len(keywords),
		"allInsideDeclaredKinds": inside,
		"declaredKinds":          declaredKinds,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-keyword-derives-one-kind", everyKeywordDerivesOneKind).
		Subject("derivation-is-total-and-valid", derivationIsTotalAndValid)
}

// endregion 🔖️Registration
