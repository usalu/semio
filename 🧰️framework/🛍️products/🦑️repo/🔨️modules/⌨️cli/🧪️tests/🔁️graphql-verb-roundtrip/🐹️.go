// 🐹️ Go side of the verb → document → engine → renderer round trip. It executes every committed
// document against the frozen recording context and renders the resulting stream three ways, so the
// closed loop is measured in the Go implementation exactly as it is in the others.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📥️ The frozen repository every vector executes against.
func records(ctx *host.Context) (string, error) {
	raw, err := ctx.FixtureBytes("shared://🔁️graphql-verb-roundtrip/🗄️repo-records.json")
	if err != nil {
		return "", err
	}
	return string(raw), nil
}

// 📥️ The committed vectors every scenario reads.
func vectors(ctx *host.Context) ([]map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("shared://🔁️graphql-verb-roundtrip/🔁️verb-queries.json")
	if err != nil {
		return nil, err
	}
	var document struct {
		Vectors []map[string]interface{} `json:"vectors"`
	}
	if err := json.Unmarshal(raw, &document); err != nil {
		return nil, err
	}
	return document.Vectors, nil
}

// 🔤️ A string member, empty when absent.
func text(value map[string]interface{}, key string) string {
	if member, ok := value[key].(string); ok {
		return member
	}
	return ""
}

// ❓️ Whether the vector expects a refusal.
func refuses(vector map[string]interface{}) bool {
	member, _ := vector["expectError"].(bool)
	return member
}

// ▶️ Executes one vector and returns its three renderings and exit code.
func roundtrip(recorded string, vector map[string]interface{}) (map[string]interface{}, error) {
	variables := vector["variables"]
	if variables == nil {
		variables = map[string]interface{}{}
	}
	encodedVariables, err := json.Marshal(variables)
	if err != nil {
		return nil, err
	}
	encoded, err := cli.GraphQLRoundtrip(recorded, text(vector, "query"), string(encodedVariables))
	if err != nil {
		return nil, err
	}
	var result map[string]interface{}
	return result, json.Unmarshal([]byte(encoded), &result)
}

// 📃️ The lines of a text, counted the way `str::lines` counts them.
func lines(body string) []string {
	if body == "" {
		return []string{}
	}
	return strings.Split(strings.TrimSuffix(body, "\n"), "\n")
}

// 🔢️ The exit code of one round trip, or -1 when it carried none.
func exitCode(result map[string]interface{}) int64 {
	if member, ok := result["exitCode"].(float64); ok {
		return int64(member)
	}
	return -1
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyVerbDocumentExecutesAndRenders(ctx *host.Context) (host.Outcome, error) {
	recorded, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if refuses(vector) {
			continue
		}
		result, err := roundtrip(recorded, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		if code := exitCode(result); code != 0 {
			return host.Outcome{}, fmt.Errorf("%s: expected a successful stream, got exit %d", text(vector, "id"), code)
		}
		ndjson := text(result, "ndjson")
		if len(lines(ndjson)) != 1 {
			return host.Outcome{}, fmt.Errorf("%s: NDJSON must carry exactly one line, got %d", text(vector, "id"), len(lines(ndjson)))
		}
		var body interface{}
		if err := json.Unmarshal([]byte(strings.TrimRight(ndjson, "\n")), &body); err != nil {
			return host.Outcome{}, fmt.Errorf("%s: NDJSON is not one JSON document: %w", text(vector, "id"), err)
		}
		if markers, ok := vector["mustContain"].(map[string]interface{}); ok {
			for _, format := range []string{"ndjson", "markdown", "human"} {
				marker := text(markers, format)
				if marker == "" {
					continue
				}
				if !strings.Contains(text(result, format), marker) {
					return host.Outcome{}, fmt.Errorf("%s: the %s rendering is missing %s", text(vector, "id"), format, marker)
				}
			}
		}
		rows = append(rows, map[string]any{
			"id":            text(vector, "id"),
			"verb":          text(vector, "verb"),
			"ndjsonLines":   1,
			"markdownLines": len(lines(text(result, "markdown"))),
			"humanLines":    len(lines(text(result, "human"))),
		})
	}
	return host.Outcome{Projection: map[string]any{"roundtrips": rows}}, nil
}

func anExecutionFailureBecomesAFailingStream(ctx *host.Context) (host.Outcome, error) {
	recorded, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if !refuses(vector) {
			continue
		}
		result, err := roundtrip(recorded, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		if code := exitCode(result); code != 1 {
			return host.Outcome{}, fmt.Errorf("%s: a refusal owes exit code 1, got %d", text(vector, "id"), code)
		}
		if text(result, "ndjson") != "" {
			return host.Outcome{}, fmt.Errorf("%s: a refusal must write nothing to standard output", text(vector, "id"))
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "exitCode": 1})
	}
	return host.Outcome{Projection: map[string]any{"refused": rows}}, nil
}

func renderingIsDeterministic(ctx *host.Context) (host.Outcome, error) {
	recorded, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if refuses(vector) {
			continue
		}
		first, err := roundtrip(recorded, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		second, err := roundtrip(recorded, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		firstEncoded, _ := json.Marshal(first)
		secondEncoded, _ := json.Marshal(second)
		if string(firstEncoded) != string(secondEncoded) {
			return host.Outcome{}, fmt.Errorf("%s: rendering is not deterministic", text(vector, "id"))
		}
		rows = append(rows, text(vector, "id"))
	}
	return host.Outcome{Projection: map[string]any{"stable": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🧭️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-verb-document-executes-and-renders", everyVerbDocumentExecutesAndRenders).
		Subject("an-execution-failure-becomes-a-failing-stream", anExecutionFailureBecomesAFailingStream).
		Subject("rendering-is-deterministic", renderingIsDeterministic)
}

// endregion 🔖️Registration
