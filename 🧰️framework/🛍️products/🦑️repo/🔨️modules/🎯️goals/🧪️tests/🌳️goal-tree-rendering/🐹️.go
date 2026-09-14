// 🐹️ Go side of the goal tree rendering case.
package adapter

import (
	"encoding/json"
	"fmt"

	goals "github.com/usalu/semio/repo/goals"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type treeVectorFile struct {
	Schema  string             `json:"schema"`
	Goals   []goals.GoalSeed   `json:"goals"`
	Tickets []goals.TicketSeed `json:"tickets"`
}

func loadTreeVectors(ctx *host.Context) (treeVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://🌳️tree-vectors.json")
	if err != nil {
		return treeVectorFile{}, err
	}
	var file treeVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func theForestNestsAndSorts(ctx *host.Context) (host.Outcome, error) {
	file, err := loadTreeVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	roots := goals.BuildGoalTree(file.Goals, file.Tickets)
	counts := make([]string, 0, len(roots))
	for _, root := range roots {
		counts = append(counts, fmt.Sprintf("%s|%d|%d", root.Title, goals.CountOpenSubgoals(root), goals.CountOpenTickets(root)))
	}
	return host.Outcome{Projection: map[string]any{
		"text":     goals.RenderGoalTree(roots, goals.TreeFormatText, goals.PlainTreeLines{}),
		"markdown": goals.RenderGoalTree(roots, goals.TreeFormatMarkdown, goals.PlainTreeLines{}),
		"counts":   counts,
	}}, nil
}

func goalOrderDoesNotReachTheRendering(ctx *host.Context) (host.Outcome, error) {
	file, err := loadTreeVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	forward := goals.RenderGoalTree(goals.BuildGoalTree(file.Goals, file.Tickets), goals.TreeFormatText, goals.PlainTreeLines{})
	reversed := make([]goals.GoalSeed, 0, len(file.Goals))
	for index := len(file.Goals) - 1; index >= 0; index-- {
		reversed = append(reversed, file.Goals[index])
	}
	backward := goals.RenderGoalTree(goals.BuildGoalTree(reversed, file.Tickets), goals.TreeFormatText, goals.PlainTreeLines{})
	return host.Outcome{Projection: map[string]any{"stable": forward == backward, "rendering": forward}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-forest-nests-and-sorts", theForestNestsAndSorts).
		Subject("goal-order-does-not-reach-the-rendering", goalOrderDoesNotReachTheRendering)
}

// endregion 🔖️Registration
