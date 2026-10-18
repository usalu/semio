// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🕰️time carries the UTC civil time arithmetic and the time bucketing.

// #endregion 🧲️Header

package metrics

import (
	"fmt"
)

// #region 🕰️Time

// 🕰️TimeBucket is the granularity a commit stream is grouped at.
type TimeBucket string

// 🕰️The supported bucket granularities.
const (
	BucketCommit TimeBucket = "commit"
	BucketHour   TimeBucket = "hour"
	BucketDay    TimeBucket = "day"
	BucketWeek   TimeBucket = "week"
	BucketMonth  TimeBucket = "month"
	BucketYear   TimeBucket = "year"
)

// 🧮️floorDiv is Euclidean division, which negative Unix seconds need.
func floorDiv(value, divisor int64) int64 {
	quotient := value / divisor
	if value%divisor != 0 && (value < 0) != (divisor < 0) {
		quotient--
	}
	return quotient
}

// 🧮️floorMod is the non-negative remainder matching floorDiv.
func floorMod(value, divisor int64) int64 {
	return value - floorDiv(value, divisor)*divisor
}

// 🗓️CivilFromUnix is the UTC civil date of a Unix second.
func CivilFromUnix(unix int64) (int64, int64, int64) {
	days := floorDiv(unix, 86400)
	z := days + 719468
	era := floorDiv(z, 146097)
	dayOfEra := floorMod(z, 146097)
	yearOfEra := (dayOfEra - dayOfEra/1460 + dayOfEra/36524 - dayOfEra/146096) / 365
	year := yearOfEra + era*400
	dayOfYear := dayOfEra - (365*yearOfEra + yearOfEra/4 - yearOfEra/100)
	monthPrime := (5*dayOfYear + 2) / 153
	day := dayOfYear - (153*monthPrime+2)/5 + 1
	month := monthPrime + 3
	if monthPrime >= 10 {
		month = monthPrime - 9
	}
	if month <= 2 {
		year++
	}
	return year, month, day
}

// 🗓️UnixDaysFromCivil is the inverse of CivilFromUnix.
func UnixDaysFromCivil(year, month, day int64) int64 {
	if month <= 2 {
		year--
	}
	era := floorDiv(year, 400)
	yearOfEra := floorMod(year, 400)
	monthPrime := month + 9
	if month > 2 {
		monthPrime = month - 3
	}
	dayOfYear := (153*monthPrime+2)/5 + day - 1
	dayOfEra := yearOfEra*365 + yearOfEra/4 - yearOfEra/100 + dayOfYear
	return era*146097 + dayOfEra - 719468
}

// 🗓️ISOWeekday is the ISO weekday of a Unix second, Monday = 1 … Sunday = 7.
func ISOWeekday(unix int64) int64 {
	return floorMod(floorDiv(unix, 86400), 7) + 4
}

// ⏱️FormatRFC3339UTC renders a Unix second exactly as the history rows carry it.
func FormatRFC3339UTC(unix int64) string {
	year, month, day := CivilFromUnix(unix)
	secondsOfDay := floorMod(unix, 86400)
	return fmt.Sprintf("%04d-%02d-%02dT%02d:%02d:%02dZ", year, month, day, secondsOfDay/3600, (secondsOfDay%3600)/60, secondsOfDay%60)
}

// 🕰️BucketKey is the bucket key of a Unix second at one granularity.
func BucketKey(unix int64, bucket TimeBucket) string {
	year, month, day := CivilFromUnix(unix)
	switch bucket {
	case BucketHour:
		return fmt.Sprintf("%04d-%02d-%02dT%02dZ", year, month, day, floorMod(unix, 86400)/3600)
	case BucketDay:
		return fmt.Sprintf("%04d-%02d-%02d", year, month, day)
	case BucketWeek:
		weekday := (ISOWeekday(unix)-1)%7 + 1
		thursday := floorDiv(unix, 86400) - (weekday - 1) + 3
		isoYear, _, _ := CivilFromUnix(thursday * 86400)
		januaryFirst := UnixDaysFromCivil(isoYear, 1, 1)
		return fmt.Sprintf("%04d-W%02d", isoYear, (thursday-januaryFirst)/7+1)
	case BucketMonth:
		return fmt.Sprintf("%04d-%02d", year, month)
	case BucketYear:
		return fmt.Sprintf("%04d", year)
	default:
		return FormatRFC3339UTC(unix)
	}
}

// 🕰️BucketStart is the first Unix second of the bucket a timestamp falls in.
func BucketStart(unix int64, bucket TimeBucket) int64 {
	year, month, _ := CivilFromUnix(unix)
	switch bucket {
	case BucketHour:
		return unix - floorMod(unix, 3600)
	case BucketDay:
		return floorDiv(unix, 86400) * 86400
	case BucketWeek:
		weekday := (ISOWeekday(unix)-1)%7 + 1
		return (floorDiv(unix, 86400) - (weekday - 1)) * 86400
	case BucketMonth:
		return UnixDaysFromCivil(year, month, 1) * 86400
	case BucketYear:
		return UnixDaysFromCivil(year, 1, 1) * 86400
	default:
		return unix
	}
}

// 🕰️TimeBucketGroup is one time bucket with its commits and summed deltas.
type TimeBucketGroup struct {
	Key       string              `json:"key"`
	StartUnix int64               `json:"start_unix"`
	Commits   []string            `json:"commits"`
	Delta     map[string]LinePair `json:"delta"`
}

// 🕰️BucketCommits groups a commit stream into ordered time buckets.
func BucketCommits(commits []CommitDelta, bucket TimeBucket) []TimeBucketGroup {
	order := make([]string, 0)
	groups := make(map[string]*TimeBucketGroup)
	for _, commit := range commits {
		key := BucketKey(commit.WhenUnix, bucket)
		group, ok := groups[key]
		if !ok {
			group = &TimeBucketGroup{Key: key, StartUnix: BucketStart(commit.WhenUnix, bucket), Commits: []string{}, Delta: map[string]LinePair{}}
			groups[key] = group
			order = append(order, key)
		}
		group.Commits = append(group.Commits, commit.SHA)
		for name, pair := range commit.Delta {
			total := group.Delta[name]
			total.Added += pair.Added
			total.Removed += pair.Removed
			group.Delta[name] = total
		}
	}
	out := make([]TimeBucketGroup, 0, len(order))
	for _, key := range order {
		out = append(out, *groups[key])
	}
	return out
}

// #endregion 🕰️Time
