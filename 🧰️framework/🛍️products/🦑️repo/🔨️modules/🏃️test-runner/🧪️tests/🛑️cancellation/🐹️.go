// 🐹️ Go side of the cancellation case: partial outcomes, a progress stream, and refusals.
package adapter

import (
	"encoding/json"
	"fmt"

	testrunner "github.com/usalu/semio/repo/testrunner"
	host "semio.tech/repo/test"
)

// region 🔖️Support

func vectors(ctx *host.Context) (testrunner.CancellationVectors, error) {
	source, err := ctx.FixtureBytes("shared://🛑️cancellation-vectors.json")
	if err != nil {
		return testrunner.CancellationVectors{}, err
	}
	return testrunner.ParseCancellationVectors(source)
}

func decode(text string) (any, error) {
	var value any
	if err := json.Unmarshal([]byte(text), &value); err != nil {
		return nil, err
	}
	return value, nil
}

// endregion 🔖️Support

// region 🔖️Scenarios

func cancellingMidPlanKeepsCompletedOutcomes(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	runner := &testrunner.RecordedProcessRunner{Transcripts: fixture.Transcripts}
	cancellation := testrunner.NewCancellationToken()
	finished := 0
	progress := func(event testrunner.ProgressEvent) {
		if event.Event != testrunner.ProgressInvocationFinished {
			return
		}
		finished++
		if finished >= fixture.CancelAfter {
			cancellation.Cancel()
		}
	}
	report := testrunner.ExecutePlan(fixture.Plan, runner, cancellation, progress)
	if !report.Cancelled || report.Completed != fixture.CancelAfter || len(report.Outcomes) != fixture.CancelAfter || report.Total != len(fixture.Plan.Invocations) {
		return host.Outcome{}, fmt.Errorf("cancelled %t, completed %d of %d, outcomes %d", report.Cancelled, report.Completed, report.Total, len(report.Outcomes))
	}
	value, err := decode(testrunner.ReportToJSONText(report))
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{"report": value, "cancelledAfter": fixture.CancelAfter, "skipped": report.Total - report.Completed}
	return host.Outcome{Projection: projection}, nil
}

func progressIsReportedBeforeAndAfterEveryInvocation(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	runner := &testrunner.RecordedProcessRunner{Transcripts: fixture.Transcripts}
	events := []testrunner.ProgressEvent{}
	report := testrunner.ExecutePlan(fixture.Plan, runner, testrunner.NewCancellationToken(), func(event testrunner.ProgressEvent) {
		events = append(events, event)
	})
	expected := 2 + 2*len(fixture.Plan.Invocations)
	if report.Cancelled || report.Completed != len(fixture.Plan.Invocations) || len(events) != expected {
		return host.Outcome{}, fmt.Errorf("completed %d of %d, cancelled %t, %d event(s) where %d were expected", report.Completed, report.Total, report.Cancelled, len(events), expected)
	}
	value, err := decode(testrunner.ProgressToJSONText(events))
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{"events": value, "completed": report.Completed, "cancelled": report.Cancelled}
	return host.Outcome{Projection: projection}, nil
}

func aMissingTranscriptIsAProblemNotAPass(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	report := testrunner.ExecutePlan(fixture.Plan, &testrunner.RecordedProcessRunner{}, testrunner.NewCancellationToken(), nil)
	statuses := []any{}
	anyPassed := false
	notRun := true
	for _, outcome := range report.Outcomes {
		if outcome.Status == testrunner.RunStatusNotRun {
			statuses = append(statuses, "not-run")
		} else {
			statuses = append(statuses, "ran")
			notRun = false
		}
		anyPassed = anyPassed || outcome.Status == testrunner.RunStatusPassed
	}
	if len(report.Problems) != len(fixture.Plan.Invocations) || !notRun {
		return host.Outcome{}, fmt.Errorf("%d problem(s) for %d invocation(s)", len(report.Problems), len(fixture.Plan.Invocations))
	}
	problems := []any{}
	for _, problem := range report.Problems {
		problems = append(problems, problem)
	}
	projection := map[string]any{"statuses": statuses, "problems": problems, "anyPassed": anyPassed}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("cancelling-mid-plan-keeps-completed-outcomes", cancellingMidPlanKeepsCompletedOutcomes).
		Subject("progress-is-reported-before-and-after-every-invocation", progressIsReportedBeforeAndAfterEveryInvocation).
		Subject("a-missing-transcript-is-a-problem-not-a-pass", aMissingTranscriptIsAProblemNotAPass)
}

// endregion 🔖️Registration
