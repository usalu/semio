// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⏱️benchmark carries the benchmark output parser, summaries and CSV.

// #endregion 🧲️Header

package metrics

import (
	"math"
	"sort"
	"strconv"
	"strings"
)

// #region ⏱️Benchmark

// 🔶️BenchmarkResult is one timing a benchmark run reported.
type BenchmarkResult struct {
	Test string `json:"test"`
	Lang string `json:"lang"`
	Time string `json:"time"`
}

// 🔬️ParseBenchmarkOutput extracts `name,time` timing lines from one ecosystem's stdout.
func ParseBenchmarkOutput(lang, output string) []BenchmarkResult {
	results := make([]BenchmarkResult, 0)
	for _, raw := range strings.Split(output, "\n") {
		trimmed := strings.TrimSpace(raw)
		if trimmed == "" {
			continue
		}
		parts := strings.Split(trimmed, ",")
		if len(parts) != 2 || strings.Contains(parts[0], "warning") || strings.Contains(parts[0], ":") || strings.Contains(parts[0], "/") || strings.Contains(parts[0], "\\") {
			continue
		}
		results = append(results, BenchmarkResult{Test: parts[0], Lang: lang, Time: parts[1]})
	}
	return results
}

// ⏱️ParseDurationSeconds parses a timing into seconds; a bare number is already seconds.
func ParseDurationSeconds(raw string) (float64, bool) {
	text := strings.TrimSpace(raw)
	if text == "" {
		return 0, false
	}
	factor := 1.0
	switch {
	case strings.HasSuffix(text, "ms"):
		text, factor = strings.TrimSuffix(text, "ms"), 1e-3
	case strings.HasSuffix(text, "us"):
		text, factor = strings.TrimSuffix(text, "us"), 1e-6
	case strings.HasSuffix(text, "µs"):
		text, factor = strings.TrimSuffix(text, "µs"), 1e-6
	case strings.HasSuffix(text, "ns"):
		text, factor = strings.TrimSuffix(text, "ns"), 1e-9
	case strings.HasSuffix(text, "s"):
		text = strings.TrimSuffix(text, "s")
	}
	value, err := strconv.ParseFloat(strings.TrimSpace(text), 64)
	if err != nil {
		return 0, false
	}
	return value * factor, true
}

// 📊️BenchmarkRow is one summary row: a case, its timing per ecosystem and the fastest one.
type BenchmarkRow struct {
	Test    string            `json:"test"`
	Timings map[string]string `json:"timings"`
	Fastest string            `json:"fastest,omitempty"`
}

// 📊️BenchmarkSummary is the whole benchmark table.
type BenchmarkSummary struct {
	Tests     []string       `json:"tests"`
	Languages []string       `json:"languages"`
	Rows      []BenchmarkRow `json:"rows"`
}

// 📊️SummarizeBenchmarks folds raw timings into the report table.
func SummarizeBenchmarks(results []BenchmarkResult) BenchmarkSummary {
	seen := map[string]bool{}
	tests := make([]string, 0)
	for _, result := range results {
		if !seen[result.Test] {
			seen[result.Test] = true
			tests = append(tests, result.Test)
		}
	}
	sort.Strings(tests)
	languages := append([]string(nil), BenchmarkLanguages...)
	rows := make([]BenchmarkRow, 0, len(tests))
	for _, test := range tests {
		row := BenchmarkRow{Test: test, Timings: map[string]string{}}
		fastestSeconds := math.Inf(1)
		fastest := ""
		for _, language := range languages {
			for _, result := range results {
				if result.Test == test && result.Lang == language {
					row.Timings[language] = result.Time
					break
				}
			}
			if time, ok := row.Timings[language]; ok {
				if seconds, valid := ParseDurationSeconds(time); valid && (seconds < fastestSeconds || (seconds == fastestSeconds && (fastest == "" || language < fastest))) {
					fastestSeconds = seconds
					fastest = language
				}
			}
		}
		row.Fastest = fastest
		rows = append(rows, row)
	}
	return BenchmarkSummary{Tests: tests, Languages: languages, Rows: rows}
}

// ✏️BenchmarkCSV renders the benchmark report as CSV.
func BenchmarkCSV(results []BenchmarkResult) string {
	summary := SummarizeBenchmarks(results)
	var out strings.Builder
	out.WriteString("Test," + strings.Join(summary.Languages, ",") + "\n")
	for _, row := range summary.Rows {
		out.WriteString(row.Test)
		for _, language := range summary.Languages {
			out.WriteString("," + row.Timings[language])
		}
		out.WriteString("\n")
	}
	return out.String()
}

// #endregion ⏱️Benchmark
