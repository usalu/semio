// 🐹️ Go subject for the LOC aggregation case, written against the same frozen contract as the
// Rust subject and the library reference.
//
// The subject is `github.com/usalu/semio/repo/metrics`, the exported, git-free reimplementation of
// the pre-split godfile's unexported `loc*` aggregation helpers.
package adapter

import (
	"encoding/json"
	"sort"

	metrics "github.com/usalu/semio/repo/metrics"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// 🎞️ transcript is the recorded git session every scenario reads.
func transcript(ctx *host.Context) (*metrics.GitTranscript, error) {
	raw, err := ctx.FixtureBytes("shared://🎞️git-transcript.json")
	if err != nil {
		return nil, err
	}
	return metrics.ParseGitTranscript(raw)
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

// 📊️ snapshotReport is the report the last two scenarios both start from.
func snapshotReport(ctx *host.Context) (*metrics.LocReport, error) {
	recorded, err := transcript(ctx)
	if err != nil {
		return nil, err
	}
	return metrics.BuildLocReport(recorded, metrics.DefaultLocOptions(), nil, nil)
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func countsUnifiedLocPerTrackedFile(ctx *host.Context) (host.Outcome, error) {
	recorded, err := transcript(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	paths := append([]string(nil), recorded.Tracked[""]...)
	sort.Strings(paths)
	rows := map[string]int{}
	for _, path := range paths {
		body, err := recorded.TrackedBytes("", path)
		if err != nil {
			continue
		}
		rows[path] = metrics.CountUnifiedLocForFile(path, body)
	}
	return projection(rows)
}

func composesTheSnapshotTable(ctx *host.Context) (host.Outcome, error) {
	report, err := snapshotReport(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(report)
}

func historyStampsDeltaAgainstPreviousRow(ctx *host.Context) (host.Outcome, error) {
	recorded, err := transcript(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	options := metrics.DefaultLocOptions()
	options.History = true
	options.Branch = "⛳️wip"
	report, err := metrics.BuildLocReport(recorded, options, nil, nil)
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(report.History)
}

func rendersTheMarkdownSnapshotTable(ctx *host.Context) (host.Outcome, error) {
	report, err := snapshotReport(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"markdown": metrics.MarkdownTable("Snapshot", report.Snapshot, metrics.UseFullTreeTable(report.Snapshot), false),
		"order":    metrics.SortedRowKeys(report.Snapshot),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("counts-unified-loc-per-tracked-file", countsUnifiedLocPerTrackedFile).
		Subject("composes-the-snapshot-table", composesTheSnapshotTable).
		Subject("history-stamps-delta-against-previous-row", historyStampsDeltaAgainstPreviousRow).
		Subject("renders-the-markdown-snapshot-table", rendersTheMarkdownSnapshotTable)
}

// endregion 🔖️Registration
