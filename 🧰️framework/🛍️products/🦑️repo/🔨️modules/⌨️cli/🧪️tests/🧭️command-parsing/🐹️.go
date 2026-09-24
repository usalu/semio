// 🐹️ Go side of the argv → command projection case. It parses every committed vector against the
// repo command tree of the Go implementation and emits the same narrowed projection the other
// adapters emit, so a disagreement is a disagreement about the grammar.
package adapter

import (
	"encoding/json"
	"fmt"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📥️ The committed vectors every scenario reads.
func vectors(ctx *host.Context) ([]map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("shared://🧭️command-parsing/🔣️argv-vectors.json")
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

// 📜️ A string-array member as a `[]string`.
func strings(value map[string]interface{}, key string) []string {
	items, _ := value[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		out = append(out, fmt.Sprintf("%v", item))
	}
	return out
}

// 🧾️ One projection reduced to the members its vector states, so a vector expresses an intent
// rather than the whole default flag table.
func narrow(projection map[string]interface{}, stated map[string]interface{}) map[string]interface{} {
	flags := map[string]interface{}{}
	if wanted, ok := stated["flags"].(map[string]interface{}); ok {
		actual, _ := projection["flags"].(map[string]interface{})
		for name := range wanted {
			flags[name] = actual[name]
		}
	}
	return map[string]interface{}{"path": projection["path"], "positional": projection["positional"], "flags": flags, "help": projection["help"]}
}

// 🧭️ Parses one vector and returns its narrowed projection.
func project(vector map[string]interface{}, stated map[string]interface{}) (map[string]interface{}, error) {
	encoded, err := cli.ProjectArgv(strings(vector, "argv"))
	if err != nil {
		return nil, err
	}
	var projection map[string]interface{}
	if err := json.Unmarshal([]byte(encoded), &projection); err != nil {
		return nil, err
	}
	return narrow(projection, stated), nil
}

// ⚖️ Canonical JSON of one value, for comparing a projection with its stated form.
func canonical(value interface{}) string {
	encoded, _ := json.Marshal(value)
	return string(encoded)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func argvProjectsIntoACommand(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		stated, ok := vector["expect"].(map[string]interface{})
		if !ok {
			continue
		}
		projection, err := project(vector, stated)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: unexpected refusal %s", text(vector, "id"), err)
		}
		if canonical(projection) != canonical(stated) {
			return host.Outcome{}, fmt.Errorf("%s: expected %s got %s", text(vector, "id"), canonical(stated), canonical(projection))
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "projection": projection})
	}
	return host.Outcome{Projection: map[string]any{"accepted": rows}}, nil
}

func refusalsCarryTheirVerbatimMessage(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		stated := text(vector, "error")
		if _, declared := vector["error"]; !declared {
			continue
		}
		if _, err := cli.ProjectArgv(strings(vector, "argv")); err == nil {
			return host.Outcome{}, fmt.Errorf("%s: expected the refusal %s", text(vector, "id"), stated)
		} else if err.Error() != stated {
			return host.Outcome{}, fmt.Errorf("%s: expected %s got %s", text(vector, "id"), stated, err.Error())
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "message": stated})
	}
	return host.Outcome{Projection: map[string]any{"refused": rows}}, nil
}

func parsingIsIdempotent(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if _, declared := vector["expect"]; !declared {
			continue
		}
		first, err := cli.ProjectArgv(strings(vector, "argv"))
		if err != nil {
			return host.Outcome{}, err
		}
		second, err := cli.ProjectArgv(strings(vector, "argv"))
		if err != nil {
			return host.Outcome{}, err
		}
		if first != second {
			return host.Outcome{}, fmt.Errorf("%s: parsing is not idempotent", text(vector, "id"))
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
		Subject("argv-projects-into-a-command", argvProjectsIntoACommand).
		Subject("refusals-carry-their-verbatim-message", refusalsCarryTheirVerbatimMessage).
		Subject("parsing-is-idempotent", parsingIsIdempotent)
}

// endregion 🔖️Registration
