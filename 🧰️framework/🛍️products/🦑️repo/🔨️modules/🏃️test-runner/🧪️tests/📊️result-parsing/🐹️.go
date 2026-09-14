// 🐹️ Go side of the result-parsing case: every runner dialect folded into one outcome model.
package adapter

import (
	"encoding/json"
	"errors"
	"fmt"

	testrunner "github.com/usalu/semio/repo/testrunner"
	host "semio.tech/repo/test"
)

// region 🔖️Support

func vectors(ctx *host.Context) (testrunner.TranscriptVectors, error) {
	source, err := ctx.FixtureBytes("shared://📜️runner-transcripts.json")
	if err != nil {
		return testrunner.TranscriptVectors{}, err
	}
	return testrunner.ParseTranscriptVectors(source)
}

func outcomeJSON(outcome testrunner.TestOutcome) (any, error) {
	var value any
	if err := json.Unmarshal([]byte(testrunner.OutcomeToJSONText(outcome)), &value); err != nil {
		return nil, err
	}
	return value, nil
}

func countStatus(outcome testrunner.TestOutcome, status testrunner.TestStatus) int {
	count := 0
	for _, test := range outcome.Tests {
		if test.Status == status {
			count++
		}
	}
	return count
}

// endregion 🔖️Support

// region 🔖️Scenarios

func everyDialectFoldsIntoOneModel(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range fixture.Vectors {
		parsed := testrunner.ParseOutcome(vector.Runner, vector.Output)
		value, err := outcomeJSON(parsed)
		if err != nil {
			return host.Outcome{}, err
		}
		rows = append(rows, map[string]any{"id": vector.ID, "outcome": value})
	}
	return host.Outcome{Projection: map[string]any{"parsed": rows}}, nil
}

func totalsAreRecomputedNotTrusted(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range fixture.Vectors {
		parsed := testrunner.ParseOutcome(vector.Runner, vector.Output)
		passed := countStatus(parsed, testrunner.TestStatusPassed)
		failed := countStatus(parsed, testrunner.TestStatusFailed)
		skipped := countStatus(parsed, testrunner.TestStatusSkipped)
		agrees := passed == parsed.Totals.Passed && failed == parsed.Totals.Failed && skipped == parsed.Totals.Skipped && len(parsed.Tests) == parsed.Totals.Total
		rows = append(rows, map[string]any{
			"id":            vector.ID,
			"total":         parsed.Totals.Total,
			"passed":        parsed.Totals.Passed,
			"failed":        parsed.Totals.Failed,
			"skipped":       parsed.Totals.Skipped,
			"recountAgrees": agrees,
		})
		if !agrees {
			return host.Outcome{}, fmt.Errorf("%s: totals disagree with the parsed tests", vector.ID)
		}
	}
	return host.Outcome{Projection: map[string]any{"totals": rows}}, nil
}

func unparseableOutputIsAnEmptyRun(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	for _, vector := range fixture.Vectors {
		if vector.ID != "unparseable-output-yields-no-tests" {
			continue
		}
		parsed := testrunner.ParseOutcome(vector.Runner, vector.Output)
		if len(parsed.Tests) != 0 || parsed.Status != testrunner.RunStatusFailed || parsed.Totals.Total != 0 {
			return host.Outcome{}, fmt.Errorf("unparseable output produced %d test(s) and status %q", len(parsed.Tests), parsed.Status)
		}
		status := "not-failed"
		if parsed.Status == testrunner.RunStatusFailed {
			status = "failed"
		}
		exitStatus := 0
		if parsed.ExitStatus != nil {
			exitStatus = *parsed.ExitStatus
		}
		projection := map[string]any{"tests": len(parsed.Tests), "status": status, "total": parsed.Totals.Total, "exitStatus": exitStatus}
		return host.Outcome{Projection: projection}, nil
	}
	return host.Outcome{}, errors.New("the transcript fixture has no unparseable vector")
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-dialect-folds-into-one-model", everyDialectFoldsIntoOneModel).
		Subject("totals-are-recomputed-not-trusted", totalsAreRecomputedNotTrusted).
		Subject("unparseable-output-is-an-empty-run", unparseableOutputIsAnEmptyRun)
}

// endregion 🔖️Registration
