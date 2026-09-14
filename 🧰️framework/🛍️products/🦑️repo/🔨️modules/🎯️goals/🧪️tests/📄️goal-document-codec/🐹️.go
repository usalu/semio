// 🐹️ Go side of the goal document codec case.
package adapter

import (
	"encoding/json"
	"fmt"

	goals "github.com/usalu/semio/repo/goals"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type goalDocument struct {
	ID   string `json:"id"`
	JSON string `json:"json"`
}

type goalDocumentFile struct {
	Schema    string         `json:"schema"`
	Documents []goalDocument `json:"documents"`
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func storedDocumentsRoundTrip(ctx *host.Context) (host.Outcome, error) {
	data, err := ctx.FixtureBytes("shared://🎯️goal-documents.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var file goalDocumentFile
	if err := json.Unmarshal(data, &file); err != nil {
		return host.Outcome{}, err
	}
	documents := make([]string, 0, len(file.Documents))
	members := make([]string, 0, len(file.Documents))
	for _, entry := range file.Documents {
		var stored struct {
			Parent string `json:"parent"`
		}
		if err := json.Unmarshal([]byte(entry.JSON), &stored); err != nil {
			return host.Outcome{}, err
		}
		goal, err := goals.DecodeGoal(entry.ID, entry.JSON)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: %w", entry.ID, err)
		}
		derivedParent := goal.Parent
		// The stored `parent` is already a compose identifier, so putting it back exercises the
		// encoder's leave-it-alone branch and makes the byte claim about the real file exact.
		goal.Parent = stored.Parent
		encoded, err := goals.EncodeGoal(goal)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("%s: %w", entry.ID, err)
		}
		milestone, issue := "", ""
		if goal.Management != nil {
			milestone, issue = goal.Management.Milestone, goal.Management.Issue
		}
		documents = append(documents, fmt.Sprintf("%s=%s", entry.ID, encoded))
		members = append(members, fmt.Sprintf("%s|%s|%s|%s|%s|%s|%s|%s|%s",
			entry.ID, goal.Status, goal.LLM, goal.Client, goal.Dates.Due, derivedParent,
			stored.Parent, milestone, issue))
	}
	return host.Outcome{Projection: map[string]any{"documents": documents, "members": members}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("stored-documents-round-trip", storedDocumentsRoundTrip)
}

// endregion 🔖️Registration
