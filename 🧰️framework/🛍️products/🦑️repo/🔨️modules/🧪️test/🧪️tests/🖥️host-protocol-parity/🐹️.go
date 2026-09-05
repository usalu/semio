// 🐹️ Go side of the host protocol conformance case. Written independently of the other four
// adapters against the same frozen contract — pairwise equivalence is the whole point.
package adapter

import (
	"errors"
	"os"
	"path/filepath"
	"strings"

	host "semio.tech/repo/test"
)

// region 🔖️Scenarios

func digestAndFixtureResolution(ctx *host.Context) (host.Outcome, error) {
	vector, err := ctx.FixtureBytes("shared://📡️protocol-vector.txt")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"vectorDigest":  host.Digest(vector),
		"literalDigest": host.Digest([]byte("semio")),
		"fixtureName":   "📡️protocol-vector.txt",
		"seed":          ctx.Seed(),
		"level":         ctx.Scenario.Level,
		"steps":         len(ctx.Scenario.Steps),
	}}, nil
}

func fixtureNotInPlanIsAnError(ctx *host.Context) (host.Outcome, error) {
	_, err := ctx.Fixture("shared://this-fixture-is-not-declared")
	return host.Outcome{Projection: map[string]any{"resolverReportedFailure": err != nil}}, nil
}

func workDirectoryIsCacheLocal(ctx *host.Context) (host.Outcome, error) {
	workDir := strings.ReplaceAll(ctx.WorkDir, "\\", "/")
	_, statErr := os.Stat(filepath.Join(ctx.WorkDir, "🧾️marker.json"))
	return host.Outcome{Projection: map[string]any{
		"insideTestCache":   strings.Contains(workDir, "/.🧬semio/🦑️repo/⚡️cache/tests/"),
		"hasOwnershipMarker": !errors.Is(statErr, os.ErrNotExist),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("digest-and-fixture-resolution", digestAndFixtureResolution).
		Subject("fixture-not-in-plan-is-an-error", fixtureNotInPlanIsAnError).
		Subject("work-directory-is-cache-local", workDirectoryIsCacheLocal)
}

// endregion 🔖️Registration
