// 🐹️ Go subject for the numstat parsing case, written against the same frozen contract as the
// Rust subject and the git oracle.
//
// The subject is `github.com/usalu/semio/repo/metrics`, whose `ParseNumstatLog` is the exported,
// git-free reimplementation of the pre-split godfile's unexported `locParseNumstatLog`.
package adapter

import (
	"encoding/json"

	metrics "github.com/usalu/semio/repo/metrics"
	host "semio.tech/repo/test"
)

// region 🔖️Helpers

// 🎞️ transcript is the recorded git session this case replays.
func transcript(ctx *host.Context) (*metrics.GitTranscript, error) {
	raw, err := ctx.FixtureBytes("shared://🎞️git-transcript.json")
	if err != nil {
		return nil, err
	}
	return metrics.ParseGitTranscript(raw)
}

// 🧩️ commitsOf parses one recorded ref into commit records.
func commitsOf(ctx *host.Context, gitRef string) ([]metrics.CommitDelta, error) {
	recorded, err := transcript(ctx)
	if err != nil {
		return nil, err
	}
	return metrics.ParseNumstatLog(recorded.Logs[gitRef], metrics.MakeNumstatLangSet(metrics.DefaultCodeLanguages), nil), nil
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

func recordedTranscriptMatchesRealGit(ctx *host.Context) (host.Outcome, error) {
	recorded, err := transcript(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	noMerges := recorded.Logs[""]
	commits := metrics.ParseNumstatLog(noMerges, metrics.MakeNumstatLangSet(metrics.DefaultCodeLanguages), nil)
	shas := make([]string, 0, len(commits))
	for _, commit := range commits {
		shas = append(shas, commit.SHA)
	}
	tracked := recorded.Tracked[""]
	if tracked == nil {
		tracked = []string{}
	}
	return host.Outcome{Projection: map[string]any{
		"noMergesDigest":   host.Digest([]byte(noMerges)),
		"withMergesDigest": host.Digest([]byte(recorded.Logs["🔀️with-merges"])),
		"commitCount":      len(commits),
		"shas":             shas,
		"trackedAtHead":    tracked,
	}}, nil
}

func parsesRecordedNumstatStream(ctx *host.Context) (host.Outcome, error) {
	commits, err := commitsOf(ctx, "")
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(commits)
}

func resolvesRenamesAndQuotedPaths(ctx *host.Context) (host.Outcome, error) {
	commits, err := commitsOf(ctx, "")
	if err != nil {
		return host.Outcome{}, err
	}
	files := make([]metrics.FileDelta, 0)
	for _, commit := range commits {
		files = append(files, commit.Files...)
	}
	return projection(files)
}

func mergeCommitCarriesNoFileRows(ctx *host.Context) (host.Outcome, error) {
	commits, err := commitsOf(ctx, "🔀️with-merges")
	if err != nil {
		return host.Outcome{}, err
	}
	return projection(commits)
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("recorded-transcript-matches-real-git", recordedTranscriptMatchesRealGit).
		Subject("parses-recorded-numstat-stream", parsesRecordedNumstatStream).
		Subject("resolves-renames-and-quoted-paths", resolvesRenamesAndQuotedPaths).
		Subject("merge-commit-carries-no-file-rows", mergeCommitCarriesNoFileRows)
}

// endregion 🔖️Registration
