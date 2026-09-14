// 🐹️ Go side of the usage-text case. It renders the usage text of the Go command tree and holds it
// against the same verbatim goldens, the same structural law and the same registered root verbs.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Goldens

// 📥️ The committed goldens every scenario reads.
func goldens(ctx *host.Context) (map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("local://📖️usage-goldens.json")
	if err != nil {
		return nil, err
	}
	var document map[string]interface{}
	return document, json.Unmarshal(raw, &document)
}

// 🔤️ A string member, empty when absent.
func text(value map[string]interface{}, key string) string {
	if member, ok := value[key].(string); ok {
		return member
	}
	return ""
}

// 📜️ A string-array member as a `[]string`.
func list(value map[string]interface{}, key string) []string {
	items, _ := value[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		out = append(out, fmt.Sprintf("%v", item))
	}
	return out
}

// 📜️ A `/`-joined command path as its segments.
func segments(path string) []string {
	out := []string{}
	for _, segment := range strings.Split(path, "/") {
		if segment != "" {
			out = append(out, segment)
		}
	}
	return out
}

// endregion 🔖️Goldens

// region 🔖️Scenarios

func representativeCommandsStateTheirText(ctx *host.Context) (host.Outcome, error) {
	document, err := goldens(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	entries, _ := document["goldens"].([]interface{})
	rows := []any{}
	for _, entry := range entries {
		golden, _ := entry.(map[string]interface{})
		path := list(golden, "path")
		actual, ok := cli.UsageText(path)
		if !ok {
			return host.Outcome{}, fmt.Errorf("%v: no such command", path)
		}
		if actual != text(golden, "text") {
			return host.Outcome{}, fmt.Errorf("%v: expected %q got %q", path, text(golden, "text"), actual)
		}
		rows = append(rows, map[string]any{"path": strings.Join(path, "/"), "text": actual})
	}
	return host.Outcome{Projection: map[string]any{"goldens": rows}}, nil
}

func everyCommandObeysTheUsageLaw(ctx *host.Context) (host.Outcome, error) {
	document, err := goldens(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	structure, ok := document["structure"].(map[string]interface{})
	if !ok {
		return host.Outcome{}, fmt.Errorf("the goldens state no structure")
	}
	usageMarker := text(structure, "usageMarker")
	commandsMarker := text(structure, "commandsMarker")
	childIndent := text(structure, "childIndent")
	rows := []any{}
	for _, path := range cli.CommandPaths() {
		body, ok := cli.UsageText(segments(path))
		if !ok {
			return host.Outcome{}, fmt.Errorf("%s: no such command", path)
		}
		if !strings.Contains(body, usageMarker) {
			return host.Outcome{}, fmt.Errorf("%s: the usage marker is missing", path)
		}
		if strings.HasPrefix(body, usageMarker) {
			return host.Outcome{}, fmt.Errorf("%s: the summary is missing", path)
		}
		if index := strings.Index(body, commandsMarker); index >= 0 {
			for _, line := range lines(body[index+len(commandsMarker):]) {
				if line == "" {
					continue
				}
				if !strings.HasPrefix(line, childIndent) {
					return host.Outcome{}, fmt.Errorf("%s: the child line %q is not indented", path, line)
				}
			}
		}
		rows = append(rows, map[string]any{"path": path, "lines": len(lines(body))})
	}
	root, ok := cli.UsageText(nil)
	if !ok {
		return host.Outcome{}, fmt.Errorf("no root command")
	}
	if !strings.HasPrefix(root, text(structure, "rootPrefix")) {
		return host.Outcome{}, fmt.Errorf("the root usage text does not open with its stated prefix")
	}
	return host.Outcome{Projection: map[string]any{"commands": rows}}, nil
}

// 📃️ The lines of a text, counted the way `str::lines` counts them: a trailing newline closes the
// last line rather than opening an empty one.
func lines(body string) []string {
	if body == "" {
		return []string{}
	}
	trimmed := strings.TrimSuffix(body, "\n")
	return strings.Split(trimmed, "\n")
}

func theRootRegistersItsStatedVerbs(ctx *host.Context) (host.Outcome, error) {
	document, err := goldens(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	wanted := list(document, "rootVerbs")
	actual := []string{}
	for _, path := range cli.CommandPaths() {
		if path == "" || strings.Contains(path, "/") {
			continue
		}
		actual = append(actual, path)
	}
	if strings.Join(actual, ",") != strings.Join(wanted, ",") {
		return host.Outcome{}, fmt.Errorf("expected %v got %v", wanted, actual)
	}
	verbs := make([]any, 0, len(actual))
	for _, verb := range actual {
		verbs = append(verbs, verb)
	}
	return host.Outcome{Projection: map[string]any{"verbs": verbs}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🧭️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("representative-commands-state-their-text", representativeCommandsStateTheirText).
		Subject("every-command-obeys-the-usage-law", everyCommandObeysTheUsageLaw).
		Subject("the-root-registers-its-stated-verbs", theRootRegistersItsStatedVerbs)
}

// endregion 🔖️Registration
