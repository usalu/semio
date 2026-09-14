// 🐹️ Go side of the goal identifier scheme case.
package adapter

import (
	"encoding/json"
	"fmt"

	goals "github.com/usalu/semio/repo/goals"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type titleVector struct {
	Parent string `json:"parent"`
	Title  string `json:"title"`
}

type idVectorFile struct {
	Schema     string        `json:"schema"`
	Paths      []string      `json:"paths"`
	Titles     []titleVector `json:"titles"`
	References []string      `json:"references"`
}

func loadIDVectors(ctx *host.Context) (idVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://🪪️id-vectors.json")
	if err != nil {
		return idVectorFile{}, err
	}
	var file idVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func identifiersResolveToOnePlace(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	paths := make([]string, 0, len(file.Paths))
	compose := make([]string, 0, len(file.Paths))
	for _, path := range file.Paths {
		paths = append(paths, fmt.Sprintf("%s|%d|%t|%t|%t|%s|%s|%s",
			path,
			goals.GoalDepth(path),
			goals.IsRootGoal(path),
			goals.IsFirstGenGoal(path),
			goals.IsDeeperGoal(path),
			goals.GoalIDForFilesystem(path),
			goals.RootGoalID(path),
			goals.ParentGoalID(path)))
		composed := goals.GoalPathToComposeID(path)
		compose = append(compose, fmt.Sprintf("%s=%s|%s", path, composed, goals.ComposeIDToGoalPath(composed)))
	}
	titles := make([]string, 0, len(file.Titles))
	for _, vector := range file.Titles {
		titles = append(titles, fmt.Sprintf("%s+%s=%s", vector.Parent, vector.Title, goals.ComposeGoalID(vector.Parent, vector.Title)))
	}
	references := make([]string, 0, len(file.References))
	for _, reference := range file.References {
		milestone := "unparsable"
		if number, err := goals.ParseMilestoneNumber(reference); err == nil {
			milestone = fmt.Sprintf("%d", number)
		}
		issue := "unparsable"
		if number, err := goals.ParseIssueNumber(reference); err == nil {
			issue = fmt.Sprintf("%d", number)
		}
		references = append(references, fmt.Sprintf("%s=%s|%s", reference, milestone, issue))
	}
	return host.Outcome{Projection: map[string]any{
		"paths":      paths,
		"compose":    compose,
		"titles":     titles,
		"references": references,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("identifiers-resolve-to-one-place", identifiersResolveToOnePlace)
}

// endregion 🔖️Registration
