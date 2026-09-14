// 🐹️ Go subject for the Mermaid treemap rendering case, written against the same frozen contract
// as the Rust subject.
//
// 🚚️ The package `github.com/usalu/semio/repo/tree` is produced by the Go split of
// `💻️client/⌨️cli/🧩️component.go`; until it exists this adapter cannot compile and the Go subject
// of this case is reported as blocked rather than passing.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	tree "github.com/usalu/semio/repo/tree"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// 📥️ fixture decodes the committed Mermaid fixture.
func fixture(ctx *host.Context) (map[string]any, error) {
	raw, err := ctx.FixtureBytes("shared://🧜️mermaid-vectors.json")
	if err != nil {
		return nil, err
	}
	var decoded map[string]any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// 🧾️ vectors reads the vector list of the fixture.
func vectors(document map[string]any) []map[string]any {
	rows, _ := document["vectors"].([]any)
	out := make([]map[string]any, 0, len(rows))
	for _, row := range rows {
		if entry, ok := row.(map[string]any); ok {
			out = append(out, entry)
		}
	}
	return out
}

// 🧜️ treemapOf decodes the treemap member of a JSON object.
func treemapOf(document map[string]any, field string) (*tree.MermaidTreemap, error) {
	raw, err := json.Marshal(document[field])
	if err != nil {
		return nil, err
	}
	return tree.ParseMermaidTreemap(raw)
}

// 📃️ stringList reads a string list out of a decoded JSON object.
func stringList(document map[string]any, field string) []string {
	rows, _ := document[field].([]any)
	out := make([]string, 0, len(rows))
	for _, row := range rows {
		value, _ := row.(string)
		out = append(out, value)
	}
	return out
}

// 📏️ lines splits a rendered diagram into its lines.
func lines(text string) []string { return strings.Split(text, "\n") }

// ⚖️ require fails the scenario when a projection drifts from the committed expectation.
func require(name string, actual, want []string) error {
	if len(actual) == len(want) {
		same := true
		for index := range actual {
			if actual[index] != want[index] {
				same = false
				break
			}
		}
		if same {
			return nil
		}
	}
	return fmt.Errorf("%s: expected %q, got %q", name, want, actual)
}

// 🏷️ labels collects every label of a treemap node, depth first.
func labels(node tree.MermaidNode, out *[]string) {
	*out = append(*out, node.Label)
	for _, child := range node.Children {
		labels(child, out)
	}
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func rendersEveryTreemap(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projected := map[string]any{}
	for _, vector := range vectors(document) {
		treemap, err := treemapOf(vector, "treemap")
		if err != nil {
			return host.Outcome{}, err
		}
		id, _ := vector["id"].(string)
		rendered := lines(tree.RenderMermaidTreemap(treemap))
		if err := require(id, rendered, stringList(vector, "lines")); err != nil {
			return host.Outcome{}, err
		}
		projected[id] = rendered
	}
	return host.Outcome{Projection: projected}, nil
}

func projectsATreeIntoATreemap(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection, _ := document["projection"].(map[string]any)
	raw, err := json.Marshal(projection["tree"])
	if err != nil {
		return host.Outcome{}, err
	}
	spec, err := tree.ParseTreeNodeSpec(raw)
	if err != nil {
		return host.Outcome{}, err
	}
	title, _ := projection["title"].(string)
	weightKey, _ := projection["weightKey"].(string)
	rendered := lines(tree.RenderMermaidTreemap(tree.MermaidTreemapFromTree(spec.ToTreeNode(), title, weightKey)))
	if err := require("projection", rendered, stringList(projection, "lines")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: rendered}, nil
}

func escapesQuotesInLabels(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	candidates := []string{`"""`}
	for _, vector := range vectors(document) {
		treemap, err := treemapOf(vector, "treemap")
		if err != nil {
			return host.Outcome{}, err
		}
		candidates = append(candidates, treemap.Title)
		for _, node := range treemap.Nodes {
			labels(node, &candidates)
		}
	}
	escaped := []string{}
	for _, candidate := range candidates {
		once := tree.MermaidEscapeLabel(candidate)
		if strings.Contains(once, `"`) {
			return host.Outcome{}, fmt.Errorf("a double quote survived the escape of %q", candidate)
		}
		if tree.MermaidEscapeLabel(once) != once {
			return host.Outcome{}, fmt.Errorf("escaping %q is not idempotent", candidate)
		}
		escaped = append(escaped, once)
	}
	return host.Outcome{Projection: escaped}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("renders-every-treemap", rendersEveryTreemap).
		Subject("projects-a-tree-into-a-treemap", projectsATreeIntoATreemap).
		Subject("escapes-quotes-in-labels", escapesQuotesInLabels)
}

// endregion 🔖️Registration
