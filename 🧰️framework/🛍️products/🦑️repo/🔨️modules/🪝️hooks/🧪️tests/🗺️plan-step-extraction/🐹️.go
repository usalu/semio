// 🐹️ Go side of the plan step extraction case, folded against the same frozen plan vectors as the
// Rust adapter and the TypeScript oracle.
package adapter

import (
	"encoding/json"
	"fmt"
	"reflect"

	hooks "github.com/usalu/semio/repo/hooks"
	model "github.com/usalu/semio/repo/model"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const planVectors = "shared://🗺️plan-step-extraction/🗺️plans.json"

type planPayloadVector struct {
	ID       string          `json:"id"`
	Input    json.RawMessage `json:"input"`
	ToolArgs string          `json:"toolArgs"`
}

type planMergeVector struct {
	ID       string                      `json:"id"`
	Second   string                      `json:"second"`
	Existing []model.TicketAgentPlanStep `json:"existing"`
	Incoming []model.HookPlanStep        `json:"incoming"`
}

type planVectorFile struct {
	Payloads []planPayloadVector `json:"payloads"`
	Merges   []planMergeVector   `json:"merges"`
}

func loadPlanVectors(ctx *host.Context) (planVectorFile, error) {
	var file planVectorFile
	raw, err := ctx.FixtureBytes(planVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("plan vectors are not valid JSON: %w", err)
	}
	return file, nil
}

// 📨️payloadOf answers nothing when the fixture says the IDE sent no payload at all.
func payloadOf(raw json.RawMessage) json.RawMessage {
	if len(raw) == 0 || string(raw) == "null" {
		return nil
	}
	return raw
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyPayloadShapeYieldsTheSameSteps(ctx *host.Context) (host.Outcome, error) {
	file, err := loadPlanVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Payloads {
		projection[vector.ID] = hooks.ExtractPlanSteps(payloadOf(vector.Input), vector.ToolArgs)
	}
	return host.Outcome{Projection: projection}, nil
}

func foldingAPlanKeepsItsHistory(ctx *host.Context) (host.Outcome, error) {
	file, err := loadPlanVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Merges {
		projection[vector.ID] = hooks.MergePlanSteps(vector.Existing, vector.Incoming, vector.Second)
	}
	return host.Outcome{Projection: projection}, nil
}

func foldingTheSamePlanTwiceChangesNothing(ctx *host.Context) (host.Outcome, error) {
	file, err := loadPlanVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Merges {
		first := hooks.MergePlanSteps(vector.Existing, vector.Incoming, vector.Second)
		second := hooks.MergePlanSteps(first, vector.Incoming, "2099-01-01T00:00:00Z")
		projection[vector.ID] = map[string]any{"stable": reflect.DeepEqual(first, second), "second": second}
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-payload-shape-yields-the-same-steps", everyPayloadShapeYieldsTheSameSteps).
		Subject("folding-a-plan-keeps-its-history", foldingAPlanKeepsItsHistory).
		Subject("folding-the-same-plan-twice-changes-nothing", foldingTheSamePlanTwiceChangesNothing)
}

// endregion 🔖️Registration
