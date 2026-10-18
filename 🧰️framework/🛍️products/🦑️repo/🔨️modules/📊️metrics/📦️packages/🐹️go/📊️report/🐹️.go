// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📊️report carries the line-of-code report model, cumulative aggregation and history statistics.

// #endregion 🧲️Header

package metrics

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

// #region 📊️Report

// 💿️LocLangStats is one row of a `loc` table.
type LocLangStats struct {
	Loc                 int64    `json:"loc"`
	Percent             float64  `json:"percent"`
	SincePrevLocPercent *float64 `json:"since_prev_loc_percent,omitempty"`
	WipPercent          float64  `json:"wip_percent"`
	Edited              int64    `json:"edited"`
	Added               int64    `json:"added"`
	Removed             int64    `json:"removed"`
}

// 💿️LocHistoryEntry is one commit step of the history series.
type LocHistoryEntry struct {
	SHA            string                             `json:"sha"`
	Date           string                             `json:"date"`
	Author         string                             `json:"author,omitempty"`
	Languages      map[string]LocLangStats            `json:"languages,omitempty"`
	ByContributors map[string]map[string]LocLangStats `json:"byContributors,omitempty"`
}

// 💿️LocReport is the whole `loc` payload.
type LocReport struct {
	Snapshot       map[string]LocLangStats            `json:"snapshot"`
	ByContributors map[string]map[string]LocLangStats `json:"byContributors,omitempty"`
	History        []LocHistoryEntry                  `json:"history,omitempty"`
	Branch         string                             `json:"branch,omitempty"`
}

// 🧮️Cumulative holds running added/removed sums keyed by bucket.
type Cumulative map[string]LinePair

// 🧮️CumulativeByContributor holds running sums keyed by alias and then by bucket.
type CumulativeByContributor map[string]Cumulative

// 👤️DefaultContributorAlias is `Name <mail>`, or "unknown" when git recorded neither.
func DefaultContributorAlias(name, email string) string {
	author := strings.TrimSpace(name)
	mail := strings.TrimSpace(email)
	combined := author
	if mail != "" {
		combined = fmt.Sprintf("%s <%s>", author, mail)
	}
	if strings.TrimSpace(combined) == "" {
		return "unknown"
	}
	return combined
}

// 🧮️zeroCumulative is a cumulative map with one zeroed entry per weighted bucket.
func zeroCumulative(weights map[string]bool) Cumulative {
	out := make(Cumulative, len(weights))
	for bucket := range weights {
		out[bucket] = LinePair{}
	}
	return out
}

// 🧮️CumulativeFromRaw folds a commit stream into cumulative sums.
func CumulativeFromRaw(commits []CommitDelta, languages []string, byContributor bool, contributorFilter string, alias AliasFunc) (Cumulative, CumulativeByContributor) {
	if alias == nil {
		alias = DefaultContributorAlias
	}
	weights := MakeNumstatLangSet(languages)
	cumulative := zeroCumulative(weights)
	var byContributors CumulativeByContributor
	if byContributor {
		byContributors = CumulativeByContributor{}
	}
	filter := strings.TrimSpace(contributorFilter)
	for _, commit := range commits {
		who := alias(commit.Author, commit.AuthorMail)
		if filter != "" && !strings.EqualFold(filter, who) {
			continue
		}
		if byContributor && byContributors[who] == nil {
			byContributors[who] = zeroCumulative(weights)
		}
		for bucket, pair := range commit.Delta {
			if !weights[bucket] {
				continue
			}
			total := cumulative[bucket]
			total.Added += pair.Added
			total.Removed += pair.Removed
			cumulative[bucket] = total
			if byContributor {
				row := byContributors[who][bucket]
				row.Added += pair.Added
				row.Removed += pair.Removed
				byContributors[who][bucket] = row
			}
		}
	}
	return cumulative, byContributors
}

// 🧩️StatFromPairAndScan builds one table row from the tree scan and the cumulative pair.
func StatFromPairAndScan(cumulative Cumulative, scanned map[string]int64, key string) LocLangStats {
	stat := LocLangStats{}
	if scanned != nil {
		stat.Loc = scanned[key]
	}
	if pair, ok := cumulative[key]; ok {
		stat.Added = pair.Added
		stat.Removed = pair.Removed
		stat.Edited = pair.Added + pair.Removed
	}
	return stat
}

// 🧮️ApplyPercents stamps `percent` with Total.loc as denominator.
func ApplyPercents(rows map[string]LocLangStats) {
	denominator := rows[AggTotal].Loc
	if denominator <= 0 {
		for key, row := range rows {
			row.Percent = 0
			rows[key] = row
		}
		return
	}
	for key, row := range rows {
		if key == AggTotal {
			row.Percent = 100
		} else {
			row.Percent = math.Round(10000*float64(row.Loc)/float64(denominator)) / 100
		}
		rows[key] = row
	}
}

// 📊️SumEditedPairs sums added + removed over every weighted bucket.
func SumEditedPairs(cumulative Cumulative, weights map[string]bool) int64 {
	var total int64
	for bucket := range weights {
		if pair, ok := cumulative[bucket]; ok {
			total += pair.Added + pair.Removed
		}
	}
	return total
}

// 🧮️ApplyWipPercents stamps `wip_percent` from edited churn.
func ApplyWipPercents(rows map[string]LocLangStats, globalDenominator int64) {
	denominator := globalDenominator
	if denominator <= 0 {
		denominator = rows[AggTotal].Edited
	}
	if denominator <= 0 {
		for key, row := range rows {
			row.WipPercent = 0
			rows[key] = row
		}
		return
	}
	for key, row := range rows {
		row.WipPercent = math.Round(10000*float64(row.Edited)/float64(denominator)) / 100
		rows[key] = row
	}
}

// 📊️ComposeLocReportSnapshot merges tree scan and git deltas into the full row set.
func ComposeLocReportSnapshot(cumulative Cumulative, scanned map[string]int64, languages []string, wipGlobalDenominator int64) map[string]LocLangStats {
	weights := MakeNumstatLangSet(languages)
	rows := make(map[string]LocLangStats)
	code := LocLangStats{}
	for _, language := range DefaultCodeLanguages {
		if !weights[language] {
			continue
		}
		stat := StatFromPairAndScan(cumulative, scanned, language)
		rows[language] = stat
		code.Loc += stat.Loc
		code.Added += stat.Added
		code.Removed += stat.Removed
		code.Edited += stat.Edited
	}
	markup := StatFromPairAndScan(cumulative, scanned, AggMarkup)
	data := StatFromPairAndScan(cumulative, scanned, AggData)
	rows[AggMarkup] = markup
	rows[AggData] = data
	rows[AggCode] = code
	rows[AggTotal] = LocLangStats{
		Loc:     code.Loc + markup.Loc + data.Loc,
		Edited:  code.Edited + markup.Edited + data.Edited,
		Added:   code.Added + markup.Added + data.Added,
		Removed: code.Removed + markup.Removed + data.Removed,
	}
	ApplyPercents(rows)
	ApplyWipPercents(rows, wipGlobalDenominator)
	return rows
}

// 📶️sortedRowsBy orders rows by one numeric key descending, name ascending, Total last.
func sortedRowsBy(rows map[string]LocLangStats, key func(LocLangStats) int64) []string {
	names := make([]string, 0, len(rows))
	for name := range rows {
		if name == AggTotal {
			continue
		}
		names = append(names, name)
	}
	sort.Slice(names, func(left, right int) bool {
		valueLeft := key(rows[names[left]])
		valueRight := key(rows[names[right]])
		if valueLeft != valueRight {
			return valueLeft > valueRight
		}
		return names[left] < names[right]
	})
	if _, ok := rows[AggTotal]; ok {
		names = append(names, AggTotal)
	}
	return names
}

// 📶️SortedRowKeys orders row names by tree LOC descending, Total last.
func SortedRowKeys(rows map[string]LocLangStats) []string {
	return sortedRowsBy(rows, func(row LocLangStats) int64 { return row.Loc })
}

// 📶️SortedRowKeysChurn orders row names by edited churn descending, Total last.
func SortedRowKeysChurn(rows map[string]LocLangStats) []string {
	return sortedRowsBy(rows, func(row LocLangStats) int64 { return row.Edited })
}

// 🧾️UseFullTreeTable reports whether a table has real tree LOC.
func UseFullTreeTable(rows map[string]LocLangStats) bool {
	return rows != nil && rows[AggTotal].Loc > 0
}

// 🧾️zeroScanCounts is a zeroed scan map for every weighted bucket.
func zeroScanCounts(languages []string) map[string]int64 {
	weights := MakeNumstatLangSet(languages)
	out := make(map[string]int64, len(weights))
	for bucket := range weights {
		out[bucket] = 0
	}
	return out
}

// 🔧️ByContributorsToSnapshot turns per-contributor pairs into tables with zero tree LOC.
func ByContributorsToSnapshot(source CumulativeByContributor, languages []string, branchWipDenominator int64) map[string]map[string]LocLangStats {
	if source == nil {
		return nil
	}
	zero := zeroScanCounts(languages)
	out := make(map[string]map[string]LocLangStats, len(source))
	for alias, pairs := range source {
		out[alias] = ComposeLocReportSnapshot(pairs, zero, languages, branchWipDenominator)
	}
	return out
}

// 📉️PctLocSincePrev is the percent change in tree LOC against the previous history row.
func PctLocSincePrev(previous, current int64) float64 {
	var value float64
	switch {
	case previous == 0 && current == 0:
		value = 0
	case previous == 0:
		value = 100
	default:
		value = 100 * float64(current-previous) / float64(previous)
	}
	return math.Round(100*value) / 100
}

// 📉️HistoryEntryStats returns the language rows of one history step.
func HistoryEntryStats(entry *LocHistoryEntry) map[string]LocLangStats {
	if entry == nil {
		return nil
	}
	if len(entry.Languages) > 0 {
		return entry.Languages
	}
	for _, rows := range entry.ByContributors {
		return rows
	}
	return nil
}

// 📉️ApplyHistoryLocSincePrev stamps every history row's SincePrevLocPercent.
func ApplyHistoryLocSincePrev(history []LocHistoryEntry) {
	var previous map[string]int64
	for index := range history {
		rows := HistoryEntryStats(&history[index])
		if rows == nil {
			continue
		}
		for name, row := range rows {
			if previous == nil {
				row.SincePrevLocPercent = nil
			} else {
				value := PctLocSincePrev(previous[name], row.Loc)
				row.SincePrevLocPercent = &value
			}
			rows[name] = row
		}
		next := make(map[string]int64, len(rows))
		for name, row := range rows {
			next[name] = row.Loc
		}
		previous = next
	}
}

// 📉️DisplayHistoryBranch maps the common spellings of the dev branch onto its emoji id.
func DisplayHistoryBranch(gitRef string) string {
	trimmed := strings.TrimSpace(gitRef)
	if trimmed == "" {
		return trimmed
	}
	short := strings.TrimPrefix(trimmed, "refs/heads/")
	if strings.EqualFold(short, "wip") || short == DefaultBranch {
		return DefaultBranch
	}
	return trimmed
}

// 📉️HistoryCheckpointLabel is the short checkpoint id of a commit.
func HistoryCheckpointLabel(sha string) string {
	runes := []rune(strings.TrimSpace(sha))
	if len(runes) > 7 {
		runes = runes[:7]
	}
	return "🔀️" + string(runes)
}

// 📉️ContributorEmojiID is the contributor entity id of an alias.
func ContributorEmojiID(alias string) string {
	return "🧑️‍💻️" + strings.ReplaceAll(strings.ToLower(strings.TrimSpace(alias)), " ", "")
}

// #endregion 📊️Report
