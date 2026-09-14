// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📊️ Package metrics is the repository's line-of-code and benchmark domain: the pure
// `git log --numstat` parser, the unified-LOC counters, the LocReport aggregation, UTC time
// bucketing and the benchmark timing model. Behaviour twin of the Rust crate
// `semio-framework-repo-metrics`.
//
// Everything here is a pure function over data; the only door to the outside world is
// [GitLogSource], which the CLI satisfies with [SystemGit] and the tests satisfy with a recorded
// [GitTranscript].
package metrics

// #endregion 🧲️Header

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
)

// #region 🏷️Vocabulary

// ⛳️DefaultBranch is the development branch a `loc --history` walk defaults to.
const DefaultBranch = "⛳️wip"

// 🧮️AggCode, AggMarkup, AggData and AggTotal are the synthetic aggregate row labels.
const (
	AggCode   = "Code"
	AggMarkup = "Markup"
	AggData   = "Data"
	AggTotal  = "Total"
)

// 🧬️SemioDirName is the repository's own meta directory, which is never counted.
const SemioDirName = ".🧬semio"

// 🖨️NumstatPretty is the pretty format the numstat walk asks git for.
const NumstatPretty = "COMMIT%x09%H%x09%aN%x09%aE%x09%at"

// 🗣️DefaultCodeLanguages are the code buckets a `loc` run counts, in report order.
var DefaultCodeLanguages = []string{"TypeScript", "Go", "C#", "Python", "Rust"}

// ⏱️BenchmarkLanguages are the ecosystems a benchmark sweep reports, in column order.
var BenchmarkLanguages = []string{"Typescript", "Python", "Go", "C#", "Rust"}

// 🙈️Ignorer is the ignore rule set a path is matched against; 🏠️workspace implements it.
type Ignorer interface {
	MatchesPath(path string) bool
}

// 👤️AliasFunc resolves a git author name and mail to the canonical contributor alias.
type AliasFunc func(name, email string) string

// 📜️NumstatLogArgs is the exact `git log` argument vector the numstat walk uses.
func NumstatLogArgs(gitRef string) []string {
	args := []string{"log", "--no-merges", "--numstat", "--first-parent", "--reverse", "--pretty=format:" + NumstatPretty}
	if strings.TrimSpace(gitRef) != "" {
		args = append(args, strings.TrimSpace(gitRef))
	}
	return args
}

// 📜️NumstatLogRangeArgs is the exact `git log` argument vector one chunk of the numstat walk uses.
//
// A chunk states a contiguous segment of the first-parent chain as `<older>..<newer>`, or a single
// commit for the segment that reaches the root. Every other argument is [NumstatLogArgs]', so a
// chunked walk and a whole-history walk ask git the same question about every commit.
func NumstatLogRangeArgs(commitRange string) []string {
	return append(NumstatLogArgs(""), commitRange)
}

// 🧵️NumstatLogRanges splits a first-parent chain, newest commit first, into contiguous
// `<older>..<newer>` ranges that together cover it exactly once, oldest range first.
//
// The ranges are what makes the walk parallel: `git log --numstat` reads both blobs of every changed
// file, which is the whole cost of a `loc` run over a large repository, and that cost is per commit
// and therefore divisible. Concatenating the ranges' outputs in the returned order reproduces the
// single walk byte for byte, because `--reverse` orders each range internally and the ranges are
// already in history order.
func NumstatLogRanges(chain []string, chunks int) []string {
	if len(chain) == 0 {
		return nil
	}
	if chunks < 1 {
		chunks = 1
	}
	if chunks > len(chain) {
		chunks = len(chain)
	}
	size := (len(chain) + chunks - 1) / chunks
	ranges := []string{}
	for start := 0; start < len(chain); start += size {
		end := start + size - 1
		if end >= len(chain) {
			end = len(chain) - 1
		}
		if end+1 < len(chain) {
			ranges = append(ranges, chain[end+1]+".."+chain[start])
		} else {
			ranges = append(ranges, chain[start])
		}
	}
	for left, right := 0, len(ranges)-1; left < right; left, right = left+1, right-1 {
		ranges[left], ranges[right] = ranges[right], ranges[left]
	}
	return ranges
}

// 🧵️JoinNumstatChunks joins the outputs of [NumstatLogRanges] back into one stream.
//
// `--pretty=format:` writes no trailing newline and separates commits with a blank line, so the seam
// between two ranges is exactly one newline more than what each range carries on its own.
func JoinNumstatChunks(chunks []string) string {
	joined := strings.Builder{}
	for _, chunk := range chunks {
		if chunk == "" {
			continue
		}
		if joined.Len() > 0 {
			joined.WriteString("\n")
		}
		joined.WriteString(chunk)
	}
	return joined.String()
}

// 🧾️NormalizeRepoPath normalises a path to forward slashes without a leading "./".
func NormalizeRepoPath(path string) string {
	slashed := strings.ReplaceAll(path, "\\", "/")
	return strings.TrimPrefix(slashed, "./")
}

// 🧩️extensionOf is the lowercase extension of a path's last segment.
func extensionOf(path string) string {
	base := path
	if idx := strings.LastIndexAny(base, "/\\"); idx >= 0 {
		base = base[idx+1:]
	}
	idx := strings.LastIndex(base, ".")
	if idx < 0 {
		return ""
	}
	return strings.ToLower(base[idx:])
}

// 🏷️ClassifyLocBucket maps a path extension to a `loc` bucket, or "" when it has none.
func ClassifyLocBucket(path string) string {
	switch extensionOf(path) {
	case ".ts", ".tsx", ".cts", ".mts", ".mtsx":
		return "TypeScript"
	case ".go":
		return "Go"
	case ".cs":
		return "C#"
	case ".py":
		return "Python"
	case ".rs":
		return "Rust"
	case ".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme":
		return AggMarkup
	case ".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules":
		return AggData
	default:
		return ""
	}
}

// 🏷️ClassifyLanguageFine maps a path to the fine-grained bucket the TypeScript library reports.
func ClassifyLanguageFine(path string) string {
	rel := NormalizeRepoPath(path)
	base := rel
	if idx := strings.LastIndex(base, "/"); idx >= 0 {
		base = base[idx+1:]
	}
	base = strings.ToLower(base)
	if base == "dockerfile" || strings.HasPrefix(base, "dockerfile.") {
		return "Dockerfile"
	}
	if base == "makefile" || base == "justfile" {
		return "Makefile"
	}
	switch extensionOf(rel) {
	case ".ts", ".tsx", ".cts", ".mts", ".mtsx":
		return "TypeScript"
	case ".js", ".mjs", ".cjs":
		return "JavaScript"
	case ".go":
		return "Go"
	case ".cs":
		return "C#"
	case ".py":
		return "Python"
	case ".rs":
		return "Rust"
	case ".sh", ".bash", ".zsh":
		return "Shell"
	case ".ps1":
		return "PowerShell"
	case ".css", ".scss", ".sass":
		return "CSS"
	case ".sql":
		return "SQL"
	case ".html", ".htm", ".xhtml":
		return "HTML"
	case ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx":
		return "Markdown"
	case ".tex", ".sty", ".cls", ".ltx", ".bib":
		return "TeX"
	case ".json", ".jsonc":
		return "JSON"
	case ".yaml", ".yml":
		return "YAML"
	case ".toml":
		return "TOML"
	case ".csv":
		return "CSV"
	case ".xml":
		return "XML"
	default:
		return ""
	}
}

// 🧾️MakeLangSet is the enabled code-language set, trimmed as the CLI flag delivers it.
func MakeLangSet(languages []string) map[string]bool {
	set := make(map[string]bool, len(languages))
	for _, language := range languages {
		set[strings.TrimSpace(language)] = true
	}
	return set
}

// 🧾️MakeNumstatLangSet is the enabled code languages plus the two aggregate buckets.
func MakeNumstatLangSet(languages []string) map[string]bool {
	set := MakeLangSet(languages)
	set[AggMarkup] = true
	set[AggData] = true
	return set
}

// 🏷️ClassifyForNumstat is the bucket of a path when the active weight set enables it.
func ClassifyForNumstat(path string, weights map[string]bool) string {
	bucket := ClassifyLocBucket(path)
	if bucket == "" || !weights[bucket] {
		return ""
	}
	return bucket
}

// 🫥️PathHasHiddenSegment reports whether any path segment starts with a dot.
func PathHasHiddenSegment(rel string) bool {
	for _, segment := range strings.Split(NormalizeRepoPath(rel), "/") {
		if segment == "" || segment == "." || segment == ".." {
			continue
		}
		if strings.HasPrefix(segment, ".") {
			return true
		}
	}
	return false
}

// 🧬️PathIsRepoMeta reports whether a path is the repository's own meta tree.
func PathIsRepoMeta(rel string) bool {
	normalized := NormalizeRepoPath(rel)
	return normalized == SemioDirName || strings.HasPrefix(normalized, SemioDirName+"/")
}

// 🗂️PathSkippedForLoc reports whether a path must not be counted.
func PathSkippedForLoc(rel string, ignore Ignorer) bool {
	normalized := NormalizeRepoPath(rel)
	if normalized == "" || PathIsRepoMeta(normalized) {
		return true
	}
	if ignore != nil && ignore.MatchesPath(normalized) {
		return true
	}
	return PathHasHiddenSegment(normalized)
}

// #endregion 🏷️Vocabulary

// #region 📏️Counting

// 📏️PhysicalLineCount counts newline-terminated lines; a non-empty file has at least one.
func PhysicalLineCount(data []byte) int {
	if len(data) == 0 {
		return 0
	}
	return bytes.Count(data, []byte("\n")) + 1
}

// 🧮️CountJSONKeys recursively counts object keys; arrays contribute only their members.
func CountJSONKeys(value any) int {
	switch typed := value.(type) {
	case map[string]any:
		total := len(typed)
		for _, item := range typed {
			total += CountJSONKeys(item)
		}
		return total
	case []any:
		total := 0
		for _, item := range typed {
			total += CountJSONKeys(item)
		}
		return total
	default:
		return 0
	}
}

// 🧮️JSONKeyCount is the object-key count of a document, 0 when blank or not valid JSON.
func JSONKeyCount(data []byte) int {
	trimmed := bytes.TrimSpace(data)
	if len(trimmed) == 0 {
		return 0
	}
	var value any
	if err := json.Unmarshal(trimmed, &value); err != nil {
		return 0
	}
	return CountJSONKeys(value)
}

// 📏️CountUnifiedLocForFile is the repository-wide unified LOC of one tracked file body.
func CountUnifiedLocForFile(rel string, data []byte) int {
	switch extensionOf(rel) {
	case ".json", ".jsonc":
		if keys := JSONKeyCount(data); keys > 0 {
			return keys
		}
		if len(bytes.TrimSpace(data)) == 0 {
			return 0
		}
		return PhysicalLineCount(data)
	default:
		return PhysicalLineCount(data)
	}
}

// #endregion 📏️Counting

// #region 🧩️Numstat

// 🟨️LinePair holds added and removed line counts.
type LinePair struct {
	Added   int64 `json:"added"`
	Removed int64 `json:"removed"`
}

// 🧩️FileDelta is one file entry of a numstat commit block.
type FileDelta struct {
	Path       string `json:"path"`
	RenameFrom string `json:"rename_from,omitempty"`
	Added      int64  `json:"added"`
	Removed    int64  `json:"removed"`
	Binary     bool   `json:"binary"`
	Bucket     string `json:"bucket,omitempty"`
}

// 💿️CommitDelta is one parsed commit and the per-bucket line deltas it contributes.
type CommitDelta struct {
	SHA        string              `json:"sha"`
	WhenUnix   int64               `json:"when_unix"`
	Author     string              `json:"author"`
	AuthorMail string              `json:"author_mail"`
	Files      []FileDelta         `json:"files"`
	Delta      map[string]LinePair `json:"delta"`
}

// 🔤️UnquoteGitPath decodes the octal-escaped quoted form git writes for non-ASCII paths.
func UnquoteGitPath(raw string) string {
	trimmed := strings.TrimSpace(raw)
	if len(trimmed) < 2 || !strings.HasPrefix(trimmed, "\"") || !strings.HasSuffix(trimmed, "\"") {
		return raw
	}
	inner := []rune(trimmed[1 : len(trimmed)-1])
	out := make([]byte, 0, len(inner))
	for index := 0; index < len(inner); {
		if inner[index] != '\\' {
			out = append(out, string(inner[index])...)
			index++
			continue
		}
		index++
		if index >= len(inner) {
			break
		}
		escape := inner[index]
		index++
		switch escape {
		case 'n':
			out = append(out, '\n')
		case 't':
			out = append(out, '\t')
		case 'r':
			out = append(out, '\r')
		case 'a':
			out = append(out, 7)
		case 'b':
			out = append(out, 8)
		case 'f':
			out = append(out, 12)
		case 'v':
			out = append(out, 11)
		case '"':
			out = append(out, '"')
		case '\\':
			out = append(out, '\\')
		default:
			if escape >= '0' && escape <= '7' {
				value := int(escape - '0')
				for taken := 1; taken < 3 && index < len(inner) && inner[index] >= '0' && inner[index] <= '7'; taken++ {
					value = value*8 + int(inner[index]-'0')
					index++
				}
				out = append(out, byte(value&0xff))
				continue
			}
			out = append(out, string(escape)...)
		}
	}
	return string(out)
}

// ↩️ResolveNumstatPath splits a numstat path field into the new path and its rename source.
func ResolveNumstatPath(raw string) (string, string) {
	field := strings.TrimRight(raw, "\r")
	open := strings.Index(field, "{")
	closing := strings.Index(field, "}")
	if open >= 0 && closing > open {
		middle := field[open+1 : closing]
		if arrow := strings.Index(middle, " => "); arrow >= 0 {
			prefix := field[:open]
			suffix := field[closing+1:]
			old := collapseSlashes(prefix + middle[:arrow] + suffix)
			updated := collapseSlashes(prefix + middle[arrow+4:] + suffix)
			return UnquoteGitPath(updated), UnquoteGitPath(old)
		}
	}
	if arrow := strings.Index(field, " => "); arrow >= 0 {
		return UnquoteGitPath(strings.TrimSpace(field[arrow+4:])), UnquoteGitPath(strings.TrimSpace(field[:arrow]))
	}
	return UnquoteGitPath(field), ""
}

// 🧹️collapseSlashes collapses the empty segment a `{ => new}` rename leaves behind.
func collapseSlashes(path string) string {
	for strings.Contains(path, "//") {
		path = strings.ReplaceAll(path, "//", "/")
	}
	return path
}

// 🧩️ParseNumstatLog breaks a `git log --numstat` stream into per-commit records.
func ParseNumstatLog(stdout string, weights map[string]bool, ignore Ignorer) []CommitDelta {
	out := make([]CommitDelta, 0)
	var current *CommitDelta
	buckets := map[string]string{}
	for _, raw := range strings.Split(stdout, "\n") {
		line := strings.TrimRight(raw, "\r")
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "COMMIT") {
			fields := strings.SplitN(line, "\t", 5)
			if len(fields) < 5 {
				continue
			}
			if current != nil {
				out = append(out, *current)
			}
			when, _ := strconv.ParseInt(strings.TrimSpace(fields[4]), 10, 64)
			current = &CommitDelta{SHA: fields[1], Author: fields[2], AuthorMail: fields[3], WhenUnix: when, Files: []FileDelta{}, Delta: map[string]LinePair{}}
			continue
		}
		if current == nil {
			continue
		}
		parts := strings.SplitN(line, "\t", 3)
		if len(parts) < 3 {
			continue
		}
		binary := parts[0] == "-" || parts[1] == "-"
		added, errAdded := strconv.ParseInt(parts[0], 10, 64)
		removed, errRemoved := strconv.ParseInt(parts[1], 10, 64)
		if !binary && (errAdded != nil || errRemoved != nil) {
			continue
		}
		path, renameFrom := ResolveNumstatPath(parts[2])
		bucket := ""
		if !binary {
			remembered, seen := buckets[path]
			if !seen {
				if !PathSkippedForLoc(path, ignore) {
					remembered = ClassifyForNumstat(path, weights)
				}
				buckets[path] = remembered
			}
			bucket = remembered
		}
		if bucket != "" {
			pair := current.Delta[bucket]
			pair.Added += added
			pair.Removed += removed
			current.Delta[bucket] = pair
		}
		entry := FileDelta{Path: path, RenameFrom: renameFrom, Binary: binary, Bucket: bucket}
		if !binary {
			entry.Added = added
			entry.Removed = removed
		}
		current.Files = append(current.Files, entry)
	}
	if current != nil {
		out = append(out, *current)
	}
	return out
}

// #endregion 🧩️Numstat

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

// #region 🔌️Git Source

// 🔌️GitLogSource is the only door this package has to git.
type GitLogSource interface {
	NumstatLog(gitRef string) (string, error)
	TrackedPaths(gitRef string) ([]string, error)
	TrackedBytes(gitRef, rel string) ([]byte, error)
}

// 🖥️SystemGit is the real git, reached through os/exec.
type SystemGit struct {
	Repo string
}

// 🖥️run executes git in the bound repository and returns stdout.
func (source SystemGit) run(args []string) ([]byte, error) {
	command := exec.Command("git", args...)
	command.Dir = source.Repo
	var stdout, stderr bytes.Buffer
	command.Stdout = &stdout
	command.Stderr = &stderr
	if err := command.Run(); err != nil {
		return nil, fmt.Errorf("git %s: %s", strings.Join(args, " "), strings.TrimSpace(stderr.String()))
	}
	return stdout.Bytes(), nil
}

// 🧵️MinimumChunkedCommits is the shortest history a numstat walk splits; below it the split would
// cost more than it saves.
const MinimumChunkedCommits = 32

// 🧵️MaximumNumstatWorkers is the most concurrent `git log` processes a numstat walk starts.
const MaximumNumstatWorkers = 16

// 🧵️NumstatChunksPerWorker gives every worker several ranges, so one that draws a cheap range picks
// up another instead of idling.
const NumstatChunksPerWorker = 4

// 🧵️firstParentChain lists the first-parent chain of a ref, newest commit first, and nothing when
// git cannot name one.
func (source SystemGit) firstParentChain(gitRef string) []string {
	ref := strings.TrimSpace(gitRef)
	if ref == "" {
		ref = "HEAD"
	}
	out, err := source.run([]string{"rev-list", "--first-parent", ref})
	if err != nil {
		return nil
	}
	chain := []string{}
	for _, line := range strings.Split(string(out), "\n") {
		if trimmed := strings.TrimSpace(line); trimmed != "" {
			chain = append(chain, trimmed)
		}
	}
	return chain
}

// 🧵️numstatChunks is how many ranges a chain of that length is split into.
func numstatChunks(commits int) int {
	if commits <= MinimumChunkedCommits {
		return 1
	}
	workers := runtime.NumCPU()
	if workers < 1 {
		workers = 1
	}
	if workers > MaximumNumstatWorkers {
		workers = MaximumNumstatWorkers
	}
	chunks := workers * NumstatChunksPerWorker
	if chunks > commits {
		chunks = commits
	}
	return chunks
}

// 📜️NumstatLog is the raw `git log --numstat` stream for a ref, walked in parallel over contiguous
// ranges of the first-parent chain and joined back into the single stream the parser reads.
func (source SystemGit) NumstatLog(gitRef string) (string, error) {
	chain := source.firstParentChain(gitRef)
	chunks := numstatChunks(len(chain))
	if chunks < 2 {
		out, err := source.run(NumstatLogArgs(gitRef))
		if err != nil {
			return "", err
		}
		return string(out), nil
	}
	ranges := NumstatLogRanges(chain, chunks)
	outputs := make([]string, len(ranges))
	failures := make([]error, len(ranges))
	workers := len(ranges)
	if workers > MaximumNumstatWorkers {
		workers = MaximumNumstatWorkers
	}
	next := int64(0)
	group := sync.WaitGroup{}
	for worker := 0; worker < workers; worker++ {
		group.Add(1)
		go func() {
			defer group.Done()
			for {
				index := int(atomic.AddInt64(&next, 1)) - 1
				if index >= len(ranges) {
					return
				}
				out, err := source.run(NumstatLogRangeArgs(ranges[index]))
				outputs[index], failures[index] = string(out), err
			}
		}()
	}
	group.Wait()
	for _, failure := range failures {
		if failure != nil {
			return "", failure
		}
	}
	return JoinNumstatChunks(outputs), nil
}

// 🧾️TrackedPaths lists the repository-relative tracked paths at a ref.
func (source SystemGit) TrackedPaths(gitRef string) ([]string, error) {
	args := []string{"ls-files", "-z"}
	if strings.TrimSpace(gitRef) != "" {
		args = []string{"ls-tree", "-r", "--name-only", "-z", gitRef}
	}
	out, err := source.run(args)
	if err != nil {
		return nil, err
	}
	paths := make([]string, 0)
	for _, entry := range strings.Split(string(out), "\x00") {
		if entry == "" {
			continue
		}
		paths = append(paths, filepath.ToSlash(entry))
	}
	return paths, nil
}

// 📄️TrackedBytes reads the bytes of one tracked path at a ref.
func (source SystemGit) TrackedBytes(gitRef, rel string) ([]byte, error) {
	if strings.TrimSpace(gitRef) == "" {
		return os.ReadFile(filepath.Join(source.Repo, filepath.FromSlash(rel)))
	}
	return source.run([]string{"show", gitRef + ":" + rel})
}

// 🎞️GitTranscript is a recorded git session: exactly what the real CLI answered, keyed by ref.
type GitTranscript struct {
	ID      string              `json:"id"`
	Logs    map[string]string   `json:"logs"`
	Tracked map[string][]string `json:"tracked"`
	Blobs   map[string]string   `json:"blobs"`
}

// 🔑️TranscriptBlobKey is the blob key for a ref and path pair.
func TranscriptBlobKey(gitRef, rel string) string {
	return gitRef + "\x01" + rel
}

// 📥️ParseGitTranscript parses a transcript from its JSON encoding.
func ParseGitTranscript(text []byte) (*GitTranscript, error) {
	transcript := &GitTranscript{}
	if err := json.Unmarshal(text, transcript); err != nil {
		return nil, err
	}
	return transcript, nil
}

// 📜️NumstatLog replays the recorded log for a ref.
func (transcript *GitTranscript) NumstatLog(gitRef string) (string, error) {
	if value, ok := transcript.Logs[gitRef]; ok {
		return value, nil
	}
	return "", fmt.Errorf("transcript has no log for ref %q", gitRef)
}

// 🧾️TrackedPaths replays the recorded tracked path list for a ref.
func (transcript *GitTranscript) TrackedPaths(gitRef string) ([]string, error) {
	if value, ok := transcript.Tracked[gitRef]; ok {
		return value, nil
	}
	return nil, fmt.Errorf("transcript has no tracked path list for ref %q", gitRef)
}

// 📄️TrackedBytes replays the recorded body of one path at a ref.
func (transcript *GitTranscript) TrackedBytes(gitRef, rel string) ([]byte, error) {
	if value, ok := transcript.Blobs[TranscriptBlobKey(gitRef, rel)]; ok {
		return []byte(value), nil
	}
	return nil, fmt.Errorf("transcript has no blob for %q:%s", gitRef, rel)
}

// #endregion 🔌️Git Source

// #region 🧮️Pipeline

// 🧮️SnapshotLocCounts walks the tracked tree at a ref and sums unified LOC per bucket.
func SnapshotLocCounts(source GitLogSource, gitRef string, languages []string, ignore Ignorer) (map[string]int64, error) {
	weights := MakeNumstatLangSet(languages)
	counts := make(map[string]int64, len(weights))
	for bucket := range weights {
		counts[bucket] = 0
	}
	paths, err := source.TrackedPaths(gitRef)
	if err != nil {
		return nil, err
	}
	type consideredFile struct {
		rel    string
		bucket string
	}
	considered := []consideredFile{}
	for _, rel := range paths {
		if PathSkippedForLoc(rel, ignore) {
			continue
		}
		if bucket := ClassifyForNumstat(rel, weights); bucket != "" {
			considered = append(considered, consideredFile{rel: rel, bucket: bucket})
		}
	}
	workers := snapshotScanWorkers(len(considered))
	partials := make([]map[string]int64, workers)
	next := int64(0)
	group := sync.WaitGroup{}
	for worker := 0; worker < workers; worker++ {
		partials[worker] = map[string]int64{}
		group.Add(1)
		go func(local map[string]int64) {
			defer group.Done()
			for {
				index := int(atomic.AddInt64(&next, 1)) - 1
				if index >= len(considered) {
					return
				}
				entry := considered[index]
				data, err := source.TrackedBytes(gitRef, entry.rel)
				if err != nil {
					continue
				}
				if len(data) > 0 && bytes.IndexByte(data, 0) >= 0 {
					continue
				}
				local[entry.bucket] += int64(CountUnifiedLocForFile(entry.rel, data))
			}
		}(partials[worker])
	}
	group.Wait()
	for _, partial := range partials {
		for bucket, lines := range partial {
			counts[bucket] += lines
		}
	}
	return counts, nil
}

// 🧵️snapshotScanWorkers is how many goroutines read the tracked tree: the scan is one file read per
// entry and nothing else, so it is bounded by the disk rather than by the host's cores.
func snapshotScanWorkers(files int) int {
	if files < 64 {
		return 1
	}
	workers := runtime.NumCPU()
	if workers < 1 {
		workers = 1
	}
	if workers > MaximumNumstatWorkers {
		workers = MaximumNumstatWorkers
	}
	return workers
}

// 📅️BuildHistory assembles the per-commit history series, scanning the tree at every commit.
func BuildHistory(source GitLogSource, languages []string, byContributor bool, commits []CommitDelta, contributorFilter string, ignore Ignorer, alias AliasFunc) ([]LocHistoryEntry, error) {
	if alias == nil {
		alias = DefaultContributorAlias
	}
	weights := MakeNumstatLangSet(languages)
	running := zeroCumulative(weights)
	runningBranch := zeroCumulative(weights)
	runningByContributor := CumulativeByContributor{}
	history := make([]LocHistoryEntry, 0)
	filter := strings.TrimSpace(contributorFilter)
	for _, commit := range commits {
		for bucket, pair := range commit.Delta {
			if !weights[bucket] {
				continue
			}
			branch := runningBranch[bucket]
			branch.Added += pair.Added
			branch.Removed += pair.Removed
			runningBranch[bucket] = branch
		}
		who := alias(commit.Author, commit.AuthorMail)
		if filter != "" && !strings.EqualFold(filter, who) {
			continue
		}
		if byContributor && runningByContributor[who] == nil {
			runningByContributor[who] = zeroCumulative(weights)
		}
		for bucket, pair := range commit.Delta {
			if !weights[bucket] {
				continue
			}
			total := running[bucket]
			total.Added += pair.Added
			total.Removed += pair.Removed
			running[bucket] = total
			if byContributor {
				row := runningByContributor[who][bucket]
				row.Added += pair.Added
				row.Removed += pair.Removed
				runningByContributor[who][bucket] = row
			}
		}
		scan, err := SnapshotLocCounts(source, commit.SHA, languages, ignore)
		if err != nil {
			return nil, err
		}
		wipDenominator := SumEditedPairs(runningBranch, weights)
		entry := LocHistoryEntry{SHA: commit.SHA, Date: FormatRFC3339UTC(commit.WhenUnix), Author: who}
		if byContributor {
			entry.ByContributors = map[string]map[string]LocLangStats{who: ComposeLocReportSnapshot(runningByContributor[who], scan, languages, wipDenominator)}
		} else {
			entry.Languages = ComposeLocReportSnapshot(running, scan, languages, wipDenominator)
		}
		history = append(history, entry)
	}
	return history, nil
}

// 🎛️LocOptions is everything a `loc` run decides before any git call happens.
type LocOptions struct {
	Languages      []string `json:"languages"`
	History        bool     `json:"history"`
	ByContributors bool     `json:"by_contributors"`
	Branch         string   `json:"branch"`
	Contributor    string   `json:"contributor"`
}

// 🎛️DefaultLocOptions is the option set the `loc` verb starts from.
func DefaultLocOptions() LocOptions {
	return LocOptions{Languages: append([]string(nil), DefaultCodeLanguages...)}
}

// 📊️BuildLocReport is the whole `loc` pipeline with git behind GitLogSource.
func BuildLocReport(source GitLogSource, options LocOptions, ignore Ignorer, alias AliasFunc) (*LocReport, error) {
	if alias == nil {
		alias = DefaultContributorAlias
	}
	weights := MakeNumstatLangSet(options.Languages)
	logRef := ""
	if options.History {
		logRef = strings.TrimSpace(options.Branch)
		if logRef == "" {
			logRef = DefaultBranch
		}
	}
	stdout, err := source.NumstatLog(logRef)
	if err != nil {
		return nil, err
	}
	commits := ParseNumstatLog(stdout, weights, ignore)
	cumulativeAll, byContributor := CumulativeFromRaw(commits, options.Languages, options.ByContributors, "", alias)
	filter := strings.TrimSpace(options.Contributor)
	cumulativeForSnapshot := cumulativeAll
	if filter != "" {
		cumulativeForSnapshot, _ = CumulativeFromRaw(commits, options.Languages, false, filter, alias)
	}
	scan, err := SnapshotLocCounts(source, "", options.Languages, ignore)
	if err != nil {
		return nil, err
	}
	wipDenominator := SumEditedPairs(cumulativeAll, weights)
	report := &LocReport{Snapshot: ComposeLocReportSnapshot(cumulativeForSnapshot, scan, options.Languages, wipDenominator), Branch: logRef}
	if options.ByContributors {
		selected := byContributor
		if filter != "" && byContributor != nil {
			selected = CumulativeByContributor{}
			for alias, rows := range byContributor {
				if strings.EqualFold(alias, filter) {
					selected[alias] = rows
				}
			}
		}
		report.ByContributors = ByContributorsToSnapshot(selected, options.Languages, wipDenominator)
	}
	if options.History {
		history, err := BuildHistory(source, options.Languages, options.ByContributors, commits, filter, ignore, alias)
		if err != nil {
			return nil, err
		}
		ApplyHistoryLocSincePrev(history)
		report.History = history
	}
	return report, nil
}

// #endregion 🧮️Pipeline

// #region 📤️Render

// 📤️MarkdownTable renders a GitHub-flavoured pipe table.
func MarkdownTable(title string, rows map[string]LocLangStats, fullTree, sincePrev bool) string {
	names := SortedRowKeysChurn(rows)
	if fullTree {
		names = SortedRowKeys(rows)
	}
	var out strings.Builder
	if title != "" {
		out.WriteString("### " + title + "\n\n")
	}
	switch {
	case fullTree && sincePrev:
		out.WriteString("| Category | loc | Δ% | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	case fullTree:
		out.WriteString("| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	default:
		out.WriteString("| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: |\n")
	}
	for _, name := range names {
		row := rows[name]
		switch {
		case fullTree && sincePrev:
			delta := "—"
			if row.SincePrevLocPercent != nil {
				delta = fmt.Sprintf("%+.2f%%", *row.SincePrevLocPercent)
			}
			out.WriteString(fmt.Sprintf("| %s | %d | %s | %.2f%% | %.2f%% | %d | %d | %d |\n", name, row.Loc, delta, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		case fullTree:
			out.WriteString(fmt.Sprintf("| %s | %d | %.2f%% | %.2f%% | %d | %d | %d |\n", name, row.Loc, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		default:
			out.WriteString(fmt.Sprintf("| %s | %.2f%% | %d | %d | %d |\n", name, row.WipPercent, row.Edited, row.Added, row.Removed))
		}
	}
	return out.String()
}

// 📤️RenderMarkdown renders the whole report as GitHub-flavoured markdown.
func RenderMarkdown(report *LocReport, withHistory, byContributor bool) string {
	var out strings.Builder
	out.WriteString("## LOC\n")
	out.WriteString(MarkdownTable("Snapshot", report.Snapshot, UseFullTreeTable(report.Snapshot), false))
	out.WriteString("\n")
	if byContributor && len(report.ByContributors) > 0 {
		out.WriteString("\n## By contributor\n")
		for _, alias := range sortedKeys(report.ByContributors) {
			table := report.ByContributors[alias]
			out.WriteString(fmt.Sprintf("\n### %s\n\n", ContributorEmojiID(alias)))
			out.WriteString(MarkdownTable("", table, UseFullTreeTable(table), false))
			out.WriteString("\n")
		}
	}
	if withHistory && len(report.History) > 0 {
		out.WriteString(fmt.Sprintf("\n## History ( %s )\n", DisplayHistoryBranch(report.Branch)))
		for _, entry := range report.History {
			if byContributor {
				out.WriteString(fmt.Sprintf("\n#### %s  %s  %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date, ContributorEmojiID(entry.Author)))
				for _, alias := range sortedKeys(entry.ByContributors) {
					table := entry.ByContributors[alias]
					out.WriteString(fmt.Sprintf("\n- **%s**\n\n", ContributorEmojiID(alias)))
					out.WriteString(MarkdownTable("", table, UseFullTreeTable(table), true))
					out.WriteString("\n")
				}
			} else if entry.Languages != nil {
				out.WriteString(fmt.Sprintf("\n#### %s  %s\n\n", HistoryCheckpointLabel(entry.SHA), entry.Date))
				out.WriteString(MarkdownTable("", entry.Languages, UseFullTreeTable(entry.Languages), true))
				out.WriteString("\n")
			}
		}
	}
	return out.String()
}

// 🔤️sortedKeys is the sorted key list of a contributor-keyed table.
func sortedKeys(rows map[string]map[string]LocLangStats) []string {
	names := make([]string, 0, len(rows))
	for name := range rows {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// 📤️TextTable renders a fixed-width plain-text table.
func TextTable(title string, rows map[string]LocLangStats, fullTree, sincePrev bool) string {
	names := SortedRowKeysChurn(rows)
	if fullTree {
		names = SortedRowKeys(rows)
	}
	var out strings.Builder
	if title != "" && !strings.HasPrefix(title, "  ") {
		out.WriteString(title + "\n")
	}
	if len(names) == 0 {
		return out.String()
	}
	switch {
	case fullTree && sincePrev:
		out.WriteString(fmt.Sprintf("%-14s%8s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "Δ%", "%", "wip", "edited", "added", "removed"))
	case fullTree:
		out.WriteString(fmt.Sprintf("%-14s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "%", "wip", "edited", "added", "removed"))
	default:
		out.WriteString(fmt.Sprintf("%-14s%7s%8s%8s%8s\n", "Category", "wip", "edited", "added", "removed"))
	}
	for _, name := range names {
		row := rows[name]
		switch {
		case fullTree && sincePrev:
			delta := "    —"
			if row.SincePrevLocPercent != nil {
				delta = fmt.Sprintf("%+6.1f%%", *row.SincePrevLocPercent)
			}
			out.WriteString(fmt.Sprintf("%-14s%8d%8s%6.1f%%%6.1f%%%8d%8d%8d\n", name, row.Loc, delta, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		case fullTree:
			out.WriteString(fmt.Sprintf("%-14s%8d%6.1f%%%6.1f%%%8d%8d%8d\n", name, row.Loc, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		default:
			out.WriteString(fmt.Sprintf("%-14s%6.1f%%%8d%8d%8d\n", name, row.WipPercent, row.Edited, row.Added, row.Removed))
		}
	}
	return out.String()
}

// 📤️RenderText renders the whole report as plain text, without terminal colour.
func RenderText(report *LocReport, withHistory, byContributor bool) string {
	var out strings.Builder
	out.WriteString(TextTable("Snapshot", report.Snapshot, UseFullTreeTable(report.Snapshot), false))
	if byContributor {
		for _, alias := range sortedKeys(report.ByContributors) {
			table := report.ByContributors[alias]
			out.WriteString("\nContributor: " + ContributorEmojiID(alias) + "\n")
			out.WriteString(TextTable("", table, UseFullTreeTable(table), false))
		}
	}
	if withHistory && len(report.History) > 0 {
		out.WriteString("\nHistory: " + DisplayHistoryBranch(report.Branch) + "\n")
		for _, entry := range report.History {
			if byContributor {
				out.WriteString(fmt.Sprintf("%s  %s %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date, ContributorEmojiID(entry.Author)))
				for _, alias := range sortedKeys(entry.ByContributors) {
					table := entry.ByContributors[alias]
					out.WriteString("  " + ContributorEmojiID(alias) + "\n")
					out.WriteString(TextTable("  ", table, UseFullTreeTable(table), true))
				}
			} else if entry.Languages != nil {
				out.WriteString(fmt.Sprintf("%s  %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date))
				out.WriteString(TextTable("", entry.Languages, UseFullTreeTable(entry.Languages), true))
			}
		}
	}
	return out.String()
}

// #endregion 📤️Render

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
