// 🐹️ Go subject for the monorepo tree build case, written against the same frozen contract as the
// Rust subject.
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

// region 🔖️Stub

// 🔑️ key is the first non-empty identifying string of a node's data, as the feature states it.
func key(data map[string]any) string {
	for _, name := range []string{"name", "title", "slug", "id", "path"} {
		if value, ok := data[name].(string); ok && value != "" {
			return value
		}
	}
	return ""
}

// 🎨️ stub stands in for 🪪️identity and the CLI renderers.
type stub struct{}

func (stub) Human(kind string, data map[string]any) string { return kind + "#" + key(data) }

func (s stub) Markdown(kind string, data map[string]any) string {
	return "- " + s.MarkdownLink(kind, data)
}

func (stub) MarkdownLink(kind string, data map[string]any) string {
	return "[" + kind + "#" + key(data) + "]"
}

func (stub) ArtifactID(kind string, data map[string]any) string {
	parent, _ := data["parentId"].(string)
	return parent + "/" + kind + ":" + key(data)
}

// endregion 🔖️Stub

// region 🔖️Helpers

// 📥️ records is the committed record set every scenario projects.
func records(ctx *host.Context) (*tree.MemoryTreeSource, error) {
	raw, err := ctx.FixtureBytes("shared://🌳️tree-source.json")
	if err != nil {
		return nil, err
	}
	return tree.ParseTreeSource(raw)
}

// 📥️ fixtureObject decodes a JSON fixture into a generic object.
func fixtureObject(ctx *host.Context, uri string) (map[string]any, error) {
	raw, err := ctx.FixtureBytes(uri)
	if err != nil {
		return nil, err
	}
	var decoded map[string]any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// 📥️ expected is the committed expectations document.
func expected(ctx *host.Context) (map[string]any, error) {
	return fixtureObject(ctx, "shared://📤️tree-build-expectations.json")
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

// 📏️ lines splits a rendered document into the lines the expectation states.
func lines(text string) []string { return strings.Split(text, "\n") }

// 🧬️ stamps flattens a stamped tree into `<kind>|<id>|<parentId>` rows.
func stamps(node *tree.TreeNode, out *[]string) {
	parent := ""
	if node.Data != nil {
		parent, _ = node.Data["parentId"].(string)
	}
	*out = append(*out, fmt.Sprintf("%s|%s|%s", node.Kind, node.ID, parent))
	for _, child := range node.Children {
		stamps(child, out)
	}
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func projectsTheRecordSet(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := tree.TreeOutline(tree.BuildMonorepoTree(source, tree.TreeBuildOptions{}))
	if err := require("outline", outline, stringList(want, "outline")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func includesSectionsWhenRequested(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := tree.TreeOutline(tree.BuildMonorepoTree(source, tree.TreeBuildOptions{IncludeSections: true}))
	if err := require("outlineWithSections", outline, stringList(want, "outlineWithSections")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func rendersTextAndMarkdown(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	built := tree.BuildMonorepoTree(source, tree.TreeBuildOptions{})
	text := lines(tree.RenderMonorepoTree(built, stub{}))
	markdown := lines(tree.RenderMonorepoTreeMarkdown(built, stub{}))
	if err := require("text", text, stringList(want, "text")); err != nil {
		return host.Outcome{}, err
	}
	if err := require("markdown", markdown, stringList(want, "markdown")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"text": text, "markdown": markdown}}, nil
}

func stampsParentArtifactIDs(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	built := tree.BuildMonorepoTree(source, tree.TreeBuildOptions{})
	tree.PropagateParentIDs(built, "", stub{})
	stamped := []string{}
	stamps(built, &stamped)
	if err := require("parentIds", stamped, stringList(want, "parentIds")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: stamped}, nil
}

func cachesByContentDigest(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	plain := tree.BuildMonorepoTree(source, tree.TreeBuildOptions{})
	sectioned := tree.BuildMonorepoTree(source, tree.TreeBuildOptions{IncludeSections: true})
	again := tree.BuildMonorepoTree(source, tree.TreeBuildOptions{})
	plainDigest := tree.TreeContentDigest(plain)
	sectionedDigest := tree.TreeContentDigest(sectioned)
	wantPlain, _ := want["contentDigest"].(string)
	wantSectioned, _ := want["contentDigestWithSections"].(string)
	if err := require("contentDigest", []string{plainDigest, sectionedDigest, tree.TreeContentDigest(again)}, []string{wantPlain, wantSectioned, wantPlain}); err != nil {
		return host.Outcome{}, err
	}
	meta := tree.TreeCacheMetaOf(plain, "fingerprint-a", false)
	stale := meta
	stale.SchemaVersion = tree.TreeCacheSchemaVersion - 1
	decisions := []string{
		fmt.Sprintf("same:%t", tree.TreeCacheIsValid(meta, "fingerprint-a", false)),
		fmt.Sprintf("otherFingerprint:%t", tree.TreeCacheIsValid(meta, "fingerprint-b", false)),
		fmt.Sprintf("otherSections:%t", tree.TreeCacheIsValid(meta, "fingerprint-a", true)),
		fmt.Sprintf("otherSchema:%t", tree.TreeCacheIsValid(stale, "fingerprint-a", false)),
	}
	if err := require("cacheDecisions", decisions, []string{"same:true", "otherFingerprint:false", "otherSections:false", "otherSchema:false"}); err != nil {
		return host.Outcome{}, err
	}
	if plainDigest == sectionedDigest {
		return host.Outcome{}, fmt.Errorf("the sectioned tree must not share the digest of the plain tree")
	}
	return host.Outcome{Projection: map[string]any{"digest": plainDigest, "digestWithSections": sectionedDigest, "decisions": decisions}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("projects-the-record-set", projectsTheRecordSet).
		Subject("includes-sections-when-requested", includesSectionsWhenRequested).
		Subject("renders-text-and-markdown", rendersTextAndMarkdown).
		Subject("stamps-parent-artifact-ids", stampsParentArtifactIDs).
		Subject("caches-by-content-digest", cachesByContentDigest)
}

// endregion 🔖️Registration
