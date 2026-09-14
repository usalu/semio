// 🐹️ Go side of the invocation-planning case: scope in, ordered argv out, nothing executed.
package adapter

import (
	"encoding/json"
	"errors"
	"fmt"
	"strings"

	testrunner "github.com/usalu/semio/repo/testrunner"
	host "semio.tech/repo/test"
)

// region 🔖️Support

func vectors(ctx *host.Context) (testrunner.PlanningVectors, error) {
	source, err := ctx.FixtureBytes("shared://🗺️planning-vectors.json")
	if err != nil {
		return testrunner.PlanningVectors{}, err
	}
	return testrunner.ParsePlanningVectors(source)
}

func planJSON(plan testrunner.InvocationPlan) (any, error) {
	var value any
	if err := json.Unmarshal([]byte(testrunner.PlanToJSONText(plan)), &value); err != nil {
		return nil, err
	}
	return value, nil
}

func planOf(fixture *testrunner.PlanningVectors, id string) testrunner.InvocationPlan {
	for _, vector := range fixture.Vectors {
		if vector.ID == id {
			return fixture.Snapshot.PlanScope(vector.Scope)
		}
	}
	return testrunner.NewInvocationPlan()
}

func equalStrings(left []string, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

// endregion 🔖️Support

// region 🔖️Scenarios

func scopePlansMatchTheFrozenArgv(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	wrong := []string{}
	for _, vector := range fixture.Vectors {
		planned := fixture.Snapshot.PlanScope(vector.Scope)
		text := testrunner.PlanToJSONText(planned)
		if vector.Expected == nil || testrunner.PlanToJSONText(*vector.Expected) != text {
			wrong = append(wrong, fmt.Sprintf("%s: planned %s", vector.ID, text))
		}
		value, err := planJSON(planned)
		if err != nil {
			return host.Outcome{}, err
		}
		rows = append(rows, map[string]any{"id": vector.ID, "plan": value})
	}
	if len(wrong) > 0 {
		return host.Outcome{}, errors.New(strings.Join(wrong, " | "))
	}
	return host.Outcome{Projection: map[string]any{"planned": rows}}, nil
}

func aScopeWithNoRunnerIsRefused(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	for _, vector := range fixture.Vectors {
		if vector.Expected == nil || len(vector.Expected.Problems) == 0 {
			continue
		}
		planned := fixture.Snapshot.PlanScope(vector.Scope)
		if !equalStrings(planned.Problems, vector.Expected.Problems) || len(planned.Invocations) != 0 {
			return host.Outcome{}, fmt.Errorf("%s: refusal is %q with %d invocation(s)", vector.ID, planned.Problems, len(planned.Invocations))
		}
		rows = append(rows, map[string]any{"id": vector.ID, "problems": planned.Problems, "invocations": len(planned.Invocations)})
	}
	if len(rows) == 0 {
		return host.Outcome{}, errors.New("the planning fixture declares no refusal vector")
	}
	return host.Outcome{Projection: map[string]any{"refused": rows}}, nil
}

func narrowingNeverWidensThePlan(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	widths := []int{}
	for _, id := range []string{"whole-repository", "technology-semio", "bundle-go", "file-go-narrows-to-its-directory"} {
		widths = append(widths, len(planOf(&fixture, id).Invocations))
	}
	monotonic := true
	for index := 1; index < len(widths); index++ {
		monotonic = monotonic && widths[index-1] >= widths[index]
	}
	repeated := testrunner.PlanToJSONText(planOf(&fixture, "whole-repository")) == testrunner.PlanToJSONText(planOf(&fixture, "whole-repository"))
	if !monotonic || !repeated {
		return host.Outcome{}, fmt.Errorf("narrowing widths %v, repeatable %t", widths, repeated)
	}
	projection := map[string]any{"widths": widths, "narrowingIsMonotonic": monotonic, "planningIsRepeatable": repeated}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("scope-plans-match-the-frozen-argv", scopePlansMatchTheFrozenArgv).
		Subject("a-scope-with-no-runner-is-refused", aScopeWithNoRunnerIsRefused).
		Subject("narrowing-never-widens-the-plan", narrowingNeverWidensThePlan)
}

// endregion 🔖️Registration
