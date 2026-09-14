// 🐹️ Go side of the runner-detection case: the manifest order and the JavaScript runner choice.
package adapter

import (
	"errors"
	"fmt"
	"strings"

	testrunner "github.com/usalu/semio/repo/testrunner"
	host "semio.tech/repo/test"
)

// region 🔖️Support

func vectors(ctx *host.Context) (testrunner.DetectionVectors, error) {
	source, err := ctx.FixtureBytes("shared://🧭️detection-vectors.json")
	if err != nil {
		return testrunner.DetectionVectors{}, err
	}
	return testrunner.ParseDetectionVectors(source)
}

func jsArgv(fixture *testrunner.DetectionVectors, bundleRoot string, filter string) []string {
	_, args := fixture.Snapshot.DetectJSTestRunner(fixture.Snapshot.Absolute(bundleRoot), filter)
	return args
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

func manifestSelectsTheLanguage(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	wrong := []string{}
	for _, vector := range fixture.Vectors {
		language := fixture.Snapshot.DetectBundleLanguage(vector.BundleRoot)
		if language != vector.ExpectedLanguage {
			wrong = append(wrong, fmt.Sprintf("%s: detected %q, vector declares %q", vector.ID, language, vector.ExpectedLanguage))
		}
		rows = append(rows, map[string]any{"id": vector.ID, "language": language})
	}
	if len(wrong) > 0 {
		return host.Outcome{}, errors.New(strings.Join(wrong, "; "))
	}
	return host.Outcome{Projection: map[string]any{"detected": rows}}, nil
}

func javascriptManifestSelectsTheRunner(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := []any{}
	wrong := []string{}
	for _, vector := range fixture.Vectors {
		if vector.ExpectedArgv == nil {
			continue
		}
		runner, args := fixture.Snapshot.DetectJSTestRunner(fixture.Snapshot.Absolute(vector.BundleRoot), vector.TestFilter)
		if !equalStrings(args, *vector.ExpectedArgv) {
			wrong = append(wrong, fmt.Sprintf("%s: detected %v, vector declares %v", vector.ID, args, *vector.ExpectedArgv))
		}
		rows = append(rows, map[string]any{"id": vector.ID, "program": runner.Program(), "args": args})
	}
	if len(wrong) > 0 {
		return host.Outcome{}, errors.New(strings.Join(wrong, "; "))
	}
	return host.Outcome{Projection: map[string]any{"runners": rows}}, nil
}

func detectionIsIdempotentAndFilterOnlyAppends(ctx *host.Context) (host.Outcome, error) {
	fixture, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	stable, appends := true, true
	for _, vector := range fixture.Vectors {
		first := fixture.Snapshot.DetectBundleLanguage(vector.BundleRoot)
		second := fixture.Snapshot.DetectBundleLanguage(vector.BundleRoot)
		stable = stable && first == second
		bare := jsArgv(&fixture, vector.BundleRoot, "")
		filtered := jsArgv(&fixture, vector.BundleRoot, "renders")
		appends = appends && len(filtered) >= len(bare) && equalStrings(filtered[:len(bare)], bare)
	}
	if !stable || !appends {
		return host.Outcome{}, fmt.Errorf("detection stability %t, filter-appends law %t", stable, appends)
	}
	projection := map[string]any{"detectionIsStable": stable, "filterOnlyAppends": appends, "vectors": len(fixture.Vectors)}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("manifest-selects-the-language", manifestSelectsTheLanguage).
		Subject("javascript-manifest-selects-the-runner", javascriptManifestSelectsTheRunner).
		Subject("detection-is-idempotent-and-filter-only-appends", detectionIsIdempotentAndFilterOnlyAppends)
}

// endregion 🔖️Registration
