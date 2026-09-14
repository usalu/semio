// 🐹️ Go side of the glob matching case.
package adapter

import (
	"encoding/json"
	"fmt"

	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type globVector struct {
	Name     string `json:"name"`
	Pattern  string `json:"pattern"`
	Path     string `json:"path"`
	Expected bool   `json:"expected"`
	Owned    bool   `json:"owned"`
	Glob     bool   `json:"glob"`
}

type globVectorFile struct {
	Schema           string       `json:"schema"`
	Vectors          []globVector `json:"vectors"`
	BraceVectors     []globVector `json:"braceVectors"`
	DivergentVectors []globVector `json:"divergentVectors"`
}

func loadGlobVectors(ctx *host.Context) (globVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://📡️glob-vectors.json")
	if err != nil {
		return globVectorFile{}, err
	}
	var file globVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

func globVerdictsFor(vectors []globVector) []string {
	verdicts := make([]string, 0, len(vectors))
	for _, vector := range vectors {
		matched, matchErr := workspace.Match(vector.Pattern, vector.Path)
		if matchErr != nil {
			verdicts = append(verdicts, fmt.Sprintf("%s=error", vector.Name))
			continue
		}
		verdicts = append(verdicts, fmt.Sprintf("%s=%t", vector.Name, matched))
	}
	return verdicts
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func vectorsMatchTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := loadGlobVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"verdicts": globVerdictsFor(file.Vectors)}}, nil
}

func braceAlternationExpandsTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := loadGlobVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"verdicts": globVerdictsFor(file.BraceVectors)}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("vectors-match-the-same-way", vectorsMatchTheSameWay).
		Subject("brace-alternation-expands-the-same-way", braceAlternationExpandsTheSameWay)
}

// endregion 🔖️Registration
