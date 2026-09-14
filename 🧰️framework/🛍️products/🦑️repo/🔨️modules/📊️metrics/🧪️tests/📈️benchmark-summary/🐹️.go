// 🐹️ Go subject for the benchmark summary case, written against the same frozen contract as the
// Rust subject.
package adapter

import (
	"encoding/json"

	metrics "github.com/usalu/semio/repo/metrics"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// ⏱️ timings is the recorded stdout of every ecosystem, parsed in the report's column order.
func timings(ctx *host.Context) ([]metrics.BenchmarkResult, error) {
	raw, err := ctx.FixtureBytes("shared://⏱️benchmark-output.json")
	if err != nil {
		return nil, err
	}
	outputs := map[string]string{}
	if err := json.Unmarshal(raw, &outputs); err != nil {
		return nil, err
	}
	results := make([]metrics.BenchmarkResult, 0)
	for _, language := range metrics.BenchmarkLanguages {
		results = append(results, metrics.ParseBenchmarkOutput(language, outputs[language])...)
	}
	return results, nil
}

// 🔣️ projection renders any metrics value through JSON so both languages emit one shape.
func projection(value any) (host.Outcome, error) {
	encoded, err := json.Marshal(value)
	if err != nil {
		return host.Outcome{}, err
	}
	var decoded any
	if err := json.Unmarshal(encoded, &decoded); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: decoded}, nil
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func parsesBenchmarkStdout(ctx *host.Context) (host.Outcome, error) {
	results, err := timings(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(results)
}

func summarizesAndRendersCsv(ctx *host.Context) (host.Outcome, error) {
	results, err := timings(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(map[string]any{"summary": metrics.SummarizeBenchmarks(results), "csv": metrics.BenchmarkCSV(results)})
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("parses-benchmark-stdout", parsesBenchmarkStdout).
		Subject("summarizes-and-renders-csv", summarizesAndRendersCsv)
}

// endregion 🔖️Registration
