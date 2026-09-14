// 🐹️ Go subject for the time bucketing case, written against the same frozen contract as the
// Rust subject.
package adapter

import (
	"encoding/json"
	"strconv"

	metrics "github.com/usalu/semio/repo/metrics"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// 🕰️ granularities are the six bucket sizes, in the order the feature states them.
var granularities = []struct {
	Name   string
	Bucket metrics.TimeBucket
}{
	{"commit", metrics.BucketCommit},
	{"hour", metrics.BucketHour},
	{"day", metrics.BucketDay},
	{"week", metrics.BucketWeek},
	{"month", metrics.BucketMonth},
	{"year", metrics.BucketYear},
}

// ⏱️ instants are the timestamps the feature states.
var instants = []int64{-2208988800, 0, 951782400, 1234567890, 1767225600, 1769904000, 4102444800}

// 🧩️ commitsOf is the recorded commit stream every scenario groups.
func commitsOf(ctx *host.Context) ([]metrics.CommitDelta, error) {
	raw, err := ctx.FixtureBytes("shared://🎞️git-transcript.json")
	if err != nil {
		return nil, err
	}
	recorded, err := metrics.ParseGitTranscript(raw)
	if err != nil {
		return nil, err
	}
	return metrics.ParseNumstatLog(recorded.Logs[""], metrics.MakeNumstatLangSet(metrics.DefaultCodeLanguages), nil), nil
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

func groupsCommitsByGranularity(ctx *host.Context) (host.Outcome, error) {
	stream, err := commitsOf(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rows := map[string]any{}
	for _, granularity := range granularities {
		rows[granularity.Name] = metrics.BucketCommits(stream, granularity.Bucket)
	}
	return projection(rows)
}

func formatsUtcTimestamps(_ *host.Context) (host.Outcome, error) {
	rows := map[string]any{}
	for _, unix := range instants {
		keys := map[string]any{"rfc3339": metrics.FormatRFC3339UTC(unix)}
		for _, granularity := range granularities {
			keys[granularity.Name] = metrics.BucketKey(unix, granularity.Bucket)
			keys[granularity.Name+"Start"] = metrics.BucketStart(unix, granularity.Bucket)
		}
		rows[strconv.FormatInt(unix, 10)] = keys
	}
	return projection(rows)
}

func civilDateRoundTrips(_ *host.Context) (host.Outcome, error) {
	checked := 0
	mismatches := []int64{}
	for day := int64(-30000); day <= 30000; day += 97 {
		year, month, ofMonth := metrics.CivilFromUnix(day * 86400)
		if metrics.UnixDaysFromCivil(year, month, ofMonth) != day {
			mismatches = append(mismatches, day)
		}
		checked++
	}
	return projection(map[string]any{"checked": checked, "mismatches": mismatches})
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("groups-commits-by-granularity", groupsCommitsByGranularity).
		Subject("formats-utc-timestamps", formatsUtcTimestamps).
		Subject("civil-date-round-trips", civilDateRoundTrips)
}

// endregion 🔖️Registration
