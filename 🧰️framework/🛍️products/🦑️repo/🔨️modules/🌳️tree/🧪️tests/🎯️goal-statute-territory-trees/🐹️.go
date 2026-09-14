// 🐹️ Go subject for the goal, statute and territory tree case, written against the same frozen
// contract as the Rust subject.
//
// 🚚️ The package `github.com/usalu/semio/repo/tree` is produced by the Go split of
// `💻️client/⌨️cli/🧩️component.go`; until it exports the projection API below this adapter cannot
// compile and the Go subject of this case is reported as blocked rather than passing.
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

// 🎨️ stub stands in for the CLI renderers, stated once by the monorepo tree case.
type stub struct{}

func (stub) Human(kind string, data map[string]any) string { return kind + "#" + key(data) }

func (s stub) Markdown(kind string, data map[string]any) string {
	return "- " + s.MarkdownLink(kind, data)
}

func (stub) MarkdownLink(kind string, data map[string]any) string {
	return "[" + kind + "#" + key(data) + "]"
}

// endregion 🔖️Stub

// region 🔖️Helpers

// 📥️ fixtureObject decodes a JSON fixture into a generic object keeping its raw members.
func fixtureObject(ctx *host.Context, uri string) (map[string]json.RawMessage, error) {
	raw, err := ctx.FixtureBytes(uri)
	if err != nil {
		return nil, err
	}
	var decoded map[string]json.RawMessage
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// 📥️ inputs is the committed catalog, statute list and territory forest.
func inputs(ctx *host.Context) (map[string]json.RawMessage, error) {
	return fixtureObject(ctx, "shared://🎯️goal-statute-territory.json")
}

// 📥️ expected is the committed expectations document.
func expected(ctx *host.Context) (map[string]any, error) {
	raw, err := ctx.FixtureBytes("shared://📤️goal-statute-territory-expectations.json")
	if err != nil {
		return nil, err
	}
	var decoded map[string]any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// 📥️ records is the record set the goals and tickets come from.
func records(ctx *host.Context) (*tree.MemoryTreeSource, error) {
	raw, err := ctx.FixtureBytes("shared://🌳️tree-source.json")
	if err != nil {
		return nil, err
	}
	return tree.ParseTreeSource(raw)
}

// 📜️ catalog is the catalog the statute ports are satisfied with.
func catalog(ctx *host.Context) (*tree.MemoryStatuteCatalog, error) {
	document, err := inputs(ctx)
	if err != nil {
		return nil, err
	}
	raw, ok := document["catalog"]
	if !ok {
		return nil, fmt.Errorf("the fixture states no catalog")
	}
	return tree.ParseStatuteCatalog(raw)
}

// 📜️ member is one raw member of the input document, defaulting to an empty list.
func member(document map[string]json.RawMessage, field string) []byte {
	if raw, ok := document[field]; ok {
		return raw
	}
	return []byte("[]")
}

// 🎯️ goalTree is the goal tree every goal scenario works on.
func goalTree(ctx *host.Context) ([]tree.GoalNode, error) {
	source, err := records(ctx)
	if err != nil {
		return nil, err
	}
	return tree.BuildGoalTree(source.Goals, source.Tickets), nil
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

// 📏️ lines splits a rendered document into the lines the expectation states.
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

// 🧾️ forestOutline is the outline of a node forest.
func forestOutline(forest []*tree.TreeNode) []string {
	out := []string{}
	for _, node := range forest {
		out = append(out, tree.TreeOutline(node)...)
	}
	return out
}

// 🎯️ goalOutline is the outline of a goal forest, goals before their tickets.
func goalOutline(goals []tree.GoalNode, depth int, out *[]string) {
	for _, goal := range goals {
		*out = append(*out, fmt.Sprintf("%d|goal|%s|%s", depth, goal.ID, goal.Title))
		goalOutline(goal.Children, depth+1, out)
		ticketOutline(goal.Tickets, depth+1, out)
	}
}

// 🎫️ ticketOutline is the outline of a ticket forest.
func ticketOutline(tickets []tree.TicketNode, depth int, out *[]string) {
	for _, ticket := range tickets {
		*out = append(*out, fmt.Sprintf("%d|ticket|%s|%s", depth, ticket.ID, ticket.Slug))
		ticketOutline(ticket.Children, depth+1, out)
	}
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func buildsTheGoalTree(ctx *host.Context) (host.Outcome, error) {
	forest, err := goalTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := []string{}
	goalOutline(forest, 0, &outline)
	if err := require("goalTree", outline, stringList(want, "goalTree")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func rendersTheGoalTree(ctx *host.Context) (host.Outcome, error) {
	forest, err := goalTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	text := lines(tree.RenderGoalTreeNodes(forest, tree.TreeRenderFormatText, stub{}))
	markdown := lines(tree.RenderGoalTreeNodes(forest, tree.TreeRenderFormatMarkdown, stub{}))
	if err := require("goalTreeText", text, stringList(want, "goalTreeText")); err != nil {
		return host.Outcome{}, err
	}
	if err := require("goalTreeMarkdown", markdown, stringList(want, "goalTreeMarkdown")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"text": text, "markdown": markdown}}, nil
}

func buildsTheStatuteTree(ctx *host.Context) (host.Outcome, error) {
	document, err := inputs(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	statutes, err := tree.DecodeStatutes(member(document, "statutes"))
	if err != nil {
		return host.Outcome{}, err
	}
	resolved, err := catalog(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := forestOutline(tree.BuildStatuteTree(statutes, resolved))
	if err := require("statuteTree", outline, stringList(want, "statuteTree")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func buildsTheTerritoryTree(ctx *host.Context) (host.Outcome, error) {
	document, err := inputs(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	territories, err := tree.DecodeTerritories(member(document, "territories"))
	if err != nil {
		return host.Outcome{}, err
	}
	resolved, err := catalog(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := forestOutline(tree.BuildTerritoryTree(territories, resolved))
	if err := require("territoryTree", outline, stringList(want, "territoryTree")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func groupsStatutesByEntityKind(ctx *host.Context) (host.Outcome, error) {
	document, err := inputs(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	territories, err := tree.DecodeTerritories(member(document, "territories"))
	if err != nil {
		return host.Outcome{}, err
	}
	resolved, err := catalog(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	outline := forestOutline(tree.BuildPolicyEntityKindTree(territories, resolved))
	if err := require("entityKindTree", outline, stringList(want, "entityKindTree")); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: outline}, nil
}

func countsOpenSubgoalsAndTickets(ctx *host.Context) (host.Outcome, error) {
	forest, err := goalTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	want, err := expected(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	counted := make([]string, 0, len(forest))
	for index := range forest {
		goal := &forest[index]
		counted = append(counted, fmt.Sprintf("%s|%d|%d", goal.ID, tree.CountOpenSubgoals(goal), tree.CountOpenTickets(goal)))
	}
	rows, _ := want["openSubgoals"].([]any)
	expectedCounts := make([]string, 0, len(rows))
	for _, row := range rows {
		entry, _ := row.(map[string]any)
		name, _ := entry["goal"].(string)
		subgoals, _ := entry["subgoals"].(float64)
		tickets, _ := entry["tickets"].(float64)
		expectedCounts = append(expectedCounts, fmt.Sprintf("%s|%d|%d", name, int(subgoals), int(tickets)))
	}
	if err := require("openCounts", counted, expectedCounts); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: counted}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("builds-the-goal-tree", buildsTheGoalTree).
		Subject("renders-the-goal-tree", rendersTheGoalTree).
		Subject("builds-the-statute-tree", buildsTheStatuteTree).
		Subject("builds-the-territory-tree", buildsTheTerritoryTree).
		Subject("groups-statutes-by-entity-kind", groupsStatutesByEntityKind).
		Subject("counts-open-subgoals-and-tickets", countsOpenSubgoalsAndTickets)
}

// endregion 🔖️Registration
