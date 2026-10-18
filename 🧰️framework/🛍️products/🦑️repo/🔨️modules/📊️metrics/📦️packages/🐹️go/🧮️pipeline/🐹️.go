// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧮️pipeline carries the snapshot counting, history building and report pipeline.

// #endregion 🧲️Header

package metrics

import (
	"bytes"
	"runtime"
	"strings"
	"sync"
	"sync/atomic"
)

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
