// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Lines-of-code models, classification, and snapshot composition.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// 📈️locDefaultLanguageList: internal scan + git line classification.
const locDefaultBranch = "⛳️wip"

// 🧮️locAggCode/locAggMarkup/locAggData/locAggTotal: synthetic aggregate row labels for `loc` tables.
const (
	locAggCode   = "Code"
	locAggMarkup = "Markup"
	locAggData   = "Data"
	locAggTotal  = "Total"
)

// 💿️LocLangStats holds the data fields for a line-of-code per-language metrics record.
// 📊️LocLangStats: per-row LOC, % of tree Total, % of branch WIP churn, and git delta sums.
type LocLangStats struct {
	Loc                 int      `json:"loc"`
	Percent             float64  `json:"percent"`
	SincePrevLocPercent *float64 `json:"since_prev_loc_percent,omitempty"`
	WipPercent          float64  `json:"wip_percent"`
	Edited              int      `json:"edited"`
	Added               int      `json:"added"`
	Removed             int      `json:"removed"`
}

// 💿️LocReport: JSON payload for `loc` output and rendering.
type LocReport struct {
	Snapshot       map[string]LocLangStats            `json:"snapshot"`
	ByContributors map[string]map[string]LocLangStats `json:"byContributors,omitempty"`
	History        []LocHistoryEntry                  `json:"history,omitempty"`
	Branch         string                             `json:"branch,omitempty"`
}

// 💿️LocHistoryEntry: one commit step on the time series; Languages xor ByContributors.
type LocHistoryEntry struct {
	SHA            string                             `json:"sha"`
	Date           string                             `json:"date"`
	Author         string                             `json:"author,omitempty"`
	Languages      map[string]LocLangStats            `json:"languages,omitempty"`
	ByContributors map[string]map[string]LocLangStats `json:"byContributors,omitempty"`
}

// 💿️locRawCommit holds a single parsed commit and its per-language line deltas.
type locRawCommit struct {
	SHA        string
	WhenUnix   int64
	Author     string
	AuthorMail string
	Delta      map[string]locPair
}

// 💿️locPair: added/removed in one file change.
type locPair struct{ Added, Removed int }

// 🔧️locContributorAlias maps git author to canonical id via first-author resolution.
// 👤️ locContributorAlias resolves first-author to contributor alias.
func locContributorAlias(name, email string) string {
	author := strings.TrimSpace(name)
	em := strings.TrimSpace(email)
	if em != "" {
		author = fmt.Sprintf("%s <%s>", author, em)
	}
	if author == "" {
		return "unknown"
	}
	return FindAndUpdateContributor(author)
}

// 🪷️ locPathSkipped returns true for paths that must not be counted (.🧬semio, gitignore).
// 🗂️ locPathSkipped filters repo-internal and gitignored relative paths.
func locPathSkipped(relPath string) bool {
	rel := normalizeRepoPath(relPath)
	rel = filepath.ToSlash(rel)
	if rel == "" {
		return true
	}
	if rel == ".🧬semio" || strings.HasPrefix(rel, ".🧬semio/") {
		return true
	}
	return isIgnoredByGitignore(rel)
}

// 🫥️ locPathHasHiddenSegment returns true when any path segment starts with "." (excluding "." / "..").
func locPathHasHiddenSegment(relPath string) bool {
	rel := filepath.ToSlash(normalizeRepoPath(relPath))
	for _, seg := range strings.Split(rel, "/") {
		if seg == "" || seg == "." || seg == ".." {
			continue
		}
		if strings.HasPrefix(seg, ".") {
			return true
		}
	}
	return false
}

// 🗂️ locPathSkippedForLoc combines gitignore/.🧬semio skips with hidden-directory-segment skips for `loc`.
func locPathSkippedForLoc(relPath string) bool {
	if locPathSkipped(relPath) {
		return true
	}
	return locPathHasHiddenSegment(relPath)
}

// 🏷️ locClassifyLocBucket maps a path extension to a `loc` bucket key (code language, Markup, Data) or "".
func locClassifyLocBucket(path string) string {
	ext := strings.ToLower(filepath.Ext(path))
	switch ext {
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
		return locAggMarkup
	case ".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules":
		return locAggData
	default:
		return ""
	}
}

// 🏷️ locClassifyForNumstat returns a bucket key when the path is allowed by the active `loc` weight map.
func locClassifyForNumstat(path string, w map[string]bool) string {
	b := locClassifyLocBucket(path)
	if b == "" || !w[b] {
		return ""
	}
	return b
}

// 🏷️ locClassifyLanguage classifies a repo-relative path when the bucket is enabled in langs, else "".
func locClassifyLanguage(path string, langs map[string]bool) string {
	return locClassifyForNumstat(path, langs)
}

// 🧾️ locMakeNumstatLangSet extends the `--languages` code set with Markup and Data buckets for git deltas.
func locMakeNumstatLangSet(languages []string) map[string]bool {
	m := locMakeLangSet(languages)
	m[locAggMarkup] = true
	m[locAggData] = true
	return m
}

func locMakeLangSet(languages []string) map[string]bool {
	m := make(map[string]bool, len(languages))
	for _, l := range languages {
		m[strings.TrimSpace(l)] = true
	}
	return m
}

func locZeroSnapshot(langs map[string]bool) map[string]LocLangStats {
	s := make(map[string]LocLangStats)
	for l := range langs {
		s[l] = LocLangStats{}
	}
	return s
}

// 🔧️locCumulativePair holds running added/removed per key.
// 📊️ locCumulativePair: running sums for a dimension (language or contributor+language).
type locCumulativePair struct {
	Added, Removed int
}

func locCumulativeFromRaw(commits []locRawCommit, languages []string, byContrib bool, contribFilter string) (map[string]locCumulativePair, map[string]map[string]locCumulativePair, error) {
	langW := locMakeNumstatLangSet(languages)
	cum := make(map[string]locCumulativePair)
	for l := range langW {
		cum[l] = locCumulativePair{}
	}
	var byCont map[string]map[string]locCumulativePair
	if byContrib {
		byCont = make(map[string]map[string]locCumulativePair)
	}
	filt := strings.TrimSpace(contribFilter)
	for _, c := range commits {
		alias := locContributorAlias(c.Author, c.AuthorMail)
		if filt != "" && !strings.EqualFold(filt, alias) {
			continue
		}
		if byContrib {
			if byCont[alias] == nil {
				byCont[alias] = make(map[string]locCumulativePair)
				for l := range langW {
					byCont[alias][l] = locCumulativePair{}
				}
			}
		}
		for l, p := range c.Delta {
			if !langW[l] {
				continue
			}
			agg := cum[l]
			agg.Added += p.Added
			agg.Removed += p.Removed
			cum[l] = agg
			if byContrib {
				agg2 := byCont[alias][l]
				agg2.Added += p.Added
				agg2.Removed += p.Removed
				byCont[alias][l] = agg2
			}
		}
	}
	return cum, byCont, nil
}

// 📏️ locPhysicalLineCount counts newline-terminated lines like wc -l (non-empty files have ≥1 line).
func locPhysicalLineCount(data []byte) int {
	if len(data) == 0 {
		return 0
	}
	return bytes.Count(data, []byte("\n")) + 1
}

// 🧮️ locCountJSONKeys counts JSON object keys recursively (each key is one logical LOC unit for Data JSON).
func locCountJSONKeys(v any) int {
	switch t := v.(type) {
	case map[string]any:
		n := len(t)
		for _, vv := range t {
			n += locCountJSONKeys(vv)
		}
		return n
	case []any:
		var n int
		for _, vv := range t {
			n += locCountJSONKeys(vv)
		}
		return n
	default:
		return 0
	}
}

// 🧮️ locJSONKeyCount parses JSON and counts object keys; returns 0 when not valid JSON.
func locJSONKeyCount(data []byte) int {
	d := bytes.TrimSpace(data)
	if len(d) == 0 {
		return 0
	}
	var v any
	if err := json.Unmarshal(d, &v); err != nil {
		return 0
	}
	return locCountJSONKeys(v)
}

// 🧮️ locCountBucketLoc returns LOC for a tracked file body using Data JSON key rules vs physical lines.
func locCountBucketLoc(path string, data []byte) int {
	ext := strings.ToLower(filepath.Ext(path))
	switch ext {
	case ".json", ".jsonc":
		k := locJSONKeyCount(data)
		if k > 0 {
			return k
		}
		if len(bytes.TrimSpace(data)) == 0 {
			return 0
		}
		return locPhysicalLineCount(data)
	default:
		return locPhysicalLineCount(data)
	}
}

// 🧾️ locGitListTrackedPaths lists repo-relative paths at HEAD (tracked) or at ref (git tree).
func locGitListTrackedPaths(repo, ref string) ([]string, error) {
	var args []string
	if ref == "" {
		args = []string{"-C", repo, "ls-files", "-z"}
	} else {
		args = []string{"-C", repo, "ls-tree", "-r", "--name-only", "-z", ref}
	}
	stdout, stderr, code := ExecCommand("git", args, repo)
	if code != 0 {
		return nil, fmt.Errorf("git paths: %s", strings.TrimSpace(stderr))
	}
	var out []string
	for _, p := range strings.Split(stdout, "\x00") {
		if p == "" {
			continue
		}
		out = append(out, filepath.ToSlash(p))
	}
	return out, nil
}

// 🧾️ locGitReadTrackedBytes reads tracked file bytes at HEAD or at ref via git show.
func locGitReadTrackedBytes(repo, ref, relPath string) ([]byte, error) {
	rel := filepath.ToSlash(relPath)
	if ref == "" {
		fp := filepath.Join(repo, filepath.FromSlash(rel))
		return os.ReadFile(fp)
	}
	spec := ref + ":" + rel
	stdout, stderr, code := ExecCommand("git", []string{"-C", repo, "show", spec}, repo)
	if code != 0 {
		return nil, fmt.Errorf("git show: %s", strings.TrimSpace(stderr))
	}
	return []byte(stdout), nil
}

// 🧮️ locGitSnapshotLocCounts walks tracked paths and sums LOC per `loc` bucket (no cloc).
func locGitSnapshotLocCounts(repo, ref string, languages []string) (map[string]int, error) {
	w := locMakeNumstatLangSet(languages)
	paths, err := locGitListTrackedPaths(repo, ref)
	if err != nil {
		return nil, err
	}
	counts := make(map[string]int, len(w))
	for k := range w {
		counts[k] = 0
	}
	for _, rel := range paths {
		if locPathSkippedForLoc(rel) {
			continue
		}
		bucket := locClassifyForNumstat(rel, w)
		if bucket == "" {
			continue
		}
		data, err := locGitReadTrackedBytes(repo, ref, rel)
		if err != nil {
			continue
		}
		if len(data) > 0 && bytes.IndexByte(data, 0) >= 0 {
			continue
		}
		counts[bucket] += locCountBucketLoc(rel, data)
	}
	return counts, nil
}

// 🧾️ locZeroScanCounts returns zero scan counts for every numstat bucket (contributor-only rows).
func locZeroScanCounts(languages []string) map[string]int {
	w := locMakeNumstatLangSet(languages)
	z := make(map[string]int, len(w))
	for k := range w {
		z[k] = 0
	}
	return z
}

// 🧩️ locStatFromPairAndScan builds one row from scan LOC and cumulative git pairs.
func locStatFromPairAndScan(c map[string]locCumulativePair, scanned map[string]int, key string) LocLangStats {
	st := LocLangStats{}
	if scanned != nil {
		st.Loc = scanned[key]
	}
	if p, ok := c[key]; ok {
		st.Added = p.Added
		st.Removed = p.Removed
		st.Edited = p.Added + p.Removed
	}
	return st
}

// 🧮️ locApplyPercents sets Percent from Total.loc as denominator (Total row is 100%).
func locApplyPercents(rows map[string]LocLangStats) {
	denom := rows[locAggTotal].Loc
	if denom <= 0 {
		for k, s := range rows {
			s.Percent = 0
			rows[k] = s
		}
		return
	}
	for k, s := range rows {
		if k == locAggTotal {
			s.Percent = 100
		} else {
			s.Percent = math.Round(10000*float64(s.Loc)/float64(denom)) / 100
		}
		rows[k] = s
	}
}

// 📊️ locSumEditedPairs sums added+removed across all weighted buckets (branch WIP impulse on the logged ref).
func locSumEditedPairs(m map[string]locCumulativePair, w map[string]bool) int {
	var t int
	for l := range w {
		if p, ok := m[l]; ok {
			t += p.Added + p.Removed
		}
	}
	return t
}

// 🧮️ locApplyWipPercents sets WipPercent from edited-line churn: each row vs globalDenom (0 = use Total.Edited).
func locApplyWipPercents(rows map[string]LocLangStats, globalDenom int) {
	denom := globalDenom
	if denom <= 0 {
		denom = rows[locAggTotal].Edited
	}
	if denom <= 0 {
		for k, s := range rows {
			s.WipPercent = 0
			rows[k] = s
		}
		return
	}
	for k, s := range rows {
		s.WipPercent = math.Round(10000*float64(s.Edited)/float64(denom)) / 100
		rows[k] = s
	}
}

// 📊️ locComposeLocReportSnapshot merges scan counts, git deltas, and aggregate Code/Total rows with percents.
func locComposeLocReportSnapshot(c map[string]locCumulativePair, scanned map[string]int, languages []string, wipGlobalDenom int) map[string]LocLangStats {
	w := locMakeNumstatLangSet(languages)
	out := make(map[string]LocLangStats)
	codeOrder := []string{"TypeScript", "Go", "C#", "Python", "Rust"}
	var codeLoc, codeAdded, codeRemoved, codeEdited int
	for _, ln := range codeOrder {
		if !w[ln] {
			continue
		}
		st := locStatFromPairAndScan(c, scanned, ln)
		out[ln] = st
		codeLoc += st.Loc
		codeAdded += st.Added
		codeRemoved += st.Removed
		codeEdited += st.Edited
	}
	mk := locStatFromPairAndScan(c, scanned, locAggMarkup)
	out[locAggMarkup] = mk
	da := locStatFromPairAndScan(c, scanned, locAggData)
	out[locAggData] = da
	out[locAggCode] = LocLangStats{
		Loc:     codeLoc,
		Edited:  codeEdited,
		Added:   codeAdded,
		Removed: codeRemoved,
	}
	totalLoc := codeLoc + mk.Loc + da.Loc
	totalEdited := codeEdited + mk.Edited + da.Edited
	totalAdded := codeAdded + mk.Added + da.Added
	totalRemoved := codeRemoved + mk.Removed + da.Removed
	out[locAggTotal] = LocLangStats{
		Loc:     totalLoc,
		Edited:  totalEdited,
		Added:   totalAdded,
		Removed: totalRemoved,
	}
	locApplyPercents(out)
	locApplyWipPercents(out, wipGlobalDenom)
	return out
}

// 📶️ locSortedRowKeys returns row names sorted by LOC descending with Total last.
func locSortedRowKeys(rows map[string]LocLangStats) []string {
	var ks []string
	for k := range rows {
		if k == locAggTotal {
			continue
		}
		ks = append(ks, k)
	}
	sort.Slice(ks, func(i, j int) bool {
		li := rows[ks[i]].Loc
		lj := rows[ks[j]].Loc
		if li != lj {
			return li > lj
		}
		return ks[i] < ks[j]
	})
	ks = append(ks, locAggTotal)
	return ks
}

// 📶️ locSortedRowKeysChurn sorts detail rows by edited (git churn) descending; Total last (delta-only tables).
func locSortedRowKeysChurn(rows map[string]LocLangStats) []string {
	var ks []string
	for k := range rows {
		if k == locAggTotal {
			continue
		}
		ks = append(ks, k)
	}
	sort.Slice(ks, func(i, j int) bool {
		ei := rows[ks[i]].Edited
		ej := rows[ks[j]].Edited
		if ei != ej {
			return ei > ej
		}
		return ks[i] < ks[j]
	})
	ks = append(ks, locAggTotal)
	return ks
}

// 🧾️ locUseFullTreeTable is true when the table has non-zero tree LOC (snapshot / merged history); false for contributor-only deltas.
func locUseFullTreeTable(rows map[string]LocLangStats) bool {
	return rows != nil && rows[locAggTotal].Loc > 0
}
