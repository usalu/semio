package metrics

import (
	"strings"
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

// #endregion 🏷️Vocabulary
