// 🐹️ Go subject for the child sorting case, written against the same frozen contract as the Rust
// subject.
//
// 🚚️ The package `github.com/usalu/semio/repo/tree` is produced by the Go split of
// `💻️client/⌨️cli/🧩️component.go`; until it exists this adapter cannot compile and the Go subject
// of this case is reported as blocked rather than passing.
package adapter

import (
	"encoding/json"
	"fmt"

	tree "github.com/usalu/semio/repo/tree"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// 📥️ vectors decodes the committed sorting vectors.
func vectors(ctx *host.Context) ([]map[string]any, error) {
	raw, err := ctx.FixtureBytes("shared://🔀️sort-vectors.json")
	if err != nil {
		return nil, err
	}
	var decoded map[string]any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	rows, _ := decoded["vectors"].([]any)
	out := make([]map[string]any, 0, len(rows))
	for _, row := range rows {
		if entry, ok := row.(map[string]any); ok {
			out = append(out, entry)
		}
	}
	return out, nil
}

// 🌿️ treeOf is the tree of one vector.
func treeOf(vector map[string]any) (*tree.TreeNode, error) {
	raw, err := json.Marshal(vector["tree"])
	if err != nil {
		return nil, err
	}
	spec, err := tree.ParseTreeNodeSpec(raw)
	if err != nil {
		return nil, err
	}
	return spec.ToTreeNode(), nil
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

// endregion 🔖️Helpers

// region 🔖️Scenarios

func sortsEveryVector(ctx *host.Context) (host.Outcome, error) {
	rows, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projected := map[string]any{}
	for _, vector := range rows {
		node, err := treeOf(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		tree.SortTreeChildren(node)
		id, _ := vector["id"].(string)
		outline := tree.TreeOutline(node)
		if err := require(id, outline, stringList(vector, "outline")); err != nil {
			return host.Outcome{}, err
		}
		projected[id] = outline
	}
	return host.Outcome{Projection: projected}, nil
}

func sortingIsIdempotent(ctx *host.Context) (host.Outcome, error) {
	rows, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	checked := []string{}
	for _, vector := range rows {
		node, err := treeOf(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		tree.SortTreeChildren(node)
		once := tree.TreeOutline(node)
		tree.SortTreeChildren(node)
		id, _ := vector["id"].(string)
		if err := require(id, tree.TreeOutline(node), once); err != nil {
			return host.Outcome{}, err
		}
		checked = append(checked, id)
	}
	return host.Outcome{Projection: checked}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("sorts-every-vector", sortsEveryVector).
		Subject("sorting-is-idempotent", sortingIsIdempotent)
}

// endregion 🔖️Registration
