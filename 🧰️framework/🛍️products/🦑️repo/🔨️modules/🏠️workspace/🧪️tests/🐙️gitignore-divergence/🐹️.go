// 🐹️ Go side of the ignore precedence case.
package adapter

import (
	"encoding/json"
	"fmt"

	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type ignoreVector struct {
	Name     string   `json:"name"`
	Rules    []string `json:"rules"`
	Path     string   `json:"path"`
	Expected bool     `json:"expected"`
	Owned    bool     `json:"owned"`
	Git      bool     `json:"git"`
}

type ignoreVectorFile struct {
	Schema           string         `json:"schema"`
	Vectors          []ignoreVector `json:"vectors"`
	DivergentVectors []ignoreVector `json:"divergentVectors"`
}

func loadIgnoreVectors(ctx *host.Context) (ignoreVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://📡️ignore-vectors.json")
	if err != nil {
		return ignoreVectorFile{}, err
	}
	var file ignoreVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

func verdictsFor(vectors []ignoreVector) []string {
	verdicts := make([]string, 0, len(vectors))
	for _, vector := range vectors {
		matcher := workspace.CompileIgnoreLines(vector.Rules...)
		verdicts = append(verdicts, fmt.Sprintf("%s=%t", vector.Name, matcher.MatchesPath(vector.Path)))
	}
	return verdicts
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func recordedDivergencesFromGitHold(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIgnoreVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"verdicts": verdictsFor(file.DivergentVectors)}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("recorded-divergences-from-git-hold", recordedDivergencesFromGitHold)
}

// endregion 🔖️Registration
