// 🐹️ Go side of the YAML codec case. Written against the frozen vector set, never against the
// other adapters.
package adapter

import (
	"encoding/json"
	"fmt"

	yaml "github.com/usalu/semio/repo/yaml"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type codecVector struct {
	Name   string `json:"name"`
	Source string `json:"source"`
}

type codecVectorFile struct {
	Schema  string        `json:"schema"`
	Vectors []codecVector `json:"vectors"`
}

func loadVectors(ctx *host.Context) ([]codecVector, error) {
	data, err := ctx.FixtureBytes("shared://📡️codec-vectors.json")
	if err != nil {
		return nil, err
	}
	var file codecVectorFile
	if err := json.Unmarshal(data, &file); err != nil {
		return nil, err
	}
	return file.Vectors, nil
}

func canonical(value interface{}) string {
	encoded, err := json.Marshal(value)
	if err != nil {
		return "!" + err.Error()
	}
	return string(encoded)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func vectorsDecodeToTheSameValue(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	decoded := make([]string, 0, len(vectors))
	for _, vector := range vectors {
		var node interface{}
		if err := yaml.Unmarshal([]byte(vector.Source), &node); err != nil {
			decoded = append(decoded, fmt.Sprintf("%s!error", vector.Name))
			continue
		}
		decoded = append(decoded, fmt.Sprintf("%s=%s", vector.Name, canonical(node)))
	}
	return host.Outcome{Projection: map[string]any{"decoded": decoded}}, nil
}

func decodeEncodeDecodeIsIdempotent(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	reDecoded := make([]string, 0, len(vectors))
	for _, vector := range vectors {
		var node interface{}
		if err := yaml.Unmarshal([]byte(vector.Source), &node); err != nil {
			reDecoded = append(reDecoded, fmt.Sprintf("%s!decode", vector.Name))
			continue
		}
		encoded, err := yaml.Marshal(node)
		if err != nil {
			reDecoded = append(reDecoded, fmt.Sprintf("%s!encode", vector.Name))
			continue
		}
		var again interface{}
		if err := yaml.Unmarshal(encoded, &again); err != nil {
			reDecoded = append(reDecoded, fmt.Sprintf("%s!redecode", vector.Name))
			continue
		}
		reDecoded = append(reDecoded, fmt.Sprintf("%s=%s", vector.Name, canonical(again)))
	}
	return host.Outcome{Projection: map[string]any{"reDecoded": reDecoded}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("vectors-decode-to-the-same-value", vectorsDecodeToTheSameValue).
		Subject("decode-encode-decode-is-idempotent", decodeEncodeDecodeIsIdempotent)
}

// endregion 🔖️Registration
