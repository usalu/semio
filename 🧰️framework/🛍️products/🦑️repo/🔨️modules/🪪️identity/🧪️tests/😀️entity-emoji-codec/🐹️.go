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

func theLeadingEmojiGraphemeIsUnicodes(ctx *host.Context) (host.Outcome, error) {
	file, err := loadEmojiVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	graphemes := make([]string, 0, len(file.Graphemes))
	for _, vector := range file.Graphemes {
		emoji, remaining := identity.ExtractEntityEmoji(vector.Input)
		graphemes = append(graphemes, fmt.Sprintf("%s=%s|%s", vector.Name, emoji, remaining))
	}
	return host.Outcome{Projection: map[string]any{"graphemes": graphemes}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-leading-emoji-grapheme-is-unicodes", theLeadingEmojiGraphemeIsUnicodes)
}

// endregion 🔖️Registration
