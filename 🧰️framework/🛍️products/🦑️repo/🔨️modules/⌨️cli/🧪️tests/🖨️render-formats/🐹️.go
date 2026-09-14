// 🐹️ Go side of the stream-rendering case. It replays every committed event stream through the
// three renderers of the Go implementation with the terminal decision and the elapsed time
// supplied, so the bytes a vector states are the bytes the renderer owes on every host.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📥️ The committed vectors every scenario reads.
func vectors(ctx *host.Context) ([]map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("local://📡️event-streams.json")
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

// 🔢️ A numeric member, zero when absent.
func number(value map[string]interface{}, key string) int64 {
	if member, ok := value[key].(float64); ok {
		return int64(member)
	}
	return 0
}

// ☑️ A boolean member, false when absent.
func flag(value map[string]interface{}, key string) bool {
	member, _ := value[key].(bool)
	return member
}

// 🖨️ Renders one vector in a given format.
func render(vector map[string]interface{}, format string, isTTY bool) (map[string]interface{}, error) {
	events, err := json.Marshal(vector["events"])
	if err != nil {
		return nil, err
	}
	encoded, err := cli.RenderEventStream(string(events), format, isTTY, flag(vector, "verbose"), number(vector, "elapsedMs"))
	if err != nil {
		return nil, err
	}
	var rendered map[string]interface{}
	return rendered, json.Unmarshal([]byte(encoded), &rendered)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyStreamRendersItsStatedBytes(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		stated, ok := vector["expect"].(map[string]interface{})
		if !ok {
			return host.Outcome{}, fmt.Errorf("%s: no expectation", text(vector, "id"))
		}
		actual, err := render(vector, text(vector, "format"), flag(vector, "isTty"))
		if err != nil {
			return host.Outcome{}, err
		}
		for _, member := range []string{"out", "err"} {
			if text(actual, member) != text(stated, member) {
				return host.Outcome{}, fmt.Errorf("%s: %s expected %q got %q", text(vector, "id"), member, text(stated, member), text(actual, member))
			}
		}
		if number(actual, "exitCode") != number(stated, "exitCode") {
			return host.Outcome{}, fmt.Errorf("%s: exit code expected %d got %d", text(vector, "id"), number(stated, "exitCode"), number(actual, "exitCode"))
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "rendered": actual})
	}
	return host.Outcome{Projection: map[string]any{"rendered": rows}}, nil
}

func noColourWithoutATerminal(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		if text(vector, "format") != "text" {
			continue
		}
		actual, err := render(vector, "text", false)
		if err != nil {
			return host.Outcome{}, err
		}
		body := text(actual, "out") + text(actual, "err")
		if strings.ContainsRune(body, 0x1b) {
			return host.Outcome{}, fmt.Errorf("%s: an escape sequence survived a non-terminal render", text(vector, "id"))
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "plain": body})
	}
	return host.Outcome{Projection: map[string]any{"plain": rows}}, nil
}

func theExitCodeFollowsTheDoneEvent(ctx *host.Context) (host.Outcome, error) {
	parsed, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range parsed {
		wanted := int64(0)
		events, _ := vector["events"].([]interface{})
		for _, item := range events {
			event, _ := item.(map[string]interface{})
			if done, ok := event["done"].(map[string]interface{}); ok {
				wanted = number(done, "exit_code")
			}
		}
		for _, format := range []string{"json", "text", "md"} {
			actual, err := render(vector, format, false)
			if err != nil {
				return host.Outcome{}, err
			}
			if number(actual, "exitCode") != wanted {
				return host.Outcome{}, fmt.Errorf("%s: %s reported %d, the done event carried %d", text(vector, "id"), format, number(actual, "exitCode"), wanted)
			}
		}
		rows = append(rows, map[string]any{"id": text(vector, "id"), "exitCode": wanted})
	}
	return host.Outcome{Projection: map[string]any{"codes": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🧭️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-stream-renders-its-stated-bytes", everyStreamRendersItsStatedBytes).
		Subject("no-colour-without-a-terminal", noColourWithoutATerminal).
		Subject("the-exit-code-follows-the-done-event", theExitCodeFollowsTheDoneEvent)
}

// endregion 🔖️Registration
