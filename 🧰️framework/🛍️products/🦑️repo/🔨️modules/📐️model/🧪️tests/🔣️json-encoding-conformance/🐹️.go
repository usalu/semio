// 🐹️ Go side of the JSON encoding conformance case.
package adapter

import (
	"encoding/json"
	"fmt"

	host "semio.tech/repo/test"

	model "github.com/usalu/semio/repo/model"
	tree "github.com/usalu/semio/repo/tree"
)

// region 🔖️Domain Types

// domainTypes is the decoder table: one fresh, zero-valued destination per golden document type.
var domainTypes = map[string]func() any{
	"Statute":                         func() any { return new(model.Statute) },
	"Range":                           func() any { return new(model.Range) },
	"LineMetrics":                     func() any { return new(model.LineMetrics) },
	"CountMetrics":                    func() any { return new(model.CountMetrics) },
	"PriorityCount":                   func() any { return new(model.PriorityCount) },
	"AnalyzeMetrics":                  func() any { return new(model.AnalyzeMetrics) },
	"Package":                         func() any { return new(model.Package) },
	"Bundle":                          func() any { return new(model.Bundle) },
	"Technology":                      func() any { return new(model.Technology) },
	"Repo":                            func() any { return new(model.Repo) },
	"Folder":                          func() any { return new(model.Folder) },
	"File":                            func() any { return new(model.File) },
	"Definition":                      func() any { return new(model.Definition) },
	"Section":                         func() any { return new(model.Section) },
	"ContributorIcons":                func() any { return new(model.ContributorIcons) },
	"ContributorLink":                 func() any { return new(model.ContributorLink) },
	"ContributorTicket":               func() any { return new(model.ContributorTicket) },
	"ContributorCheckpoint":           func() any { return new(model.ContributorCheckpoint) },
	"ContributorContributionsStorage": func() any { return new(model.ContributorContributionsStorage) },
	"Contributor":                     func() any { return new(model.Contributor) },
	"Checkpoint":                      func() any { return new(model.Checkpoint) },
	"CheckpointDiffRename":            func() any { return new(model.CheckpointDiffRename) },
	"CheckpointDiffStats":             func() any { return new(model.CheckpointDiffStats) },
	"InteractionFile":                 func() any { return new(model.InteractionFile) },
	"TicketPlan":                      func() any { return new(model.TicketPlan) },
	"TicketAgentPlanStep":             func() any { return new(model.TicketAgentPlanStep) },
	"TicketAgent":                     func() any { return new(model.TicketAgent) },
	"TicketManagementData":            func() any { return new(model.TicketManagementData) },
	"Ticket":                          func() any { return new(model.Ticket) },
	"TicketData":                      func() any { return new(model.TicketData) },
	"TicketFile":                      func() any { return new(model.TicketFile) },
	"TicketFileRenamed":               func() any { return new(model.TicketFileRenamed) },
	"TicketDiffSet":                   func() any { return new(model.TicketDiffSet) },
	"TicketSectionMetrics":            func() any { return new(model.TicketSectionMetrics) },
	"TicketFileMetricsEntry":          func() any { return new(model.TicketFileMetricsEntry) },
	"GoalDates":                       func() any { return new(model.GoalDates) },
	"GoalManagementData":              func() any { return new(model.GoalManagementData) },
	"Draft":                           func() any { return new(model.Draft) },
	"Location":                        func() any { return new(model.Location) },
	"Todo":                            func() any { return new(model.Todo) },
	"Territory":                       func() any { return new(model.Territory) },
	"StatuteMeta":                     func() any { return new(model.StatuteMeta) },
	"Breach":                          func() any { return new(model.Breach) },
	"RangePosition":                   func() any { return new(model.RangePosition) },
	"FileRange":                       func() any { return new(model.FileRange) },
	"BreachFile":                      func() any { return new(model.BreachFile) },
	"BreachFolder":                    func() any { return new(model.BreachFolder) },
	"CodebaseBreach":                  func() any { return new(model.CodebaseBreach) },
	"BundleMetricsInternal":           func() any { return new(model.BundleMetricsInternal) },
	"CodebaseFile":                    func() any { return new(model.CodebaseFile) },
	"CbTreeNode":                      func() any { return new(model.CbTreeNode) },
	"Codebase":                        func() any { return new(model.Codebase) },
	"TreeNode":                        func() any { return new(tree.TreeNode) },
	"TicketNode":                      func() any { return new(model.TicketNode) },
	"GoalNode":                        func() any { return new(model.GoalNode) },
	"SemanticChange":                  func() any { return new(model.SemanticChange) },
	"DiffLines":                       func() any { return new(model.DiffLines) },
	"FileListInput":                   func() any { return new(model.FileListInput) },
	"TicketOpenInput":                 func() any { return new(model.TicketOpenInput) },
	"TicketCloseInput":                func() any { return new(model.TicketCloseInput) },
	"FilterInput":                     func() any { return new(model.FilterInput) },
}

// endregion 🔖️Domain Types

// region 🔖️Scenarios

const goldensURI = "local://🔣️goldens.json"

func goldenDocumentsRoundTrip(ctx *host.Context) (host.Outcome, error) {
	raw, err := ctx.FixtureBytes(goldensURI)
	if err != nil {
		return host.Outcome{}, err
	}
	var doc struct {
		Goldens []struct {
			Type string `json:"type"`
			JSON string `json:"json"`
		} `json:"goldens"`
	}
	if err := json.Unmarshal(raw, &doc); err != nil {
		return host.Outcome{}, err
	}
	encoded := make([]map[string]any, 0, len(doc.Goldens))
	for _, golden := range doc.Goldens {
		destination, ok := domainTypes[golden.Type]
		if !ok {
			return host.Outcome{}, fmt.Errorf("unknown domain type %s", golden.Type)
		}
		value := destination()
		if err := json.Unmarshal([]byte(golden.JSON), value); err != nil {
			return host.Outcome{}, fmt.Errorf("decoding %s: %w", golden.Type, err)
		}
		out, err := json.Marshal(value)
		if err != nil {
			return host.Outcome{}, fmt.Errorf("encoding %s: %w", golden.Type, err)
		}
		encoded = append(encoded, map[string]any{"type": golden.Type, "encoded": string(out)})
	}
	return host.Outcome{Projection: map[string]any{"encoded": encoded}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").Subject("golden-documents-round-trip", goldenDocumentsRoundTrip)
}

// endregion 🔖️Registration
