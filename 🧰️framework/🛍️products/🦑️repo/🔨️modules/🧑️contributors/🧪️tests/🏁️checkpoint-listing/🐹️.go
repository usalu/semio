// 🐹️ Go side of the checkpoint listing case.
package adapter

import (
	"encoding/json"
	"fmt"

	contributors "github.com/usalu/semio/repo/contributors"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type checkpointLogFile struct {
	Schema     string   `json:"schema"`
	Log        string   `json:"log"`
	Identities []string `json:"identities"`
	Malformed  []string `json:"malformed"`
}

func renderCheckpoint(checkpoint contributors.Checkpoint) string {
	author := ""
	if checkpoint.AuthorID != nil {
		author = *checkpoint.AuthorID
	}
	return fmt.Sprintf("%s|%s|%s|%s", checkpoint.SHA, author, checkpoint.Date, checkpoint.Title)
}

func loadCheckpointLog(ctx *host.Context) (checkpointLogFile, error) {
	data, err := ctx.FixtureBytes("shared://🏁️checkpoint-log.json")
	if err != nil {
		return checkpointLogFile{}, err
	}
	var file checkpointLogFile
	err = json.Unmarshal(data, &file)
	return file, err
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func theLogParsesIntoCheckpoints(ctx *host.Context) (host.Outcome, error) {
	file, err := loadCheckpointLog(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	source := contributors.NewMemoryCheckpointSource(file.Log)
	checkpoints := contributors.ListCheckpoints(source, nil)
	parsed := make([]string, 0, len(checkpoints))
	identifiers := make([]string, 0, len(checkpoints))
	for _, checkpoint := range checkpoints {
		parsed = append(parsed, renderCheckpoint(checkpoint))
		identifiers = append(identifiers, fmt.Sprintf("%s=%s", checkpoint.SHA, contributors.CheckpointID(checkpoint)))
	}
	refusals := make([]string, 0, len(file.Malformed))
	for _, line := range file.Malformed {
		refusals = append(refusals, fmt.Sprintf("%s=%d", line, len(contributors.ParseCheckpointLog(line))))
	}
	three := 3
	limited := make([]string, 0, three)
	for _, checkpoint := range contributors.ListCheckpoints(source, &three) {
		limited = append(limited, renderCheckpoint(checkpoint))
	}
	searched := make([]string, 0)
	for _, checkpoint := range contributors.SearchCheckpoints(source, nil, "🚩️59") {
		searched = append(searched, renderCheckpoint(checkpoint))
	}
	return host.Outcome{Projection: map[string]any{
		"checkpoints": parsed,
		"identifiers": identifiers,
		"refusals":    refusals,
		"limited":     limited,
		"searched":    searched,
	}}, nil
}

func aLimitIsAPrefixOfTheListing(ctx *host.Context) (host.Outcome, error) {
	file, err := loadCheckpointLog(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	source := contributors.NewMemoryCheckpointSource(file.Log)
	all := make([]string, 0)
	for _, checkpoint := range contributors.ListCheckpoints(source, nil) {
		all = append(all, renderCheckpoint(checkpoint))
	}
	prefixes := make([]string, 0, len(all)+1)
	for limit := 0; limit <= len(all); limit++ {
		bounded := limit
		limited := make([]string, 0, limit)
		for _, checkpoint := range contributors.ListCheckpoints(source, &bounded) {
			limited = append(limited, renderCheckpoint(checkpoint))
		}
		matches := len(limited) == limit
		for index := 0; matches && index < limit; index++ {
			matches = limited[index] == all[index]
		}
		prefixes = append(prefixes, fmt.Sprintf("%d=%t", limit, matches))
	}
	return host.Outcome{Projection: map[string]any{"prefixes": prefixes, "total": len(all)}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-log-parses-into-checkpoints", theLogParsesIntoCheckpoints).
		Subject("a-limit-is-a-prefix-of-the-listing", aLimitIsAPrefixOfTheListing)
}

// endregion 🔖️Registration
