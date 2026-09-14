// 🐹️ Go side of the entity emoji codec case.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	identity "github.com/usalu/semio/repo/identity"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type namedInput struct {
	Name  string `json:"name"`
	Input string `json:"input"`
}

type emojiVectorFile struct {
	Schema          string       `json:"schema"`
	Graphemes       []namedInput `json:"graphemes"`
	Normalizations  []string     `json:"normalizations"`
	Refs            []namedInput `json:"refs"`
	GoalPaths       []string     `json:"goalPaths"`
	Contributors    []string     `json:"contributors"`
	IdentifierSeeds []uint64     `json:"identifierSeeds"`
}

func loadEmojiVectors(ctx *host.Context) (emojiVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://📡️emoji-vectors.json")
	if err != nil {
		return emojiVectorFile{}, err
	}
	var file emojiVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

// codePoints renders a string as a `U+XXXX` sequence so a projection never hides a selector.
func codePoints(value string) string {
	if value == "" {
		return "-"
	}
	parts := make([]string, 0, len(value))
	for _, current := range value {
		parts = append(parts, fmt.Sprintf("U+%04X", current))
	}
	return strings.Join(parts, " ")
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func theOwnedCodecsRoundTrip(ctx *host.Context) (host.Outcome, error) {
	file, err := loadEmojiVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	normalizations := make([]string, 0, len(file.Normalizations))
	for _, emoji := range file.Normalizations {
		normalizations = append(normalizations, fmt.Sprintf("%s=%s", codePoints(emoji), codePoints(identity.EmojiText(emoji))))
	}
	refs := make([]string, 0, len(file.Refs))
	for _, vector := range file.Refs {
		parsed := identity.ParseArtifactRef(vector.Input)
		refs = append(refs, fmt.Sprintf("%s=%s|%s|%s", vector.Name, parsed.Kind, parsed.Path, strings.Join(parsed.SectionParts, "#")))
	}
	goals := make([]string, 0, len(file.GoalPaths))
	for _, goalPath := range file.GoalPaths {
		compose := identity.GoalPathToComposeID(goalPath)
		goals = append(goals, fmt.Sprintf("%s=%s|%s", goalPath, compose, strings.Join(identity.ComposeIDToGoalSegments(compose), "/")))
	}
	contributors := make([]string, 0, len(file.Contributors))
	for _, alias := range file.Contributors {
		compose := identity.ContributorToComposeID(alias)
		contributors = append(contributors, fmt.Sprintf("%s=%s|%s", alias, compose, identity.ComposeIDToContributorFlat(compose)))
	}
	identifiers := make([]string, 0, len(file.IdentifierSeeds))
	for _, seed := range file.IdentifierSeeds {
		value, err := identity.NewFrom(identity.NewSeededEntropy(seed))
		if err != nil {
			return host.Outcome{}, err
		}
		identifiers = append(identifiers, fmt.Sprintf("%d=%s", seed, value))
	}
	vocabulary := make([]string, 0)
	for _, emoji := range identity.AllEntityEmojis() {
		vocabulary = append(vocabulary, codePoints(emoji))
	}
	return host.Outcome{Projection: map[string]any{
		"normalizations": normalizations,
		"refs":           refs,
		"goals":          goals,
		"contributors":   contributors,
		"identifiers":    identifiers,
		"vocabulary":     vocabulary,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-owned-codecs-round-trip", theOwnedCodecsRoundTrip)
}

// endregion 🔖️Registration
