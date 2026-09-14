// 🐹️ Go side of the `test` verb planning case: operands in, ordered announcements out, nothing
// executed.
package adapter

import (
	"encoding/json"
	"fmt"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📥️ The committed document every scenario reads.
func document(ctx *host.Context) (map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("local://🧪️test-verb-vectors.json")
	if err != nil {
		return nil, err
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal(raw, &parsed); err != nil {
		return nil, err
	}
	return parsed, nil
}

// 🌍️ The frozen snapshot, as the JSON text the verb entry point takes.
func snapshot(parsed map[string]interface{}) string {
	encoded, _ := json.Marshal(parsed["snapshot"])
	return string(encoded)
}

// 📜️ The vectors of the document.
func vectors(parsed map[string]interface{}) []map[string]interface{} {
	items, _ := parsed["vectors"].([]interface{})
	out := make([]map[string]interface{}, 0, len(items))
	for _, item := range items {
		if vector, ok := item.(map[string]interface{}); ok {
			out = append(out, vector)
		}
	}
	return out
}

// 🔤️ A string member, empty when absent.
func text(value map[string]interface{}, key string) string {
	if member, ok := value[key].(string); ok {
		return member
	}
	return ""
}

// 📜️ A string-array member as a `[]string`, never nil.
func strings(value map[string]interface{}, key string) []string {
	items, _ := value[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		out = append(out, fmt.Sprintf("%v", item))
	}
	return out
}

// 🧾️ The announcement of one vector: its lines and its refusals.
func plan(parsed map[string]interface{}, vector map[string]interface{}) (map[string]interface{}, error) {
	encoded, err := cli.TestVerbLines(snapshot(parsed), strings(vector, "operands"))
	if err != nil {
		return nil, err
	}
	var value map[string]interface{}
	if err := json.Unmarshal([]byte(encoded), &value); err != nil {
		return nil, err
	}
	return value, nil
}

// ⚖️ Canonical JSON of one value.
func canonical(value interface{}) string {
	encoded, _ := json.Marshal(value)
	return string(encoded)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func operandsPlanTheirStatedInvocations(ctx *host.Context) (host.Outcome, error) {
	parsed, err := document(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := map[string]any{}
	for _, vector := range vectors(parsed) {
		actual, err := plan(parsed, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		wanted := map[string]any{"lines": strings(vector, "lines"), "problems": strings(vector, "problems")}
		if canonical(actual) != canonical(wanted) {
			return host.Outcome{}, fmt.Errorf("%s: expected %s got %s", text(vector, "id"), canonical(wanted), canonical(actual))
		}
		rows[text(vector, "id")] = actual
	}
	return host.Outcome{Projection: rows}, nil
}

func unplannableScopesRefuseInsteadOfRunning(ctx *host.Context) (host.Outcome, error) {
	parsed, err := document(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := map[string]any{}
	for _, vector := range vectors(parsed) {
		wanted := strings(vector, "problems")
		if len(wanted) == 0 {
			continue
		}
		actual, err := plan(parsed, vector)
		if err != nil {
			return host.Outcome{}, err
		}
		problems := []string{}
		for _, problem := range actual["problems"].([]interface{}) {
			problems = append(problems, fmt.Sprintf("%v", problem))
		}
		if canonical(problems) != canonical(wanted) {
			return host.Outcome{}, fmt.Errorf("%s: expected problems %s got %s", text(vector, "id"), canonical(wanted), canonical(problems))
		}
		if lines, _ := actual["lines"].([]interface{}); len(lines) > 0 {
			return host.Outcome{}, fmt.Errorf("%s: a refusing vector announced a runner", text(vector, "id"))
		}
		rows[text(vector, "id")] = problems
	}
	return host.Outcome{Projection: rows}, nil
}

func planningIsDeterministic(ctx *host.Context) (host.Outcome, error) {
	parsed, err := document(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range vectors(parsed) {
		first, err := cli.TestVerbLines(snapshot(parsed), strings(vector, "operands"))
		if err != nil {
			return host.Outcome{}, err
		}
		second, err := cli.TestVerbLines(snapshot(parsed), strings(vector, "operands"))
		if err != nil {
			return host.Outcome{}, err
		}
		if first != second {
			return host.Outcome{}, fmt.Errorf("%s: planning is not deterministic", text(vector, "id"))
		}
		rows = append(rows, text(vector, "id"))
	}
	return host.Outcome{Projection: map[string]any{"stable": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🧪️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("operands-plan-their-stated-invocations", operandsPlanTheirStatedInvocations).
		Subject("unplannable-scopes-refuse-instead-of-running", unplannableScopesRefuseInsteadOfRunning).
		Subject("planning-is-deterministic", planningIsDeterministic)
}

// endregion 🔖️Registration
