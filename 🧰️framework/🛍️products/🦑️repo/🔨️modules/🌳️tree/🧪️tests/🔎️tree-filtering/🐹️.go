// 🐹️ Go subject for the tree filtering case, written against the same frozen contract as the Rust
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

// 📥️ fixture decodes the committed filter fixture.
func fixture(ctx *host.Context) (map[string]any, error) {
	raw, err := ctx.FixtureBytes("shared://🔎️filter-vectors.json")
	if err != nil {
		return nil, err
	}
	var decoded map[string]any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// 🔣️ reencode renders a decoded JSON member back into the text a spec decoder reads.
func reencode(value any) ([]byte, error) { return json.Marshal(value) }

// 🌿️ treeOf is the filter tree the whole case works on.
func treeOf(document map[string]any) (*tree.TreeNode, error) {
	raw, err := reencode(document["tree"])
	if err != nil {
		return nil, err
	}
	spec, err := tree.ParseTreeNodeSpec(raw)
	if err != nil {
		return nil, err
	}
	return spec.ToTreeNode(), nil
}

// 🧹️ filterOf is the filter of one vector.
func filterOf(vector map[string]any) (*tree.TreeFilter, error) {
	raw, err := reencode(vector["filter"])
	if err != nil {
		return nil, err
	}
	spec, err := tree.ParseTreeFilterSpec(raw)
	if err != nil {
		return nil, err
	}
	return spec.ToFilter(), nil
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

func appliesEveryFilterVector(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	root, err := treeOf(document)
	if err != nil {
		return host.Outcome{}, err
	}
	projected := map[string]any{}
	for _, vector := range vectors(document) {
		filter, err := filterOf(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		id, _ := vector["id"].(string)
		outline := tree.TreeOutline(tree.FilterMonorepoTree(root, filter))
		if err := require(id, outline, stringList(vector, "outline")); err != nil {
			return host.Outcome{}, err
		}
		projected[id] = outline
	}
	return host.Outcome{Projection: projected}, nil
}

func absentFilterReturnsTheTree(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	root, err := treeOf(document)
	if err != nil {
		return host.Outcome{}, err
	}
	whole := tree.TreeOutline(root)
	without := tree.TreeOutline(tree.FilterMonorepoTree(root, nil))
	empty := tree.TreeOutline(tree.FilterMonorepoTree(root, (&tree.TreeFilterSpec{}).ToFilter()))
	if err := require("noFilter", without, whole); err != nil {
		return host.Outcome{}, err
	}
	if err := require("emptyFilter", empty, whole); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"noFilter": without, "emptyFilter": empty}}, nil
}

func filteringIsIdempotent(ctx *host.Context) (host.Outcome, error) {
	document, err := fixture(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	root, err := treeOf(document)
	if err != nil {
		return host.Outcome{}, err
	}
	checked := []string{}
	for _, vector := range vectors(document) {
		filter, err := filterOf(vector)
		if err != nil {
			return host.Outcome{}, err
		}
		id, _ := vector["id"].(string)
		once := tree.FilterMonorepoTree(root, filter)
		twice := tree.FilterMonorepoTree(once, filter)
		if err := require(id, tree.TreeOutline(twice), tree.TreeOutline(once)); err != nil {
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
		Subject("applies-every-filter-vector", appliesEveryFilterVector).
		Subject("absent-filter-returns-the-tree", absentFilterReturnsTheTree).
		Subject("filtering-is-idempotent", filteringIsIdempotent)
}

// endregion 🔖️Registration
