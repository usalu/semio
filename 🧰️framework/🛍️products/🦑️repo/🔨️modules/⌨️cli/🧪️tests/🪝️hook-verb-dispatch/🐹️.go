// 🐹️ Go side of the `hook` verb dispatch case: one native invocation in, the bytes the client
// reads out, with an inert environment and an inert test-file resolver.
package adapter

import (
	"encoding/json"
	"fmt"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Support

// 📥️ The committed invocations every scenario reads.
func vectors(ctx *host.Context) ([]map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("local://🪝️hook-invocations.json")
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

// ❓️ Whether a vector states that the verb refuses it.
func refuses(vector map[string]interface{}) bool {
	member, _ := vector["refused"].(bool)
	return member
}

// 🪝️ The bytes and exit code one invocation writes.
func dispatch(vector map[string]interface{}) (map[string]interface{}, error) {
	request, err := json.Marshal(vector["request"])
	if err != nil {
		return nil, err
	}
	encoded, err := cli.HookVerbDispatch(string(request))
	if err != nil {
		return nil, err
	}
	var answer map[string]interface{}
	if err := json.Unmarshal([]byte(encoded), &answer); err != nil {
		return nil, err
	}
	return answer, nil
}

// ⚖️ Canonical JSON of one value.
func canonical(value interface{}) string {
	encoded, _ := json.Marshal(value)
	return string(encoded)
}

// endregion 🔖️Support

// region 🔖️Scenarios

func everyInvocationWritesItsBytes(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := map[string]any{}
	for _, vector := range parsed {
		if refuses(vector) {
			continue
		}
		answer, err := dispatch(vector)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: %s", text(vector, "id"), err)
		}
		rows[text(vector, "id")] = answer
	}
	return host.Outcome{Projection: rows}, nil
}

func aRefusalNeverReachesTheDomain(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := map[string]any{}
	for _, vector := range parsed {
		if !refuses(vector) {
			continue
		}
		answer, err := dispatch(vector)
		if err == nil {
			return host.Outcome{}, fmt.Errorf("%s: expected a refusal, got %s", text(vector, "id"), canonical(answer))
		}
		rows[text(vector, "id")] = err.Error() != ""
	}
	return host.Outcome{Projection: rows}, nil
}

func dispatchIsDeterministic(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if refuses(vector) {
			continue
		}
		first, err := dispatch(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		second, err := dispatch(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		if canonical(first) != canonical(second) {
			return host.Outcome{}, fmt.Errorf("%s: dispatch is not deterministic", text(vector, "id"))
		}
		rows = append(rows, text(vector, "id"))
	}
	return host.Outcome{Projection: map[string]any{"stable": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🪝️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-invocation-writes-its-bytes", everyInvocationWritesItsBytes).
		Subject("a-refusal-never-reaches-the-domain", aRefusalNeverReachesTheDomain).
		Subject("dispatch-is-deterministic", dispatchIsDeterministic)
}

// endregion 🔖️Registration
