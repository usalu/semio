// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Lines-of-code git history and cumulative statistics.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

// 🔗️ locMergeCumulativeCloc materializes LocLangStats rows (scan replaces cloc).
func locMergeCumulativeCloc(c map[string]locCumulativePair, scanned map[string]int, languages []string) map[string]LocLangStats {
	return locComposeLocReportSnapshot(c, scanned, languages, 0)
}

// 💿️LocCumulative: exported for tests; mirrors locCumulativePair from numstat.
type LocCumulative struct{ Added, Removed int }

// 🔗️ locMergeCumulativeClocEx wraps locMergeCumulativeCloc for exported LocCumulative maps.
// 🔗️ locMergeCumulativeClocEx builds LocLangStats from a LocCumulative map.
func locMergeCumulativeClocEx(c map[string]LocCumulative, clocData map[string]int, languages []string) map[string]LocLangStats {
	m := make(map[string]locCumulativePair, len(c))
	for k, v := range c {
		m[k] = locCumulativePair{Added: v.Added, Removed: v.Removed}
	}
	return locMergeCumulativeCloc(m, clocData, languages)
}

// 🔧️ locWalkGitLog returns commits oldest-first; ref empty means from HEAD, else limit to that ref.
// 📜️ locWalkGitLog runs `git log --numstat` and parses the stream.
func locWalkGitLog(repo string, languages []string, ref string) ([]locRawCommit, error) {
	// COMMIT%09%H%09%aN%09%aE%09%at
	pretty := "COMMIT%x09%H%x09%aN%x09%aE%x09%at"
	args := []string{
		"-C", repo, "log", "--no-merges", "--numstat", "--first-parent", "--reverse",
		"--pretty=format:" + pretty,
	}
	if ref != "" {
		args = append(args, ref)
	}
	stdout, stderr, code := ExecCommand("git", args, repo)
	if code != 0 {
		return nil, fmt.Errorf("git log failed: %s", strings.TrimSpace(stderr))
	}
	return locParseNumstatLog(stdout, locMakeNumstatLangSet(languages))
}

// 🧩️ locParseNumstatLog breaks numstat + COMMIT-pretty output into per-commit records.
// 🧩️ locParseNumstatLog is exported for unit tests.
func locParseNumstatLog(stdout string, langW map[string]bool) ([]locRawCommit, error) {
	var out []locRawCommit
	lines := strings.Split(stdout, "\n")
	var cur *locRawCommit
	for _, line := range lines {
		line = strings.TrimRight(line, "\r")
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "COMMIT") {
			fields := strings.SplitN(line, "\t", 5)
			if len(fields) < 5 {
				continue
			}
			if cur != nil {
				out = append(out, *cur)
			}
			sha := fields[1]
			authorName := fields[2]
			authorMail := fields[3]
			ts, _ := strconv.ParseInt(fields[4], 10, 64)
			cur = &locRawCommit{SHA: sha, Author: authorName, AuthorMail: authorMail, WhenUnix: ts, Delta: make(map[string]locPair)}
			continue
		}
		if cur == nil {
			continue
		}
		parts := strings.SplitN(line, "\t", 3)
		if len(parts) < 3 {
			continue
		}
		if parts[0] == "-" || parts[1] == "-" {
			continue
		}
		added, err1 := strconv.Atoi(parts[0])
		removed, err2 := strconv.Atoi(parts[1])
		if err1 != nil || err2 != nil {
			continue
		}
		relpath := parts[2]
		if locPathSkippedForLoc(relpath) {
			continue
		}
		lang := locClassifyForNumstat(relpath, langW)
		if lang == "" {
			continue
		}
		d := cur.Delta[lang]
		d.Added += added
		d.Removed += removed
		cur.Delta[lang] = d
	}
	if cur != nil {
		out = append(out, *cur)
	}
	return out, nil
}

// 🔧️ locByContributorsToSnapshot builds a contributor -> language -> stats map; loc=0 in leaf rows.
// 🧩️ locByContributorsToSnapshot builds contributor breakdown from by-contrib cumulative pairs.
func locByContributorsToSnapshot(m map[string]map[string]locCumulativePair, languages []string, branchWipDenom int) map[string]map[string]LocLangStats {
	if m == nil {
		return nil
	}
	out := make(map[string]map[string]LocLangStats, len(m))
	for alias, lp := range m {
		out[alias] = locComposeLocReportSnapshot(lp, locZeroScanCounts(languages), languages, branchWipDenom)
	}
	return out
}

// 🔧️ locBuildHistory: running totals + per-commit tree scan for accurate LOC at each SHA on the logged ref.
// 📅️ locBuildHistory assembles the optional --history time series.
func locBuildHistory(repo string, languages []string, byContrib bool, commits []locRawCommit, contribFilter string) ([]LocHistoryEntry, error) {
	langW := locMakeNumstatLangSet(languages)
	running := make(map[string]locCumulativePair)
	runningBranch := make(map[string]locCumulativePair)
	for l := range langW {
		running[l] = locCumulativePair{}
		runningBranch[l] = locCumulativePair{}
	}
	byContRun := make(map[string]map[string]locCumulativePair)
	if byContrib {
		byContRun = make(map[string]map[string]locCumulativePair)
	}
	var hist []LocHistoryEntry
	var lastScan map[string]int
	filt := strings.TrimSpace(contribFilter)
	for _, c := range commits {
		for l, p := range c.Delta {
			if !langW[l] {
				continue
			}
			br := runningBranch[l]
			br.Added += p.Added
			br.Removed += p.Removed
			runningBranch[l] = br
		}
		alias := locContributorAlias(c.Author, c.AuthorMail)
		if filt != "" && !strings.EqualFold(filt, alias) {
			continue
		}
		if byContrib {
			if byContRun[alias] == nil {
				byContRun[alias] = make(map[string]locCumulativePair)
				for l := range langW {
					byContRun[alias][l] = locCumulativePair{}
				}
			}
		}
		for l, p := range c.Delta {
			if !langW[l] {
				continue
			}
			agg := running[l]
			agg.Added += p.Added
			agg.Removed += p.Removed
			running[l] = agg
			if byContrib {
				a2 := byContRun[alias][l]
				a2.Added += p.Added
				a2.Removed += p.Removed
				byContRun[alias][l] = a2
			}
		}
		// Always scan each commit tree so history rows show true LOC at that SHA (Δ% vs prior row uses consecutive samples).
		m, err := locGitSnapshotLocCounts(repo, c.SHA, languages)
		if err != nil {
			return nil, err
		}
		lastScan = m
		wipBranchDenom := locSumEditedPairs(runningBranch, langW)
		iso := time.Unix(c.WhenUnix, 0).UTC().Format(time.RFC3339)
		entry := LocHistoryEntry{SHA: c.SHA, Date: iso, Author: alias}
		if byContrib {
			row := make(map[string]map[string]LocLangStats, 1)
			row[alias] = locComposeLocReportSnapshot(byContRun[alias], lastScan, languages, wipBranchDenom)
			entry.ByContributors = row
		} else {
			entry.Languages = locComposeLocReportSnapshot(running, lastScan, languages, wipBranchDenom)
		}
		hist = append(hist, entry)
	}
	return hist, nil
}

// 📉️ locHistoryEntryStatsMap returns language rows for one history step (merged tree or single contributor snapshot).
func locHistoryEntryStatsMap(e *LocHistoryEntry) map[string]LocLangStats {
	if e == nil {
		return nil
	}
	if len(e.Languages) > 0 {
		return e.Languages
	}
	if e.ByContributors != nil {
		for _, row := range e.ByContributors {
			return row
		}
	}
	return nil
}

// 📉️ locPctLocSincePrevPtr is percent change in physical LOC from the prior history row (same category); prev 0 and curr>0 yields 100.
func locPctLocSincePrevPtr(prevLoc, currLoc int) *float64 {
	var v float64
	switch {
	case prevLoc == 0 && currLoc == 0:
		v = 0
	case prevLoc == 0:
		v = 100
	default:
		v = 100 * float64(currLoc-prevLoc) / float64(prevLoc)
	}
	v = math.Round(100*v) / 100
	return &v
}

// 📉️ locApplyHistoryLocSincePrev stamps SincePrevLocPercent vs the previous history entry (nil on first row / missing keys).
func locApplyHistoryLocSincePrev(h []LocHistoryEntry) {
	var prev map[string]int
	first := true
	for i := range h {
		rows := locHistoryEntryStatsMap(&h[i])
		if rows == nil {
			continue
		}
		for k, st := range rows {
			var pct *float64
			if !first && prev != nil {
				pr := 0
				if p, ok := prev[k]; ok {
					pr = p
				}
				pct = locPctLocSincePrevPtr(pr, st.Loc)
			}
			st.SincePrevLocPercent = pct
			rows[k] = st
		}
		next := make(map[string]int, len(rows))
		for k, st := range rows {
			next[k] = st.Loc
		}
		prev = next
		first = false
	}
}

// 📉️ locDisplayHistoryBranch maps common ref spellings to the same emoji id as the default wip branch (⛳️wip).
func locDisplayHistoryBranch(ref string) string {
	s := strings.TrimSpace(ref)
	if s == "" {
		return s
	}
	s = strings.TrimPrefix(s, "refs/heads/")
	if strings.EqualFold(s, "wip") {
		return locDefaultBranch
	}
	if s == locDefaultBranch {
		return locDefaultBranch
	}
	return strings.TrimSpace(ref)
}

// 📉️ locHistoryCheckpointLabel is the short checkpoint-style id (🔀️ + 7 hex), matching checkpoint nodes.
func locHistoryCheckpointLabel(sha string) string {
	short := strings.TrimSpace(sha)
	if len(short) > 7 {
		short = short[:7]
	}
	return emojiText(EmojiCheckpoint) + short
}

// 📉️ locContributorEmojiID matches contributor entity ids (🧑️‍💻️ + flat alias).
func locContributorEmojiID(alias string) string {
	return emojiText(EmojiContributor) + Flat(strings.TrimSpace(alias))
}

// 🫡️ runLocCommand runs `loc` and prints using global --format.
func runLocCommand(cmd *command.Command, config *Config, languages []string, history, byContrib bool, historyBranch, contribFilter string) error {
	repoRoot := config.Repo
	if strings.TrimSpace(repoRoot) == "" {
		if wd, err := os.Getwd(); err == nil {
			repoRoot = findRepoRoot(wd)
		} else {
			repoRoot = "."
		}
	} else {
		repoRoot = findRepoRoot(repoRoot)
	}
	SetRootDir(repoRoot)
	logRef := ""
	if history {
		if strings.TrimSpace(historyBranch) == "" {
			historyBranch = locDefaultBranch
		}
		logRef = historyBranch
	}
	commits, err := locWalkGitLog(repoRoot, languages, logRef)
	if err != nil {
		return err
	}
	cumAll, byC, _ := locCumulativeFromRaw(commits, languages, byContrib, "")
	if cumAll == nil {
		cumAll = make(map[string]locCumulativePair)
	}
	cumForSnap := cumAll
	cf := strings.TrimSpace(contribFilter)
	if cf != "" {
		c2, _, _ := locCumulativeFromRaw(commits, languages, false, cf)
		if c2 != nil {
			cumForSnap = c2
		}
	}
	cl, err := locGitSnapshotLocCounts(repoRoot, "", languages)
	if err != nil {
		return err
	}
	wipBranchDenom := locSumEditedPairs(cumAll, locMakeNumstatLangSet(languages))
	snap := locComposeLocReportSnapshot(cumForSnap, cl, languages, wipBranchDenom)
	rep := LocReport{Snapshot: snap, Branch: ""}
	if logRef != "" {
		rep.Branch = logRef
	}
	if byContrib {
		byUse := byC
		if cf != "" && byC != nil {
			if one, ok := byC[cf]; ok {
				byUse = map[string]map[string]locCumulativePair{cf: one}
			} else {
				found := false
				for k, v := range byC {
					if strings.EqualFold(k, cf) {
						byUse = map[string]map[string]locCumulativePair{k: v}
						found = true
						break
					}
				}
				if !found {
					byUse = nil
				}
			}
		}
		rep.ByContributors = locByContributorsToSnapshot(byUse, languages, wipBranchDenom)
	}
	if history {
		h, err := locBuildHistory(repoRoot, languages, byContrib, commits, cf)
		if err != nil {
			return err
		}
		locApplyHistoryLocSincePrev(h)
		rep.History = h
	}
	out := cmd.OutOrStdout()
	if config.IsJSON() {
		enc := json.NewEncoder(out)
		enc.SetIndent("", "  ")
		if err := enc.Encode(map[string]LocReport{"loc": rep}); err != nil {
			return err
		}
		return nil
	}
	if config.IsMarkdown() {
		renderLocMarkdown(out, &rep, history, byContrib)
		return nil
	}
	isTTY := false
	if f, ok := out.(*os.File); ok {
		if st, e := f.Stat(); e == nil {
			if (st.Mode() & os.ModeCharDevice) != 0 {
				isTTY = true
			}
		}
	}
	if os.Getenv("NO_COLOR") != "" {
		isTTY = false
	}
	renderLocText(out, &rep, history, byContrib, isTTY)
	return nil
}
